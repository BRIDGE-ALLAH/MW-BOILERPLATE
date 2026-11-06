# M02-005 — Implement accessible dialog

**Priority:** P2  
**Owner:** FRONTEND_AGENT  
**Dependencies:** M02-001  
**Status:** IN_PROGRESS

## Goal

Provide a safe, accessible dialog example in the web showcase without shipping a simulated modal that lacks reliable focus and keyboard behavior.

## Implementation

- Use the platform HTML `<dialog>` element.
- Open it with `showModal()` and close it with `close()`.
- Supply an accessible name using `aria-labelledby` and a description using `aria-describedby`.
- Prefer native modal semantics and browser-managed focus behavior over custom focus-trap code.
- Keep this as a showcase example until a reusable component API has a documented lifecycle and close contract.

## Acceptance criteria

- [x] Showcase route opens and closes a native modal dialog.
- [x] Dialog has an accessible name and description.
- [x] Dialog implementation delegates modal behavior to the browser.
- [ ] Browser confirms focus enters the dialog on open.
- [ ] Tab and Shift+Tab remain within the modal.
- [ ] Escape closes the dialog.
- [ ] Focus returns to the invoking control after close.
- [ ] Backdrop prevents interaction with background content.
- [ ] Accessibility tree/screen-reader announcement is verified.
- [ ] Rust formatting, compile, clippy, and web build pass.

## Validation status

Source and documentation changes are committed. No browser or Rust toolchain validation has been executed in the current environment. Do not mark this task complete until all acceptance criteria have evidence.
