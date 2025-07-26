use leptos::prelude::*;
use leptos::task::spawn_local;
use gloo_timers::future::TimeoutFuture;
use crate::components::{message_bubble::MessageBubble, input_area::InputArea};
use crate::models::{Message, MessageRole};
use crate::webllm_binding::{init_webllm, send_message_to_llm};
use log::info;

#[component]
pub fn ChatArea(
    knowledge_enabled: ReadSignal<bool>,
    set_knowledge_enabled: WriteSignal<bool>,
    set_status_message: WriteSignal<String>,
    selected_llm: ReadSignal<String>,
) -> impl IntoView {
    // Existing state
    let (messages, set_messages) = signal(vec![
        Message {
            id: "1".to_string(),
            role: MessageRole::Assistant,
            content: "Ciao! Sono un assistente AI che funziona completamente nel tuo browser. Come posso aiutarti?".to_string(),
            timestamp: js_sys::Date::now(),
        }
    ]);
    
    let (input_value, set_input_value) = signal(String::new());
    let (is_loading, set_is_loading) = signal(false);
    
    // WebLLM state - using a simple boolean to track readiness
    let (model_ready, set_model_ready) = signal(false);
    
    // Store the engine in a static location using thread_local
    use std::cell::RefCell;
    thread_local! {
        static WEBLLM_ENGINE: RefCell<Option<wasm_bindgen::JsValue>> = const { RefCell::new(None) };
    }
    
    // Initialize WebLLM when component loads or model changes
    Effect::new(move |_| {
        let current_model = selected_llm.get();
        spawn_local(async move {
            set_status_message.set(format!("Caricamento modello {}...", current_model));
            info!("Inizializzazione WebLLM con modello: {}", current_model);
            
            match init_webllm(&current_model).await {
                Ok(engine) => {
                    info!("WebLLM inizializzato con successo con modello: {}", current_model);
                    WEBLLM_ENGINE.with(|e| {
                        *e.borrow_mut() = Some(engine);
                    });
                    set_model_ready.set(true);
                    set_status_message.set(format!("Modello {} pronto", current_model));
                }
                Err(e) => {
                    log::error!("Errore inizializzazione WebLLM: {:?}", e);
                    set_status_message.set("Errore caricamento modello".to_string());
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
        set_messages.update(|msgs| msgs.push(user_message));
        set_input_value.set(String::new());
        set_is_loading.set(true);
        set_status_message.set("AI sta pensando...".to_string());

        if model_ready.get() {
            let current_messages = messages.get();
            
            spawn_local(async move {
                // Get the engine from thread local storage
                let engine_opt = WEBLLM_ENGINE.with(|e| e.borrow().clone());
                
                if let Some(engine) = engine_opt {
                    match send_message_to_llm(&engine, current_messages).await {
                        Ok(response) => {
                            let ai_message = Message::new(MessageRole::Assistant, response);
                            set_messages.update(|msgs| msgs.push(ai_message));
                            set_status_message.set("Pronto".to_string());
                        }
                        Err(e) => {
                            log::error!("Errore risposta AI: {:?}", e);
                            let error_message = Message::new(
                                MessageRole::Assistant, 
                                "Scusa, ho avuto un problema nel rispondere. Riprova.".to_string()
                            );
                            set_messages.update(|msgs| msgs.push(error_message));
                            set_status_message.set("Errore AI".to_string());
                        }
                    }
                } else {
                    let error_message = Message::new(
                        MessageRole::Assistant, 
                        "Il modello AI non è disponibile. Riprova.".to_string()
                    );
                    set_messages.update(|msgs| msgs.push(error_message));
                    set_status_message.set("Modello non disponibile".to_string());
                }
                set_is_loading.set(false);
            });
        } else {
            // Fallback to simulated response if WebLLM is not ready
            spawn_local(async move {
                TimeoutFuture::new(1500).await;
                let ai_message = Message::new(
                    MessageRole::Assistant, 
                    "Il modello AI non è ancora pronto. Riprova tra qualche momento.".to_string()
                );
                set_messages.update(|msgs| msgs.push(ai_message));
                set_is_loading.set(false);
                set_status_message.set("Modello non pronto".to_string());
            });
        }
    };
    
    view! {
        <div class="flex-1 flex flex-col bg-base-100">
            // Model loading status
            <Show when=move || !model_ready.get()>
                <div class="mx-6 mt-4 p-4 bg-info/10 rounded-lg border border-info/20">
                    <div class="flex items-center space-x-3">
                        <div class="loading loading-spinner loading-sm text-info"></div>
                        <div>
                            <p class="text-info font-medium">
                                "Caricamento modello AI in corso..."
                            </p>
                            <p class="text-sm text-base-content/60 mt-1">
                                "Potrebbero essere necessari alcuni minuti al primo avvio"
                            </p>
                        </div>
                    </div>
                </div>
            </Show>

            // Messages area
            <div class="flex-1 overflow-y-auto custom-scrollbar">
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
        </div>
    }
}