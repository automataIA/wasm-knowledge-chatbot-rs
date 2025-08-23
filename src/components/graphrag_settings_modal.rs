use leptos::prelude::*;
use leptos::task::spawn_local;
use crate::components::ui_primitives::{Button, Toggle};
use crate::graphrag_config::{GraphRAGConfigManager, GraphRAGConfig};
use crate::components::graphrag_settings::GraphRAGSettings;
use crate::state::GraphRAGStateContext;
use crate::models::graphrag::{RAGQuery, SearchStrategy};

#[component]
pub fn GraphRAGSettingsModal(
    show: ReadSignal<bool>,
    set_show: WriteSignal<bool>,
) -> impl IntoView {
    let config_manager = GraphRAGConfigManager::new();
    let current_config = config_manager.get_config_untracked();
    let (hyde_enabled, set_hyde_enabled) = signal(current_config.hyde_enabled);
    let (community_detection, set_community_detection) = signal(current_config.community_detection_enabled);
    let (pagerank_enabled, set_pagerank_enabled) = signal(current_config.pagerank_enabled);
    let (reranking_enabled, set_reranking_enabled) = signal(current_config.reranking_enabled);
    let (synthesis_enabled, set_synthesis_enabled) = signal(current_config.synthesis_enabled);
    let (max_query_time, set_max_query_time) = signal(current_config.max_query_time_ms);
    let (max_memory, set_max_memory) = signal(current_config.max_memory_mb);
    let (batch_size, set_batch_size) = signal(current_config.batch_size);
    // Persistent search strategy selection
    let (default_strategy, set_default_strategy) = signal(current_config.search_strategy.clone());

    // Inline GraphRAG query controls
    let (query, set_query) = signal(String::new());
    let (strategy, set_strategy) = signal(SearchStrategy::Combined);
    let (use_rerank, set_use_rerank) = signal(current_config.reranking_enabled);
    let ctx = expect_context::<GraphRAGStateContext>();
    let is_indexing = ctx.is_indexing();
    let is_searching = ctx.is_searching();
    let index_progress = ctx.index_progress();

    // Handlers will be provided inline to avoid FnOnce moves and Rc Send/Sync issues

    // Derive signals from the same manager instance so state stays consistent
    let config_signal = {
        let m = config_manager.clone();
        Signal::derive(move || m.get_config())
    };
    let metrics_signal = {
        let m = config_manager.clone();
        Signal::derive(move || m.get_metrics())
    };

    view! {
        <Show when=move || show.get()>
            // Modal backdrop
            <div class="fixed inset-0 bg-black/50 backdrop-blur-sm z-50 flex items-center justify-center p-4 animate-fade-in">
                // Modal content
                <div class="bg-base-100 rounded-xl shadow-xl max-w-2xl w-full max-h-[90vh] overflow-y-auto animate-slide-up">
                    // Header
                    <div class="flex items-center justify-between p-6 border-b border-base-300">
                        <div class="flex items-center gap-3">
                            <div class="p-2 bg-primary/10 rounded-lg">
                                <i data-lucide="settings" class="h-5 w-5 text-primary"></i>
                            </div>
                            <div>
                                <h2 class="text-xl font-semibold text-base-content">
                                    "GraphRAG Configuration"
                                </h2>
                                <p class="text-sm text-base-content/60">
                                    "Configure knowledge graph and search settings"
                                </p>
                            </div>
                        </div>
                        <Button
                            label=Signal::derive(|| "".to_string())
                            variant=Signal::derive(|| "btn-ghost btn-sm btn-circle".to_string())
                            icon=Signal::derive(|| "x".to_string())
                            on_click=Box::new({
                                let set_show = set_show;
                                move || {
                                    set_show.set(false);
                                }
                            })
                        />
                    </div>

                    // Content
                    <div class="p-6 space-y-6">
                        // Query section (moved from floating panel)
                        <div class="space-y-3">
                            <h3 class="font-medium text-base-content">"GraphRAG Query"</h3>
                            <div class="card bg-base-200">
                                <div class="card-body p-3 gap-2">
                                    <div class="flex items-center gap-2">
                                        <input
                                            class="input input-sm input-bordered flex-1"
                                            type="text"
                                            placeholder="GraphRAG query"
                                            prop:value=query
                                            on:input=move |ev| set_query.set(event_target_value(&ev))
                                        />
                                        <button class="btn btn-primary btn-sm" disabled=move || is_indexing.get() || is_searching.get() on:click=move |_| {
                                            let q_text = query.get();
                                            if q_text.trim().is_empty() { return; }
                                            let mut q = RAGQuery::new(q_text);
                                            q.config.use_reranking = use_rerank.get();
                                            let ctx_local = expect_context::<GraphRAGStateContext>();
                                            ctx_local.run_query(q, strategy.get());
                                        }>
                                            "Search"
                                        </button>
                                        <button class="btn btn-ghost btn-sm" disabled=move || is_indexing.get() || is_searching.get() on:click=move |_| {
                                            let ctx_local = expect_context::<GraphRAGStateContext>();
                                            ctx_local.reindex();
                                        }>
                                            "Reindex"
                                        </button>
                                    </div>
                                    <div class="flex items-center gap-3 text-xs">
                                        <label class="label cursor-pointer gap-2">
                                            <input class="checkbox checkbox-xs" type="checkbox" prop:checked=use_rerank on:change=move |ev| {
                                                let v = event_target_checked(&ev);
                                                set_use_rerank.set(v);
                                            } />
                                            <span class="label-text">"Use reranking"</span>
                                        </label>
                                    </div>
                                    <div class="flex items-center gap-2 text-xs">
                                        <span class="opacity-70">"Strategy:"</span>
                                        <select class="select select-bordered select-xs"
                                            on:change=move |ev| {
                                                let v = event_target_value(&ev);
                                                let s = match v.as_str() {
                                                    "Local" => SearchStrategy::Local,
                                                    "Global" => SearchStrategy::Global,
                                                    "Combined" => SearchStrategy::Combined,
                                                    _ => SearchStrategy::Combined,
                                                };
                                                set_strategy.set(s);
                                            }
                                        >
                                            <option selected=true value="Combined">"Combined"</option>
                                            <option value="Local">"Local"</option>
                                            <option value="Global">"Global"</option>
                                        </select>
                                        <Show when=move || is_indexing.get()>
                                            <span class="badge badge-sm badge-info">"Indexing"</span>
                                        </Show>
                                        <Show when=move || is_searching.get()>
                                            <span class="badge badge-sm badge-accent">"Searching"</span>
                                        </Show>
                                        <Show when=move || index_progress.get().is_some()>
                                            <div class="flex items-center gap-2">
                                                <progress class="progress progress-xs w-32" max="1.0" prop:value=move || index_progress.get().unwrap_or(0.0).to_string()></progress>
                                                <span class="tabular-nums opacity-70">
                                                    {move || {
                                                        let p = (index_progress.get().unwrap_or(0.0) * 100.0).round() as i32;
                                                        format!("{}%", p)
                                                    }}
                                                </span>
                                            </div>
                                        </Show>
                                    </div>
                                </div>
                            </div>
                        </div>
                        // Enable/Disable GraphRAG
                        <div class="space-y-2">
                            <h3 class="font-medium text-base-content">"Enable GraphRAG"</h3>
                            <p class="text-sm text-base-content/60">
                                "Turn on enhanced knowledge graph search capabilities"
                            </p>
                            <Toggle
                                checked=hyde_enabled
                                set_checked=set_hyde_enabled
                                label=Signal::derive(|| "Enable GraphRAG processing".to_string())
                            />
                        </div>

                        <div class="divider"></div>

                        // Advanced Settings
                        <div class="space-y-4">
                            <h4 class="font-medium text-base-content">"Advanced Settings"</h4>

                            <div class="grid grid-cols-2 gap-4">
                                <div>
                                    <label class="label">
                                        <span class="label-text">"Query Timeout (ms)"</span>
                                    </label>
                                    <input
                                        type="number"
                                        class="input input-bordered w-full"
                                        value=move || max_query_time.get().to_string()
                                        on:input=move |ev| {
                                            if let Ok(val) = event_target_value(&ev).parse::<u32>() {
                                                set_max_query_time.set(val);
                                            }
                                        }
                                        min="1000"
                                        max="30000"
                                    />
                                </div>

                                <div>
                                    <label class="label">
                                        <span class="label-text">"Memory Limit (MB)"</span>
                                    </label>
                                    <input
                                        type="number"
                                        class="input input-bordered w-full"
                                        value=move || max_memory.get().to_string()
                                        on:input=move |ev| {
                                            if let Ok(val) = event_target_value(&ev).parse::<u32>() {
                                                set_max_memory.set(val);
                                            }
                                        }
                                        min="50"
                                        max="1000"
                                    />
                                </div>

                                <div>
                                    <label class="label">
                                        <span class="label-text">"Batch Size"</span>
                                    </label>
                                    <input
                                        type="number"
                                        class="input input-bordered w-full"
                                        value=move || batch_size.get().to_string()
                                        on:input=move |ev| {
                                            if let Ok(val) = event_target_value(&ev).parse::<usize>() {
                                                set_batch_size.set(val);
                                            }
                                        }
                                        min="1"
                                        max="100"
                                    />
                                </div>
                            </div>
                        </div>

                        // Feature toggles
                        <div class="space-y-4">
                            <h4 class="font-medium text-base-content">"Feature Configuration"</h4>

                            // Default Search Strategy (persistent)
                            <div class="space-y-2">
                                <label class="label">
                                    <span class="label-text">"Default Search Strategy"</span>
                                </label>
                                <select class="select select-bordered w-full"
                                    on:change=move |ev| {
                                        let v = event_target_value(&ev);
                                        let s = match v.as_str() {
                                            "Automatic" => SearchStrategy::Automatic,
                                            "Local" => SearchStrategy::Local,
                                            "Global" => SearchStrategy::Global,
                                            "Combined" => SearchStrategy::Combined,
                                            _ => SearchStrategy::Automatic,
                                        };
                                        set_default_strategy.set(s);
                                    }
                                >
                                    <option selected=move || default_strategy.get() == SearchStrategy::Automatic value="Automatic">"Automatic"</option>
                                    <option selected=move || default_strategy.get() == SearchStrategy::Combined value="Combined">"Combined"</option>
                                    <option selected=move || default_strategy.get() == SearchStrategy::Local value="Local">"Local"</option>
                                    <option selected=move || default_strategy.get() == SearchStrategy::Global value="Global">"Global"</option>
                                </select>
                                <p class="text-xs text-base-content/60">"Used by chat when Knowledge is enabled."</p>
                            </div>

                            <Toggle
                                checked=community_detection
                                set_checked=set_community_detection
                                label=Signal::derive(|| "Community Detection".to_string())
                            />

                            <Toggle
                                checked=pagerank_enabled
                                set_checked=set_pagerank_enabled
                                label=Signal::derive(|| "PageRank Scoring".to_string())
                            />

                            <Toggle
                                checked=reranking_enabled
                                set_checked=set_reranking_enabled
                                label=Signal::derive(|| "Advanced Reranking".to_string())
                            />

                            <Toggle
                                checked=synthesis_enabled
                                set_checked=set_synthesis_enabled
                                label=Signal::derive(|| "Result Synthesis".to_string())
                            />
                        </div>

                        <div class="divider"></div>

                        // Integrated consolidated settings component
                        <GraphRAGSettings
                            config=config_signal
                            metrics=metrics_signal
                            manager=config_manager.clone()
                        />
                    </div>

                    // Footer
                    <div class="flex items-center justify-between p-6 border-t border-base-300 bg-base-50">
                        <Button
                            label=Signal::derive(|| "Reset to Defaults".to_string())
                            variant=Signal::derive(|| "btn-ghost btn-sm".to_string())
                            icon=Signal::derive(|| "rotate-ccw".to_string())
                            on_click=Box::new({
                                let set_hyde_enabled = set_hyde_enabled;
                                let set_community_detection = set_community_detection;
                                let set_pagerank_enabled = set_pagerank_enabled;
                                let set_reranking_enabled = set_reranking_enabled;
                                let set_synthesis_enabled = set_synthesis_enabled;
                                let set_max_query_time = set_max_query_time;
                                let set_max_memory = set_max_memory;
                                let set_batch_size = set_batch_size;
                                move || {
                                    let default_config = GraphRAGConfig::default();
                                    set_hyde_enabled.set(default_config.hyde_enabled);
                                    set_community_detection.set(default_config.community_detection_enabled);
                                    set_pagerank_enabled.set(default_config.pagerank_enabled);
                                    set_reranking_enabled.set(default_config.reranking_enabled);
                                    set_synthesis_enabled.set(default_config.synthesis_enabled);
                                    set_max_query_time.set(default_config.max_query_time_ms);
                                    set_max_memory.set(default_config.max_memory_mb);
                                    set_batch_size.set(default_config.batch_size);
                                }
                            })
                        />
                        <div class="flex gap-3">
                            <Button
                                label=Signal::derive(|| "Cancel".to_string())
                                variant=Signal::derive(|| "btn-ghost".to_string())
                                on_click=Box::new({
                                    let set_show = set_show;
                                    move || {
                                        set_show.set(false);
                                    }
                                })
                            />
                            <Button
                                label=Signal::derive(|| "Save Changes".to_string())
                                variant=Signal::derive(|| "btn-primary".to_string())
                                icon=Signal::derive(|| "check".to_string())
                                on_click=Box::new({
                                    let config_manager2 = config_manager.clone();
                                    let set_show = set_show;
                                    move || {
                                        let config_manager = config_manager2.clone();
                                        let hyde = hyde_enabled.get();
                                        let community = community_detection.get();
                                        let pagerank = pagerank_enabled.get();
                                        let rerank = reranking_enabled.get();
                                        let synth = synthesis_enabled.get();
                                        let max_time = max_query_time.get();
                                        let max_mem = max_memory.get();
                                        let batch = batch_size.get();

                                        spawn_local(async move {
                                            config_manager.update_config(|config| {
                                                config.hyde_enabled = hyde;
                                                config.community_detection_enabled = community;
                                                config.pagerank_enabled = pagerank;
                                                config.reranking_enabled = rerank;
                                                config.synthesis_enabled = synth;
                                                config.max_query_time_ms = max_time;
                                                config.max_memory_mb = max_mem;
                                                config.batch_size = batch;
                                                config.search_strategy = default_strategy.get();
                                            });
                                        });

                                        set_show.set(false);
                                    }
                                })
                            />
                        </div>
                    </div>
                </div>
            </div>
        </Show>
    }
}
