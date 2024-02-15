#[allow(clippy::single_component_path_imports)]
#[allow(unused_imports)]
use app; // Do NOT remove this: it’s necessary for wasm_bindgen to pick up islands

use leptos::*;
use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen]
pub fn hydrate() {
    // initializes logging using the `log` crate
    _ = console_log::init_with_level(log::Level::Debug);
    console_error_panic_hook::set_once();
    // Provides Query Client for entire app.
    leptos_query::provide_query_client();

    leptos_dom::HydrationCtx::stop_hydrating();
}
