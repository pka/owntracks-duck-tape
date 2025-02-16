use crate::owntracks::Location;
use duckdb::{params, Connection, DuckdbConnectionManager};

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
