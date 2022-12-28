# Master's PoC

[Design on Excalidraw](https://excalidraw.com/#token=9wvvufCJTAaAYfN1Qjf9I)

## Dockerized Setup

So far only organisation is dockerized.
truffle, solidity, ganache, ipfs..

this could be helpful
https://liyi-zhou.medium.com/the-complete-truffle-suite-on-docker-truffle-ganache-drizzle-47ab18b1ec83

## Local Dev Setup

### blockchain
todo install truffle, solidity, ganache.
```bash
```

### organisation
todo: dev dependencies - install rust, cargo task.

To generate rust bindings for the smart contracts, run:
```bash
cargo task gen-bindings 
```

To run the organisation, run:
```bash
cargo run --release
```

To run unit tests, run:
```bash
cargo test -- --nocapture
```

### Miscellaneous
