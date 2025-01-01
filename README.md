# OwnTracks DuckDB Recorder

## Setup

Set env vars:

```
 cat >.env <<EOF
MQTT_URL="mqtts://owntracks.example:8883"
MQTT_USER=$(id -u)
MQTT_PASSWORD="xxx"
EOF
chmod 600 .env
```
