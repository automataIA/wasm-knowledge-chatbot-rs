use crate::models::{
    app::AppError,
    chat::{Message, MessageRole},
    webllm::{LLMModel, ModelStatus},
};
use crate::state::{
    app_state_simple::AppStateContext, conversation_state_simple::ConversationStateContext,
    webllm_state_simple::WebLLMStateContext,
};
use gloo_timers::future::TimeoutFuture;
use leptos::prelude::*;

/// Actions for WebLLM operations
pub struct WebLLMActions;

impl WebLLMActions {
    /// Initialize a model
    pub async fn initialize_model(
        webllm_ctx: &WebLLMStateContext,
        app_ctx: &AppStateContext,
        model: LLMModel,
    ) -> Result<(), AppError> {
        // Set loading states
        app_ctx.set_loading(true);
        webllm_ctx.set_current_model(Some(model.clone()));
        webllm_ctx.set_model_status(ModelStatus::Loading { progress: 0.0 });
        webllm_ctx.clear_error();

        // Simulate model initialization with progress updates
        for progress in (0..=100).step_by(10) {
            webllm_ctx.set_initialization_progress(progress as f32);

            // Simulate async delay
            TimeoutFuture::new(100).await;
        }

        // Set ready state
        webllm_ctx.set_model_status(ModelStatus::Ready);
        app_ctx.set_loading(false);

        Ok(())
    }

    /// Generate response from model
    pub async fn generate_response(
        webllm_ctx: &WebLLMStateContext,
        conversation_ctx: &ConversationStateContext,
        content: String,
    ) -> Result<String, AppError> {
        // Check if model is ready
        if !webllm_ctx.is_model_ready() {
            return Err(AppError::ModelLoadError("Model not ready".to_string()));
        }

        // Set generation state
        webllm_ctx.set_generating(true);
        conversation_ctx.set_streaming(true);
        webllm_ctx.clear_error();

        // Add user message
        let user_message = Message::new(MessageRole::User, content.clone());
        conversation_ctx.add_message_to_current(user_message);

        // Simulate response generation
        let response = format!("This is a simulated response to: {}", content);

        // Simulate streaming delay
        TimeoutFuture::new(1000).await;

        // Add assistant message
        let assistant_message = Message::new(MessageRole::Assistant, response.clone());
        conversation_ctx.add_message_to_current(assistant_message);

        // Clear generation states
        webllm_ctx.set_generating(false);
        conversation_ctx.set_streaming(false);

        Ok(response)
    }

    /// Load available models
    pub async fn load_available_models(
        webllm_ctx: &WebLLMStateContext,
    ) -> Result<Vec<LLMModel>, AppError> {
        // Simulate loading models
        TimeoutFuture::new(500).await;

        let models = vec![
            LLMModel::new(
                "llama2-7b".to_string(),
                "Llama 2 7B".to_string(),
                "Meta".to_string(),
                "llama".to_string(),
            )
            .with_size(3500)
            .with_context_length(4096),
            LLMModel::new(
                "mistral-7b".to_string(),
                "Mistral 7B".to_string(),
                "Mistral AI".to_string(),
                "mistral".to_string(),
            )
            .with_size(4100)
            .with_context_length(8192),
        ];

        webllm_ctx.set_available_models(models.clone());
        Ok(models)
    }
}

/// Actions for conversation operations
pub struct ConversationActions;

impl ConversationActions {
    /// Send a message
    pub async fn send_message(
        webllm_ctx: &WebLLMStateContext,
        conversation_ctx: &ConversationStateContext,
        content: String,
    ) -> Result<(), AppError> {
        if content.trim().is_empty() {
            return Err(AppError::ValidationError(
                "Message cannot be empty".to_string(),
            ));
        }

        // Ensure we have a current conversation
        if conversation_ctx.get_current_conversation().is_none() {
            conversation_ctx.create_new_conversation(None);
        }

        conversation_ctx.set_sending(true);
        conversation_ctx.clear_error();

        // Generate response using WebLLM
        match WebLLMActions::generate_response(webllm_ctx, conversation_ctx, content).await {
            Ok(_) => {
                conversation_ctx.clear_current_message();
                conversation_ctx.set_sending(false);
                Ok(())
            }
            Err(e) => {
                conversation_ctx.set_error(Some(e.clone()));
                conversation_ctx.set_sending(false);
                Err(e)
            }
        }
    }

    /// Create a new conversation
    pub fn create_conversation(
        conversation_ctx: &ConversationStateContext,
        title: Option<String>,
    ) -> String {
        conversation_ctx.create_new_conversation(title);
        "New conversation created".to_string()
    }

    /// Delete a conversation
    pub fn delete_conversation(conversation_ctx: &ConversationStateContext, id: String) {
        conversation_ctx.delete_conversation(&id);
    }
}

/// Actions for app operations
pub struct AppActions;

impl AppActions {
    /// Initialize the application
    pub async fn initialize_app(
        app_ctx: &AppStateContext,
        webllm_ctx: &WebLLMStateContext,
    ) -> Result<(), AppError> {
        app_ctx.set_loading(true);
        app_ctx.clear_error();

        // Load available models
        match WebLLMActions::load_available_models(webllm_ctx).await {
            Ok(_) => {
                app_ctx.set_loading(false);
                Ok(())
            }
            Err(e) => {
                let _current_theme = app_ctx.state.get().theme;
                app_ctx.set_loading(false);
                Err(e)
            }
        }
    }

    /// Handle errors globally
    pub fn handle_error(app_ctx: &AppStateContext, error: AppError) {
        app_ctx.set_error(Some(error));
        app_ctx.set_loading(false);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use wasm_bindgen_test::*;

    #[wasm_bindgen_test]
    async fn test_model_initialization() {
        let app_ctx = AppStateContext::new();
        let webllm_ctx = WebLLMStateContext::new();

        let model = LLMModel::new(
            "test-model".to_string(),
            "Test Model".to_string(),
            "Test Provider".to_string(),
            "test".to_string(),
        );

        let result = WebLLMActions::initialize_model(&webllm_ctx, &app_ctx, model).await;
        assert!(result.is_ok());
        assert!(webllm_ctx.is_model_ready());
        assert!(!app_ctx.is_loading());
    }

    #[wasm_bindgen_test]
    async fn test_send_message() {
        let app_ctx = AppStateContext::new();
        let webllm_ctx = WebLLMStateContext::new();
        let conversation_ctx = ConversationStateContext::new();

        // Initialize model first
        let model = LLMModel::new(
            "test-model".to_string(),
            "Test Model".to_string(),
            "Test Provider".to_string(),
            "test".to_string(),
        );
        WebLLMActions::initialize_model(&webllm_ctx, &app_ctx, model)
            .await
            .unwrap();

        // Send message
        let result = ConversationActions::send_message(
            &webllm_ctx,
            &conversation_ctx,
            "Hello, world!".to_string(),
        )
        .await;

        assert!(result.is_ok());
        assert_eq!(conversation_ctx.get_current_messages().len(), 2); // User + Assistant
    }
}
