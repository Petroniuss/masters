# Master's PoC

<p align="center">
    <img width="200" src="./imgs/ferris.png" alt="Ferris">
</p>

## Blockchain

For developing smart contracts I've chosen [foundry](https://github.com/foundry-rs/foundry).
Follow the instructions to install foundry via foundryup:

- anvil (local Ethereum node, akin to Ganache)
- forge (Ethereum testing framework, like Truffle)

To run anvil, run:

```bash
docker-compose up 
```

To compile smart contracts, run:

```bash
forge build
```

To test smart contracts, run:

```bash
forge test 
```

To test smart contracts and see logs, gas usage etc, run:

```bash
forge test -vvvvv
```

To generate rust bindings for smart contracts, run:

```bash
forge bind \
  --bindings-path ../organisation/src/transport/ethereum \
  --module \
  --overwrite
```

## Organisation

To implement a prototype of an organisation/peer, I've chosen [rust](https://www.rust-lang.org/).

There's a neat little script that runs a couple of peers and a coordinator simultaneously. It requires tmux.

```bash
./run-dev.sh
```

To run unit tests, run:

```bash
cargo test
```

To run integration tests, run:

```bash
cargo integration-tests
```

To generate gRPC code, run:

```bash
cargo build --features gen-proto
```

To control log level output use env variable `RUST_LOG` for example:

```bash
export RUST_LOG=INFO
```

To control backtrace level use env variable `RUST_BACKTRACE` for example:

```bash
export RUST_BACKTRACE=full
```

To run application via docker, run:

```bash
docker build -t organisation .
dokcer run organisation
```

For building docker images, I use [buildkit](https://docs.docker.com/build/buildkit/).
To enable it:

```bash
export DOCKER_BUILDKIT=1
```
