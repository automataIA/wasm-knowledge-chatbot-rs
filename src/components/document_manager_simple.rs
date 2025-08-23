use crate::components::ui_primitives::Button;
use crate::error_handling::AppError;
use crate::state::GraphRAGStateContext;
use crate::storage::ConversationStorage;
use crate::utils::storage::StorageUtils;
use leptos::html::Input;
use leptos::prelude::*;
use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen_futures::JsFuture;

#[component]
pub fn DocumentManagerSimple() -> impl IntoView {
    // Local storage instance (component-scoped)
    let storage = match ConversationStorage::new() {
        Ok(s) => Some(s),
        Err(e) => {
            web_sys::console::error_1(&format!("Storage init failed: {e}").into());
            None
        }
    };

    // UI state
    let (json_text, set_json_text) = signal(String::new());
    let (merge, set_merge) = signal(true);
    let (error_msg, set_error_msg) = signal(Option::<String>::None);
    let (success_msg, set_success_msg) = signal(Option::<String>::None);
    // Import progress state
    let (_importing, set_importing) = signal(false);
    let (import_total, set_import_total) = signal(0u32);
    let (import_done, set_import_done) = signal(0u32);
    // Removed inline knowledge search; main chat handles knowledge queries
    let file_input: NodeRef<Input> = NodeRef::new();

    // Optional GraphRAG context (present when provider is mounted)
    let graphrag_ctx = use_context::<GraphRAGStateContext>();
    // Derived progress signal to avoid moving ctx into closures
    let ctx_for_progress = graphrag_ctx.clone();
    let index_progress = Signal::derive(move || {
        ctx_for_progress
            .as_ref()
            .and_then(|c| c.index_progress().get())
    });

    // Helpers
    let show_error = move |err: AppError| {
        set_success_msg.set(None);
        set_error_msg.set(Some(err.to_string()));
    };
    let show_success = move |msg: &str| {
        set_error_msg.set(None);
        set_success_msg.set(Some(msg.to_string()));
    };

    // Clear any lingering success banner while indexing is active
    Effect::new(move |_| {
        if index_progress.get().is_some() {
            set_success_msg.set(None);
        }
    });

    // Actions
    let storage_export = storage.clone();
    let on_export = Box::new(move || match &storage_export {
        None => show_error(AppError::Storage("storage unavailable".into())),
        Some(s) => match s.export_json() {
            Ok(bundle) => {
                set_json_text.set(bundle);
                show_success("Export completed.");
            }
            Err(e) => show_error(AppError::Storage(format!("export failed: {e}"))),
        },
    });

    let storage_import = storage.clone();
    let graphrag_ctx_on_import = graphrag_ctx.clone();
    let on_import = Box::new(move || {
        let txt = json_text.get();
        if txt.trim().is_empty() {
            show_error(AppError::Validation(
                "Paste an export bundle before importing".into(),
            ));
            return;
        }
        match &storage_import {
            None => show_error(AppError::Storage("storage unavailable".into())),
            Some(s) => match s.import_json(&txt, merge.get()) {
                Ok(()) => {
                    show_success("Import completed.");
                    // Persist current buffer for KnowledgeStorageContext
                    let _ =
                        StorageUtils::store_local("knowledge_upload_buffer_v1", &json_text.get());
                    // Prompt to reindex now
                    let confirm = web_sys::window()
                        .and_then(|w| w.confirm_with_message("Index with GraphRAG now?").ok())
                        .unwrap_or(false);
                    if confirm {
                        if let Some(ctx) = graphrag_ctx_on_import.clone() {
                            ctx.reindex();
                        }
                        // Do not show a persistent "started" banner; progress alert will be shown separately
                        set_success_msg.set(None);
                    } else {
                        set_success_msg.set(Some(
                            "Import completed. You can index later from GraphRAG settings."
                                .to_string(),
                        ));
                    }
                }
                Err(e) => show_error(AppError::Validation(format!("import failed: {e}"))),
            },
        }
    });

    view! {
        <div class="p-6 space-y-6">
            // Header Section (simplified)
            <div class="flex flex-col sm:flex-row sm:items-center justify-between gap-4">
                <div>
                    <h2 class="text-2xl font-bold text-base-content">"Document Manager"</h2>
                    <p class="text-sm text-base-content/70 mt-1">
                        "Upload, manage, and search your documents"
                    </p>
                </div>
            </div>

            // Quick Actions Card
            <div class="card bg-base-100 shadow-sm border border-base-300 rounded-xl">
                <div class="card-body p-4">
                    <h3 class="card-title text-lg mb-3">"Quick Actions"</h3>
                    <div class="grid grid-cols-1 sm:grid-cols-3 gap-3 w-full">
                        <div class="tooltip" attr:data-tip="Load .md/.txt files">
                            <Button
                                label=Signal::derive(|| "Load Markdown".to_string())
                                on_click=Box::new({
                                    move || {
                                        if let Some(input) = file_input.get() {
                                            input.click();
                                        }
                                    }
                                })
                                variant=Signal::derive(|| {
                                    "btn-primary btn-lg w-full rounded-lg".to_string()
                                })
                                icon=Signal::derive(|| "upload".to_string())
                                icon_position=Signal::derive(|| "left".to_string())
                            />
                        </div>
                        <div class="tooltip" attr:data-tip="Export current KB data">
                            <Button
                                label=Signal::derive(|| "Export Data".to_string())
                                on_click=on_export
                                variant=Signal::derive(|| {
                                    "btn-outline btn-lg w-full rounded-lg".to_string()
                                })
                                icon=Signal::derive(|| "download".to_string())
                                icon_position=Signal::derive(|| "left".to_string())
                            />
                        </div>
                        <div class="tooltip" attr:data-tip="Import exported bundle">
                            <Button
                                label=Signal::derive(|| "Import Data".to_string())
                                on_click=on_import
                                variant=Signal::derive(|| {
                                    "btn-outline btn-lg w-full rounded-lg".to_string()
                                })
                                icon=Signal::derive(|| "upload-cloud".to_string())
                                icon_position=Signal::derive(|| "left".to_string())
                            />
                        </div>
                    </div>

                    // Modern Toggle Switch
                    <div class="form-control mt-4">
                        <label class="label cursor-pointer justify-start gap-3">
                            <input
                                type="checkbox"
                                class="toggle toggle-primary rounded-full"
                                prop:checked=merge
                                on:change=move |ev| {
                                    let checked = event_target_checked(&ev);
                                    set_merge.set(checked);
                                }
                            />
                            <span class="label-text font-medium">"Merge on import"</span>
                            <span class="label-text-alt text-base-content/60">
                                "Combine with existing data"
                            </span>
                        </label>
                    </div>
                </div>
            </div>

            // Import Progress
            <Show when=move || { import_total.get() > 0 && import_done.get() < import_total.get() }>
                <div class="card bg-base-100 shadow-sm border border-base-300 rounded-xl">
                    <div class="card-body p-4">
                        <div class="flex items-center justify-between mb-2">
                            <h3 class="card-title text-sm">"Importing files"</h3>
                            <span class="text-xs opacity-70 font-mono">
                                {move || {
                                    let done = import_done.get();
                                    let total = import_total.get();
                                    let pct = if total > 0 {
                                        (done as f32 / total as f32) * 100.0
                                    } else {
                                        0.0
                                    };
                                    format!("{}/{} ({:.0}%)", done, total, pct)
                                }}
                            </span>
                        </div>
                        <progress
                            class="progress progress-primary w-full"
                            max=move || import_total.get().to_string()
                            value=move || import_done.get().to_string()
                        ></progress>
                    </div>
                </div>
            </Show>

            // GraphRAG indexing status (shows percent with DaisyUI alert)
            <Show when=move || index_progress.get().is_some() fallback=|| view! { <></> }>
                {move || {
                    let p = index_progress.get().unwrap_or(0.0);
                    let pct = (p.clamp(0.0, 1.0) * 100.0).round() as u32;
                    let is_done = pct >= 100;
                    let alert_class = if is_done {
                        "alert alert-success shadow-sm rounded-lg"
                    } else {
                        "alert alert-warning shadow-sm rounded-lg"
                    };
                    let progress_class = if is_done {
                        "progress progress-success w-32 ml-3"
                    } else {
                        "progress progress-warning w-32 ml-3"
                    };
                    view! {
                        <div class=alert_class>
                            // Icon
                            <Show when=move || is_done>
                                <svg
                                    xmlns="http://www.w3.org/2000/svg"
                                    width="24"
                                    height="24"
                                    viewBox="0 0 24 24"
                                    fill="none"
                                    stroke="currentColor"
                                    stroke-width="2"
                                    stroke-linecap="round"
                                    stroke-linejoin="round"
                                    data-lucide="check-circle"
                                    class="lucide lucide-check-circle w-5 h-5"
                                >
                                    <path d="M21.801 10A10 10 0 1 1 17 3.335"></path>
                                    <path d="m9 11 3 3L22 4"></path>
                                </svg>
                            </Show>
                            <Show when=move || !is_done>
                                <svg
                                    xmlns="http://www.w3.org/2000/svg"
                                    width="24"
                                    height="24"
                                    viewBox="0 0 24 24"
                                    fill="none"
                                    stroke="currentColor"
                                    stroke-width="2"
                                    stroke-linecap="round"
                                    stroke-linejoin="round"
                                    data-lucide="loader"
                                    class="lucide lucide-loader w-5 h-5 animate-spin"
                                >
                                    <path d="M12 2v4"></path>
                                    <path d="M12 18v4"></path>
                                    <path d="m4.93 4.93 2.83 2.83"></path>
                                    <path d="m16.24 16.24 2.83 2.83"></path>
                                    <path d="M2 12h4"></path>
                                    <path d="M18 12h4"></path>
                                    <path d="m4.93 19.07 2.83-2.83"></path>
                                    <path d="m16.24 7.76 2.83-2.83"></path>
                                </svg>
                            </Show>
                            <span>
                                {if is_done {
                                    "Indexing with GraphRAG completed".to_string()
                                } else {
                                    format!("Indexing with GraphRAG... {}%", pct)
                                }}
                            </span>
                            <progress
                                class=progress_class
                                max="100"
                                value=pct.to_string()
                            ></progress>
                        </div>
                    }
                }}
            </Show>

            // Status Messages
            <Show when=move || error_msg.get().is_some() || success_msg.get().is_some()>
                <div class="space-y-2">
                    <Show when=move || error_msg.get().is_some()>
                        <div class="alert alert-error shadow-sm rounded-lg">
                            <i data-lucide="alert-circle" class="w-5 h-5"></i>
                            <span>{move || error_msg.get().unwrap_or_default()}</span>
                        </div>
                    </Show>
                    <Show when=move || success_msg.get().is_some()>
                        <div class="alert alert-success shadow-sm rounded-lg">
                            <i data-lucide="check-circle" class="w-5 h-5"></i>
                            <span>{move || success_msg.get().unwrap_or_default()}</span>
                        </div>
                    </Show>
                </div>
            </Show>

            // Content Editor Card
            <div class="card bg-base-100 shadow-sm border border-base-300 rounded-xl">
                <div class="card-body p-4">
                    <div class="flex items-center justify-between mb-3">
                        <h3 class="card-title text-lg">"Content Editor"</h3>
                        <div class="flex gap-2">
                            <div class="badge badge-ghost">
                                <i data-lucide="type" class="w-3 h-3 mr-1"></i>
                                {move || {
                                    let text = json_text.get();
                                    format!("{} chars", text.len())
                                }}
                            </div>
                        </div>
                    </div>
                    <textarea
                        class="textarea textarea-bordered w-full font-mono text-sm rounded-xl focus:ring-2 focus:ring-primary/30"
                        rows="14"
                        placeholder="Paste export data here or load markdown files above..."
                        prop:value=json_text
                        on:input=move |ev| {
                            let value = event_target_value(&ev);
                            set_json_text.set(value);
                        }
                    ></textarea>
                </div>
            </div>

            // Hidden file input for Markdown/Text upload
            <input
                node_ref=file_input
                type="file"
                accept=".md,.markdown,.txt,text/markdown,text/plain"
                multiple
                style="display:none"
                on:change=move |ev| {
                    let target: web_sys::HtmlInputElement = event_target(&ev);
                    if let Some(files) = target.files() {
                        web_sys::console::log_1(&"Markdown upload: change event".into());
                        let len = files.length();
                        if len == 0 {
                            return;
                        }
                        set_error_msg.set(None);
                        set_success_msg.set(Some(format!("Reading {} file(s)...", len)));
                        let mut supported_total: u32 = 0;
                        for i in 0..len {
                            if let Some(file) = files.item(i) {
                                let name = file.name();
                                let mime = file.type_();
                                let is_text = name.ends_with(".md") || name.ends_with(".markdown")
                                    || name.ends_with(".txt") || mime == "text/markdown"
                                    || mime == "text/plain";
                                if is_text {
                                    supported_total += 1;
                                }
                            }
                        }
                        if supported_total == 0 {
                            show_error(
                                AppError::Validation(
                                    "No supported files selected (.md/.txt)".into(),
                                ),
                            );
                            return;
                        }
                        set_import_done.set(0);
                        set_import_total.set(supported_total);
                        set_importing.set(true);
                        let completed: Rc<RefCell<u32>> = Rc::new(RefCell::new(0));
                        let graphrag_ctx_after = graphrag_ctx.clone();
                        let total_supported = supported_total;
                        let mut handled: u32 = 0;
                        for i in 0..len {
                            if let Some(file) = files.item(i) {
                                let name = file.name();
                                let mime = file.type_();
                                let is_text = name.ends_with(".md") || name.ends_with(".markdown")
                                    || name.ends_with(".txt") || mime == "text/markdown"
                                    || mime == "text/plain";
                                if !is_text {
                                    continue;
                                }
                                let set_json_text = set_json_text;
                                let set_error_msg = set_error_msg;
                                let set_success_msg = set_success_msg;
                                let set_import_done = set_import_done;
                                let set_importing = set_importing;
                                let json_text = json_text;
                                let completed_cl = completed.clone();
                                let graphrag_ctx_done = graphrag_ctx_after.clone();
                                leptos::task::spawn_local(async move {
                                    match JsFuture::from(file.text()).await {
                                        Ok(js_val) => {
                                            let content = js_val.as_string().unwrap_or_default();
                                            let mut current = json_text.get_untracked();
                                            if !current.is_empty() {
                                                current.push_str("\n\n---\n\n");
                                            }
                                            current
                                                .push_str(&format!("# File: {}\n\n{}", name, content));
                                            set_json_text.set(current);
                                            let _ = StorageUtils::store_local(
                                                "knowledge_upload_buffer_v1",
                                                &json_text.get_untracked(),
                                            );
                                            set_error_msg.set(None);
                                            set_success_msg.set(Some(format!("Loaded: {}", name)));
                                            web_sys::console::log_1(
                                                &format!("Markdown upload: loaded {}", name).into(),
                                            );
                                        }
                                        Err(e) => {
                                            set_success_msg.set(None);
                                            set_error_msg
                                                .set(Some(format!("Failed to read {}: {:?}", name, e)));
                                            web_sys::console::error_1(
                                                &format!("Markdown upload: failed {} -> {:?}", name, e)
                                                    .into(),
                                            );
                                        }
                                    }
                                    let mut done = completed_cl.borrow_mut();
                                    *done += 1;
                                    set_import_done.set(*done);
                                    if *done == total_supported && total_supported > 0 {
                                        set_importing.set(false);
                                        if let Some(ctx) = graphrag_ctx_done.clone() {
                                            ctx.reindex();
                                        }
                                        set_success_msg.set(None);
                                    }
                                });
                                handled += 1;
                            }
                        }
                        target.set_value("");
                        web_sys::console::log_1(
                            &format!(
                                "Markdown upload: started reading {} supported file(s)",
                                handled,
                            )
                                .into(),
                        );
                    }
                }
            />
        </div>
    }
}
