FROM rust:alpine AS builder

RUN apk add --no-cache musl-dev

WORKDIR /code

COPY Cargo.toml Cargo.lock /code/
COPY src /code/src

RUN cargo build --release

FROM alpine

COPY --from=builder /code/target/release/rest-api /usr/bin/rest-api

ENTRYPOINT [ "/usr/bin/rest-api" ]