use leptos::prelude::*;
use leptos::task::spawn_local;
use gloo_timers::future::TimeoutFuture;
use web_sys::window;
use crate::features::graphrag::GraphRAGPipeline;
use crate::graphrag_config::{GraphRAGMetrics, with_graphrag_manager};
use crate::utils::storage::StorageUtils;
use crate::models::graphrag::DocumentIndex;
use crate::state::webllm_state_simple::use_webllm_state;
use crate::models::webllm::ModelStatus;

#[component]
pub fn StatusBar(
    message: ReadSignal<String>,
    selected_llm: ReadSignal<String>,
    knowledge_enabled: ReadSignal<bool>,
    #[prop(optional)] graphrag_metrics: Option<Signal<GraphRAGMetrics>>,
) -> impl IntoView {
    // Touch optional metrics to avoid unused warning without changing behavior
    let _ = &graphrag_metrics;
    // We no longer use the selected_llm fallback; silence unused param warning
    let _ = &selected_llm;
    // Single source of truth for model info: WebLLM context current model id; fallback to selected_llm
    let wl_ctx = use_webllm_state();
    let wl_ctx_for_model = wl_ctx.clone();
    let model_info = Memo::new(move |_| -> Option<(String, String, String)> {
        wl_ctx_for_model.get_current_model().map(|m| {
            let size_str = m
                .size_mb
                .map(|mb| {
                    if mb >= 1024 {
                        format!("~{:.1}GB", (mb as f32) / 1024.0)
                    } else {
                        format!("~{}MB", mb)
                    }
                })
                .unwrap_or_else(|| "~?GB".to_string());
            (m.name.clone(), size_str, m.provider.clone())
        })
    });

    // Determine status from WebLLM state (single source of truth) using a memo to allow multiple reads
    let wl_ctx_for_status = wl_ctx.clone();
    let status_kind = Memo::new(move |_| -> (&'static str, &'static str) {
        match wl_ctx_for_status.get_model_status() {
            ModelStatus::Ready => ("success", "READY"),
            ModelStatus::Downloading { .. } | ModelStatus::Loading { .. } => ("warning", "LOADING"),
            ModelStatus::Error { .. } => ("error", "ERROR"),
            ModelStatus::NotInitialized => ("warning", "INIT"),
        }
    });

    // Reactive document count with periodic refresh to capture same-tab localStorage updates
    let (doc_count_state, set_doc_count_state) = signal(0usize);
    // Docs modal state and data
    let (show_docs_modal, set_show_docs_modal) = signal(false);
    let (docs, set_docs) = signal::<Vec<DocumentIndex>>(Vec::new());
    let (doc_filter, set_doc_filter) = signal(String::new());

    // Helper to compute count from storage
    let read_doc_count = || -> usize {
        if let Ok(Some(v)) = StorageUtils::retrieve_local::<Vec<DocumentIndex>>("graphrag_document_index_v1") {
            v.len()
        } else {
            StorageUtils::retrieve_local::<Vec<DocumentIndex>>("graphrag_document_index")
                .ok()
                .flatten()
                .map(|v| v.len())
                .unwrap_or(0)
        }
    };

    // Helper to load full docs list
    let read_docs = || -> Vec<DocumentIndex> {
        if let Ok(Some(v)) = StorageUtils::retrieve_local::<Vec<DocumentIndex>>("graphrag_document_index_v1") {
            v
        } else {
            StorageUtils::retrieve_local::<Vec<DocumentIndex>>("graphrag_document_index")
                .ok()
                .flatten()
                .unwrap_or_default()
        }
    };

    // Initial read and tie to status message updates
    Effect::new(move |_| {
        let _ = message.get();
        set_doc_count_state.set(read_doc_count());
    });

    // Lightweight polling loop (same-tab storage changes don't fire 'storage' event)
    Effect::new(move |_| {
        spawn_local(async move {
            // Poll every 1s while component is alive
            loop {
                TimeoutFuture::new(1000).await;
                set_doc_count_state.set(read_doc_count());
            }
        });
    });
    // Derived filtered docs
    let filtered_docs = Signal::derive({
        move || {
            let q = doc_filter.get().to_lowercase();
            if q.is_empty() { return docs.get(); }
            docs.get().into_iter().filter(|d| {
                d.title.to_lowercase().contains(&q)
                    || d.file_type.to_lowercase().contains(&q)
                    || d.id.to_lowercase().contains(&q)
            }).collect()
        }
    });

    view! {
        <div class="bg-base-200 border-t border-base-300 px-4 py-2 min-w-0 overflow-x-clip">
            <div class="flex items-center justify-between text-xs text-base-content/70 min-w-0">
                <div class="flex items-center gap-4 min-w-0">
                    // GraphRAG status indicator (dot + label)
                    <div class="flex items-center gap-1 min-w-0">
                        <div
                            class=move || {
                                format!(
                                    "w-2 h-2 rounded-full {}",
                                    if knowledge_enabled.get() {
                                        "bg-success"
                                    } else {
                                        "bg-base-content/30"
                                    },
                                )
                            }
                            title=move || {
                                if knowledge_enabled.get() {
                                    "GraphRAG active".to_string()
                                } else {
                                    "GraphRAG inactive".to_string()
                                }
                            }
                        ></div>
                        <span class="font-medium">GraphRAG</span>
                    </div>

                    // Model status LED + Model name (moved from right) + message
                    <div class="flex items-center gap-2 min-w-0">
                        <div class=move || {
                            let (status_type, _) = status_kind.get();
                            format!(
                                "w-2 h-2 rounded-full {}",
                                match status_type {
                                    "success" => "bg-success",
                                    "error" => "bg-error",
                                    "warning" => "bg-warning",
                                    "info" => "bg-info",
                                    _ => "bg-base-content/30",
                                },
                            )
                        }></div>
                        // Dynamic model label (was on the right; now next to the LED)
                        <span
                            class="truncate max-w-[200px]"
                            title=move || {
                                match model_info.get() {
                                    Some((_, _, provider)) => format!("Provider: {}", provider),
                                    None => "Select a model".to_string(),
                                }
                            }
                        >
                            {move || match model_info.get() {
                                Some((name, _, _)) => name,
                                None => "Select model".to_string(),
                            }}
                        </span>
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
                        <span>{move || {
                            match model_info.get() {
                                Some((_, size, _)) => size,
                                None => "~?GB".to_string(),
                            }
                        }}</span>
                    </div>

                    // Document count (indexed) - clickable to open modal
                    <button
                        class="flex items-center gap-1 hover:underline cursor-pointer min-w-0"
                        title="View indexed documents"
                        on:click=move |_| {
                            set_docs.set(read_docs());
                            set_show_docs_modal.set(true);
                        }
                    >
                        <div class="w-2 h-2 bg-base-content/50 rounded-full"></div>
                        <span class="font-mono">
                            {move || format!("Docs: {}", doc_count_state.get())}
                        </span>
                    </button>

                    // Hybrid Fusion time badge (from global manager performance metrics)
                    <div class="flex items-center gap-1">
                        <div class="w-2 h-2 bg-success rounded-full"></div>
                        <span class="font-mono" title="Hybrid fusion time">
                            {move || {
                                let mut ms: u32 = 0;
                                with_graphrag_manager(|m| {
                                    ms = m.get_performance_metrics().hybrid_fusion_time_ms;
                                });
                                format!("Hybrid Fusion: {}ms", ms)
                            }}
                        </span>
                    </div>

                    // Status indicator
                    <div class="flex items-center gap-1">
                        <div class=move || {
                            let (status_type, _) = status_kind.get();
                            format!(
                                "w-2 h-2 rounded-full {}",
                                match status_type {
                                    "success" => "bg-success",
                                    "error" => "bg-error",
                                    "warning" => "bg-warning animate-pulse",
                                    "info" => "bg-info animate-pulse",
                                    _ => "bg-base-content/30",
                                },
                            )
                        }></div>
                        <span class="font-mono">{move || status_kind.get().1}</span>
                    </div>
                </div>
            </div>
        </div>

        // Documents Modal
        <Show when=move || show_docs_modal.get()>
            <div class="fixed inset-0 z-50 flex items-center justify-center">
                <div
                    class="absolute inset-0 bg-black/40"
                    on:click=move |_| set_show_docs_modal.set(false)
                ></div>
                <div class="relative bg-base-100 rounded-lg shadow-xl border border-base-300">
                    <div class="flex items-center justify-between px-4 py-3 border-b border-base-300">
                        <h3 class="font-semibold text-base">Indexed Documents</h3>
                        <button
                            class="btn btn-ghost btn-sm"
                            on:click=move |_| set_show_docs_modal.set(false)
                        >
                            Close
                        </button>
                    </div>
                    <div class="p-3 border-b border-base-300">
                        <input
                            class="input input-bordered input-sm w-full"
                            type="text"
                            placeholder="Search by title, type, or id..."
                            on:input=move |ev| set_doc_filter.set(event_target_value(&ev))
                        />
                    </div>
                    <div class="p-3 overflow-auto" style="max-height: 60vh;">
                        <Show
                            when=move || !filtered_docs.get().is_empty()
                            fallback=move || {
                                view! { <p class="text-sm opacity-70">No documents found.</p> }
                            }
                        >
                            <ul class="menu w-full">
                                {move || {
                                    filtered_docs
                                        .get()
                                        .into_iter()
                                        .map(|d| {
                                            let title_attr = d.title.clone();
                                            let title_text = d.title.clone();
                                            let file_type = d.file_type.clone();
                                            let size_kb = format!(
                                                "{:.1} KB",
                                                d.size_bytes as f64 / 1024.0,
                                            );
                                            let node_count = d.node_count;
                                            let id = d.id.clone();
                                            let short_id = d
                                                .id
                                                .split(':')
                                                .next()
                                                .unwrap_or("")
                                                .to_string();
                                            // Use a separate clone for display (badge/title) to avoid borrow-after-move when `id` is moved into the delete closure
                                            let id_for_badge = id.clone();
                                            view! {
                                                <li class="!px-0">
                                                    <div class="px-3 py-2 hover:bg-base-200">
                                                        <div class="flex items-center justify-between">
                                                            <div class="min-w-0">
                                                                <div class="flex items-center justify-between gap-3">
                                                                    <p class="font-medium truncate" title=title_attr>
                                                                        {title_text}
                                                                    </p>
                                                                    <button
                                                                        class="btn btn-ghost btn-xs text-error shrink-0"
                                                                        title="Delete document"
                                                                        on:click=move |_| {
                                                                            let id_to_delete = id.clone();
                                                                            // confirm
                                                                            let proceed = window()
                                                                                .and_then(|w| w.confirm_with_message(&format!("Delete document {}? This cannot be undone.", id_to_delete)).ok())
                                                                                .unwrap_or(false);
                                                                            if !proceed { return; }
                                                                            // Perform deletion and refresh UI state
                                                                            spawn_local(async move {
                                                                                let pipeline = GraphRAGPipeline::new();
                                                                                // Best-effort delete; ignore specific error to keep UI responsive
                                                                                let _ = pipeline.delete_document_by_id(&id_to_delete);
                                                                                // Refresh docs and count
                                                                                set_docs.set(read_docs());
                                                                                set_doc_count_state.set(read_doc_count());
                                                                            });
                                                                        }
                                                                    >
                                                                        Delete
                                                                    </button>
                                                                </div>
                                                                <div class="text-xs opacity-70 truncate flex items-center gap-2">
                                                                    <span>{file_type.clone()}</span>
                                                                    <span>"."</span>
                                                                    <span>{size_kb.clone()}</span>
                                                                    <span>"."</span>
                                                                    <span>{format!("nodes: {}", node_count)}</span>
                                                                    <span
                                                                        class="badge badge-ghost badge-sm font-mono ml-1"
                                                                        title={id_for_badge.clone()}
                                                                    >
                                                                        {short_id}
                                                                    </span>
                                                                </div>
                                                            </div>
                                                            <div class="shrink-0"></div>
                                                        </div>
                                                    </div>
                                                </li>
                                            }
                                        })
                                        .collect_view()
                                }}
                            </ul>
                        </Show>
                    </div>
                </div>
            </div>
        </Show>
    }
}
