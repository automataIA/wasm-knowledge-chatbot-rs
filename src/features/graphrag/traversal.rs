use crate::models::graph_store::{GraphEdge, GraphStore};
use std::collections::{HashMap, HashSet, VecDeque};

#[derive(Debug, Clone, Default)]
pub struct TraversalFilters<'a> {
    pub allowed_relations: Option<&'a [String]>,
    pub max_depth: Option<usize>,
    pub max_nodes: Option<usize>,
    pub max_edges: Option<usize>,
}

#[derive(Debug, Clone)]
pub struct TraversalResult {
    pub visited_nodes: Vec<String>,
    pub visited_edges: Vec<String>,
}

fn build_adjacency(store: &GraphStore) -> HashMap<String, Vec<&GraphEdge>> {
    let mut adj: HashMap<String, Vec<&GraphEdge>> = HashMap::new();
    for e in &store.edges {
        adj.entry(e.from.clone()).or_default().push(e);
        adj.entry(e.to.clone()).or_default().push(e);
    }
    adj
}

fn relation_allowed(edge: &GraphEdge, filters: &TraversalFilters) -> bool {
    if let Some(allowed) = filters.allowed_relations.as_ref() {
        return allowed.iter().any(|r| r == &edge.relation);
    }
    true
}

pub fn bfs(store: &GraphStore, start_id: &str, filters: &TraversalFilters) -> TraversalResult {
    let adj = build_adjacency(store);
    let mut q: VecDeque<(String, usize)> = VecDeque::new();
    let mut visited_n: HashSet<String> = HashSet::new();
    let mut visited_e: HashSet<String> = HashSet::new();

    if !store.nodes.iter().any(|n| n.id == start_id) {
        return TraversalResult {
            visited_nodes: vec![],
            visited_edges: vec![],
        };
    }

    q.push_back((start_id.to_string(), 0));
    visited_n.insert(start_id.to_string());

    let max_depth = filters.max_depth.unwrap_or(usize::MAX);
    let max_nodes = filters.max_nodes.unwrap_or(usize::MAX);
    let max_edges = filters.max_edges.unwrap_or(usize::MAX);

    while let Some((nid, depth)) = q.pop_front() {
        if depth >= max_depth {
            continue;
        }
        if let Some(edges) = adj.get(&nid) {
            for e in edges {
                if !relation_allowed(e, filters) {
                    continue;
                }
                if visited_e.len() >= max_edges {
                    break;
                }
                let other = if e.from == nid { &e.to } else { &e.from };
                if visited_n.contains(other) && visited_e.contains(&e.id) {
                    continue;
                }
                visited_e.insert(e.id.clone());
                if visited_n.len() < max_nodes && visited_n.insert(other.clone()) {
                    q.push_back((other.clone(), depth + 1));
                }
            }
        }
        if visited_n.len() >= max_nodes {
            break;
        }
    }

    TraversalResult {
        visited_nodes: visited_n.into_iter().collect(),
        visited_edges: visited_e.into_iter().collect(),
    }
}

pub fn dfs(store: &GraphStore, start_id: &str, filters: &TraversalFilters) -> TraversalResult {
    let adj = build_adjacency(store);
    let mut stack: Vec<(String, usize)> = vec![(start_id.to_string(), 0)];
    let mut visited_n: HashSet<String> = HashSet::new();
    let mut visited_e: HashSet<String> = HashSet::new();

    if !store.nodes.iter().any(|n| n.id == start_id) {
        return TraversalResult {
            visited_nodes: vec![],
            visited_edges: vec![],
        };
    }

    let max_depth = filters.max_depth.unwrap_or(usize::MAX);
    let max_nodes = filters.max_nodes.unwrap_or(usize::MAX);
    let max_edges = filters.max_edges.unwrap_or(usize::MAX);

    while let Some((nid, depth)) = stack.pop() {
        if visited_n.len() >= max_nodes {
            break;
        }
        if !visited_n.insert(nid.clone()) {
            continue;
        }
        if depth >= max_depth {
            continue;
        }
        if let Some(edges) = adj.get(&nid) {
            for e in edges {
                if !relation_allowed(e, filters) {
                    continue;
                }
                if visited_e.len() >= max_edges {
                    break;
                }
                let other = if e.from == nid { &e.to } else { &e.from };
                if visited_n.contains(other) && visited_e.contains(&e.id) {
                    continue;
                }
                visited_e.insert(e.id.clone());
                stack.push((other.clone(), depth + 1));
            }
        }
    }

    TraversalResult {
        visited_nodes: visited_n.into_iter().collect(),
        visited_edges: visited_e.into_iter().collect(),
    }
}
