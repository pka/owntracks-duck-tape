set dotenv-load

DUCKDB_VERSION := "1.1.1" # Current version is 1.1.3, but libduckdb-sys crate is 1.1.1
DB_NAME := "owntracks"

# Build & run in debug mode
run:
	test -f ./duckdb/libduckdb.so || just getlib
	DUCKDB_LIB_DIR=./duckdb DUCKDB_INCLUDE_DIR=./duckdb LD_LIBRARY_PATH=./duckdb cargo run

# Release package
dist arch="aarch64-unknown-linux-musl":
    nice dist build --target {{arch}}

# Download DuckDB shared library
getlib arch="linux-amd64":
	mkdir -p duckdb
	wget -O duckdb/libduckdb-{{arch}}.zip https://github.com/duckdb/duckdb/releases/download/v{{DUCKDB_VERSION}}/libduckdb-{{arch}}.zip
	cd duckdb && unzip libduckdb-{{arch}}.zip

# Create local database
create-db:
    psql postgres -c "DROP DATABASE IF EXISTS {{DB_NAME}}"
    psql postgres -c "CREATE DATABASE {{DB_NAME}}"
    psql {{DB_NAME}} -c "CREATE SCHEMA IF NOT EXISTS ${DB_SCHEMA}"

# Migrate DB schema with refinery CLI
migrate:
    refinery migrate -e DB_CONNECTION -p ./migrations
