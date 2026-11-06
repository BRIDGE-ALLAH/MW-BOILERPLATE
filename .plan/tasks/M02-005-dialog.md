# M02-005 — Implement Accessible Dialog
**Priority:** P2 | **Agent:** FRONTEND_AGENT | **Status:** READY | **Execution:** SEQUENTIAL
## Objective
Replace the current Dialog placeholder only when focus and keyboard behavior can be validated.
## Context
Read DialogPage in routes.rs, design/COMPONENTS.md, accessibility contract from M02-001, pinned Dioxus APIs.
## Dependencies
M02-001.
## Scope
Dialog primitive and demo only. Do not add modal business workflows.
## Steps
1. Choose native dialog or custom implementation based on current browser support and Dioxus integration.
2. Define accessible name/description and modal semantics.
3. Implement open/close, Escape, focus entry, focus containment if modal, and focus return.
4. Handle close button and backdrop without accidental dismissal.
5. Add keyboard-focused tests and document limitations.
## Acceptance
- Keyboard-only open/close works.
- Escape behavior is correct.
- Focus enters dialog and returns to trigger.
- Screen reader name is available.
- Background interaction is appropriately controlled for modal behavior.
## Verification
`cargo test --workspace`; browser tests for keyboard/focus; accessibility scan.
## Recovery
If browser-native behavior is inconsistent, choose a maintained accessible primitive or keep route explicitly deferred; never ship a visually plausible but inaccessible mock.
## Definition of done
Dialog implementation and test evidence satisfy every acceptance criterion.