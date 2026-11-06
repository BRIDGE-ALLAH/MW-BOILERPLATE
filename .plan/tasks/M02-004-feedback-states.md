# M02-004 — Implement Feedback and Application States
**Priority:** P2 | **Agent:** FRONTEND_AGENT | **Status:** BLOCKED | **Execution:** PARALLEL
## Objective
Standardize Alert/Badge/Toast only when justified and app-level loading, empty, error, and not-found presentation.
## Context
Read M01-003 error contract, M02-001 UI contract, routes.rs and design docs.
## Dependencies
M02-001, M01-003.
## Scope
Reusable feedback components and sample app states; no backend-driven artificial data.
## Steps
1. Define severity and status semantics.
2. Build Alert and Badge using semantic tokens.
3. Add Toast only with an explicit lifecycle/timing/accessibility design; otherwise defer.
4. Add reusable page-level loading/empty/error patterns.
5. Add route-level recovery and examples.
## Acceptance
- Status is not conveyed by color alone.
- Announcements use appropriate live-region semantics.
- Empty and error states offer meaningful next action where possible.
- No mock API state is presented as real backend functionality.
## Verification
`cargo test --workspace`; `cargo clippy --workspace --all-targets -- -D warnings`; browser tests in M03.
## Recovery
If toast timing cannot meet accessible announcement/interaction requirements, defer Toast and ship Alert only.
## Definition of done
State patterns are documented and used in the sample app.