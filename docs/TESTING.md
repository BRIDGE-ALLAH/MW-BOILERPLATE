# Testing

Run the following checks locally and in CI:

```bash
cargo fmt --all --check
cargo check --workspace
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace
cargo check -p mw-web --target wasm32-unknown-unknown
```

Browser end-to-end tests are not included yet. A successful compile is not evidence of browser-level behavior.
