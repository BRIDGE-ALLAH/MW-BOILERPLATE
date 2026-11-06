# M02-006 — Integrate Component Showcase and Route UX
**Priority:** P2 | **Agent:** INTEGRATION_AGENT | **Status:** READY | **Execution:** INTEGRATION_REQUIRED
## Objective
Make the sample app demonstrate the actual reusable components and states without becoming a product dashboard.
## Context
Read all component task outputs, apps/web/src/routes.rs, design docs, README and docs/TESTING.md.
## Dependencies
M02-002, M02-003, M02-004, M02-005.
## Scope
Route/page composition and examples only; do not redefine shared component contracts.
## Steps
1. Update Components route with truthful component inventory and usage examples.
2. Integrate actual form controls and feedback examples.
3. Integrate dialog demo only if M02-005 passed.
4. Add useful empty/error/loading examples only where their behavior can be triggered honestly.
5. Check responsive nav and theme toggle.
## Acceptance
- All showcased components exist and compile.
- No route advertises deferred or fake functionality as implemented.
- Required routes remain available.
- Mobile navigation and theme controls remain accessible.
## Verification
`cargo check -p mw-web --target wasm32-unknown-unknown`; `dx build --release --package mw-web`; M03 browser tests.
## Recovery
If a component task is deferred, keep its route honest and do not add fake controls.
## Definition of done
Routes, docs, and exported component inventory match exactly.