DUCKDB_VERSION := "1.1.2"

run:
	DUCKDB_LIB_DIR=./duckdb DUCKDB_INCLUDE_DIR=./duckdb LD_LIBRARY_PATH=./duckdb cargo run

getlib:
	mkdir -p duckdb
	wget -O duckdb/libduckdb-linux-amd64.zip https://github.com/duckdb/duckdb/releases/download/v{{DUCKDB_VERSION}}/libduckdb-linux-amd64.zip
	cd duckdb && unzip libduckdb-linux-amd64.zip
