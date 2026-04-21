//! **mai** — An AI-integrated e-commerce system.
//!
//! This crate is compiled as both a `cdylib` (for WASM hydration) and an `rlib`
//! (for the SSR server binary). Feature flags `hydrate` and `ssr` control which
//! code paths are included in each target.

pub mod ai;
pub mod app;
pub mod models;
pub mod server;

// ── Client-side hydration entry point ───────────────────────────────────────

#[cfg(feature = "hydrate")]
#[wasm_bindgen::prelude::wasm_bindgen]
pub fn hydrate() {
    use crate::app::App;
    console_error_panic_hook::set_once();
    leptos::mount::hydrate_body(App);
}
