mod db;
mod mqtt;

use db::Db;
use env_logger::Env;

fn main() -> anyhow::Result<()> {
    dotenvy::dotenv()?;
    env_logger::init_from_env(Env::default().default_filter_or("info"));

    db::extensions()?;
    let db = Db::connect()?;
    db.query_migrations()?;
    mqtt::pubsub()?;
    Ok(())
}
