# M02-003 — Implement Accessible Form Components
**Priority:** P2 | **Agent:** FRONTEND_AGENT | **Status:** IN_PROGRESS | **Execution:** PARALLEL
## Objective
Provide reusable Input, Textarea, Checkbox, Switch, Select, and Label primitives in coherent increments.
## Context
Read M02-001, current InputPage in apps/web/src/routes.rs, crates/ui, design/COMPONENTS.md.
## Dependencies
M02-001.
## Scope
Implement only components with verified native semantics and Dioxus event/prop APIs. Avoid custom widgets when native HTML suffices.
## Steps
1. Start with Label/Input/Textarea.
2. Add validation/help/error/disabled state contracts.
3. Add Checkbox/Switch/Select only with keyboard and screen-reader behavior documented.
4. Use controlled values/events with explicit state ownership.
5. Add tests and component examples.
## Acceptance
- Labels are programmatically associated.
- Required/invalid/disabled states are conveyed semantically.
- Keyboard operation works without a pointer.
- No state is hidden in global mutable state.
- Tests cover input changes and validation states.
## Verification
`cargo test --workspace`; `cargo clippy --workspace --all-targets -- -D warnings`; wasm check; M03 browser coverage.
## Recovery
Use native input controls if custom semantics cannot be made reliable. Defer advanced Select rather than ship an inaccessible imitation.
## Definition of done
Components and docs are exported and tested.