# M00-001 — Verify Rust, Dioxus, and CLI Compatibility
**Priority:** P0 | **Agent:** ARCHITECT_AGENT | **Status:** READY | **Execution:** SEQUENTIAL
## Objective
Verify the pinned Rust 1.98.1, declared Dioxus 0.7.10, Dioxus CLI, router, asset pipeline, and WASM target form a supported stable combination.
## Context
Read rust-toolchain.toml, root Cargo.toml, apps/web/Cargo.toml, Dioxus.toml, .github/workflows/ci.yml, deploy/Dockerfile, docs/DEVELOPMENT.md, docs/DEPLOYMENT.md.
## Scope
Only toolchain/version evidence, minimal manifest/toolchain corrections, and compatibility documentation. Do not upgrade dependencies merely to chase a version number.
## Steps
1. Inspect official Rust stable and Dioxus release/CLI documentation for current supported compatibility.
2. Install or use the pinned toolchain and inspect `dx --version`, `rustc --version`, `cargo --version`.
3. Verify router/asset features against the actual app manifest.
4. Run workspace check and WASM check; verify CLI serve/build syntax.
5. Record sources, versions, date, and any unavoidable pin.
## Acceptance
- Version matrix includes evidence/source URLs and compatibility notes.
- Toolchain pin and CLI installation instructions agree.
- No alpha/beta/unverified dependency introduced.
- Relevant check and wasm commands have recorded outcomes.
## Verification
`rustc --version`; `cargo --version`; `dx --version`; `cargo check --workspace`; `cargo check -p mw-web --target wasm32-unknown-unknown`; `dx build --release --package mw-web`.
## Recovery
If a registry or docs source is unavailable, record the exact failure, use installed lock/manifest evidence, retry once, and mark BLOCKED rather than guessing.
## Definition of done
Commit focused changes and update M00 progress with version evidence and raw command outcomes.