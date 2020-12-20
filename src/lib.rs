#![recursion_limit = "1024"]
use wasm_bindgen::prelude::*;

mod app_router;

mod routes;

use app_router::AppRouter;

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen(start)]
pub fn run_app() {
    yew::start_app::<AppRouter>();
}
