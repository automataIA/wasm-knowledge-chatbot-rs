// Navigation molecule components
use leptos::prelude::*;
// Removed unused Button imports

/// Navigation item data structure
#[derive(Debug, Clone, PartialEq)]
pub struct NavItem {
    pub label: String,
    pub href: Option<String>,
    pub icon: Option<String>,
    pub active: bool,
    pub disabled: bool,
    pub badge: Option<String>,
}

impl NavItem {
    pub fn new(label: impl Into<String>) -> Self {
        Self {
            label: label.into(),
            href: None,
            icon: None,
            active: false,
            disabled: false,
            badge: None,
        }
    }

    pub fn with_href(mut self, href: impl Into<String>) -> Self {
        self.href = Some(href.into());
        self
    }

    pub fn with_icon(mut self, icon: impl Into<String>) -> Self {
        self.icon = Some(icon.into());
        self
    }

    pub fn active(mut self) -> Self {
        self.active = true;
        self
    }

    pub fn disabled(mut self) -> Self {
        self.disabled = true;
        self
    }

    pub fn with_badge(mut self, badge: impl Into<String>) -> Self {
        self.badge = Some(badge.into());
        self
    }
}

/// Horizontal navigation bar component
#[component]
pub fn NavBar(
    #[prop(optional)] brand: Option<String>,
    #[prop(optional)] brand_href: Option<String>,
    #[prop(default = vec![])] items: Vec<NavItem>,
    #[prop(optional)] on_item_click: Option<Callback<String>>,
    #[prop(optional)] class: Option<String>,
) -> impl IntoView {
    view! {
        <nav class={format!("navbar bg-base-100 shadow-sm {}", class.unwrap_or_default())}>
            <div class="navbar-start">
                {brand.as_ref().map(|brand_text| view! {
                    <a
                        href={brand_href.clone().unwrap_or_else(|| "#".to_string())}
                        class="btn btn-ghost text-xl font-bold"
                    >
                        {brand_text.clone()}
                    </a>
                })}
            </div>

            <div class="navbar-center hidden lg:flex">
                <ul class="menu menu-horizontal px-1 gap-1">
                    {items.clone().into_iter().map(|item| {
                        let item_label = item.label.clone();
                        view! {
                            <li>
                                <a
                                    href={item.href.unwrap_or_else(|| "#".to_string())}
                                    class={format!(
                                        "flex items-center gap-2 {}",
                                        if item.active { "active" } else { "" }
                                    )}
                                    class:disabled={item.disabled}
                                    on:click=move |_| {
                                        if let Some(handler) = on_item_click {
                                            handler.run(item_label.clone());
                                        }
                                    }
                                >
                                    {item.icon.as_ref().map(|icon| view! {
                                        <span class={format!("icon-{}", icon)}></span>
                                    })}

                                    <span>{item.label}</span>

                                    {item.badge.as_ref().map(|badge_text| view! {
                                        <span class="badge badge-sm badge-primary">
                                            {badge_text.clone()}
                                        </span>
                                    })}
                                </a>
                            </li>
                        }
                    }).collect::<Vec<_>>()}
                </ul>
            </div>

            <div class="navbar-end">
                <div class="dropdown dropdown-end lg:hidden">
                    <div tabindex="0" role="button" class="btn btn-ghost">
                        <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 6h16M4 12h16M4 18h16"/>
                        </svg>
                    </div>
                    <ul tabindex="0" class="menu menu-sm dropdown-content mt-3 z-[1] p-2 shadow bg-base-100 rounded-box w-52">
                        {items.into_iter().map(|item| {
                            let item_label = item.label.clone();
                            view! {
                                <li>
                                    <a
                                        href={item.href.unwrap_or_else(|| "#".to_string())}
                                        class={if item.active { "active" } else { "" }}
                                        class:disabled={item.disabled}
                                        on:click=move |_| {
                                            if let Some(handler) = on_item_click {
                                                handler.run(item_label.clone());
                                            }
                                        }
                                    >
                                        {item.icon.as_ref().map(|icon| view! {
                                            <span class={format!("icon-{}", icon)}></span>
                                        })}

                                        {item.label}

                                        {item.badge.as_ref().map(|badge_text| view! {
                                            <span class="badge badge-xs badge-primary">
                                                {badge_text.clone()}
                                            </span>
                                        })}
                                    </a>
                                </li>
                            }
                        }).collect::<Vec<_>>()}
                    </ul>
                </div>
            </div>
        </nav>
    }
}

