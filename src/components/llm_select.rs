use leptos::prelude::*;
use crate::models::LLMModel;

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
            <button 
                class="btn btn-outline w-full justify-between"
                on:click=move |_| set_is_open.update(|open| *open = !*open)
            >
                <span>
                    {move || {
                        llms_signal.get().iter()
                            .find(|llm| llm.id == selected.get())
                            .map(|llm| format!("{} ({})", llm.name, llm.provider))
                            .unwrap_or_else(|| "Seleziona modello".to_string())
                    }}
                </span>
                <i data-lucide="chevron-down" class="h-4 w-4"></i>
            </button>
            
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
                                                set_status_message.set(format!("Modello cambiato a {}", llm_name.clone()));
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