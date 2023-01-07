use ethers::types::Address;

#[derive(Clone, Debug)]
pub struct PeerSet {
    pub peers: Vec<Peer>,
}

#[derive(Clone, Debug)]
pub struct Peer {
    pub ethereum_address: Address,
}

impl PeerSet {
    pub(crate) fn get_peer_ethereum_addresses(
        &self,
    ) -> Vec<Address> {
        self.peers
            .iter()
            .map(|peer| peer.ethereum_address)
            .collect()
    }
}
