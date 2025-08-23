use log::{error, info};
use wasm_bindgen::JsValue;
use wasm_bindgen_futures::spawn_local;
use gloo_timers::callback::Interval;
use std::cell::RefCell;
use std::rc::Rc;

use crate::models::webllm::{LLMModel, ModelStatus};
use crate::state::webllm_state_simple::WebLLMStateContext;

/// Initialize a WebLLM model with progress updates wired into WebLLMState
pub fn init_model(ctx: WebLLMStateContext, model: LLMModel) {
    // Set initial state
    ctx.set_current_model(Some(model.clone()));
    ctx.set_model_status(ModelStatus::Loading { progress: 0.0 });
    ctx.set_initialization_progress(0.0);

    // Try real WebLLM init; fall back to simulated progress if it errors
    spawn_local(async move {
        let model_id = model.id.clone();
        info!("Starting WebLLM init for {}", model_id);

        let state_for_progress = ctx.clone();
        let progress_cb = move |text: String, p: f64| {
            // Map the text to statuses if desired; we just store progress
            let progress = p.clamp(0.0, 1.0) as f32;
            state_for_progress.set_model_status(ModelStatus::Loading { progress });
            state_for_progress.set_initialization_progress(progress);
            log::debug!("webllm progress: {} => {:.0}%", text, progress * 100.0);
        };

        // Attempt to call into JS binding
        let res: Result<JsValue, JsValue> = crate::webllm_binding::init_webllm_with_progress(
            &model_id,
            progress_cb,
        )
        .await;

        match res {
            Ok(_engine) => {
                ctx.set_model_status(ModelStatus::Ready);
                ctx.set_initialization_progress(1.0);
                info!("WebLLM init finished for {}", model_id);
            }
            Err(e) => {
                error!("WebLLM init failed for {}: {:?}. Falling back to simulated init.", model_id, e);
                simulate_progress(ctx.clone());
            }
        }
    });
}

/// Simulate initialization progress for environments without WebLLM JS available
pub fn simulate_progress(ctx: WebLLMStateContext) {
    ctx.set_model_status(ModelStatus::Loading { progress: 0.0 });
    ctx.set_initialization_progress(0.0);

    // Tick progress every ~120ms to ~3s total
    let mut i = 0u32;

    // Keep a handle so we can stop the interval when done
    let handle_slot: Rc<RefCell<Option<Interval>>> = Rc::new(RefCell::new(None));
    let handle_slot_cloned = handle_slot.clone();

    let interval = Interval::new(120, move || {
        i += 1;
        let p = ((i as f32) / 25.0).min(1.0); // 0..1 over ~3s
        ctx.set_initialization_progress(p);
        ctx.set_model_status(ModelStatus::Loading { progress: p });

        if (p - 1.0).abs() < f32::EPSILON {
            ctx.set_model_status(ModelStatus::Ready);
            // Stop the interval by dropping the handle
            if let Some(handle) = handle_slot_cloned.borrow_mut().take() {
                drop(handle);
            }
        }
    });

    // Store the handle so closure can drop it later
    *handle_slot.borrow_mut() = Some(interval);
}
