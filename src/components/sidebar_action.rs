use crate::components::ui_primitives::Button;
use leptos::prelude::*;

#[component]
pub fn SidebarAction(
    icon: &'static str,
    label: &'static str,
    collapsed: ReadSignal<bool>,
    #[prop(optional)] on_click: Option<Box<dyn Fn() + 'static>>,
) -> impl IntoView {
    view! {
        <Button
            label=Signal::derive(move || if collapsed.get() { "".to_string() } else { label.to_string() })
            variant=Signal::derive(|| "btn-ghost justify-start w-full".to_string())
            icon=Signal::derive(|| icon.to_string())
            icon_position=Signal::derive(|| "left".to_string())
            on_click=on_click.unwrap_or_else(|| Box::new(|| {}))
        />
    }
}
