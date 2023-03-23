use crate::core::protocol::IPFSEvent;
use crate::errors::Result;
use crate::grpc::command::{
    Edge, Edges, Node, NodeType, PermissionGraph,
};
use crate::ipfs::ipfs_client::CID;
use std::collections::HashMap;

pub trait IPFSFacade: Send {
    fn load_permission_graph(
        &self,
        cid: CID,
        peerset_address: String,
    );
}

pub struct IPFSFacadeMock {
    pub sender: tokio::sync::mpsc::Sender<IPFSEvent>,
}

/// Actual implementation will schedule IPFS RPC call and emit an event back to the protocol thread.
impl IPFSFacade for IPFSFacadeMock {
    fn load_permission_graph(
        &self,
        cid: CID,
        peerset_address: String,
    ) {
        self.sender.send(IPFSEvent::PermissionGraphLoaded {
            cid,
            permission_graph: test_graph_with_user_and_group(),
            peerset_address,
        });
    }
}

fn test_graph_with_user_and_group() -> PermissionGraph {
    return PermissionGraph {
        edges: HashMap::from([
            (
                "ur_1".to_string(),
                Edges {
                    source: Some(Node {
                        id: "ur_1".to_string(),
                        r#type: NodeType::User as i32,
                        peerset_address: None,
                    }),
                    edges: vec![Edge {
                        destination_node_id: "gr_1".to_string(),
                        permission: "belongs".to_string(),
                    }],
                },
            ),
            (
                "gr_1".to_string(),
                Edges {
                    source: Some(Node {
                        id: "gr_1".to_string(),
                        r#type: NodeType::User as i32,
                        peerset_address: None,
                    }),
                    edges: vec![],
                },
            ),
        ]),
    };
}
