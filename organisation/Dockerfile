FROM rust:1.66.0 AS build
WORKDIR /usr/src/

RUN rustup target add x86_64-unknown-linux-musl

# create an empty project
RUN USER=root cargo new organisation
WORKDIR /usr/src/organisation

# build dependencies
COPY Cargo.toml Cargo.lock ./
RUN cargo build --target x86_64-unknown-linux-musl --release

# intall app
RUN rm ./src/main.rs
COPY ./src/* ./src/
RUN cargo install --target x86_64-unknown-linux-musl --path .

FROM alpine:3.17
WORKDIR /usr/local/bin
COPY --from=build /usr/local/cargo/bin/organisation .

CMD ["organisation"]
