use std::collections::HashSet;

#[derive(Debug, Clone, Copy)]
pub struct SearchScoringOptions {
    pub tag_match_boost: f32,
    pub title_weight: f32,
    pub content_weight: f32,
}

impl Default for SearchScoringOptions {
    fn default() -> Self {
        Self {
            tag_match_boost: 2.0,
            title_weight: 2.0,
            content_weight: 1.0,
        }
    }
}

/// Split tags by comma or whitespace, trim, lowercase, and deduplicate (preserve first-seen order).
pub fn parse_tags(input: &str) -> Vec<String> {
    let mut seen = HashSet::new();
    let mut out = Vec::new();
    for raw in input
        .split(|c: char| c.is_whitespace() || c == ',')
        .map(|s| s.trim())
        .filter(|s| !s.is_empty())
    {
        let tag = raw.to_lowercase();
        if seen.insert(tag.clone()) {
            out.push(tag);
        }
    }
    out
}

fn tokenize_lower(s: &str) -> Vec<String> {
    s.split(|c: char| !c.is_alphanumeric())
        .map(|t| t.trim().to_lowercase())
        .filter(|t| !t.is_empty())
        .collect()
}

/// A very simple bag-of-words scoring with optional tag boosts.
/// - Counts query token occurrences in title and content with different weights.
/// - Adds a fixed boost if any query token exactly matches a tag.
pub fn simple_score(
    title: &str,
    content: &str,
    tags: &[String],
    query: &str,
    opts: &SearchScoringOptions,
) -> f32 {
    let q = tokenize_lower(query);
    if q.is_empty() {
        return 0.0;
    }
    let title_toks = tokenize_lower(title);
    let content_toks = tokenize_lower(content);

    let mut score = 0.0f32;

    for token in &q {
        let t_count = title_toks.iter().filter(|w| *w == token).count() as f32;
        let c_count = content_toks.iter().filter(|w| *w == token).count() as f32;
        score += t_count * opts.title_weight + c_count * opts.content_weight;
    }

    // Tag boost if any query token equals a tag
    if !tags.is_empty() {
        let tagset: HashSet<&str> = tags.iter().map(|s| s.as_str()).collect();
        if q.iter().any(|t| tagset.contains(t.as_str())) {
            score += opts.tag_match_boost;
        }
    }

    score
}
