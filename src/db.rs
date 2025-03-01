use crate::owntracks::Location;
use duckdb::{params, types::Value, Connection, DuckdbConnectionManager};
use r2d2::ManageConnection;
use serde::{Serialize, Serializer};
use time::OffsetDateTime;

pub struct Track {
    pub user: String,
    pub device: String,
    pub date: time::Date,
    pub points: Vec<GpsPoint>,
}

pub struct GpsPoint {
    pub y: f32,
    pub x: f32,
    pub ts: OffsetDateTime,
    pub speed: i16,
    pub elevation: i16,
    /// Accuracy in meters. 0 = unknown
    pub accuracy: i32, // owntracks: u32
    /// Vertical accuracy in meters. 0 = unknown
    pub v_accuracy: i16,
}

#[derive(Serialize)]
pub struct TrackInfo {
    pub user: String,
    pub device: String,
    #[serde(serialize_with = "serialize_to_string")]
    pub date: time::Date,
    #[serde(serialize_with = "serialize_to_string")]
    pub ts_start: OffsetDateTime,
    #[serde(serialize_with = "serialize_to_string")]
    pub ts_end: OffsetDateTime,
    pub speed_min: Option<i16>,
    pub speed_max: Option<i16>,
    pub elevation_min: Option<i16>,
    pub elevation_max: Option<i16>,
}

#[derive(Clone)]
pub struct Db {
    pool: r2d2::Pool<DuckdbConnectionManager>,
}

#[derive(Debug)]
struct ConnectionCustomizer;

impl<E> r2d2::CustomizeConnection<Connection, E> for ConnectionCustomizer {
    fn on_acquire(&self, conn: &mut Connection) -> Result<(), E> {
        if let Ok(db_schema) = dotenvy::var("DB_SCHEMA") {
            conn.execute_batch(&format!("USE db.{db_schema};")).unwrap();
        }
        Ok(())
    }
}

impl Db {
    pub fn connect() -> duckdb::Result<Self> {
        let conn_str = dotenvy::var("DB_CONNECTION").expect("DB_CONNECTION");
        let manager = DuckdbConnectionManager::memory()?;
        let conn = manager.connect()?;
        conn.execute_batch("INSTALL postgres; LOAD postgres;")?;
        conn.execute_batch(&format!("ATTACH '{conn_str}' AS db (TYPE POSTGRES);"))?;
        let pool = r2d2::Pool::builder()
            .connection_customizer(Box::new(ConnectionCustomizer))
            .build(manager)
            .unwrap();
        Ok(Db { pool })
    }

    pub fn insert_location(&self, user: &str, device: &str, loc: &Location) -> duckdb::Result<()> {
        self.pool.get().unwrap().execute(
            "INSERT INTO gpslog
               (user, device, tid, ts, velocity, lat, lon, alt, accuracy, v_accuracy, batt_level, batt_status,
                cog, rad, trigger, pressure, poi, conn_status, tag, topic, inregions, inrids, ssid, bssid,
                created_at, mmode)
                VALUES (?, ?, ?, to_timestamp(?), ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?,
                        to_timestamp(?), ?)",
            params![user, device, loc.tid, loc.ts, loc.velocity, loc.lat, loc.lon, loc.alt, loc.accuracy, loc.v_accuracy, loc.batt_level, loc.batt_status,
                loc.cog, loc.rad, loc.trigger, loc.pressure, loc.poi, loc.conn_status, loc.tag, loc.topic, loc.inregions, loc.inrids, loc.ssid, loc.bssid,
                loc.created_at, loc.mmode],
        )?;
        Ok(())
    }

    /// Return track infos of a given date
    pub fn query_tracks_info(&self, date: &str) -> duckdb::Result<Vec<TrackInfo>> {
        let conn = self.pool.get().unwrap();
        let mut stmt = conn.prepare(
            "SELECT user, device, ts::date,
                    min(ts) as ts_start, max(ts) as ts_end, min(velocity) as speed_min, max(velocity) as speed_max,
                    min(alt) as elevation_min, max(alt) as elevation_max
            FROM gpslog
            WHERE ts::date = ?
            GROUP BY user, device, ts::date",
        )?;
        let tracks: duckdb::Result<Vec<TrackInfo>> = stmt
            .query_map(duckdb::params![date], |row| {
                let date = match row.get::<_, Value>(2)? {
                    Value::Date32(days) => Some(
                        OffsetDateTime::from_unix_timestamp(days as i64 * 86_400)
                            .unwrap()
                            .date(),
                    ),
                    _ => None,
                }
                .unwrap();
                let ts_start = match row.get::<_, Value>(3)? {
                    Value::Timestamp(_unit, micros) => Some(
                        OffsetDateTime::from_unix_timestamp_nanos(micros as i128 * 1_000).unwrap(),
                    ),
                    _ => None,
                }
                .unwrap();
                let ts_end = match row.get::<_, Value>(4)? {
                    Value::Timestamp(_unit, micros) => Some(
                        OffsetDateTime::from_unix_timestamp_nanos(micros as i128 * 1_000).unwrap(),
                    ),
                    _ => None,
                }
                .unwrap();
                Ok(TrackInfo {
                    user: row.get(0)?,
                    device: row.get(1)?,
                    date,
                    ts_start,
                    ts_end,
                    speed_min: row.get(5)?,
                    speed_max: row.get(6)?,
                    elevation_min: row.get(7)?,
                    elevation_max: row.get(8)?,
                })
            })?
            .collect();
        Ok(tracks?)
    }

