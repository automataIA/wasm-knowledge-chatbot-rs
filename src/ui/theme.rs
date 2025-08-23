// Design tokens and theme system for the WebLLM Knowledge Chatbot
// Aligned with UI_DESIGN_GUIDELINES.md and DaisyUI integration

use serde::{Deserialize, Serialize};

/// Core color palette following the design guidelines
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ColorTokens {
    // Primary colors
    pub primary: &'static str,
    pub primary_focus: &'static str,
    pub primary_content: &'static str,
    
    // Secondary colors
    pub secondary: &'static str,
    pub secondary_focus: &'static str,
    pub secondary_content: &'static str,
    
    // Accent colors
    pub accent: &'static str,
    pub accent_focus: &'static str,
    pub accent_content: &'static str,
    
    // Neutral colors
    pub neutral: &'static str,
    pub neutral_focus: &'static str,
    pub neutral_content: &'static str,
    
    // Base colors
    pub base_100: &'static str,
    pub base_200: &'static str,
    pub base_300: &'static str,
    pub base_content: &'static str,
    
    // Semantic colors
    pub info: &'static str,
    pub success: &'static str,
    pub warning: &'static str,
    pub error: &'static str,
}

/// Spacing tokens based on 4px base unit
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpacingTokens {
    pub xs: &'static str,    // 4px
    pub sm: &'static str,    // 8px
    pub md: &'static str,    // 16px
    pub lg: &'static str,    // 24px
    pub xl: &'static str,    // 32px
    pub xxl: &'static str,   // 48px
    pub xxxl: &'static str,  // 64px
}

/// Typography tokens
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TypographyTokens {
    // Font families
    pub font_sans: &'static str,
    pub font_mono: &'static str,
    
    // Font sizes
    pub text_xs: &'static str,   // 12px
    pub text_sm: &'static str,   // 14px
    pub text_base: &'static str, // 16px
    pub text_lg: &'static str,   // 18px
    pub text_xl: &'static str,   // 20px
    pub text_2xl: &'static str,  // 24px
    pub text_3xl: &'static str,  // 30px
    pub text_4xl: &'static str,  // 36px
    
    // Line heights
    pub leading_tight: &'static str,  // 1.25
    pub leading_normal: &'static str, // 1.5
    pub leading_relaxed: &'static str, // 1.75
    
    // Font weights
    pub font_normal: &'static str,    // 400
    pub font_medium: &'static str,    // 500
    pub font_semibold: &'static str,  // 600
    pub font_bold: &'static str,      // 700
}

/// Border and radius tokens
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BorderTokens {
    pub radius_sm: &'static str,   // 4px
    pub radius_md: &'static str,   // 8px
    pub radius_lg: &'static str,   // 12px
    pub radius_xl: &'static str,   // 16px
    pub radius_full: &'static str, // 9999px
    
    pub border_width: &'static str,      // 1px
    pub border_width_2: &'static str,    // 2px
    pub border_width_4: &'static str,    // 4px
    
    pub focus_ring_width: &'static str,  // 2px (accessibility requirement)
}

/// Shadow tokens for depth
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShadowTokens {
    pub shadow_sm: &'static str,
    pub shadow_md: &'static str,
    pub shadow_lg: &'static str,
    pub shadow_xl: &'static str,
}

/// Animation and transition tokens
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnimationTokens {
    pub duration_fast: &'static str,     // 150ms
    pub duration_normal: &'static str,   // 300ms
    pub duration_slow: &'static str,     // 500ms
    
    pub ease_in: &'static str,
    pub ease_out: &'static str,
    pub ease_in_out: &'static str,
}

/// Complete theme definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Theme {
    pub name: &'static str,
    pub colors: ColorTokens,
    pub spacing: SpacingTokens,
    pub typography: TypographyTokens,
    pub borders: BorderTokens,
    pub shadows: ShadowTokens,
    pub animations: AnimationTokens,
}

/// Light theme implementation
pub const LIGHT_THEME: Theme = Theme {
    name: "light",
    colors: ColorTokens {
        primary: "#3b82f6",
        primary_focus: "#2563eb",
        primary_content: "#ffffff",
        
        secondary: "#64748b",
        secondary_focus: "#475569",
        secondary_content: "#ffffff",
        
        accent: "#8b5cf6",
        accent_focus: "#7c3aed",
        accent_content: "#ffffff",
        
        neutral: "#374151",
        neutral_focus: "#1f2937",
        neutral_content: "#ffffff",
        
        base_100: "#ffffff",
        base_200: "#f8fafc",
        base_300: "#e2e8f0",
        base_content: "#1e293b",
        
        info: "#0ea5e9",
        success: "#22c55e",
        warning: "#f59e0b",
        error: "#ef4444",
    },
    spacing: SPACING_TOKENS,
    typography: TYPOGRAPHY_TOKENS,
    borders: BORDER_TOKENS,
    shadows: SHADOW_TOKENS,
    animations: ANIMATION_TOKENS,
};

