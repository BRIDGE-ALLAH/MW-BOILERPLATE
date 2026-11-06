# Testing

## Current automated checks

Run from repository root:

```bash
cargo fmt --all --check
cargo check --workspace
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace
cargo check -p mw-web --target wasm32-unknown-unknown
dx build --release --package mw-web
```

The Cargo aliases are documented in [Development](DEVELOPMENT.md). `cargo audit` is part of the security baseline and requires cargo-audit plus a resolvable lockfile.

## Current test coverage

- `mw-core`: identifier validation and preservation unit tests.
- `mw-contracts`: pagination default and error serialization unit tests.
- `mw-ui`: four unit tests cover the helper that associates optional help/error message IDs; component render/browser tests are not configured.
- `mw-web`: no browser E2E suite is currently configured.

A successful compile is not evidence of browser-level behavior. Do not claim route, theme, form, keyboard, responsive, or accessibility behavior passed without browser execution.

## Planned browser coverage

The M03 browser test task will select and pin one maintained browser automation tool after compatibility review. The suite must cover:
- App startup and all required routes.
- Navigation and not-found behavior.
- Light/dark theme switching.
- Form labels, input events, and validation states.
- Dialog keyboard, Escape, focus entry/return when dialog is implemented.
- Responsive layout at representative viewport sizes.
- Critical browser console errors.
- Accessibility scan and triage of critical violations.

## Integration and release gates

A release candidate requires successful format, workspace check, Clippy, unit/integration tests, WASM check, production Dioxus build, dependency audit, browser E2E/accessibility, Docker build, and container HTTP/health verification. Each result must be linked or recorded in [.plan/PROGRESS.md](../.plan/PROGRESS.md).

## Evidence policy

Record exact commands, tool versions, exit status, and relevant CI run URL. Missing or unavailable status is UNKNOWN, not PASS. If infrastructure is unavailable, mark the gate BLOCKED and continue independent work.
