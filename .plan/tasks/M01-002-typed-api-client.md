# M01-002 — Implement Typed API Client Boundary
**Priority:** P1 | **Agent:** API_AGENT | **Status:** READY | **Execution:** SEQUENTIAL
## Objective
Provide a typed, replaceable HTTP boundary that consumes existing contracts without coupling UI to transport.
## Context
Read crates/contracts, crates/core, docs/CONTRACTS.md, docs/DEPENDENCY_RULES.md, and M01-001 outcome.
## Dependencies
M01-001.
## Scope
Only transport abstractions and generic request/response/error handling. No product endpoints, auth backend, or fake service.
## Steps
1. Confirm supported Dioxus/WASM target and maintained HTTP crate/version.
2. Choose minimal client dependency/features compatible with wasm.
3. Define typed client construction and generic request boundary.
4. Serialize requests and deserialize responses using existing contracts.
5. Map network, timeout, non-success, and decode failures distinctly.
6. Add deterministic tests using a local/mock transport only as a test fixture, not production behavior.
## Acceptance
- Client depends on contracts/core, never UI.
- No secret token persistence or hardcoded production URL.
- Success and each failure class have tests.
- Native and WASM builds both work.
## Verification
`cargo tree --workspace`; `cargo test --workspace`; `cargo check --workspace`; `cargo check -p mw-web --target wasm32-unknown-unknown`.
## Recovery
If chosen HTTP crate lacks WASM support, select a maintained compatible alternative and retry once; if no real endpoint contract exists, ship only generic boundary, not invented endpoint methods.
## Definition of done
Typed boundary compiles, tests pass, dependency graph and docs are updated.