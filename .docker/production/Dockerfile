FROM rust:alpine3.22 AS builder

RUN apk update && apk add pkgconfig openssl-dev musl-dev libcrypto3 libssl3 openssl-libs-static

RUN mkdir /build
COPY src /build/src
COPY templates /build/templates
COPY Cargo.lock /build/Cargo.lock
COPY Cargo.toml /build/Cargo.toml

RUN cd /build && cargo build --release

FROM alpine:3.22 AS deploy

RUN apk update && apk add parallel nginx

ENV ROCKET_ADDRESS="0.0.0.0"
ENV ROCKET_PORT="8000"
ENV LOCALSTACK_URL="http://localhost:4566"

RUN mkdir /localstack-viewer && mkdir /localstack-viewer/templates

COPY --from=builder /build/target/release/localstack-viewer /localstack-viewer/localstack-viewer
COPY templates /localstack-viewer/templates

COPY .docker/production/commands.txt /localstack-viewer/commands.txt
COPY .docker/production/nginx.conf /etc/nginx/nginx.conf

WORKDIR /localstack-viewer

CMD ["/usr/bin/parallel", "--will-cite", "-u", "-j", "2", "--halt", "now,fail=1", "-a", "/localstack-viewer/commands.txt"]
