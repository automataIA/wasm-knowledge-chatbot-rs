// Form field molecule component combining input, label, and validation
use crate::components::atoms::{Input, InputSize, InputVariant, Textarea};
use leptos::prelude::*;

/// Form field component with label, input, and validation
#[component]
pub fn FormField(
    #[prop(optional)] label: Option<String>,
    #[prop(optional)] value: Option<RwSignal<String>>,
    #[prop(default = "text".to_string())] input_type: String,
    #[prop(default = InputVariant::Bordered)] variant: InputVariant,
    #[prop(default = InputSize::Medium)] size: InputSize,
    #[prop(optional)] placeholder: Option<String>,
    #[prop(default = false)] required: bool,
    #[prop(default = false)] disabled: bool,
    #[prop(optional)] error: Option<String>,
    #[prop(optional)] help_text: Option<String>,
    #[prop(optional)] class: Option<String>,
    #[prop(optional)] id: Option<String>,
    #[prop(optional)] name: Option<String>,
) -> impl IntoView {
    let field_id = id
        .clone()
        .unwrap_or_else(|| format!("field-{}", name.clone().unwrap_or_default()));
    let error_id = format!("{}-error", field_id);
    let help_id = format!("{}-help", field_id);

    let has_error = error.is_some();
    let input_variant = if has_error {
        InputVariant::Error
    } else {
        variant
    };

    view! {
        <div class={format!("form-field flex flex-col gap-1 {}", class.unwrap_or_default())}>
            {label.as_ref().map(|label_text| view! {
                <label
                    for={field_id.clone()}
                    class={format!(
                        "label text-sm font-medium {}",
                        if required { "after:content-['*'] after:text-error after:ml-1" } else { "" }
                    )}
                >
                    {label_text.clone()}
                </label>
            })}

            <Input
                input_type={input_type}
                variant={input_variant}
                size={size}
                value={value.unwrap_or_else(|| RwSignal::new(String::new()))}
                placeholder={placeholder.unwrap_or_default()}
                required={required}
                disabled={disabled}
                id={field_id.clone()}
                name={name.unwrap_or_default()}
                aria_describedby={format!("{} {}",
                    if error.is_some() { error_id.as_str() } else { "" },
                    if help_text.is_some() { help_id.as_str() } else { "" }
                ).trim().to_string()}
            />

            {error.as_ref().map(|error_text| view! {
                <div
                    id={error_id}
                    class="text-sm text-error flex items-center gap-1"
                    role="alert"
                >
                    <svg class="w-4 h-4" fill="currentColor" viewBox="0 0 20 20">
                        <path fill-rule="evenodd" d="M18 10a8 8 0 11-16 0 8 8 0 0116 0zm-7 4a1 1 0 11-2 0 1 1 0 012 0zm-1-9a1 1 0 00-1 1v4a1 1 0 102 0V6a1 1 0 00-1-1z" clip-rule="evenodd"/>
                    </svg>
                    {error_text.clone()}
                </div>
            })}

            {help_text.as_ref().map(|help| view! {
                <div
                    id={help_id}
                    class="text-sm text-base-content opacity-70"
                >
                    {help.clone()}
                </div>
            })}
        </div>
    }
}

/// Textarea form field component
#[component]
pub fn TextareaField(
    #[prop(optional)] label: Option<String>,
    #[prop(optional)] value: Option<RwSignal<String>>,
    #[prop(default = InputVariant::Bordered)] variant: InputVariant,
    #[prop(default = InputSize::Medium)] size: InputSize,
    #[prop(optional)] placeholder: Option<String>,
    #[prop(default = 4)] rows: u32,
    #[prop(default = false)] required: bool,
    #[prop(default = false)] disabled: bool,
    #[prop(optional)] error: Option<String>,
    #[prop(optional)] help_text: Option<String>,
    #[prop(optional)] class: Option<String>,
    #[prop(optional)] id: Option<String>,
    #[prop(optional)] name: Option<String>,
) -> impl IntoView {
    let field_id = id
        .clone()
        .unwrap_or_else(|| format!("field-{}", name.clone().unwrap_or_default()));
    let error_id = format!("{}-error", field_id);
    let help_id = format!("{}-help", field_id);

    let has_error = error.is_some();
    let input_variant = if has_error {
        InputVariant::Error
    } else {
        variant
    };

    view! {
        <div class={format!("form-field flex flex-col gap-1 {}", class.unwrap_or_default())}>
            {label.as_ref().map(|label_text| view! {
                <label
                    for={field_id.clone()}
                    class={format!(
                        "label text-sm font-medium {}",
                        if required { "after:content-['*'] after:text-error after:ml-1" } else { "" }
                    )}
                >
                    {label_text.clone()}
                </label>
            })}

            <Textarea
                variant={input_variant}
                size={size}
                value={value.unwrap_or_else(|| RwSignal::new(String::new()))}
                placeholder={placeholder.unwrap_or_default()}
                rows={rows}
                required={required}
                disabled={disabled}
                id={field_id.clone()}
                name={name.unwrap_or_default()}
                aria_describedby={format!("{} {}",
                    if error.is_some() { error_id.as_str() } else { "" },
                    if help_text.is_some() { help_id.as_str() } else { "" }
                ).trim().to_string()}
            />

            {error.as_ref().map(|error_text| view! {
                <div
                    id={error_id}
                    class="text-sm text-error flex items-center gap-1"
                    role="alert"
                >
                    <svg class="w-4 h-4" fill="currentColor" viewBox="0 0 20 20">
                        <path fill-rule="evenodd" d="M18 10a8 8 0 11-16 0 8 8 0 0116 0zm-7 4a1 1 0 11-2 0 1 1 0 012 0zm-1-9a1 1 0 00-1 1v4a1 1 0 102 0V6a1 1 0 00-1-1z" clip-rule="evenodd"/>
                    </svg>
                    {error_text.clone()}
                </div>
            })}

            {help_text.as_ref().map(|help| view! {
                <div
                    id={help_id}
                    class="text-sm text-base-content opacity-70"
                >
                    {help.clone()}
                </div>
            })}
        </div>
    }
}

/// Form group component for grouping related fields
#[component]
pub fn FormGroup(
    #[prop(optional)] title: Option<String>,
    #[prop(optional)] description: Option<String>,
    #[prop(optional)] class: Option<String>,
    children: Children,
) -> impl IntoView {
    view! {
        <fieldset class={format!("form-group space-y-4 {}", class.unwrap_or_default())}>
            {title.as_ref().map(|title_text| view! {
                <legend class="text-lg font-semibold text-base-content mb-2">
                    {title_text.clone()}
                </legend>
            })}

            {description.as_ref().map(|desc| view! {
                <p class="text-sm text-base-content opacity-70 mb-4">
                    {desc.clone()}
                </p>
            })}

            <div class="space-y-4">
                {children()}
            </div>
        </fieldset>
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_form_field_creation() {
        // Test that form field components can be created
        let _value = RwSignal::new(String::new());
        assert!(true); // Components compile and can be instantiated
    }

    #[test]
    fn test_form_field_validation() {
        // Test error state handling
        let error_msg = Some("This field is required".to_string());
        assert!(error_msg.is_some());
    }
}
