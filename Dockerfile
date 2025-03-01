FROM rust:bookworm AS builder

WORKDIR /build
#RUN curl --proto '=https' --tlsv1.2 -sSf https://just.systems/install.sh | bash -s -- --to /usr/local/bin
#RUN curl --proto '=https' --tlsv1.2 -LsSf https://axodotdev.artifacts.axodotdev.host/cargo-dist/v0.28.0/cargo-dist-installer.sh
COPY . .

RUN cargo install --path .

# -- Runtime stage
FROM debian:bookworm-slim

COPY --from=builder /usr/local/cargo/bin/owntracks-duck-tape /usr/local/bin/owntracks-duck-tape

ENV DB_SCHEMA=public

CMD ["owntracks-duck-tape"]