/// Vertical sidebar navigation component
#[component]
pub fn SideNav(
    #[prop(default = vec![])] items: Vec<NavItem>,
    #[prop(default = false)] collapsed: bool,
    #[prop(optional)] on_item_click: Option<Callback<String>>,
    #[prop(optional)] class: Option<String>,
) -> impl IntoView {
    view! {
        <nav class={format!(
            "sidenav flex flex-col bg-base-200 h-full transition-all duration-300 {} {}",
            if collapsed { "w-16" } else { "w-64" },
            class.unwrap_or_default()
        )}>
            <ul class="menu p-2 gap-1">
                {items.into_iter().map(|item| {
                    let item_label = item.label.clone();
                    view! {
                        <li>
                            <a
                                href={item.href.unwrap_or_else(|| "#".to_string())}
                                class={format!(
                                    "flex items-center gap-3 {}",
                                    if item.active { "active" } else { "" }
                                )}
                                class:disabled={item.disabled}
                                class:tooltip={collapsed}
                                class:tooltip-right={collapsed}
                                data-tip={if collapsed { Some(item.label.clone()) } else { None }}
                                on:click=move |_| {
                                    if let Some(handler) = on_item_click {
                                        handler.run(item_label.clone());
                                    }
                                }
                            >
                                {item.icon.as_ref().map(|icon| view! {
                                    <span class={format!("icon-{} text-lg", icon)}></span>
                                })}

                                {(!collapsed).then(|| view! {
                                    <span class="flex-1">{item.label.clone()}</span>
                                    {item.badge.as_ref().map(|badge_text| view! {
                                        <span class="badge badge-sm badge-primary">
                                            {badge_text.clone()}
                                        </span>
                                    })}
                                })}
                            </a>
                        </li>
                    }
                }).collect::<Vec<_>>()}
            </ul>
        </nav>
    }
}

/// Breadcrumb navigation component
#[component]
pub fn Breadcrumb(
    #[prop(default = vec![])] items: Vec<NavItem>,
    #[prop(optional)] on_item_click: Option<Callback<String>>,
    #[prop(optional)] class: Option<String>,
) -> impl IntoView {
    view! {
        <nav class={format!("breadcrumbs text-sm {}", class.unwrap_or_default())}>
            <ul class="flex items-center gap-2">
                {items.clone().into_iter().enumerate().map(|(index, item)| {
                let items_len = items.len();
                let is_last = index == items_len - 1;
                let item_label = item.label.clone();

                view! {
                    <li class="flex items-center gap-2">
                        {if !is_last {
                            view! {
                                <a
                                    href={item.href.clone().unwrap_or_else(|| "#".to_string())}
                                    class="link link-hover"
                                    on:click=move |_| {
                                        if let Some(handler) = on_item_click {
                                            handler.run(item_label.clone());
                                        }
                                    }
                                >
                                    {item.label.clone()}
                                </a>
                            }.into_any()
                        } else {
                            view! {
                                <span class="text-base-content opacity-70">
                                    {item.label.clone()}
                                </span>
                            }.into_any()
                        }}

                        {(!is_last).then(|| view! {
                            <svg class="w-4 h-4 opacity-50" fill="currentColor" viewBox="0 0 20 20">
                                <path fill-rule="evenodd" d="M7.293 14.707a1 1 0 010-1.414L10.586 10 7.293 6.707a1 1 0 011.414-1.414l4 4a1 1 0 010 1.414l-4 4a1 1 0 01-1.414 0z" clip-rule="evenodd"/>
                            </svg>
                        })}
                    </li>
                }
            }).collect::<Vec<_>>()}
            </ul>
        </nav>
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_nav_item_builder() {
        let item = NavItem::new("Home")
            .with_href("/")
            .with_icon("home")
            .active();

        assert_eq!(item.label, "Home");
        assert_eq!(item.href, Some("/".to_string()));
        assert_eq!(item.icon, Some("home".to_string()));
        assert!(item.active);
    }

    #[test]
    fn test_nav_components_creation() {
        let items = vec![
            NavItem::new("Home").with_href("/"),
            NavItem::new("About").with_href("/about"),
        ];

        assert_eq!(items.len(), 2);
    }
}