/// Dark theme implementation
pub const DARK_THEME: Theme = Theme {
    name: "dark",
    colors: ColorTokens {
        primary: "#60a5fa",
        primary_focus: "#3b82f6",
        primary_content: "#1e293b",
        
        secondary: "#94a3b8",
        secondary_focus: "#64748b",
        secondary_content: "#1e293b",
        
        accent: "#a78bfa",
        accent_focus: "#8b5cf6",
        accent_content: "#1e293b",
        
        neutral: "#64748b",
        neutral_focus: "#94a3b8",
        neutral_content: "#f1f5f9",
        
        base_100: "#1e293b",
        base_200: "#334155",
        base_300: "#475569",
        base_content: "#f1f5f9",
        
        info: "#38bdf8",
        success: "#4ade80",
        warning: "#fbbf24",
        error: "#f87171",
    },
    spacing: SPACING_TOKENS,
    typography: TYPOGRAPHY_TOKENS,
    borders: BORDER_TOKENS,
    shadows: SHADOW_TOKENS,
    animations: ANIMATION_TOKENS,
};

/// Shared spacing tokens
const SPACING_TOKENS: SpacingTokens = SpacingTokens {
    xs: "4px",
    sm: "8px",
    md: "16px",
    lg: "24px",
    xl: "32px",
    xxl: "48px",
    xxxl: "64px",
};

/// Shared typography tokens
const TYPOGRAPHY_TOKENS: TypographyTokens = TypographyTokens {
    font_sans: "ui-sans-serif, system-ui, -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, 'Helvetica Neue', Arial, 'Noto Sans', sans-serif",
    font_mono: "ui-monospace, SFMono-Regular, 'SF Mono', Consolas, 'Liberation Mono', Menlo, monospace",
    
    text_xs: "12px",
    text_sm: "14px",
    text_base: "16px",
    text_lg: "18px",
    text_xl: "20px",
    text_2xl: "24px",
    text_3xl: "30px",
    text_4xl: "36px",
    
    leading_tight: "1.25",
    leading_normal: "1.5",
    leading_relaxed: "1.75",
    
    font_normal: "400",
    font_medium: "500",
    font_semibold: "600",
    font_bold: "700",
};

/// Shared border tokens
const BORDER_TOKENS: BorderTokens = BorderTokens {
    radius_sm: "4px",
    radius_md: "8px",
    radius_lg: "12px",
    radius_xl: "16px",
    radius_full: "9999px",
    
    border_width: "1px",
    border_width_2: "2px",
    border_width_4: "4px",
    
    focus_ring_width: "2px", // WCAG 2.1 AA requirement
};

/// Shared shadow tokens
const SHADOW_TOKENS: ShadowTokens = ShadowTokens {
    shadow_sm: "0 1px 2px 0 rgb(0 0 0 / 0.05)",
    shadow_md: "0 4px 6px -1px rgb(0 0 0 / 0.1), 0 2px 4px -2px rgb(0 0 0 / 0.1)",
    shadow_lg: "0 10px 15px -3px rgb(0 0 0 / 0.1), 0 4px 6px -4px rgb(0 0 0 / 0.1)",
    shadow_xl: "0 20px 25px -5px rgb(0 0 0 / 0.1), 0 8px 10px -6px rgb(0 0 0 / 0.1)",
};

/// Shared animation tokens
const ANIMATION_TOKENS: AnimationTokens = AnimationTokens {
    duration_fast: "150ms",
    duration_normal: "300ms",
    duration_slow: "500ms",
    
    ease_in: "cubic-bezier(0.4, 0, 1, 1)",
    ease_out: "cubic-bezier(0, 0, 0.2, 1)",
    ease_in_out: "cubic-bezier(0.4, 0, 0.2, 1)",
};

