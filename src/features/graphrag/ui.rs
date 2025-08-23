use crate::features::graphrag::traversal::TraversalResult;
use crate::models::graphrag::{RAGQuery, SearchStrategy};
use crate::state::knowledge_storage_context::KnowledgeStorageContext;
use crate::state::GraphRAGStateContext;
use crate::utils::error_handling::ErrorHandler;
use leptos::prelude::*;

#[component]
pub fn GraphRAGPanel() -> impl IntoView {
    let ctx = expect_context::<GraphRAGStateContext>();
    let (query, set_query) = signal(String::new());
    let (strategy, set_strategy) = signal(SearchStrategy::Combined);
    let (use_rerank, set_use_rerank) = signal(false);
    // Derive read-only signals to avoid capturing the context in multiple closures
    let is_indexing = ctx.is_indexing();
    let is_searching = ctx.is_searching();
    let last_error = ctx.last_error();
    let last_result = ctx.last_result();
    let index_progress = ctx.index_progress();

    // Avoid capturing non-Copy context in event handler; fetch it inside the closure

    view! {
        <ErrorBoundary fallback=|_errors| view! {
            <div class="alert alert-error">
                <span>{"An unexpected error occurred in GraphRAG panel. Please try again."}</span>
            </div>
        }>
        <div class="card card-compact bg-base-200 shadow-sm p-3 gap-2">
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
                    // fetch context inside the handler to avoid capturing it
                    let ctx_local = expect_context::<GraphRAGStateContext>();
                    ctx_local.run_query(q, strategy.get());
                }>"Search"</button>
                <button class="btn btn-ghost btn-sm" disabled=move || is_indexing.get() || is_searching.get() on:click=move |_| {
                    let ctx_local = expect_context::<GraphRAGStateContext>();
                    ctx_local.reindex();
                }>"Reindex"</button>
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
            <Show when=move || last_error.get().is_some()>
                <div class="alert alert-warning">
                    <span>{move || {
                        last_error.get().map(|e| ErrorHandler::get_user_message(&e)).unwrap_or_default()
                    }}</span>
                </div>
            </Show>

            <Show when=move || last_result.get().is_some()>
                <div class="mt-2">
                    <div class="text-xs opacity-70 mb-1">
                        {move || {
                            let r = last_result.get().unwrap();
                            let algo = if r.metadata.algorithms_used.is_empty() { String::new() } else { format!(" · {}", r.metadata.algorithms_used.join(", ")) };
                            format!(
                                "Results: {} nodes, {} edges · {} ms{}",
                                r.nodes.len(), r.edges.len(), r.metadata.processing_time_ms, algo
                            )
                        }}
                    </div>
                    <div>
                        <Show when=move || last_result.get().map(|r| r.nodes.is_empty()).unwrap_or(false)>
                            <div class="text-sm opacity-70">{"No results found. Try different keywords."}</div>
                        </Show>
                        <Show when=move || last_result.get().map(|r| !r.nodes.is_empty()).unwrap_or(false)>
                            {move || {
                                let r = last_result.get().unwrap();
                                view!{
                                    <ul class="menu menu-sm bg-base-100 rounded-box">
                                        {r.nodes
                                            .into_iter()
                                            .take(5)
                                            .map(|n| {
                                                let text = n.content.clone();
                                                let title_text = text.clone();
                                                view!{ <li><span class="truncate max-w-[320px]" title={title_text}>{text}</span></li> }
                                            })
                                            .collect::<Vec<_>>()
                                        }
                                    </ul>
                                    <Show when=move || last_result.get().map(|r| !r.edges.is_empty()).unwrap_or(false)>
                                        {move || {
                                            let r2 = last_result.get().unwrap();
                                            view!{
                                                <div class="mt-2">
                                                    <div class="text-xs opacity-60 mb-1">"Edges (sample):"</div>
                                                    <ul class="menu menu-xs bg-base-100 rounded-box">
                                                        {r2.edges
                                                            .into_iter()
                                                            .take(5)
                                                            .map(|e| {
                                                                let txt = format!("{} → {}  (w={:.2})", e.source_id, e.target_id, e.weight);
                                                                let title_val = txt.clone();
                                                                let content_val = txt;
                                                                view!{ <li><span class="truncate max-w-[320px]" title={title_val}>{content_val}</span></li> }
                                                            })
                                                            .collect::<Vec<_>>()
                                                        }
                                                    </ul>
                                                </div>
                                            }
                                        }}
                                    </Show>
                                }
                            }}
                        </Show>
                    </div>
                </div>
            </Show>
            <Show when=move || last_result.get().is_none() && !is_indexing.get() && !is_searching.get()>
                <div class="text-xs opacity-60">{"Enter a query and press Search to see results."}</div>
            </Show>
            // Neighborhood Explorer
            <div class="mt-3 p-3 bg-base-100 rounded-lg border border-base-300">
                <div class="flex items-center justify-between mb-2">
                    <div class="font-medium text-sm">"Neighborhood Explorer"</div>
                    <div class="text-xs opacity-60">"BFS/DFS over GraphStore"</div>
                </div>
                {|| {
                    // Local UI state for traversal controls
                    let (start_id, set_start_id) = signal(String::new());
                    let (relations, set_relations) = signal(String::new());
                    let (max_depth, set_max_depth) = signal(String::new());
                    let (max_nodes, set_max_nodes) = signal(String::new());
                    let (max_edges, set_max_edges) = signal(String::new());
                    let (use_bfs, set_use_bfs) = signal(true);
                    let (trav_result, set_trav_result) = signal::<Option<TraversalResult>>(None);
                    let (trav_error, set_trav_error) = signal(String::new());

                    let run_traversal = move || {
                        set_trav_error.set(String::new());
                        let kctx: KnowledgeStorageContext = expect_context::<KnowledgeStorageContext>();
                        let allowed: Option<Vec<String>> = {
                            let raw = relations.get().trim().to_string();
                            if raw.is_empty() { None } else {
                                Some(raw.split(',').map(|s| s.trim().to_string()).filter(|s| !s.is_empty()).collect())
                            }
                        };
                        let md = max_depth.get().trim().parse::<usize>().ok();
                        let mn = max_nodes.get().trim().parse::<usize>().ok();
                        let me = max_edges.get().trim().parse::<usize>().ok();
                        let sid = start_id.get();
                        if sid.trim().is_empty() {
                            set_trav_error.set("Enter a start node id".to_string());
                            return;
                        }
                        let res = if use_bfs.get() {
                            kctx.traverse_bfs(&sid, allowed.as_deref(), md, mn, me)
                        } else {
                            kctx.traverse_dfs(&sid, allowed.as_deref(), md, mn, me)
                        };
                        match res {
                            Ok(r) => set_trav_result.set(Some(r)),
                            Err(e) => set_trav_error.set(format!("{}", e)),
                        }
                    };

                    view! {
                        <div class="grid grid-cols-1 md:grid-cols-2 gap-2">
                            <input class="input input-sm input-bordered" placeholder="Start node id (e.g., ent:Alice)" prop:value=start_id on:input=move |ev| set_start_id.set(event_target_value(&ev)) />
                            <input class="input input-sm input-bordered" placeholder="Allowed relations (comma)" prop:value=relations on:input=move |ev| set_relations.set(event_target_value(&ev)) />
                            <input class="input input-sm input-bordered" placeholder="Max depth (opt)" prop:value=max_depth on:input=move |ev| set_max_depth.set(event_target_value(&ev)) />
                            <div class="grid grid-cols-2 gap-2">
                                <input class="input input-sm input-bordered" placeholder="Max nodes (opt)" prop:value=max_nodes on:input=move |ev| set_max_nodes.set(event_target_value(&ev)) />
                                <input class="input input-sm input-bordered" placeholder="Max edges (opt)" prop:value=max_edges on:input=move |ev| set_max_edges.set(event_target_value(&ev)) />
                            </div>
                        </div>
                        <div class="flex items-center gap-2 mt-2">
                            <label class="label cursor-pointer gap-2">
                                <input class="checkbox checkbox-xs" type="checkbox" prop:checked=use_bfs on:change=move |ev| set_use_bfs.set(event_target_checked(&ev)) />
                                <span class="label-text text-xs">"Use BFS (uncheck for DFS)"</span>
                            </label>
                            <button class="btn btn-sm btn-outline" on:click=move |_| run_traversal()>"Traverse"</button>
                            <Show when=move || !trav_error.get().is_empty()>
                                <span class="text-xs text-error">{trav_error}</span>
                            </Show>
                        </div>
                        <Show when=move || trav_result.get().is_some()>
                            {move || {
                                let r = trav_result.get().unwrap();
                                view! {
                                    <div class="mt-2 text-xs opacity-70">{format!("Visited: {} nodes, {} edges", r.visited_nodes.len(), r.visited_edges.len())}</div>
                                    <div class="grid grid-cols-1 md:grid-cols-2 gap-2 mt-2">
                                        <div>
                                            <div class="text-xs opacity-60 mb-1">"Nodes (sample):"</div>
                                            <ul class="menu menu-xs bg-base-100 rounded-box">
                                                {r.visited_nodes.iter().take(6).cloned().map(|n| {
                                                    let n_title = n.clone();
                                                    let n_content = n;
                                                    view!{ <li><span class="truncate max-w-[300px]" title={n_title}>{n_content}</span></li> }
                                                }).collect::<Vec<_>>()}
                                            </ul>
                                        </div>
                                        <div>
                                            <div class="text-xs opacity-60 mb-1">"Edges (sample):"</div>
                                            <ul class="menu menu-xs bg-base-100 rounded-box">
                                                {r.visited_edges.iter().take(6).cloned().map(|e| {
                                                    let e_title = e.clone();
                                                    let e_content = e;
                                                    view!{ <li><span class="truncate max-w-[300px]" title={e_title}>{e_content}</span></li> }
                                                }).collect::<Vec<_>>()}
                                            </ul>
                                        </div>
                                    </div>
                                }
                            }}
                        </Show>
                    }
                }}
            </div>
        </div>
        </ErrorBoundary>
    }
}
