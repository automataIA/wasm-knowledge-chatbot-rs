use crate::advanced_graphrag::{HyDEConfig, HyDEEngine};
use crate::components::ui_primitives::{Button, Input, ProgressBar};
use crate::components::{input_area::InputArea, message_bubble::MessageBubble};
use crate::features::graphrag::retrieval::Retriever;
use crate::graphrag_config::{
    GraphRAGConfig, GraphRAGConfigManager, GraphRAGMetrics, PerformanceMetrics,
};
use crate::models::graphrag::RAGQuery;
use crate::models::{Message, MessageMetadata, MessageRole, SourceAttribution};
use crate::storage::ConversationStorage;
use crate::utils::icons::schedule_icon_render;
use crate::utils::storage::StorageUtils;
use crate::webllm_binding::{init_webllm_with_progress, send_message_to_llm};
use gloo_timers::future::TimeoutFuture;
use leptos::prelude::*;
use leptos::task::spawn_local;
use log::info;
use wasm_bindgen::JsCast;

#[component]
pub fn ChatArea(
    knowledge_enabled: ReadSignal<bool>,
    set_knowledge_enabled: WriteSignal<bool>,
    set_status_message: WriteSignal<String>,
    selected_llm: ReadSignal<String>,
    graphrag_config: Signal<GraphRAGConfig>,
    graphrag_metrics: Signal<GraphRAGMetrics>,
    graphrag_manager: GraphRAGConfigManager,
    storage: ReadSignal<Option<ConversationStorage>>,
    current_conversation_id: ReadSignal<Option<String>>,
    set_current_conversation_id: WriteSignal<Option<String>>,
    set_conversation_list_refresh: WriteSignal<u32>,
) -> impl IntoView {
    // Existing state
    let (messages, set_messages) = signal(vec![Message::new(
        MessageRole::Assistant,
        "Hello! I'm an AI assistant that runs completely in your browser. How can I help you?"
            .to_string(),
    )]);

    // Remove local storage state since it's now passed as props

    let (input_value, set_input_value) = signal(String::new());
    let (is_loading, set_is_loading) = signal(false);

    // Menu state
    let (menu_open, set_menu_open) = signal(false);
    let (show_delete_confirm, set_show_delete_confirm) = signal(false);
    let (show_rename_dialog, set_show_rename_dialog) = signal(false);
    let (conversation_title, set_conversation_title) = signal("Chat".to_string());
    let (rename_input, set_rename_input) = signal(String::new());

    // System prompt UI state
    let (_show_edit_global_prompt, set_show_edit_global_prompt) = signal(false);
    let (show_edit_conv_prompt, set_show_edit_conv_prompt) = signal(false);
    let (_global_prompt_input, set_global_prompt_input) = signal(String::new());
    let (conv_prompt_input, set_conv_prompt_input) = signal(String::new());

    // Cached prompts
    let (global_system_prompt, set_global_system_prompt) = signal(Option::<String>::None);
    let (conversation_system_prompt, set_conversation_system_prompt) =
        signal(Option::<String>::None);

    // WebLLM state - using a simple boolean to track readiness
    let (model_ready, set_model_ready) = signal(false);
    let (loading_progress, set_loading_progress) = signal(0.0);
    let (loading_text, set_loading_text) = signal("Initializing...".to_string());

    // Derived percent for ProgressBar primitive (0-100)
    let progress_percent =
        Signal::derive(move || (loading_progress.get() * 100.0_f64).round() as u32);

    // Store the engine in a static location using thread_local
    use std::cell::RefCell;
    thread_local! {
        static WEBLLM_ENGINE: RefCell<Option<wasm_bindgen::JsValue>> = const { RefCell::new(None) };
    }

    // Empty conversation cleanup is now handled in ConversationList

    // Function to load conversation history
    let load_conversation = move |conversation_id: String| {
        if let Some(ref storage) = storage.get() {
            // Load conversation title from the list
            match storage.list_conversations() {
                Ok(conversations) => {
                    if let Some(conv_info) = conversations.iter().find(|c| c.id == conversation_id)
                    {
                        set_conversation_title.set(conv_info.title.clone());
                    }
                }
                Err(e) => {
                    log::error!("Failed to load conversation list: {:?}", e);
                }
            }

            // Load messages
            match storage.load_conversation(&conversation_id) {
                Ok(Some(loaded_messages)) => {
                    info!(
                        "Loaded {} messages from conversation",
                        loaded_messages.len()
                    );
                    set_messages.set(loaded_messages);
                }
                Ok(None) => {
                    info!("No messages found for conversation");
                    // Set default welcome message for empty conversations
                    set_messages.set(vec![
                            Message::new(
                                MessageRole::Assistant,
                                "Hello! I'm an AI assistant that runs completely in your browser. How can I help you?".to_string(),
                            )
                        ]);
                }
                Err(e) => {
                    log::error!("Failed to load conversation: {:?}", e);
                }
            }
        }
    };

    // Ensure icons are rendered when component loads
    Effect::new(move |_| {
        schedule_icon_render();
    });

    // Re-render icons when menu state changes
    Effect::new(move |_| {
        let _ = menu_open.get();
        let _ = show_delete_confirm.get();
        let _ = show_rename_dialog.get();
        schedule_icon_render();
    });

    // Note: Empty conversation cleanup is now handled in ConversationList

    // Load conversation when current_conversation_id changes
    Effect::new(move |_| {
        if let Some(conversation_id) = current_conversation_id.get() {
            load_conversation(conversation_id);
        }
    });

    // Load global prompt once and on demand
    Effect::new(move |_| {
        // Attempt to load from localStorage
        if let Ok(Some(p)) = StorageUtils::retrieve_local::<String>("global_system_prompt") {
            set_global_system_prompt.set(Some(p));
        }
    });

    // Load per-conversation prompt whenever conversation changes
    Effect::new(move |_| {
        if let (Some(ref storage), Some(ref conv_id)) =
            (storage.get(), current_conversation_id.get())
        {
            if let Ok(p) = storage.load_conversation_system_prompt(conv_id) {
                set_conversation_system_prompt.set(p);
            }
        } else {
            set_conversation_system_prompt.set(None);
        }
    });

    // Create initial conversation if none exists
    Effect::new(move |_| {
        if storage.get().is_some() && current_conversation_id.get().is_none() {
            if let Some(ref storage) = storage.get() {
                match storage.create_conversation("New Chat".to_string()) {
                    Ok(conversation_id) => {
                        set_current_conversation_id.set(Some(conversation_id));
                        // Don't refresh conversation list yet - wait for first user message
                        info!("Initial conversation created (not yet in history)");
                    }
                    Err(e) => {
                        log::error!("Failed to create initial conversation: {:?}", e);
                    }
                }
            }
        }
    });

    // Initialize WebLLM when component loads or model changes
    Effect::new(move |_| {
        let current_model = selected_llm.get();
        spawn_local(async move {
            set_model_ready.set(false);
            set_loading_progress.set(0.0);
            set_loading_text.set("Initializing...".to_string());
            set_status_message.set("Loading model...".to_string());
            info!("Initializing WebLLM with model: {}", current_model);

            // Create progress callback
            let progress_callback = move |text: String, progress: f64| {
                set_loading_text.set(text.clone());
                set_loading_progress.set(progress);
                set_status_message.set(format!("{} ({:.1}%)", text, progress * 100.0));
            };

            match init_webllm_with_progress(&current_model, progress_callback).await {
                Ok(engine) => {
                    info!(
                        "WebLLM initialized successfully with model: {}",
                        current_model
                    );
                    WEBLLM_ENGINE.with(|e| {
                        *e.borrow_mut() = Some(engine);
                    });
                    set_model_ready.set(true);
                    set_loading_progress.set(1.0);
                    set_loading_text.set("- Completed".to_string());
                    set_status_message.set("- Ready".to_string());
                }
                Err(e) => {
                    log::error!("WebLLM initialization error: {:?}", e);
                    set_loading_text.set("Error".to_string());
                    set_status_message.set("Model loading error".to_string());
                }
            }
        });
    });

    // Send message function with WebLLM integration
    let send_message_cb: std::rc::Rc<dyn Fn(leptos::ev::MouseEvent) + 'static> =
        std::rc::Rc::new(move |_| {
            let content = input_value.get();
            if content.trim().is_empty() || is_loading.get() || !model_ready.get() {
                return;
            }

            // Process with GraphRAG when knowledge is enabled (internal processing only)
            let cfg = graphrag_config.get();
            let mut perf = PerformanceMetrics::default();

            if knowledge_enabled.get() && cfg.hyde_enabled {
                let t0 = js_sys::Date::now();
                let hyde = HyDEEngine::new(HyDEConfig::default());
                let _hypothetical_docs = hyde.generate_hypothetical_docs(&content);
                // Note: HyDE docs are for internal search enhancement, not sent to LLM
                let t1 = js_sys::Date::now();
                perf.hyde_time_ms = (t1 - t0) as u32;
            }

            // Placeholders for other toggled phases (no-ops for now)
            if knowledge_enabled.get() && cfg.community_detection_enabled {
                // TODO: integrate CommunityDetectionEngine with a real graph implementing GraphAccess
                perf.community_detection_time_ms = 0;
            }
            if knowledge_enabled.get() && cfg.pagerank_enabled {
                // TODO: integrate PageRankEngine::score_nodes with a GraphAccess graph
                perf.pagerank_time_ms = 0;
            }
            if knowledge_enabled.get() && cfg.reranking_enabled {
                // TODO: integrate AdvancedReranker on candidate scores
                perf.reranking_time_ms = 0;
            }
            if knowledge_enabled.get() && cfg.synthesis_enabled {
                // TODO: integrate ResultSynthesizer on top snippets
                perf.synthesis_time_ms = 0;
            }

            let user_message = Message::new(MessageRole::User, content.clone());
            set_messages.update(|msgs| msgs.push(user_message.clone()));
            set_input_value.set(String::new());
            set_is_loading.set(true);
            set_status_message.set("AI is thinking...".to_string());

            // Save user message to storage
            if let (Some(ref storage), Some(ref conv_id)) =
                (storage.get(), current_conversation_id.get())
            {
                if let Err(e) = storage.save_message(conv_id, &user_message) {
                    log::error!("Failed to save user message: {:?}", e);
                } else {
                    // Always refresh the conversation list when a user message is saved
                    // This ensures the conversation appears in history immediately
                    info!("User message saved, refreshing conversation list");
                    set_conversation_list_refresh.update(|n| {
                        let new_value = *n + 1;
                        info!("Updated refresh signal to: {}", new_value);
                        *n = new_value;
                    });
                }
            }

            // Re-render icons for new message
            schedule_icon_render();

            if model_ready.get() {
                let start_ms = js_sys::Date::now();
                let mgr = graphrag_manager.clone();
                let mut perf_local = perf.clone();
                let current_messages = messages.get();
                // Snapshot flags and prompt for async move
                let use_knowledge = knowledge_enabled.get();
                let prompt_text = content.clone();
                let model_id = selected_llm.get();
                // Snapshot prompts for async move (refresh global from localStorage to reflect sidebar edits)
                let global_prompt_snapshot =
                    StorageUtils::retrieve_local::<String>("global_system_prompt")
                        .ok()
                        .flatten()
                        .or_else(|| global_system_prompt.get());
                let conv_prompt_snapshot = conversation_system_prompt.get();
                // Use configured search strategy
                let strategy_to_use = cfg.search_strategy;

                spawn_local(async move {
                    // Get the engine from thread local storage
                    let engine_opt = WEBLLM_ENGINE.with(|e| e.borrow().clone());

                    if let Some(engine) = engine_opt {
                        // Optionally run GraphRAG retrieval and inject system preamble
                        let mut provenance: Option<Vec<SourceAttribution>> = None;
                        // Start with any system prompts (global, per-conversation)
                        let mut sys_msgs: Vec<Message> = Vec::new();
                        if let Some(ref gp) = global_prompt_snapshot {
                            if !gp.trim().is_empty() {
                                sys_msgs.push(Message::new(MessageRole::System, gp.clone()));
                            }
                        }
                        if let Some(ref cp) = conv_prompt_snapshot {
                            if !cp.trim().is_empty() {
                                sys_msgs.push(Message::new(MessageRole::System, cp.clone()));
                            }
                        }

                        let augmented_messages = if use_knowledge {
                            // Build a minimal RAG query from prompt and current toggles
                            let mut q = RAGQuery::new(prompt_text.clone());
                            q.config.max_results = 5;
                            q.config.use_hyde = cfg.hyde_enabled;
                            q.config.use_community_detection = cfg.community_detection_enabled;
                            q.config.use_reranking = cfg.reranking_enabled;

                            let retriever = Retriever::new();
                            let rag_result = retriever.search(&q, strategy_to_use).await;

                            // Compose a short system preamble from summary + top snippets
                            let mut preamble = String::new();
                            if let Some(summary) = rag_result.metadata.summary.clone() {
                                preamble.push_str("Knowledge summary: ");
                                preamble.push_str(&summary);
                                preamble.push_str("\n\n");
                            }
                            if !rag_result.nodes.is_empty() {
                                preamble.push_str("Top snippets:\n");
                                for n in rag_result.nodes.iter().take(3) {
                                    let mut snip = n.content.clone();
                                    if snip.len() > 300 {
                                        snip.truncate(300);
                                    }
                                    preamble.push_str("- ");
                                    preamble.push_str(&snip);
                                    preamble.push('\n');
                                }
                                // Build provenance from top results
                                let mut attrs: Vec<SourceAttribution> = Vec::new();
                                for n in rag_result.nodes.iter().take(5) {
                                    let title = n
                                        .metadata
                                        .source
                                        .clone()
                                        .unwrap_or_else(|| "Untitled source".to_string());
                                    attrs.push(SourceAttribution {
                                        source_id: n.id.clone(),
                                        title,
                                        confidence: n.metadata.confidence,
                                    });
                                }
                                if !attrs.is_empty() {
                                    provenance = Some(attrs);
                                }
                            }

                            let mut aug =
                                Vec::with_capacity(sys_msgs.len() + current_messages.len() + 1);
                            // system prompts first
                            aug.extend(sys_msgs);
                            if !preamble.is_empty() {
                                aug.push(Message::new(MessageRole::System, preamble));
                            }
                            aug.extend(current_messages.clone());
                            aug
                        } else {
                            let mut aug =
                                Vec::with_capacity(sys_msgs.len() + current_messages.len());
                            aug.extend(sys_msgs);
                            aug.extend(current_messages.clone());
                            aug
                        };

                        match send_message_to_llm(&engine, augmented_messages).await {
                            Ok(response) => {
                                let mut ai_message = Message::new(MessageRole::Assistant, response);
                                set_messages.update(|msgs| msgs.push(ai_message.clone()));
                                set_status_message.set("Ready".to_string());
                                // Update GraphRAG metrics (elapsed time, keep memory placeholder)
                                let elapsed = js_sys::Date::now() - start_ms;
                                perf_local.total_time_ms = elapsed as u32;
                                let mem_placeholder = graphrag_metrics.get().memory_usage_mb; // keep as-is
                                mgr.update_query_metrics(elapsed as u32, mem_placeholder);
                                mgr.update_performance_metrics(perf_local.clone());

                                // Attach provenance and metadata to assistant message
                                let md = MessageMetadata {
                                    tokens_used: None,
                                    processing_time_ms: Some(elapsed as u32),
                                    model_used: Some(model_id.clone()),
                                    graphrag_enhanced: use_knowledge,
                                    error: None,
                                    provenance,
                                };
                                ai_message = ai_message.with_metadata(md);

                                // Update the pushed message with metadata
                                set_messages.update(|msgs| {
                                    if let Some(last) = msgs.last_mut() {
                                        *last = ai_message.clone();
                                    }
                                });

                                // Save AI message to storage
                                if let (Some(ref storage), Some(ref conv_id)) =
                                    (storage.get(), current_conversation_id.get())
                                {
                                    if let Err(e) = storage.save_message(conv_id, &ai_message) {
                                        log::error!("Failed to save AI message: {:?}", e);
                                    } else {
                                        set_conversation_list_refresh.update(|n| *n += 1);
                                    }
                                }

                                // Re-render icons for AI response
                                schedule_icon_render();
                            }
                            Err(e) => {
                                log::error!("AI response error: {:?}", e);
                                let error_message = Message::new(
                                    MessageRole::Assistant,
                                    "Sorry, I had a problem responding. Please try again."
                                        .to_string(),
                                );
                                set_messages.update(|msgs| msgs.push(error_message));
                                set_status_message.set("AI Error".to_string());
                                // Record failed attempt time as well
                                let elapsed = js_sys::Date::now() - start_ms;
                                perf_local.total_time_ms = elapsed as u32;
                                let mem_placeholder = graphrag_metrics.get().memory_usage_mb;
                                mgr.update_query_metrics(elapsed as u32, mem_placeholder);
                                mgr.update_performance_metrics(perf_local.clone());
                                // Re-render icons for error message
                                schedule_icon_render();
                            }
                        }
                    } else {
                        let error_message = Message::new(
                            MessageRole::Assistant,
                            "The AI model is not available. Please try again.".to_string(),
                        );
                        set_messages.update(|msgs| msgs.push(error_message));
                        set_status_message.set("Model not available".to_string());
                        // Still record a minimal metric
                        let elapsed = js_sys::Date::now() - start_ms;
                        perf_local.total_time_ms = elapsed as u32;
                        let mem_placeholder = graphrag_metrics.get().memory_usage_mb;
                        mgr.update_query_metrics(elapsed as u32, mem_placeholder);
                        mgr.update_performance_metrics(perf_local.clone());
                        // Re-render icons for error message
                        schedule_icon_render();
                    }
                    set_is_loading.set(false);
                });
            } else {
                // Fallback to simulated response if WebLLM is not ready
                let start_ms = js_sys::Date::now();
                let mgr = graphrag_manager.clone();
                let mut perf_local = perf.clone();
                spawn_local(async move {
                    TimeoutFuture::new(1500).await;
                    let ai_message = Message::new(
                        MessageRole::Assistant,
                        "The AI model is not ready yet. Please try again in a moment.".to_string(),
                    );
                    set_messages.update(|msgs| msgs.push(ai_message));
                    set_is_loading.set(false);
                    set_status_message.set("Model not ready".to_string());
                    // Record simulated elapsed time
                    let elapsed = js_sys::Date::now() - start_ms;
                    perf_local.total_time_ms = elapsed as u32;
                    let mem_placeholder = graphrag_metrics.get().memory_usage_mb;
                    mgr.update_query_metrics(elapsed as u32, mem_placeholder);
                    mgr.update_performance_metrics(perf_local.clone());
                    // Re-render icons for fallback message
                    schedule_icon_render();
                });
            }
        });

    // Show delete confirmation (no-arg)
    let _show_delete_confirmation = move || {
        set_show_delete_confirm.set(true);
        set_menu_open.set(false);
    };

    // Delete conversation function (no-arg)
    let _delete_conversation = move || {
        if let (Some(ref storage), Some(ref conv_id)) =
            (storage.get(), current_conversation_id.get())
        {
            match storage.delete_conversation(conv_id) {
                Ok(_) => {
                    info!("Conversation deleted successfully");
                    set_current_conversation_id.set(None);
                    set_conversation_title.set("Chat".to_string());
                    set_conversation_list_refresh.update(|n| *n += 1);
                    set_status_message.set("Conversation deleted".to_string());

                    // Clear messages and show welcome message
                    set_messages.set(vec![
                        Message::new(
                            MessageRole::Assistant,
                            "Hello! I'm an AI assistant that runs completely in your browser. How can I help you?".to_string(),
                        )
                    ]);
                }
                Err(e) => {
                    log::error!("Failed to delete conversation: {:?}", e);
                    set_status_message.set("Failed to delete conversation".to_string());
                }
            }
        }
        set_show_delete_confirm.set(false);
    };

    // Cancel delete (no-arg)
    let _cancel_delete = move || {
        set_show_delete_confirm.set(false);
    };

    // Show rename dialog (no-arg)
    let _show_rename_conversation = move || {
        set_rename_input.set(conversation_title.get());
        set_show_rename_dialog.set(true);
        set_menu_open.set(false);
    };

    // Rename conversation function (no-arg)
    let rename_conversation = move || {
        let new_title = rename_input.get().trim().to_string();
        if new_title.is_empty() {
            set_status_message.set("Title cannot be empty".to_string());
            return;
        }

        if let (Some(ref storage), Some(ref conv_id)) =
            (storage.get(), current_conversation_id.get())
        {
            match storage.update_conversation_title(conv_id, new_title.clone()) {
                Ok(_) => {
                    // Update local state
                    set_conversation_title.set(new_title.clone());
                    set_status_message.set("Conversation renamed".to_string());

                    // Force a hard refresh of the conversation list
                    spawn_local(async move {
                        // First increment to trigger refresh
                        set_conversation_list_refresh.update(|n| *n += 1);

                        // Small delay to ensure storage is updated
                        gloo_timers::future::TimeoutFuture::new(100).await;

                        // Force a second refresh after a small delay
                        set_conversation_list_refresh.update(|n| *n += 1);

                        // Force a third refresh after another small delay
                        gloo_timers::future::TimeoutFuture::new(50).await;
                        set_conversation_list_refresh.update(|n| *n += 1);
                    });
                }
                Err(e) => {
                    log::error!("Failed to rename conversation: {:?}", e);
                    set_status_message.set("Failed to rename conversation".to_string());
                }
            }
        }
        set_show_rename_dialog.set(false);
    };

    // Cancel rename (no-arg)
    let cancel_rename = move || {
        set_show_rename_dialog.set(false);
    };

    // Show global prompt editor
    let _show_edit_global = move || {
        // Prefill with current
        set_global_prompt_input.set(global_system_prompt.get().unwrap_or_default());
        set_show_edit_global_prompt.set(true);
        set_menu_open.set(false);
    };

    // Show per-conversation prompt editor
    let _show_edit_conv = move || {
        set_conv_prompt_input.set(conversation_system_prompt.get().unwrap_or_default());
        set_show_edit_conv_prompt.set(true);
        set_menu_open.set(false);
    };

    // Save handlers inlined in on_click below

    // Handle Enter key in rename input
    let handle_rename_keydown = move |ev: leptos::ev::KeyboardEvent| {
        if ev.key() == "Enter" {
            let new_title = rename_input.get().trim().to_string();
            if new_title.is_empty() {
                set_status_message.set("Title cannot be empty".to_string());
                return;
            }

            if let (Some(ref storage), Some(ref conv_id)) =
                (storage.get(), current_conversation_id.get())
            {
                match storage.update_conversation_title(conv_id, new_title.clone()) {
                    Ok(_) => {
                        info!("Conversation renamed successfully to: {}", new_title);
                        set_conversation_title.set(new_title);
                        set_conversation_list_refresh.update(|n| {
                            let new_value = *n + 1;
                            info!("Updated refresh signal after rename to: {}", new_value);
                            *n = new_value;
                        });
                        set_status_message.set("Conversation renamed".to_string());
                    }
                    Err(e) => {
                        log::error!("Failed to rename conversation: {:?}", e);
                        set_status_message.set("Failed to rename conversation".to_string());
                    }
                }
            }
            set_show_rename_dialog.set(false);
        } else if ev.key() == "Escape" {
            set_show_rename_dialog.set(false);
        }
    };

    // Save as markdown function (no-arg)
    let _save_as_markdown = move || {
        let current_messages = messages.get();
        if current_messages.is_empty() {
            set_status_message.set("No messages to save".to_string());
            set_menu_open.set(false);
            return;
        }

        // Generate markdown content
        let mut markdown_content = String::new();
        let title = conversation_title.get();
        markdown_content.push_str(&format!("# {}\n\n", title));

        // Add export timestamp
        let export_date = js_sys::Date::new(&wasm_bindgen::JsValue::from(js_sys::Date::now()));
        let export_timestamp = export_date.to_locale_string("en-US", &js_sys::Object::new());
        markdown_content.push_str(&format!(
            "*Exported on: {}*\n\n---\n\n",
            export_timestamp.as_string().unwrap_or_default()
        ));

        for message in current_messages {
            let role = match message.role {
                MessageRole::User => "## 👤 You",
                MessageRole::Assistant => "## 🤖 Assistant",
                MessageRole::System => "## ⚙️ System",
            };

            // Format timestamp
            let date = js_sys::Date::new(&wasm_bindgen::JsValue::from(message.timestamp));
            let formatted_date = date.to_locale_string("en-US", &js_sys::Object::new());
            let timestamp = formatted_date.as_string().unwrap_or_default();

            markdown_content.push_str(&format!(
                "{}\n*{}*\n\n{}\n\n",
                role, timestamp, message.content
            ));
        }

        // Create and download the file
        let blob_parts = js_sys::Array::new();
        blob_parts.push(&wasm_bindgen::JsValue::from_str(&markdown_content));

        match web_sys::Blob::new_with_str_sequence(&blob_parts) {
            Ok(blob) => {
                match web_sys::Url::create_object_url_with_blob(&blob) {
                    Ok(url) => {
                        // Create download link
                        if let Some(window) = web_sys::window() {
                            if let Some(document) = window.document() {
                                if let Ok(link) = document.create_element("a") {
                                    if let Ok(link) = link.dyn_into::<web_sys::HtmlAnchorElement>()
                                    {
                                        link.set_href(&url);
                                        // Create a safe filename from the conversation title
                                        let safe_title = conversation_title
                                            .get()
                                            .chars()
                                            .map(|c| if c.is_alphanumeric() { c } else { '_' })
                                            .collect::<String>();
                                        let filename = format!("{}.md", safe_title);
                                        link.set_download(&filename);
                                        link.click();

                                        // Clean up
                                        let _ = web_sys::Url::revoke_object_url(&url);

                                        set_status_message
                                            .set("Conversation saved as markdown".to_string());
                                    }
                                }
                            }
                        }
                    }
                    Err(e) => {
                        log::error!("Failed to create object URL: {:?}", e);
                        set_status_message.set("Failed to save conversation".to_string());
                    }
                }
            }
            Err(e) => {
                log::error!("Failed to create blob: {:?}", e);
                set_status_message.set("Failed to save conversation".to_string());
            }
        }

        set_menu_open.set(false);
    };

    // Toggle menu function (no-arg for Button callbacks)
    let _toggle_menu = move || {
        set_menu_open.update(|open| *open = !*open);
    };

    // Close menu when clicking outside (no-arg; use inline adapter for HTML event)
    let close_menu = move || {
        set_menu_open.set(false);
    };

    view! {
        <div class="flex-1 flex flex-col bg-base-100">
            // Chat content

            // Header with hamburger on the left and title
            <div class="relative z-20 overflow-visible px-4 py-2 border-b border-base-300 flex items-center gap-3 select-none">
                <div class="relative">
                    <Button
                        label=Signal::derive(|| "".to_string())
                        variant=Signal::derive(|| "btn-ghost btn-sm btn-square".to_string())
                        icon=Signal::derive(|| "menu".to_string())
                        on_click=Box::new({
                            move || {
                                set_menu_open.update(|open| *open = !*open);
                            }
                        })
                    />
                    <Show when=move || menu_open.get()>
                        <div class="absolute left-0 top-full mt-2 z-50 w-64 bg-base-100 border border-base-200 rounded-md shadow-xl p-2 pointer-events-auto chat-menu">
                            <div class="flex flex-col gap-1">
                                <Button
                                    label=Signal::derive(|| "Local Prompt".to_string())
                                    variant=Signal::derive(|| "btn-ghost w-full justify-start text-left whitespace-nowrap gap-2".to_string())
                                    icon=Signal::derive(|| "settings".to_string())
                                    on_click=Box::new({
                                        move || {
                                            set_conv_prompt_input.set(conversation_system_prompt.get().unwrap_or_default());
                                            set_show_edit_conv_prompt.set(true);
                                            set_menu_open.set(false);
                                        }
                                    })
                                />
                                <Button
                                    label=Signal::derive(|| "Rename Conversation".to_string())
                                    variant=Signal::derive(|| "btn-ghost w-full justify-start text-left whitespace-nowrap".to_string())
                                    icon=Signal::derive(|| "edit-3".to_string())
                                    on_click=Box::new({
                                        move || {
                                            set_rename_input.set(conversation_title.get());
                                            set_show_rename_dialog.set(true);
                                            set_menu_open.set(false);
                                        }
                                    })
                                />
                                <Button
                                    label=Signal::derive(|| "Save as Markdown".to_string())
                                    variant=Signal::derive(|| "btn-ghost w-full justify-start text-left whitespace-nowrap".to_string())
                                    icon=Signal::derive(|| "download".to_string())
                                    on_click=Box::new({
                                        move || {
                                            let current_messages = messages.get();
                                            if current_messages.is_empty() {
                                                set_status_message.set("No messages to save".to_string());
                                                set_menu_open.set(false);
                                            } else {
                                                // Existing save logic placeholder
                                            }
                                        }
                                    })
                                />
                                <Button
                                    label=Signal::derive(|| "Delete Conversation".to_string())
                                    variant=Signal::derive(|| "btn-ghost w-full justify-start text-left whitespace-nowrap text-error".to_string())
                                    icon=Signal::derive(|| "trash-2".to_string())
                                    on_click=Box::new({
                                        move || {
                                            set_show_delete_confirm.set(true);
                                            set_menu_open.set(false);
                                        }
                                    })
                                />
                            </div>
                        </div>
                    </Show>
                </div>
                <div class="font-semibold truncate" title=move || conversation_title.get()>
                    {move || conversation_title.get()}
                </div>
            </div>

        // Messages area
        <div class="flex-1 overflow-y-auto custom-scrollbar" on:click=move |_| close_menu()>
            <div class="h-full flex flex-col">
                <div class="flex-1 px-6 py-8">
                    <div class="max-w-4xl mx-auto w-full space-y-4">
                        <For
                            each=messages
                            key=|msg| msg.id.clone()
                            children=move |msg| view! { <MessageBubble message=msg /> }
                        />

                        // Loading indicator
                        <Show when=move || is_loading.get()>
                            <div class="flex justify-start">
                                <div class="bg-base-200 rounded-lg p-3 max-w-xs">
                                    <div class="flex space-x-1">
                                        <div class="w-2 h-2 bg-primary rounded-full animate-bounce"></div>
                                        <div
                                            class="w-2 h-2 bg-primary rounded-full animate-bounce"
                                            style="animation-delay: 0.1s"
                                        ></div>
                                        <div
                                            class="w-2 h-2 bg-primary rounded-full animate-bounce"
                                            style="animation-delay: 0.2s"
                                        ></div>
                                    </div>
                                </div>
                            </div>
                        </Show>

            // Global system prompt modal removed from ChatArea (moved to Sidebar)

            // Per-conversation system prompt modal (opened from burger menu)
            <Show when=move || show_edit_conv_prompt.get()>
                <div class="fixed inset-0 bg-black/50 flex items-center justify-center z-50">
                    <div class="bg-base-100 rounded-lg p-6 max-w-2xl w-full mx-4 shadow-xl">
                        <h3 class="text-lg font-semibold mb-4">"Edit Conversation System Prompt"</h3>
                        <div class="mb-4">
                            <label class="block text-sm font-medium text-base-content/70 mb-2">"Prompt"</label>
                            <textarea
                                class="textarea textarea-bordered w-full min-h-[160px]"
                                prop:value=move || conv_prompt_input.get()
                                on:input=move |ev| set_conv_prompt_input.set(event_target_value(&ev))
                            ></textarea>
                        </div>
                        <div class="flex gap-3 justify-end">
                            <Button
                                label=Signal::derive(|| "Cancel".to_string())
                                variant=Signal::derive(|| "btn-ghost".to_string())
                                on_click=Box::new({
                                    let set_show = set_show_edit_conv_prompt;
                                    move || set_show.set(false)
                                })
                            />
                            {
                                let can_save = Signal::derive(move || !conv_prompt_input.get().trim().is_empty());
                                view! {
                                    <Button
                                        label=Signal::derive(|| "Save".to_string())
                                        variant=Signal::derive(|| "btn-primary".to_string())
                                        disabled=Signal::derive(move || !can_save.get())
                                        on_click=Box::new({
                                            let set_show = set_show_edit_conv_prompt;
                                            move || {
                                                if let (Some(ref storage), Some(ref conv_id)) = (storage.get(), current_conversation_id.get()) {
                                                    let text = conv_prompt_input.get();
                                                    let _ = storage.update_conversation_system_prompt(conv_id, Some(text.clone()));
                                                    set_conversation_system_prompt.set(Some(text));
                                                    set_status_message.set("Conversation prompt saved".to_string());
                                                }
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
                </div>
            </div>

            // Model loading status
            <Show when=move || !model_ready.get()>
                <div class="mx-6 mt-4 p-4 bg-info/10 rounded-lg border border-info/20">
                    <div class="space-y-3">
                        <div class="flex items-center space-x-3">
                            <div class="loading loading-spinner loading-sm text-info"></div>
                            <div class="flex-1">
                                <p class="text-info font-medium">{move || loading_text.get()}</p>
                                <p class="text-sm text-base-content/60 mt-1">
                                    "This may take a few minutes on first startup"
                                </p>
                            </div>
                            <div class="text-info font-mono text-sm">
                                {move || {
                                    format!(
                                        "{}%",
                                        (loading_progress.get() * 100.0_f64).round() as u32,
                                    )
                                }}
                            </div>
                        </div>

                        // Progress bar
                        <ProgressBar
                            value=progress_percent
                            max=100u32
                            color=Signal::derive(|| "progress-info".to_string())
                        />
                    </div>
                </div>
            </Show>

            // Rename conversation modal
            <Show when=move || show_rename_dialog.get()>
                <div class="fixed inset-0 bg-black/50 flex items-center justify-center z-50">
                    <div class="bg-base-100 rounded-lg p-6 max-w-md w-full mx-4 shadow-xl">
                        <h3 class="text-lg font-semibold mb-4">"Rename Conversation"</h3>
                        <div class="mb-6">
                            <label class="block text-sm font-medium text-base-content/70 mb-2">
                                "Conversation Title"
                            </label>
                            <Input
                                value=rename_input
                                set_value=set_rename_input
                                placeholder=Signal::derive(|| "Enter new title...".to_string())
                                on_keypress=Box::new(handle_rename_keydown)
                            />
                        </div>
                        <div class="flex gap-3 justify-end">
                            <Button
                                label=Signal::derive(|| "Cancel".to_string())
                                variant=Signal::derive(|| "btn-ghost".to_string())
                                on_click=Box::new(cancel_rename)
                            />
                            {
                                let disabled_sig = Signal::derive(move || {
                                    rename_input.get().trim().is_empty()
                                });
                                view! {
                                    <Button
                                        label=Signal::derive(|| "Rename".to_string())
                                        variant=Signal::derive(|| "btn-primary".to_string())
                                        disabled=disabled_sig
                                        on_click=Box::new(rename_conversation)
                                    />
                                }
                            }
                        </div>
                    </div>
                </div>
            </Show>

            // Input area
            <div class="border-t border-base-300 p-2">
                <InputArea
                    input_value=input_value
                    set_input_value=set_input_value
                    on_send={send_message_cb.clone()}
                    knowledge_enabled=knowledge_enabled
                    set_knowledge_enabled=set_knowledge_enabled
                    is_loading=is_loading
                    set_status_message=set_status_message
                />
            </div>
        </div>
        </div>
    }
}
