FROM rust:1.35 as build

RUN USER=root cargo new --bin eve-gql
WORKDIR /eve-gql

COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml

# this build step will cache dependencies
RUN cargo build --release
RUN rm src/*.rs

COPY ./src ./src

RUN rm ./target/release/eve-gql
RUN cargo build --release

FROM debian:stable-slim

COPY --from=build /eve-gql/target/release/eve-gql .

CMD ["./eve-gql"]