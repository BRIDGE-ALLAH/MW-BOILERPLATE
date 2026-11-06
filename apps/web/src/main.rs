mod app;
mod routes;

use dioxus::prelude::*;

fn main() {
    tracing_wasm::set_as_global_default();
    dioxus::launch(app::App);
}
