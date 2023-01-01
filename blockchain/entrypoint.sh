#!/bin/sh

while ! cast chain-id ; do
  echo "Waiting for anvil to become active..."
  sleep 1
done

echo "Anvil is running.. Deploying smart contract."
forge create --unlocked \
  --from 0xd13c4379bfc9a0ea5e147b2d37f65eb2400dfd7b \
  src/PermissionGraph.sol:PermissionGraph
