use crate::models::app::AppError;

/// Text and data formatting utilities
pub struct FormatUtils;

impl FormatUtils {
    /// Format timestamp for display
    pub fn format_timestamp(timestamp: f64) -> String {
        let date = js_sys::Date::new(&wasm_bindgen::JsValue::from(timestamp));
        date.to_locale_string("en-US", &js_sys::Object::new())
            .as_string()
            .unwrap_or_else(|| "Invalid date".to_string())
    }

    /// Format timestamp as relative time (e.g., "2 hours ago")
    pub fn format_relative_time(timestamp: f64) -> String {
        let now = js_sys::Date::now();
        let diff_ms = now - timestamp;
        let diff_seconds = diff_ms / 1000.0;
        let diff_minutes = diff_seconds / 60.0;
        let diff_hours = diff_minutes / 60.0;
        let diff_days = diff_hours / 24.0;

        if diff_seconds < 60.0 {
            "Just now".to_string()
        } else if diff_minutes < 60.0 {
            format!(
                "{:.0} minute{} ago",
                diff_minutes,
                if diff_minutes as i32 == 1 { "" } else { "s" }
            )
        } else if diff_hours < 24.0 {
            format!(
                "{:.0} hour{} ago",
                diff_hours,
                if diff_hours as i32 == 1 { "" } else { "s" }
            )
        } else if diff_days < 30.0 {
            format!(
                "{:.0} day{} ago",
                diff_days,
                if diff_days as i32 == 1 { "" } else { "s" }
            )
        } else {
            Self::format_timestamp(timestamp)
        }
    }

    /// Format file size in human-readable format
    pub fn format_file_size(bytes: u64) -> String {
        const UNITS: &[&str] = &["B", "KB", "MB", "GB", "TB"];
        const THRESHOLD: f64 = 1024.0;

        if bytes == 0 {
            return "0 B".to_string();
        }

        let bytes_f = bytes as f64;
        let unit_index = (bytes_f.log10() / THRESHOLD.log10()).floor() as usize;
        let unit_index = unit_index.min(UNITS.len() - 1);

        let size = bytes_f / THRESHOLD.powi(unit_index as i32);

        if size >= 100.0 {
            format!("{:.0} {}", size, UNITS[unit_index])
        } else if size >= 10.0 {
            format!("{:.1} {}", size, UNITS[unit_index])
        } else {
            format!("{:.2} {}", size, UNITS[unit_index])
        }
    }

    /// Format duration in human-readable format
    pub fn format_duration(seconds: f64) -> String {
        if seconds < 60.0 {
            format!("{:.1}s", seconds)
        } else if seconds < 3600.0 {
            let minutes = seconds / 60.0;
            format!("{:.1}m", minutes)
        } else if seconds < 86400.0 {
            let hours = seconds / 3600.0;
            format!("{:.1}h", hours)
        } else {
            let days = seconds / 86400.0;
            format!("{:.1}d", days)
        }
    }

    /// Format percentage with appropriate precision
    pub fn format_percentage(value: f32) -> String {
        if value >= 99.95 {
            "100%".to_string()
        } else if value >= 10.0 {
            format!("{:.0}%", value)
        } else if value >= 1.0 {
            format!("{:.1}%", value)
        } else {
            format!("{:.2}%", value)
        }
    }

    /// Format number with thousands separators
    pub fn format_number(number: i64) -> String {
        let mut result = String::new();
        let number_str = number.abs().to_string();
        let chars: Vec<char> = number_str.chars().collect();

        for (i, &ch) in chars.iter().enumerate() {
            if i > 0 && (chars.len() - i).is_multiple_of(3) {
                result.push(',');
            }
            result.push(ch);
        }

        if number < 0 {
            format!("-{}", result)
        } else {
            result
        }
    }

    /// Truncate text with ellipsis
    pub fn truncate_text(text: &str, max_length: usize) -> String {
        if text.len() <= max_length {
            text.to_string()
        } else if max_length <= 3 {
            "...".to_string()
        } else {
            format!("{}...", &text[..max_length - 3])
        }
    }

    /// Convert text to title case
    pub fn to_title_case(text: &str) -> String {
        text.split_whitespace()
            .map(|word| {
                let mut chars = word.chars();
                match chars.next() {
                    None => String::new(),
                    Some(first) => {
                        first.to_uppercase().collect::<String>() + &chars.as_str().to_lowercase()
                    }
                }
            })
            .collect::<Vec<_>>()
            .join(" ")
    }

    /// Convert camelCase or PascalCase to human-readable format
    pub fn camel_to_human(text: &str) -> String {
        let mut result = String::new();
        let chars = text.chars();

        for ch in chars {
            if ch.is_uppercase() && !result.is_empty() {
                result.push(' ');
            }
            result.push(ch);
        }

        Self::to_title_case(&result)
    }

