# Moon UI Components

## Implemented source APIs

The following components are currently exported by `crates/ui`. Their source has been committed, but compilation and browser behavior remain unverified until the Rust/Dioxus toolchain gate can run.

| Component | Current API | Behavior |
|---|---|---|
| MoonButton | `label: String`, optional `disabled: bool`, optional `onclick: EventHandler<MouseEvent>` | Native button, default type button, disabled state, optional click callback |
| MoonCard | `title: String, description: String` | Section surface with heading and description |
| MoonBadge | `label: String` | Neutral status text, not color-dependent |
| MoonSeparator | no props | Native horizontal rule |
| MoonSpinner | optional `label: String` | Indeterminate status indicator; default label Loading |
| MoonVisuallyHidden | `children` | Visually hidden content retained for assistive technology |
| MoonLabel | `for_id: String, label: String`, optional `required: bool` | Native associated label with visible required marker |
| MoonInput | `id, label, value`, optional `required, disabled, invalid, help_text, error_text, oninput` | Controlled native text input with associated label and optional descriptions |
| MoonTextarea | `id, label, value`, optional `rows, required, disabled, invalid, help_text, error_text, oninput` | Controlled native textarea with associated label and optional descriptions |
| MoonSwitch | `id, label`, optional `checked, disabled, onchange` | Native checkbox exposed with switch role and native keyboard behavior |
| MoonSelect | `id, label, value, options`, optional `placeholder, required, disabled, onchange` | Controlled native select with keyboard behavior and associated label |
| MoonCheckbox | `id, label`, optional `checked, required, disabled, onchange` | Native checkbox nested in its associated label |

The component showcase uses MoonBadge, MoonSeparator, MoonSpinner, MoonInput, MoonTextarea, MoonCheckbox, MoonSwitch, and MoonSelect. MoonLabel is also used internally by MoonInput and MoonTextarea. MoonVisuallyHidden is exported for reuse but does not yet have a dedicated showcase example.

## Usage examples

```rust
MoonButton { label: "Save".to_string() }
MoonButton { label: "Unavailable".to_string(), disabled: true }
MoonCard { title: "Title", description: "Description" }
MoonBadge { label: "Ready" }
MoonSeparator {}
MoonSpinner { label: "Loading report" }
// MoonInput and MoonTextarea render their associated MoonLabel internally.
MoonInput {
    id: "display-name",
    label: "Display name",
    value: "{value}",
    oninput: move |event| value.set(event.value())
}
MoonTextarea {
    id: "notes",
    label: "Notes",
    value: "{notes}",
    rows: 4,
    oninput: move |event| notes.set(event.value())
}
MoonCheckbox { id: "terms", label: "Accept terms" }
MoonSwitch { id: "beta", label: "Enable beta" }
MoonSelect {
    id: "plan",
    label: "Plan",
    value: "starter",
    options: vec![MoonSelectOption { label: "Starter".to_string(), value: "starter".to_string() }]
}
```

## Known limitations

- MoonButton has no variant, size, loading state, or click callback prop yet.
- MoonInput is currently text-only; specialized email/number/password semantics are not exposed.
- MoonTextarea and MoonInput rely on the caller to own validation and state.
- MoonCheckbox exposes a native change event but no custom form-state abstraction.
- Select, Switch, Alert, Toast, Tooltip, Dialog, Tabs, and layout primitives remain deferred.
- Component render tests and browser E2E are not configured or executed yet.

## Implementation requirements

Every interactive component needs a documented public API, defaults, accessible naming, keyboard interaction, disabled behavior, focus behavior, CSS ownership, usage example, and tests. Prefer native HTML semantics. Avoid duplicating validation or product-specific business rules in generic controls.

Dialog remains gated on focus entry/return, Escape behavior, accessible naming, and keyboard tests. Do not replace the existing honest placeholder with a non-functional visual mock.
