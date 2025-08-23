//! Test utilities for wasm-based tests

#![cfg(target_arch = "wasm32")]

use wasm_bindgen_test::*;

wasm_bindgen_test::wasm_bindgen_test_configure!(run_in_browser);

/// Simple performance timer helper
pub struct PerfTimer {
    label: &'static str,
    start: f64,
}

impl PerfTimer {
    pub fn start(label: &'static str) -> Self {
        let start = js_sys::Date::now();
        Self { label, start }
    }
    pub fn end(self) -> f64 {
        let end = js_sys::Date::now();
        let ms = end - self.start;
        web_sys::console::log_1(&format!("[perf] {}: {:.2}ms", self.label, ms).into());
        ms
    }
}

/// DOM helper: create a container div for rendering
pub fn create_test_container(id: &str) -> web_sys::Element {
    let document = web_sys::window().unwrap().document().unwrap();
    let el = document.create_element("div").unwrap();
    el.set_id(id);
    document.body().unwrap().append_child(&el).unwrap();
    el
}

/// Assertion with tolerance for floats
pub fn assert_approx_eq(a: f64, b: f64, tol: f64) {
    let diff = (a - b).abs();
    assert!(diff <= tol, "expected ~{} got {} (tol={})", b, a, tol);
}

#[wasm_bindgen_test]
fn utils_smoke() {
    let _container = create_test_container("test-root");
    let t = PerfTimer::start("smoke");
    let elapsed = t.end();
    assert!(elapsed >= 0.0);
    assert_approx_eq(1.0, 1.0 + 1e-9, 1e-6);
}
