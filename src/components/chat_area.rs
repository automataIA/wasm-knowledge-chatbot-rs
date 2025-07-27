use leptos::prelude::*;
use leptos::task::spawn_local;
use gloo_timers::future::TimeoutFuture;
use wasm_bindgen::JsCast;
use crate::components::{message_bubble::MessageBubble, input_area::InputArea};
use crate::models::{Message, MessageRole};
use crate::webllm_binding::{init_webllm_with_progress, send_message_to_llm};
use crate::utils::icons::schedule_icon_render;
use crate::storage::ConversationStorage;
use log::info;


#[component]
pub fn ChatArea(
    knowledge_enabled: ReadSignal<bool>,
    set_knowledge_enabled: WriteSignal<bool>,
    set_status_message: WriteSignal<String>,
    selected_llm: ReadSignal<String>,
    storage: ReadSignal<Option<ConversationStorage>>,
    current_conversation_id: ReadSignal<Option<String>>,
    set_current_conversation_id: WriteSignal<Option<String>>,
    set_conversation_list_refresh: WriteSignal<u32>,
) -> impl IntoView {
    // Existing state
    let (messages, set_messages) = signal(vec![
        Message {
            id: "1".to_string(),
            role: MessageRole::Assistant,
            content: "Hello! I'm an AI assistant that runs completely in your browser. How can I help you?".to_string(),
            timestamp: js_sys::Date::now(),
        }
    ]);
    
    // Remove local storage state since it's now passed as props
    
    let (input_value, set_input_value) = signal(String::new());
    let (is_loading, set_is_loading) = signal(false);
    
    // Menu state
    let (menu_open, set_menu_open) = signal(false);
    let (show_delete_confirm, set_show_delete_confirm) = signal(false);
    let (show_rename_dialog, set_show_rename_dialog) = signal(false);
    let (conversation_title, set_conversation_title) = signal("Chat".to_string());
    let (rename_input, set_rename_input) = signal(String::new());
    
    // WebLLM state - using a simple boolean to track readiness
    let (model_ready, set_model_ready) = signal(false);
    let (loading_progress, set_loading_progress) = signal(0.0);
    let (loading_text, set_loading_text) = signal("Initializing...".to_string());
    
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
                        if let Some(conv_info) = conversations.iter().find(|c| c.id == conversation_id) {
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
                        info!("Loaded {} messages from conversation", loaded_messages.len());
                        set_messages.set(loaded_messages);
                    }
                    Ok(None) => {
                        info!("No messages found for conversation");
                        // Set default welcome message for empty conversations
                        set_messages.set(vec![
                            Message {
                                id: "1".to_string(),
                                role: MessageRole::Assistant,
                                content: "Hello! I'm an AI assistant that runs completely in your browser. How can I help you?".to_string(),
                                timestamp: js_sys::Date::now(),
                            }
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
            set_status_message.set(format!("Loading model {}...", current_model));
            info!("Initializing WebLLM with model: {}", current_model);
            
            // Create progress callback
            let progress_callback = move |text: String, progress: f64| {
                    set_loading_text.set(text.clone());
                    set_loading_progress.set(progress);
                    set_status_message.set(format!("{} ({:.1}%)", text, progress * 100.0));
                };
            
            match init_webllm_with_progress(&current_model, progress_callback).await {
                Ok(engine) => {
                    info!("WebLLM initialized successfully with model: {}", current_model);
                    WEBLLM_ENGINE.with(|e| {
                        *e.borrow_mut() = Some(engine);
                    });
                    set_model_ready.set(true);
                    set_loading_progress.set(1.0);
                    set_loading_text.set("Completed".to_string());
                    set_status_message.set(format!("Model {} ready", current_model));
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
    let send_message = move |_| {
        let content = input_value.get();
        if content.trim().is_empty() || is_loading.get() || !model_ready.get() {
            return;
        }

        let user_message = Message::new(MessageRole::User, content.clone());
        set_messages.update(|msgs| msgs.push(user_message.clone()));
        set_input_value.set(String::new());
        set_is_loading.set(true);
        set_status_message.set("AI is thinking...".to_string());
        
        // Save user message to storage
        if let (Some(ref storage), Some(ref conv_id)) = (storage.get(), current_conversation_id.get()) {
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
            let current_messages = messages.get();
            
            spawn_local(async move {
                // Get the engine from thread local storage
                let engine_opt = WEBLLM_ENGINE.with(|e| e.borrow().clone());
                
                if let Some(engine) = engine_opt {
                    match send_message_to_llm(&engine, current_messages).await {
                        Ok(response) => {
                            let ai_message = Message::new(MessageRole::Assistant, response);
                            set_messages.update(|msgs| msgs.push(ai_message.clone()));
                            set_status_message.set("Ready".to_string());
                            
                            // Save AI message to storage
                            if let (Some(ref storage), Some(ref conv_id)) = (storage.get(), current_conversation_id.get()) {
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
                                "Sorry, I had a problem responding. Please try again.".to_string()
                            );
                            set_messages.update(|msgs| msgs.push(error_message));
                            set_status_message.set("AI Error".to_string());
                            // Re-render icons for error message
                            schedule_icon_render();
                        }
                    }
                } else {
                    let error_message = Message::new(
                        MessageRole::Assistant, 
                        "The AI model is not available. Please try again.".to_string()
                    );
                    set_messages.update(|msgs| msgs.push(error_message));
                    set_status_message.set("Model not available".to_string());
                    // Re-render icons for error message
                    schedule_icon_render();
                }
                set_is_loading.set(false);
            });
        } else {
            // Fallback to simulated response if WebLLM is not ready
            spawn_local(async move {
                TimeoutFuture::new(1500).await;
                let ai_message = Message::new(
                    MessageRole::Assistant, 
                    "The AI model is not ready yet. Please try again in a moment.".to_string()
                );
                set_messages.update(|msgs| msgs.push(ai_message));
                set_is_loading.set(false);
                set_status_message.set("Model not ready".to_string());
                // Re-render icons for fallback message
                schedule_icon_render();
            });
        }
    };
    
    // Show delete confirmation
    let show_delete_confirmation = move |_| {
        set_show_delete_confirm.set(true);
        set_menu_open.set(false);
    };
    
    // Delete conversation function
    let delete_conversation = move |_| {
        if let (Some(ref storage), Some(ref conv_id)) = (storage.get(), current_conversation_id.get()) {
            match storage.delete_conversation(conv_id) {
                Ok(_) => {
                    info!("Conversation deleted successfully");
                    set_current_conversation_id.set(None);
                    set_conversation_title.set("Chat".to_string());
                    set_conversation_list_refresh.update(|n| *n += 1);
                    set_status_message.set("Conversation deleted".to_string());
                    
                    // Clear messages and show welcome message
                    set_messages.set(vec![
                        Message {
                            id: "1".to_string(),
                            role: MessageRole::Assistant,
                            content: "Hello! I'm an AI assistant that runs completely in your browser. How can I help you?".to_string(),
                            timestamp: js_sys::Date::now(),
                        }
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
    
    // Cancel delete
    let cancel_delete = move |_| {
        set_show_delete_confirm.set(false);
    };
    
    // Show rename dialog
    let show_rename_conversation = move |_| {
        set_rename_input.set(conversation_title.get());
        set_show_rename_dialog.set(true);
        set_menu_open.set(false);
    };
    
    // Rename conversation function
    let rename_conversation = move |_| {
        let new_title = rename_input.get().trim().to_string();
        if new_title.is_empty() {
            set_status_message.set("Title cannot be empty".to_string());
            return;
        }

        if let (Some(ref storage), Some(ref conv_id)) = (storage.get(), current_conversation_id.get()) {
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
    
    // Cancel rename
    let cancel_rename = move |_| {
        set_show_rename_dialog.set(false);
    };
    
    // Handle Enter key in rename input
    let handle_rename_keydown = move |ev: leptos::ev::KeyboardEvent| {
        if ev.key() == "Enter" {
            let new_title = rename_input.get().trim().to_string();
            if new_title.is_empty() {
                set_status_message.set("Title cannot be empty".to_string());
                return;
            }
            
            if let (Some(ref storage), Some(ref conv_id)) = (storage.get(), current_conversation_id.get()) {
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
    
    // Save as markdown function
    let save_as_markdown = move |_| {
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
        markdown_content.push_str(&format!("*Exported on: {}*\n\n---\n\n", export_timestamp.as_string().unwrap_or_default()));
        
        for message in current_messages {
            let role = match message.role {
                MessageRole::User => "## 👤 You",
                MessageRole::Assistant => "## 🤖 Assistant",
            };
            
            // Format timestamp
            let date = js_sys::Date::new(&wasm_bindgen::JsValue::from(message.timestamp));
            let formatted_date = date.to_locale_string("en-US", &js_sys::Object::new());
            let timestamp = formatted_date.as_string().unwrap_or_default();
            
            markdown_content.push_str(&format!("{}\n*{}*\n\n{}\n\n", role, timestamp, message.content));
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
                                    if let Ok(link) = link.dyn_into::<web_sys::HtmlAnchorElement>() {
                                        link.set_href(&url);
                                        // Create a safe filename from the conversation title
                                        let safe_title = conversation_title.get()
                                            .chars()
                                            .map(|c| if c.is_alphanumeric() || c == ' ' || c == '-' || c == '_' { c } else { '_' })
                                            .collect::<String>()
                                            .trim()
                                            .replace(' ', "_");
                                        let filename = if safe_title.is_empty() { 
                                            "chat-conversation.md".to_string() 
                                        } else { 
                                            format!("{}.md", safe_title) 
                                        };
                                        link.set_download(&filename);
                                        link.click();
                                        
                                        // Clean up
                                        let _ = web_sys::Url::revoke_object_url(&url);
                                        
                                        set_status_message.set("Conversation saved as markdown".to_string());
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
    
    // Toggle menu function
    let toggle_menu = move |_| {
        set_menu_open.update(|open| *open = !*open);
    };
    
    // Close menu when clicking outside
    let close_menu = move |_| {
        set_menu_open.set(false);
    };
    
    view! {
        <div class="flex-1 flex flex-col bg-base-100">
            // Chat header with menu
            <Show when=move || current_conversation_id.get().is_some()>
                <div class="flex items-center px-6 py-3 border-b border-base-300">
                    // Meatball menu (moved to left)
                    <div class="relative mr-3">
                        <button 
                            class="btn btn-ghost btn-sm btn-circle"
                            on:click=toggle_menu
                        >
                            <i data-lucide="more-vertical" class="h-4 w-4"></i>
                        </button>
                        
                        // Dropdown menu
                        <Show when=move || menu_open.get()>
                            <div class="absolute left-0 top-full mt-1 w-52 bg-base-100 rounded-lg shadow-lg border border-base-300 z-50 min-w-max animate-in fade-in duration-200">
                                <div class="py-1">
                                    <button 
                                        class="flex items-center gap-2 w-full px-4 py-2 text-sm text-base-content hover:bg-base-200 transition-colors"
                                        on:click=show_rename_conversation
                                    >
                                        <i data-lucide="edit-3" class="h-4 w-4"></i>
                                        "Rename Conversation"
                                    </button>
                                    <button 
                                        class="flex items-center gap-2 w-full px-4 py-2 text-sm text-base-content hover:bg-base-200 transition-colors"
                                        on:click=save_as_markdown
                                    >
                                        <i data-lucide="download" class="h-4 w-4"></i>
                                        "Save as Markdown"
                                    </button>
                                    <div class="border-t border-base-300 my-1"></div>
                                    <button 
                                        class="flex items-center gap-2 w-full px-4 py-2 text-sm text-error hover:bg-error/10 transition-colors"
                                        on:click=show_delete_confirmation
                                    >
                                        <i data-lucide="trash-2" class="h-4 w-4"></i>
                                        "Delete Conversation"
                                    </button>
                                </div>
                            </div>
                        </Show>
                    </div>
                    
                    <h2 class="text-lg font-semibold text-base-content truncate flex-1">{move || conversation_title.get()}</h2>
                </div>
            </Show>
            
            // Model loading status
            <Show when=move || !model_ready.get()>
                <div class="mx-6 mt-4 p-4 bg-info/10 rounded-lg border border-info/20">
                    <div class="space-y-3">
                        <div class="flex items-center space-x-3">
                            <div class="loading loading-spinner loading-sm text-info"></div>
                            <div class="flex-1">
                                <p class="text-info font-medium">
                                    {move || loading_text.get()}
                                </p>
                                <p class="text-sm text-base-content/60 mt-1">
                                    "This may take a few minutes on first startup"
                                </p>
                            </div>
                            <div class="text-info font-mono text-sm">
                                {move || format!("{:.1}%", loading_progress.get() * 100.0)}
                            </div>
                        </div>
                        
                        // Progress bar
                        <div class="w-full bg-base-300 rounded-full h-2">
                            <div 
                                class="bg-info h-2 rounded-full transition-all duration-300 ease-out"
                                style=move || format!("width: {:.1}%", loading_progress.get() * 100.0)
                            ></div>
                        </div>
                    </div>
                </div>
            </Show>

            // Messages area
            <div class="flex-1 overflow-y-auto custom-scrollbar" on:click=close_menu>
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
                        </div>
                    </div>
                </div>
            </div>

            // Input area
            <InputArea
                input_value=input_value
                set_input_value=set_input_value
                on_send=send_message
                knowledge_enabled=knowledge_enabled
                set_knowledge_enabled=set_knowledge_enabled
                is_loading=is_loading
                set_status_message=set_status_message
            />
            
            // Delete confirmation modal
            <Show when=move || show_delete_confirm.get()>
                <div class="fixed inset-0 bg-black/50 flex items-center justify-center z-50">
                    <div class="bg-base-100 rounded-lg p-6 max-w-md w-full mx-4 shadow-xl">
                        <h3 class="text-lg font-semibold mb-4">"Delete Conversation"</h3>
                        <p class="text-base-content/70 mb-6">
                            "Are you sure you want to delete this conversation? This action cannot be undone."
                        </p>
                        <div class="flex gap-3 justify-end">
                            <button 
                                class="btn btn-ghost"
                                on:click=cancel_delete
                            >
                                "Cancel"
                            </button>
                            <button 
                                class="btn btn-error"
                                on:click=delete_conversation
                            >
                                "Delete"
                            </button>
                        </div>
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
                            <input 
                                type="text"
                                class="input input-bordered w-full focus:input-primary"
                                placeholder="Enter new title..."
                                prop:value=rename_input
                                on:input=move |ev| set_rename_input.set(event_target_value(&ev))
                                on:keydown=handle_rename_keydown
                                autofocus
                                maxlength="100"
                            />
                        </div>
                        <div class="flex gap-3 justify-end">
                            <button 
                                class="btn btn-ghost"
                                on:click=cancel_rename
                            >
                                "Cancel"
                            </button>
                            <button 
                                class="btn btn-primary"
                                class:btn-disabled=move || rename_input.get().trim().is_empty()
                                on:click=rename_conversation
                            >
                                "Rename"
                            </button>
                        </div>
                    </div>
                </div>
            </Show>
        </div>
    }
}