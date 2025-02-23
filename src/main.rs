pub mod db;
mod geojson;
mod gpx;
mod http;
mod mqtt;
mod owntracks;

use db::Db;
use env_logger::Env;
use std::thread;

fn main() -> anyhow::Result<()> {
    match dotenvy::dotenv() {
        Ok(_) | Err(dotenvy::Error::Io(_)) => {} // ignore missing .env file
        Err(err) => anyhow::bail!(err),
    }
    env_logger::init_from_env(Env::default().default_filter_or("info"));

    log::info!("Connecting to database...");
    let db = Db::connect()?;
    let mqtt_db = db.clone();
    let _handler = thread::spawn(move || mqtt::subscribe(&mqtt_db).unwrap());
    http::webserver(db)?;
    Ok(())
}
