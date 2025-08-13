# Localstack-Viewer

A very simple application to view the status and resources of an associated LocalStack installation.

Currently, used primarily for viewing (mock) SMS message traffic in a testing environment.

## Developer Setup

1. Install Docker Desktop
2. Install Localstack
3. Install dependencies:
```
rustup target add wasm32-unknown-unknown
cargo install wasm-bindgen-cli dioxus-cli
```

## How to Run Locally

1. Start localstack
2. Start the backend: `cd server && LOCALSTACK_URL=http://localhost:4566 cargo run`
3. Start the frontend: `cd ui && dx serve`
