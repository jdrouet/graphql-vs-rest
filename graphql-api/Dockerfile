FROM rust:alpine AS builder

RUN apk add --no-cache musl-dev

WORKDIR /code

COPY Cargo.toml Cargo.lock /code/
COPY src /code/src

RUN cargo build --release

FROM alpine

COPY --from=builder /code/target/release/graphql-api /usr/bin/graphql-api

ENTRYPOINT [ "/usr/bin/graphql-api" ]