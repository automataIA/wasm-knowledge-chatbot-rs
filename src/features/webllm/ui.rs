use crate::features::webllm::service::{init_model, simulate_progress};
use crate::models::webllm::{LLMModel, ModelCapability, ModelStatus};
use crate::state::webllm_state_simple::use_webllm_state;
use crate::utils::storage::StorageUtils;
use js_sys::{Array, Object, Reflect};
use leptos::prelude::*;
use serde::{Deserialize, Serialize};
use serde_json;
use wasm_bindgen::{JsCast, JsValue};
use web_sys::window;

#[component]
pub fn WebLLMInitPanel() -> impl IntoView {
    let ctx = use_webllm_state();

    // Helper: fetch models from window.webllm.prebuiltAppConfig.model_list if present
    fn fetch_prebuilt_models_from_js() -> Option<Vec<LLMModel>> {
        let win = window()?;
        let webllm = Reflect::get(&win, &JsValue::from_str("webllm")).ok()?;
        let prebuilt = Reflect::get(&webllm, &JsValue::from_str("prebuiltAppConfig")).ok()?;
        let list = Reflect::get(&prebuilt, &JsValue::from_str("model_list")).ok()?;
        if !list.is_object() {
            return None;
        }
        let arr: Array = list.dyn_into().ok()?;
        let mut out = Vec::new();
        for v in arr.iter() {
            if let Ok(obj) = v.dyn_into::<Object>() {
                // Try common keys to derive an id and name
                let id = Reflect::get(&obj, &JsValue::from_str("model_id"))
                    .ok()
                    .and_then(|x| x.as_string())
                    .or_else(|| {
                        Reflect::get(&obj, &JsValue::from_str("id"))
                            .ok()
                            .and_then(|x| x.as_string())
                    })
                    .or_else(|| {
                        Reflect::get(&obj, &JsValue::from_str("model"))
                            .ok()
                            .and_then(|x| x.as_string())
                    });
                if let Some(id) = id {
                    let name = Reflect::get(&obj, &JsValue::from_str("name"))
                        .ok()
                        .and_then(|x| x.as_string())
                        .unwrap_or_else(|| id.clone());
                    let family = Reflect::get(&obj, &JsValue::from_str("family"))
                        .ok()
                        .and_then(|x| x.as_string())
                        .unwrap_or_else(|| "webllm".to_string());
                    out.push(
                        LLMModel::new(id, name, "WebLLM".to_string(), family)
                            .with_capabilities(vec![ModelCapability::TextGeneration]),
                    );
                }
            }
        }
        if out.is_empty() {
            None
        } else {
            Some(out)
        }
    }

    // Seed available models on first render if empty, prefer JS prebuilt list
    Effect::new({
        let ctx = ctx.clone();
        move |_| {
            if ctx.get_available_models().is_empty() {
                if let Some(models) = fetch_prebuilt_models_from_js() {
                    ctx.set_available_models(models);
                } else {
                    let models = vec![
                        LLMModel::new(
                            "Llama-3.2-1B-Instruct-q4f32_1-MLC".to_string(),
                            "Llama 3.2 1B Instruct".to_string(),
                            "WebLLM".to_string(),
                            "llama3".to_string(),
                        )
                        .with_capabilities(vec![ModelCapability::TextGeneration])
                        .with_size(350),
                        LLMModel::new(
                            "Phi-3.5-mini-instruct-q4f16_1-MLC".to_string(),
                            "Phi 3.5 Mini".to_string(),
                            "WebLLM".to_string(),
                            "phi3".to_string(),
                        )
                        .with_capabilities(vec![ModelCapability::TextGeneration])
                        .with_size(250),
                        LLMModel::new(
                            "Qwen2.5-0.5B-Instruct-q4f16_1-MLC".to_string(),
                            "Qwen2.5 0.5B Instruct".to_string(),
                            "WebLLM".to_string(),
                            "qwen2.5".to_string(),
                        )
                        .with_capabilities(vec![ModelCapability::TextGeneration])
                        .with_size(220),
                        LLMModel::new(
                            "Qwen2.5-1.5B-Instruct-q4f16_1-MLC".to_string(),
                            "Qwen2.5 1.5B Instruct".to_string(),
                            "WebLLM".to_string(),
                            "qwen2.5".to_string(),
                        )
                        .with_capabilities(vec![ModelCapability::TextGeneration])
                        .with_size(500),
                    ];
                    ctx.set_available_models(models);
                }
            }
        }
    });

    let (selected, set_selected) = signal(String::new());

    // Persist key for last-used model id
    const LAST_MODEL_KEY: &str = "webllm_last_model_id";

    // Load last-used model id on mount
    Effect::new({
        move |_| {
            if let Ok(Some(id)) = StorageUtils::retrieve_local::<String>(LAST_MODEL_KEY) {
                if !id.trim().is_empty() {
                    set_selected.set(id);
                }
            }
        }
    });

    // Keep local selected in sync when the active chat model changes programmatically
    Effect::new({
        let ctx = ctx.clone();
        move |_| {
            if let Some(m) = ctx.get_current_model() {
                if selected.get_untracked() != m.id {
                    set_selected.set(m.id);
                }
            }
        }
    });

    // Advanced: custom models persistence
    #[derive(Clone, Debug, Serialize, Deserialize)]
    struct CustomModelEntry {
        model: String,
        model_lib: String,
        model_id: String,
        name: Option<String>,
        family: Option<String>,
        size_mb: Option<u32>,
    }

    let (adv_open, set_adv_open) = signal(false);
    let (cm_model, set_cm_model) = signal(String::new());
    let (cm_model_lib, set_cm_model_lib) = signal(String::new());
    let (cm_model_id, set_cm_model_id) = signal(String::new());
    let (cm_name, set_cm_name) = signal(String::new());
    let (cm_family, set_cm_family) = signal(String::from("custom"));
    let (cm_size, set_cm_size) = signal(String::new());

    // Load saved custom models and merge into available on mount
    Effect::new({
        let ctx = ctx.clone();
        move |_| {
            if let Some(win) = window() {
                if let Ok(Some(storage)) = win.local_storage() {
                    if let Ok(Some(raw)) = storage.get_item("webllm_custom_models") {
                        if !raw.is_empty() {
                            if let Ok(entries) = serde_json::from_str::<Vec<CustomModelEntry>>(&raw)
                            {
                                if !entries.is_empty() {
                                    let mut avail = ctx.get_available_models();
                                    for e in entries {
                                        let name =
                                            e.name.clone().unwrap_or_else(|| e.model_id.clone());
                                        let family = e
                                            .family
                                            .clone()
                                            .unwrap_or_else(|| "custom".to_string());
                                        let mut m = LLMModel::new(
                                            e.model_id.clone(),
                                            name,
                                            "WebLLM".to_string(),
                                            family,
                                        )
                                        .with_capabilities(vec![ModelCapability::TextGeneration]);
                                        if let Some(sz) = e.size_mb {
                                            m = m.with_size(sz);
                                        }
                                        // Avoid duplicates by id
                                        if !avail.iter().any(|am| am.id == m.id) {
                                            avail.push(m);
                                        }
                                    }
                                    ctx.set_available_models(avail);
                                }
                            }
                        }
                    }
                }
            }
        }
    });

    // Derived values for UI
    let status = Signal::derive({
        let ctx = ctx.clone();
        move || ctx.get_model_status()
    });

    let progress_pct = Signal::derive({
        let ctx = ctx.clone();
        move || (ctx.get_initialization_progress() * 100.0).round() as i32
    });

    let is_initializing = Signal::derive({
        move || {
            matches!(
                status.get(),
                ModelStatus::Downloading { .. } | ModelStatus::Loading { .. }
            )
        }
    });

    let status_text = Signal::derive({
        move || match status.get() {
            ModelStatus::NotInitialized => "Not initialized".to_string(),
            ModelStatus::Downloading { .. } | ModelStatus::Loading { .. } => {
                format!("Loading {}%", progress_pct.get())
            }
            ModelStatus::Ready => "Ready".to_string(),
            ModelStatus::Error { .. } => "Error".to_string(),
        }
    });

    let status_badge_class = Signal::derive({
        move || match status.get() {
            ModelStatus::Ready => "badge badge-success badge-outline".to_string(),
            ModelStatus::Downloading { .. } | ModelStatus::Loading { .. } => {
                "badge badge-info badge-outline".to_string()
            }
            ModelStatus::Error { .. } => "badge badge-error badge-outline".to_string(),
            ModelStatus::NotInitialized => "badge".to_string(),
        }
    });

    let available = Signal::derive({
        let ctx = ctx.clone();
        move || ctx.get_available_models()
    });
    // Store commonly used values to prevent moving them into event handlers
    let ctx_sv = StoredValue::new(ctx.clone());
    let available_sv = StoredValue::new(available);

    // One-time auto-init guard
    let (auto_init_done, set_auto_init_done) = signal(false);

    // Auto-initialize the selected (or first) model once models are available and not initialized yet
    Effect::new({
        let ctx = ctx_sv.get_value().clone();
        move |_| {
            if auto_init_done.get() {
                return;
            }
            let models = available_sv.get_value().get();
            if models.is_empty() {
                return;
            }
            if matches!(ctx.get_model_status(), ModelStatus::NotInitialized) {
                // Prefer explicitly selected; else first available
                let chosen = models
                    .iter()
                    .find(|m| m.id == selected.get())
                    .cloned()
                    .or_else(|| models.first().cloned());
                if let Some(m) = chosen {
                    set_auto_init_done.set(true);
                    // Persist auto-chosen model id
                    let _ = StorageUtils::store_local(LAST_MODEL_KEY, &m.id);
                    // Reflect in the UI select as well
                    set_selected.set(m.id.clone());
                    init_model(ctx.clone(), m);
                }
            }
        }
    });

    // Actions handled inline to avoid moving closures that make the view FnOnce

    view! {
        <div class="p-3 border border-base-300 rounded-lg bg-base-100 w-full max-w-full min-w-0 overflow-x-clip">
            <div class="flex items-center gap-2 w-full min-w-0">
                <div class="flex-1 min-w-0">
                    <select
                        class="select select-bordered select-sm rounded-lg w-full"
                        prop:value=move || selected.get()
                        on:change=move |ev| {
                            let v = event_target_value(&ev);
                            set_selected.set(v.clone());
                            let _ = StorageUtils::store_local(LAST_MODEL_KEY, &v);
                            // Immediately initialize the chosen model so StatusBar reflects it
                            let available_sig = available_sv.get_value();
                            if let Some(model) = available_sig
                                .get()
                                .into_iter()
                                .find(|m| m.id == v)
                            {
                                init_model(ctx_sv.get_value().clone(), model);
                            }
                        }
                    >
                        <option value="">{"Select model"}</option>
                        {move || {
                            available
                                .get()
                                .into_iter()
                                .map(|m| view! { <option value=m.id.clone()>{m.name}</option> })
                                .collect_view()
                        }}
                    </select>
                </div>

                <div class="flex items-center gap-2">
                    <div
                        class="tooltip"
                        data-tip=move || {
                            if is_initializing.get() {
                                "Initializing...".to_string()
                            } else {
                                "Initialize model".to_string()
                            }
                        }
                    >
                        <button
                            class="btn btn-primary btn-sm btn-square"
                            on:click=move |_| {
                                let available_sig = available_sv.get_value();
                                let model = available_sig
                                    .get()
                                    .into_iter()
                                    .find(|m| m.id == selected.get())
                                    .or_else(|| available_sig.get().into_iter().next());
                                let ctx_local = ctx_sv.get_value().clone();
                                if let Some(model) = model {
                                    set_selected.set(model.id.clone());
                                    init_model(ctx_local, model);
                                } else {
                                    simulate_progress(ctx_sv.get_value().clone());
                                }
                            }
                            disabled=move || is_initializing.get()
                        >
                            <svg
                                xmlns="http://www.w3.org/2000/svg"
                                class="h-4 w-4"
                                viewBox="0 0 24 24"
                                fill="none"
                                stroke="currentColor"
                                stroke-width="2"
                                stroke-linecap="round"
                                stroke-linejoin="round"
                            >
                                <polygon points="5 3 19 12 5 21 5 3" />
                            </svg>
                        </button>
                    </div>

                    <div
                        class="tooltip"
                        data-tip=move || {
                            if adv_open.get() {
                                "Hide advanced".to_string()
                            } else {
                                "Show advanced".to_string()
                            }
                        }
                    >
                        <button
                            class="btn btn-outline btn-sm btn-square"
                            on:click=move |_| set_adv_open.update(|v| *v = !*v)
                        >
                            <svg
                                xmlns="http://www.w3.org/2000/svg"
                                class="h-4 w-4"
                                viewBox="0 0 24 24"
                                fill="none"
                                stroke="currentColor"
                                stroke-width="2"
                                stroke-linecap="round"
                                stroke-linejoin="round"
                            >
                                <line x1="4" y1="21" x2="4" y2="14" />
                                <line x1="4" y1="10" x2="4" y2="3" />
                                <line x1="12" y1="21" x2="12" y2="12" />
                                <line x1="12" y1="8" x2="12" y2="3" />
                                <line x1="20" y1="21" x2="20" y2="16" />
                                <line x1="20" y1="12" x2="20" y2="3" />
                                <line x1="2" y1="14" x2="6" y2="14" />
                                <line x1="10" y1="8" x2="14" y2="8" />
                                <line x1="18" y1="16" x2="22" y2="16" />
                            </svg>
                        </button>
                    </div>
                </div>
            </div>

            <div class="mt-3 flex items-center gap-2 min-w-0">
                <span class=move || status_badge_class.get()>{move || status_text.get()}</span>
            </div>

            <div class="mt-4">
                <Show when=move || adv_open.get()>
                    <div class="mt-3 p-3 rounded-lg border border-base-300 bg-base-200/40 space-y-2 max-w-full min-w-0 overflow-x-clip">
                        <div class="grid grid-cols-1 md:grid-cols-2 gap-2 max-w-full min-w-0">
                            <input
                                class="input input-bordered input-sm rounded-lg"
                                type="text"
                                placeholder="Model URL (artifacts)"
                                on:input=move |ev| set_cm_model.set(event_target_value(&ev))
                            />
                            <input
                                class="input input-bordered input-sm rounded-lg"
                                type="text"
                                placeholder="Model Lib URL (.wasm)"
                                on:input=move |ev| set_cm_model_lib.set(event_target_value(&ev))
                            />
                            <input
                                class="input input-bordered input-sm rounded-lg"
                                type="text"
                                placeholder="Model ID (e.g., MyLlama-3b-q4f32_0)"
                                on:input=move |ev| set_cm_model_id.set(event_target_value(&ev))
                            />
                            <input
                                class="input input-bordered input-sm rounded-lg"
                                type="text"
                                placeholder="Display Name (optional)"
                                on:input=move |ev| set_cm_name.set(event_target_value(&ev))
                            />
                            <input
                                class="input input-bordered input-sm rounded-lg"
                                type="text"
                                placeholder="Family (optional, e.g., llama3)"
                                on:input=move |ev| set_cm_family.set(event_target_value(&ev))
                            />
                            <input
                                class="input input-bordered input-sm rounded-lg"
                                type="number"
                                min="0"
                                placeholder="Approx size MB (optional)"
                                on:input=move |ev| set_cm_size.set(event_target_value(&ev))
                            />
                        </div>
                        <div class="flex items-center gap-2">
                            <button
                                class="btn btn-sm"
                                on:click=move |_| {
                                    let id = cm_model_id.get().trim().to_string();
                                    let model = cm_model.get().trim().to_string();
                                    let model_lib = cm_model_lib.get().trim().to_string();
                                    if id.is_empty() || model.is_empty() || model_lib.is_empty() {
                                        return;
                                    }
                                    let name = {
                                        let n = cm_name.get();
                                        if n.trim().is_empty() { id.clone() } else { n }
                                    };
                                    let family = {
                                        let f = cm_family.get();
                                        if f.trim().is_empty() { "custom".to_string() } else { f }
                                    };
                                    let mut m = LLMModel::new(
                                            id.clone(),
                                            name.clone(),
                                            "WebLLM".to_string(),
                                            family.clone(),
                                        )
                                        .with_capabilities(vec![ModelCapability::TextGeneration]);
                                    if let Ok(sz) = cm_size.get().trim().parse::<u32>() {
                                        m = m.with_size(sz);
                                    }
                                    let mut avail = available_sv.get_value().get();
                                    if !avail.iter().any(|am| am.id == id) {
                                        avail.push(m);
                                    }
                                    let ctx2 = ctx_sv.get_value().clone();
                                    ctx2.set_available_models(avail.clone());
                                    let entry = CustomModelEntry {
                                        model,
                                        model_lib,
                                        model_id: id,
                                        name: Some(name),
                                        family: Some(family),
                                        size_mb: cm_size.get().trim().parse().ok(),
                                    };
                                    if let Some(win) = window() {
                                        if let Ok(Some(storage)) = win.local_storage() {
                                            let mut list: Vec<CustomModelEntry> = storage
                                                .get_item("webllm_custom_models")
                                                .ok()
                                                .flatten()
                                                .and_then(|raw| serde_json::from_str(&raw).ok())
                                                .unwrap_or_default();
                                            if !list.iter().any(|e| e.model_id == entry.model_id) {
                                                list.push(entry);
                                            }
                                            if let Ok(raw) = serde_json::to_string(&list) {
                                                let _ = storage.set_item("webllm_custom_models", &raw);
                                            }
                                        }
                                    }
                                }
                            >
                                {"Add custom model"}
                            </button>
                        </div>
                        <p class="text-xs opacity-70">
                            {"Tip: Provide model and model_lib URLs pointing to your MLC-built artifacts and wasm runtime."}
                        </p>
                    </div>
                </Show>
            </div>
        </div>
    }
}
