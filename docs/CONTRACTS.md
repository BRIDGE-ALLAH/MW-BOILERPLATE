# Contracts

This document inventories current public interfaces. The [Product Blueprint](PRODUCT-BLUEPRINT.md) describes planned boundaries; planned interfaces below are not current implementations.

## Core crate

### Identifier

- Type: `Identifier(String)`
- Constructor: `Identifier::new(value: impl Into<String>) -> Result<Identifier, CoreError>`
- Validation: rejects values whose trimmed representation is empty; preserves the supplied non-empty string.
- Accessor: `as_str() -> &str`

### CoreError

- Current variant: `InvalidIdentifier`
- Display text: `invalid identifier`

## Contracts crate

All DTOs derive Serde serialization/deserialization and do not depend on Dioxus, HTTP transport, or a database.

### PageRequest

| Field | Type | Default |
|---|---|---|
| page | u32 | 1 |
| per_page | u32 | 20 |

### Page<T>

| Field | Type |
|---|---|
| items | Vec<T> |
| page | u32 |
| per_page | u32 |
| total_items | u64 |

### ApiError

| Field | Type |
|---|---|
| code | String |
| message | String |
| request_id | Option<String> |

The DTO does not currently enforce a code vocabulary or validate message contents. Consumers must not treat arbitrary server messages as safe HTML or expose internal diagnostics.

## UI crate

The current exported source APIs are:
- `MoonButton { label: String, disabled?: bool, onclick?: EventHandler<MouseEvent> }`: native button; default type button.
- `MoonCard { title: String, description: String }`: section surface with heading and description.
- `MoonBadge { label: String }`: neutral status text.
- `MoonSeparator {}`: semantic horizontal rule.
- `MoonSpinner { label?: String }`: status indicator with accessible label.
- `MoonVisuallyHidden { children }`: visually hidden accessible content.
- `MoonLabel { for_id, label, required? }`: reusable associated label.
- `MoonInput { id, label, value, required?, disabled?, invalid?, help_text?, error_text?, oninput? }`: controlled text input.
- `MoonTextarea { id, label, value, rows?, required?, disabled?, invalid?, help_text?, error_text?, oninput? }`: controlled textarea.
- `MoonCheckbox { id, label, checked?, required?, disabled?, onchange? }`: native checkbox.

These source declarations have not yet been compiled in this environment. The UI crate does not provide a Select, Switch, Alert, Toast, Tooltip, Dialog, or Tabs component. See [Moon UI Components](../design/COMPONENTS.md) for examples and limitations.

## Compatibility policy

Serialized field names and meanings are public contract. Additive fields can still affect consumers depending on deserialization policy. Breaking changes require an explicit migration note, updated serialization tests, and a changelog entry. Keep contracts independent of UI, transport, and persistence.
