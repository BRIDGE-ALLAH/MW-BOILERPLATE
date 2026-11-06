# M02-001 — Define Moon UI Component Contracts and Tokens
**Priority:** P2 | **Agent:** UI_DESIGN_AGENT | **Status:** READY | **Execution:** SEQUENTIAL
## Objective
Establish a reusable API/accessibility/styling contract before expanding Moon UI.
## Context
Read crates/ui, apps/web/assets/main.css, design/DESIGN_SYSTEM.md, design/COMPONENTS.md, docs/PRODUCT-BLUEPRINT.md.
## Dependencies
M00-004.
## Scope
Component contract docs and semantic token definitions only; no large component implementation.
## Steps
1. Inventory current MoonButton/MoonCard APIs and CSS.
2. Define naming, variants, sizes, disabled/loading, events, composition, and CSS ownership.
3. Define keyboard, accessible name, focus, label, error, and reduced-motion expectations.
4. Add only tokens needed by planned components.
5. Update design docs and examples.
## Acceptance
- API rules are consistent and neutral.
- No product brand is hardcoded.
- Color/spacing/radius/shadow/motion tokens are semantic.
- Every planned component has an accessibility contract.
## Verification
Manual token/reference audit; run `cargo fmt --all --check` and `cargo check --workspace` if Rust files change.
## Recovery
If a token is not used by a concrete component, defer it rather than add speculative variables.
## Definition of done
design docs are the single source of truth for UI implementation tasks.