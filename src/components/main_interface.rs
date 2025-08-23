use crate::components::{chat_area::ChatArea, sidebar::Sidebar, status_bar::StatusBar, document_manager_simple::DocumentManagerSimple, sidebar_monitor::SidebarMonitorRight};
use crate::components::ui_primitives::Button;
use crate::state::webllm_state_simple::WebLLMStateProvider;
use crate::state::GraphRAGStateProvider;
use crate::state::KnowledgeStorageContext;
// use crate::features::crm::CRMPanel; // removed floating CRM panel
use crate::storage::ConversationStorage;
use crate::utils::icons::schedule_icon_render;
use crate::utils::storage::StorageUtils;
use crate::state::GraphRAGStateContext;
use crate::graphrag_config::create_graphrag_signals;
use leptos::prelude::*;

#[component]
pub fn MainInterface() -> impl IntoView {
    let (sidebar_collapsed, set_sidebar_collapsed) = signal(false);
    let (monitor_collapsed, set_monitor_collapsed) = signal(true);
    let (selected_llm, _set_selected_llm) = signal("Llama-3.2-1B-Instruct-q4f32_1-MLC".to_string());
    let (knowledge_enabled, set_knowledge_enabled) = signal(false);
    let (status_message, set_status_message) = signal("Ready".to_string());
    
    // Document manager modal state
    let (show_document_manager, set_show_document_manager) = signal(false);

    // Global conversation state
    let (storage, set_storage) = signal::<Option<ConversationStorage>>(None);
    let (current_conversation_id, set_current_conversation_id) = signal::<Option<String>>(None);
    let (conversation_list_refresh, set_conversation_list_refresh) = signal(0u32);

    // GraphRAG configuration and metrics
    let (graphrag_config, graphrag_metrics, graphrag_manager) = create_graphrag_signals();

    // Initialize storage
    Effect::new(move |_| match ConversationStorage::new() {
        Ok(storage_instance) => {
            set_storage.set(Some(storage_instance));
        }
        Err(e) => {
            log::error!("Failed to initialize storage: {:?}", e);
            set_status_message.set("Storage initialization failed".to_string());
        }
    });

    // Effect to re-render Lucide icons when state changes
    Effect::new(move |_| {
        let _ = sidebar_collapsed.get();
        let _ = monitor_collapsed.get();
        schedule_icon_render();
    });

    // Initial icon render
    Effect::new(move |_| {
        schedule_icon_render();
    });

    // Provide shared knowledge storage context at the app root
    provide_context(KnowledgeStorageContext::new());

    // Startup coherence check: if buffer exists and index is empty, prompt to reindex
    let graphrag_ctx = use_context::<GraphRAGStateContext>();
    Effect::new(move |_| {
        // Run once at mount
        let buffer_exists = StorageUtils::retrieve_local::<String>("knowledge_upload_buffer_v1")
            .ok()
            .flatten()
            .map(|s| !s.trim().is_empty())
            .unwrap_or(false);
        // Prefer v1 key for index emptiness; fallback to legacy
        let index_empty = if let Ok(Some(v)) = StorageUtils::retrieve_local::<Vec<crate::models::graphrag::DocumentIndex>>("graphrag_document_index_v1") {
            v.is_empty()
        } else {
            StorageUtils::retrieve_local::<Vec<crate::models::graphrag::DocumentIndex>>("graphrag_document_index")
                .ok()
                .flatten()
                .map(|v| v.is_empty())
                .unwrap_or(true)
        };
        if buffer_exists && index_empty {
            if let Some(win) = web_sys::window() {
                if let Ok(true) = win.confirm_with_message("Detected uploaded documents without an index. Index with GraphRAG now?") {
                    if let Some(ctx) = graphrag_ctx.clone() {
                        ctx.reindex();
                        set_status_message.set("Reindexing with GraphRAG started...".to_string());
                    }
                }
            }
        }
    });

    view! {
        <GraphRAGStateProvider>
        <WebLLMStateProvider>
        <div class="app-scope h-screen flex flex-col bg-base-100 overflow-x-hidden hide-scrollbar">
            <div class="flex flex-1 min-h-0 relative overflow-x-hidden hide-scrollbar">
                <Sidebar
                    collapsed=sidebar_collapsed
                    set_collapsed=set_sidebar_collapsed
                    set_status_message=set_status_message
                    storage=storage
                    current_conversation_id=current_conversation_id
                    set_current_conversation_id=set_current_conversation_id
                    conversation_list_refresh=conversation_list_refresh
                    _set_conversation_list_refresh=set_conversation_list_refresh
                    set_show_document_manager=set_show_document_manager
                />

                // Chat area with floating monitor toggle
                <div class="flex-1 relative min-w-0">
                    <ChatArea
                    knowledge_enabled=knowledge_enabled
                    set_knowledge_enabled=set_knowledge_enabled
                    set_status_message=set_status_message
                    selected_llm=selected_llm
                    graphrag_config=graphrag_config
                    graphrag_metrics=graphrag_metrics
                    graphrag_manager=graphrag_manager.clone()
                    storage=storage
                    current_conversation_id=current_conversation_id
                    set_current_conversation_id=set_current_conversation_id
                    set_conversation_list_refresh=set_conversation_list_refresh
                    />

                    // Open button shown when monitor is collapsed
                    <Show when=move || monitor_collapsed.get()>
                        <div class="absolute right-2 top-2 z-20">
                            <Button
                                label=Signal::derive(|| "".to_string())
                                variant=Signal::derive(|| "btn-ghost btn-square btn-md".to_string())
                                icon=Signal::derive(|| "panel-right".to_string())
                                on_click=Box::new(move || set_monitor_collapsed.set(false))
                            />
                        </div>
                    </Show>
                </div>

                // Right monitoring sidebar
                <SidebarMonitorRight
                    collapsed=monitor_collapsed
                    set_collapsed=set_monitor_collapsed
                    graphrag_config=graphrag_config
                    graphrag_metrics=graphrag_metrics
                    graphrag_manager=graphrag_manager.clone()
                />
            </div>


            <StatusBar
                message=status_message
                selected_llm=selected_llm
                knowledge_enabled=knowledge_enabled
                graphrag_metrics=graphrag_metrics
            />
            


            
            // Document Manager Modal
            <Show when=move || show_document_manager.get()>
                <div class="fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center z-50">
                    <div class="bg-base-100 rounded-lg shadow-xl mx-4 max-h-[90vh] overflow-hidden inline-block w-auto min-w-0 max-w-md sm:max-w-lg md:max-w-2xl modal-fit">
                        <div class="flex justify-between items-center p-4 border-b border-base-300">
                            <h2 class="text-lg font-semibold">"Document Manager"</h2>
                            <button
                                class="btn btn-ghost btn-sm btn-circle"
                                on:click=move |_| set_show_document_manager.set(false)
                            >
                                "✕"
                            </button>
                        </div>
                        <div class="p-4 overflow-y-auto max-h-[calc(90vh-80px)]">
                            <DocumentManagerSimple />
                        </div>
                    </div>
                </div>
            </Show>
        </div>
        </WebLLMStateProvider>
        </GraphRAGStateProvider>
    }
}
