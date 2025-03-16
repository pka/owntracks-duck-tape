# Owntrack-rs

## Overview

Owntrack-rs is a self-hosted GPS tracking solution that allows you to record and manage your location data from mobile phones or IoT devices.
It provides [OwnTracks](https://owntracks.org/booklet/) compatible HTTP and MQTT endpoints and a built-in viewer application.

![Screenshot](images/screenshot.jpg)

Features:
- [x] Owntracks compatible HTTP endpoint
- [x] Owntracks compatible MQTT interface
- [x] SQLite local file storage
- [x] PostgreSQL database storage
- [x] GeoJSON and GPX track exports
- [x] Built-In Viewer
- [ ] Password protected and public views
- [x] Mobile friendly vector tile maps

## Installation

### Pre-built binaries

We provide several options to access pre-built binaries for a variety of platforms. If you would like to manually download a pre-built binary, checkout [the latest release on GitHub](https://github.com/pka/owntrack-rs/releases/latest).

### Installer scripts

#### macOS and Linux:

```sh
curl --proto '=https' --tlsv1.2 -LsSf https://github.com/pka/owntrack-rs/releases/latest/download/owntrack-rs-installer.sh | sh
```

### Build From Source

For users who need to install owntrack-rs on platforms that we do not yet provide pre-built binaries for, you will need to build from source.
`owntrack-rs` is written in [Rust](https://rust-lang.org) and uses [cargo](https://doc.rust-lang.org/cargo/index.html) to build. Once you've [installed the Rust toolchain (`rustup`)](https://rustup.rs/), run:

```sh
cargo install --path .
```

## Setup & run

Run with default configuration:

```
owntrack-rs
```

### MQTT

For getting location data via MQTT, an MQTT broker like Mosquitto is required.

Add MQTT access configuration to `.env`:
```
cat >.env <<EOF
MQTT_URL="mqtts://owntracks.example:8883"
MQTT_USER=$(id -u)
MQTT_PASSWORD="xxx"
EOF
chmod 600 .env
edit .env
```

### SQLite database

Add connection information to `.env`:
```
cat >>.env <<EOF
DB_CONNECTION="sqlite://owntracks.sqlite"
EOF
```

### PostgreSQL database

Create database:
```
psql postgres -c "CREATE DATABASE owntracks"
```

Add connection information to `.env`:
```
cat >>.env <<EOF
# libpq connection string or PostgreSQL URI
DB_CONNECTION="postgres://user:pass@localhost:5432/owntracks"
EOF
```

## Setup tracking devices

### OwnTracks apps

The [OwnTracks](https://owntracks.org/booklet/) apps can be used in MQTT or in HTTP mode.

|  iOS   | Android |
| :----: | :-----: |
| [![AppStore](images/appstore.png)](https://apps.apple.com/us/app/owntracks/id692424691) | [![PlayStore](images/playstore.png)](https://play.google.com/store/apps/details?id=org.owntracks.android) |

- [Configure the Android app](https://owntracks.org/booklet/guide/app/android/)

### Use your own devices

Send a POST request to the `owntracks` endpoint:
```
curl --data '{"_type":"location","lat":48.856826,"lon":2.292713,"tid":"me","tst":'$(date +%s)'}' -H "Content-Type: application/json" "http://127.0.0.1:8083/owntracks?u=me&d=mydevice"
```

## Development

### Prerequisites

* Just: https://just.systems/man/en/

### Build and rund application

```
cargo run
```

### Frontend development

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
