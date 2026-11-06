# M01-003 — Standardize Error Mapping and Recovery
**Priority:** P1 | **Agent:** INTEGRATION_AGENT | **Status:** BLOCKED | **Execution:** SEQUENTIAL
## Objective
Make transport and application errors explicit and render safe, recoverable user feedback.
## Context
Read mw-contracts ApiError, API client outcome, apps/web routes, docs/SECURITY.md and docs/TESTING.md.
## Dependencies
M01-002.
## Scope
Error mapping and UI recovery pattern. Do not leak internal errors, credentials, or PII.
## Steps
1. Define typed error categories and stable user-facing messages.
2. Preserve request/correlation identifier only when supplied by contract.
3. Separate diagnostic detail from user-visible text.
4. Add retry affordance only for retryable failures.
5. Add tests for each mapping and fallback.
## Acceptance
- No swallowed errors or panic-based normal control flow.
- User-facing messages are safe and actionable.
- Internal/network details are not rendered to users.
- All error variants are tested.
## Verification
`cargo test --workspace`; `cargo clippy --workspace --all-targets -- -D warnings`; app WASM check.
## Recovery
If global error boundaries are unsupported in current Dioxus version, implement explicit route/page-level recovery and document the limitation.
## Definition of done
Error contract, UI mapping, tests, and security docs agree.