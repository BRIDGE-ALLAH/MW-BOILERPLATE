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

- `MoonButton { label: String }`: native button with primary styling and text label.
- `MoonCard { title: String, description: String }`: section surface with heading and descriptive paragraph.

No variants, loading API, event callback API, form components, or dialog are currently exported.

## Compatibility policy

Serialized field names and meanings are public contract. Additive fields can still affect consumers depending on deserialization policy. Breaking changes require an explicit migration note, updated serialization tests, and a changelog entry. Keep contracts independent of UI, transport, and persistence.
