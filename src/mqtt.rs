use crate::db::Db;
use crate::owntracks::Message;
use gethostname::gethostname;
use rumqttc::{AsyncClient, Event, Incoming, MqttOptions, QoS};
use std::process;
use std::time::Duration;
use tokio::time;

pub async fn subscribe(db: &Db) -> anyhow::Result<()> {
    let mqtt_url = match dotenvy::var("MQTT_URL") {
        Ok(url) if !url.is_empty() => url,
        Err(_) | Ok(_) => {
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
    mqttoptions.set_clean_session(false);

    let (client, mut eventloop) = AsyncClient::new(mqttoptions, 10);
    client.subscribe("owntracks/#", QoS::AtMostOnce).await?;

    loop {
        let notification = eventloop.poll().await;
        log::debug!("Notification = {notification:?}");
        match notification {
            Ok(Event::Incoming(Incoming::Publish(packet))) => {
                log::info!(
                    "{}: {}",
                    packet.topic,
                    String::from_utf8_lossy(packet.payload.as_ref()),
                );
                let msg: Message = match serde_json::from_slice(packet.payload.as_ref()) {
                    Ok(msg) => msg,
                    Err(e) => {
                        log::error!("{e}");
                        continue;
                    }
                };
                log::debug!("{msg:?}");
                if let Message::Location(loc) = msg {
                    let Some((user, device)) = get_user_device_from_topic(&packet.topic) else {
                        log::error!("Unexpected topic `{}`", packet.topic);
                        continue;
                    };
                    if let Err(e) = db.insert_location(&user, &device, &loc).await {
                        log::error!("{e}");
                    }
                }
            }
            Ok(_ev) => {}
            Err(error) => {
                log::info!("MQTT error: {error}");
                // Avoid error flood
                time::sleep(Duration::from_millis(500)).await;
            }
        }
    }
}

pub fn get_user_device_from_topic(topic: &str) -> Option<(String, String)> {
    // topic: "onwntrack/{user}/{device}"
    let parts: Vec<&str> = topic.split('/').collect();
    if parts.len() != 3 {
        return None;
    }
    let user = parts[1].to_string();
    let device = parts[2].to_string();
    Some((user, device))
}
