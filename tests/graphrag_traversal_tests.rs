use wasm_bindgen_test::wasm_bindgen_test as test;
use wasm_bindgen_test::*;

use serde_json::json;
use wasm_knowledge_chatbot_rs::features::graphrag::traversal::{bfs, dfs, TraversalFilters};
use wasm_knowledge_chatbot_rs::models::graph_store::{GraphEdge, GraphNode, GraphStore};

wasm_bindgen_test_configure!(run_in_browser);

fn node(id: &str, label: &str) -> GraphNode {
    GraphNode {
        id: id.into(),
        label: Some(label.into()),
        node_type: "entity".into(),
        source_document_id: None,
        metadata: json!({}),
    }
}

fn edge(id: &str, from: &str, to: &str, rel: &str) -> GraphEdge {
    GraphEdge {
        id: id.into(),
        from: from.into(),
        to: to.into(),
        relation: rel.into(),
        weight: 1.0,
        metadata: json!({}),
    }
}

fn small_store() -> GraphStore {
    // Graph: A -is_a-> B, A -works_at-> C, B -related-> D
    let mut s = GraphStore::new();
    s.add_node(node("A", "A"));
    s.add_node(node("B", "B"));
    s.add_node(node("C", "C"));
    s.add_node(node("D", "D"));
    s.add_edge(edge("e1", "A", "B", "is_a"));
    s.add_edge(edge("e2", "A", "C", "works_at"));
    s.add_edge(edge("e3", "B", "D", "related"));
    s
}

#[test]
fn bfs_with_depth_and_relation_filters() {
    let store = small_store();
    // Allow only is_a edges, depth 1 from A should visit A and B
    let filters = TraversalFilters {
        allowed_relations: Some(&["is_a".to_string()]),
        max_depth: Some(1),
        max_nodes: None,
        max_edges: None,
    };
    let res = bfs(&store, "A", &filters);
    assert!(res.visited_nodes.contains(&"A".to_string()));
    assert!(res.visited_nodes.contains(&"B".to_string()));
    assert!(!res.visited_nodes.contains(&"C".to_string()));
    assert!(!res.visited_nodes.contains(&"D".to_string()));
}

#[test]
fn dfs_with_limits() {
    let store = small_store();
    // No relation filter, but limit nodes and edges to 2 beyond start
    let filters = TraversalFilters {
        allowed_relations: None,
        max_depth: Some(3),
        max_nodes: Some(3),
        max_edges: Some(2),
    };
    let res = dfs(&store, "A", &filters);
    // Should include start and up to two others
    assert!(res.visited_nodes.contains(&"A".to_string()));
    assert!(res.visited_nodes.len() <= 3);
    assert!(res.visited_edges.len() <= 2);
}
