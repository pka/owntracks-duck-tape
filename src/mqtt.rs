use crate::db::Db;
use crate::owntracks::Message;
use gethostname::gethostname;
use rumqttc::{Client, Event, Incoming, MqttOptions, QoS};
use std::process;
use std::time::Duration;

pub fn subscribe(db: &Db) -> anyhow::Result<()> {
    let mqtt_url = match dotenvy::var("MQTT_URL") {
        Ok(url) => url,
        Err(_) => {
            log::info!("MQTT_URL not set, skipping MQTT client");
            return Ok(());
        }
    };
    let mqtt_user = dotenvy::var("MQTT_USER")?;
    let mqtt_password = dotenvy::var("MQTT_PASSWORD")?;
    let client_id = format!("{}-{}", gethostname().to_string_lossy(), process::id());

    let mut mqttoptions = MqttOptions::parse_url(format!("{mqtt_url}?client_id={client_id}"))?;
    mqttoptions.set_credentials(mqtt_user, mqtt_password);
    mqttoptions.set_keep_alive(Duration::from_secs(5));

    let (client, mut connection) = Client::new(mqttoptions, 10);
    client.subscribe("owntracks/#", QoS::AtMostOnce)?;

    // Iterate to poll the eventloop for connection progress
    for notification in connection.iter() {
        log::debug!("Notification = {notification:?}");
        if let Ok(Event::Incoming(Incoming::Publish(packet))) = notification {
            log::info!(
                "Payload = {}",
                String::from_utf8_lossy(packet.payload.as_ref())
            );
            let msg: Message = serde_json::from_slice(packet.payload.as_ref())?;
            log::debug!("{msg:?}");
            if let Message::Location(loc) = msg {
                if let Err(e) = db.insert_location(&loc) {
                    log::error!("{e}");
                }
            }
        }
    }
    Ok(())
}
