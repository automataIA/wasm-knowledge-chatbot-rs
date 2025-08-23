use crate::models::graphrag::DocumentIndex;
use crate::models::graph_store::{GraphEdge, GraphNode};
use serde_json::{json, Value};
use std::collections::{HashMap, HashSet};

/// WASM-safe, heuristic NER/RE stub.
/// - Creates a document node per `DocumentIndex`
/// - Extracts simple entity candidates: unique TitleCase words (len>=3)
/// - Creates `mentions` edges from document -> entity
pub fn extract_entities_relations(docs: &[DocumentIndex]) -> (Vec<GraphNode>, Vec<GraphEdge>) {
    let mut nodes: Vec<GraphNode> = Vec::new();
    let mut edges: Vec<GraphEdge> = Vec::new();

    // Track unique entity strings to node ids
    let mut entity_map: HashMap<String, String> = HashMap::new();
    let mut existing_ids: HashSet<String> = HashSet::new();

    // Helper to ensure unique ids
    fn unique_id(base: &str, existing: &mut HashSet<String>) -> String {
        if !existing.contains(base) {
            existing.insert(base.to_string());
            return base.to_string();
        }
        let mut i = 2usize;
        loop {
            let cand = format!("{}#{}", base, i);
            if !existing.contains(&cand) {
                existing.insert(cand.clone());
                return cand;
            }
            i += 1;
        }
    }

    for d in docs {
        // Document node
        let doc_id = unique_id(&format!("doc:{}", d.id), &mut existing_ids);
        nodes.push(GraphNode {
            id: doc_id.clone(),
            label: Some(d.title.clone()),
            node_type: "document".to_string(),
            source_document_id: Some(d.id.clone()),
            metadata: json!({
                "file_type": d.file_type,
                "size_bytes": d.size_bytes,
                "created_at": d.created_at,
            }),
        });

        // 1) Chunk markdown into passages
        let passages = chunk_markdown(&d.content, 500);

        // For collecting back-references per entity: entity -> array of {doc, passage_idx}
        let mut ent_backrefs: HashMap<String, Vec<Value>> = HashMap::new();

        // Process each passage
        for (pidx, passage) in passages.iter().enumerate() {
            // 2) Simple NER: TitleCase tokens length>=3 scoped per passage
            let mut seen_in_passage: HashSet<String> = HashSet::new();
            for token in passage
                .split(|c: char| !c.is_alphanumeric())
                .filter(|t| !t.is_empty())
            {
                let is_title_case = token.chars().next().map(|c| c.is_uppercase()).unwrap_or(false);
                if is_title_case && token.len() >= 3 {
                    let key = token.to_string();
                    if seen_in_passage.insert(key.clone()) {
                        // get or create entity node id
                        let ent_id = if let Some(id) = entity_map.get(&key) { id.clone() } else {
                            let id = unique_id(&format!("ent:{}", key), &mut existing_ids);
                            entity_map.insert(key.clone(), id.clone());
                            nodes.push(GraphNode {
                                id: id.clone(),
                                label: Some(key.clone()),
                                node_type: "entity".to_string(),
                                source_document_id: None,
                                metadata: json!({
                                    "aliases": [key.clone()],
                                    "backrefs": [],
                                }),
                            });
                            id
                        };

                        // record backref for entity
                        ent_backrefs.entry(ent_id.clone()).or_default().push(json!({
                            "doc_id": d.id,
                            "passage_index": pidx,
                        }));

                        // mentions edge document -> entity with passage context
                        let edge_id = unique_id(&format!("e:{}->{}#p{}", doc_id, ent_id, pidx), &mut existing_ids);
                        edges.push(GraphEdge {
                            id: edge_id,
                            from: doc_id.clone(),
                            to: ent_id,
                            relation: "mentions".to_string(),
                            weight: 1.0,
                            metadata: json!({
                                "source": "stub",
                                "doc_id": d.id,
                                "passage_index": pidx,
                            }),
                        });
                    }
                }
            }

            // 3) Simple rule-based RE: detect a few patterns in passage text
            // Patterns handled (very heuristic):
            // - "X is a Y" => (X, is_a, Y)
            // - "X works at Y" => (X, works_at, Y)
            for triple in simple_relation_extraction(passage) {
                let (subj, pred, obj) = triple;
                let sid = ensure_entity(&subj, &mut entity_map, &mut existing_ids, &mut nodes);
                let oid = ensure_entity(&obj, &mut entity_map, &mut existing_ids, &mut nodes);

                // backrefs
                ent_backrefs.entry(sid.clone()).or_default().push(json!({"doc_id": d.id, "passage_index": pidx}));
                ent_backrefs.entry(oid.clone()).or_default().push(json!({"doc_id": d.id, "passage_index": pidx}));

                // relation edge subj -> obj
                let edge_id = unique_id(&format!("e:{}:{}->{}#p{}", pred, sid, oid, pidx), &mut existing_ids);
                edges.push(GraphEdge {
                    id: edge_id,
                    from: sid,
                    to: oid,
                    relation: pred.clone(),
                    weight: 1.0,
                    metadata: json!({
                        "source": "stub_re",
                        "doc_id": d.id,
                        "passage_index": pidx,
                        "triple": {"subject": subj, "predicate": pred, "object": obj},
                    }),
                });
            }
        }

        // Merge backrefs into entity node metadata
        if !ent_backrefs.is_empty() {
            for n in nodes.iter_mut() {
                if n.node_type == "entity" {
                    if let Some(brs) = ent_backrefs.get(&n.id) {
                        let mut meta = n.metadata.clone();
                        // ensure backrefs as array
                        let arr = match meta.get_mut("backrefs") {
                            Some(Value::Array(a)) => a,
                            _ => {
                                meta["backrefs"] = json!([]);
                                meta.get_mut("backrefs").unwrap().as_array_mut().unwrap()
                            }
                        };
                        for br in brs { arr.push(br.clone()); }
                        n.metadata = meta;
                    }
                }
            }
        }
    }

    (nodes, edges)
}

