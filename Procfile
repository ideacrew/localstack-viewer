localstack: localstack start
server: cd server && LOCALSTACK_URL=http://localhost:4566 cargo run
ui-tailwind: cd ui && npx @tailwindcss/cli -i ./input.css -o ./assets/tailwind.css --watch
ui: cd ui && dx serve
