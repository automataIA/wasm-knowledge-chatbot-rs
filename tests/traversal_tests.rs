use serde_json::json;
use wasm_knowledge_chatbot_rs::features::graphrag::traversal::{bfs, dfs, TraversalFilters};
use wasm_knowledge_chatbot_rs::models::graph_store::{GraphEdge, GraphNode, GraphStore};

fn make_store() -> GraphStore {
    let mut s = GraphStore::new();
    // Nodes A, B, C, D
    s.add_node(GraphNode {
        id: "A".into(),
        label: Some("A".into()),
        node_type: "entity".into(),
        source_document_id: None,
        metadata: json!({}),
    });
    s.add_node(GraphNode {
        id: "B".into(),
        label: Some("B".into()),
        node_type: "entity".into(),
        source_document_id: None,
        metadata: json!({}),
    });
    s.add_node(GraphNode {
        id: "C".into(),
        label: Some("C".into()),
        node_type: "entity".into(),
        source_document_id: None,
        metadata: json!({}),
    });
    s.add_node(GraphNode {
        id: "D".into(),
        label: Some("D".into()),
        node_type: "entity".into(),
        source_document_id: None,
        metadata: json!({}),
    });

    // Edges: A-B (rel1), B-C (rel2), C-D (rel1)
    s.add_edge(GraphEdge {
        id: "e1".into(),
        from: "A".into(),
        to: "B".into(),
        relation: "rel1".into(),
        weight: 1.0,
        metadata: json!({}),
    });
    s.add_edge(GraphEdge {
        id: "e2".into(),
        from: "B".into(),
        to: "C".into(),
        relation: "rel2".into(),
        weight: 1.0,
        metadata: json!({}),
    });
    s.add_edge(GraphEdge {
        id: "e3".into(),
        from: "C".into(),
        to: "D".into(),
        relation: "rel1".into(),
        weight: 1.0,
        metadata: json!({}),
    });

    s
}

#[test]
fn bfs_depth_limit() {
    let s = make_store();
    let filters = TraversalFilters {
        max_depth: Some(1),
        ..Default::default()
    };
    let res = bfs(&s, "A", &filters);
    // With depth 1 from A, should visit A and B, and only edge e1
    assert!(res.visited_nodes.contains(&"A".into()));
    assert!(res.visited_nodes.contains(&"B".into()));
    assert!(!res.visited_nodes.contains(&"C".into()));
    assert!(res.visited_edges.contains(&"e1".into()));
    assert!(!res.visited_edges.contains(&"e2".into()));
}

#[test]
fn dfs_relation_filter() {
    let s = make_store();
    let allowed = vec!["rel1".to_string()];
    let filters = TraversalFilters {
        allowed_relations: Some(&allowed),
        ..Default::default()
    };
    let res = dfs(&s, "A", &filters);
    // rel2 edge should be ignored; path goes A-B and C-D not reachable without rel2
    assert!(res.visited_nodes.contains(&"A".into()));
    assert!(res.visited_nodes.contains(&"B".into()));
    assert!(!res.visited_nodes.contains(&"C".into()));
    assert!(res.visited_edges.contains(&"e1".into()));
    assert!(!res.visited_edges.contains(&"e2".into()));
}

#[test]
fn bfs_node_edge_limits() {
    let s = make_store();
    let filters = TraversalFilters {
        max_nodes: Some(2),
        max_edges: Some(1),
        ..Default::default()
    };
    let res = bfs(&s, "A", &filters);
    assert!(res.visited_nodes.len() <= 2);
    assert!(res.visited_edges.len() <= 1);
}
