set dotenv-load

DUCKDB_VERSION := "1.1.1" # 1.2.0 fails linking on Ubuntu 22.04
DB_NAME := "owntracks"

# Build & run in debug mode
run:
	test -f ./duckdb/libduckdb.so || just getlib
	DUCKDB_LIB_DIR=./duckdb DUCKDB_INCLUDE_DIR=./duckdb LD_LIBRARY_PATH=./duckdb cargo run

# Build package
build-dist:
    docker run --rm -v $PWD:/build -u $(id -u):$(id -g) rust:bullseye bash -c "curl --proto '=https' --tlsv1.2 -LsSf https://axodotdev.artifacts.axodotdev.host/cargo-dist/v0.28.0/cargo-dist-installer.sh | sh; cd /build; nice dist build"

# Build package for architecture
build-dist-arch arch="aarch64-unknown-linux-musl":
    nice dist build --target {{arch}}

# Local release build
build-cargo-dist:
    nice cargo build --profile dist --features bundled

# Build Docker image
docker-build:
    docker build -t sourcepole/owntracks-duck-tape .

# Download DuckDB shared library
getlib arch="linux-amd64":
	mkdir -p duckdb
	wget -O duckdb/libduckdb-{{arch}}.zip https://github.com/duckdb/duckdb/releases/download/v{{DUCKDB_VERSION}}/libduckdb-{{arch}}.zip
	cd duckdb && unzip libduckdb-{{arch}}.zip

# Connection with DuckDB CLI
duckdb:
    duckdb -cmd "ATTACH '$DB_CONNECTION' AS db (TYPE postgres); SET search_path = 'db.$DB_SCHEMA';"

user := env('MQTT_USER', 'nobody')
device := "mockup"
payload := '{"_type":"location","t":"u","batt":11,"bs":0,"lat":48.856826,"lon":2.292713,"tid":"'+\
    user+'","tst":'+`date +%s`+',"topic":"owntracks/'+user+"/"+device+'","_id":"0"}'

# Test call for JSON endpoint
call:
    curl --data '{{payload}}' -H "Content-Type: application/json" "http://127.0.0.1:8083/owntracks?u={{user}}&d={{device}}"

# Create local database
create-db:
    psql postgres -c "DROP DATABASE IF EXISTS {{DB_NAME}}"
    psql postgres -c "CREATE DATABASE {{DB_NAME}}"
    psql {{DB_NAME}} -c "CREATE SCHEMA IF NOT EXISTS ${DB_SCHEMA}"

# Migrate DB schema with refinery CLI
migrate:
    refinery migrate -e DB_CONNECTION -p ./migrations
