# syntax=docker/dockerfile:1.4
FROM rust:1.66.0 AS chef
WORKDIR /usr/masters/organisation
RUN cargo install cargo-chef

FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder
COPY --from=planner /usr/masters/organisation/recipe.json recipe.json

# Build dependencies - this is the caching Docker layer!
RUN --mount=type=cache,target=~/.cargo \
    --mount=type=cache,target=/usr/masters/organisation/target \
    cargo chef cook --release --recipe-path recipe.json

# Build application
COPY . .
RUN --mount=type=cache,target=~/.cargo \
    --mount=type=cache,target=/usr/masters/organisation/target \
    cargo build --release

FROM builder as test

RUN --mount=type=cache,target=~/.cargo \
    --mount=type=cache,target=/usr/masters/organisation/target \
    cargo test --release

FROM builder AS release

# install the binaries
RUN --mount=type=cache,target=~/.cargo \
    --mount=type=cache,target=/usr/masters/organisation/target \
    cargo install --path .

# final slim image
FROM frolvlad/alpine-glibc:alpine-3.17
WORKDIR /usr/local/bin

# note that we have multiple binaries in the same image.
COPY --from=release /usr/local/cargo/bin/organisation .
COPY --from=release /usr/local/cargo/bin/start-the-world .

CMD ["organisation"]
