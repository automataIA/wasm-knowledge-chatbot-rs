use crate::models::graphrag::{
    DocumentIndex, EdgeMetadata, EdgeType, GraphEdge, GraphNode, NodeType, RAGQuery, RAGResult,
    ResultMetadata, SearchStrategy,
};
use crate::models::graph_store::GraphStore;
use crate::utils::storage::StorageUtils;
use crate::graphrag_config::{GraphRAGConfig, PerformanceMetrics, with_graphrag_manager};
use std::collections::{HashMap, HashSet};

/// GraphRAG retrieval entrypoints. Stubs returning empty results.
pub struct Retriever;

impl Retriever {
    pub fn new() -> Self { Self }

    pub async fn search(&self, q: &RAGQuery, strategy: SearchStrategy) -> RAGResult {
        // Start timer and record algorithms used
        let t0 = js_sys::Date::now();
        // Stage timers
        let mut hyde_time_ms: u32 = 0;
        let mut pagerank_time_ms: u32 = 0;
        let mut community_time_ms: u32 = 0;
        let mut reranking_time_ms: u32 = 0;
        let mut hybrid_fusion_time_ms: u32 = 0;
        let mut synthesis_time_ms: u32 = 0;
        let mut algorithms = vec![format!("strategy:{:?}", strategy)];

        // Load GraphRAGConfig from localStorage (prefer v1 key, fallback to legacy, else defaults)
        let config: GraphRAGConfig = if let Ok(Some(c)) = StorageUtils::retrieve_local::<GraphRAGConfig>("graphrag_config_v1") {
            c
        } else {
            match StorageUtils::retrieve_local::<GraphRAGConfig>("graphrag_config") {
                Ok(Some(c)) => c,
                _ => GraphRAGConfig::default(),
            }
        };

        // Load persisted index (versioned key with legacy fallback)
        let docs: Vec<DocumentIndex> = if let Ok(Some(v)) = StorageUtils::retrieve_local::<Vec<DocumentIndex>>("graphrag_document_index_v1") {
            v
        } else {
            match StorageUtils::retrieve_local::<Vec<DocumentIndex>>("graphrag_document_index") {
                Ok(Some(v)) => v,
                _ => Vec::new(),
            }
        };

        // Tokenize query for TF-IDF style scoring
        let mut q_tokens: Vec<String> = q
            .text
            .to_lowercase()
            .split_whitespace()
            .map(|s| s.trim_matches(|c: char| !c.is_alphanumeric()).to_string())
            .filter(|s| !s.is_empty())
            .collect();

        // HyDE expansion (very light heuristic): duplicate tokens to upweight terms if enabled
        let hyde_on = q.config.use_hyde || config.hyde_enabled;
        if hyde_on {
            let t_h0 = js_sys::Date::now();
            algorithms.push("hyde".into());
            let mut extra = q_tokens.clone();
            // Simple bigram-like concatenation of adjacent tokens to simulate hypothetical variants
            for w in q_tokens.windows(2) {
                extra.push(format!("{}{}", w[0], w[1]));
            }
            q_tokens.extend(extra);
            hyde_time_ms = (js_sys::Date::now() - t_h0) as u32;
        }
        // Precompute document term sets and term frequencies
        let mut doc_tokens: Vec<HashMap<String, usize>> = Vec::with_capacity(docs.len());
        let mut doc_sets: Vec<HashSet<String>> = Vec::with_capacity(docs.len());
        let mut df: HashMap<String, usize> = HashMap::new();

        for d in &docs {
            let content = if d.content.is_empty() { d.title.clone() } else { d.content.clone() };
            let toks: Vec<String> = content
                .to_lowercase()
                .split_whitespace()
                .map(|s| s.trim_matches(|c: char| !c.is_alphanumeric()).to_string())
                .filter(|s| !s.is_empty())
                .collect();
            let mut tf: HashMap<String, usize> = HashMap::new();
            let mut set: HashSet<String> = HashSet::new();
            for t in toks {
                *tf.entry(t.clone()).or_insert(0) += 1;
                set.insert(t);
            }
            for term in &set {
                *df.entry(term.clone()).or_insert(0) += 1;
            }
            doc_tokens.push(tf);
            doc_sets.push(set);
        }

        // Compute TF-IDF style scores for query terms
        let n_docs = docs.len() as f32;
        let mut scored: Vec<(usize, f32)> = Vec::new(); // (doc_idx, score)
        for (i, tf) in doc_tokens.iter().enumerate() {
            let mut score = 0.0f32;
            for qt in &q_tokens {
                if let Some(&f) = tf.get(qt) {
                    let df_t = *df.get(qt).unwrap_or(&0) as f32;
                    if df_t > 0.0 {
                        // idf smoothing
                        let idf = ((n_docs + 1.0) / (df_t + 1.0)).ln() + 1.0;
                        score += (f as f32) * idf;
                    }
                }
            }
            scored.push((i, score));
        }

        // Sort by score desc and take top K according to config
        scored.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));
        let k = q.config.max_results.max(1);
        let mut top = scored.into_iter().take(k).collect::<Vec<_>>();

        // Optional PageRank-like centrality weighting over top docs
        // Uses Jaccard similarities among top docs as edge weights; boosts central/important docs.
        let use_pr = config.pagerank_enabled;
        if use_pr && top.len() > 1 {
            let t_pr0 = js_sys::Date::now();
            algorithms.push("pagerank_weighting".into());
            // Build a simple centrality score: sum of Jaccard weights to others
            let mut centrality: Vec<f32> = vec![0.0; top.len()];
            for (i, (di, _)) in top.iter().enumerate() {
                let di = *di;
                for (j, (dj, _)) in top.iter().enumerate() {
                    if i == j { continue; }
                    let dj = *dj;
                    let set_i = &doc_sets[di];
                    let set_j = &doc_sets[dj];
                    let inter = set_i.intersection(set_j).count() as f32;
                    let uni = set_i.union(set_j).count() as f32;
                    if uni > 0.0 {
                        let w = inter / uni; // Jaccard weight
                        centrality[i] += w;
                    }
                }
            }
            // Normalize centrality to 0..1
            if let Some(max_c) = centrality.iter().cloned().fold(None, |acc: Option<f32>, x| Some(acc.map_or(x, |m| if x > m { x } else { m }))) {
                if max_c > 0.0 {
                    for c in &mut centrality { *c /= max_c; }
                }
            }
            // Apply boost to scores and re-sort
            let alpha = 0.2f32;
            for (i, (_idx, s)) in top.iter_mut().enumerate() {
                *s *= 1.0 + alpha * centrality[i];
            }
            top.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));
            pagerank_time_ms = (js_sys::Date::now() - t_pr0) as u32;
        }

        // Optional community boosting: lightweight cluster-based boost using token overlap
        let use_community = q.config.use_community_detection || config.community_detection_enabled;
        if use_community && top.len() > 1 {
            let t_c0 = js_sys::Date::now();
            algorithms.push("community_boost".into());
            // Build neighbor counts based on Jaccard >= threshold within top-K
            let mut neighbor_counts: Vec<u32> = vec![0; top.len()];
            let thr = 0.25f32;
            for (i, (di, _)) in top.iter().enumerate() {
                let di = *di;
                for (j, (dj, _)) in top.iter().enumerate() {
                    if i == j { continue; }
                    let dj = *dj;
                    let set_i = &doc_sets[di];
                    let set_j = &doc_sets[dj];
                    let inter = set_i.intersection(set_j).count() as f32;
                    let uni = set_i.union(set_j).count() as f32;
                    if uni > 0.0 {
                        let jacc = inter / uni;
                        if jacc >= thr { neighbor_counts[i] += 1; }
                    }
                }
            }
            // Normalize neighbor counts to 0..1 and apply a small boost
            if let Some(&max_cnt) = neighbor_counts.iter().max() {
                if max_cnt > 0 {
                    let beta = 0.15f32;
                    for (i, (_idx, s)) in top.iter_mut().enumerate() {
                        let c = neighbor_counts[i] as f32 / max_cnt as f32;
                        *s *= 1.0 + beta * c;
                    }
                    top.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));
                }
            }
            community_time_ms = (js_sys::Date::now() - t_c0) as u32;
        }

        // Optional improved reranking: apply small deterministic tiebreak and resort
        let mut was_reranked = false;
        let do_rerank = q.config.use_reranking || config.reranking_enabled;
        if do_rerank {
            let t_r0 = js_sys::Date::now();
            algorithms.push("advanced_rerank".into());
            was_reranked = true;
            for (i, (_idx, s)) in top.iter_mut().enumerate() {
                // tiny index-based perturbation to stabilize ordering and break ties
                *s += (i as f32) * 1e-6;
            }
            top.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));
            reranking_time_ms = (js_sys::Date::now() - t_r0) as u32;
        }

        // Hybrid fusion: combine text scores with simple graph scores (mentions degree)
        if config.hybrid_enabled && !top.is_empty() {
            let t_hf0 = js_sys::Date::now();
            algorithms.push("hybrid_fusion".into());
            // Load graph store and compute a simple graph score per document id: mentions degree
            let store = GraphStore::load().unwrap_or_default();
            let doc_id_set: std::collections::HashSet<String> = docs.iter().map(|d| d.id.clone()).collect();
            let mut degree: std::collections::HashMap<String, f32> = std::collections::HashMap::new();
            for e in &store.edges {
                if e.relation == "mentions" {
                    if doc_id_set.contains(&e.from) {
                        *degree.entry(e.from.clone()).or_insert(0.0) += 1.0;
                    }
                    if doc_id_set.contains(&e.to) {
                        *degree.entry(e.to.clone()).or_insert(0.0) += 1.0;
                    }
                }
            }
            // Gather graph scores for top docs and normalize 0..1
            let mut g_scores: Vec<f32> = top
                .iter()
                .map(|(idx, _)| degree.get(&docs[*idx].id).cloned().unwrap_or(0.0))
                .collect();
            if let Some(gmax) = g_scores.iter().cloned().fold(None, |acc: Option<f32>, x| Some(acc.map_or(x, |m| if x > m { x } else { m }))) {
                if gmax > 0.0 {
                    for g in &mut g_scores { *g /= gmax; }
                }
            }
            // Normalize current text scores (copy) and fuse
            let t_scores: Vec<f32> = top.iter().map(|(_, s)| *s).collect();
            let mut t_norm = t_scores.clone();
            if let Some(tmax) = t_norm.iter().cloned().fold(None, |acc: Option<f32>, x| Some(acc.map_or(x, |m| if x > m { x } else { m }))) {
                if tmax > 0.0 {
                    for t in &mut t_norm { *t /= tmax; }
                }
            }
            for (i, (_, s)) in top.iter_mut().enumerate() {
                let fused = config.fusion_text_weight * t_norm[i] + config.fusion_graph_weight * g_scores[i];
                *s = fused;
            }
            // Resort after fusion
            top.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));
            hybrid_fusion_time_ms = (js_sys::Date::now() - t_hf0) as u32;
        }

        // Build nodes with stable IDs from DocumentIndex and annotate source/confidence
        let mut nodes: Vec<GraphNode> = Vec::with_capacity(top.len());
        let mut scores: Vec<f32> = Vec::with_capacity(top.len());
        for (idx, sc) in &top {
            let d = &docs[*idx];
            let content = if d.content.is_empty() { d.title.clone() } else { d.content.clone() };
            let mut node = GraphNode::new(content, NodeType::Document);
            // Use stable id and enrich metadata
            node.id = d.id.clone();
            node.metadata.source = Some(d.title.clone());
            node.metadata.confidence = (*sc).clamp(0.0, 1e9); // raw score stored as confidence proxy
            nodes.push(node);
            scores.push(*sc);
        }

        // Normalize scores to 0..1 for UI friendliness
        if let Some(max_sc) = scores.iter().cloned().fold(None, |acc: Option<f32>, x| {
            Some(acc.map_or(x, |m| if x > m { x } else { m }))
        }) {
            if max_sc > 0.0 {
                for s in &mut scores {
                    *s /= max_sc;
                }
            }
        }

        // Construct co-occurrence edges among top nodes via Jaccard similarity over token sets
        let mut edges: Vec<GraphEdge> = Vec::new();
        if top.len() > 1 {
            algorithms.push("cooccurrence_edges".into());
            let created_at = js_sys::Date::now();
            for i in 0..top.len() {
                for j in (i + 1)..top.len() {
                    let di = top[i].0;
                    let dj = top[j].0;
                    let set_i = &doc_sets[di];
                    let set_j = &doc_sets[dj];
                    let inter = set_i.intersection(set_j).count() as f32;
                    let uni = set_i.union(set_j).count() as f32;
                    if uni > 0.0 {
                        let jacc = inter / uni;
                        if jacc >= 0.2 { // threshold
                            let src_id = docs[di].id.clone();
                            let tgt_id = docs[dj].id.clone();
                            edges.push(GraphEdge {
                                id: format!("{}-{}-{}", src_id, "rel", tgt_id),
                                source_id: src_id,
                                target_id: tgt_id,
                                edge_type: EdgeType::RelatedTo,
                                weight: jacc,
                                metadata: EdgeMetadata {
                                    created_at,
                                    confidence: jacc.clamp(0.0, 1.0),
                                    properties: HashMap::new(),
                                },
                            });
                        }
                    }
                }
            }
        }

        // Strategy annotation only for now
        match strategy {
            SearchStrategy::Local => algorithms.push("local".into()),
            SearchStrategy::Global => algorithms.push("global".into()),
            SearchStrategy::Combined => algorithms.push("combined".into()),
            SearchStrategy::Automatic => algorithms.push("auto".into()),
        }

        // Tag algorithms used
        algorithms.push("tfidf".into());

        // Community detection metadata flag (placeholder): set when enabled
        let community_on = q.config.use_community_detection || config.community_detection_enabled;

        // Optional synthesis: create a brief extractive summary from top documents
        let mut summary: Option<String> = None;
        if config.synthesis_enabled && !top.is_empty() {
            let t_s0 = js_sys::Date::now();
            algorithms.push("synthesis".into());
            // Take up to first 3 sentences from the highest-scoring documents
            let mut parts: Vec<String> = Vec::new();
            for (idx, _sc) in top.iter().take(3) {
                let d = &docs[*idx];
                let content = if d.content.is_empty() { d.title.clone() } else { d.content.clone() };
                // naive sentence split on '.', '!' or '?' and filter empties
                let sentences: Vec<String> = content
                    .split(['.', '!', '?'])
                    .map(|s| s.trim())
                    .filter(|s| !s.is_empty())
                    .map(|s| s.to_string())
                    .collect();
                for s in sentences.into_iter().take(1) { // first sentence per doc
                    parts.push(s);
                }
                if parts.len() >= 3 { break; }
            }
            let mut s = parts.join(". ");
            if !s.is_empty() { s.push('.'); }
            // limit length to 512 chars
            if s.len() > 512 { s.truncate(512); }
            summary = if s.is_empty() { None } else { Some(s) };
            synthesis_time_ms = (js_sys::Date::now() - t_s0) as u32;
        }

        // Finalize processing time and update metrics after all stages (including synthesis)
        let processing_time_ms = (js_sys::Date::now() - t0) as u32;
        let perf = PerformanceMetrics {
            hyde_time_ms,
            community_detection_time_ms: community_time_ms,
            pagerank_time_ms,
            reranking_time_ms,
            hybrid_fusion_time_ms,
            synthesis_time_ms,
            total_time_ms: processing_time_ms,
        };
        with_graphrag_manager(|m| {
            m.update_performance_metrics(perf.clone());
            m.update_query_metrics(processing_time_ms, 0.0);
        });

        RAGResult {
            id: q.id.clone(),
            query_id: q.id.clone(),
            nodes,
            edges,
            scores,
            metadata: ResultMetadata {
                processing_time_ms,
                total_nodes_searched: docs.len(),
                reranked: was_reranked,
                hyde_enhanced: hyde_on,
                community_filtered: community_on,
                algorithms_used: algorithms,
                summary,
            },
        }
    }
}

impl Default for Retriever {
    fn default() -> Self { Self::new() }
}
