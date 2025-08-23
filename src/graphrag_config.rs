use serde::{Deserialize, Serialize};
use leptos::prelude::*;
use std::sync::OnceLock;
use crate::models::graphrag::SearchStrategy;

// Core GraphRAG Configuration with Feature Toggles
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
#[serde(default)]
pub struct GraphRAGConfig {
    // Feature toggles
    pub hyde_enabled: bool,
    pub community_detection_enabled: bool,
    pub pagerank_enabled: bool,
    pub reranking_enabled: bool,
    pub synthesis_enabled: bool,
    // Hybrid retrieval toggle and fusion weights
    pub hybrid_enabled: bool,
    pub fusion_text_weight: f32,
    pub fusion_graph_weight: f32,
    // Search strategy for chat-integrated retrieval
    pub search_strategy: SearchStrategy,
    
    // Performance settings
    pub max_query_time_ms: u32,
    pub max_memory_mb: u32,
    pub batch_size: usize,
}

impl Default for GraphRAGConfigManager {
    fn default() -> Self {
        Self::new()
    }
}

// Global accessor for metrics/config manager so non-UI modules can update metrics
static GRAPHRAG_MANAGER: OnceLock<GraphRAGConfigManager> = OnceLock::new();

pub fn set_global_graphrag_manager(manager: &GraphRAGConfigManager) {
    let _ = GRAPHRAG_MANAGER.set(manager.clone());
}

pub fn with_graphrag_manager<F: FnOnce(&GraphRAGConfigManager)>(f: F) {
    if let Some(m) = GRAPHRAG_MANAGER.get() { f(m); }
}

impl Default for GraphRAGConfig {
    fn default() -> Self {
        Self {
            hyde_enabled: true,
            community_detection_enabled: true,
            pagerank_enabled: true,
            reranking_enabled: false, // Computationally expensive
            synthesis_enabled: true,
            hybrid_enabled: true,
            fusion_text_weight: 0.7,
            fusion_graph_weight: 0.3,
            search_strategy: SearchStrategy::Automatic,
            max_query_time_ms: 5000,
            max_memory_mb: 100,
            batch_size: 10,
        }
    }
}

// Real-time metrics for StatusBar
#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct GraphRAGMetrics {
    pub last_query_time_ms: u32,
    pub memory_usage_mb: f32,
    pub queries_processed: u32,
    pub cache_hit_rate: f32,
    pub active_features: Vec<String>,
    pub performance_score: f32, // 0-100
}

// Performance monitoring
#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct PerformanceMetrics {
    pub hyde_time_ms: u32,
    pub community_detection_time_ms: u32,
    pub pagerank_time_ms: u32,
    pub reranking_time_ms: u32,
    pub hybrid_fusion_time_ms: u32,
    pub synthesis_time_ms: u32,
    pub total_time_ms: u32,
}

// Configuration Manager with localStorage persistence
#[derive(Clone, Debug)]
pub struct GraphRAGConfigManager {
    config: RwSignal<GraphRAGConfig>,
    metrics: RwSignal<GraphRAGMetrics>,
    performance: RwSignal<PerformanceMetrics>,
}

impl GraphRAGConfigManager {
    pub fn new() -> Self {
        let config = Self::load_config();
        let manager = Self {
            config: RwSignal::new(config),
            metrics: RwSignal::new(GraphRAGMetrics::default()),
            performance: RwSignal::new(PerformanceMetrics::default()),
        };
        manager.save_config(); // Ensure localStorage is initialized
        manager
    }

    // Configuration management
    pub fn get_config(&self) -> GraphRAGConfig {
        self.config.get()
    }

    // Non-tracking getters for use outside reactive contexts
    pub fn get_config_untracked(&self) -> GraphRAGConfig {
        self.config.get_untracked()
    }

    pub fn update_config<F>(&self, f: F) 
    where F: FnOnce(&mut GraphRAGConfig) {
        self.config.update(f);
        self.save_config();
        self.update_active_features();
    }

    pub fn toggle_hyde(&self) {
        self.update_config(|c| c.hyde_enabled = !c.hyde_enabled);
    }

    pub fn toggle_community_detection(&self) {
        self.update_config(|c| c.community_detection_enabled = !c.community_detection_enabled);
    }

    pub fn toggle_pagerank(&self) {
        self.update_config(|c| c.pagerank_enabled = !c.pagerank_enabled);
    }

    pub fn toggle_reranking(&self) {
        self.update_config(|c| c.reranking_enabled = !c.reranking_enabled);
    }

