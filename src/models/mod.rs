// Re-export all model modules
pub mod app;
pub mod chat;
pub mod crm;
pub mod graph_store;
pub mod graphrag;
pub mod webllm;

// Re-export commonly used types
pub use app::{AppConfig, AppError, AppResult, ThemeMode};
pub use chat::{Conversation, Message, MessageMetadata, MessageRole, SourceAttribution};
pub use crm::{Contact, Customer, Deal, Lead, PipelineStage};
pub use graphrag::{
    DocumentIndex, GraphEdge, GraphNode, PerformanceMode, RAGQuery, RAGResult, SearchStrategy,
};
pub use webllm::{ChatSession, LLMModel, ModelConfig, ModelStatus};
