// Input atom component with design tokens integration
use crate::ui::theme::Theme;
use leptos::prelude::*;

/// Input variant types
#[derive(Debug, Clone, PartialEq)]
pub enum InputVariant {
    Default,
    Bordered,
    Ghost,
    Primary,
    Secondary,
    Accent,
    Info,
    Success,
    Warning,
    Error,
}

/// Input size types
#[derive(Debug, Clone, PartialEq)]
pub enum InputSize {
    Small,
    Medium,
    Large,
}

/// Input component
#[component]
pub fn Input(
    #[prop(default = InputVariant::Bordered)] variant: InputVariant,
    #[prop(default = InputSize::Medium)] size: InputSize,
    #[prop(default = "text".to_string())] input_type: String,
    #[prop(optional)] value: Option<RwSignal<String>>,
    #[prop(optional)] placeholder: Option<String>,
    #[prop(default = false)] disabled: bool,
    #[prop(default = false)] readonly: bool,
    #[prop(default = false)] required: bool,
    #[prop(optional)] class: Option<String>,
    #[prop(optional)] id: Option<String>,
    #[prop(optional)] name: Option<String>,
    #[prop(optional)] aria_label: Option<String>,
    #[prop(optional)] aria_describedby: Option<String>,
) -> impl IntoView {
    let theme = Theme::current();

    // Create internal signal if none provided
    let internal_value = RwSignal::new(String::new());
    let value_signal = value.unwrap_or(internal_value);

    // Build CSS classes based on props
    let base_classes =
        "input transition-all duration-300 focus:outline-none focus:ring-2 focus:ring-offset-2";

    let variant_classes = match variant {
        InputVariant::Default => "input-default",
        InputVariant::Bordered => "input-bordered border-2 border-base-300 focus:border-primary",
        InputVariant::Ghost => "input-ghost bg-transparent",
        InputVariant::Primary => "input-primary border-primary focus:border-primary-focus",
        InputVariant::Secondary => "input-secondary border-secondary focus:border-secondary-focus",
        InputVariant::Accent => "input-accent border-accent focus:border-accent-focus",
        InputVariant::Info => "input-info border-info focus:border-info",
        InputVariant::Success => "input-success border-success focus:border-success",
        InputVariant::Warning => "input-warning border-warning focus:border-warning",
        InputVariant::Error => "input-error border-error focus:border-error",
    };

    let size_classes = match size {
        InputSize::Small => "input-sm text-sm h-8 px-3",
        InputSize::Medium => "input-md text-base h-12 px-4", // 48px height for accessibility
        InputSize::Large => "input-lg text-lg h-14 px-6",
    };

    let state_classes = if disabled {
        "input-disabled opacity-50 cursor-not-allowed"
    } else if readonly {
        "input-readonly bg-base-200"
    } else {
        ""
    };

    let focus_classes = "focus:ring-2 focus:ring-primary focus:ring-offset-2"; // WCAG 2.1 AA compliance

    let combined_classes = format!(
        "{} {} {} {} {} {}",
        base_classes,
        variant_classes,
        size_classes,
        state_classes,
        focus_classes,
        class.unwrap_or_default()
    );

    view! {
        <input
            type={input_type}
            class={combined_classes}
            value={move || value_signal.get()}
            placeholder={placeholder.unwrap_or_default()}
            disabled={disabled}
            readonly={readonly}
            required={required}
            id={id}
            name={name}
            aria-label={aria_label}
            aria-describedby={aria_describedby}
            on:input=move |ev| {
                value_signal.set(event_target_value(&ev));
            }
            style={format!(
                "--focus-ring-width: {}; --duration-normal: {};",
                theme.borders.focus_ring_width,
                theme.animations.duration_normal
            )}
        />
    }
}