/// Theme utilities
impl Theme {
    /// Get the current theme based on system preference or user selection
    pub fn current() -> &'static Theme {
        // For now, return light theme. This can be enhanced with:
        // - System preference detection
        // - User preference from local storage
        // - Dynamic theme switching
        &LIGHT_THEME
    }
    
    /// Get theme by name
    pub fn by_name(name: &str) -> Option<&'static Theme> {
        match name {
            "light" => Some(&LIGHT_THEME),
            "dark" => Some(&DARK_THEME),
            _ => None,
        }
    }
    
    /// Generate CSS custom properties for the theme
    pub fn to_css_vars(&self) -> String {
        format!(
            r#":root {{
  /* Colors */
  --color-primary: {};
  --color-primary-focus: {};
  --color-primary-content: {};
  --color-secondary: {};
  --color-secondary-focus: {};
  --color-secondary-content: {};
  --color-accent: {};
  --color-accent-focus: {};
  --color-accent-content: {};
  --color-neutral: {};
  --color-neutral-focus: {};
  --color-neutral-content: {};
  --color-base-100: {};
  --color-base-200: {};
  --color-base-300: {};
  --color-base-content: {};
  --color-info: {};
  --color-success: {};
  --color-warning: {};
  --color-error: {};
  
  /* Spacing */
  --spacing-xs: {};
  --spacing-sm: {};
  --spacing-md: {};
  --spacing-lg: {};
  --spacing-xl: {};
  --spacing-xxl: {};
  --spacing-xxxl: {};
  
  /* Typography */
  --font-sans: {};
  --font-mono: {};
  --text-xs: {};
  --text-sm: {};
  --text-base: {};
  --text-lg: {};
  --text-xl: {};
  --text-2xl: {};
  --text-3xl: {};
  --text-4xl: {};
  
  /* Borders */
  --radius-sm: {};
  --radius-md: {};
  --radius-lg: {};
  --radius-xl: {};
  --radius-full: {};
  --border-width: {};
  --border-width-2: {};
  --border-width-4: {};
  --focus-ring-width: {};
  
  /* Shadows */
  --shadow-sm: {};
  --shadow-md: {};
  --shadow-lg: {};
  --shadow-xl: {};
  
  /* Animations */
  --duration-fast: {};
  --duration-normal: {};
  --duration-slow: {};
  --ease-in: {};
  --ease-out: {};
  --ease-in-out: {};
}}"#,
            // Colors
            self.colors.primary, self.colors.primary_focus, self.colors.primary_content,
            self.colors.secondary, self.colors.secondary_focus, self.colors.secondary_content,
            self.colors.accent, self.colors.accent_focus, self.colors.accent_content,
            self.colors.neutral, self.colors.neutral_focus, self.colors.neutral_content,
            self.colors.base_100, self.colors.base_200, self.colors.base_300, self.colors.base_content,
            self.colors.info, self.colors.success, self.colors.warning, self.colors.error,
            
            // Spacing
            self.spacing.xs, self.spacing.sm, self.spacing.md, self.spacing.lg,
            self.spacing.xl, self.spacing.xxl, self.spacing.xxxl,
            
            // Typography
            self.typography.font_sans, self.typography.font_mono,
            self.typography.text_xs, self.typography.text_sm, self.typography.text_base,
            self.typography.text_lg, self.typography.text_xl, self.typography.text_2xl,
            self.typography.text_3xl, self.typography.text_4xl,
            
            // Borders
            self.borders.radius_sm, self.borders.radius_md, self.borders.radius_lg,
            self.borders.radius_xl, self.borders.radius_full,
            self.borders.border_width, self.borders.border_width_2, self.borders.border_width_4,
            self.borders.focus_ring_width,
            
            // Shadows
            self.shadows.shadow_sm, self.shadows.shadow_md, self.shadows.shadow_lg, self.shadows.shadow_xl,
            
            // Animations
            self.animations.duration_fast, self.animations.duration_normal, self.animations.duration_slow,
            self.animations.ease_in, self.animations.ease_out, self.animations.ease_in_out,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_theme_creation() {
        let theme = Theme::current();
        assert_eq!(theme.name, "light");
        assert_eq!(theme.colors.primary, "#3b82f6");
        assert_eq!(theme.spacing.md, "16px");
    }

    #[test]
    fn test_theme_by_name() {
        assert!(Theme::by_name("light").is_some());
        assert!(Theme::by_name("dark").is_some());
        assert!(Theme::by_name("nonexistent").is_none());
    }

    #[test]
    fn test_css_vars_generation() {
        let theme = &LIGHT_THEME;
        let css = theme.to_css_vars();
        assert!(css.contains("--color-primary: #3b82f6"));
        assert!(css.contains("--spacing-md: 16px"));
        assert!(css.contains("--focus-ring-width: 2px"));
    }

    #[test]
    fn test_accessibility_requirements() {
        let theme = Theme::current();
        // Ensure focus ring meets WCAG 2.1 AA requirements
        assert_eq!(theme.borders.focus_ring_width, "2px");
        
        // Ensure touch targets meet minimum size (handled in components)
        // Base spacing unit supports 44px minimum touch targets
        assert_eq!(theme.spacing.xs, "4px"); // 11 * 4px = 44px
    }
}
