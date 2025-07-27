use leptos::prelude::*;
use crate::components::{sidebar::Sidebar, chat_area::ChatArea, status_bar::StatusBar};
use crate::utils::icons::schedule_icon_render;
use crate::storage::ConversationStorage;

#[component]
pub fn MainInterface() -> impl IntoView {
    let (sidebar_collapsed, set_sidebar_collapsed) = signal(false);
    let (selected_llm, set_selected_llm) = signal("Llama-3.2-1B-Instruct-q4f32_1-MLC".to_string());
    let (knowledge_enabled, set_knowledge_enabled) = signal(false);
    let (status_message, set_status_message) = signal("Ready".to_string());
    
    // Global conversation state
    let (storage, set_storage) = signal::<Option<ConversationStorage>>(None);
    let (current_conversation_id, set_current_conversation_id) = signal::<Option<String>>(None);
    let (conversation_list_refresh, set_conversation_list_refresh) = signal(0u32);
    
    // Initialize storage
    Effect::new(move |_| {
        match ConversationStorage::new() {
            Ok(storage_instance) => {
                set_storage.set(Some(storage_instance));
            }
            Err(e) => {
                log::error!("Failed to initialize storage: {:?}", e);
                set_status_message.set("Storage initialization failed".to_string());
            }
        }
    });

    // Effect to re-render Lucide icons when state changes
    Effect::new(move |_| {
        let _ = sidebar_collapsed.get();
        schedule_icon_render();
    });

    // Initial icon render
    Effect::new(move |_| {
        schedule_icon_render();
    });

    view! {
        <div class="h-screen flex flex-col bg-base-100">
            <div class="flex h-full">
                <Sidebar 
                    collapsed=sidebar_collapsed
                    set_collapsed=set_sidebar_collapsed
                    selected_llm=selected_llm
                    set_selected_llm=set_selected_llm
                    set_status_message=set_status_message
                    storage=storage
                    current_conversation_id=current_conversation_id
                    set_current_conversation_id=set_current_conversation_id
                    conversation_list_refresh=conversation_list_refresh
                    _set_conversation_list_refresh=set_conversation_list_refresh
                />
                
                <ChatArea 
                    knowledge_enabled=knowledge_enabled
                    set_knowledge_enabled=set_knowledge_enabled
                    set_status_message=set_status_message
                    selected_llm=selected_llm
                    storage=storage
                    current_conversation_id=current_conversation_id
                    set_current_conversation_id=set_current_conversation_id
                    set_conversation_list_refresh=set_conversation_list_refresh
                />
            </div>
            
            <StatusBar 
                message=status_message 
                selected_llm=selected_llm
                knowledge_enabled=knowledge_enabled
            />
        </div>
    }
}
