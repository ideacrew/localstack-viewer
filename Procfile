localstack: localstack start
server: cd server && LOCALSTACK_URL=http://localhost:4566 EVENT_SOURCE_AMQP_URL=amqp://guest:guest@localhost:5672 EVENT_SOURCE_VHOST=event_source cargo run
ui-tailwind: cd ui && npx @tailwindcss/cli -i ./input.css -o ./assets/tailwind.css --watch
ui: cd ui && dx serve --port 8080
