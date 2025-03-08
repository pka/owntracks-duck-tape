set dotenv-load

DB_NAME := "owntracks"

# Build & run in debug mode
run:
	cargo run

# Start frontend in dev mode
ui:
    cd frontend && PUBLIC_BASE_URL=http://127.0.0.1:8083 npm run dev

# Build frontend in production mode
build-ui:
    cd frontend && PUBLIC_BASE_URL="" npm run build
    rm -rf static; mkdir static
    cp -r frontend/.svelte-kit/output/prerendered/pages/* static/
    cp -r frontend/.svelte-kit/output/client/* static/
    git add static
    touch src/http.rs

# Build package
build-dist:
    docker run --rm -v $PWD:/build -u $(id -u):$(id -g) rust:bullseye bash -c "curl --proto '=https' --tlsv1.2 -LsSf https://axodotdev.artifacts.axodotdev.host/cargo-dist/v0.28.0/cargo-dist-installer.sh | sh; cd /build; nice dist build"

# Build package for architecture
build-dist-arch arch="aarch64-unknown-linux-musl":
    nice dist build --target {{arch}}

# Local release build
build-cargo-dist:
    nice cargo build --profile dist

# Build Docker image
docker-build:
    docker build -t sourcepole/owntrack-rs .

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
