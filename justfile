set dotenv-load

DUCKDB_VERSION := "1.1.2"
DB_NAME := "owntracks"

# Build & run in debug mode
run:
	test -f ./duckdb/libduckdb.so || just getlib
	DUCKDB_LIB_DIR=./duckdb DUCKDB_INCLUDE_DIR=./duckdb LD_LIBRARY_PATH=./duckdb cargo run

# Release build
build:
    cargo build --release --features bundled

# Download DuckDB shared library
getlib:
	mkdir -p duckdb
	wget -O duckdb/libduckdb-linux-amd64.zip https://github.com/duckdb/duckdb/releases/download/v{{DUCKDB_VERSION}}/libduckdb-linux-amd64.zip
	cd duckdb && unzip libduckdb-linux-amd64.zip

# Create local database
create-db:
    psql postgres -c "DROP DATABASE IF EXISTS {{DB_NAME}}"
    psql postgres -c "CREATE DATABASE {{DB_NAME}}"
    psql {{DB_NAME}} -c "CREATE SCHEMA IF NOT EXISTS ${DB_SCHEMA}"

# Migrate DB schema with refinery CLI
migrate:
    refinery migrate -e DB_CONNECTION -p ./migrations
