use crate::core::ipfs::CID;
use crate::errors::Result;
use crate::transport::grpc::command::{Edge, Edges, Node, NodeType, PermissionGraph};
use std::collections::HashMap;

pub static ORGANISATION_ONE_ADDR: &'static str = "0xd13c4379bfc9a0ea5e147b2d37f65eb2400dfd7b";
pub static ORGANISATION_TWO_ADDR: &'static str = "0xd248e4a8407ed7ff9bdbc396ba46723b8101c86e";

pub fn init() -> Result<()> {
    color_eyre::install()?;
    pretty_env_logger::try_init()?;
    Ok(())
}
pub fn demo_graph() -> PermissionGraph {
    PermissionGraph {
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
    }
}

pub fn demo_graph_cid() -> CID {
    return "ipfs://cid--1".to_string();
}

pub fn demo_graph_cid_2() -> CID {
    return "ipfs://cid--2".to_string();
}
