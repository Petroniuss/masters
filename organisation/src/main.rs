mod permission_graph;

fn main() {
    println!("Master's");
}

// Organisation should be both an HTTP server so that other organisations can communicate with it
// and be an interactive CLI so that we can play with it easily.
// interactive CLI should be behind -i flag.

// let's start with creating a PoC representation of permissions graph.
// use-cases:
//    - organisation adds a user.
//    - organisation adds a group.
//    - organisation adds a space.
//    - organisation adds a permission.
//    - organisation publishes a new version of the graph on blockchain
//      to a hardcoded smart contract.
//    - verify if the graph is valid.
