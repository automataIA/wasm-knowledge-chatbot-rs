#![cfg(target_arch = "wasm32")]

use wasm_bindgen_test::wasm_bindgen_test_configure;
use wasm_bindgen_test::*;
wasm_bindgen_test_configure!(run_in_browser);

use wasm_knowledge_chatbot_rs::advanced_graphrag::{
    CommunityDetectionConfig, CommunityDetectionEngine, HyDEConfig, HyDEEngine,
};
use wasm_knowledge_chatbot_rs::pagerank_reranking::{
    AdvancedReranker, GraphAccess, PageRankConfig, PageRankEngine, RerankingConfig,
    ResultSynthesizer, SynthesisConfig,
};

#[wasm_bindgen_test]
fn hyde_engine_defaults() {
    let cfg = HyDEConfig::default();
    let engine = HyDEEngine::new(cfg);
    let q = "a".repeat(600);
    let docs = engine.generate_hypothetical_docs(&q);
    assert_eq!(docs.len(), 3);
    for d in docs {
        assert!(d.len() <= 512);
        assert!(d.contains("HyDE"));
    }

    #[wasm_bindgen_test]
    fn community_two_clusters_with_sparse_cross_edges() {
        use wasm_knowledge_chatbot_rs::pagerank_reranking::GraphAccess;
        struct G {
            adj: Vec<Vec<usize>>,
        }
        impl GraphAccess for G {
            fn node_count(&self) -> usize {
                self.adj.len()
            }
            fn out_neighbors(&self, u: usize) -> &[usize] {
                &self.adj[u]
            }
        }

        // Two cliques of size 5: {0..4}, {5..9} with a couple of one-way cross edges 1->6 and 3->7
        let n = 10usize;
        let mut adj = vec![vec![]; n];
        let clique = |nodes: &[usize], adj: &mut Vec<Vec<usize>>| {
            for &u in nodes {
                for &v in nodes {
                    if u != v {
                        adj[u].push(v);
                    }
                }
            }
        };
        clique(&[0, 1, 2, 3, 4], &mut adj);
        clique(&[5, 6, 7, 8, 9], &mut adj);
        // Add sparse cross edges from first cluster to second (directed)
        adj[1].push(6);
        adj[3].push(7);

        let g = G { adj };
        let engine = CommunityDetectionEngine::new(CommunityDetectionConfig::default());
        let mut comms = engine.detect_communities(&g);
        // Sort for deterministic asserts
        for c in &mut comms {
            c.sort_unstable();
        }
        comms.sort();

        assert_eq!(comms.len(), 2);
        assert_eq!(comms[0], vec![0, 1, 2, 3, 4]);
        assert_eq!(comms[1], vec![5, 6, 7, 8, 9]);
    }
    #[wasm_bindgen_test]
    fn community_detects_two_clusters() {
        use wasm_knowledge_chatbot_rs::pagerank_reranking::GraphAccess;
        struct G {
            adj: Vec<Vec<usize>>,
        }
        impl GraphAccess for G {
            fn node_count(&self) -> usize {
                self.adj.len()
            }
            fn out_neighbors(&self, u: usize) -> &[usize] {
                &self.adj[u]
            }
        }
        // Two bidirectional clusters: {0,1,2} and {3,4,5}
        let mut adj = vec![vec![]; 6];
        let clique = |nodes: &[usize], adj: &mut Vec<Vec<usize>>| {
            for &u in nodes {
                for &v in nodes {
                    if u != v {
                        adj[u].push(v);
                    }
                }
            }
        };
        clique(&[0, 1, 2], &mut adj);
        clique(&[3, 4, 5], &mut adj);
        let g = G { adj };

        let engine = CommunityDetectionEngine::new(CommunityDetectionConfig::default());
        let comms = engine.detect_communities(&g);

        // Expect exactly two communities, each of size 3, matching the clusters (order may vary)
        assert_eq!(comms.len(), 2);
        let mut sizes: Vec<usize> = comms.iter().map(|c| c.len()).collect();
        sizes.sort();
        assert_eq!(sizes, vec![3, 3]);

        // Verify the sets are {0,1,2} and {3,4,5}
        let mut sets: Vec<Vec<usize>> = comms
            .into_iter()
            .map(|mut c| {
                c.sort_unstable();
                c
            })
            .collect();
        sets.sort();
        assert_eq!(sets, vec![vec![0, 1, 2], vec![3, 4, 5]]);
    }

    #[wasm_bindgen_test]
    fn pagerank_converges_and_normalizes() {
        // Ring graph of 10 nodes should converge to uniform distribution and sum ~ 1.0
        struct SimpleGraph {
            adj: Vec<Vec<usize>>,
        }
        impl GraphAccess for SimpleGraph {
            fn node_count(&self) -> usize {
                self.adj.len()
            }
            fn out_neighbors(&self, u: usize) -> &[usize] {
                &self.adj[u]
            }
        }
        let n = 10usize;
        let mut adj = vec![Vec::new(); n];
        for u in 0..n {
            adj[u].push((u + 1) % n);
        }
        let g = SimpleGraph { adj };

        let cfg = PageRankConfig {
            damping: 0.85,
            iterations: 200,
            convergence: 1e-8,
            ..Default::default()
        };
        let engine = PageRankEngine::new(cfg);
        let scores = engine.score_nodes(&g);

        assert_eq!(scores.len(), n);
        let sum: f32 = scores.iter().sum();
        assert!((sum - 1.0).abs() < 1e-4);
        for s in scores {
            assert!((s - 1.0 / n as f32).abs() < 1e-3);
        }
    }

    #[wasm_bindgen_test]
    fn community_partition_invariants() {
        // Invariants: cover all nodes, disjoint communities
        struct G {
            adj: Vec<Vec<usize>>,
        }
        impl GraphAccess for G {
            fn node_count(&self) -> usize {
                self.adj.len()
            }
            fn out_neighbors(&self, u: usize) -> &[usize] {
                &self.adj[u]
            }
        }
        let n = 6usize;
        let g = G {
            adj: vec![vec![1], vec![2], vec![3], vec![4], vec![5], vec![0]],
        };
        let engine = CommunityDetectionEngine::new(CommunityDetectionConfig::default());
        let comms = engine.detect_communities(&g);

        // Invariants
        let mut covered = vec![false; n];
        for c in &comms {
            for &u in c {
                covered[u] = true;
            }
        }
        assert!(covered.into_iter().all(|x| x));
        // Disjointness: no node appears twice
        let mut seen = vec![0u32; n];
        for c in &comms {
            for &u in c {
                seen[u] += 1;
            }
        }
        assert!(seen.into_iter().all(|cnt| cnt == 1));
    }

    #[wasm_bindgen_test]
    fn reranker_weight_effects() {
        // If we set mono_t5 weight to 1, output should equal scores_a
        let cfg = RerankingConfig {
            weight_mono_t5: 1.0,
            weight_tildev2: 0.0,
            weight_original: 0.0,
        };
        let reranker = AdvancedReranker::new(cfg);
        let a = vec![0.9, 0.1, 0.5];
        let b = vec![0.0, 1.0, 0.0];
        let o = vec![0.2, 0.2, 0.2];
        let out = reranker.rerank(&a, &b, &o);
        assert_eq!(out, a);

        // Balanced weights should lie between min/max of inputs per index
        let cfg2 = RerankingConfig::default();
        let rer2 = AdvancedReranker::new(cfg2);
        let out2 = rer2.rerank(&a, &b, &o);
        assert_eq!(out2.len(), 3);
        for i in 0..3 {
            let mn = a[i].min(b[i]).min(o[i]);
            let mx = a[i].max(b[i]).max(o[i]);
            assert!(out2[i] >= mn - 1e-6 && out2[i] <= mx + 1e-6);
        }
    }

    #[wasm_bindgen_test]
    fn synthesizer_concatenates_multiple_snippets() {
        let cfg = SynthesisConfig { max_chars: 10000 };
        let synth = ResultSynthesizer::new(cfg);
        let out = synth.synthesize(&["alpha".into(), "beta".into(), "gamma".into()]);
        assert!(out.contains("alpha"));
        assert!(out.contains("beta"));
        assert!(out.contains("gamma"));
        assert!(out.contains(' '));
    }
}

