use serde::{Deserialize, Serialize};

#[derive(Clone, PartialEq, Debug, Serialize, Deserialize)]
pub enum MessageRole {
    User,
    Assistant,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Message {
    pub id: String,
    pub role: MessageRole,
    pub content: String,
    pub timestamp: f64, // Using f64 for js_sys::Date compatibility
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Conversation {
    pub id: String,
    pub title: String,
    pub messages: Vec<Message>,
    pub last_message: f64,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LLMModel {
    pub id: String,
    pub name: String,
    pub provider: String,
    pub logo_slug: String,
}

impl Message {
    pub fn new(role: MessageRole, content: String) -> Self {
        Self {
            id: format!("{}", js_sys::Date::now()),
            role,
            content,
            timestamp: js_sys::Date::now(),
        }
    }
}