# Moonbase Master Plan

## Objective

Deliver a compact, reusable, production-oriented Rust/Dioxus web starter for MoonWitness, A-L-L-A-H, and future independent applications. The shared starter provides conventions and reusable infrastructure; each consumer retains its own domain logic, visual identity, backend, data ownership, and deployment secrets.

## Audited baseline

Repository: `BRIDGE-ALLAH/MW-BOILERPLATE`, branch `main`. Audit baseline commit: `7cd66a0701723594c2616012b3fbe5a1db072896`.

Existing: Cargo workspace (web/core/contracts/ui/xtask), Dioxus 0.7.10 dependency declaration, typed routes, responsive shell, theme toggle, MoonButton/MoonCard, basic DTOs and core identifier, Cargo aliases, CI, security workflow, Dockerfile, Compose, and development/deployment/security docs.

Gaps: no config or API-client implementation; UI library has only two primitives; reusable form/feedback/dialog patterns absent; no browser E2E; no standardized app loading/empty/error recovery; observability conventions absent; no .plan before this plan; CI does not yet verify Docker build and action references are version tags rather than immutable SHAs. Docker output path/runtime not tested in this environment. See [product blueprint](../docs/PRODUCT-BLUEPRINT.md).

## Scope

### In scope
- Verify toolchain/framework compatibility and dependency health.
- Complete architectural and product-neutral documentation.
- Add validated configuration only where it has a concrete public client use case.
- Establish typed API-client boundary without inventing a backend.
- Grow Moon UI incrementally with accessible, tested components.
- Add error/loading/empty patterns, observability conventions, browser tests, security/CI/deployment verification.
- Keep commands reproducible and onboarding accurate.

### Out of scope
- MoonWitness or A-L-L-A-H business logic.
- Backend, database, authentication provider, production secrets, or fabricated endpoints.
- SSR/hydration until a supported use case and validated Dioxus configuration exist.
- Kubernetes, arbitrary microservices, unnecessary workspace crates, or a generic dashboard redesign.

## Target architecture

See [docs/MONOREPO.md](../docs/MONOREPO.md) and [docs/DEPENDENCY_RULES.md](../docs/DEPENDENCY_RULES.md). Existing crates remain canonical. Planned API client must depend on contracts/core and never UI. Config crate is conditional on multiple consumers. No new crate is created without an independent responsibility and real consumer.

## Milestones

| Milestone | Priority | Outcome | Exit gate |
|---|---:|---|---|
| M00 Foundation & audit | P0 | Toolchain/dependency baseline and repository readiness known | Versions, commands, and existing behavior have evidence |
| M01 Core contracts & integration | P1 | Validated config/API boundary and reliable app error handling | Unit/integration/WASM checks pass |
| M02 Essential UX & Moon UI | P2 | Accessible primitives, forms, states, and showcase | Component and keyboard/accessibility tests pass |
| M03 Advanced quality & operations | P3 | Browser E2E, security, CI, Docker and operations verified | CI + browser + image/runtime evidence pass |
| M04 Innovation & future targets | P4 | Evidence-led future evolution, no premature architecture | Consumer-backed ADR and compatibility evidence |

## Critical path

M00-001 → M00-002 → M01-001 → M01-002 → M01-003 → M02-001 → M02-002 → M03-001 → M03-002 → M03-003. Independent UI component tasks can run in parallel after the design/accessibility contract is fixed. M04 is not a release blocker.

## Definition of done

A milestone is complete only when:
- all tasks in the milestone are COMPLETED or explicitly excluded with a recorded reason;
- dependency direction is valid and no cycles exist;
- format, lint, workspace tests, WASM check, and production build have successful evidence;
- browser E2E and accessibility checks cover the implemented interactions;
- security audit and Docker image/runtime checks are executed;
- docs, README, plan status, and changelog match actual behavior;
- no secrets or consumer-specific business rules are committed.

## Environment and evidence policy

The current authoring environment did not expose `cargo`, `rustc`, `dx`, or Docker in PATH during prior checks. Therefore no local Rust, WASM, Dioxus CLI, or container result is claimed. CI status for the baseline commit was empty/unavailable. Re-check these gates in a toolchain-enabled environment. Do not interpret missing status as success.

## Execution order

Run M00 first. After the baseline is confirmed, execute independent tasks in parallel only when file ownership and dependencies do not overlap. Never parallelize changes to the same Cargo manifest, shared public API, or central CSS without a designated integration owner.

## Authorization boundaries

Routine code, tests, docs, and bounded refactors require no human approval. Do not perform production deployment, secret changes, destructive operations, irreversible data changes, or externally breaking public contract changes without the authorization required by the environment.
