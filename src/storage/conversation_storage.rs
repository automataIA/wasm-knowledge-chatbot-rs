use crate::models::Message;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Conversation {
    pub id: String,
    pub title: String,
    pub created_at: f64,
    pub updated_at: f64,
    pub messages: Vec<Message>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConversationInfo {
    pub id: String,
    pub title: String,
    pub updated_at: f64,
}

#[derive(Clone)]
pub struct ConversationStorage {
    storage_key: String,
}

impl ConversationStorage {
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        Ok(Self {
            storage_key: "wasm_llm_conversations".to_string(),
        })
    }
    
    fn get_local_storage(&self) -> Result<web_sys::Storage, Box<dyn std::error::Error>> {
        let window = web_sys::window().ok_or("No window object")?;
        let storage = window.local_storage()
            .map_err(|_| "Failed to get localStorage")?
            .ok_or("localStorage not available")?;
        Ok(storage)
    }
    
    fn load_conversations(&self) -> Result<Vec<Conversation>, Box<dyn std::error::Error>> {
        let storage = self.get_local_storage()?;
        
        match storage.get_item(&self.storage_key) {
            Ok(Some(data)) => {
                let conversations: Vec<Conversation> = serde_json::from_str(&data)?;
                Ok(conversations)
            }
            Ok(None) => Ok(vec![]),
            Err(_) => Ok(vec![]),
        }
    }
    
    fn save_conversations(&self, conversations: &[Conversation]) -> Result<(), Box<dyn std::error::Error>> {
        let storage = self.get_local_storage()?;
        let data = serde_json::to_string(conversations)?;
        storage.set_item(&self.storage_key, &data)
            .map_err(|_| "Failed to save to localStorage")?;
        Ok(())
    }
    
    pub fn create_conversation(&self, title: String) -> Result<String, Box<dyn std::error::Error>> {
        let mut conversations = self.load_conversations()?;
        let conversation_id = Uuid::new_v4().to_string();
        let now = js_sys::Date::now();
        
        let conversation = Conversation {
            id: conversation_id.clone(),
            title,
            created_at: now,
            updated_at: now,
            messages: vec![],
        };
        
        conversations.push(conversation);
        self.save_conversations(&conversations)?;
        
        Ok(conversation_id)
    }
    
    pub fn save_message(&self, conversation_id: &str, message: &Message) -> Result<(), Box<dyn std::error::Error>> {
        let mut conversations = self.load_conversations()?;
        let now = js_sys::Date::now();
        
        if let Some(conversation) = conversations.iter_mut().find(|c| c.id == conversation_id) {
            conversation.messages.push(message.clone());
            conversation.updated_at = now;
            self.save_conversations(&conversations)?;
        }
        
        Ok(())
    }
    
    pub fn load_conversation(&self, conversation_id: &str) -> Result<Option<Vec<Message>>, Box<dyn std::error::Error>> {
        let conversations = self.load_conversations()?;
        
        if let Some(conversation) = conversations.iter().find(|c| c.id == conversation_id) {
            if conversation.messages.is_empty() {
                Ok(None)
            } else {
                Ok(Some(conversation.messages.clone()))
            }
        } else {
            Ok(None)
        }
    }
    
    pub fn list_conversations(&self) -> Result<Vec<ConversationInfo>, Box<dyn std::error::Error>> {
        let conversations = self.load_conversations()?;
        
        let mut result: Vec<ConversationInfo> = conversations
            .into_iter()
            .map(|c| ConversationInfo {
                id: c.id,
                title: c.title,
                updated_at: c.updated_at,
            })
            .collect();
        
        // Sort by updated_at descending
        result.sort_by(|a, b| b.updated_at.partial_cmp(&a.updated_at).unwrap_or(std::cmp::Ordering::Equal));
        
        Ok(result)
    }
    
    #[allow(dead_code)]
    pub fn delete_conversation(&self, conversation_id: &str) -> Result<(), Box<dyn std::error::Error>> {
        let mut conversations = self.load_conversations()?;
        conversations.retain(|c| c.id != conversation_id);
        self.save_conversations(&conversations)?;
        Ok(())
    }
    
    #[allow(dead_code)]
    pub fn update_conversation_title(&self, conversation_id: &str, new_title: String) -> Result<(), Box<dyn std::error::Error>> {
        let mut conversations = self.load_conversations()?;
        let now = js_sys::Date::now();
        
        if let Some(conversation) = conversations.iter_mut().find(|c| c.id == conversation_id) {
            conversation.title = new_title;
            conversation.updated_at = now;
            self.save_conversations(&conversations)?;
        }
        
        Ok(())
    }
}