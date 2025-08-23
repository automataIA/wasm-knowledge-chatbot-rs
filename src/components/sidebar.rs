use crate::components::ui_primitives::Button;
use crate::components::{
    conversation_list::ConversationList, sidebar_action::SidebarAction, theme_toggle::ThemeToggle,
};
use crate::features::webllm::ui::WebLLMInitPanel;
use crate::models::{webllm::ModelCapability, LLMModel};
use crate::utils::storage::StorageUtils;
use leptos::prelude::*;

#[component]
pub fn Sidebar(
    collapsed: ReadSignal<bool>,
    set_collapsed: WriteSignal<bool>,
    set_status_message: WriteSignal<String>,
    storage: ReadSignal<Option<crate::storage::ConversationStorage>>,
    current_conversation_id: ReadSignal<Option<String>>,
    set_current_conversation_id: WriteSignal<Option<String>>,
    conversation_list_refresh: ReadSignal<u32>,
    _set_conversation_list_refresh: WriteSignal<u32>,
    set_show_document_manager: WriteSignal<bool>,
) -> impl IntoView {
    // Global prompt modal state
    let (show_edit_global_prompt, set_show_edit_global_prompt) = signal(false);
    let (global_prompt_input, set_global_prompt_input) = signal(String::new());

    // Open global prompt editor
    let open_global_prompt = move || {
        if let Ok(Some(p)) = StorageUtils::retrieve_local::<String>("global_system_prompt") {
            set_global_prompt_input.set(p);
        } else {
            set_global_prompt_input.set(String::new());
        }
        set_show_edit_global_prompt.set(true);
    };
    let _llms = vec![
        // Llama 3.2 Models
        LLMModel::new(
            "Llama-3.2-1B-Instruct-q4f32_1-MLC".to_string(),
            "Llama 3.2 1B".to_string(),
            "Meta".to_string(),
            "meta-color".to_string(),
        )
        .with_size(1000)
        .with_context_length(8192)
        .with_capabilities(vec![ModelCapability::TextGeneration]),
        LLMModel::new(
            "Llama-3.2-3B-Instruct-q4f32_1-MLC".to_string(),
            "Llama 3.2 3B".to_string(),
            "Meta".to_string(),
            "meta-color".to_string(),
        )
        .with_size(3000)
        .with_context_length(8192)
        .with_capabilities(vec![ModelCapability::TextGeneration]),
        // Llama 3.1 Models
        LLMModel::new(
            "Llama-3.1-8B-Instruct-q4f32_1-MLC".to_string(),
            "Llama 3.1 8B".to_string(),
            "Meta".to_string(),
            "meta-color".to_string(),
        )
        .with_size(8000)
        .with_context_length(32768)
        .with_capabilities(vec![ModelCapability::TextGeneration]),
        LLMModel::new(
            "Llama-3.1-8B-Instruct-q4f16_1-MLC".to_string(),
            "Llama 3.1 8B (16-bit)".to_string(),
            "Meta".to_string(),
            "meta-color".to_string(),
        )
        .with_size(8000)
        .with_context_length(32768)
        .with_capabilities(vec![ModelCapability::TextGeneration]),
        // Hermes Models
        LLMModel::new(
            "Hermes-3-Llama-3.1-8B-q4f32_1-MLC".to_string(),
            "Hermes 3 Llama 3.1 8B".to_string(),
            "NousResearch".to_string(),
            "huggingface".to_string(),
        )
        .with_size(8000)
        .with_context_length(32768)
        .with_capabilities(vec![ModelCapability::TextGeneration]),
        LLMModel::new(
            "Hermes-2-Pro-Llama-3-8B-q4f32_1-MLC".to_string(),
            "Hermes 2 Pro Llama 3 8B".to_string(),
            "NousResearch".to_string(),
            "huggingface".to_string(),
        )
        .with_size(8000)
        .with_context_length(8192)
        .with_capabilities(vec![ModelCapability::TextGeneration]),
        // Phi Models
        LLMModel::new(
            "Phi-3.5-mini-instruct-q4f32_1-MLC".to_string(),
            "Phi 3.5 Mini".to_string(),
            "Microsoft".to_string(),
            "microsoft".to_string(),
        )
        .with_size(3800)
        .with_context_length(128000)
        .with_capabilities(vec![ModelCapability::TextGeneration]),
        LLMModel::new(
            "Phi-3.5-vision-instruct-q4f32_1-MLC".to_string(),
            "Phi 3.5 Vision".to_string(),
            "Microsoft".to_string(),
            "microsoft".to_string(),
        )
        .with_size(4200)
        .with_context_length(128000)
        .with_capabilities(vec![
            ModelCapability::TextGeneration,
            ModelCapability::VisionUnderstanding,
        ]),
        // Mistral Models
        LLMModel::new(
            "Mistral-7B-Instruct-v0.3-q4f32_1-MLC".to_string(),
            "Mistral 7B v0.3".to_string(),
            "Mistral AI".to_string(),
            "mistral".to_string(),
        )
        .with_size(7000)
        .with_context_length(32768)
        .with_capabilities(vec![ModelCapability::TextGeneration]),
        LLMModel::new(
            "Hermes-2-Pro-Mistral-7B-q4f16_1-MLC".to_string(),
            "Hermes 2 Pro Mistral 7B".to_string(),
            "NousResearch".to_string(),
            "huggingface".to_string(),
        )
        .with_size(7000)
        .with_context_length(8192)
        .with_capabilities(vec![ModelCapability::TextGeneration]),
        // Gemma Models
        LLMModel::new(
            "gemma-2-2b-it-q4f32_1-MLC".to_string(),
            "Gemma 2 2B".to_string(),
            "Google".to_string(),
            "google".to_string(),
        )
        .with_size(2000)
        .with_context_length(8192)
        .with_capabilities(vec![ModelCapability::TextGeneration]),
        LLMModel::new(
            "gemma-2-9b-it-q4f32_1-MLC".to_string(),
            "Gemma 2 9B".to_string(),
            "Google".to_string(),
            "google".to_string(),
        )
        .with_size(9000)
        .with_context_length(8192)
        .with_capabilities(vec![ModelCapability::TextGeneration]),
        // Qwen Models
        LLMModel::new(
            "Qwen2.5-0.5B-Instruct-q4f32_1-MLC".to_string(),
            "Qwen 2.5 0.5B".to_string(),
            "Alibaba".to_string(),
            "alibaba".to_string(),
        )
        .with_size(500)
        .with_context_length(32768)
        .with_capabilities(vec![ModelCapability::TextGeneration]),
        LLMModel::new(
            "Qwen2.5-1.5B-Instruct-q4f32_1-MLC".to_string(),
            "Qwen 2.5 1.5B".to_string(),
            "Alibaba".to_string(),
            "alibaba".to_string(),
        )
        .with_size(1500)
        .with_context_length(32768)
        .with_capabilities(vec![ModelCapability::TextGeneration]),
        LLMModel::new(
            "Qwen3-1.7B-q4f32_1-MLC".to_string(),
            "Qwen 3 1.7B".to_string(),
            "Alibaba".to_string(),
            "alibaba".to_string(),
        )
        .with_size(1700)
        .with_context_length(32768)
        .with_capabilities(vec![ModelCapability::TextGeneration]),
        // SmolLM Models
        LLMModel::new(
            "SmolLM2-360M-Instruct-q4f32_1-MLC".to_string(),
            "SmolLM2 360M".to_string(),
            "HuggingFace".to_string(),
            "huggingface".to_string(),
        )
        .with_size(360)
        .with_context_length(8192)
        .with_capabilities(vec![ModelCapability::TextGeneration]),
        LLMModel::new(
            "SmolLM2-1.7B-Instruct-q4f32_1-MLC".to_string(),
            "SmolLM2 1.7B".to_string(),
            "HuggingFace".to_string(),
            "huggingface".to_string(),
        )
        .with_size(1700)
        .with_context_length(8192)
        .with_capabilities(vec![ModelCapability::TextGeneration]),
        // DeepSeek Models
        LLMModel::new(
            "DeepSeek-R1-Distill-Llama-8B-q4f32_1-MLC".to_string(),
            "DeepSeek R1 Distill Llama 8B".to_string(),
            "DeepSeek".to_string(),
            "deepseek".to_string(),
        )
        .with_size(8000)
        .with_context_length(32768)
        .with_capabilities(vec![ModelCapability::TextGeneration]),
        LLMModel::new(
            "DeepSeek-R1-Distill-Qwen-7B-q4f32_1-MLC".to_string(),
            "DeepSeek R1 Distill Qwen 7B".to_string(),
            "DeepSeek".to_string(),
            "deepseek".to_string(),
        )
        .with_size(7000)
        .with_context_length(32768)
        .with_capabilities(vec![ModelCapability::TextGeneration]),
    ];

    // New chat handler
    let create_new_chat = move |_| {
        if let Some(ref storage) = storage.get() {
            match storage.create_conversation("New Chat".to_string()) {
                Ok(conversation_id) => {
                    set_current_conversation_id.set(Some(conversation_id));
                    // Don't refresh conversation list yet - wait for first user message
                    log::info!("Created new conversation (not yet in history)");
                }
                Err(e) => {
                    log::error!("Failed to create conversation: {:?}", e);
                    set_status_message.set("Failed to create new chat".to_string());
                }
            }
        }
    };

    // Conversation selection handler
    let on_conversation_select = move |conversation_id: String| {
        set_current_conversation_id.set(Some(conversation_id));
        log::info!("Selected conversation");
    };

    view! {
        <div class=move || {
            let width = if collapsed.get() { "w-16" } else { "w-80" };
            let collapsed_cls = if collapsed.get() { "sidebar-collapsed" } else { "" };
            format!(
                "sidebar-panel {} flex flex-col border-r border-base-300 bg-base-200 transition-all duration-300 {}",
                collapsed_cls,
                width,
            )
        }>

            // Header with controls
            <div class=move || {
                if collapsed.get() {
                    "flex flex-col items-center gap-2 p-2 border-b border-base-300".to_string()
                } else {
                    "flex flex-col gap-2 p-4 border-b border-base-300".to_string()
                }
            }>
                <div class=move || {
                    if collapsed.get() {
                        "flex flex-col items-center gap-2 w-full".to_string()
                    } else {
                        "flex items-center gap-2 w-full justify-between".to_string()
                    }
                }>
                    <div class="flex items-center gap-2">
                        <Button
                            label=Signal::derive(|| "".to_string())
                            variant=Signal::derive(move || if collapsed.get() { "btn-ghost btn-lg btn-square".to_string() } else { "btn-ghost btn-md btn-square".to_string() })
                            icon=Signal::derive(move || {
                                if collapsed.get() {
                                    "panel-left".to_string()
                                } else {
                                    "panel-left-close".to_string()
                                }
                            })
                            on_click=Box::new(move || set_collapsed.update(|c| *c = !*c))
                        />
                    </div>
                    // Theme toggle: inline when expanded, centered when collapsed
                    <div class=move || if collapsed.get() { "".to_string() } else { "ml-auto".to_string() }><ThemeToggle /></div>
                </div>

                // LLM Selection (embedded WebLLM init panel)
                <Show when=move || !collapsed.get()>
                    <div class="flex flex-col gap-2">
                        <label class="text-xs text-base-content/70">"LLM Model"</label>
                        <WebLLMInitPanel />
                    </div>
                </Show>
            </div>

            // Actions
            <div class="flex flex-col gap-2 p-4">
                <SidebarAction
                    icon="settings"
                    label="Global Prompt"
                    collapsed=collapsed
                    on_click=Box::new(open_global_prompt)
                />
                <SidebarAction
                    icon="file-text"
                    label="Load Markdown"
                    collapsed=collapsed
                    on_click=Box::new(move || set_show_document_manager.set(true))
                />

                <Button
                    label=Signal::derive(move || {
                        if collapsed.get() { "".to_string() } else { "New Chat".to_string() }
                    })
                    variant=Signal::derive(|| "btn-ghost justify-start w-full".to_string())
                    icon=Signal::derive(|| "plus".to_string())
                    icon_position=Signal::derive(|| "left".to_string())
                    on_click=Box::new(move || create_new_chat(()))
                />
            </div>

            // Conversation history
            <Show when=move || !collapsed.get()>
                <div class="border-t border-base-300"></div>
                <div class="flex-1 overflow-y-auto">
                    <ConversationList
                        storage=storage
                        on_conversation_select=on_conversation_select
                        refresh_signal=conversation_list_refresh
                        current_conversation_id=current_conversation_id
                    />
                </div>
            </Show>

            // Global system prompt modal
            <Show when=move || show_edit_global_prompt.get()>
                <div class="fixed inset-0 bg-black/50 flex items-center justify-center z-50">
                    <div class="bg-base-100 rounded-lg p-6 max-w-2xl w-full mx-4 shadow-xl">
                        <h3 class="text-lg font-semibold mb-4">"Edit Global System Prompt"</h3>
                        <div class="mb-4">
                            <label class="block text-sm font-medium text-base-content/70 mb-2">"Prompt"</label>
                            <textarea
                                class="textarea textarea-bordered w-full min-h-[160px]"
                                prop:value=move || global_prompt_input.get()
                                on:input=move |ev| set_global_prompt_input.set(event_target_value(&ev))
                            ></textarea>
                        </div>
                        <div class="flex gap-3 justify-end">
                            <Button
                                label=Signal::derive(|| "Cancel".to_string())
                                variant=Signal::derive(|| "btn-ghost".to_string())
                                on_click=Box::new({
                                    let set_show = set_show_edit_global_prompt;
                                    move || set_show.set(false)
                                })
                            />
                            {
                                let can_save = Signal::derive(move || !global_prompt_input.get().trim().is_empty());
                                view! {
                                    <Button
                                        label=Signal::derive(|| "Save".to_string())
                                        variant=Signal::derive(|| "btn-primary".to_string())
                                        disabled=Signal::derive(move || !can_save.get())
                                        on_click=Box::new({
                                            let set_show = set_show_edit_global_prompt;
                                            move || {
                                                let text = global_prompt_input.get();
                                                let _ = StorageUtils::store_local("global_system_prompt", &text);
                                                set_status_message.set("Global prompt saved".to_string());
                                                set_show.set(false);
                                            }
                                        })
                                    />
                                }
                            }
                        </div>
                    </div>
                </div>
            </Show>

        </div>
    }
}
