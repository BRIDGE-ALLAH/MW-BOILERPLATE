# MW-BOILERPLATE

Reusable Rust + Dioxus application foundation. The workspace separates web composition, reusable UI, serializable contracts, framework-independent core types, and Cargo task automation.

## Quick start

Install the Dioxus CLI and run the browser development server:

```bash
cargo install dioxus-cli --locked
cargo dev
```

`cargo dev` is the Rust-workspace equivalent of a pnpm dev script. It runs `dx serve --package mw-web`, including Dioxus rebuild and browser hot reload. No Node.js or pnpm is required.

## Commands

| Command | Purpose |
|---|---|
| `cargo dev` | Start local dev server with hot reload |
| `cargo build-web` | Build optimized web bundle |
| `cargo check-all` | Check all workspace crates |
| `cargo test-all` | Run all tests |
| `cargo lint` | Run Clippy with warnings denied |
| `cargo fmt --all` | Format Rust code |

## Workspace

- `apps/web`: Dioxus web app and routes.
- `crates/ui`: reusable Dioxus primitives.
- `crates/contracts`: Serde DTOs for API boundaries.
- `crates/core`: framework-independent types.
- `xtask`: implementation behind Cargo task aliases.

## Validation

```bash
cargo fmt --all --check
cargo check --workspace
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace
cargo check -p mw-web --target wasm32-unknown-unknown
```

CI runs the validation suite on pushes to main and pull requests. A green workflow is required before treating a revision as validated.

## Documentation

- `docs/ARCHITECTURE.md`
- `docs/DEVELOPMENT.md`
- `docs/CONTRACTS.md`
- `docs/TESTING.md`
- `docs/SECURITY.md`
- `design/DESIGN_SYSTEM.md`
- `design/COMPONENTS.md`

## Scope

This is a frontend and shared-library foundation. Authentication, database persistence, API server, SSR, and product-specific business rules are deliberately not claimed as implemented.
