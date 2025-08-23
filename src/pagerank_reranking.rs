use serde::{Deserialize, Serialize};

/// Minimal graph access trait for PageRank
pub trait GraphAccess {
    fn node_count(&self) -> usize;
    fn out_neighbors(&self, u: usize) -> &[usize];
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PageRankConfig {
    pub damping: f32,
    pub iterations: usize,
    pub convergence: f32,
    /// Optional personalization vector (length N). If provided, teleport uses this distribution.
    /// Must be non-negative and sum to > 0; will be normalized internally.
    pub personalization: Option<Vec<f32>>,
    /// Optional dangling distribution vector for distributing rank from dangling nodes.
    /// If None, uses uniform distribution.
    pub dangling_distribution: Option<Vec<f32>>,
}

impl Default for PageRankConfig {
    fn default() -> Self {
        Self {
            damping: 0.85,
            iterations: 50,
            convergence: 1e-6,
            personalization: None,
            dangling_distribution: None,
        }
    }
}

#[derive(Clone, Debug)]
pub struct PageRankEngine {
    pub config: PageRankConfig,
}

impl PageRankEngine {
    pub fn new(config: PageRankConfig) -> Self {
        Self { config }
    }

    pub fn score_nodes<G: GraphAccess>(&self, graph: &G) -> Vec<f32> {
        let n = graph.node_count();
        if n == 0 {
            return vec![];
        }

        // Initialize ranks uniformly
        let mut rank = vec![1.0f32 / n as f32; n];
        let mut next = vec![0.0f32; n];
        let d = self.config.damping;
        // Prepare teleport distribution
        let teleport_vec: Vec<f32> = if let Some(p) = &self.config.personalization {
            if p.len() == n {
                let mut sum: f32 = p.iter().cloned().sum();
                if sum <= 0.0 { sum = 1.0; }
                p.iter().map(|&x| if x < 0.0 { 0.0 } else { x / sum }).collect()
            } else {
                vec![1.0 / n as f32; n]
            }
        } else {
            vec![1.0 / n as f32; n]
        };
        // Prepare dangling distribution
        let dangling_vec: Vec<f32> = if let Some(q) = &self.config.dangling_distribution {
            if q.len() == n {
                let mut sum: f32 = q.iter().cloned().sum();
                if sum <= 0.0 { sum = 1.0; }
                q.iter().map(|&x| if x < 0.0 { 0.0 } else { x / sum }).collect()
            } else {
                vec![1.0 / n as f32; n]
            }
        } else {
            vec![1.0 / n as f32; n]
        };

        for _ in 0..self.config.iterations {
            // Start with teleport mass according to personalization
            next
                .iter_mut()
                .zip(teleport_vec.iter())
                .for_each(|(ni, &t)| *ni = (1.0 - d) * t);

            // Distribute ranks
            for (u, &ru) in rank.iter().enumerate() {
                let outs = graph.out_neighbors(u);
                if outs.is_empty() {
                    // Dangling node: distribute using dangling distribution vector
                    let add = d * ru;
                    for (nv, &dv) in next.iter_mut().zip(dangling_vec.iter()) {
                        *nv += add * dv;
                    }
                } else {
                    let share = d * ru / outs.len() as f32;
                    for &v in outs {
                        next[v] += share;
                    }
                }
            }

            // Check convergence (L1 diff)
            let mut diff = 0.0f32;
            for (&ni, &ri) in next.iter().zip(rank.iter()) {
                diff += (ni - ri).abs();
            }
            rank.clone_from_slice(&next);
            if diff < self.config.convergence {
                break;
            }
        }

        rank
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RerankingConfig {
    pub weight_mono_t5: f32,
    pub weight_tildev2: f32,
    pub weight_original: f32,
}

impl Default for RerankingConfig {
    fn default() -> Self {
        Self {
            weight_mono_t5: 0.4,
            weight_tildev2: 0.3,
            weight_original: 0.3,
        }
    }
}

#[derive(Clone, Debug)]
pub struct AdvancedReranker {
    pub config: RerankingConfig,
}

impl AdvancedReranker {
    pub fn new(config: RerankingConfig) -> Self {
        Self { config }
    }

    pub fn rerank(&self, scores_a: &[f32], scores_b: &[f32], scores_orig: &[f32]) -> Vec<f32> {
        let n = scores_a.len().min(scores_b.len()).min(scores_orig.len());
        (0..n)
            .map(|i| {
                self.config.weight_mono_t5 * scores_a[i]
                    + self.config.weight_tildev2 * scores_b[i]
                    + self.config.weight_original * scores_orig[i]
            })
            .collect()
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SynthesisConfig {
    pub max_chars: usize,
}

impl Default for SynthesisConfig {
    fn default() -> Self {
        Self { max_chars: 2048 }
    }
}

#[derive(Clone, Debug)]
pub struct ResultSynthesizer {
    pub config: SynthesisConfig,
}

impl ResultSynthesizer {
    pub fn new(config: SynthesisConfig) -> Self {
        Self { config }
    }

    pub fn synthesize(&self, snippets: &[String]) -> String {
        // no-op stub: join with spaces and truncate
        let mut combined = snippets.join(" ");
        if combined.len() > self.config.max_chars {
            combined.truncate(self.config.max_chars);
        }
        combined
    }
}