// --- helpers ---

fn chunk_markdown(content: &str, max_len: usize) -> Vec<String> {
    // naive chunking: split by headings or blank lines, then pack up to max_len chars
    let mut chunks: Vec<String> = Vec::new();
    let mut current = String::new();
    for line in content.lines() {
        let is_heading = line.trim_start().starts_with('#');
        let is_blank = line.trim().is_empty();
        let candidate_len = current.len() + line.len() + 1;
        if (is_heading || is_blank || candidate_len > max_len) && !current.trim().is_empty() {
            chunks.push(current.trim().to_string());
            current.clear();
        }
        if !line.trim().is_empty() {
            if !current.is_empty() { current.push('\n'); }
            current.push_str(line);
        }
    }
    if !current.trim().is_empty() { chunks.push(current.trim().to_string()); }
    if chunks.is_empty() { chunks.push(content.to_string()); }
    chunks
}

fn ensure_entity(key: &str, entity_map: &mut HashMap<String, String>, existing_ids: &mut HashSet<String>, nodes: &mut Vec<GraphNode>) -> String {
    if let Some(id) = entity_map.get(key) { return id.clone(); }
    // Allow non-TitleCase from RE too
    let id = unique_like("ent", key, existing_ids);
    entity_map.insert(key.to_string(), id.clone());
    nodes.push(GraphNode {
        id: id.clone(),
        label: Some(key.to_string()),
        node_type: "entity".to_string(),
        source_document_id: None,
        metadata: json!({"aliases": [key]}),
    });
    id
}

fn unique_like(prefix: &str, key: &str, existing: &mut HashSet<String>) -> String {
    let base = format!("{}:{}", prefix, key);
    if !existing.contains(&base) { existing.insert(base.clone()); return base; }
    let mut i = 2usize;
    loop {
        let cand = format!("{}#{}", base, i);
        if !existing.contains(&cand) { existing.insert(cand.clone()); return cand; }
        i += 1;
    }
}

fn simple_relation_extraction(passage: &str) -> Vec<(String, String, String)> {
    // extremely simple pattern-based RE
    // We scan sentences split by ., !, ?
    let mut triples = Vec::new();
    for sent in passage.split(['.', '!', '?']) {
        let s = sent.trim();
        if s.is_empty() { continue; }
        // pattern: "X is a Y"
        if let Some(idx) = s.find(" is a ") {
            let (l, r) = s.split_at(idx);
            let r = &r[6..]; // skip " is a "
            let subj = l.trim();
            let obj = r.split_whitespace().take(4).collect::<Vec<_>>().join(" ");
            if !subj.is_empty() && !obj.is_empty() {
                triples.push((subj.to_string(), "is_a".to_string(), obj.trim_matches(',').to_string()));
                continue;
            }
        }
        // pattern: "X works at Y"
        if let Some(idx) = s.find(" works at ") {
            let (l, r) = s.split_at(idx);
            let r = &r[10..]; // skip " works at "
            let subj = l.trim();
            let obj = r.trim_matches(',').trim();
            if !subj.is_empty() && !obj.is_empty() {
                triples.push((subj.to_string(), "works_at".to_string(), obj.to_string()));
                continue;
            }
        }
    }
    triples
}
