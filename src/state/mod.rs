// State management modules - Simplified for Leptos 0.8 compatibility

pub mod app_state_simple;
pub mod webllm_state_simple;
pub mod conversation_state_simple;
pub mod actions_simple;
pub mod mod_simple;
pub mod integration_test;
pub mod graphrag_state_simple;
pub mod crm_state_simple;
pub mod knowledge_storage_context;

// Re-export all state management functionality
pub use mod_simple::*;
pub use app_state_simple::{AppStateContext, AppStateProvider, use_app_state};
pub use webllm_state_simple::{WebLLMStateContext, WebLLMStateProvider, use_webllm_state};
pub use conversation_state_simple::{ConversationStateContext, ConversationStateProvider, use_conversation_state};
pub use actions_simple::{WebLLMActions, ConversationActions, AppActions};
pub use graphrag_state_simple::{GraphRAGStateContext, GraphRAGStateProvider, use_graphrag_state};
pub use crm_state_simple::{CRMStateContext, CRMStateProvider, use_crm_state};
pub use knowledge_storage_context::KnowledgeStorageContext;
