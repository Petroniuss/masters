use crate::core::protocol::IPFSEvent;
use crate::ipfs::ipfs_client::CID;
use crate::shared::shared;
use crate::transport::grpc::command::PermissionGraph;

use tokio::sync::mpsc::Sender;

/// Actual implementation will schedule IPFS RPC call and emit an event back to the protocol thread.
pub trait IPFSFacade: Send {
    fn async_load_permission_graph(&self, cid: CID, peerset_address: String);

    /// Context is passed during asynchronous callback
    fn async_save_permission_graph(
        &self,
        permission_graph: PermissionGraph,
        peerset_address: Option<String>,
    );
}

pub struct CheatingIPFSFacade {
    pub ipfs_sender: Sender<IPFSEvent>,
}

impl CheatingIPFSFacade {
    pub fn new(ipfs_sender: Sender<IPFSEvent>) -> Self {
        return Self { ipfs_sender };
    }
}

impl IPFSFacade for CheatingIPFSFacade {
    fn async_load_permission_graph(&self, cid: CID, peerset_address: String) {
        let sender = self.ipfs_sender.clone();
        tokio::spawn(async move {
            sender
                .send(IPFSEvent::PermissionGraphLoaded {
                    cid,
                    permission_graph: shared::demo_graph(),
                    peerset_address,
                })
                .await
                .unwrap();
        });
    }

    fn async_save_permission_graph(
        &self,
        permission_graph: PermissionGraph,
        peerset_address: Option<String>,
    ) {
        let sender = self.ipfs_sender.clone();
        tokio::spawn(async move {
            let cid = if !permission_graph.edges.contains_key("ur_2") {
                shared::demo_graph_cid()
            } else {
                shared::demo_graph_cid_2()
            };

            sender
                .send(IPFSEvent::PermissionGraphSaved {
                    cid,
                    peerset_address,
                })
                .await
                .unwrap();
        });
    }
}
