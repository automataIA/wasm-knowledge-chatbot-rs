use crate::models::LLMModel;
use crate::components::ui_primitives::Button;
use leptos::prelude::*;

#[component]
pub fn LLMSelect(
    selected: ReadSignal<String>,
    set_selected: WriteSignal<String>,
    llms: Vec<LLMModel>,
    set_status_message: WriteSignal<String>,
) -> impl IntoView {
    let (is_open, set_is_open) = signal(false);

    // Create a memo for the llms to make them reactive
    let llms_signal = Memo::new(move |_| llms.clone());

    view! {
        <div class="relative w-full">
            <Button
                label=Signal::derive(move || {
                    llms_signal.get().iter()
                        .find(|llm| llm.id == selected.get())
                        .map(|llm| format!("{} ({})", llm.name, llm.provider))
                        .unwrap_or_else(|| "Select model".to_string())
                })
                variant=Signal::derive(|| "btn-outline w-full justify-start pr-10".to_string())
                on_click=Box::new(move || set_is_open.update(|open| *open = !*open))
            />
            <i data-lucide="chevron-down" class="h-4 w-4 absolute right-3 top-1/2 -translate-y-1/2 pointer-events-none"></i>

            <Show when=move || is_open.get()>
                <div class="absolute top-full left-0 w-full mt-1 bg-base-100 border border-base-300 rounded-lg shadow-lg z-50">
                    <ul class="menu p-2">
                        {move || {
                            llms_signal.get().into_iter().map(|llm| {
                                let llm_id = llm.id.clone();
                                let llm_id_for_class = llm_id.clone();
                                let llm_name = llm.name.clone();
                                let llm_provider = llm.provider.clone();

                                view! {
                                    <li>
                                        <a
                                            class=move || if selected.get() == llm_id_for_class { "active" } else { "" }
                                            on:click=move |_| {
                                                set_selected.set(llm_id.clone());
                                                set_is_open.set(false);
                                                set_status_message.set("Model changed".to_string());
                                            }
                                        >
                                            <div class="flex flex-col items-start">
                                                <span class="font-medium">{llm_name.clone()}</span>
                                                <span class="text-xs opacity-70">{llm_provider.clone()}</span>
                                            </div>
                                        </a>
                                    </li>
                                }
                            }).collect_view()
                        }}
                    </ul>
                </div>
            </Show>
        </div>
    }
}
