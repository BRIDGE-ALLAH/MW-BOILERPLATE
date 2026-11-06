use dioxus::prelude::*;
use mw_ui::{MoonButton, MoonCard};

#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[layout(SiteLayout)]
    #[route("/")]
    Home {},
    #[route("/about")]
    About {},
    #[route("/components")]
    Components {},
    #[route("/components/button")]
    ButtonPage {},
    #[route("/components/input")]
    InputPage {},
    #[route("/components/dialog")]
    DialogPage {},
    #[route("/settings")]
    Settings {},
    #[route("/:..segments")]
    NotFound { segments: Vec<String> },
}

#[component]
fn SiteLayout() -> Element {
    rsx! {
        div { class: "shell",
            header { class: "topbar",
                Link { to: Route::Home {}, class: "brand", "MW / BOILERPLATE" }
                nav { "Foundation" }
                ThemeToggle {}
            }
            div { class: "body",
                aside { class: "sidebar",
                    Link { to: Route::Home {}, "Overview" }
                    Link { to: Route::About {}, "About" }
                    Link { to: Route::Components {}, "Components" }
                    Link { to: Route::ButtonPage {}, "Button" }
                    Link { to: Route::InputPage {}, "Input" }
                    Link { to: Route::DialogPage {}, "Dialog" }
                    Link { to: Route::Settings {}, "Settings" }
                }
                main { class: "content", Outlet::<Route> {} }
            }
            footer { "MW-BOILERPLATE · Rust + Dioxus · Web foundation" }
        }
    }
}

#[component]
fn ThemeToggle() -> Element {
    let mut dark = use_signal(|| false);
    rsx! {
        button {
            class: "theme-toggle",
            onclick: move |_| {
                dark.set(!dark());
                document::eval(if dark() {
                    "document.documentElement.dataset.theme = 'dark'"
                } else {
                    "document.documentElement.dataset.theme = 'light'"
                });
            },
            if dark() { "Light mode" } else { "Dark mode" }
        }
    }
}

#[component]
fn Home() -> Element {
    rsx! {
        p { class: "eyebrow", "ENTERPRISE STARTER / 0.1" }
        h1 { "A dependable foundation for Rust applications." }
        p { class: "lead", "A compact, contract-first Dioxus workspace designed to be reused by independent products without coupling their business logic or visual identity." }
        div { class: "actions",
            MoonButton { label: "Explore components" }
            Link { to: Route::About {}, class: "text-link", "Read architecture →" }
        }
        div { class: "card-grid",
            MoonCard { title: "Rust first", description: "Shared types and contracts remain framework-independent." }
            MoonCard { title: "Composable UI", description: "Reusable Dioxus components with neutral CSS tokens." }
            MoonCard { title: "Web baseline", description: "WASM-targeted web app with a path to future platforms." }
        }
        h2 { "Engineering principles" }
        ul {
            li { "Contracts before implementations." }
            li { "Explicit dependency direction and small workspace." }
            li { "Secure defaults and public/private configuration separation." }
            li { "Validation evidence over assumed completion." }
        }
    }
}

#[component]
fn About() -> Element {
    rsx! {
        h1 { "About MW-BOILERPLATE" }
        p { class: "lead", "A reusable technical foundation for MoonWitness, A-L-L-A-H, and future applications." }
        MoonCard { title: "Independent by design", description: "This repository contains no product-specific business rules. Each consuming application owns its domain and brand." }
    }
}

#[component]
fn Components() -> Element {
    rsx! {
        h1 { "Component library" }
        p { class: "lead", "A small initial set of primitives. Expand only with documented APIs, accessibility behavior, and tests." }
        div { class: "card-grid",
            MoonCard { title: "Button", description: "Basic action primitive.", }
            MoonCard { title: "Card", description: "Surface for grouping related content." }
        }
        Link { to: Route::ButtonPage {}, "Button documentation →" }
    }
}

#[component]
fn ButtonPage() -> Element {
    rsx! {
        h1 { "Button" }
        p { class: "lead", "A minimal action component with a text label." }
        MoonButton { label: "Primary action" }
        pre { "MoonButton { label: \"Save changes\" }" }
    }
}

#[component]
fn InputPage() -> Element {
    let mut value = use_signal(String::new);
    rsx! {
        h1 { "Input" }
        p { class: "lead", "Native input example with controlled Dioxus signal state." }
        label { class: "field-label", r#for: "demo-input", "Display name" }
        input {
            id: "demo-input",
            class: "text-input",
            value: "{value}",
            oninput: move |event| value.set(event.value())
        }
        p { "Current value: {value}" }
    }
}

#[component]
fn DialogPage() -> Element {
    rsx! {
        h1 { "Dialog" }
        p { class: "lead", "Dialog is intentionally deferred until keyboard focus management and accessibility behavior can be implemented and tested." }
        MoonCard { title: "Not implemented yet", description: "This route documents the planned component without presenting a non-functional mock." }
    }
}

#[component]
fn Settings() -> Element {
    rsx! {
        h1 { "Settings" }
        p { class: "lead", "Starter preferences and appearance." }
        MoonCard { title: "Appearance", description: "Use the theme control in the header to switch between light and dark CSS tokens." }
    }
}

#[component]
fn NotFound(segments: Vec<String>) -> Element {
    rsx! {
        h1 { "404 — Page not found" }
        p { "No route matches /{segments.join(\"/\")}." }
        Link { to: Route::Home {}, "Return to overview →" }
    }
}
