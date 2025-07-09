FROM rust:alpine3.22 AS builder

RUN apk update && apk add pkgconfig openssl-dev musl-dev libcrypto3 libssl3 openssl-libs-static

RUN mkdir /build
COPY . /build

RUN cd /build && cargo build --release

FROM alpine:3.22 AS deploy

ENV ROCKET_ADDRESS="0.0.0.0"
ENV ROCKET_PORT="8000"
ENV LOCALSTACK_URL="http://localhost:4566"

RUN mkdir /localstack-viewer && mkdir /localstack-viewer/templates

COPY --from=builder /build/templates /localstack-viewer/templates
COPY --from=builder /build/target/release/localstack-viewer /localstack-viewer/localstack-viewer

WORKDIR /localstack-viewer

CMD ["./localstack-viewer"]
