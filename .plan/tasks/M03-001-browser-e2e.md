# M03-001 — Add Browser E2E and Accessibility Checks
**Priority:** P3 | **Agent:** TESTING_AGENT | **Status:** READY | **Execution:** SEQUENTIAL
## Objective
Add a maintained browser test harness compatible with the generated Dioxus web app.
## Context
Read docs/TESTING.md, CI, routes, component showcase, M02-006 output.
## Dependencies
M02-006.
## Scope
Browser test tooling/config/tests only. Select one tool after compatibility review; avoid redundant harnesses.
## Steps
1. Compare maintained browser automation choices and CI support.
2. Pin tool version and document setup.
3. Cover app startup, all required routes, navigation, theme toggle, forms, dialog keyboard/focus, responsive layout, and console errors.
4. Add accessibility scan for representative routes/components.
5. Integrate browser suite into CI with artifacts/logs on failure.
## Acceptance
- Tests execute headlessly in CI on clean checkout.
- All specified flows are covered or explicitly deferred with rationale.
- No critical console error.
- Accessibility violations are triaged and critical violations fixed.
## Verification
Use exact browser test command defined by the chosen tool; run locally and in CI.
## Recovery
If browser binaries cannot install, cache/pin compatible binaries or isolate infrastructure failure; do not replace E2E with compile-only checks.
## Definition of done
A successful CI run demonstrates the browser test suite and its report is linked in progress.