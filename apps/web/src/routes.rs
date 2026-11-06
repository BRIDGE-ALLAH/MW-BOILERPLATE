use dioxus::prelude::*;
use mw_ui::{MoonBadge, MoonButton, MoonCard, MoonInput, MoonSeparator, MoonSpinner, MoonTextarea};

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
                nav { class: "sidebar", aria_label: "Primary navigation",
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
            aria_label: "Toggle color theme",
            aria_pressed: dark(),
            onclick: move |_| {
                let next = !dark();
                dark.set(next);
                document::eval(if next {
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
            Link { to: Route::Components {}, class: "moon-button", "Explore components" }
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
            MoonCard { title: "Button", description: "Native action primitive with disabled state." }
            MoonCard { title: "Card", description: "Surface for grouping related content." }
            MoonCard { title: "Badge", description: "Compact status text that remains readable without color." }
            MoonCard { title: "Separator", description: "Semantic divider between content groups." }
            MoonCard { title: "Spinner", description: "Accessible indeterminate progress indicator." }
        }
        MoonSeparator {}
        h2 { "Status and progress" }
        div { class: "actions",
            MoonBadge { label: "Foundation ready" }
            MoonSpinner { label: "Example progress status" }
        }
        Link { to: Route::ButtonPage {}, "Button documentation →" }
        Link { to: Route::InputPage {}, "Form controls →" }
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
    let mut notes = use_signal(String::new);
    rsx! {
        h1 { "Form controls" }
        p { class: "lead", "Reusable native controls with explicit labels, controlled values, and optional validation messaging." }
        MoonInput {
            id: "demo-input",
            label: "Display name",
            value: "{value}",
            help_text: Some("This value is held in page state.".to_string()),
            oninput: move |event| value.set(event.value())
        }
        p { "Current value: {value}" }
        MoonTextarea {
            id: "demo-notes",
            label: "Notes",
            value: "{notes}",
            rows: 4,
            help_text: Some("A native multi-line text control.".to_string()),
            oninput: move |event| notes.set(event.value())
        }
        p { "Character count: {notes.chars().count()}" }
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
