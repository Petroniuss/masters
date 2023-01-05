mod bindings;
mod permission_graph;
mod smart_contract;

fn main() {
    println!("Ala!");
}


// let's start with having a smart contract per peerset.
// this is just a start, we could also consider creating a single smart contract :/

// let's create some initial state.
// Let's create PeerSet1 with org1A, org1B
// and PeerSet2 with org2C


// one more thing:
// who's going to propose the changes?
// let's assume that an organisation proposes the changes on behalf of a user.



// Initial world-view.
// Smart contract per peerset?

// PeerSet:
// essentially a subgraph of the permission graph - controlled by a subset of organisations.
// For now let's assume that peerset is static - it cannot be changed after its creation.

// Let's start by creating a peerset.
// Let's create a peerset with a single organisation.
// Let's create a peerset with two organisations.


// Now do we have a smart contract per peerset?
// Or do we have a single smart contract that is shared by all peersets?



// One more question:
// How do we verify that a given organisation is a member of a given peerset?
// I guess we've to rely on public-private key pairs.


// How do verify adding an edge between peersets?
// For example I might have a space and a user associated with given space.

// and a different group in peerset2
// and I want to add an edge between them.

// space1 in peerset1
// group2 in peerset2


// If user that wants to add group2 to space1 has permissions to do so
// we should be able to validate that.

// And who is going to validate that?
// It seems that it should be peerset1
// so as peerset1 is validating that change it should hold that information.

// But where will this information be stored?
// Information that such an edge was added?.
// I think it should be stored in the smart contract of peerset1, because it is the one that is going to validate that change!
// All right enough chit chat :P

// Maybe if a graph is directed we can always


// let's start with creating a PoC representation of permissions graph.
// use-cases:
//    - organisation adds a user.
//    - organisation adds a group.
//    - organisation adds a space.
//    - organisation adds a permission.
//    - organisation publishes a new version of the graph on blockchain
//      to a hardcoded smart contract.
//    - verify if the graph is valid.
