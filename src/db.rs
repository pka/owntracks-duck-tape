use crate::owntracks::Location;
use duckdb::{params, types::Value, Connection, DuckdbConnectionManager};
use geo_types::Point;
use gpx::{Gpx, GpxVersion, Track, TrackSegment, Waypoint};

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

    pub fn insert_location(&self, loc: &Location) -> duckdb::Result<()> {
        self.pool.get().unwrap().execute(
            "INSERT INTO gpslog
               (tid, ts, velocity, lat, lon, alt, accuracy, v_accuracy, batt_level, batt_status,
                cog, rad, trigger, pressure, poi, conn_status, tag, topic, inregions, inrids, ssid, bssid,
                created_at, mmode)
                VALUES (?, to_timestamp(?), ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?,
                        to_timestamp(?), ?)",
            params![loc.tid, loc.ts, loc.velocity, loc.lat, loc.lon, loc.alt, loc.accuracy, loc.v_accuracy, loc.batt_level, loc.batt_status,
                loc.cog, loc.rad, loc.trigger, loc.pressure, loc.poi, loc.conn_status, loc.tag, loc.topic, loc.inregions, loc.inrids, loc.ssid, loc.bssid,
                loc.created_at, loc.mmode],
        )?;
        Ok(())
    }

    pub fn query_tracks(&self, date: &str) -> duckdb::Result<String> {
        let conn = self.pool.get().unwrap();
        let mut stmt = conn.prepare(
            "SELECT tid, ts::date, array_agg((lat,lon) ORDER BY id) AS points
            FROM gpslog
            WHERE ts::date = ?
            GROUP BY tid, ts::date",
        )?;
        let mut rows = stmt.query(duckdb::params![date])?;

        let mut gpx = Gpx {
            version: GpxVersion::Gpx11,
            creator: None,
            metadata: None,
            waypoints: vec![],
            tracks: vec![],
            routes: vec![],
        };

        while let Some(row) = rows.next()? {
            let tid: String = row.get(0)?;
            let date: Value = row.get(1)?;
            let points: Value = row.get(2)?;
            let mut track_segment = TrackSegment { points: vec![] };
            if let Value::List(list) = points {
                for point in list {
                    if let Value::Struct(map) = point {
                        let values: Vec<&Value> = map.values().collect();
                        if let (Value::Float(y), Value::Float(x)) = (values[0], values[1]) {
                            let wpt = Waypoint::new(Point::new(*x as f64, *y as f64));
                            track_segment.points.push(wpt);
                        }
                    }
                }
            }
            let track = Track {
                name: Some(format!("Track {date:?}-{tid}")),
                comment: None,
                description: None,
                source: None,
                links: vec![],
                type_: None,
                number: None,
                segments: vec![track_segment],
            };
            gpx.tracks.push(track);
        }

        let mut vec = Vec::new();
        gpx::write(&gpx, &mut vec).unwrap();
        let gpxstr = String::from_utf8(vec).unwrap();
        Ok(gpxstr)
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
