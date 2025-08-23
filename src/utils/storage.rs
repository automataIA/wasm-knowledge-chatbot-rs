use crate::models::app::AppError;
use serde::{Serialize, Deserialize};
use web_sys::{Storage, window};

/// Browser storage utilities for data persistence
pub struct StorageUtils;

impl StorageUtils {
    /// Get localStorage instance
    fn get_local_storage() -> Result<Storage, AppError> {
        window()
            .ok_or_else(|| AppError::storage("Window not available".to_string()))?
            .local_storage()
            .map_err(|_| AppError::storage("LocalStorage not available".to_string()))?
            .ok_or_else(|| AppError::storage("LocalStorage not supported".to_string()))
    }

    /// Get sessionStorage instance
    fn get_session_storage() -> Result<Storage, AppError> {
        window()
            .ok_or_else(|| AppError::storage("Window not available".to_string()))?
            .session_storage()
            .map_err(|_| AppError::storage("SessionStorage not available".to_string()))?
            .ok_or_else(|| AppError::storage("SessionStorage not supported".to_string()))
    }

    /// Store data in localStorage
    pub fn store_local<T: Serialize>(key: &str, data: &T) -> Result<(), AppError> {
        let storage = Self::get_local_storage()?;
        let serialized = serde_json::to_string(data)
            .map_err(|e| AppError::storage(format!("Serialization failed: {}", e)))?;
        
        storage
            .set_item(key, &serialized)
            .map_err(|_| AppError::storage(format!("Failed to store data for key: {}", key)))
    }

    /// Retrieve data from localStorage
    pub fn retrieve_local<T: for<'de> Deserialize<'de>>(key: &str) -> Result<Option<T>, AppError> {
        let storage = Self::get_local_storage()?;
        
        match storage.get_item(key) {
            Ok(Some(data)) => {
                let deserialized = serde_json::from_str(&data)
                    .map_err(|e| AppError::storage(format!("Deserialization failed: {}", e)))?;
                Ok(Some(deserialized))
            }
            Ok(None) => Ok(None),
            Err(_) => Err(AppError::storage(format!("Failed to retrieve data for key: {}", key))),
        }
    }

    /// Store data in sessionStorage
    pub fn store_session<T: Serialize>(key: &str, data: &T) -> Result<(), AppError> {
        let storage = Self::get_session_storage()?;
        let serialized = serde_json::to_string(data)
            .map_err(|e| AppError::storage(format!("Serialization failed: {}", e)))?;
        
        storage
            .set_item(key, &serialized)
            .map_err(|_| AppError::storage(format!("Failed to store session data for key: {}", key)))
    }

    /// Retrieve data from sessionStorage
    pub fn retrieve_session<T: for<'de> Deserialize<'de>>(key: &str) -> Result<Option<T>, AppError> {
        let storage = Self::get_session_storage()?;
        
        match storage.get_item(key) {
            Ok(Some(data)) => {
                let deserialized = serde_json::from_str(&data)
                    .map_err(|e| AppError::storage(format!("Deserialization failed: {}", e)))?;
                Ok(Some(deserialized))
            }
            Ok(None) => Ok(None),
            Err(_) => Err(AppError::storage(format!("Failed to retrieve session data for key: {}", key))),
        }
    }

    /// Remove item from localStorage
    pub fn remove_local(key: &str) -> Result<(), AppError> {
        let storage = Self::get_local_storage()?;
        storage
            .remove_item(key)
            .map_err(|_| AppError::storage(format!("Failed to remove data for key: {}", key)))
    }

    /// Remove item from sessionStorage
    pub fn remove_session(key: &str) -> Result<(), AppError> {
        let storage = Self::get_session_storage()?;
        storage
            .remove_item(key)
            .map_err(|_| AppError::storage(format!("Failed to remove session data for key: {}", key)))
    }

    /// Clear all localStorage data
    pub fn clear_local() -> Result<(), AppError> {
        let storage = Self::get_local_storage()?;
        storage
            .clear()
            .map_err(|_| AppError::storage("Failed to clear localStorage".to_string()))
    }

    /// Clear all sessionStorage data
    pub fn clear_session() -> Result<(), AppError> {
        let storage = Self::get_session_storage()?;
        storage
            .clear()
            .map_err(|_| AppError::storage("Failed to clear sessionStorage".to_string()))
    }