#[wasm_bindgen_test]
fn community_detection_defaults() {
    let cfg = CommunityDetectionConfig::default();
    let engine = CommunityDetectionEngine::new(cfg);
    // Graph with 0 nodes -> expect empty
    struct EmptyGraph;
    impl GraphAccess for EmptyGraph {
        fn node_count(&self) -> usize {
            0
        }
        fn out_neighbors(&self, _u: usize) -> &[usize] {
            &[]
        }
    }
    let comms = engine.detect_communities(&EmptyGraph);
    assert!(comms.is_empty());
}

#[wasm_bindgen_test]
fn pagerank_uniform_scores() {
    let cfg = PageRankConfig::default();
    let engine = PageRankEngine::new(cfg);
    // Build a simple graph with 5 nodes and a ring structure to ensure uniform distribution
    struct SimpleGraph {
        adj: Vec<Vec<usize>>,
    }
    impl GraphAccess for SimpleGraph {
        fn node_count(&self) -> usize {
            self.adj.len()
        }
        fn out_neighbors(&self, u: usize) -> &[usize] {
            &self.adj[u]
        }
    }
    let n = 5usize;
    let mut adj = vec![Vec::new(); n];
    for u in 0..n {
        adj[u].push((u + 1) % n);
    }
    let g = SimpleGraph { adj };
    let scores = engine.score_nodes(&g);
    assert_eq!(scores.len(), 5);
    for s in scores {
        assert!((s - 0.2).abs() < 1e-6);
    }
}

