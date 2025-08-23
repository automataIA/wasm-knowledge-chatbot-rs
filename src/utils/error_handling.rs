use crate::models::app::AppError;
use web_sys::console;
use wasm_bindgen::prelude::*;

/// Error handling utilities for consistent error management
pub struct ErrorHandler;

impl ErrorHandler {
    /// Log error to console
    pub fn log_error(error: &AppError) {
        let error_msg = format!("Error: {}", error);
        console::error_1(&error_msg.into());
    }

    /// Convert JS error to AppError
    pub fn from_js_error(js_error: JsValue) -> AppError {
        let message = js_error
            .as_string()
            .unwrap_or_else(|| "Unknown JavaScript error".to_string());
        
        AppError::runtime(message)
    }

    /// Create user-friendly error message
    pub fn get_user_message(error: &AppError) -> String {
        match error {
            AppError::ModelNotFound(msg) | AppError::ModelLoadError(msg) | AppError::InferenceError(msg) => {
                format!("AI Model Error: {}", msg)
            },
            AppError::GraphRAGError(msg) | AppError::IndexingError(msg) | AppError::QueryError(msg) => {
                format!("Search Error: {}", msg)
            },
            AppError::StorageError(msg) | AppError::SerializationError(msg) => {
                format!("Storage Error: {}", msg)
            },
            // Ensure tests find the expected substring
            AppError::NetworkError(msg) => format!("Network connection issue: {}", msg),
            AppError::ConnectionTimeout => "Network connection issue: Connection timeout".to_string(),
            AppError::ValidationError(msg) | AppError::InvalidInput(msg) => {
                format!("Input Error: {}", msg)
            },
            AppError::ConfigurationError(msg) => format!("Configuration Error: {}", msg),
            AppError::InternalError(msg) | AppError::NotImplemented(msg) => {
                format!("An unexpected error occurred: {}", msg)
            },
        }
    }

    /// Handle async operation errors
    pub fn handle_async_error<T>(result: Result<T, JsValue>) -> Result<T, AppError> {
        result.map_err(Self::from_js_error)
    }

    /// Create error boundary for component errors
    pub fn create_error_boundary(component_name: &str) -> impl Fn(AppError) -> AppError {
        let name = component_name.to_string();
        move |error: AppError| {
            console::log_1(&format!("Component error in {}: {}", name, error).into());
            Self::log_error(&error);
            error
        }
    }

    /// Retry operation with exponential backoff
    pub async fn retry_with_backoff<F, T, E>(
        mut operation: F,
        max_retries: u32,
        initial_delay_ms: u32,
    ) -> Result<T, E>
    where
        F: FnMut() -> Result<T, E>,
        E: std::fmt::Debug,
    {
        let mut delay = initial_delay_ms;
        
        for attempt in 0..=max_retries {
            match operation() {
                Ok(result) => return Ok(result),
                Err(error) => {
                    if attempt == max_retries {
                        console::error_1(&format!("Operation failed after {} retries: {:?}", max_retries, error).into());
                        return Err(error);
                    }
                    
                    console::warn_1(&format!("Attempt {} failed, retrying in {}ms: {:?}", attempt + 1, delay, error).into());
                    
                    // Wait before retry
                    let promise = js_sys::Promise::new(&mut |resolve, _| {
                        web_sys::window()
                            .unwrap()
                            .set_timeout_with_callback_and_timeout_and_arguments_0(&resolve, delay as i32)
                            .unwrap();
                    });
                    
                    wasm_bindgen_futures::JsFuture::from(promise).await.ok();
                    
                    // Exponential backoff
                    delay *= 2;
                }
            }
        }
        
        unreachable!()
    }

    /// Collect and format multiple errors
    pub fn collect_errors(errors: &[AppError]) -> AppError {
        if errors.is_empty() {
            return AppError::validation("No errors to collect".to_string());
        }

        if errors.len() == 1 {
            return errors[0].clone();
        }

        let messages: Vec<String> = errors.iter().map(|e| e.to_string()).collect();
        let combined_message = format!("Multiple errors occurred:\n{}", messages.join("\n"));
        
        AppError::validation(combined_message)
    }

    /// Validate and handle form submission errors
    pub fn handle_form_errors(validation_results: Vec<Result<(), AppError>>) -> Result<(), AppError> {
        let errors: Vec<AppError> = validation_results
            .into_iter()
            .filter_map(|result| result.err())
            .collect();

        if errors.is_empty() {
            Ok(())
        } else {
            Err(Self::collect_errors(&errors))
        }
    }
}

/// Error recovery strategies
pub struct ErrorRecovery;

impl ErrorRecovery {
    /// Attempt to recover from storage errors
    pub fn recover_storage_error() -> Result<(), AppError> {
        // Try to clear corrupted data
        if let Some(storage) = web_sys::window()
            .and_then(|w| w.local_storage().ok())
            .flatten()
        {
            // Clear potentially corrupted keys
            let keys_to_clear = ["conversations", "settings", "cache"];
            for key in &keys_to_clear {
                if storage.remove_item(key).is_err() {
                    console::warn_1(&format!("Failed to clear storage key: {}", key).into());
                }
            }
            
            console::log_1(&"Storage cleared for recovery".into());
            Ok(())
        } else {
            Err(AppError::storage("Unable to access browser storage".to_string()))
        }
    }

    /// Recover from model loading errors
    pub fn recover_model_error() -> Vec<String> {
        vec![
            "Try refreshing the page".to_string(),
            "Check your internet connection".to_string(),
            "Try a smaller model if available".to_string(),
            "Clear browser cache and try again".to_string(),
        ]
    }

    /// Recover from network errors
    pub fn recover_network_error() -> Vec<String> {
        vec![
            "Check your internet connection".to_string(),
            "Try again in a few moments".to_string(),
            "Refresh the page".to_string(),
        ]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_collect_errors() {
        let errors = vec![
            AppError::validation("Error 1".to_string()),
            AppError::network("Error 2".to_string()),
        ];

        let collected = ErrorHandler::collect_errors(&errors);
        let collected_str = collected.to_string();
        assert!(collected_str.contains("Multiple errors occurred"));
        assert!(collected_str.contains("Error 1"));
        assert!(collected_str.contains("Error 2"));
    }

    #[test]
    fn test_get_user_message() {
        let validation_error = AppError::validation("Field is required".to_string());
        let user_message = ErrorHandler::get_user_message(&validation_error);
        assert!(user_message.contains("Input Error"));

        let network_error = AppError::network("Connection failed".to_string());
        let user_message = ErrorHandler::get_user_message(&network_error);
        assert!(user_message.contains("Network connection issue"));
    }
}
