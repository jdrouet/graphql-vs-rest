FROM rust:alpine AS builder

RUN apk add --no-cache musl-dev

WORKDIR /code

COPY Cargo.toml /code/

COPY common/Cargo.toml /code/common/
COPY common/src /code/common/src

COPY graphql-api/Cargo.toml /code/graphql-api/
COPY graphql-api/src /code/graphql-api/src

COPY rest-api/Cargo.toml /code/rest-api/
COPY rest-api/src /code/rest-api/src

RUN cargo build --release

FROM alpine AS graphql-api

COPY --from=builder /code/target/release/graphql-api /usr/bin/graphql-api

ENTRYPOINT [ "/usr/bin/graphql-api" ]

FROM alpine AS rest-api

COPY --from=builder /code/target/release/rest-api /usr/bin/rest-api

ENTRYPOINT [ "/usr/bin/rest-api" ]