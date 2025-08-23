use crate::models::{
    app::AppError,
    graphrag::{GraphEdge, GraphNode, PerformanceMode, RAGQuery, RAGResult, SearchStrategy},
};
use std::collections::HashMap;

/// GraphRAG utility functions for graph operations and search optimization
pub struct GraphRAGUtils;

impl GraphRAGUtils {
    /// Calculate similarity between two text strings using simple word overlap
    pub fn calculate_similarity(text1: &str, text2: &str) -> f32 {
        let words1: std::collections::HashSet<&str> = text1.split_whitespace().collect();
        let words2: std::collections::HashSet<&str> = text2.split_whitespace().collect();

        let intersection = words1.intersection(&words2).count();
        let union = words1.union(&words2).count();

        if union == 0 {
            0.0
        } else {
            intersection as f32 / union as f32
        }
    }

    /// Build adjacency matrix from graph nodes and edges
    pub fn build_adjacency_matrix(
        nodes: &[GraphNode],
        edges: &[GraphEdge],
    ) -> HashMap<String, Vec<String>> {
        let mut adjacency = HashMap::new();

        // Initialize all nodes
        for node in nodes {
            adjacency.insert(node.id.clone(), Vec::new());
        }

        // Add edges
        for edge in edges {
            if let Some(connections) = adjacency.get_mut(&edge.source_id) {
                connections.push(edge.target_id.clone());
            }

            // For undirected graphs, add reverse connection
            if let Some(connections) = adjacency.get_mut(&edge.target_id) {
                connections.push(edge.source_id.clone());
            }
        }

        adjacency
    }

    /// Find shortest path between two nodes using BFS
    pub fn find_shortest_path(
        adjacency: &HashMap<String, Vec<String>>,
        start: &str,
        end: &str,
    ) -> Option<Vec<String>> {
        use std::collections::{HashSet, VecDeque};

        if start == end {
            return Some(vec![start.to_string()]);
        }

        let mut queue = VecDeque::new();
        let mut visited = HashSet::new();
        let mut parent = HashMap::new();

        queue.push_back(start.to_string());
        visited.insert(start.to_string());

        while let Some(current) = queue.pop_front() {
            if let Some(neighbors) = adjacency.get(&current) {
                for neighbor in neighbors {
                    if !visited.contains(neighbor) {
                        visited.insert(neighbor.clone());
                        parent.insert(neighbor.clone(), current.clone());
                        queue.push_back(neighbor.clone());

                        if neighbor == end {
                            // Reconstruct path
                            let mut path = Vec::new();
                            let mut node = end.to_string();

                            while let Some(p) = parent.get(&node) {
                                path.push(node.clone());
                                node = p.clone();
                            }
                            path.push(start.to_string());
                            path.reverse();

                            return Some(path);
                        }
                    }
                }
            }
        }

        None
    }

    /// Optimize search strategy based on query characteristics
    pub fn optimize_search_strategy(query: &RAGQuery) -> SearchStrategy {
        let query_length = query.text.len();
        let has_filters = !query.filters.tags.is_empty() || query.filters.date_range.is_some();

        match (query_length, has_filters) {
            // Short queries with filters - use local search
            (0..=50, true) => SearchStrategy::Local,
            // Long complex queries - use global search
            (200.., _) => SearchStrategy::Global,
            // Medium queries - use combined approach
            (51..=199, _) => SearchStrategy::Combined,
            // Default to automatic
            _ => SearchStrategy::Automatic,
        }
    }

    /// Get performance mode based on system capabilities
    pub fn get_optimal_performance_mode() -> PerformanceMode {
        // Simple heuristic based on available memory
        let available_memory = crate::utils::webllm::WebLLMUtils::estimate_available_memory();

        if available_memory > 8192.0 {
            PerformanceMode::MaxQuality
        } else if available_memory > 4096.0 {
            PerformanceMode::Balanced
        } else {
            PerformanceMode::MaxSpeed
        }
    }

    /// Validate GraphRAG query
    pub fn validate_query(query: &RAGQuery) -> Result<(), AppError> {
        if query.text.trim().is_empty() {
            return Err(AppError::validation(
                "Query text cannot be empty".to_string(),
            ));
        }

        if query.text.len() > 10000 {
            return Err(AppError::validation(
                "Query text too long (max 10000 characters)".to_string(),
            ));
        }

        if query.config.max_results == 0 {
            return Err(AppError::validation(
                "Max results must be greater than 0".to_string(),
            ));
        }

        if query.config.max_results > 1000 {
            return Err(AppError::validation(
                "Max results cannot exceed 1000".to_string(),
            ));
        }

        Ok(())
    }

