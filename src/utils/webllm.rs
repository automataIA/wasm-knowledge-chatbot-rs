use crate::models::{
    app::AppError,
    webllm::{LLMModel, ModelConfig, ModelStatus},
};
use web_sys::console;

/// WebLLM utility functions for model management and interaction
pub struct WebLLMUtils;

impl WebLLMUtils {
    /// Check if a model is compatible with the current device
    pub fn is_model_compatible(model: &LLMModel) -> bool {
        // Check WebGPU support
        if !Self::has_webgpu_support() {
            return false;
        }

        // Check memory requirements (rough estimate)
        if let Some(size_mb) = model.size_mb {
            let available_memory = Self::estimate_available_memory();
            // Require at least 2x model size for safe operation
            if available_memory < (size_mb as f64 * 2.0) {
                return false;
            }
        }

        true
    }

    /// Estimate available device memory in MB
    pub fn estimate_available_memory() -> f64 {
        // Use navigator.deviceMemory if available, otherwise estimate
        if let Some(window) = web_sys::window() {
            let navigator = window.navigator();

            // Try to get device memory (Chrome only)
            if let Ok(memory) = js_sys::Reflect::get(&navigator, &"deviceMemory".into()) {
                if let Some(memory_gb) = memory.as_f64() {
                    return memory_gb * 1024.0; // Convert GB to MB
                }
            }

            // Fallback estimation based on user agent
            if let Ok(user_agent) = navigator.user_agent() {
                if user_agent.contains("Mobile") {
                    return 4096.0; // Assume 4GB for mobile devices
                }
            }
        }

        8192.0 // Default assumption for desktop devices
    }

    /// Check WebGPU support
    pub fn has_webgpu_support() -> bool {
        if let Some(window) = web_sys::window() {
            let navigator = window.navigator();
            js_sys::Reflect::has(&navigator, &"gpu".into()).unwrap_or(false)
        } else {
            false
        }
    }

    /// Get recommended models for the current device
    pub fn get_recommended_models(all_models: &[LLMModel]) -> Vec<LLMModel> {
        let available_memory = Self::estimate_available_memory();

        all_models
            .iter()
            .filter(|model| {
                Self::is_model_compatible(model)
                    && model
                        .size_mb
                        .is_none_or(|size| (size as f64) < available_memory / 2.0)
            })
            .cloned()
            .collect()
    }

    /// Format model size for display
    pub fn format_model_size(size_mb: Option<u32>) -> String {
        match size_mb {
            Some(size) if size >= 1024 => format!("{:.1} GB", size as f32 / 1024.0),
            Some(size) => format!("{} MB", size),
            None => "Unknown".to_string(),
        }
    }

    /// Get model status display text
    pub fn get_status_text(status: &ModelStatus) -> String {
        match status {
            ModelStatus::NotInitialized => "Not initialized".to_string(),
            ModelStatus::Downloading {
                progress,
                eta_seconds,
            } => {
                let eta_text = eta_seconds
                    .map(|eta| format!(" (ETA: {}s)", eta))
                    .unwrap_or_default();
                format!("Downloading {:.1}%{}", progress * 100.0, eta_text)
            }
            ModelStatus::Loading { progress } => {
                format!("Loading {:.1}%", progress * 100.0)
            }
            ModelStatus::Ready => "Ready".to_string(),
            ModelStatus::Error { message } => format!("Error: {}", message),
        }
    }

    /// Create default model configuration
    pub fn create_default_config(model_id: String) -> ModelConfig {
        ModelConfig {
            model_id,
            temperature: 0.7,
            max_tokens: Some(2048),
            top_p: Some(0.9),
            frequency_penalty: None,
            presence_penalty: None,
            stop_sequences: Vec::new(),
        }
    }

    /// Validate model configuration
    pub fn validate_config(config: &ModelConfig) -> Result<(), AppError> {
        if config.model_id.is_empty() {
            return Err(AppError::validation("Model ID cannot be empty".to_string()));
        }

        if config.temperature < 0.0 || config.temperature > 2.0 {
            return Err(AppError::validation(
                "Temperature must be between 0.0 and 2.0".to_string(),
            ));
        }

        if let Some(max_tokens) = config.max_tokens {
            if max_tokens == 0 || max_tokens > 32768 {
                return Err(AppError::validation(
                    "Max tokens must be between 1 and 32768".to_string(),
                ));
            }
        }

        if let Some(top_p) = config.top_p {
            if !(0.0..=1.0).contains(&top_p) {
                return Err(AppError::validation(
                    "Top-p must be between 0.0 and 1.0".to_string(),
                ));
            }
        }

        Ok(())
    }

    /// Log model initialization progress
    pub fn log_progress(model_id: &str, phase: &str, progress: f32) {
        console::log_1(&format!("Model {}: {} - {:.1}%", model_id, phase, progress * 100.0).into());
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::webllm::ModelCapability;

    #[test]
    fn test_format_model_size() {
        assert_eq!(WebLLMUtils::format_model_size(Some(512)), "512 MB");
        assert_eq!(WebLLMUtils::format_model_size(Some(2048)), "2.0 GB");
        assert_eq!(WebLLMUtils::format_model_size(None), "Unknown");
    }

    #[test]
    fn test_validate_config() {
        let valid_config = ModelConfig {
            model_id: "test-model".to_string(),
            temperature: 0.7,
            max_tokens: Some(1024),
            top_p: Some(0.9),
            frequency_penalty: None,
            presence_penalty: None,
            stop_sequences: Vec::new(),
        };

        assert!(WebLLMUtils::validate_config(&valid_config).is_ok());

        let invalid_config = ModelConfig {
            model_id: "".to_string(),
            temperature: 3.0,    // Invalid
            max_tokens: Some(0), // Invalid
            top_p: Some(1.5),    // Invalid
            frequency_penalty: None,
            presence_penalty: None,
            stop_sequences: Vec::new(),
        };

        assert!(WebLLMUtils::validate_config(&invalid_config).is_err());
    }
}
