FROM rust:1.66.0 AS build
WORKDIR /usr/src/

# create an empty project
RUN USER=root cargo new organisation
WORKDIR /usr/src/organisation

# build dependencies
COPY Cargo.toml Cargo.lock ./
RUN cargo build

# intall app
COPY src ./src
RUN cargo install --path .

FROM build AS application
WORKDIR /usr/local/bin
COPY --from=build /usr/local/cargo/bin/organisation .

CMD ["organisation"]
