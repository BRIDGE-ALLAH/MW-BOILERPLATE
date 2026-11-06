//! Reusable Dioxus primitives.
use dioxus::prelude::*;
#[component]
pub fn MoonButton(label: String) -> Element { rsx! { button { class: "moon-button", "{label}" } } }
#[component]
pub fn MoonCard(title: String, description: String) -> Element { rsx! { section { class: "moon-card", h3 { "{title}" } p { "{description}" } } } }