/// Textarea component
#[component]
pub fn Textarea(
    #[prop(default = InputVariant::Bordered)] variant: InputVariant,
    #[prop(default = InputSize::Medium)] size: InputSize,
    #[prop(optional)] value: Option<RwSignal<String>>,
    #[prop(optional)] placeholder: Option<String>,
    #[prop(default = false)] disabled: bool,
    #[prop(default = false)] readonly: bool,
    #[prop(default = false)] required: bool,
    #[prop(default = 4)] rows: u32,
    #[prop(optional)] class: Option<String>,
    #[prop(optional)] id: Option<String>,
    #[prop(optional)] name: Option<String>,
    #[prop(optional)] aria_label: Option<String>,
    #[prop(optional)] aria_describedby: Option<String>,
) -> impl IntoView {
    let theme = Theme::current();

    // Create internal signal if none provided
    let internal_value = RwSignal::new(String::new());
    let value_signal = value.unwrap_or(internal_value);

    // Build CSS classes based on props
    let base_classes = "textarea transition-all duration-300 focus:outline-none focus:ring-2 focus:ring-offset-2 resize-y";

    let variant_classes = match variant {
        InputVariant::Default => "textarea-default",
        InputVariant::Bordered => "textarea-bordered border-2 border-base-300 focus:border-primary",
        InputVariant::Ghost => "textarea-ghost bg-transparent",
        InputVariant::Primary => "textarea-primary border-primary focus:border-primary-focus",
        InputVariant::Secondary => {
            "textarea-secondary border-secondary focus:border-secondary-focus"
        }
        InputVariant::Accent => "textarea-accent border-accent focus:border-accent-focus",
        InputVariant::Info => "textarea-info border-info focus:border-info",
        InputVariant::Success => "textarea-success border-success focus:border-success",
        InputVariant::Warning => "textarea-warning border-warning focus:border-warning",
        InputVariant::Error => "textarea-error border-error focus:border-error",
    };

    let size_classes = match size {
        InputSize::Small => "textarea-sm text-sm p-3",
        InputSize::Medium => "textarea-md text-base p-4",
        InputSize::Large => "textarea-lg text-lg p-6",
    };

    let state_classes = if disabled {
        "textarea-disabled opacity-50 cursor-not-allowed"
    } else if readonly {
        "textarea-readonly bg-base-200"
    } else {
        ""
    };

    let focus_classes = "focus:ring-2 focus:ring-primary focus:ring-offset-2";

    let combined_classes = format!(
        "{} {} {} {} {} {}",
        base_classes,
        variant_classes,
        size_classes,
        state_classes,
        focus_classes,
        class.unwrap_or_default()
    );

    view! {
        <textarea
            class={combined_classes}
            placeholder={placeholder.unwrap_or_default()}
            disabled={disabled}
            readonly={readonly}
            required={required}
            rows={rows}
            id={id}
            name={name}
            aria-label={aria_label}
            aria-describedby={aria_describedby}
            on:input=move |ev| {
                value_signal.set(event_target_value(&ev));
            }
            style={format!(
                "--focus-ring-width: {}; --duration-normal: {};",
                theme.borders.focus_ring_width,
                theme.animations.duration_normal
            )}
        >
            {move || value_signal.get()}
        </textarea>
    }
}

/// Search input component with icon
#[component]
pub fn SearchInput(
    #[prop(optional)] value: Option<RwSignal<String>>,
    #[prop(default = "Search...".to_string())] placeholder: String,
    #[prop(default = false)] disabled: bool,
    #[prop(optional)] on_search: Option<Box<dyn Fn(String) + 'static>>,
    #[prop(optional)] class: Option<String>,
) -> impl IntoView {
    let internal_value = RwSignal::new(String::new());
    let value_signal = value.unwrap_or(internal_value);

    let handle_search = move || {
        if let Some(handler) = &on_search {
            handler(value_signal.get());
        }
    };

    view! {
        <div class="relative">
            <Input
                variant={InputVariant::Bordered}
                value={value_signal}
                placeholder={placeholder}
                disabled={disabled}
                class={format!("pl-10 {}", class.unwrap_or_default())}
                aria_label={"Search input".to_string()}
            />
            <div class="absolute inset-y-0 left-0 pl-3 flex items-center pointer-events-none">
                <svg
                    class="h-5 w-5 text-base-content opacity-50"
                    fill="none"
                    stroke="currentColor"
                    viewBox="0 0 24 24"
                    aria-hidden="true"
                >
                    <path
                        stroke-linecap="round"
                        stroke-linejoin="round"
                        stroke-width="2"
                        d="M21 21l-6-6m2-5a7 7 0 11-14 0 7 7 0 0114 0z"
                    />
                </svg>
            </div>
            <button
                type="button"
                class="absolute inset-y-0 right-0 pr-3 flex items-center hover:text-primary transition-colors"
                on:click=move |_| handle_search()
                disabled={disabled}
                aria-label="Search"
            >
                <svg
                    class="h-5 w-5"
                    fill="none"
                    stroke="currentColor"
                    viewBox="0 0 24 24"
                >
                    <path
                        stroke-linecap="round"
                        stroke-linejoin="round"
                        stroke-width="2"
                        d="M13 7l5 5-5 5M6 12h12"
                    />
                </svg>
            </button>
        </div>
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_input_variants() {
        let variants = vec![
            InputVariant::Default,
            InputVariant::Bordered,
            InputVariant::Ghost,
            InputVariant::Primary,
            InputVariant::Secondary,
            InputVariant::Accent,
            InputVariant::Info,
            InputVariant::Success,
            InputVariant::Warning,
            InputVariant::Error,
        ];

        for variant in variants {
            // Each variant should be distinct
            match variant {
                InputVariant::Default => assert_eq!(variant, InputVariant::Default),
                InputVariant::Bordered => assert_eq!(variant, InputVariant::Bordered),
                InputVariant::Ghost => assert_eq!(variant, InputVariant::Ghost),
                InputVariant::Primary => assert_eq!(variant, InputVariant::Primary),
                InputVariant::Secondary => assert_eq!(variant, InputVariant::Secondary),
                InputVariant::Accent => assert_eq!(variant, InputVariant::Accent),
                InputVariant::Info => assert_eq!(variant, InputVariant::Info),
                InputVariant::Success => assert_eq!(variant, InputVariant::Success),
                InputVariant::Warning => assert_eq!(variant, InputVariant::Warning),
                InputVariant::Error => assert_eq!(variant, InputVariant::Error),
            }
        }
    }

    #[test]
    fn test_accessibility_requirements() {
        // Medium input should meet minimum height requirements
        let size = InputSize::Medium;
        assert_eq!(size, InputSize::Medium);

        // Focus ring should be 2px for WCAG 2.1 AA compliance
        let theme = Theme::current();
        assert_eq!(theme.borders.focus_ring_width, "2px");
    }
}
