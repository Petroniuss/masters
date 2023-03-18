# Master's PoC

<p align="center">
    <img width="200" src="./imgs/ferris.png" alt="Ferris">
</p>

[Design on Excalidraw](https://excalidraw.com/#token=9wvvufCJTAaAYfN1Qjf9I)

## What's implemented?

- [blockchain](./blockchain)
  - [x] the simplest possible implementation of [peer-set smart contract](./blockchain/src/peer-set)
    and an [oracle](./blockchain/src/oracle).
  - [x] [peer-broadcast](./blockchain/src/peer-broadcast) - smart contract for registering peers and peersets.
  - [x] [test suite](./blockchain/test/PeerSet.t.sol) for the peer-set smart contract and oracle.
- [organisation](./organisation)
    - [start-the-world](./organisation/src/bin/start-the-world):
        - [x] deployment of oracle smart contract.
        - [x] deployment of peer-broadcast smart contract.
    - [register-peer-set](./organisation/src/bin/register-peer-set):
        - [x] broadcasting itself via peer-broadcast smart contract.
        - [x] deployment of peer-set smart contract.
    - [propose-change](./organisation/src/bin/propose-change):
        - [x] propose a change to existing peer-set smart contract.
    - [validate-change](./organisation/src/bin/validate-change):
        - [x] query oracle for validation requests.
        - [x] validate a change to existing peer-set smart contract.
    - [ ] listening to events from the peer-set smart contracts to build a local view of the permission graph.
    - [ ] listening to events from the oracle to validate results.
    - [ ] validating incoming permission graph change requests:
        - [ ] simple permission model - as close as possible to what OneData already has.

## Local Dev Setup

### blockchain

For developing smart contracts I've chosen [foundry](https://github.com/foundry-rs/foundry).
Follow the instructions to install foundry via foundryup:

- anvil (local Ethereum node, akin to Ganache)
- forge (Ethereum testing framework, like Truffle)

There exists a docker-compose that runs an anvil node
and deploys the smart contracts.

```bash
docker-compose up --build
```

To start a local ethereum node, run:

```bash
anvil \
  --mnemonic "risk upset sort tank hazard ignore used clap unveil festival barrel wrap"
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
  --bindings-path ../organisation/src/bindings \
  --module \
  --overwrite
```

To generate gRPC code, run:

```bash
cargo build --release --features gen-proto
```

To deploy smart contracts to local anvil node, run:

```bash
forge create --unlocked \
  --from 0xd13c4379bfc9a0ea5e147b2d37f65eb2400dfd7b \
  src/PermissionGraph.sol:PermissionGraph
```

### organisation

Off-chain code interacting with blockchain is written in [rust](https://www.rust-lang.org/).

#### miscellaneous

As an introduction material for learning Rust I
recommend [The Rust Programming Language](https://doc.rust-lang.org/book/title-page.html).

To run the code, you need to install [rustup](https://rustup.rs/).

Code is organised according to [Package Layout](https://doc.rust-lang.org/cargo/guide/project-layout.html)
from [The Cargo Book](https://doc.rust-lang.org/cargo/index.html).

To control log level output use env variable `RUST_LOG` for example:

```bash
export RUST_LOG=INFO
```

To control backtrace level use env variable `RUST_BACKTRACE` for example:

```bash
export RUST_BACKTRACE=full
```

To run start-the-world, run:

```bash
cargo run --bin start-the-world
```

To run unit tests, run:

```bash
cargo test
```

Integration tests are marked as `#[ignored]` and have 'integration_test' suffix.
To run integration tests, run:

```bash
cargo integration-tests
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
