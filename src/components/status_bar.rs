use leptos::prelude::*;

#[component]
pub fn StatusBar(
    message: ReadSignal<String>,
    selected_llm: ReadSignal<String>,
    knowledge_enabled: ReadSignal<bool>,
) -> impl IntoView {
    view! {
        <div class="bg-base-200 border-t border-base-300 px-4 py-2">
            <div class="flex items-center justify-between text-xs text-base-content/70">
                <div class="flex items-center gap-4">
                    <div class="flex items-center gap-1">
                        <div class="w-2 h-2 bg-primary rounded-full"></div>
                        <span>{move || selected_llm.get().to_uppercase()}</span>
                    </div>
                    <div class="flex items-center gap-1">
                        <div class=move || {
                            format!(
                                "w-2 h-2 rounded-full {}",
                                if knowledge_enabled.get() {
                                    "bg-success"
                                } else {
                                    "bg-base-content/30"
                                },
                            )
                        }></div>
                        <span>
                            {move || if knowledge_enabled.get() { "KG ON" } else { "KG OFF" }}
                        </span>
                    </div>
                    <span class="ml-2">{message}</span>
                </div>
                <div class="flex items-center gap-4">
                    <div class="flex items-center gap-1">
                        <div class="w-2 h-2 bg-warning rounded-full"></div>
                        <span>"WASM"</span>
                    </div>
                    <div class="flex items-center gap-1">
                        <div class="w-2 h-2 bg-info rounded-full"></div>
                        <span>"8MB"</span>
                    </div>
                    <div class="flex items-center gap-1">
                        <div class="w-2 h-2 bg-success rounded-full"></div>
                        <span>"23%"</span>
                    </div>
                </div>
            </div>
        </div>
    }
}
