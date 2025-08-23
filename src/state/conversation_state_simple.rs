use leptos::prelude::*;
use serde::{Deserialize, Serialize};
use crate::models::{
    app::AppError,
    chat::{Conversation, Message},
};

/// Simplified conversation state for chat management
#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct ConversationState {
    pub conversations: Vec<Conversation>,
    pub current_conversation_id: Option<String>,
    pub current_message: String,
    pub is_sending: bool,
    pub is_streaming: bool,
    pub error: Option<AppError>,
}

impl Default for ConversationStateContext {
    fn default() -> Self {
        Self::new()
    }
}



/// Conversation state context for chat management
#[derive(Clone)]
pub struct ConversationStateContext {
    pub state: RwSignal<ConversationState>,
}

impl ConversationStateContext {
    pub fn new() -> Self {
        Self {
            state: RwSignal::new(ConversationState::default()),
        }
    }

    // Conversation methods
    pub fn get_conversations(&self) -> Vec<Conversation> {
        self.state.get().conversations
    }

    pub fn add_conversation(&self, conversation: Conversation) {
        self.state.update(|s| s.conversations.push(conversation));
    }

    pub fn get_current_conversation(&self) -> Option<Conversation> {
        let state = self.state.get();
        if let Some(id) = &state.current_conversation_id {
            state.conversations.iter().find(|c| &c.id == id).cloned()
        } else {
            None
        }
    }

    pub fn set_current_conversation(&self, id: Option<String>) {
        self.state.update(|s| s.current_conversation_id = id);
    }

    pub fn create_new_conversation(&self, title: Option<String>) {
        let conversation = Conversation::new(title.unwrap_or_else(|| "New Conversation".to_string()));
        let id = conversation.id.clone();
        self.add_conversation(conversation);
        self.set_current_conversation(Some(id));
    }

    // Message methods
    pub fn get_current_message(&self) -> String {
        self.state.get().current_message
    }

    pub fn set_current_message(&self, message: String) {
        self.state.update(|s| s.current_message = message);
    }

    pub fn clear_current_message(&self) {
        self.state.update(|s| s.current_message.clear());
    }

    pub fn add_message_to_current(&self, message: Message) {
        self.state.update(|s| {
            if let Some(id) = &s.current_conversation_id {
                if let Some(conv) = s.conversations.iter_mut().find(|c| &c.id == id) {
                    conv.messages.push(message);
                }
            }
        });
    }

    // State methods
    pub fn is_sending(&self) -> bool {
        self.state.get().is_sending
    }

    pub fn set_sending(&self, sending: bool) {
        self.state.update(|s| s.is_sending = sending);
    }

    pub fn is_streaming(&self) -> bool {
        self.state.get().is_streaming
    }

    pub fn set_streaming(&self, streaming: bool) {
        self.state.update(|s| s.is_streaming = streaming);
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

    // Utility methods
    pub fn get_current_messages(&self) -> Vec<Message> {
        if let Some(conv) = self.get_current_conversation() {
            conv.messages
        } else {
            Vec::new()
        }
    }

    pub fn delete_conversation(&self, id: &str) {
        self.state.update(|s| {
            s.conversations.retain(|c| c.id != id);
            if s.current_conversation_id.as_ref() == Some(&id.to_string()) {
                s.current_conversation_id = None;
            }
        });
    }
}

/// Provider component for conversation state
#[allow(non_snake_case)]
#[component]
pub fn ConversationStateProvider(children: Children) -> impl IntoView {
    let conversation_state = ConversationStateContext::new();
    provide_context(conversation_state);
    children()
}

/// Hook to use conversation state context
pub fn use_conversation_state() -> ConversationStateContext {
    expect_context::<ConversationStateContext>()
}

#[cfg(test)]
#[cfg(target_arch = "wasm32")]
mod tests {
    use super::*;
    use crate::models::chat::MessageRole;

    #[test]
    fn test_conversation_state_creation() {
        let ctx = ConversationStateContext::new();
        assert!(ctx.get_conversations().is_empty());
        assert!(ctx.get_current_conversation().is_none());
        assert!(!ctx.is_sending());
        assert!(!ctx.is_streaming());
    }

    #[test]
    fn test_conversation_management() {
        let ctx = ConversationStateContext::new();
        
        // Create new conversation
        ctx.create_new_conversation(Some("Test Conversation".to_string()));
        assert_eq!(ctx.get_conversations().len(), 1);
        assert!(ctx.get_current_conversation().is_some());
        
        // Add message
        let message = Message::new(MessageRole::User, "Hello".to_string());
        ctx.add_message_to_current(message);
        assert_eq!(ctx.get_current_messages().len(), 1);
    }
}
