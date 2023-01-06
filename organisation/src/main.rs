mod bindings;
mod permission_graph;
mod smart_contract;

fn main() {
    println!("Ala!");
}

// Goal for today:
// Let's now write the off-chain code that interacts with the smart contract
// Things that we need:
// - deploy multiple smart contracts but initialized with different arguments
// - interact with the smart contract:
//   - propose a change
//   - validate the change

// Stretch:
// - off-chain code assembling knowledge from multiple smart contracts,
// - that's able to respond to queries whether given operation is allowed.
