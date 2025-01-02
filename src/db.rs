use duckdb::Connection;

pub struct Db {
    conn: Connection,
}

impl Db {
    pub fn connect() -> duckdb::Result<Self> {
        let conn_str = dotenvy::var("DB_CONNECTION").expect("DB_CONNECTION");
        let db_schema = dotenvy::var("DB_SCHEMA").expect("DB_SCHEMA");
        let conn = Connection::open_in_memory()?;

        conn.execute_batch("INSTALL postgres; LOAD postgres;")?;
        conn.execute_batch(&format!("ATTACH '{conn_str}' AS db (TYPE POSTGRES);"))?;
        conn.execute_batch(&format!("USE db.{db_schema};"))?;
        Ok(Db { conn })
    }

    pub fn query_migrations(&self) -> duckdb::Result<()> {
        let mut stmt = self
            .conn
            .prepare("SELECT * FROM refinery_schema_history;")?;
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
