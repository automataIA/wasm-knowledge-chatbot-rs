use leptos::prelude::*;
use crate::components::{llm_select::LLMSelect, sidebar_action::SidebarAction, conversation_history::ConversationHistory};
use crate::models::LLMModel;

#[component]
pub fn Sidebar(
    collapsed: ReadSignal<bool>,
    set_collapsed: WriteSignal<bool>,
    selected_llm: ReadSignal<String>,
    set_selected_llm: WriteSignal<String>,
    set_status_message: WriteSignal<String>,
) -> impl IntoView {
    let llms = vec![
        // Llama 3.2 Models
        LLMModel {
            id: "Llama-3.2-1B-Instruct-q4f32_1-MLC".to_string(),
            name: "Llama 3.2 1B".to_string(),
            provider: "Meta".to_string(),
            logo_slug: "meta-color".to_string(),
        },
        LLMModel {
            id: "Llama-3.2-3B-Instruct-q4f32_1-MLC".to_string(),
            name: "Llama 3.2 3B".to_string(),
            provider: "Meta".to_string(),
            logo_slug: "meta-color".to_string(),
        },
        // Llama 3.1 Models
        LLMModel {
            id: "Llama-3.1-8B-Instruct-q4f32_1-MLC".to_string(),
            name: "Llama 3.1 8B".to_string(),
            provider: "Meta".to_string(),
            logo_slug: "meta-color".to_string(),
        },
        LLMModel {
            id: "Llama-3.1-8B-Instruct-q4f16_1-MLC".to_string(),
            name: "Llama 3.1 8B (16-bit)".to_string(),
            provider: "Meta".to_string(),
            logo_slug: "meta-color".to_string(),
        },
        // Hermes Models
        LLMModel {
            id: "Hermes-3-Llama-3.1-8B-q4f32_1-MLC".to_string(),
            name: "Hermes 3 Llama 3.1 8B".to_string(),
            provider: "NousResearch".to_string(),
            logo_slug: "huggingface".to_string(),
        },
        LLMModel {
            id: "Hermes-2-Pro-Llama-3-8B-q4f32_1-MLC".to_string(),
            name: "Hermes 2 Pro Llama 3 8B".to_string(),
            provider: "NousResearch".to_string(),
            logo_slug: "huggingface".to_string(),
        },
        // Phi Models
        LLMModel {
            id: "Phi-3.5-mini-instruct-q4f32_1-MLC".to_string(),
            name: "Phi 3.5 Mini".to_string(),
            provider: "Microsoft".to_string(),
            logo_slug: "microsoft".to_string(),
        },
        LLMModel {
            id: "Phi-3.5-vision-instruct-q4f32_1-MLC".to_string(),
            name: "Phi 3.5 Vision".to_string(),
            provider: "Microsoft".to_string(),
            logo_slug: "microsoft".to_string(),
        },
        // Mistral Models
        LLMModel {
            id: "Mistral-7B-Instruct-v0.3-q4f32_1-MLC".to_string(),
            name: "Mistral 7B v0.3".to_string(),
            provider: "Mistral AI".to_string(),
            logo_slug: "mistral".to_string(),
        },
        LLMModel {
            id: "Hermes-2-Pro-Mistral-7B-q4f16_1-MLC".to_string(),
            name: "Hermes 2 Pro Mistral 7B".to_string(),
            provider: "NousResearch".to_string(),
            logo_slug: "huggingface".to_string(),
        },
        // Gemma Models
        LLMModel {
            id: "gemma-2-2b-it-q4f32_1-MLC".to_string(),
            name: "Gemma 2 2B".to_string(),
            provider: "Google".to_string(),
            logo_slug: "google".to_string(),
        },
        LLMModel {
            id: "gemma-2-9b-it-q4f32_1-MLC".to_string(),
            name: "Gemma 2 9B".to_string(),
            provider: "Google".to_string(),
            logo_slug: "google".to_string(),
        },
        // Qwen Models
        LLMModel {
            id: "Qwen2.5-0.5B-Instruct-q4f32_1-MLC".to_string(),
            name: "Qwen 2.5 0.5B".to_string(),
            provider: "Alibaba".to_string(),
            logo_slug: "alibaba".to_string(),
        },
        LLMModel {
            id: "Qwen2.5-1.5B-Instruct-q4f32_1-MLC".to_string(),
            name: "Qwen 2.5 1.5B".to_string(),
            provider: "Alibaba".to_string(),
            logo_slug: "alibaba".to_string(),
        },
        LLMModel {
            id: "Qwen3-1.7B-q4f32_1-MLC".to_string(),
            name: "Qwen 3 1.7B".to_string(),
            provider: "Alibaba".to_string(),
            logo_slug: "alibaba".to_string(),
        },
        // SmolLM Models
        LLMModel {
            id: "SmolLM2-360M-Instruct-q4f32_1-MLC".to_string(),
            name: "SmolLM2 360M".to_string(),
            provider: "HuggingFace".to_string(),
            logo_slug: "huggingface".to_string(),
        },
        LLMModel {
            id: "SmolLM2-1.7B-Instruct-q4f32_1-MLC".to_string(),
            name: "SmolLM2 1.7B".to_string(),
            provider: "HuggingFace".to_string(),
            logo_slug: "huggingface".to_string(),
        },
        // DeepSeek Models
        LLMModel {
            id: "DeepSeek-R1-Distill-Llama-8B-q4f32_1-MLC".to_string(),
            name: "DeepSeek R1 Distill Llama 8B".to_string(),
            provider: "DeepSeek".to_string(),
            logo_slug: "deepseek".to_string(),
        },
        LLMModel {
            id: "DeepSeek-R1-Distill-Qwen-7B-q4f32_1-MLC".to_string(),
            name: "DeepSeek R1 Distill Qwen 7B".to_string(),
            provider: "DeepSeek".to_string(),
            logo_slug: "deepseek".to_string(),
        },
    ];

    view! {
        <div class=move || format!(
            "flex flex-col border-r border-base-300 bg-base-200 transition-all duration-300 {}",
            if collapsed.get() { "w-16" } else { "w-80" }
        )>
            
            // Header with controls
            <div class="flex flex-col gap-2 p-4 border-b border-base-300">
                <div class="flex justify-start gap-2">
                    <button 
                        class="btn btn-ghost btn-sm btn-square"
                        on:click=move |_| set_collapsed.update(|c| *c = !*c)
                    >
                        <i data-lucide=move || if collapsed.get() { "panel-left" } else { "panel-left-close" }
                           class="h-4 w-4"></i>
                    </button>
                </div>
                
                // LLM Selection
                <Show when=move || !collapsed.get()>
                    <div class="flex flex-col gap-1">
                        <label class="text-xs text-base-content/70">"Modello LLM"</label>
                        <LLMSelect 
                            selected=selected_llm
                            set_selected=set_selected_llm
                            llms=llms.clone()
                            set_status_message=set_status_message
                        />
                    </div>
                </Show>
            </div>
            
            // Actions
            <div class="flex flex-col gap-2 p-4">
                <SidebarAction 
                    icon="file-text"
                    label="Carica Markdown"
                    collapsed=collapsed
                />
                <SidebarAction 
                    icon="network"
                    label="Genera Knowledge Graph"
                    collapsed=collapsed
                />
                <SidebarAction 
                    icon="plus"
                    label="Nuova Chat"
                    collapsed=collapsed
                />
            </div>
            
            // Conversation history
            <Show when=move || !collapsed.get()>
                <div class="border-t border-base-300"></div>
                <ConversationHistory />
            </Show>
        </div>
    }
}