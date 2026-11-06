# M01-001 — Define Public Client Configuration Boundary
**Priority:** P1 | **Agent:** ARCHITECT_AGENT | **Status:** READY | **Execution:** SEQUENTIAL
## Objective
Decide and implement the smallest safe configuration mechanism for values genuinely needed by the browser app.
## Context
Read docs/PRODUCT-BLUEPRINT.md, docs/SECURITY.md, docs/DEPLOYMENT.md, apps/web manifest and Dioxus asset/build config.
## Dependencies
M00-001.
## Scope
Public, non-secret browser configuration only. Do not introduce server secret handling into WASM and do not create a config crate without multiple consumers or a stable independent API.
## Steps
1. Inventory current and near-term config consumers.
2. Define precedence, defaults, validation, and failure behavior.
3. Decide local module vs crate using ownership rules.
4. Implement only required public fields and tests.
5. Update .env.example/docs only for variables actually consumed.
## Acceptance
- Invalid required public config fails safely with typed actionable errors.
- No secrets or private API keys can enter the WASM bundle by convention or code path.
- Tests cover missing, malformed, default, and valid values.
- No unused config abstraction is created.
## Verification
`cargo test --workspace`; `cargo check -p mw-web --target wasm32-unknown-unknown`; inspect generated bundle for accidental secret-like constants.
## Recovery
If no real public configuration consumer exists, document a defer decision and close with COMPLETED_WITH_LIMITATIONS; do not invent an API URL.
## Definition of done
A validated, minimal configuration boundary or an evidence-based defer decision is merged.