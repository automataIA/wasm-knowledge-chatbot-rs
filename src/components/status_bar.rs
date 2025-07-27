use leptos::prelude::*;

#[component]
pub fn StatusBar(
    message: ReadSignal<String>,
    selected_llm: ReadSignal<String>,
    knowledge_enabled: ReadSignal<bool>,
) -> impl IntoView {
    // Extract model info from selected_llm
    let model_info = move || {
        let model_id = selected_llm.get();
        if model_id.contains("Llama-3.2-1B") {
            ("Llama 3.2 1B", "~1.1GB", "Meta")
        } else if model_id.contains("Llama-3.2-3B") {
            ("Llama 3.2 3B", "~3.0GB", "Meta")
        } else if model_id.contains("Llama-3.1-8B") {
            ("Llama 3.1 8B", "~6.1GB", "Meta")
        } else if model_id.contains("Phi-3.5-mini") {
            ("Phi 3.5 Mini", "~5.5GB", "Microsoft")
        } else if model_id.contains("Gemma-2-2b") {
            ("Gemma 2 2B", "~2.5GB", "Google")
        } else if model_id.contains("Gemma-2-9b") {
            ("Gemma 2 9B", "~8.4GB", "Google")
        } else if model_id.contains("Mistral-7B") {
            ("Mistral 7B", "~5.6GB", "Mistral")
        } else if model_id.contains("Qwen2.5-0.5B") {
            ("Qwen 2.5 0.5B", "~1.0GB", "Alibaba")
        } else if model_id.contains("Qwen2.5-1.5B") {
            ("Qwen 2.5 1.5B", "~1.6GB", "Alibaba")
        } else if model_id.contains("SmolLM2-360M") {
            ("SmolLM2 360M", "~580MB", "HF")
        } else if model_id.contains("SmolLM2-1.7B") {
            ("SmolLM2 1.7B", "~2.7GB", "HF")
        } else if model_id.contains("Hermes") {
            ("Hermes", "~6.0GB", "Nous")
        } else if model_id.contains("DeepSeek") {
            ("DeepSeek R1", "~6.0GB", "DeepSeek")
        } else {
            ("Unknown", "~?GB", "Unknown")
        }
    };
    
    // Determine status based on message content
    let status_info = move || {
        let msg = message.get();
        if msg.contains("ready") || msg == "Ready" {
            ("success", "READY")
        } else if msg.contains("Error") || msg.contains("error") {
            ("error", "ERROR")
        } else if msg.contains("Loading") || msg.contains("loading") || msg.contains("Download") {
            ("warning", "LOADING")
        } else if msg.contains("thinking") {
            ("info", "THINKING")
        } else {
            ("warning", "INIT")
        }
    };
    view! {
        <div class="bg-base-200 border-t border-base-300 px-4 py-2">
            <div class="flex items-center justify-between text-xs text-base-content/70">
                <div class="flex items-center gap-4">
                    // Model info
                    <div class="flex items-center gap-1">
                        <div class="w-2 h-2 bg-primary rounded-full"></div>
                        <span class="font-medium">{move || model_info().0}</span>
                        <span class="text-base-content/50">
                            "(" {move || model_info().2} ")"
                        </span>
                    </div>
                    
                    // Knowledge Graph status
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
                    
                    // Status message with dynamic phases
                    <div class="flex items-center gap-2">
                        <div class=move || {
                            let (status_type, _) = status_info();
                            format!(
                                "w-2 h-2 rounded-full {}",
                                match status_type {
                                    "success" => "bg-success",
                                    "error" => "bg-error",
                                    "warning" => "bg-warning",
                                    "info" => "bg-info",
                                    _ => "bg-base-content/30",
                                }
                            )
                        }></div>
                        <span class="font-medium">{message}</span>
                    </div>
                </div>
                
                <div class="flex items-center gap-4">
                    // Runtime info
                    <div class="flex items-center gap-1">
                        <div class="w-2 h-2 bg-warning rounded-full"></div>
                        <span>"WASM"</span>
                    </div>
                    
                    // Memory usage
                    <div class="flex items-center gap-1">
                        <div class="w-2 h-2 bg-info rounded-full"></div>
                        <span>{move || model_info().1}</span>
                    </div>
                    
                    // Status indicator
                    <div class="flex items-center gap-1">
                        <div class=move || {
                            let (status_type, _) = status_info();
                            format!(
                                "w-2 h-2 rounded-full {}",
                                match status_type {
                                    "success" => "bg-success",
                                    "error" => "bg-error",
                                    "warning" => "bg-warning animate-pulse",
                                    "info" => "bg-info animate-pulse",
                                    _ => "bg-base-content/30",
                                }
                            )
                        }></div>
                        <span class="font-mono">
                            {move || status_info().1}
                        </span>
                    </div>
                </div>
            </div>
        </div>
    }
}
