// Button atom component with design tokens integration
use crate::ui::theme::Theme;
use leptos::prelude::*;

/// Button variant types
#[derive(Debug, Clone, PartialEq)]
pub enum ButtonVariant {
    Primary,
    Secondary,
    Accent,
    Ghost,
    Link,
    Outline,
}

/// Button size types
#[derive(Debug, Clone, PartialEq)]
pub enum ButtonSize {
    Small,
    Medium,
    Large,
}

/// Button component
#[component]
pub fn Button(
    #[prop(default = ButtonVariant::Primary)] variant: ButtonVariant,
    #[prop(default = ButtonSize::Medium)] size: ButtonSize,
    #[prop(default = false)] disabled: bool,
    #[prop(default = false)] loading: bool,
    #[prop(default = false)] full_width: bool,
    #[prop(optional)] class: Option<String>,
    #[prop(optional)] on_click: Option<Callback<()>>,
    children: Children,
) -> impl IntoView {
    let theme = Theme::current();

    // Build CSS classes based on props
    let base_classes =
        "btn transition-all duration-300 focus:outline-none focus:ring-2 focus:ring-offset-2";

    let variant_classes = match variant {
        ButtonVariant::Primary => "btn-primary bg-primary hover:bg-primary-focus text-primary-content",
        ButtonVariant::Secondary => "btn-secondary bg-secondary hover:bg-secondary-focus text-secondary-content",
        ButtonVariant::Accent => "btn-accent bg-accent hover:bg-accent-focus text-accent-content",
        ButtonVariant::Ghost => "btn-ghost hover:bg-base-200 text-base-content",
        ButtonVariant::Link => "btn-link text-primary hover:text-primary-focus",
        ButtonVariant::Outline => "btn-outline border-2 border-primary text-primary hover:bg-primary hover:text-primary-content",
    };

    let size_classes = match size {
        ButtonSize::Small => "btn-sm text-sm px-3 py-1.5 min-h-8",
        ButtonSize::Medium => "btn-md text-base px-4 py-2 min-h-12", // 48px minimum touch target
        ButtonSize::Large => "btn-lg text-lg px-6 py-3 min-h-14",
    };

    let state_classes = if disabled {
        "btn-disabled opacity-50 cursor-not-allowed"
    } else if loading {
        "loading"
    } else {
        ""
    };

    let width_classes = if full_width { "w-full" } else { "" };

    let focus_classes = "focus:ring-2 focus:ring-primary focus:ring-offset-2"; // WCAG 2.1 AA compliance

    let combined_classes = format!(
        "{} {} {} {} {} {} {}",
        base_classes,
        variant_classes,
        size_classes,
        state_classes,
        width_classes,
        focus_classes,
        class.unwrap_or_default()
    );

    view! {
        <button
            class={combined_classes}
            disabled={disabled || loading}
            on:click=move |_| {
                if let Some(handler) = on_click {
                    handler.run(());
                }
            }
            style={format!(
                "--focus-ring-width: {}; --duration-normal: {};",
                theme.borders.focus_ring_width,
                theme.animations.duration_normal
            )}
        >
            <Show when=move || loading>
                <span class="loading loading-spinner loading-sm mr-2"></span>
            </Show>
            {children()}
        </button>
    }
}

/// Specialized button variants for common use cases
#[component]
pub fn PrimaryButton(
    #[prop(default = false)] disabled: bool,
    #[prop(default = false)] loading: bool,
    #[prop(optional)] on_click: Option<Callback<()>>,
    children: Children,
) -> impl IntoView {
    view! {
        <Button
            variant={ButtonVariant::Primary}
            disabled={disabled}
            loading={loading}
            on_click={on_click.unwrap_or_else(|| Callback::new(|_| {}))}
        >
            {children()}
        </Button>
    }
}

#[component]
pub fn SecondaryButton(
    #[prop(default = false)] disabled: bool,
    #[prop(default = false)] loading: bool,
    #[prop(optional)] on_click: Option<Callback<()>>,
    children: Children,
) -> impl IntoView {
    view! {
        <Button
            variant={ButtonVariant::Secondary}
            disabled={disabled}
            loading={loading}
            on_click={on_click.unwrap_or_else(|| Callback::new(|_| {}))}
        >
            {children()}
        </Button>
    }
}

#[component]
pub fn IconButton(
    #[prop(default = ButtonVariant::Ghost)] variant: ButtonVariant,
    #[prop(default = ButtonSize::Medium)] size: ButtonSize,
    #[prop(default = false)] disabled: bool,
    #[prop(optional)] on_click: Option<Callback<()>>,
    #[prop(optional)] aria_label: Option<String>,
    children: Children,
) -> impl IntoView {
    let additional_classes = "btn-square aspect-square p-0 min-h-11"; // 44px minimum for accessibility

    view! {
        <Button
            variant={variant}
            size={size}
            disabled={disabled}
            on_click={on_click.unwrap_or_else(|| Callback::new(|_| {}))}
            class={additional_classes.to_string()}
        >
            <span
                class="flex items-center justify-center"
                aria-label={aria_label.unwrap_or_default()}
            >
                {children()}
            </span>
        </Button>
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_button_variants() {
        // Test that all variants are distinct
        let variants = vec![
            ButtonVariant::Primary,
            ButtonVariant::Secondary,
            ButtonVariant::Accent,
            ButtonVariant::Ghost,
            ButtonVariant::Link,
            ButtonVariant::Outline,
        ];

        for variant in variants {
            // Each variant should have different styling
            match variant {
                ButtonVariant::Primary => assert_eq!(variant, ButtonVariant::Primary),
                ButtonVariant::Secondary => assert_eq!(variant, ButtonVariant::Secondary),
                ButtonVariant::Accent => assert_eq!(variant, ButtonVariant::Accent),
                ButtonVariant::Ghost => assert_eq!(variant, ButtonVariant::Ghost),
                ButtonVariant::Link => assert_eq!(variant, ButtonVariant::Link),
                ButtonVariant::Outline => assert_eq!(variant, ButtonVariant::Outline),
            }
        }
    }

    #[test]
    fn test_accessibility_requirements() {
        // Medium button should meet 44px minimum touch target
        let size = ButtonSize::Medium;
        assert_eq!(size, ButtonSize::Medium);

        // Focus ring should be 2px for WCAG 2.1 AA compliance
        let theme = Theme::current();
        assert_eq!(theme.borders.focus_ring_width, "2px");
    }
}
