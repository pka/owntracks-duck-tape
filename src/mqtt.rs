use crate::db::Db;
use rumqttc::{Client, Event, Incoming, MqttOptions, QoS};
use serde::{Deserialize, Serialize};
use std::time::Duration;

/// OwnTracks JSON message
/// <https://owntracks.org/booklet/tech/json/>
#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "_type")]
#[serde(rename_all = "lowercase")]
enum Message {
    Beacon,
    Card,
    Cmd,
    Configuration,
    Encrypted,
    Location(Location),
    Lwt,
    Request,
    Status,
    Steps,
    Transition,
    Waypoint,
    Waypoints,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Location {
    pub lat: f32,
    pub lon: f32,
    pub created_at: i64,
}

pub fn subscribe(db: &Db) -> anyhow::Result<()> {
    let mqtt_url = dotenvy::var("MQTT_URL")?;
    let mqtt_user = dotenvy::var("MQTT_USER")?;
    let mqtt_password = dotenvy::var("MQTT_PASSWORD")?;
    let client_id = "duck-tape"; // TODO: hostname + pid

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
