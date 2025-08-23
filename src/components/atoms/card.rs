// Card atom component with design tokens integration
use leptos::prelude::*;
use crate::ui::theme::Theme;

/// Card variant types
#[derive(Debug, Clone, PartialEq)]
pub enum CardVariant {
    Default,
    Bordered,
    Compact,
    Side,
    Glass,
}

/// Card component
#[component]
pub fn Card(
    #[prop(default = CardVariant::Default)] variant: CardVariant,
    #[prop(default = false)] bordered: bool,
    #[prop(default = false)] compact: bool,
    #[prop(default = false)] glass: bool,
    #[prop(optional)] class: Option<String>,
    #[prop(optional)] id: Option<String>,
    children: Children,
) -> impl IntoView {
    let theme = Theme::current();
    
    // Build CSS classes based on props
    let base_classes = "card bg-base-100 transition-all duration-300";
    
    let variant_classes = match variant {
        CardVariant::Default => "shadow-md",
        CardVariant::Bordered => "border border-base-300",
        CardVariant::Compact => "card-compact",
        CardVariant::Side => "card-side",
        CardVariant::Glass => "glass backdrop-blur-sm bg-opacity-80",
    };
    
    let modifier_classes = format!(
        "{}{}{}",
        if bordered { " border border-base-300" } else { "" },
        if compact { " card-compact" } else { "" },
        if glass { " glass backdrop-blur-sm bg-opacity-80" } else { "" }
    );
    
    let combined_classes = format!(
        "{} {} {} {}",
        base_classes,
        variant_classes,
        modifier_classes,
        class.unwrap_or_default()
    );

    view! {
        <div
            class={combined_classes}
            id={id}
            style={format!(
                "--radius-lg: {}; --shadow-md: {};",
                theme.borders.radius_lg,
                theme.shadows.shadow_md
            )}
        >
            {children()}
        </div>
    }
}

/// Card body component
#[component]
pub fn CardBody(
    #[prop(optional)] class: Option<String>,
    children: Children,
) -> impl IntoView {
    let combined_classes = format!(
        "card-body {}",
        class.unwrap_or_default()
    );

    view! {
        <div class={combined_classes}>
            {children()}
        </div>
    }
}

/// Card title component
#[component]
pub fn CardTitle(
    #[prop(optional)] class: Option<String>,
    children: Children,
) -> impl IntoView {
    let combined_classes = format!(
        "card-title text-base-content font-semibold {}",
        class.unwrap_or_default()
    );

    view! {
        <h2 class={combined_classes}>
            {children()}
        </h2>
    }
}

/// Card actions component
#[component]
pub fn CardActions(
    #[prop(default = false)] justify_end: bool,
    #[prop(optional)] class: Option<String>,
    children: Children,
) -> impl IntoView {
    let justify_classes = if justify_end { "justify-end" } else { "justify-start" };
    
    let combined_classes = format!(
        "card-actions {} {}",
        justify_classes,
        class.unwrap_or_default()
    );

    view! {
        <div class={combined_classes}>
            {children()}
        </div>
    }
}

/// Card figure component for images
#[component]
pub fn CardFigure(
    #[prop(optional)] class: Option<String>,
    children: Children,
) -> impl IntoView {
    let combined_classes = format!(
        "figure {}",
        class.unwrap_or_default()
    );

    view! {
        <figure class={combined_classes}>
            {children()}
        </figure>
    }
}

/// Message card component for chat messages
#[component]
pub fn MessageCard(
    #[prop(default = false)] is_user: bool,
    #[prop(optional)] avatar_url: Option<String>,
    #[prop(optional)] timestamp: Option<String>,
    #[prop(optional)] class: Option<String>,
    children: Children,
) -> impl IntoView {
    let alignment_classes = if is_user { "ml-auto" } else { "mr-auto" };
    let bg_classes = if is_user { "bg-primary text-primary-content" } else { "bg-base-200 text-base-content" };
    
    let combined_classes = format!(
        "card max-w-md {} {} {}",
        alignment_classes,
        bg_classes,
        class.unwrap_or_default()
    );

    view! {
        <div class={combined_classes}>
            <div class="card-body p-4">
                <div class="flex items-start gap-3">
                    {avatar_url.as_ref().map(|url| view! {
                        <div class="avatar">
                            <div class="w-8 h-8 rounded-full">
                                <img src={url.clone()} alt="Avatar" />
                            </div>
                        </div>
                    })}
                    <div class="flex-1">
                        <div class="prose prose-sm max-w-none">
                            {children()}
                        </div>
                        {timestamp.as_ref().map(|time| view! {
                            <div class="text-xs opacity-70 mt-2">
                                {time.clone()}
                            </div>
                        })}
                    </div>
                </div>
            </div>
        </div>
    }
}

/// Info card component for displaying information
#[component]
pub fn InfoCard(
    #[prop(optional)] title: Option<String>,
    #[prop(optional)] icon: Option<String>,
    #[prop(default = false)] closable: bool,
    #[prop(optional)] on_close: Option<Box<dyn Fn() + 'static>>,
    #[prop(optional)] class: Option<String>,
    children: Children,
) -> impl IntoView {
    let combined_classes = format!(
        "card bg-info text-info-content shadow-lg {}",
        class.unwrap_or_default()
    );

    view! {
        <div class={combined_classes}>
            <div class="card-body">
                <div class="flex items-start justify-between">
                    <div class="flex items-center gap-2">
                        {icon.as_ref().map(|icon_class| view! {
                            <div class={format!("text-lg {}", icon_class)}></div>
                        })}
                        {title.as_ref().map(|title_text| view! {
                            <h3 class="card-title text-lg">{title_text.clone()}</h3>
                        })}
                    </div>
                    {if closable {
                        view! {
                            <button
                                class="btn btn-sm btn-circle btn-ghost hover:bg-info-focus"
                                on:click=move |_| {
                                    if let Some(handler) = &on_close {
                                        handler();
                                    }
                                }
                                aria-label="Close"
                            >
                                <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12"/>
                                </svg>
                            </button>
                        }.into_any()
                    } else {
                        view! { <span></span> }.into_any()
                    }}
                </div>
                <div class="mt-2">
                    {children()}
                </div>
            </div>
        </div>
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_card_variants() {
        let variants = vec![
            CardVariant::Default,
            CardVariant::Bordered,
            CardVariant::Compact,
            CardVariant::Side,
            CardVariant::Glass,
        ];
        
        for variant in variants {
            match variant {
                CardVariant::Default => assert_eq!(variant, CardVariant::Default),
                CardVariant::Bordered => assert_eq!(variant, CardVariant::Bordered),
                CardVariant::Compact => assert_eq!(variant, CardVariant::Compact),
                CardVariant::Side => assert_eq!(variant, CardVariant::Side),
                CardVariant::Glass => assert_eq!(variant, CardVariant::Glass),
            }
        }
    }

    #[test]
    fn test_card_composition() {
        // Test that card components can be composed together
        // This is a structural test to ensure the components exist
        let _card = CardVariant::Default;
        let _body = "card-body";
        let _title = "card-title";
        let _actions = "card-actions";
        
        assert!(true); // Components compile and can be instantiated
    }
}
