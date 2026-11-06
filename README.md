# MW-BOILERPLATE

Reusable, production-oriented Rust and Dioxus web application foundation for MoonWitness, A-L-L-A-H, and future products.

> Status: foundation bootstrap in progress. This repository starts with a compact, contract-first Cargo workspace; architecture is expanded only when a real implementation boundary exists.

## Goals

- Rust-first development with Dioxus for the web UI.
- Cargo workspace with centralized dependency versions.
- UI, contracts, configuration, and transport boundaries kept separate.
- Neutral, reusable design system with light and dark themes.
- Reproducible checks, CI, container build, and Coolify-compatible deployment.

## Prerequisites

- Stable Rust toolchain (see `rust-toolchain.toml`).
- `wasm32-unknown-unknown` target.
- Dioxus CLI (`dx`) matching the workspace Dioxus version.

## Quick start

```bash
rustup target add wasm32-unknown-unknown
cargo install dioxus-cli --locked
 dx serve --package mw-web
```

## Validation

```bash
cargo fmt --all --check
cargo check --workspace
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace
cargo check -p mw-web --target wasm32-unknown-unknown
```

## Workspace

See `docs/ARCHITECTURE.md`, `docs/DEVELOPMENT.md`, and `docs/CONTRACTS.md` for the current implemented boundaries. Documentation describes only what exists in this repository.

## Deployment

See `deploy/README.md`. Docker and Coolify instructions are provided with explicit notes about which checks require a local Docker runtime.

## License

See `LICENSE`.