    /// Score and rank search results
    pub fn rank_results(results: &mut [RAGResult], query: &RAGQuery) {
        // Calculate relevance scores based on available data
        for result in results.iter_mut() {
            // Use node content for similarity if available
            let content = result
                .nodes
                .first()
                .map(|node| node.content.as_str())
                .unwrap_or("");

            let text_similarity = Self::calculate_similarity(&query.text, content);
            let recency_bonus = 0.1; // Simple static bonus for now

            // Store relevance score in the scores vector (first element)
            let relevance_score = text_similarity * 0.7 + recency_bonus * 0.3;
            if result.scores.is_empty() {
                result.scores.push(relevance_score);
            } else {
                result.scores[0] = relevance_score;
            }
        }

        // Sort by relevance score (descending)
        results.sort_by(|a, b| {
            let score_a = a.scores.first().unwrap_or(&0.0);
            let score_b = b.scores.first().unwrap_or(&0.0);
            score_b
                .partial_cmp(score_a)
                .unwrap_or(std::cmp::Ordering::Equal)
        });
    }

    /// Extract key entities from text (simple implementation)
    pub fn extract_entities(text: &str) -> Vec<String> {
        // Simple entity extraction based on capitalized words
        text.split_whitespace()
            .filter(|word| {
                word.chars().next().is_some_and(|c| c.is_uppercase())
                    && word.len() > 2
                    && !["The", "This", "That", "And", "But", "Or", "For", "With"].contains(word)
            })
            .map(|word| {
                word.trim_matches(|c: char| !c.is_alphanumeric())
                    .to_string()
            })
            .filter(|word| !word.is_empty())
            .collect()
    }

    /// Generate document summary
    pub fn generate_summary(content: &str, max_length: usize) -> String {
        if content.len() <= max_length {
            return content.to_string();
        }

        // Simple extractive summarization - take first few sentences
        let sentences: Vec<&str> = content.split(". ").collect();
        let mut summary = String::new();

        for sentence in sentences {
            if summary.len() + sentence.len() + 2 > max_length {
                break;
            }
            if !summary.is_empty() {
                summary.push_str(". ");
            }
            summary.push_str(sentence);
        }

        if summary.len() < content.len() {
            summary.push_str("...");
        }

        summary
    }

    /// Calculate graph metrics
    pub fn calculate_graph_metrics(
        nodes: &[GraphNode],
        edges: &[GraphEdge],
    ) -> HashMap<String, f32> {
        let mut metrics = HashMap::new();

        // Node count
        metrics.insert("node_count".to_string(), nodes.len() as f32);

        // Edge count
        metrics.insert("edge_count".to_string(), edges.len() as f32);

        // Average degree
        if !nodes.is_empty() {
            let total_degree = edges.len() * 2; // Each edge contributes to 2 nodes
            metrics.insert(
                "average_degree".to_string(),
                total_degree as f32 / nodes.len() as f32,
            );
        }

        // Density
        if nodes.len() > 1 {
            let max_edges = nodes.len() * (nodes.len() - 1) / 2;
            metrics.insert("density".to_string(), edges.len() as f32 / max_edges as f32);
        }

        metrics
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::graphrag::{NodeType, QueryConfig, QueryFilters, QueryType};

    #[test]
    fn test_calculate_similarity() {
        let text1 = "hello world test";
        let text2 = "hello test example";
        let similarity = GraphRAGUtils::calculate_similarity(text1, text2);
        assert!(similarity > 0.0 && similarity < 1.0);
    }

    #[test]
    fn test_extract_entities() {
        let text = "John Smith works at Microsoft in Seattle";
        let entities = GraphRAGUtils::extract_entities(text);
        assert!(entities.contains(&"John".to_string()));
        assert!(entities.contains(&"Smith".to_string()));
        assert!(entities.contains(&"Microsoft".to_string()));
        assert!(entities.contains(&"Seattle".to_string()));
    }

    #[test]
    fn test_generate_summary() {
        let content =
            "This is the first sentence. This is the second sentence. This is the third sentence.";
        let summary = GraphRAGUtils::generate_summary(content, 50);
        assert!(summary.len() <= 53); // 50 + "..."
        assert!(summary.contains("first sentence"));
    }
}
