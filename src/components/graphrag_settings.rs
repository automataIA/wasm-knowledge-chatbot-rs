use crate::graphrag_config::{GraphRAGConfig, GraphRAGConfigManager, GraphRAGMetrics};
use gloo_timers::future::TimeoutFuture;
use leptos::prelude::*;
use leptos::task::spawn_local;

// Simplified GraphRAG Status Metrics component for StatusBar
#[component]
pub fn GraphRAGStatusMetrics(metrics: Signal<GraphRAGMetrics>) -> impl IntoView {
    // Enhanced memory effects with threshold warnings
    let (flash, set_flash) = signal(false);
    let (memory_warning, set_memory_warning) = signal(false);
    let (last_mem, set_last_mem) = signal(0.0f32);

    Effect::new(move |_| {
        let m = metrics.get();
        let current = m.memory_usage_mb;
        let prev = last_mem.get();

        // Flash effect on memory change
        if (current - prev).abs() > 0.01 {
            set_flash.set(true);
            set_last_mem.set(current);
            spawn_local(async move {
                TimeoutFuture::new(400).await;
                set_flash.set(false);
            });
        }

        // Warning effect for high memory usage (>80MB)
        let high_usage = current > 80.0;
        if high_usage != memory_warning.get() {
            set_memory_warning.set(high_usage);
        }
    });

    view! {
        <div class="flex items-center gap-2 text-sm">
            <div class="flex items-center gap-1">
                <div class="w-2 h-2 bg-success rounded-full"></div>
                <span class="text-xs">"GraphRAG"</span>
            </div>
            <span class=move || {
                let flash_active = flash.get();
                let warning_active = memory_warning.get();
                if warning_active {
                    "text-xs opacity-80 text-warning transition-colors".to_string()
                } else if flash_active {
                    "text-xs opacity-60 text-info transition-colors".to_string()
                } else {
                    "text-xs opacity-60".to_string()
                }
            }>
                {move || {
                    let m = metrics.get();
                    let mem_indicator = if memory_warning.get() { "⚠" } else { "" };
                    format!("{}ms · {:.1}MB{}", m.last_query_time_ms, m.memory_usage_mb, mem_indicator)
                }}
            </span>
        </div>
    }
}

