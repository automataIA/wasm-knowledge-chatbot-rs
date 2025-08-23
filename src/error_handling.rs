use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum EngineError {
    InvalidConfig(String),
    Graph(String),
    DidNotConverge,
    Internal(String),
}

impl core::fmt::Display for EngineError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            EngineError::InvalidConfig(msg) => write!(f, "invalid configuration: {}", msg),
            EngineError::Graph(msg) => write!(f, "graph access error: {}", msg),
            EngineError::DidNotConverge => write!(f, "computation failed to converge"),
            EngineError::Internal(msg) => write!(f, "internal error: {}", msg),
        }
    }
}

impl std::error::Error for EngineError {}

pub type EngineResult<T> = Result<T, EngineError>;

// UI-facing application error type for mapping various failure modes to user-friendly categories.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum AppError {
    Storage(String),
    Validation(String),
    Network(String),
    Processing(String),
    Internal(String),
}

impl core::fmt::Display for AppError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            AppError::Storage(msg) => write!(f, "storage error: {}", msg),
            AppError::Validation(msg) => write!(f, "validation error: {}", msg),
            AppError::Network(msg) => write!(f, "network error: {}", msg),
            AppError::Processing(msg) => write!(f, "processing error: {}", msg),
            AppError::Internal(msg) => write!(f, "internal error: {}", msg),
        }
    }
}

impl std::error::Error for AppError {}
