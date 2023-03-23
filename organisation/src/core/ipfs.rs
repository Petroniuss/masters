use crate::ipfs::ipfs_client::CID;

/// Actual implementation will schedule IPFS RPC call and emit an event back to the protocol thread.
pub trait IPFSFacade: Send {
    fn load_permission_graph(&self, cid: CID, peerset_address: String);
}