    /// Get storage usage information
    pub fn get_storage_info() -> Result<StorageInfo, AppError> {
        let local_storage = Self::get_local_storage()?;
        let session_storage = Self::get_session_storage()?;

        let local_length = local_storage.length()
            .map_err(|_| AppError::storage("Failed to get localStorage length".to_string()))?;
        
        let session_length = session_storage.length()
            .map_err(|_| AppError::storage("Failed to get sessionStorage length".to_string()))?;

        // Estimate storage size
        let mut local_size = 0;
        let mut session_size = 0;

        for i in 0..local_length {
            if let Ok(Some(key)) = local_storage.key(i) {
                if let Ok(Some(value)) = local_storage.get_item(&key) {
                    local_size += key.len() + value.len();
                }
            }
        }

        for i in 0..session_length {
            if let Ok(Some(key)) = session_storage.key(i) {
                if let Ok(Some(value)) = session_storage.get_item(&key) {
                    session_size += key.len() + value.len();
                }
            }
        }

        Ok(StorageInfo {
            local_items: local_length,
            session_items: session_length,
            local_size_bytes: local_size,
            session_size_bytes: session_size,
        })
    }

    /// Check if storage quota is exceeded
    pub fn check_storage_quota() -> bool {
        // Try to store a test value to check quota
        if let Ok(storage) = Self::get_local_storage() {
            let test_key = "__quota_test__";
            let test_value = "test";
            
            match storage.set_item(test_key, test_value) {
                Ok(_) => {
                    let _ = storage.remove_item(test_key);
                    true
                }
                Err(_) => false,
            }
        } else {
            false
        }
    }

    /// Get all keys from localStorage
    pub fn get_local_keys() -> Result<Vec<String>, AppError> {
        let storage = Self::get_local_storage()?;
        let length = storage.length()
            .map_err(|_| AppError::storage("Failed to get localStorage length".to_string()))?;

        let mut keys = Vec::new();
        for i in 0..length {
            if let Ok(Some(key)) = storage.key(i) {
                keys.push(key);
            }
        }

        Ok(keys)
    }

    /// Get all keys from sessionStorage
    pub fn get_session_keys() -> Result<Vec<String>, AppError> {
        let storage = Self::get_session_storage()?;
        let length = storage.length()
            .map_err(|_| AppError::storage("Failed to get sessionStorage length".to_string()))?;

        let mut keys = Vec::new();
        for i in 0..length {
            if let Ok(Some(key)) = storage.key(i) {
                keys.push(key);
            }
        }

        Ok(keys)
    }

    /// Backup storage data to JSON
    pub fn backup_storage() -> Result<String, AppError> {
        let local_keys = Self::get_local_keys()?;
        let storage = Self::get_local_storage()?;

        let mut backup_data = std::collections::HashMap::new();
        
        for key in local_keys {
            if let Ok(Some(value)) = storage.get_item(&key) {
                backup_data.insert(key, value);
            }
        }

        serde_json::to_string_pretty(&backup_data)
            .map_err(|e| AppError::storage(format!("Failed to serialize backup: {}", e)))
    }

    /// Restore storage data from JSON backup
    pub fn restore_storage(backup_json: &str) -> Result<(), AppError> {
        let backup_data: std::collections::HashMap<String, String> = serde_json::from_str(backup_json)
            .map_err(|e| AppError::storage(format!("Failed to parse backup: {}", e)))?;

        let storage = Self::get_local_storage()?;

        for (key, value) in backup_data {
            storage.set_item(&key, &value)
                .map_err(|_| AppError::storage(format!("Failed to restore key: {}", key)))?;
        }

        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StorageInfo {
    pub local_items: u32,
    pub session_items: u32,
    pub local_size_bytes: usize,
    pub session_size_bytes: usize,
}

impl StorageInfo {
    pub fn total_items(&self) -> u32 {
        self.local_items + self.session_items
    }

    pub fn total_size_bytes(&self) -> usize {
        self.local_size_bytes + self.session_size_bytes
    }

    pub fn format_size(bytes: usize) -> String {
        if bytes < 1024 {
            format!("{} B", bytes)
        } else if bytes < 1024 * 1024 {
            format!("{:.1} KB", bytes as f64 / 1024.0)
        } else {
            format!("{:.1} MB", bytes as f64 / (1024.0 * 1024.0))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde::{Deserialize, Serialize};

    #[derive(Serialize, Deserialize, PartialEq, Debug)]
    struct TestData {
        name: String,
        value: i32,
    }

    // Note: These tests would need to run in a browser environment
    // For now, they serve as documentation of the expected behavior

    #[test]
    fn test_storage_info_formatting() {
        let info = StorageInfo {
            local_items: 5,
            session_items: 3,
            local_size_bytes: 1536,
            session_size_bytes: 512,
        };

        assert_eq!(info.total_items(), 8);
        assert_eq!(info.total_size_bytes(), 2048);
        assert_eq!(StorageInfo::format_size(1024), "1.0 KB");
        assert_eq!(StorageInfo::format_size(1048576), "1.0 MB");
    }
}
