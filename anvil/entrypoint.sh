#!/bin/sh
echo "Starting anvil node on port 8545"
export RUST_LOG=info
anvil \
  --mnemonic "risk upset sort tank hazard ignore used clap unveil festival barrel wrap" \
  --block-time 15 \
  --host 0.0.0.0
