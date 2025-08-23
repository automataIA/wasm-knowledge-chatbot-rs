use serde::{Deserialize, Serialize};
use crate::pagerank_reranking::GraphAccess;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct HyDEConfig {
    pub num_docs: usize,
    pub max_length: usize,
    pub similarity_threshold: f32,
}

impl Default for HyDEConfig {
    fn default() -> Self {
        Self {
            num_docs: 3,
            max_length: 512,
            similarity_threshold: 0.3,
        }
    }
}

#[derive(Clone, Debug)]
pub struct HyDEEngine {
    pub config: HyDEConfig,
}

impl HyDEEngine {
    pub fn new(config: HyDEConfig) -> Self {
        Self { config }
    }

    pub fn generate_hypothetical_docs(&self, query: &str) -> Vec<String> {
        // Generate hypothetical documents for search enhancement (internal use only)
        let mut out = Vec::with_capacity(self.config.num_docs);
        for i in 0..self.config.num_docs {
            // Create variations of the query for better search matching
            let variant = match i {
                0 => format!("Question: {}", query),
                1 => format!("Answer: {}", query),
                _ => format!("Context: {}", query),
            };
            
            let s = if variant.len() > self.config.max_length {
                variant[..self.config.max_length].to_string()
            } else {
                variant
            };
            out.push(s);
        }
        out
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CommunityDetectionConfig {
    pub resolution: f32,
    pub max_iterations: usize,
    pub stability_threshold: f32,
    /// Optional initial labels to seed LPA; if provided and length matches node_count, used as starting labels
    pub seed_labels: Option<Vec<usize>>,
}

impl Default for CommunityDetectionConfig {
    fn default() -> Self {
        Self {
            resolution: 1.0,
            max_iterations: 50,
            stability_threshold: 1e-4,
            seed_labels: None,
        }
    }
}

#[derive(Clone, Debug)]
pub struct CommunityDetectionEngine {
    pub config: CommunityDetectionConfig,
}

impl CommunityDetectionEngine {
    pub fn new(config: CommunityDetectionConfig) -> Self {
        Self { config }
    }

    pub fn detect_communities<G: GraphAccess>(&self, graph: &G) -> Vec<Vec<usize>> {
        let n = graph.node_count();
        if n == 0 {
            return vec![];
        }

        // Label Propagation Algorithm (LPA) — simplistic, deterministic iteration order
        let mut labels: Vec<usize> = if let Some(seeds) = &self.config.seed_labels {
            if seeds.len() == n { seeds.clone() } else { (0..n).collect() }
        } else { (0..n).collect() };
        let mut next = labels.clone();

        for _ in 0..self.config.max_iterations {
            let mut unchanged_count = 0usize;
            for u in 0..n {
                let neigh = graph.out_neighbors(u);
                if neigh.is_empty() {
                    next[u] = labels[u];
                    unchanged_count += 1;
                    continue;
                }

                // Count neighbor labels
                // Since labels are in 0..n, we can use a small vector as histogram
                let mut counts = vec![0u32; n];
                for &v in neigh {
                    counts[labels[v]] += 1;
                }
                // Pick most frequent label (break ties by smallest label for determinism)
                let mut best_label = labels[u];
                let mut best_count = 0u32;
                for (lab, &c) in counts.iter().enumerate() {
                    if c > best_count || (c == best_count && c > 0 && lab < best_label) {
                        best_count = c;
                        best_label = lab;
                    }
                }
                next[u] = if best_count == 0 { labels[u] } else { best_label };
                if next[u] == labels[u] {
                    unchanged_count += 1;
                }
            }

            if unchanged_count as f32 / n as f32 >= 1.0 - self.config.stability_threshold {
                labels.clone_from_slice(&next);
                break;
            }
            labels.clone_from_slice(&next);
        }

        // Group nodes by label
        let mut groups: Vec<Vec<usize>> = vec![Vec::new(); n];
        for (node, &lab) in labels.iter().enumerate() {
            groups[lab].push(node);
        }
        groups.into_iter().filter(|g| !g.is_empty()).collect()
    }
}
