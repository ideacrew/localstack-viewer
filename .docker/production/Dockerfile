FROM rust:alpine3.22 AS base

RUN apk update && apk add pkgconfig build-base openssl-dev musl-dev libcrypto3 libssl3 openssl-libs-static

FROM base AS server-builder

RUN mkdir /build
COPY server/ /build/

RUN cd /build && cargo build --release

FROM base AS ui-builder

RUN rustup target add wasm32-unknown-unknown && cargo install wasm-bindgen-cli dioxus-cli

RUN mkdir /build
COPY ui/ /build/

RUN cd /build && NO_DOWNLOADS=1 dx bundle --platform web

FROM alpine:3.22 AS deploy

RUN apk update && apk add parallel nginx

ENV ROCKET_ADDRESS="0.0.0.0"
ENV ROCKET_PORT="8000"
ENV LOCALSTACK_URL="http://host.docker.internal:4566"

RUN mkdir /localstack-viewer && mkdir /localstack-viewer/templates

COPY --from=server-builder /build/target/release/localstack-viewer-server /localstack-viewer/server/localstack-viewer-server
COPY --from=ui-builder /build/target/dx/localstack-viewer-ui/release/web/public/ /localstack-viewer/ui/

COPY .docker/production/commands.txt /localstack-viewer/commands.txt
COPY .docker/production/nginx.conf /etc/nginx/nginx.conf

WORKDIR /localstack-viewer

CMD ["/usr/bin/parallel", "--will-cite", "-u", "-j", "2", "--halt", "now,fail=1", "-a", "/localstack-viewer/commands.txt"]
