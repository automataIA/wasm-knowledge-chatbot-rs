use serde::{Deserialize, Serialize};

#[derive(Clone, PartialEq, Debug, Serialize, Deserialize)]
pub enum MessageRole {
    User,
    Assistant,
    System,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Message {
    pub id: String,
    pub role: MessageRole,
    pub content: String,
    pub timestamp: f64, // Using f64 for js_sys::Date compatibility
    pub metadata: Option<MessageMetadata>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct MessageMetadata {
    pub tokens_used: Option<u32>,
    pub processing_time_ms: Option<u32>,
    pub model_used: Option<String>,
    pub graphrag_enhanced: bool,
    pub error: Option<String>,
    // Optional multi-document provenance for transparency
    pub provenance: Option<Vec<SourceAttribution>>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SourceAttribution {
    pub source_id: String,
    pub title: String,
    /// Confidence in range 0.0..1.0
    pub confidence: f32,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Conversation {
    pub id: String,
    pub title: String,
    pub messages: Vec<Message>,
    pub created_at: f64,
    pub last_message: f64,
    pub model_id: Option<String>,
    pub settings: ConversationSettings,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ConversationSettings {
    pub graphrag_enabled: bool,
    pub temperature: f32,
    pub max_tokens: Option<u32>,
    pub system_prompt: Option<String>,
}

impl Default for ConversationSettings {
    fn default() -> Self {
        Self {
            graphrag_enabled: false,
            temperature: 0.7,
            max_tokens: None,
            system_prompt: None,
        }
    }
}

impl Message {
    pub fn new(role: MessageRole, content: String) -> Self {
        let timestamp = js_sys::Date::now();
        Self {
            id: format!("{}", timestamp),
            role,
            content,
            timestamp,
            metadata: None,
        }
    }

    pub fn with_metadata(mut self, metadata: MessageMetadata) -> Self {
        self.metadata = Some(metadata);
        self
    }
}

impl Conversation {
    pub fn new(title: String) -> Self {
        let timestamp = js_sys::Date::now();
        Self {
            id: format!("{}", timestamp),
            title,
            messages: Vec::new(),
            created_at: timestamp,
            last_message: timestamp,
            model_id: None,
            settings: ConversationSettings::default(),
        }
    }

    pub fn add_message(&mut self, message: Message) {
        self.last_message = message.timestamp;
        self.messages.push(message);
    }

    pub fn message_count(&self) -> usize {
        self.messages.len()
    }

    pub fn last_user_message(&self) -> Option<&Message> {
        self.messages
            .iter()
            .rev()
            .find(|msg| matches!(msg.role, MessageRole::User))
    }

    pub fn last_assistant_message(&self) -> Option<&Message> {
        self.messages
            .iter()
            .rev()
            .find(|msg| matches!(msg.role, MessageRole::Assistant))
    }
}
