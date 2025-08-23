use leptos::prelude::*;
use crate::components::ui_primitives::Button;
use crate::components::graphrag_settings::GraphRAGSettings;
use crate::graphrag_config::{GraphRAGConfig, GraphRAGMetrics, GraphRAGConfigManager, PerformanceMetrics};

#[component]
pub fn SidebarMonitorRight(
    /// Whether the sidebar is collapsed (hidden). When true, panel is narrow; when false, it shows full width
    collapsed: ReadSignal<bool>,
    set_collapsed: WriteSignal<bool>,
    /// Live GraphRAG config
    graphrag_config: Signal<GraphRAGConfig>,
    /// Live GraphRAG metrics
    graphrag_metrics: Signal<GraphRAGMetrics>,
    /// Manager for performance metrics
    graphrag_manager: GraphRAGConfigManager,
) -> impl IntoView {
    // Pre-clone manager for closures
    let mgr_for_perf = graphrag_manager.clone();

    // Derived widths and classes
    let panel_class = Signal::derive(move || {
        if collapsed.get() {
            // Fully collapsed: no width, no border, ignore pointer events to avoid click interception
            "monitor-scope relative bg-base-200 transition-all duration-300 overflow-hidden w-0 md:w-0 border-l-0 pointer-events-none".to_string()
        } else {
            // Expanded: visible with border and active pointer events
            "monitor-scope relative border-l border-base-300 bg-base-200 transition-all duration-300 overflow-hidden w-80 md:w-96 pointer-events-auto".to_string()
        }
    });

    // Small header inside the panel with a close control
    let close_sidebar = move || set_collapsed.set(true);

    view! {
        <div class=move || panel_class.get()>
            <div class="h-full flex flex-col">
                // Top bar
                <div class="flex items-center justify-between p-3 border-b border-base-300 bg-base-100">
                    <div class="flex items-center gap-2">
                        <i data-lucide="activity" class="w-4 h-4"></i>
                        <span class="text-sm font-semibold">"GraphRAG Monitor"</span>
                    </div>
                    <Button
                        label=Signal::derive(|| "".to_string())
                        variant=Signal::derive(|| "btn-ghost btn-md btn-square".to_string())
                        icon=Signal::derive(|| "panel-right-close".to_string())
                        on_click=Box::new(close_sidebar)
                    />
                </div>

                // Content scroll area
                <div class="flex-1 overflow-y-auto overflow-x-hidden hide-scrollbar p-3 space-y-3">
                    // Query Overview
                    <div class="card bg-base-100 shadow-sm">
                        <div class="card-body p-3">
                            <div class="flex items-center justify-between">
                                <span class="text-xs font-semibold">"Query Overview"</span>
                                <span class="badge badge-ghost badge-sm">
                                    {move || {
                                        let m = graphrag_metrics.get();
                                        format!("{}ms", m.last_query_time_ms)
                                    }}
                                </span>
                            </div>
                            <div class="mt-2 grid grid-cols-2 gap-2 text-xs">
                                <div class="flex items-center justify-between">
                                    <span class="opacity-70">"Queries"</span>
                                    <span class="font-mono">{move || graphrag_metrics.get().queries_processed}</span>
                                </div>
                                <div class="flex items-center justify-between">
                                    <span class="opacity-70">"Cache Hit"</span>
                                    <span class="font-mono">{move || format!("{:.0}%", graphrag_metrics.get().cache_hit_rate * 100.0)}</span>
                                </div>
                                <div class="flex items-center justify-between">
                                    <span class="opacity-70">"Memory"</span>
                                    <span class="font-mono">{move || format!("{:.1} MB", graphrag_metrics.get().memory_usage_mb)}</span>
                                </div>
                                <div class="flex items-center justify-between">
                                    <span class="opacity-70">"Score"</span>
                                    <span class="font-mono">{move || format!("{:.0}", graphrag_metrics.get().performance_score)}</span>
                                </div>
                            </div>
                        </div>
                    </div>

                    // Active Features
                    <div class="card bg-base-100 shadow-sm">
                        <div class="card-body p-3">
                            <div class="flex items-center justify-between">
                                <span class="text-xs font-semibold">"Active Features"</span>
                                <span class="badge badge-ghost badge-sm">{move || graphrag_metrics.get().active_features.len()}</span>
                            </div>
                            <div class="mt-2 flex flex-wrap gap-1">
                                {move || {
                                    let features = graphrag_metrics.get().active_features;
                                    features.into_iter().map(|f| view! { <span class="badge badge-outline badge-xs">{f}</span> }).collect_view()
                                }}
                            </div>
                        </div>
                    </div>

                    // Performance Breakdown
                    <div class="card bg-base-100 shadow-sm">
                        <div class="card-body p-3">
                            <div class="flex items-center justify-between">
                                <span class="text-xs font-semibold">"Performance Breakdown"</span>
                                <i data-lucide="gauge" class="w-3.5 h-3.5 opacity-70"></i>
                            </div>
                            {move || {
                                let p: PerformanceMetrics = mgr_for_perf.get_performance_metrics();
                                view! {
                                    <div class="mt-2 grid grid-cols-2 gap-2 text-xs">
                                        <div class="flex items-center justify-between"><span class="opacity-70">"HyDE"</span><span class="font-mono">{p.hyde_time_ms}"ms"</span></div>
                                        <div class="flex items-center justify-between"><span class="opacity-70">"Community"</span><span class="font-mono">{p.community_detection_time_ms}"ms"</span></div>
                                        <div class="flex items-center justify-between"><span class="opacity-70">"PageRank"</span><span class="font-mono">{p.pagerank_time_ms}"ms"</span></div>
                                        <div class="flex items-center justify-between"><span class="opacity-70">"Rerank"</span><span class="font-mono">{p.reranking_time_ms}"ms"</span></div>
                                        <div class="flex items-center justify-between"><span class="opacity-70">"Fusion"</span><span class="font-mono">{p.hybrid_fusion_time_ms}"ms"</span></div>
                                        <div class="flex items-center justify-between"><span class="opacity-70">"Synthesis"</span><span class="font-mono">{p.synthesis_time_ms}"ms"</span></div>
                                        <div class="col-span-2 flex items-center justify-between border-t border-base-300 pt-1">
                                            <span class="opacity-70">"Total"</span>
                                            <span class="font-mono">{p.total_time_ms}"ms"</span>
                                        </div>
                                    </div>
                                }
                            }}
                        </div>
                    </div>

                    // Current Config Snapshot
                    <div class="card bg-base-100 shadow-sm">
                        <div class="card-body p-3">
                            <div class="flex items-center justify-between">
                                <span class="text-xs font-semibold">"Config Snapshot"</span>
                                <i data-lucide="settings" class="w-3.5 h-3.5 opacity-70"></i>
                            </div>
                            {move || {
                                let c = graphrag_config.get();
                                view! {
                                    <div class="mt-2 grid grid-cols-2 gap-2 text-xs">
                                        <div class="flex items-center justify-between"><span class="opacity-70">"Hybrid"</span><span class="font-mono">{if c.hybrid_enabled { "on" } else { "off" }}</span></div>
                                        <div class="flex items-center justify-between"><span class="opacity-70">"Fusion"</span><span class="font-mono">{format!("T:{:.2} G:{:.2}", c.fusion_text_weight, c.fusion_graph_weight)}</span></div>
                                        <div class="flex items-center justify-between"><span class="opacity-70">"HyDE"</span><span class="font-mono">{if c.hyde_enabled { "on" } else { "off" }}</span></div>
                                        <div class="flex items-center justify-between"><span class="opacity-70">"Community"</span><span class="font-mono">{if c.community_detection_enabled { "on" } else { "off" }}</span></div>
                                        <div class="flex items-center justify-between"><span class="opacity-70">"PageRank"</span><span class="font-mono">{if c.pagerank_enabled { "on" } else { "off" }}</span></div>
                                        <div class="flex items-center justify-between"><span class="opacity-70">"Rerank"</span><span class="font-mono">{if c.reranking_enabled { "on" } else { "off" }}</span></div>
                                        <div class="flex items-center justify-between"><span class="opacity-70">"Synthesis"</span><span class="font-mono">{if c.synthesis_enabled { "on" } else { "off" }}</span></div>
                                    </div>
                                }
                            }}
                        </div>
                    </div>

                    // GraphRAG Settings (moved from left sidebar modal)
                    <div class="card bg-base-100 shadow-sm">
                        <div class="card-body p-3">
                            <div class="flex items-center justify-between mb-2">
                                <span class="text-xs font-semibold">"GraphRAG Settings"</span>
                                <i data-lucide="sliders-horizontal" class="w-3.5 h-3.5 opacity-70"></i>
                            </div>
                            <GraphRAGSettings
                                config=graphrag_config
                                metrics=graphrag_metrics
                                manager=graphrag_manager.clone()
                            />
                        </div>
                    </div>
                </div>
            </div>
        </div>
    }
}
