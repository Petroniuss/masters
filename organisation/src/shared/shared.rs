use crate::errors::Result;
use crate::transport::grpc::command::{Edge, Edges, Node, NodeType, PermissionGraph};
use std::collections::hash_map::Entry;
use std::collections::HashMap;

pub static ORGANISATION_ONE_ADDR: &'static str = "0xd13c4379bfc9a0ea5e147b2d37f65eb2400dfd7b";
pub static ORGANISATION_TWO_ADDR: &'static str = "0xd248e4a8407ed7ff9bdbc396ba46723b8101c86e";

pub fn init() -> Result<()> {
    color_eyre::install()?;
    pretty_env_logger::try_init()?;
    Ok(())
}
pub fn demo_graph_p1_v1() -> PermissionGraph {
    PermissionGraph {
        edges: HashMap::from([
            (
                "ps_1_ur_1".to_string(),
                Edges {
                    source: Some(Node {
                        id: "ps_1_ur_1".to_string(),
                        r#type: NodeType::User as i32,
                        peerset_address: None,
                    }),
                    edges: vec![Edge {
                        destination_node_id: "ps_1_gr_1".to_string(),
                        permission: "belongs".to_string(),
                    }],
                },
            ),
            (
                "ps_1_gr_1".to_string(),
                Edges {
                    source: Some(Node {
                        id: "ps_1_gr_1".to_string(),
                        r#type: NodeType::User as i32,
                        peerset_address: None,
                    }),
                    edges: vec![],
                },
            ),
        ]),
    }
}

pub fn demo_graph_p1_v2() -> PermissionGraph {
    let mut graph = demo_graph_p1_v1();

    graph.edges.insert(
        "ps_1_ur_2".to_string(),
        Edges {
            source: Some(Node {
                id: "ps_1_ur_2".to_string(),
                r#type: NodeType::User as i32,
                peerset_address: None,
            }),
            edges: vec![Edge {
                destination_node_id: "ps_1_gr_1".to_string(),
                permission: "belongs".to_string(),
            }],
        },
    );

    graph
}

pub fn demo_graph_p2_v1() -> PermissionGraph {
    PermissionGraph {
        edges: HashMap::from([(
            "ps_2_ur_1".to_string(),
            Edges {
                source: Some(Node {
                    id: "ps_2_ur_1".to_string(),
                    r#type: NodeType::User as i32,
                    peerset_address: None,
                }),
                edges: vec![],
            },
        )]),
    }
}

/// adds a user ps_2_ur_1 to group ps_1_gr_1 from peerset_1
pub fn demo_graph_p2_cross_peerset_v2(ps1_address: String) -> PermissionGraph {
    let mut graph = demo_graph_p2_v1();

    // add a node to graph to indicate that this entry is owned by a different peerset
    graph.edges.insert(
        "ps_1_gr_1".to_string(),
        Edges {
            source: Some(Node {
                id: "ps_1_gr_1".to_string(),
                r#type: NodeType::Group as i32,
                peerset_address: Some(ps1_address),
            }),
            edges: vec![],
        },
    );

    // add user to group
    match graph.edges.entry("ps_2_ur_1".to_string()) {
        Entry::Occupied(mut user) => {
            user.get_mut().edges.push(Edge {
                destination_node_id: "ps_1_gr_1".to_string(),
                permission: "belongs".to_string(),
            });
        }
        _ => panic!(),
    }

    graph
}

pub fn demo_graph_p1_cross_peerset_v2(ps2_address: String) -> PermissionGraph {
    let mut graph = demo_graph_p1_v1();

    // add a user owned by peerset 2, and an edge from that user to group ps_1_gr_1
    graph.edges.insert(
        "ps_2_ur_1".to_string(),
        Edges {
            source: Some(Node {
                id: "ps_2_ur_1".to_string(),
                r#type: NodeType::User as i32,
                peerset_address: Some(ps2_address),
            }),
            edges: vec![Edge {
                destination_node_id: "ps_1_gr_1".to_string(),
                permission: "belongs".to_string(),
            }],
        },
    );

    graph
}
