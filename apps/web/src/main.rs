mod app;
mod routes;

use dioxus::prelude::*;

fn main() {
    dioxus::launch(app::App);
}
