# M00 — Foundation and Audit

**Priority:** P0  
**Objective:** Establish evidence-based compatibility, repository readiness, and documented module boundaries before implementation expands.

## Scope and dependencies

Tasks: M00-001, M00-002, M00-003, M00-004. No dependency on future milestones.

## Exit criteria

- Rust/Dioxus/CLI versions and target compatibility have explicit evidence.
- Workspace, aliases, CI, dependency graph, and security baseline are checked.
- Missing CI gates and known deployment assumptions are documented.
- Product-neutral docs and autonomous plan exist.
- No source implementation is changed merely to satisfy a target directory diagram.

## Verification gate

Run fmt/check/clippy/test/WASM build, inspect cargo tree, cargo audit, and inspect GitHub Actions results. Record actual outputs. Docker validation remains in M03.
