DUCKDB_VERSION := "1.1.2"

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
