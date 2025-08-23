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
    /// Optional per-conversation system prompt
    #[serde(default)]
    pub system_prompt: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConversationInfo {
    pub id: String,
    pub title: String,
    pub updated_at: f64,
}

// ---- Export / Import schema and validators (module scope) ----
#[derive(Debug, Clone, Serialize, Deserialize)]
struct ExportBundleV1 {
    version: u8,
    conversations: Vec<Conversation>,
}

fn validate_conversation_schema(c: &Conversation) -> Result<(), Box<dyn std::error::Error>> {
    if c.id.trim().is_empty() {
        return Err("invalid conversation: empty id".into());
    }
    if !c.created_at.is_finite() || !c.updated_at.is_finite() {
        return Err("invalid conversation: timestamps must be finite".into());
    }
    for m in &c.messages {
        if m.content.is_empty() {
            return Err("invalid message: empty content".into());
        }
        if !m.timestamp.is_finite() {
            return Err("invalid message: timestamp must be finite".into());
        }
    }
    // system_prompt is optional; no validation needed beyond size guard if desired
    Ok(())
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
        let storage = window
            .local_storage()
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

    fn save_conversations(
        &self,
        conversations: &[Conversation],
    ) -> Result<(), Box<dyn std::error::Error>> {
        let storage = self.get_local_storage()?;
        let data = serde_json::to_string(conversations)?;
        storage
            .set_item(&self.storage_key, &data)
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
            system_prompt: None,
        };

        conversations.push(conversation);
        self.save_conversations(&conversations)?;

        Ok(conversation_id)
    }

    pub fn save_message(
        &self,
        conversation_id: &str,
        message: &Message,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut conversations = self.load_conversations()?;
        let now = js_sys::Date::now();

        if let Some(conversation) = conversations.iter_mut().find(|c| c.id == conversation_id) {
            conversation.messages.push(message.clone());
            conversation.updated_at = now;
            self.save_conversations(&conversations)?;
        }

        Ok(())
    }

    pub fn load_conversation(
        &self,
        conversation_id: &str,
    ) -> Result<Option<Vec<Message>>, Box<dyn std::error::Error>> {
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
        result.sort_by(|a, b| {
            b.updated_at
                .partial_cmp(&a.updated_at)
                .unwrap_or(std::cmp::Ordering::Equal)
        });

        Ok(result)
    }

    /// Load the per-conversation system prompt, if any
    pub fn load_conversation_system_prompt(
        &self,
        conversation_id: &str,
    ) -> Result<Option<String>, Box<dyn std::error::Error>> {
        let conversations = self.load_conversations()?;
        if let Some(conversation) = conversations.iter().find(|c| c.id == conversation_id) {
            Ok(conversation.system_prompt.clone())
        } else {
            Ok(None)
        }
    }

    /// Update the per-conversation system prompt (set or clear)
    pub fn update_conversation_system_prompt(
        &self,
        conversation_id: &str,
        new_prompt: Option<String>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut conversations = self.load_conversations()?;
        let now = js_sys::Date::now();
        if let Some(conversation) = conversations.iter_mut().find(|c| c.id == conversation_id) {
            conversation.system_prompt = new_prompt.map(|p| p.trim().to_string()).filter(|p| !p.is_empty());
            conversation.updated_at = now;
            self.save_conversations(&conversations)?;
        }
        Ok(())
    }

    #[allow(dead_code)]
    pub fn delete_conversation(
        &self,
        conversation_id: &str,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut conversations = self.load_conversations()?;
        conversations.retain(|c| c.id != conversation_id);
        self.save_conversations(&conversations)?;
        Ok(())
    }

    #[allow(dead_code)]
    pub fn update_conversation_title(
        &self,
        conversation_id: &str,
        new_title: String,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut conversations = self.load_conversations()?;
        let now = js_sys::Date::now();

        if let Some(conversation) = conversations.iter_mut().find(|c| c.id == conversation_id) {
            conversation.title = new_title;
            conversation.updated_at = now;
            self.save_conversations(&conversations)?;
        }

        Ok(())
    }

    // ---- Export / Import utilities ----

    /// Export all conversations as a JSON bundle (schema v1).
    pub fn export_json(&self) -> Result<String, Box<dyn std::error::Error>> {
        let conversations = self.load_conversations()?;
        // Validate before export to ensure bundle is consistent
        for c in &conversations {
            validate_conversation_schema(c)?;
        }
        let bundle = ExportBundleV1 {
            version: 1,
            conversations,
        };
        let json = serde_json::to_string_pretty(&bundle)?;
        Ok(json)
    }

    /// Import conversations from a JSON bundle (schema v1).
    /// If merge = false, replaces existing storage with bundle content.
    /// If merge = true, upserts by id (keeps the latest updated_at on conflict).
    pub fn import_json(
        &self,
        json: &str,
        merge: bool,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let bundle: ExportBundleV1 = serde_json::from_str(json)?;
        if bundle.version != 1 {
            return Err(format!("unsupported export version: {}", bundle.version).into());
        }
        for c in &bundle.conversations {
            validate_conversation_schema(c)?;
        }

        if !merge {
            // Replace
            return self.save_conversations(&bundle.conversations);
        }

        // Merge
        let mut existing = self.load_conversations()?;
        for incoming in bundle.conversations {
            if let Some(slot) = existing.iter_mut().find(|c| c.id == incoming.id) {
                // Keep the newer conversation based on updated_at
                let replace = incoming.updated_at >= slot.updated_at;
                if replace {
                    *slot = incoming;
                }
            } else {
                existing.push(incoming);
            }
        }
        self.save_conversations(&existing)
    }
}
