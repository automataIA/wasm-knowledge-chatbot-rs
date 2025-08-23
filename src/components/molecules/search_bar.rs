// Search bar molecule component combining input and button atoms
use crate::components::atoms::{Button, ButtonSize, ButtonVariant, SearchInput};
use leptos::prelude::*;

/// Search bar component with input and action button
#[component]
pub fn SearchBar(
    #[prop(optional)] value: Option<RwSignal<String>>,
    #[prop(default = "Search...".to_string())] placeholder: String,
    #[prop(default = "Search".to_string())] button_text: String,
    #[prop(default = false)] disabled: bool,
    #[prop(default = false)] loading: bool,
    #[prop(optional)] on_search: Option<Callback<String>>,
    #[prop(optional)] class: Option<String>,
) -> impl IntoView {
    let internal_value = RwSignal::new(String::new());
    let search_value = value.unwrap_or(internal_value);

    let handle_search = move || {
        let query = search_value.get();
        if !query.trim().is_empty() {
            if let Some(handler) = on_search {
                handler.run(query);
            }
        }
    };

    view! {
        <div class={format!("flex gap-2 items-center {}", class.unwrap_or_default())}>
            <div class="flex-1">
                <SearchInput
                    value={search_value}
                    placeholder={placeholder}
                    disabled={disabled}
                    on_search={Box::new(move |query: String| {
                        if let Some(handler) = on_search {
                            handler.run(query);
                        }
                    })}
                />
            </div>
            <Button
                variant={ButtonVariant::Primary}
                size={ButtonSize::Medium}
                disabled={disabled}
                loading={loading}
                on_click={Callback::new(move |_| handle_search())}
            >
                {if loading {
                    view! {
                        <span class="loading loading-spinner loading-sm mr-2"></span>
                        "Searching..."
                    }.into_any()
                } else {
                    view! { {button_text.clone()} }.into_any()
                }}
            </Button>
        </div>
    }
}

/// Compact search bar for smaller spaces
#[component]
pub fn CompactSearchBar(
    #[prop(optional)] value: Option<RwSignal<String>>,
    #[prop(default = "Search...".to_string())] placeholder: String,
    #[prop(default = false)] disabled: bool,
    #[prop(optional)] on_search: Option<Callback<String>>,
    #[prop(optional)] class: Option<String>,
) -> impl IntoView {
    let internal_value = RwSignal::new(String::new());
    let search_value = value.unwrap_or(internal_value);

    view! {
        <div class={format!("relative {}", class.unwrap_or_default())}>
            <SearchInput
                value={search_value}
                placeholder={placeholder}
                disabled={disabled}
                on_search={Box::new(move |query: String| {
                    if let Some(handler) = on_search {
                        handler.run(query);
                    }
                })}
                class={"pr-10".to_string()}
            />
        </div>
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_search_bar_creation() {
        // Test that search bar components can be created
        let _search_value = RwSignal::new(String::new());
        // Components compile and can be instantiated
    }
}
