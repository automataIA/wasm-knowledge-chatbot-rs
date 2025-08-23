use leptos::prelude::*;
use serde::{Deserialize, Serialize};
use crate::models::app::{AppConfig, AppError};

/// Simplified global application state
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AppState {
    pub config: AppConfig,
    pub is_loading: bool,
    pub error: Option<AppError>,
    pub theme: String,
    pub sidebar_open: bool,
    pub mobile_view: bool,
}

impl Default for AppStateContext {
    fn default() -> Self {
        Self::new()
    }
}

impl Default for AppState {
    fn default() -> Self {
        Self {
            config: AppConfig::default(),
            is_loading: false,
            error: None,
            theme: "light".to_string(),
            sidebar_open: true,
            mobile_view: false,
        }
    }
}

/// App state context for global state management
#[derive(Clone)]
pub struct AppStateContext {
    pub state: RwSignal<AppState>,
}

impl AppStateContext {
    pub fn new() -> Self {
        Self {
            state: RwSignal::new(AppState::default()),
        }
    }

    // Loading state methods
    pub fn is_loading(&self) -> bool {
        self.state.get().is_loading
    }

    pub fn set_loading(&self, loading: bool) {
        self.state.update(|s| s.is_loading = loading);
    }

    // Error state methods
    pub fn get_error(&self) -> Option<AppError> {
        self.state.get().error
    }

    pub fn set_error(&self, error: Option<AppError>) {
        self.state.update(|s| s.error = error);
    }

    pub fn clear_error(&self) {
        self.state.update(|s| s.error = None);
    }

    // Theme methods
    pub fn get_theme(&self) -> String {
        self.state.get().theme
    }

    pub fn set_theme(&self, theme: String) {
        self.state.update(|s| s.theme = theme);
    }

    pub fn toggle_theme(&self) {
        self.state.update(|s| {
            s.theme = if s.theme == "light" {
                "dark".to_string()
            } else {
                "light".to_string()
            };
        });
    }

    // Sidebar methods
    pub fn is_sidebar_open(&self) -> bool {
        self.state.get().sidebar_open
    }

    pub fn set_sidebar_open(&self, open: bool) {
        self.state.update(|s| s.sidebar_open = open);
    }

    pub fn toggle_sidebar(&self) {
        self.state.update(|s| s.sidebar_open = !s.sidebar_open);
    }

    // Mobile view methods
    pub fn is_mobile_view(&self) -> bool {
        self.state.get().mobile_view
    }

    pub fn set_mobile_view(&self, mobile: bool) {
        self.state.update(|s| s.mobile_view = mobile);
    }
}

/// Provider component for app state
#[component]
pub fn AppStateProvider(children: Children) -> impl IntoView {
    let app_state = AppStateContext::new();
    provide_context(app_state);
    children()
}

/// Hook to use app state context
pub fn use_app_state() -> AppStateContext {
    expect_context::<AppStateContext>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_app_state_creation() {
        let ctx = AppStateContext::new();
        assert!(!ctx.is_loading());
        assert!(ctx.get_error().is_none());
        assert_eq!(ctx.get_theme(), "light");
        assert!(ctx.is_sidebar_open());
        assert!(!ctx.is_mobile_view());
    }

    #[test]
    fn test_app_state_methods() {
        let ctx = AppStateContext::new();
        
        // Test loading
        ctx.set_loading(true);
        assert!(ctx.is_loading());
        
        // Test theme toggle
        ctx.toggle_theme();
        assert_eq!(ctx.get_theme(), "dark");
        
        // Test sidebar toggle
        ctx.toggle_sidebar();
        assert!(!ctx.is_sidebar_open());
    }
}
