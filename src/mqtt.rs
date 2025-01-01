use rumqttc::{Client, Event, Incoming, MqttOptions, QoS};
use std::time::Duration;

pub fn pubsub() {
    let mqtt_url = dotenvy::var("MQTT_URL").unwrap();
    let mqtt_user = dotenvy::var("MQTT_USER").unwrap();
    let mqtt_password = dotenvy::var("MQTT_PASSWORD").unwrap();
    let client_id = "duck-tape"; // TODO: hostname + pid

    let mut mqttoptions =
        MqttOptions::parse_url(format!("{mqtt_url}?client_id={client_id}")).unwrap();
    mqttoptions.set_credentials(mqtt_user, mqtt_password);
    mqttoptions.set_keep_alive(Duration::from_secs(5));

    let (client, mut connection) = Client::new(mqttoptions, 10);
    client.subscribe("owntracks/#", QoS::AtMostOnce).unwrap();

    // Iterate to poll the eventloop for connection progress
    for notification in connection.iter() {
        println!("Notification = {notification:?}");
        if let Ok(Event::Incoming(Incoming::Publish(packet))) = notification {
            println!(
                "Payload = {}",
                String::from_utf8_lossy(packet.payload.as_ref())
            );
        }
    }
}
