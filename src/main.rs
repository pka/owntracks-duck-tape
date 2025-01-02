pub mod db;
mod mqtt;

use db::Db;
use env_logger::Env;

fn main() -> anyhow::Result<()> {
    dotenvy::dotenv()?;
    env_logger::init_from_env(Env::default().default_filter_or("info"));

    let db = Db::connect()?;
    mqtt::subscribe(&db)?;
    Ok(())
}
