FROM rust:1.66.0 AS build
WORKDIR /usr/src/

# create an empty project
RUN USER=root cargo new organisation
WORKDIR /usr/src/organisation

# build dependencies
COPY Cargo.toml Cargo.lock ./
RUN cargo build --release

# intall app
RUN rm ./src/main.rs
COPY ./src/* ./src/
RUN cargo install --path .

# final slim image
FROM frolvlad/alpine-glibc:alpine-3.17
WORKDIR /usr/local/bin
COPY --from=build /usr/local/cargo/bin/organisation .

CMD ["organisation"]
