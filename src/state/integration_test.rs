// Integration test for simplified state modules
use leptos::prelude::*;
use crate::models::{app::AppError, chat::Message, chat::MessageRole, webllm::LLMModel};
use crate::state::{
    AppStateContext, WebLLMStateContext, ConversationStateContext,
    // WebLLMActions, ConversationActions, AppActions,
    GlobalStateProvider
};

/// Test basic state initialization and operations
pub fn test_simplified_state_integration() {
    // Test AppState
    let app_ctx = AppStateContext::new();
    assert!(!app_ctx.is_loading());
    assert!(app_ctx.get_error().is_none());
    
    app_ctx.set_loading(true);
    assert!(app_ctx.is_loading());
    
    app_ctx.set_error(Some(AppError::network("Test error".to_string())));
    assert!(app_ctx.get_error().is_some());
    
    app_ctx.clear_error();
    assert!(app_ctx.get_error().is_none());

    // Test WebLLMState
    let webllm_ctx = WebLLMStateContext::new();
    assert!(webllm_ctx.get_current_model().is_none());
    assert_eq!(webllm_ctx.get_initialization_progress(), 0.0);
    
    let test_model = LLMModel::new(
        "test-model".to_string(),
        "Test Model".to_string(),
        "TestProvider".to_string(),
        "test-logo".to_string()
    );
    
    webllm_ctx.set_current_model(Some(test_model.clone()));
    assert!(webllm_ctx.get_current_model().is_some());
    
    webllm_ctx.set_initialization_progress(50.0);
    assert_eq!(webllm_ctx.get_initialization_progress(), 50.0);

    // Test ConversationState
    let conv_ctx = ConversationStateContext::new();
    assert_eq!(conv_ctx.get_conversations().len(), 0);
    assert!(conv_ctx.get_current_conversation().is_none());
    
    conv_ctx.create_new_conversation(Some("Test Conversation".to_string()));
    assert_eq!(conv_ctx.get_conversations().len(), 1);
    assert!(conv_ctx.get_current_conversation().is_some());
    
    let message = Message::new(MessageRole::User, "Hello, world!".to_string());
    conv_ctx.add_message_to_current(message);
    assert_eq!(conv_ctx.get_current_messages().len(), 1);
}

/// Test GlobalStateProvider component
#[allow(non_snake_case)]
#[component]
pub fn TestGlobalStateProvider() -> impl IntoView {
    view! {
        <GlobalStateProvider>
            <div>"State provider test"</div>
        </GlobalStateProvider>
    }
}

#[cfg(test)]
#[cfg(target_arch = "wasm32")]
mod tests {
    use super::*;

    #[test]
    fn test_state_contexts() {
        test_simplified_state_integration();
    }
}
