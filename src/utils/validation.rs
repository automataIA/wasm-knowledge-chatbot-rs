use crate::models::app::AppError;
use regex::Regex;

/// Form validation utilities
pub struct ValidationUtils;

impl ValidationUtils {
    /// Validate email format
    pub fn validate_email(email: &str) -> Result<(), AppError> {
        if email.is_empty() {
            return Err(AppError::validation("Email cannot be empty".to_string()));
        }

        // Simple email regex pattern
        let email_regex = Regex::new(r"^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}$")
            .map_err(|_| AppError::validation("Invalid email regex".to_string()))?;

        if !email_regex.is_match(email) {
            return Err(AppError::validation("Invalid email format".to_string()));
        }

        Ok(())
    }

    /// Validate required string field
    pub fn validate_required_string(value: &str, field_name: &str) -> Result<(), AppError> {
        if value.trim().is_empty() {
            return Err(AppError::validation(format!("{} is required", field_name)));
        }
        Ok(())
    }

    /// Validate string length
    pub fn validate_string_length(
        value: &str,
        field_name: &str,
        min_length: Option<usize>,
        max_length: Option<usize>,
    ) -> Result<(), AppError> {
        let len = value.len();

        if let Some(min) = min_length {
            if len < min {
                return Err(AppError::validation(format!(
                    "{} must be at least {} characters long",
                    field_name, min
                )));
            }
        }

        if let Some(max) = max_length {
            if len > max {
                return Err(AppError::validation(format!(
                    "{} cannot exceed {} characters",
                    field_name, max
                )));
            }
        }

        Ok(())
    }

    /// Validate numeric range
    pub fn validate_numeric_range<T>(
        value: T,
        field_name: &str,
        min: Option<T>,
        max: Option<T>,
    ) -> Result<(), AppError>
    where
        T: PartialOrd + std::fmt::Display + Copy,
    {
        if let Some(min_val) = min {
            if value < min_val {
                return Err(AppError::validation(format!(
                    "{} must be at least {}",
                    field_name, min_val
                )));
            }
        }

        if let Some(max_val) = max {
            if value > max_val {
                return Err(AppError::validation(format!(
                    "{} cannot exceed {}",
                    field_name, max_val
                )));
            }
        }

        Ok(())
    }

    /// Validate URL format
    pub fn validate_url(url: &str) -> Result<(), AppError> {
        if url.is_empty() {
            return Err(AppError::validation("URL cannot be empty".to_string()));
        }

        if !url.starts_with("http://") && !url.starts_with("https://") {
            return Err(AppError::validation(
                "URL must start with http:// or https://".to_string(),
            ));
        }

        Ok(())
    }

    /// Validate file extension
    pub fn validate_file_extension(
        filename: &str,
        allowed_extensions: &[&str],
    ) -> Result<(), AppError> {
        if filename.is_empty() {
            return Err(AppError::validation("Filename cannot be empty".to_string()));
        }

        let extension = filename.split('.').next_back().unwrap_or("").to_lowercase();

        if !allowed_extensions.contains(&extension.as_str()) {
            return Err(AppError::validation(format!(
                "File extension '{}' not allowed. Allowed extensions: {}",
                extension,
                allowed_extensions.join(", ")
            )));
        }

        Ok(())
    }

    /// Validate JSON format
    pub fn validate_json(json_str: &str) -> Result<(), AppError> {
        serde_json::from_str::<serde_json::Value>(json_str)
            .map_err(|e| AppError::validation(format!("Invalid JSON format: {}", e)))?;
        Ok(())
    }

    /// Sanitize HTML input (basic implementation)
    pub fn sanitize_html(input: &str) -> String {
        input
            .replace('<', "&lt;")
            .replace('>', "&gt;")
            .replace('"', "&quot;")
            .replace('\'', "&#x27;")
            .replace('&', "&amp;")
    }

    /// Validate password strength
    pub fn validate_password_strength(password: &str) -> Result<(), AppError> {
        if password.len() < 8 {
            return Err(AppError::validation(
                "Password must be at least 8 characters long".to_string(),
            ));
        }

        let has_uppercase = password.chars().any(|c| c.is_uppercase());
        let has_lowercase = password.chars().any(|c| c.is_lowercase());
        let has_digit = password.chars().any(|c| c.is_numeric());
        let has_special = password
            .chars()
            .any(|c| "!@#$%^&*()_+-=[]{}|;:,.<>?".contains(c));

        if !has_uppercase {
            return Err(AppError::validation(
                "Password must contain at least one uppercase letter".to_string(),
            ));
        }

        if !has_lowercase {
            return Err(AppError::validation(
                "Password must contain at least one lowercase letter".to_string(),
            ));
        }

        if !has_digit {
            return Err(AppError::validation(
                "Password must contain at least one digit".to_string(),
            ));
        }

        if !has_special {
            return Err(AppError::validation(
                "Password must contain at least one special character".to_string(),
            ));
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_email() {
        assert!(ValidationUtils::validate_email("test@example.com").is_ok());
        assert!(ValidationUtils::validate_email("invalid-email").is_err());
        assert!(ValidationUtils::validate_email("").is_err());
    }

    #[test]
    fn test_validate_required_string() {
        assert!(ValidationUtils::validate_required_string("valid", "field").is_ok());
        assert!(ValidationUtils::validate_required_string("", "field").is_err());
        assert!(ValidationUtils::validate_required_string("   ", "field").is_err());
    }

    #[test]
    fn test_validate_string_length() {
        assert!(
            ValidationUtils::validate_string_length("hello", "field", Some(3), Some(10)).is_ok()
        );
        assert!(ValidationUtils::validate_string_length("hi", "field", Some(3), Some(10)).is_err());
        assert!(ValidationUtils::validate_string_length(
            "very long string",
            "field",
            Some(3),
            Some(10)
        )
        .is_err());
    }

    #[test]
    fn test_validate_file_extension() {
        assert!(ValidationUtils::validate_file_extension("document.pdf", &["pdf", "doc"]).is_ok());
        assert!(ValidationUtils::validate_file_extension("image.jpg", &["pdf", "doc"]).is_err());
    }

    #[test]
    fn test_sanitize_html() {
        let input = "<script>alert('xss')</script>";
        let sanitized = ValidationUtils::sanitize_html(input);
        assert!(!sanitized.contains('<'));
        assert!(!sanitized.contains('>'));
    }
}
