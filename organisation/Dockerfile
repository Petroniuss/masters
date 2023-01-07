# syntax=docker/dockerfile:1.4
FROM rust:1.66.0 AS build
WORKDIR /usr/masters/organisation

# We create a new lib and then use our own Cargo.toml
RUN cargo init --lib .

# Capture dependencies
COPY Cargo.toml Cargo.lock ./

# create dummy mains to build dependencies
RUN --mount=type=cache,target=~/.cargo \
    --mount=type=cache,target=/usr/masters/organisation/target <<EOF
    set -e
    echo "fn main() { println!(\"Hello, world!\"); }" > src/main.rs
    mkdir -p src/bin/start-the-world/
    echo "fn main() { println!(\"Hello, world!\"); }" > src/bin/start-the-world/main.rs
EOF

# build dependencies
RUN --mount=type=cache,target=~/.cargo \
    --mount=type=cache,target=/usr/masters/organisation/target \
    cargo build --release

# copy the actual sorce code
COPY . .

# install the binaries
RUN --mount=type=cache,target=~/.cargo \
    --mount=type=cache,target=/usr/masters/organisation/target \
    cargo install --path .

# final slim image
FROM frolvlad/alpine-glibc:alpine-3.17
WORKDIR /usr/local/bin

# note that we have multiple binaries in the same image.
COPY --from=build /usr/local/cargo/bin/organisation .
COPY --from=build /usr/local/cargo/bin/start-the-world .

CMD ["organisation"]