    /// Return tracks of a given date
    pub fn query_tracks(&self, date: &str) -> duckdb::Result<Vec<Track>> {
        let conn = self.pool.get().unwrap();
        let mut stmt = conn.prepare(
            "SELECT user, device, ts::date, array_agg((lat, lon, ts, velocity, alt, accuracy, v_accuracy) ORDER BY id) AS points
            FROM gpslog
            WHERE ts::date = ?
            GROUP BY user, device, ts::date",
        )?;
        let mut rows = stmt.query(duckdb::params![date])?;

        let mut tracks = Vec::new();
        while let Some(row) = rows.next()? {
            let user: String = row.get(0)?;
            let device: String = row.get(1)?;
            let date: Value = row.get(2)?;
            let points: Value = row.get(3)?;

            let Value::Date32(days) = date else {
                continue;
            };
            // Convert Value::List(points) to [GpsPoint]
            let Value::List(point_list) = points else {
                continue;
            };
            let track_points = point_list.iter().filter_map(|point| {
                let Value::Struct(map) = point else {
                    return None;
                };
                let values: Vec<&Value> = map.values().collect();
                // log::debug!("{values:?}");
                let (
                    Value::Float(y),
                    Value::Float(x),
                    Value::Timestamp(_unit, micros),
                    Value::SmallInt(speed),
                    Value::SmallInt(elevation),
                ) = (values[0], values[1], values[2], values[3], values[4])
                else {
                    log::debug!("Invalid record (lat, lon, ts, velocity, alt, accuracy, v_accuracy): {values:?}");
                    return None;
                };
                let accuracy = match values[5] {
                    Value::Int(accuracy) => *accuracy,
                    Value::Null => 0,
                    val => {
                        log::debug!("Invalid accuracy: {val:?}");
                        return None;
                    }
                };
                let v_accuracy = match values[6] {
                    Value::SmallInt(v_accuracy) => *v_accuracy,
                    Value::Null => 0,
                    val => {
                        log::debug!("Invalid vertical accuracy: {val:?}");
                        return None;
                    }
                };
                let gpspt = GpsPoint {
                    y: *y,
                    x: *x,
                    ts: OffsetDateTime::from_unix_timestamp_nanos(*micros as i128 * 1_000).unwrap(),
                    speed: *speed,
                    elevation: *elevation,
                    accuracy,
                    v_accuracy,
                };
                Some(gpspt)
            });
            let track = Track {
                user,
                device,
                date: OffsetDateTime::from_unix_timestamp(days as i64 * 86_400)
                    .unwrap()
                    .date(),
                points: track_points.collect(),
            };
            tracks.push(track);
        }
        Ok(tracks)
    }

    pub fn query_migrations(&self) -> duckdb::Result<()> {
        let conn = self.pool.get().unwrap();
        let mut stmt = conn.prepare("SELECT * FROM refinery_schema_history;")?;
        let mut rows = stmt.query([])?;

        println!("schema history:");
        while let Some(row) = rows.next()? {
            let version: u64 = row.get(0)?;
            let name: String = row.get(1)?;
            let applied_on: String = row.get(2)?;
            let checksum: String = row.get(3)?;
            println!("{version} {name} {applied_on} {checksum}");
        }

        Ok(())
    }
}

fn get_version(conn: &Connection) -> duckdb::Result<String> {
    conn.query_row("SELECT version()", [], |row| row.get(0))
}

pub fn extensions() -> duckdb::Result<()> {
    let conn = Connection::open_in_memory()?;

    let version = get_version(&conn)?;
    println!("\nDuckDB version: {version}\n");

    let mut stmt = conn.prepare(
        r"SELECT extension_name, installed, description
          FROM duckdb_extensions()",
    )?;
    let mut rows = stmt.query([])?;

    println!("Extensions:");
    while let Some(row) = rows.next()? {
        let name: String = row.get(0)?;
        let installed: bool = row.get(1)?;
        let descr: String = row.get(2)?;
        println!("{name} ({installed}): {descr}");
    }

    Ok(())
}

fn serialize_to_string<T: std::fmt::Display, S: Serializer>(
    value: &T,
    serializer: S,
) -> Result<S::Ok, S::Error> {
    let s = value.to_string();
    serializer.serialize_str(&s)
}
