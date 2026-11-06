//! Reusable, accessible Dioxus primitives and native form controls.

use dioxus::prelude::*;

fn described_by(id: &str, has_help: bool, has_error: bool) -> Option<String> {
    match (has_help, has_error) {
        (true, true) => Some(format!("{id}-help {id}-error")),
        (true, false) => Some(format!("{id}-help")),
        (false, true) => Some(format!("{id}-error")),
        (false, false) => None,
    }
}

#[component]
pub fn MoonButton(
    label: String,
    #[props(default)] disabled: bool,
    onclick: Option<EventHandler<MouseEvent>>,
) -> Element {
    rsx! {
        button {
            class: "moon-button",
            r#type: "button",
            disabled,
            onclick: move |event| {
                if let Some(handler) = &onclick {
                    handler.call(event);
                }
            },
            "{label}"
        }
    }
}

#[component]
pub fn MoonCard(title: String, description: String) -> Element {
    rsx! {
        section {
            class: "moon-card",
            h3 { "{title}" }
            p { "{description}" }
        }
    }
}

#[component]
pub fn MoonBadge(label: String) -> Element {
    rsx! { span { class: "moon-badge", "{label}" } }
}

#[component]
pub fn MoonSeparator() -> Element {
    rsx! { hr { class: "moon-separator" } }
}

#[component]
pub fn MoonSpinner(
    #[props(default = "Loading".to_string())] label: String,
) -> Element {
    rsx! {
        span {
            class: "moon-spinner",
            role: "status",
            aria_label: "{label}",
            span { class: "moon-spinner__visual", aria_hidden: "true" }
            span { class: "moon-visually-hidden", "{label}" }
        }
    }
}

#[component]
pub fn MoonVisuallyHidden(children: Element) -> Element {
    rsx! { span { class: "moon-visually-hidden", {children} } }
}

#[component]
pub fn MoonLabel(
    for_id: String,
    label: String,
    #[props(default)] required: bool,
) -> Element {
    rsx! {
        label { class: "moon-field__label", r#for: "{for_id}",
            "{label}"
            if required { span { class: "moon-field__required", " (required)" } }
        }
    }
}

#[component]
pub fn MoonInput(
    id: String,
    label: String,
    value: String,
    #[props(default)] required: bool,
    #[props(default)] disabled: bool,
    #[props(default)] invalid: bool,
    help_text: Option<String>,
    error_text: Option<String>,
    oninput: Option<EventHandler<FormEvent>>,
) -> Element {
    let described_by = described_by(&id, help_text.is_some(), error_text.is_some());

    rsx! {
        div { class: "moon-field",
            MoonLabel {
                for_id: id.clone(),
                label: label.clone(),
                required
            }
            input {
                id: "{id}",
                class: if invalid { "moon-input moon-input--invalid" } else { "moon-input" },
                r#type: "text",
                value: "{value}",
                required,
                disabled,
                aria_invalid: invalid,
                aria_describedby: described_by,
                oninput: move |event| {
                    if let Some(handler) = &oninput {
                        handler.call(event);
                    }
                }
            }
            if let Some(help) = help_text {
                p { class: "moon-field__help", id: "{id}-help", "{help}" }
            }
            if let Some(error) = error_text {
                p { class: "moon-field__error", id: "{id}-error", role: "alert", "{error}" }
            }
        }
    }
}

#[component]
pub fn MoonTextarea(
    id: String,
    label: String,
    value: String,
    #[props(default = 3)] rows: u32,
    #[props(default)] required: bool,
    #[props(default)] disabled: bool,
    #[props(default)] invalid: bool,
    help_text: Option<String>,
    error_text: Option<String>,
    oninput: Option<EventHandler<FormEvent>>,
) -> Element {
    let described_by = described_by(&id, help_text.is_some(), error_text.is_some());

    rsx! {
        div { class: "moon-field",
            label { class: "moon-field__label", r#for: "{id}",
                "{label}"
                if required { span { class: "moon-field__required", " (required)" } }
            }
            textarea {
                id: "{id}",
                class: if invalid { "moon-input moon-input--invalid" } else { "moon-input" },
                value: "{value}",
                rows,
                required,
                disabled,
                aria_invalid: invalid,
                aria_describedby: described_by,
                oninput: move |event| {
                    if let Some(handler) = &oninput {
                        handler.call(event);
                    }
                }
            }
            if let Some(help) = help_text {
                p { class: "moon-field__help", id: "{id}-help", "{help}" }
            }
            if let Some(error) = error_text {
                p { class: "moon-field__error", id: "{id}-error", role: "alert", "{error}" }
            }
        }
    }
}

#[component]
pub fn MoonCheckbox(
    id: String,
    label: String,
    #[props(default)] checked: bool,
    #[props(default)] required: bool,
    #[props(default)] disabled: bool,
    onchange: Option<EventHandler<FormEvent>>,
) -> Element {
    rsx! {
        label { class: "moon-checkbox", r#for: "{id}",
            input {
                id: "{id}",
                r#type: "checkbox",
                checked,
                required,
                disabled,
                onchange: move |event| {
                    if let Some(handler) = &onchange {
                        handler.call(event);
                    }
                }
            }
            span { "{label}" }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::described_by;

    #[test]
    fn described_by_links_help_and_error_messages() {
        assert_eq!(
            described_by("profile", true, true),
            Some("profile-help profile-error".to_string())
        );
    }

    #[test]
    fn described_by_links_help_only() {
        assert_eq!(described_by("profile", true, false), Some("profile-help".to_string()));
    }

    #[test]
    fn described_by_links_error_only() {
        assert_eq!(described_by("profile", false, true), Some("profile-error".to_string()));
    }

    #[test]
    fn described_by_is_absent_without_messages() {
        assert_eq!(described_by("profile", false, false), None);
    }
}
