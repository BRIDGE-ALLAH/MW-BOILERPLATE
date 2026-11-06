use dioxus::prelude::*;

use crate::routes::Route;

const STYLE: Asset = asset!("/assets/main.css");

#[component]
pub fn App() -> Element {
    rsx! {
        document::Link { rel: "stylesheet", href: STYLE }
        Router::<Route> {}
    }
}
