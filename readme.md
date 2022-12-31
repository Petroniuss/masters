# Master's PoC

[Design on Excalidraw](https://excalidraw.com/#token=9wvvufCJTAaAYfN1Qjf9I)

## Dockerized Setup

So far only organisation is dockerized.
this could be helpful

## Local Dev Setup

### blockchain
For working with smart contracts I've chosen [foundry](https://github.com/foundry-rs/foundry). 
Follow the instructions to install foundry via foundryup:
- anvil (local Ethereum node, akin to Ganache)
- forge (Ethereum testing framework, like Truffle)

To start a local ethereum node, run:
```bash
anvil -m "risk upset sort tank hazard ignore used clap unveil festival barrel wrap"
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
forge bind --bindings-path ../organisation/src/bindings --module --overwrite
```

To deploy smart contracts to local anvil node, run:
```bash
forge create --unlocked --from 0xd13c4379bfc9a0ea5e147b2d37f65eb2400dfd7b src/PermissionGraph.sol:PermissionGraph
```

### ipfs
todo!
https://liyi-zhou.medium.com/the-complete-truffle-suite-on-docker-truffle-ganache-drizzle-47ab18b1ec83

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
