# OwnTrack-rs

## Prerequisites

* Just: https://just.systems/man/en/

* https://crates.io/crates/refinery_cli

```
cargo install refinery_cli
```

## Setup

Set env vars:

```
cat >.env <<EOF
MQTT_URL="mqtts://owntracks.example:8883"
MQTT_USER=$(id -u)
MQTT_PASSWORD="xxx"
# libpq connection string or PostgreSQL URI
DB_CONNECTION="postgres://user:pass@localhost:5432/owntracks"
DB_SCHEMA="public"
EOF
chmod 600 .env
edit .env
```

Create database:
```
just create-db
just migrate
```

## Frontend development

```
cd frontend
```

Preparation:
```
npm install
```

```
npm run dev
```
