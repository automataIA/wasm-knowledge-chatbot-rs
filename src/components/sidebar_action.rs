use leptos::prelude::*;

#[component]
pub fn SidebarAction(
    icon: &'static str,
    label: &'static str,
    collapsed: ReadSignal<bool>,
) -> impl IntoView {
    view! {
        <button class="btn btn-ghost justify-start gap-2 w-full">
            <i data-lucide=icon class="h-4 w-4 flex-shrink-0"></i>
            <Show when=move || !collapsed.get()>
                <span class="truncate">{label}</span>
            </Show>
        </button>
    }
}