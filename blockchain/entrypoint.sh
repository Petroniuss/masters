#!/bin/sh
echo "Deploying smart contract."
forge create --unlocked \
  --from 0xd13c4379bfc9a0ea5e147b2d37f65eb2400dfd7b \
  src/PermissionGraph.sol:PermissionGraph