    pub fn toggle_synthesis(&self) {
        self.update_config(|c| c.synthesis_enabled = !c.synthesis_enabled);
    }

    // Metrics management
    pub fn get_metrics(&self) -> GraphRAGMetrics {
        self.metrics.get()
    }

    pub fn get_metrics_untracked(&self) -> GraphRAGMetrics {
        self.metrics.get_untracked()
    }

    pub fn update_query_metrics(&self, time_ms: u32, memory_mb: f32) {
        self.metrics.update(|m| {
            m.last_query_time_ms = time_ms;
            m.memory_usage_mb = memory_mb;
            m.queries_processed += 1;
            m.performance_score = self.calculate_performance_score(time_ms, memory_mb);
        });
    }

    pub fn update_performance_metrics(&self, perf: PerformanceMetrics) {
        self.performance.set(perf);
    }

    pub fn get_performance_metrics(&self) -> PerformanceMetrics {
        self.performance.get()
    }

    pub fn get_performance_metrics_untracked(&self) -> PerformanceMetrics {
        self.performance.get_untracked()
    }

    // Helper methods
    fn calculate_performance_score(&self, time_ms: u32, memory_mb: f32) -> f32 {
        let config = self.config.get_untracked();
        let time_score = ((config.max_query_time_ms.saturating_sub(time_ms)) as f32 / config.max_query_time_ms as f32) * 50.0;
        let memory_score = ((config.max_memory_mb as f32 - memory_mb) / config.max_memory_mb as f32) * 50.0;
        (time_score + memory_score).clamp(0.0, 100.0)
    }

    fn update_active_features(&self) {
        let config = self.config.get_untracked();
        let mut features = Vec::new();
        
        if config.hyde_enabled { features.push("HyDE".to_string()); }
        if config.community_detection_enabled { features.push("Community".to_string()); }
        if config.pagerank_enabled { features.push("PageRank".to_string()); }
        if config.reranking_enabled { features.push("Reranking".to_string()); }
        if config.synthesis_enabled { features.push("Synthesis".to_string()); }

        self.metrics.update(|m| m.active_features = features);
    }

    // Persistence
    fn load_config() -> GraphRAGConfig {
        if let Some(window) = web_sys::window() {
            if let Ok(Some(storage)) = window.local_storage() {
                // Prefer versioned key
                if let Ok(Some(config_str)) = storage.get_item("graphrag_config_v1") {
                    if let Ok(config) = serde_json::from_str::<GraphRAGConfig>(&config_str) {
                        return config;
                    }
                }
                // Fallback to legacy key
                if let Ok(Some(config_str)) = storage.get_item("graphrag_config") {
                    if let Ok(config) = serde_json::from_str::<GraphRAGConfig>(&config_str) {
                        return config;
                    }
                }
            }
        }
        GraphRAGConfig::default()
    }

    fn save_config(&self) {
        let config = self.config.get_untracked();
        if let Some(window) = web_sys::window() {
            if let Ok(Some(storage)) = window.local_storage() {
                if let Ok(config_str) = serde_json::to_string(&config) {
                    let _ = storage.set_item("graphrag_config_v1", &config_str);
                }
            }
        }
    }

    // Export/Import functionality
    pub fn export_config(&self) -> String {
        serde_json::to_string_pretty(&self.config.get_untracked()).unwrap_or_default()
    }

    pub fn import_config(&self, config_json: &str) -> Result<(), String> {
        match serde_json::from_str::<GraphRAGConfig>(config_json) {
            Ok(config) => {
                self.config.set(config);
                self.save_config();
                self.update_active_features();
                Ok(())
            }
            Err(e) => Err(format!("Invalid configuration: {}", e))
        }
    }

    pub fn reset_to_defaults(&self) {
        self.config.set(GraphRAGConfig::default());
        self.save_config();
        self.update_active_features();
    }
}

// Reactive signals for UI components
pub fn create_graphrag_signals() -> (
    Signal<GraphRAGConfig>,
    Signal<GraphRAGMetrics>,
    GraphRAGConfigManager
) {
    let manager = GraphRAGConfigManager::new();
    // make manager available globally for metrics updates from retrieval
    set_global_graphrag_manager(&manager);
    let manager_for_config = manager.clone();
    let manager_for_metrics = manager.clone();
    let config_signal = Signal::derive(move || manager_for_config.get_config());
    let metrics_signal = Signal::derive(move || manager_for_metrics.get_metrics());
    
    (config_signal, metrics_signal, manager)
}
