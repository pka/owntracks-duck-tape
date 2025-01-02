mod db;
mod mqtt;

use env_logger::Env;

fn main() -> anyhow::Result<()> {
    dotenvy::dotenv()?;
    env_logger::init_from_env(Env::default().default_filter_or("info"));

    db::dbtest()?;
    mqtt::pubsub()?;
    Ok(())
}
