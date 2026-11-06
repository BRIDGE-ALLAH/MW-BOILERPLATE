# Autonomous Execution Strategy

## Scheduler

1. Load TASK-INDEX and all dependency edges.
2. Reconcile task status with current branch/files and latest successful evidence.
3. A task is READY only when every dependency is COMPLETED or explicitly accepted as COMPLETED_WITH_LIMITATIONS by the milestone owner.
4. Select lowest numeric priority first (P0 before P1), then critical-path tasks, then stable task ID.
5. Assign only the profile named in the task.
6. Parallelize only tasks marked PARALLEL with disjoint file ownership and no shared contract edits.
7. Run integration gates after parallel groups.

## Context package

Before execution, an agent reads:
- this plan README and MASTER-PLAN;
- its task file;
- canonical docs named in the task;
- exact source files in scope;
- current commit and working-tree state;
- dependency tasks' recorded outputs.

A task must not rely on chat history or undocumented assumptions.

## File ownership and integration

- One agent owns a file during a task.
- Cargo.toml, Cargo.lock, public Rust APIs, design tokens, global CSS, and CI workflow files are shared-contract files; serialize edits to them.
- Do not stage or overwrite unrelated user changes.
- Prefer one focused commit per completed task.
- Integration owner checks diffs, API compatibility, dependency direction, and all relevant validation.

## Validation gates

Run commands from repository root as relevant:
```bash
cargo fmt --all --check
cargo check --workspace
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace
cargo check -p mw-web --target wasm32-unknown-unknown
dx build --release --package mw-web
cargo audit
docker build -f deploy/Dockerfile -t mw-boilerplate:local .
docker run --rm -d --name mw-boilerplate-check -p 18080:80 mw-boilerplate:local
```

The final two Docker commands require Docker. Browser E2E commands are defined by M03-001 after selecting and pinning a compatible maintained tool; do not invent them before implementation. `cargo audit` requires cargo-audit and a resolvable lockfile.

## Failure classification and recovery

| Failure | Detection | Recovery |
|---|---|---|
| Toolchain/API mismatch | Install/check/build failure | Verify official release notes, align pinned toolchain/CLI, bounded retry once |
| Dependency resolution/security | Cargo resolution or audit finding | Identify direct/transitive owner, update minimally, rerun lock/check/audit |
| WASM-only incompatibility | wasm target check fails | Gate server-only dependency/features by target; do not weaken native checks |
| Accessibility regression | keyboard/axe/browser test fails | Fix semantics/focus/labels, rerun focused then full browser suite |
| Docker output mismatch | COPY source path missing | Inspect actual Dioxus build output, update Dockerfile/docs, rebuild |
| External service unavailable | network/registry/CI unavailable | Record exact error, continue independent docs/tests, leave status BLOCKED |

Retry policy: maximum two attempts after a root-cause change. Never loop unchanged commands. Preserve logs and record the failing command, root cause, fix, and final outcome.

## Progress updates

After each task, update:
- task status and evidence;
- milestone status and remaining task count;
- PROGRESS.md summary and latest commit;
- README/CHANGELOG only if public behavior or onboarding changes.

Do not report percentage complete based on file count. Count accepted task outcomes only.

## Approval boundary

No approval is needed for routine implementation, tests, docs, lint fixes, or bounded refactors. Escalate only destructive, irreversible, production, secret, or externally breaking actions requiring authorization. Continue independent tasks while such an action is isolated.
