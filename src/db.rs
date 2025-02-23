use crate::owntracks::Location;
use duckdb::{params, types::Value, Connection, DuckdbConnectionManager};
use time::OffsetDateTime;

#[derive(Clone)]
pub struct Db {
    pool: r2d2::Pool<DuckdbConnectionManager>,
}

#[derive(Debug)]
struct ConnectionCustomizer;

impl<E> r2d2::CustomizeConnection<Connection, E> for ConnectionCustomizer {
    fn on_acquire(&self, conn: &mut Connection) -> Result<(), E> {
        let db_schema = dotenvy::var("DB_SCHEMA").expect("DB_SCHEMA");
        conn.execute_batch(&format!("USE db.{db_schema};")).ok();
        Ok(())
    }
}

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
    pub accuracy: i32,
    pub v_accuracy: i16,
}

impl Db {
    pub fn connect() -> duckdb::Result<Self> {
        let conn_str = dotenvy::var("DB_CONNECTION").expect("DB_CONNECTION");
        let db_schema = dotenvy::var("DB_SCHEMA").expect("DB_SCHEMA");
        let manager = DuckdbConnectionManager::memory()?;
        let pool = r2d2::Pool::builder()
            .connection_customizer(Box::new(ConnectionCustomizer))
            .build(manager)
            .unwrap();
        let conn = pool.get().unwrap();
        conn.execute_batch("INSTALL postgres; LOAD postgres;")?;
        conn.execute_batch(&format!("ATTACH '{conn_str}' AS db (TYPE POSTGRES);"))?;
        conn.execute_batch(&format!("USE db.{db_schema};"))?; // ConnectionCustomizer is called to early
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
                    Value::Int(accuracy),
                    Value::SmallInt(v_accuracy),
                ) = (
                    values[0], values[1], values[2], values[3], values[4], values[5], values[6],
                )
                else {
                    return None;
                };
                let gpspt = GpsPoint {
                    y: *y,
                    x: *x,
                    ts: OffsetDateTime::from_unix_timestamp_nanos(*micros as i128 * 1_000).unwrap(),
                    speed: *speed,
                    elevation: *elevation,
                    accuracy: *accuracy,
                    v_accuracy: *v_accuracy,
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