#[wasm_bindgen_test]
fn reranker_combination() {
    let cfg = RerankingConfig::default();
    let reranker = AdvancedReranker::new(cfg);
    let a = vec![1.0, 0.0];
    let b = vec![0.0, 1.0];
    let o = vec![0.5, 0.5];
    let out = reranker.rerank(&a, &b, &o);
    assert_eq!(out.len(), 2);
}

#[wasm_bindgen_test]
fn synthesizer_truncates() {
    let mut long = String::new();
    for _ in 0..3000 {
        long.push('a');
    }
    let cfg = SynthesisConfig { max_chars: 1000 };
    let synth = ResultSynthesizer::new(cfg);
    let out = synth.synthesize(&[long]);
    assert_eq!(out.len(), 1000);
}

#[wasm_bindgen_test]
fn pagerank_star_graph_distribution() {
    // Star graph: center 0 -> leaves 1..n-1 (directed out from center)
    use wasm_knowledge_chatbot_rs::pagerank_reranking::GraphAccess;
    struct G {
        adj: Vec<Vec<usize>>,
    }
    impl GraphAccess for G {
        fn node_count(&self) -> usize {
            self.adj.len()
        }
        fn out_neighbors(&self, u: usize) -> &[usize] {
            &self.adj[u]
        }
    }
    let n = 6usize;
    let mut adj = vec![vec![]; n];
    for v in 1..n {
        adj[0].push(v);
    }
    let g = G { adj };

    let cfg = PageRankConfig {
        damping: 0.85,
        iterations: 200,
        convergence: 1e-8,
        ..Default::default()
    };
    let engine = PageRankEngine::new(cfg);
    let scores = engine.score_nodes(&g);

    assert_eq!(scores.len(), n);
    let sum: f32 = scores.iter().sum();
    assert!((sum - 1.0).abs() < 1e-4);
    // Leaves should be equal among themselves
    let leaf_score = scores[1];
    for i in 2..n {
        assert!((scores[i] - leaf_score).abs() < 1e-4);
    }
}

#[wasm_bindgen_test]
fn pagerank_with_dangling_nodes_normalizes() {
    // Graph: 0 -> 1, 2 is dangling (no out edges)
    use wasm_knowledge_chatbot_rs::pagerank_reranking::GraphAccess;
    struct G {
        adj: Vec<Vec<usize>>,
    }
    impl GraphAccess for G {
        fn node_count(&self) -> usize {
            self.adj.len()
        }
        fn out_neighbors(&self, u: usize) -> &[usize] {
            &self.adj[u]
        }
    }
    let adj = vec![vec![1usize], vec![], vec![]];
    let g = G { adj };

    let cfg = PageRankConfig {
        damping: 0.85,
        iterations: 200,
        convergence: 1e-8,
        ..Default::default()
    };
    let engine = PageRankEngine::new(cfg);
    let scores = engine.score_nodes(&g);
    assert_eq!(scores.len(), 3);
    let sum: f32 = scores.iter().sum();
    assert!((sum - 1.0).abs() < 1e-4);
    for s in scores {
        assert!(s.is_finite() && s >= 0.0);
    }
}