// Simplified GraphRAG Settings component with feature toggles and descriptions
#[component]
pub fn GraphRAGSettings(
    config: Signal<GraphRAGConfig>,
    metrics: Signal<GraphRAGMetrics>,
    manager: GraphRAGConfigManager,
) -> impl IntoView {
    // State for showing feature descriptions
    let (show_descriptions, set_show_descriptions) = signal(false);
    let (show_config_explanation, set_show_config_explanation) = signal(false);

    // Explicitly read props to satisfy rustc's analysis outside of macro closures
    let _ = config.get_untracked();
    let _ = metrics.get_untracked();
    let _ = manager.clone();

    view! {
        <div class="space-y-4">
            <div class="card bg-base-100 shadow">
                <div class="card-body py-3">
                    <div class="flex items-center justify-between mb-3">
                        <h2 class="card-title text-sm">"GraphRAG Configuration"</h2>
                        <div class="flex gap-1">
                            <button
                                class="btn btn-ghost btn-xs btn-circle"
                                on:click=move |_| set_show_config_explanation.set(true)
                                title="Explain current configuration"
                            >
                                <i data-lucide="info" class="w-3 h-3"></i>
                            </button>
                            <button
                                class="btn btn-ghost btn-xs btn-circle"
                                on:click=move |_| set_show_descriptions.update(|s| *s = !*s)
                                title="Toggle feature descriptions"
                            >
                                <i data-lucide="help-circle" class="w-3 h-3"></i>
                            </button>
                        </div>
                    </div>

                    <div class="space-y-3">
                        // Hybrid Retrieval Toggle
                        <div class="flex items-center justify-between p-3 bg-base-200 rounded-xl" role="group" aria-label="Hybrid retrieval configuration">
                            <div class="tooltip tooltip-right" data-tip="Combine text and graph scores with fusion weights">
                                <span class="font-medium text-sm">Hybrid Retrieval</span>
                            </div>
                            <input
                                type="checkbox"
                                class="toggle toggle-success rounded-full"
                                checked={move || config.get().hybrid_enabled}
                                aria-checked={move || config.get().hybrid_enabled}
                                aria-label="Enable or disable hybrid retrieval"
                                title="Enable or disable hybrid retrieval"
                                on:change={
                                    let m = manager.clone();
                                    move |_| m.update_config(|c| c.hybrid_enabled = !c.hybrid_enabled)
                                }
                            />
                        </div>

                        // Fusion Weights (Text vs Graph)
                        <div class="p-3 bg-base-200 rounded-xl space-y-2" role="group" aria-label="Fusion weights configuration" aria-describedby="fusion-weights-help">
                            <div class="flex items-center justify-between">
                                <span class="text-sm">Text Weight</span>
                                <div class="flex items-center gap-2" role="group" aria-label="Text weight controls">
                                    <button class="btn btn-xs" title="Increase text weight" aria-label="Increase text weight" on:click={
                                        let m = manager.clone();
                                        move |_| m.update_config(|c| {
                                            c.fusion_text_weight = (c.fusion_text_weight + 0.05).clamp(0.0, 1.0);
                                            let sum = (c.fusion_text_weight + c.fusion_graph_weight).max(0.0001);
                                            c.fusion_text_weight /= sum; c.fusion_graph_weight /= sum;
                                        })
                                    }>"+"</button>
                                    <button class="btn btn-xs" title="Decrease text weight" aria-label="Decrease text weight" on:click={
                                        let m = manager.clone();
                                        move |_| m.update_config(|c| {
                                            c.fusion_text_weight = (c.fusion_text_weight - 0.05).clamp(0.0, 1.0);
                                            let sum = (c.fusion_text_weight + c.fusion_graph_weight).max(0.0001);
                                            c.fusion_text_weight /= sum; c.fusion_graph_weight /= sum;
                                        })
                                    }>"-"</button>
                                    <span class="badge badge-ghost">{move || format!("{:.2}", config.get().fusion_text_weight)}</span>
                                </div>
                            </div>
                            <div class="flex items-center justify-between">
                                <span class="text-sm">Graph Weight</span>
                                <div class="flex items-center gap-2" role="group" aria-label="Graph weight controls">
                                    <button class="btn btn-xs" title="Increase graph weight" aria-label="Increase graph weight" on:click={
                                        let m = manager.clone();
                                        move |_| m.update_config(|c| {
                                            c.fusion_graph_weight = (c.fusion_graph_weight + 0.05).clamp(0.0, 1.0);
                                            let sum = (c.fusion_text_weight + c.fusion_graph_weight).max(0.0001);
                                            c.fusion_text_weight /= sum; c.fusion_graph_weight /= sum;
                                        })
                                    }>"+"</button>
                                    <button class="btn btn-xs" title="Decrease graph weight" aria-label="Decrease graph weight" on:click={
                                        let m = manager.clone();
                                        move |_| m.update_config(|c| {
                                            c.fusion_graph_weight = (c.fusion_graph_weight - 0.05).clamp(0.0, 1.0);
                                            let sum = (c.fusion_text_weight + c.fusion_graph_weight).max(0.0001);
                                            c.fusion_text_weight /= sum; c.fusion_graph_weight /= sum;
                                        })
                                    }>"-"</button>
                                    <span class="badge badge-ghost">{move || format!("{:.2}", config.get().fusion_graph_weight)}</span>
                                </div>
                            </div>
                            <div id="fusion-weights-help" class="text-xs opacity-60">"Weights are normalized to sum to 1.00"</div>
                        </div>
                        // HyDE Toggle with DaisyUI toggle switch
                        <div class="flex items-center justify-between p-3 bg-base-200 rounded-xl">
                            <div class="flex items-center gap-3">
                                <div class="tooltip tooltip-right" data-tip="Hypothetical Document Embeddings for better search">
                                    <span class="font-medium text-sm">HyDE</span>
                                </div>
                                <div class="text-xs text-base-content/70">
                                    "Queries: " {move || metrics.get().queries_processed} " • Score: " {move || format!("{:.1}", metrics.get().performance_score)}
                                </div>
                            </div>
                            <input
                                type="checkbox"
                                class="toggle toggle-success rounded-full"
                                checked={move || config.get().hyde_enabled}
                                on:change={
                                    let m = manager.clone();
                                    move |_| m.toggle_hyde()
                                }
                            />
                        </div>

                        // Community Detection Toggle
                        <div class="flex items-center justify-between p-3 bg-base-200 rounded-xl">
                            <div class="tooltip tooltip-right" data-tip="Groups related entities for better context">
                                <span class="font-medium text-sm">Community</span>
                            </div>
                            <input
                                type="checkbox"
                                class="toggle toggle-primary rounded-full"
                                checked={move || config.get().community_detection_enabled}
                                on:change={
                                    let m = manager.clone();
                                    move |_| m.toggle_community_detection()
                                }
                            />
                        </div>

                        // PageRank Toggle
                        <div class="flex items-center justify-between p-3 bg-base-200 rounded-xl">
                            <div class="tooltip tooltip-right" data-tip="Ranks entities by importance and connections">
                                <span class="font-medium text-sm">PageRank</span>
                            </div>
                            <input
                                type="checkbox"
                                class="toggle toggle-secondary rounded-full"
                                checked={move || config.get().pagerank_enabled}
                                on:change={
                                    let m = manager.clone();
                                    move |_| m.toggle_pagerank()
                                }
                            />
                        </div>

                        // Reranking Toggle
                        <div class="flex items-center justify-between p-3 bg-base-200 rounded-xl">
                            <div class="tooltip tooltip-right" data-tip="Advanced AI models reorder search results">
                                <span class="font-medium text-sm">Reranking</span>
                            </div>
                            <input
                                type="checkbox"
                                class="toggle toggle-accent rounded-full"
                                checked={move || config.get().reranking_enabled}
                                on:change={
                                    let m = manager.clone();
                                    move |_| m.toggle_reranking()
                                }
                            />
                        </div>

                        // Synthesis Toggle
                        <div class="flex items-center justify-between p-3 bg-base-200 rounded-xl">
                            <div class="tooltip tooltip-right" data-tip="Combines multiple sources into coherent answers">
                                <span class="font-medium text-sm">Synthesis</span>
                            </div>
                            <input
                                type="checkbox"
                                class="toggle toggle-info rounded-full"
                                checked={move || config.get().synthesis_enabled}
                                on:change={
                                    let m = manager.clone();
                                    move |_| m.toggle_synthesis()
                                }
                            />
                        </div>
                    </div>

                    // Detailed Descriptions Panel
                    <Show when=move || show_descriptions.get()>
                        <div class="mt-4 space-y-3 text-xs">
                            <div class="divider my-2"></div>

                            <div class="grid gap-3">
                                <div class="p-3 bg-base-200 rounded-lg">
                                    <div class="flex items-center gap-2 mb-1">
                                        <div class="w-2 h-2 bg-primary rounded-full"></div>
                                        <span class="font-semibold">"Community Detection"</span>
                                    </div>
                                    <p class="text-base-content/70">"Uses the Louvain algorithm to identify clusters of related entities in your knowledge graph. This helps provide more contextually relevant information by understanding how concepts are grouped together."</p>
                                </div>

                                <div class="p-3 bg-base-200 rounded-lg">
                                    <div class="flex items-center gap-2 mb-1">
                                        <div class="w-2 h-2 bg-secondary rounded-full"></div>
                                        <span class="font-semibold">"PageRank Scoring"</span>
                                    </div>
                                    <p class="text-base-content/70">"Applies Google's PageRank algorithm to rank entities by their importance and interconnections. More connected and referenced entities get higher scores, improving search relevance."</p>
                                </div>

                                <div class="p-3 bg-base-200 rounded-lg">
                                    <div class="flex items-center gap-2 mb-1">
                                        <div class="w-2 h-2 bg-accent rounded-full"></div>
                                        <span class="font-semibold">"Advanced Reranking"</span>
                                    </div>
                                    <p class="text-base-content/70">"Uses MonoT5 and TILDEv2 neural models to reorder search results based on semantic relevance. This significantly improves the quality of retrieved information."</p>
                                </div>

                                <div class="p-3 bg-base-200 rounded-lg">
                                    <div class="flex items-center gap-2 mb-1">
                                        <div class="w-2 h-2 bg-info rounded-full"></div>
                                        <span class="font-semibold">"Result Synthesis"</span>
                                    </div>
                                    <p class="text-base-content/70">"Combines extractive and abstractive summarization to create coherent, comprehensive answers from multiple knowledge sources. Reduces redundancy and improves answer quality."</p>
                                </div>
                            </div>
                        </div>
                    </Show>

                    <div class="text-xs opacity-60 mt-3">
                        {move || {
                            let m = metrics.get();
                            let perf = manager.get_performance_metrics();
                            format!(
                                "Queries: {} · Score: {:.0} · Hybrid Fusion: {}ms",
                                m.queries_processed,
                                m.performance_score,
                                perf.hybrid_fusion_time_ms
                            )
                        }}
                    </div>
                </div>
            </div>

            // Configuration Explanation Modal
            <Show when=move || show_config_explanation.get()>
                <div class="fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center z-50 p-4">
                    <div class="bg-base-100 rounded-lg shadow-xl max-w-4xl w-full max-h-[90vh] overflow-hidden">
                        <div class="flex justify-between items-center p-4 border-b border-base-300">
                            <h2 class="text-xl font-bold flex items-center gap-2">
                                <i data-lucide="settings" class="w-5 h-5"></i>
                                "Current GraphRAG Configuration"
                            </h2>
                            <button
                                class="btn btn-ghost btn-sm btn-circle"
                                on:click=move |_| set_show_config_explanation.set(false)
                            >
                                "✕"
                            </button>
                        </div>
                        <div class="p-6 overflow-y-auto max-h-[calc(90vh-80px)]">
                            {move || {
                                let cfg = config.get();
                                let m = metrics.get();
                                view! {
                                    <div class="space-y-6">
                                        // Configuration Overview
                                        <div class="alert alert-info">
                                            <i data-lucide="info" class="w-5 h-5"></i>
                                            <div>
                                                <h3 class="font-bold">"Your Configuration Analysis"</h3>
                                                <p class="text-sm">"This setup optimizes for comprehensive knowledge processing with balanced performance."</p>
                                            </div>
                                        </div>

                                        // Current Status
                                        <div class="stats stats-vertical lg:stats-horizontal shadow w-full">
                                            <div class="stat">
                                                <div class="stat-title">"Queries Processed"</div>
                                                <div class="stat-value text-2xl">{m.queries_processed}</div>
                                                <div class="stat-desc">"Total knowledge queries"</div>
                                            </div>
                                            <div class="stat">
                                                <div class="stat-title">"Performance Score"</div>
                                                <div class="stat-value text-2xl">{format!("{:.0}", m.performance_score)}</div>
                                                <div class="stat-desc">"Overall system efficiency"</div>
                                            </div>
                                            <div class="stat">
                                                <div class="stat-title">"Active Features"</div>
                                                <div class="stat-value text-2xl">
                                                    {[cfg.hyde_enabled, cfg.community_detection_enabled, cfg.pagerank_enabled, cfg.reranking_enabled, cfg.synthesis_enabled].iter().filter(|&&x| x).count()}
                                                </div>
                                                <div class="stat-desc">"Out of 5 total features"</div>
                                            </div>
                                        </div>

                                        // Feature Analysis
                                        <div class="grid gap-4">
                                            <h3 class="text-lg font-semibold flex items-center gap-2">
                                                <i data-lucide="layers" class="w-4 h-4"></i>
                                                "Feature Analysis"
                                            </h3>

                                            // HyDE
                                            <div class={format!("card shadow-sm {}", if cfg.hyde_enabled { "border-l-4 border-l-success" } else { "border-l-4 border-l-base-300" })}>
                                                <div class="card-body p-4">
                                                    <div class="flex items-center justify-between">
                                                        <div class="flex items-center gap-3">
                                                            <div class={format!("w-3 h-3 rounded-full {}", if cfg.hyde_enabled { "bg-success" } else { "bg-base-300" })}></div>
                                                            <div>
                                                                <h4 class="font-semibold">"HyDE (Hypothetical Document Embeddings)"</h4>
                                                                <p class="text-sm text-base-content/70">"Enhances search by generating query variations"</p>
                                                            </div>
                                                        </div>
                                                        <div class={format!("badge {}", if cfg.hyde_enabled { "badge-success" } else { "badge-ghost" })}>
                                                            {if cfg.hyde_enabled { "ACTIVE" } else { "DISABLED" }}
                                                        </div>
                                                    </div>
                                                    <Show when=move || cfg.hyde_enabled fallback=move || view! {
                                                        <div class="mt-3 p-3 bg-base-200 rounded-lg">
                                                            <p class="text-sm">"⚪ HyDE is disabled. Enable it to improve search accuracy through query enhancement."</p>
                                                        </div>
                                                    }>
                                                        <div class="mt-3 p-3 bg-success/10 rounded-lg">
                                                            <p class="text-sm">"✅ Your queries are being enhanced with hypothetical document variations for better search accuracy. This improves retrieval quality by ~15-25%."</p>
                                                        </div>
                                                    </Show>
                                                </div>
                                            </div>

                                            // Community Detection
                                            <div class={format!("card shadow-sm {}", if cfg.community_detection_enabled { "border-l-4 border-l-primary" } else { "border-l-4 border-l-base-300" })}>
                                                <div class="card-body p-4">
                                                    <div class="flex items-center justify-between">
                                                        <div class="flex items-center gap-3">
                                                            <div class={format!("w-3 h-3 rounded-full {}", if cfg.community_detection_enabled { "bg-primary" } else { "bg-base-300" })}></div>
                                                            <div>
                                                                <h4 class="font-semibold">"Community Detection"</h4>
                                                                <p class="text-sm text-base-content/70">"Groups related entities using Louvain algorithm"</p>
                                                            </div>
                                                        </div>
                                                        <div class={format!("badge {}", if cfg.community_detection_enabled { "badge-primary" } else { "badge-ghost" })}>
                                                            {if cfg.community_detection_enabled { "ACTIVE" } else { "DISABLED" }}
                                                        </div>
                                                    </div>
                                                    <Show when=move || cfg.community_detection_enabled fallback=move || view! {
                                                        <div class="mt-3 p-3 bg-base-200 rounded-lg">
                                                            <p class="text-sm">"⚪ Community detection is disabled. Enable it to group related concepts for better context."</p>
                                                        </div>
                                                    }>
                                                        <div class="mt-3 p-3 bg-primary/10 rounded-lg">
                                                            <p class="text-sm">"✅ Your knowledge graph is being organized into semantic communities. This provides better contextual understanding and improves answer relevance."</p>
                                                        </div>
                                                    </Show>
                                                </div>
                                            </div>

                                            // PageRank
                                            <div class={format!("card shadow-sm {}", if cfg.pagerank_enabled { "border-l-4 border-l-secondary" } else { "border-l-4 border-l-base-300" })}>
                                                <div class="card-body p-4">
                                                    <div class="flex items-center justify-between">
                                                        <div class="flex items-center gap-3">
                                                            <div class={format!("w-3 h-3 rounded-full {}", if cfg.pagerank_enabled { "bg-secondary" } else { "bg-base-300" })}></div>
                                                            <div>
                                                                <h4 class="font-semibold">"PageRank Scoring"</h4>
                                                                <p class="text-sm text-base-content/70">"Ranks entities by importance and connections"</p>
                                                            </div>
                                                        </div>
                                                        <div class={format!("badge {}", if cfg.pagerank_enabled { "badge-secondary" } else { "badge-ghost" })}>
                                                            {if cfg.pagerank_enabled { "ACTIVE" } else { "DISABLED" }}
                                                        </div>
                                                    </div>
                                                    <Show when=move || cfg.pagerank_enabled fallback=move || view! {
                                                        <div class="mt-3 p-3 bg-base-200 rounded-lg">
                                                            <p class="text-sm">"⚪ PageRank scoring is disabled. Enable it to prioritize important entities in your knowledge base."</p>
                                                        </div>
                                                    }>
                                                        <div class="mt-3 p-3 bg-secondary/10 rounded-lg">
                                                            <p class="text-sm">"✅ Entities are being ranked by their importance and interconnections. This prioritizes the most relevant and authoritative information in responses."</p>
                                                        </div>
                                                    </Show>
                                                </div>
                                            </div>

                                            // Reranking
                                            <div class={format!("card shadow-sm {}", if cfg.reranking_enabled { "border-l-4 border-l-accent" } else { "border-l-4 border-l-warning" })}>
                                                <div class="card-body p-4">
                                                    <div class="flex items-center justify-between">
                                                        <div class="flex items-center gap-3">
                                                            <div class={format!("w-3 h-3 rounded-full {}", if cfg.reranking_enabled { "bg-accent" } else { "bg-warning" })}></div>
                                                            <div>
                                                                <h4 class="font-semibold">"Advanced Reranking"</h4>
                                                                <p class="text-sm text-base-content/70">"Neural models reorder results by relevance"</p>
                                                            </div>
                                                        </div>
                                                        <div class={format!("badge {}", if cfg.reranking_enabled { "badge-accent" } else { "badge-warning" })}>
                                                            {if cfg.reranking_enabled { "ACTIVE" } else { "DISABLED" }}
                                                        </div>
                                                    </div>
                                                    <Show when=move || cfg.reranking_enabled fallback=move || view! {
                                                        <div class="mt-3 p-3 bg-warning/10 rounded-lg">
                                                            <p class="text-sm">"⚠️ Advanced reranking is disabled. This is a powerful feature that significantly improves result quality. Consider enabling it for better answers."</p>
                                                        </div>
                                                    }>
                                                        <div class="mt-3 p-3 bg-accent/10 rounded-lg">
                                                            <p class="text-sm">"✅ Search results are being reordered using MonoT5 and TILDEv2 neural models for maximum relevance."</p>
                                                        </div>
                                                    </Show>
                                                </div>
                                            </div>

                                            // Synthesis
                                            <div class={format!("card shadow-sm {}", if cfg.synthesis_enabled { "border-l-4 border-l-info" } else { "border-l-4 border-l-base-300" })}>
                                                <div class="card-body p-4">
                                                    <div class="flex items-center justify-between">
                                                        <div class="flex items-center gap-3">
                                                            <div class={format!("w-3 h-3 rounded-full {}", if cfg.synthesis_enabled { "bg-info" } else { "bg-base-300" })}></div>
                                                            <div>
                                                                <h4 class="font-semibold">"Result Synthesis"</h4>
                                                                <p class="text-sm text-base-content/70">"Combines multiple sources into coherent answers"</p>
                                                            </div>
                                                        </div>
                                                        <div class={format!("badge {}", if cfg.synthesis_enabled { "badge-info" } else { "badge-ghost" })}>
                                                            {if cfg.synthesis_enabled { "ACTIVE" } else { "DISABLED" }}
                                                        </div>
                                                    </div>
                                                    <Show when=move || cfg.synthesis_enabled fallback=move || view! {
                                                        <div class="mt-3 p-3 bg-base-200 rounded-lg">
                                                            <p class="text-sm">"⚪ Result synthesis is disabled. Enable it to combine multiple sources into better answers."</p>
                                                        </div>
                                                    }>
                                                        <div class="mt-3 p-3 bg-info/10 rounded-lg">
                                                            <p class="text-sm">"✅ Multiple knowledge sources are being synthesized into comprehensive, coherent answers with reduced redundancy."</p>
                                                        </div>
                                                    </Show>
                                                </div>
                                            </div>
                                        </div>

                                        // Performance Impact
                                        <div class="card bg-base-200 shadow-sm">
                                            <div class="card-body p-4">
                                                <h3 class="card-title text-base flex items-center gap-2">
                                                    <i data-lucide="zap" class="w-4 h-4"></i>
                                                    "Performance Impact & Recommendations"
                                                </h3>
                                                <div class="space-y-3 text-sm">
                                                    {
                                                        let active_count = [cfg.hyde_enabled, cfg.community_detection_enabled, cfg.pagerank_enabled, cfg.reranking_enabled, cfg.synthesis_enabled].iter().filter(|&&x| x).count();
                                                        view! {
                                                            <div class="alert alert-warning">
                                                                <Show when=move || active_count <= 1 fallback=move || view! {
                                                                    <Show when=move || active_count <= 3 fallback=move || view! {
                                                                        <Show when=move || active_count == 4 fallback=move || view! {
                                                                            <>
                                                                                <i data-lucide="cpu" class="w-4 h-4"></i>
                                                                                <span>"⚡ <strong>Maximum Configuration:</strong> All {active_count} features active. Best quality but highest computational cost."</span>
                                                                            </>
                                                                        }>
                                                                            <>
                                                                                <i data-lucide="check-circle" class="w-4 h-4"></i>
                                                                                <span>"⚡ <strong>High-Performance Configuration:</strong> {active_count} features active. Excellent knowledge processing with good performance."</span>
                                                                            </>
                                                                        </Show>
                                                                    }>
                                                                        <>
                                                                            <i data-lucide="info" class="w-4 h-4"></i>
                                                                            <span>"⚡ <strong>Balanced Configuration:</strong> {active_count} features active. Good balance between performance and capabilities."</span>
                                                                        </>
                                                                    </Show>
                                                                }>
                                                                    <>
                                                                        <i data-lucide="alert-triangle" class="w-4 h-4"></i>
                                                                        <span>"⚡ <strong>Minimal Configuration:</strong> Only {active_count} feature(s) active. Consider enabling more features for better knowledge processing."</span>
                                                                    </>
                                                                </Show>
                                                            </div>
                                                        }
                                                    }

                                                    <Show when=move || !cfg.reranking_enabled>
                                                        <div class="p-3 bg-warning/10 rounded-lg border border-warning/20">
                                                            <p class="font-medium text-warning">"💡 Recommendation: Enable Advanced Reranking"</p>
                                                            <p>"This feature provides the biggest improvement in answer quality with moderate performance cost."</p>
                                                        </div>
                                                    </Show>

                                                    {
                                                        let active_count = [cfg.hyde_enabled, cfg.community_detection_enabled, cfg.pagerank_enabled, cfg.reranking_enabled, cfg.synthesis_enabled].iter().filter(|&&x| x).count();
                                                        view! {
                                                            <div class="grid grid-cols-1 md:grid-cols-2 gap-3">
                                                                <div class="p-3 bg-success/10 rounded-lg">
                                                                    <p class="font-medium text-success">"✅ Strengths of Your Setup"</p>
                                                                    <ul class="list-disc list-inside text-xs mt-1 space-y-1">
                                                                        <Show when=move || cfg.hyde_enabled>
                                                                            <li>"Enhanced query understanding"</li>
                                                                        </Show>
                                                                        <Show when=move || cfg.community_detection_enabled>
                                                                            <li>"Contextual entity grouping"</li>
                                                                        </Show>
                                                                        <Show when=move || cfg.pagerank_enabled>
                                                                            <li>"Authority-based ranking"</li>
                                                                        </Show>
                                                                        <Show when=move || cfg.synthesis_enabled>
                                                                            <li>"Coherent multi-source answers"</li>
                                                                        </Show>
                                                                    </ul>
                                                                </div>
                                                                <div class="p-3 bg-info/10 rounded-lg">
                                                                    <p class="font-medium text-info">"📊 Expected Performance"</p>
                                                                    <ul class="list-disc list-inside text-xs mt-1 space-y-1">
                                                                        <li>{format!("Processing time: {}ms per query", if active_count >= 4 { "200-500" } else if active_count >= 2 { "100-300" } else { "50-150" })}</li>
                                                                        <li>{format!("Memory usage: {}MB typical", if active_count >= 4 { "40-80" } else if active_count >= 2 { "20-50" } else { "10-30" })}</li>
                                                                        <li>{format!("Answer quality: {}", if active_count >= 4 { "Excellent" } else if active_count >= 2 { "Good" } else { "Basic" })}</li>
                                                                    </ul>
                                                                </div>
                                                            </div>
                                                        }
                                                    }
                                                </div>
                                            </div>
                                        </div>
                                    </div>
                                }
                            }}
                        </div>
                    </div>
                </div>
            </Show>
        </div>
    }
}
