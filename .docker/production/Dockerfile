FROM rust:alpine3.22 AS base

RUN apk update && apk add pkgconfig build-base openssl-dev musl-dev libcrypto3 libssl3 openssl-libs-static

FROM base AS server-builder

RUN mkdir /build
COPY data/ /build/data/
COPY server/ /build/server/

RUN cd /build/server && cargo build --release

# I want to use alpine here, but dioxus doesn't have a binary yet
FROM rust:bookworm AS ui-builder

RUN apt update && apt-get -y install pkg-config libssl-dev build-essential curl
RUN rustup target add wasm32-unknown-unknown
RUN curl -L --proto '=https' --tlsv1.2 -sSf https://raw.githubusercontent.com/cargo-bins/cargo-binstall/main/install-from-binstall-release.sh | bash
RUN cargo binstall wasm-bindgen-cli@0.2.101
RUN cargo binstall dioxus-cli

RUN mkdir /build
COPY ui/ /build/ui/
COPY data/ /build/data/

RUN cd /build/ui && NO_DOWNLOADS=1 dx bundle --platform web

FROM alpine:3.22 AS deploy

RUN apk update && apk upgrade && apk add parallel nginx

ENV ROCKET_ADDRESS="0.0.0.0"
ENV ROCKET_PORT="8000"
ENV LOCALSTACK_URL="http://host.docker.internal:4566"
ENV EVENT_SOURCE_AMQP_URL=""
ENV EVENT_SOURCE_VHOST=""

RUN mkdir /localstack-viewer && mkdir /localstack-viewer/templates

COPY --from=server-builder /build/server/target/release/localstack-viewer-server /localstack-viewer/server/localstack-viewer-server
COPY --from=ui-builder /build/ui/target/dx/localstack-viewer-ui/release/web/public/ /localstack-viewer/ui/

COPY .docker/production/commands.txt /localstack-viewer/commands.txt
COPY .docker/production/nginx.conf /etc/nginx/nginx.conf

RUN chown -R nginx:nginx /localstack-viewer

USER nginx

WORKDIR /localstack-viewer

CMD ["/usr/bin/parallel", "--will-cite", "-u", "-j", "2", "--halt", "now,fail=1", "-a", "/localstack-viewer/commands.txt"]
