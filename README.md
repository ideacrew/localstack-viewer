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
cd ui && npm install
```

## How to Run Locally

**Note:** Each time you restart localstack it will wipe it's list of SMS messages.  You will need to create a new set of messages to have the web services work.  I recommend you use *awslocal* to create new messages (make sure localstack is running):
```
awslocal sns publish --phone-number "+15551234567" --message "Hello from LocalStack directly"
```

You can see the application at `localhost:8080`.

### With Itermocil (Recommended - you get a log for each window)

1. Install itermocil: `brew install TomAnthony/brews/itermocil`
2. Run: `itermocil`

### With Foreman

1. Install Foreman: `brew install foreman`
2. Run: `foreman start`

### The Hard Way

1. Start localstack
2. Start the backend: `cd server && LOCALSTACK_URL=http://localhost:4566 EVENT_SOURCE_AMQP_URL=amqp://guest:guest@localhost:5672 EVENT_SOURCE_VHOST=event_source cargo run`
3. Start tailwind: `cd ui && npx @tailwindcss/cli -i ./input.css -o ./assets/tailwind.css --watch`
3. Start the frontend: `cd ui && dx serve --port 8080`
