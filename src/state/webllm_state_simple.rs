use crate::models::{
    app::AppError,
    webllm::{ChatSession, LLMModel, ModelStatus},
};
use leptos::prelude::*;
use serde::{Deserialize, Serialize};

/// Simplified WebLLM state for model management
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct WebLLMState {
    pub available_models: Vec<LLMModel>,
    pub current_model: Option<LLMModel>,
    pub model_status: ModelStatus,
    pub initialization_progress: f32,
    pub chat_session: Option<ChatSession>,
    pub is_generating: bool,
    pub error: Option<AppError>,
}

impl Default for WebLLMStateContext {
    fn default() -> Self {
        Self::new()
    }
}

impl Default for WebLLMState {
    fn default() -> Self {
        Self {
            available_models: Vec::new(),
            current_model: None,
            model_status: ModelStatus::NotInitialized,
            initialization_progress: 0.0,
            chat_session: None,
            is_generating: false,
            error: None,
        }
    }
}

/// WebLLM state context for model management
#[derive(Clone)]
pub struct WebLLMStateContext {
    pub state: RwSignal<WebLLMState>,
}

impl WebLLMStateContext {
    pub fn new() -> Self {
        Self {
            state: RwSignal::new(WebLLMState::default()),
        }
    }

    // Model methods
    pub fn get_current_model(&self) -> Option<LLMModel> {
        self.state.get().current_model
    }

    pub fn set_current_model(&self, model: Option<LLMModel>) {
        self.state.update(|s| s.current_model = model);
    }

    pub fn get_available_models(&self) -> Vec<LLMModel> {
        self.state.get().available_models
    }

    pub fn set_available_models(&self, models: Vec<LLMModel>) {
        self.state.update(|s| s.available_models = models);
    }

    // Status methods
    pub fn get_model_status(&self) -> ModelStatus {
        self.state.get().model_status
    }

    pub fn set_model_status(&self, status: ModelStatus) {
        self.state.update(|s| s.model_status = status);
    }

    pub fn is_model_ready(&self) -> bool {
        matches!(self.state.get().model_status, ModelStatus::Ready)
    }

    // Progress methods
    pub fn get_initialization_progress(&self) -> f32 {
        self.state.get().initialization_progress
    }

    pub fn set_initialization_progress(&self, progress: f32) {
        self.state.update(|s| s.initialization_progress = progress);
    }

    // Generation methods
    pub fn is_generating(&self) -> bool {
        self.state.get().is_generating
    }

    pub fn set_generating(&self, generating: bool) {
        self.state.update(|s| s.is_generating = generating);
    }

    // Error methods
    pub fn get_error(&self) -> Option<AppError> {
        self.state.get().error
    }

    pub fn set_error(&self, error: Option<AppError>) {
        self.state.update(|s| s.error = error);
    }

    pub fn clear_error(&self) {
        self.state.update(|s| s.error = None);
    }

    // Chat session methods
    pub fn get_chat_session(&self) -> Option<ChatSession> {
        self.state.get().chat_session
    }

    pub fn set_chat_session(&self, session: Option<ChatSession>) {
        self.state.update(|s| s.chat_session = session);
    }
}

/// Provider component for WebLLM state
#[component]
pub fn WebLLMStateProvider(children: Children) -> impl IntoView {
    let webllm_state = WebLLMStateContext::new();
    provide_context(webllm_state);
    children()
}

/// Hook to use WebLLM state context
pub fn use_webllm_state() -> WebLLMStateContext {
    expect_context::<WebLLMStateContext>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_webllm_state_creation() {
        let ctx = WebLLMStateContext::new();
        assert!(ctx.get_current_model().is_none());
        assert_eq!(ctx.get_initialization_progress(), 0.0);
        assert!(!ctx.is_generating());
        assert!(!ctx.is_model_ready());
    }

    #[test]
    fn test_webllm_state_methods() {
        let ctx = WebLLMStateContext::new();

        // Test progress
        ctx.set_initialization_progress(50.0);
        assert_eq!(ctx.get_initialization_progress(), 50.0);

        // Test generation state
        ctx.set_generating(true);
        assert!(ctx.is_generating());

        // Test status
        ctx.set_model_status(ModelStatus::Ready);
        assert!(ctx.is_model_ready());
    }
}
