FROM rust:bookworm AS builder

WORKDIR /build
RUN curl --proto '=https' --tlsv1.2 -sSf https://just.systems/install.sh | bash -s -- --to /usr/local/bin
#RUN curl --proto '=https' --tlsv1.2 -LsSf https://axodotdev.artifacts.axodotdev.host/cargo-dist/v0.28.0/cargo-dist-installer.sh
COPY . .

# Install DuckDB lib
RUN just getlib
ENV DUCKDB_LIB_DIR=/build/duckdb
ENV DUCKDB_INCLUDE_DIR=/build/duckdb

RUN cargo install --path .

# -- Runtime stage
FROM debian:bookworm-slim

COPY --from=builder /usr/local/cargo/bin/owntracks-duck-tape /usr/local/bin/owntracks-duck-tape
COPY --from=builder /build/duckdb/libduckdb.so /usr/lib/libduckdb.so

ENV DB_SCHEMA=public

# Run once to install DuckDB PostgreSQL extension in /root/.duckdb/extensions/
RUN touch .env; DB_CONNECTION="postgres:///dummy" owntracks-duck-tape || true

CMD ["owntracks-duck-tape"]
