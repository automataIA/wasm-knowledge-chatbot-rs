use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = lucide)]
    fn createIcons();
    
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

pub fn render_lucide_icons() {
    // Try to render icons with error handling
    if let Some(window) = web_sys::window() {
        match js_sys::Reflect::get(&window, &"lucide".into()) {
            Ok(lucide) => {
                match js_sys::Reflect::get(&lucide, &"createIcons".into()) {
                    Ok(create_icons_fn) => {
                        let _ = js_sys::Function::from(create_icons_fn).call0(&lucide);
                    }
                    Err(_) => {
                        log("Lucide createIcons function not found");
                    }
                }
            }
            Err(_) => {
                log("Lucide library not loaded");
            }
        }
    }
}

pub fn schedule_icon_render() {
    wasm_bindgen_futures::spawn_local(async {
        // Wait a bit for DOM to be ready
        gloo_timers::future::TimeoutFuture::new(100).await;
        render_lucide_icons();
    });
}