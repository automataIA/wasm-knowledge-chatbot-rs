use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AppConfig {
    pub theme: ThemeMode,
    pub language: String,
    pub auto_save: bool,
    pub performance_mode: PerformanceMode,
    pub accessibility: AccessibilityConfig,
    pub ui_preferences: UIPreferences,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum ThemeMode {
    Light,
    Dark,
    Auto,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum PerformanceMode {
    High,
    Balanced,
    Battery,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AccessibilityConfig {
    pub high_contrast: bool,
    pub reduced_motion: bool,
    pub font_size_scale: f32, // 0.8 to 2.0
    pub focus_indicators: bool,
    pub screen_reader_mode: bool,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct UIPreferences {
    pub sidebar_collapsed: bool,
    pub show_timestamps: bool,
    pub compact_mode: bool,
    pub animation_speed: AnimationSpeed,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum AnimationSpeed {
    Disabled,
    Slow,
    Normal,
    Fast,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AppError {
    // Network and connectivity
    NetworkError(String),
    ConnectionTimeout,

    // Storage and persistence
    StorageError(String),
    SerializationError(String),

    // WebLLM specific
    ModelNotFound(String),
    ModelLoadError(String),
    InferenceError(String),

    // GraphRAG specific
    GraphRAGError(String),
    IndexingError(String),
    QueryError(String),

    // Validation and input
    ValidationError(String),
    InvalidInput(String),

    // Configuration
    ConfigurationError(String),

    // Generic
    InternalError(String),
    NotImplemented(String),
}

pub type AppResult<T> = Result<T, AppError>;

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            theme: ThemeMode::Auto,
            language: "en".to_string(),
            auto_save: true,
            performance_mode: PerformanceMode::Balanced,
            accessibility: AccessibilityConfig::default(),
            ui_preferences: UIPreferences::default(),
        }
    }
}

impl Default for AccessibilityConfig {
    fn default() -> Self {
        Self {
            high_contrast: false,
            reduced_motion: false,
            font_size_scale: 1.0,
            focus_indicators: true,
            screen_reader_mode: false,
        }
    }
}

impl Default for UIPreferences {
    fn default() -> Self {
        Self {
            sidebar_collapsed: false,
            show_timestamps: true,
            compact_mode: false,
            animation_speed: AnimationSpeed::Normal,
        }
    }
}

impl std::fmt::Display for AppError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AppError::NetworkError(msg) => write!(f, "Network error: {}", msg),
            AppError::ConnectionTimeout => write!(f, "Connection timeout"),
            AppError::StorageError(msg) => write!(f, "Storage error: {}", msg),
            AppError::SerializationError(msg) => write!(f, "Serialization error: {}", msg),
            AppError::ModelNotFound(model) => write!(f, "Model not found: {}", model),
            AppError::ModelLoadError(msg) => write!(f, "Model load error: {}", msg),
            AppError::InferenceError(msg) => write!(f, "Inference error: {}", msg),
            AppError::GraphRAGError(msg) => write!(f, "GraphRAG error: {}", msg),
            AppError::IndexingError(msg) => write!(f, "Indexing error: {}", msg),
            AppError::QueryError(msg) => write!(f, "Query error: {}", msg),
            AppError::ValidationError(msg) => write!(f, "Validation error: {}", msg),
            AppError::InvalidInput(msg) => write!(f, "Invalid input: {}", msg),
            AppError::ConfigurationError(msg) => write!(f, "Configuration error: {}", msg),
            AppError::InternalError(msg) => write!(f, "Internal error: {}", msg),
            AppError::NotImplemented(feature) => write!(f, "Not implemented: {}", feature),
        }
    }
}

impl std::error::Error for AppError {}

impl From<serde_json::Error> for AppError {
    fn from(err: serde_json::Error) -> Self {
        AppError::SerializationError(err.to_string())
    }
}

impl AppError {
    // Constructor methods for common error types
    pub fn validation(message: String) -> Self {
        AppError::ValidationError(message)
    }

    pub fn network(message: String) -> Self {
        AppError::NetworkError(message)
    }

    pub fn storage(message: String) -> Self {
        AppError::StorageError(message)
    }

    pub fn runtime(message: String) -> Self {
        AppError::InternalError(message)
    }

    pub fn model_error(message: String) -> Self {
        AppError::ModelLoadError(message)
    }

    pub fn graphrag(message: String) -> Self {
        AppError::GraphRAGError(message)
    }

    pub fn is_recoverable(&self) -> bool {
        matches!(
            self,
            AppError::NetworkError(_)
                | AppError::ConnectionTimeout
                | AppError::ValidationError(_)
                | AppError::InvalidInput(_)
        )
    }

    pub fn severity(&self) -> ErrorSeverity {
        match self {
            AppError::InternalError(_) | AppError::ModelLoadError(_) => ErrorSeverity::Critical,
            AppError::NetworkError(_) | AppError::StorageError(_) => ErrorSeverity::High,
            AppError::ValidationError(_) | AppError::InvalidInput(_) => ErrorSeverity::Medium,
            AppError::NotImplemented(_) => ErrorSeverity::Low,
            _ => ErrorSeverity::Medium,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum ErrorSeverity {
    Critical,
    High,
    Medium,
    Low,
}
