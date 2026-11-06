# Contributing

1. Create a branch using `feat/*`, `fix/*`, `chore/*`, or `docs/*`.
2. Keep changes focused and preserve crate boundaries.
3. Run `cargo fmt --all --check`, `cargo check --workspace`, `cargo clippy --workspace --all-targets -- -D warnings`, and `cargo test --workspace`.
4. Update documentation when public contracts or commands change.
5. Use Conventional Commits (for example, `feat(ui): add alert primitive`).

Do not commit secrets, generated build output, or application-specific business logic into shared crates.
