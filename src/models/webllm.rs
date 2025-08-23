use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LLMModel {
    pub id: String,
    pub name: String,
    pub provider: String,
    pub logo_slug: String,
    pub size_mb: Option<u32>,
    pub context_length: Option<u32>,
    pub capabilities: Vec<ModelCapability>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum ModelCapability {
    TextGeneration,
    VisionUnderstanding,
    CodeGeneration,
    Reasoning,
    MultiModal,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ModelConfig {
    pub model_id: String,
    pub temperature: f32,
    pub max_tokens: Option<u32>,
    pub top_p: Option<f32>,
    pub frequency_penalty: Option<f32>,
    pub presence_penalty: Option<f32>,
    pub stop_sequences: Vec<String>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum ModelStatus {
    NotInitialized,
    Downloading {
        progress: f32,
        eta_seconds: Option<u32>,
    },
    Loading {
        progress: f32,
    },
    Ready,
    Error {
        message: String,
    },
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ChatSession {
    pub id: String,
    pub model_id: String,
    pub config: ModelConfig,
    pub created_at: f64,
    pub total_tokens: u32,
    pub message_count: u32,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ModelInitProgress {
    pub phase: InitPhase,
    pub progress: f32, // 0.0 to 1.0
    pub message: String,
    pub eta_seconds: Option<u32>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum InitPhase {
    Downloading,
    Loading,
    Compiling,
    Ready,
}

impl Default for ModelConfig {
    fn default() -> Self {
        Self {
            model_id: String::new(),
            temperature: 0.7,
            max_tokens: Some(2048),
            top_p: Some(0.9),
            frequency_penalty: None,
            presence_penalty: None,
            stop_sequences: Vec::new(),
        }
    }
}

impl ModelStatus {
    pub fn is_ready(&self) -> bool {
        matches!(self, ModelStatus::Ready)
    }

    pub fn is_loading(&self) -> bool {
        matches!(
            self,
            ModelStatus::Downloading { .. } | ModelStatus::Loading { .. }
        )
    }

    pub fn is_error(&self) -> bool {
        matches!(self, ModelStatus::Error { .. })
    }

    pub fn progress(&self) -> Option<f32> {
        match self {
            ModelStatus::Downloading { progress, .. } | ModelStatus::Loading { progress } => {
                Some(*progress)
            }
            _ => None,
        }
    }
}

impl LLMModel {
    pub fn new(id: String, name: String, provider: String, logo_slug: String) -> Self {
        Self {
            id,
            name,
            provider,
            logo_slug,
            size_mb: None,
            context_length: None,
            capabilities: Vec::new(),
        }
    }

    pub fn with_size(mut self, size_mb: u32) -> Self {
        self.size_mb = Some(size_mb);
        self
    }

    pub fn with_context_length(mut self, context_length: u32) -> Self {
        self.context_length = Some(context_length);
        self
    }

    pub fn with_capabilities(mut self, capabilities: Vec<ModelCapability>) -> Self {
        self.capabilities = capabilities;
        self
    }
}

impl ChatSession {
    pub fn new(model_id: String, config: ModelConfig) -> Self {
        Self {
            id: format!("{}", js_sys::Date::now()),
            model_id,
            config,
            created_at: js_sys::Date::now(),
            total_tokens: 0,
            message_count: 0,
        }
    }

    pub fn add_tokens(&mut self, tokens: u32) {
        self.total_tokens += tokens;
        self.message_count += 1;
    }
}
