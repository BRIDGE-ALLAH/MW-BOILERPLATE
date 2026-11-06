# M00-002 — Reproducible Workspace and CI Baseline
**Priority:** P0 | **Agent:** DEVOPS_AGENT | **Status:** READY | **Execution:** SEQUENTIAL
## Objective
Make workspace validation reproducible from a clean checkout and ensure CI gates reflect the real commands.
## Context
Read Cargo.toml, rust-toolchain.toml, .cargo/config.toml, xtask, all workflows, README and docs/TESTING.md.
## Dependencies
M00-001.
## Scope
Lockfile policy, Cargo aliases, CI workflow only. Do not duplicate jobs or change runtime features.
## Steps
1. Determine whether Cargo.lock should be committed for this application/workspace and establish one consistent policy.
2. Run every documented alias from a clean checkout.
3. Compare CI steps to docs and remove redundant checks only with evidence.
4. Ensure permissions are least-privilege and action references follow the repository's reviewed pinning policy.
5. Record push/PR/manual triggers and expected checks.
## Acceptance
- Lockfile policy is explicit and consistent across local/CI/Docker.
- All documented commands resolve to existing aliases or valid Cargo commands.
- CI includes format, check, clippy, tests, WASM, and release web build.
- No duplicate validation workflow is introduced.
## Verification
`cargo fmt --all --check`; `cargo check --workspace`; `cargo clippy --workspace --all-targets -- -D warnings`; `cargo test --workspace`; `cargo check -p mw-web --target wasm32-unknown-unknown`; `cargo build-web`.
## Recovery
If CI action or registry is unavailable, keep workflow changes minimal and report unavailable status; do not state green.
## Definition of done
CI config, README, testing docs, and plan describe the same reproducible gates.