    /// Format JSON with proper indentation
    pub fn format_json(json_str: &str) -> Result<String, AppError> {
        let value: serde_json::Value = serde_json::from_str(json_str)
            .map_err(|e| AppError::validation(format!("Invalid JSON: {}", e)))?;

        serde_json::to_string_pretty(&value)
            .map_err(|e| AppError::runtime(format!("JSON formatting failed: {}", e)))
    }

    /// Extract initials from a name
    pub fn extract_initials(name: &str) -> String {
        name.split_whitespace()
            .filter_map(|word| word.chars().next())
            .map(|ch| ch.to_uppercase().to_string())
            .collect::<Vec<_>>()
            .join("")
            .chars()
            .take(3)
            .collect()
    }

    /// Generate a safe filename from text
    pub fn to_safe_filename(text: &str) -> String {
        text.chars()
            .map(|ch| {
                if ch.is_alphanumeric() || ch == '-' || ch == '_' {
                    ch
                } else if ch.is_whitespace() {
                    '_'
                } else {
                    '-'
                }
            })
            .collect::<String>()
            .trim_matches(|c| c == '-' || c == '_')
            .to_lowercase()
    }

    /// Format code with syntax highlighting classes
    pub fn format_code_block(code: &str, language: Option<&str>) -> String {
        let lang_class = language
            .map(|l| format!(" language-{}", l))
            .unwrap_or_default();

        format!(
            "<pre class=\"code-block\"><code class=\"{}\">{}</code></pre>",
            lang_class.trim(),
            html_escape::encode_text(code)
        )
    }

    /// Format markdown-like text to HTML
    pub fn format_markdown_basic(text: &str) -> String {
        let mut result = html_escape::encode_text(text).to_string();

        // Bold text
        result = regex::Regex::new(r"\*\*(.*?)\*\*")
            .unwrap()
            .replace_all(&result, "<strong>$1</strong>")
            .to_string();

        // Italic text
        result = regex::Regex::new(r"\*(.*?)\*")
            .unwrap()
            .replace_all(&result, "<em>$1</em>")
            .to_string();

        // Code spans
        result = regex::Regex::new(r"`(.*?)`")
            .unwrap()
            .replace_all(&result, "<code>$1</code>")
            .to_string();

        // Line breaks
        result = result.replace('\n', "<br>");

        result
    }

    /// Generate color from string (for avatars, etc.)
    pub fn string_to_color(text: &str) -> String {
        let mut hash = 0u32;
        for ch in text.chars() {
            hash = hash.wrapping_mul(31).wrapping_add(ch as u32);
        }

        let hue = hash % 360;
        format!("hsl({}, 70%, 50%)", hue)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_file_size() {
        assert_eq!(FormatUtils::format_file_size(0), "0 B");
        assert_eq!(FormatUtils::format_file_size(512), "512 B");
        assert_eq!(FormatUtils::format_file_size(1024), "1.00 KB");
        assert_eq!(FormatUtils::format_file_size(1536), "1.50 KB");
        assert_eq!(FormatUtils::format_file_size(1048576), "1.00 MB");
    }

    #[test]
    fn test_format_percentage() {
        assert_eq!(FormatUtils::format_percentage(0.5), "0.50%");
        assert_eq!(FormatUtils::format_percentage(5.5), "5.5%");
        assert_eq!(FormatUtils::format_percentage(55.5), "56%");
        assert_eq!(FormatUtils::format_percentage(99.99), "100%");
    }

    #[test]
    fn test_format_number() {
        assert_eq!(FormatUtils::format_number(1000), "1,000");
        assert_eq!(FormatUtils::format_number(1234567), "1,234,567");
        assert_eq!(FormatUtils::format_number(-1000), "-1,000");
    }

    #[test]
    fn test_truncate_text() {
        assert_eq!(FormatUtils::truncate_text("Hello World", 20), "Hello World");
        assert_eq!(FormatUtils::truncate_text("Hello World", 8), "Hello...");
        assert_eq!(FormatUtils::truncate_text("Hi", 2), "Hi");
    }

    #[test]
    fn test_to_title_case() {
        assert_eq!(FormatUtils::to_title_case("hello world"), "Hello World");
        assert_eq!(FormatUtils::to_title_case("HELLO WORLD"), "Hello World");
    }

    #[test]
    fn test_camel_to_human() {
        assert_eq!(FormatUtils::camel_to_human("firstName"), "First Name");
        assert_eq!(
            FormatUtils::camel_to_human("XMLHttpRequest"),
            "X M L Http Request"
        );
    }

    #[test]
    fn test_extract_initials() {
        assert_eq!(FormatUtils::extract_initials("John Doe"), "JD");
        assert_eq!(FormatUtils::extract_initials("Mary Jane Watson"), "MJW");
        assert_eq!(FormatUtils::extract_initials("Single"), "S");
    }

    #[test]
    fn test_to_safe_filename() {
        assert_eq!(FormatUtils::to_safe_filename("Hello World!"), "hello_world");
        assert_eq!(
            FormatUtils::to_safe_filename("File@Name#123"),
            "file-name-123"
        );
        assert_eq!(FormatUtils::to_safe_filename("  spaced  "), "spaced");
    }
}
