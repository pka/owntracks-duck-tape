use rumqttc::{Client, Event, Incoming, MqttOptions, QoS};
use serde::{Deserialize, Serialize};
use std::time::Duration;

/// OwnTracks JSON message
/// <https://owntracks.org/booklet/tech/json/>
#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "_type")]
#[serde(rename_all = "lowercase")]
enum Message {
    Beacon {},
    Card {},
    Cmd {},
    Configuration {},
    Encrypted {},
    Location { lat: f32, lon: f32, created_at: i64 },
    Lwt {},
    Request {},
    Status {},
    Steps {},
    Transition {},
    Waypoint {},
    Waypoints {},
}

pub fn subscribe() -> anyhow::Result<()> {
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
        log::info!("Notification = {notification:?}");
        if let Ok(Event::Incoming(Incoming::Publish(packet))) = notification {
            log::info!(
                "Payload = {}",
                String::from_utf8_lossy(packet.payload.as_ref())
            );
            let msg: Message = serde_json::from_slice(packet.payload.as_ref())?;
            log::info!("{msg:?}");
        }
    }
    Ok(())
}
