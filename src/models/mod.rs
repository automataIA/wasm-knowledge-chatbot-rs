// Re-export all model modules
pub mod chat;
pub mod webllm;
pub mod graphrag;
pub mod app;
pub mod crm;
pub mod graph_store;

// Re-export commonly used types
pub use chat::{Message, MessageRole, Conversation, MessageMetadata, SourceAttribution};
pub use webllm::{LLMModel, ModelConfig, ModelStatus, ChatSession};
pub use graphrag::{GraphNode, GraphEdge, RAGQuery, RAGResult, DocumentIndex, SearchStrategy, PerformanceMode};
pub use app::{AppConfig, ThemeMode, AppError, AppResult};
pub use crm::{Customer, Lead, Contact, PipelineStage, Deal};
