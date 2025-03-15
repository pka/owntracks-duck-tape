# Owntrack-rs

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

## Development

### Prerequisites

* Just: https://just.systems/man/en/

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
