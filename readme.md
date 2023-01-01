# Master's PoC

![](https://github.com/petroniuss/masters/actions/workflows/ci.yml/badge.svg)

[Design on Excalidraw](https://excalidraw.com/#token=9wvvufCJTAaAYfN1Qjf9I)

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

To generate rust bindings for smart contracts, run:
```bash
forge bind \ 
  --bindings-path ../organisation/src/bindings \
  --module \
  --overwrite
```

To deploy smart contracts to local anvil node, run:
```bash
forge create --unlocked \
  --from 0xd13c4379bfc9a0ea5e147b2d37f65eb2400dfd7b \
  src/PermissionGraph.sol:PermissionGraph
```

### organisation
Off-chain code interacting with blockchain is written in [rust](https://www.rust-lang.org/).
To run the code, you need to install [rustup](https://rustup.rs/).
To control log level output use env variable `RUST_LOG` for example:
```bash
export RUST_LOG=INFO
```

To run the organisation, run:
```bash
cargo run --release
```

To run unit tests, run:
```bash
cargo test
```

Integration tests are marked as `#[ignored]`. To run integration tests, run:
```bash
export RUST_LOG=INFO && cargo test -- --ignored --nocapture
```

To run application via docker, run:
```bash
docker build -t organisation .
dokcer run organisation
```

### ipfs

### Miscellaneous
