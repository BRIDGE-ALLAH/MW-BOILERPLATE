# M02-002 — Expand Moon UI Primitives
**Priority:** P2 | **Agent:** FRONTEND_AGENT | **Status:** IN_PROGRESS | **Execution:** PARALLEL
## Objective
Build a small tested set of reusable primitives beyond MoonButton and MoonCard.
## Context
Read M02-001 output, crates/ui/src/lib.rs, current CSS, Dioxus component API docs for pinned version.
## Dependencies
M02-001.
## Scope
Button improvements, Separator, Badge, Spinner, VisuallyHidden only if the Dioxus API and use cases are validated. Keep public API backwards-compatible unless documented.
## Steps
1. Add one primitive at a time with documented props/defaults.
2. Use native semantic elements where possible.
3. Add disabled/loading states only where behavior is defined.
4. Add styles through semantic tokens.
5. Add component tests or the narrowest supported render tests.
## Acceptance
- Existing call sites still compile.
- Each component has accessible semantics and usage example.
- No arbitrary color values or duplicated CSS conventions.
- Unit/render tests cover key states.
## Verification
`cargo fmt --all --check`; `cargo check --workspace`; `cargo test --workspace`; `cargo clippy --workspace --all-targets -- -D warnings`.
## Recovery
If Dioxus component testing is unavailable, isolate pure logic tests and cover rendered behavior in M03 browser tests; do not claim UI tests passed.
## Definition of done
Public exports, docs, examples, and tests are integrated.