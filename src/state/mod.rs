// State management modules - Simplified for Leptos 0.8 compatibility

pub mod actions_simple;
pub mod app_state_simple;
pub mod conversation_state_simple;
pub mod crm_state_simple;
pub mod graphrag_state_simple;
pub mod integration_test;
pub mod knowledge_storage_context;
pub mod mod_simple;
pub mod webllm_state_simple;

// Re-export all state management functionality
pub use actions_simple::{AppActions, ConversationActions, WebLLMActions};
pub use app_state_simple::{use_app_state, AppStateContext, AppStateProvider};
pub use conversation_state_simple::{
    use_conversation_state, ConversationStateContext, ConversationStateProvider,
};
pub use crm_state_simple::{use_crm_state, CRMStateContext, CRMStateProvider};
pub use graphrag_state_simple::{use_graphrag_state, GraphRAGStateContext, GraphRAGStateProvider};
pub use knowledge_storage_context::KnowledgeStorageContext;
pub use mod_simple::*;
pub use webllm_state_simple::{use_webllm_state, WebLLMStateContext, WebLLMStateProvider};
