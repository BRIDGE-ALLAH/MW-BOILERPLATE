# Development

## Prerequisites

- Rust toolchain pinned by rust-toolchain.toml.
- Dioxus CLI: cargo install dioxus-cli --locked.
- wasm32-unknown-unknown target (installed by rust-toolchain.toml).

## PNPM-style commands

Run the app in development mode with automatic rebuild and browser hot reload:

```bash
cargo dev
```

The command delegates to `dx serve --package mw-web`. Pass Dioxus CLI options after `--` where supported, for example `cargo dev -- --port 8080`.

Other task aliases:

```bash
cargo build-web
cargo check-all
cargo test-all
cargo lint
cargo fmt --all
```

## Direct commands

```bash
dx serve --package mw-web
dx build --release --package mw-web
cargo check --workspace
cargo test --workspace
```

The Cargo aliases are implemented by the small xtask package. No Node.js or pnpm installation is required for this Rust workspace.
