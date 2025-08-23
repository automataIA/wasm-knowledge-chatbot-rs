// UI system module - Design tokens and theme management

pub mod theme;

// Re-export theme utilities for easy access
pub use theme::{
    Theme, LIGHT_THEME, DARK_THEME,
    ColorTokens, SpacingTokens, TypographyTokens, 
    BorderTokens, ShadowTokens, AnimationTokens,
};
