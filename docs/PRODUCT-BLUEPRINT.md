# Product Blueprint — Moonbase

**Status:** Baseline and target blueprint; target capabilities are not claims of implementation.

## 1. Product identity

Moonbase is a reusable, Rust-first Dioxus web application foundation. It standardizes workspace boundaries, serializable contracts, shared UI conventions, developer tooling, quality gates, and deployment practices for independent applications.

Moonbase is not MoonWitness, A-L-L-A-H, a backend, an authentication provider, or a generic business dashboard. Consuming products own their domain models, business rules, visual identity, data, and production services.

## 2. Vision, mission, and positioning

- **Vision:** make new ecosystem applications start from a dependable, auditable Rust/Dioxus foundation rather than repeatedly rebuilding infrastructure.
- **Mission:** provide a compact and maintainable web starter with explicit module ownership, contract-first interfaces, reusable accessible UI, reproducible workflows, and clear operational boundaries.
- **Positioning:** a source-owned internal starter and shared engineering convention, not a hosted platform or a feature-complete component marketplace.

## 3. Users and problems

| User | Need | Moonbase response |
|---|---|---|
| Product engineers | Start a new web app consistently | Cargo workspace, Dioxus shell, routes, commands |
| Shared-library maintainers | Reuse types and UI without coupling products | Core/contracts/UI boundaries |
| AI coding agents | Make bounded changes safely | Self-contained tasks, dependency graph, verification gates |
| Platform/operations engineers | Build and deploy predictably | CI, security baseline, Docker/Coolify documentation |

## 4. Current capability inventory

Classification is based on repository files inspected on 2026-11-06. “Implemented” means code/configuration exists; it does not imply a successful runtime or CI verification.

| Area | Status | Evidence / gap |
|---|---|---|
| Cargo workspace | IMPLEMENTED | Root Cargo.toml includes web, core, contracts, UI, xtask |
| Dioxus web shell and typed routes | IMPLEMENTED | apps/web/src/app.rs and routes.rs |
| Theme toggle and responsive shell | PARTIAL | CSS tokens and toggle exist; browser behavior is not E2E verified |
| Core identifier and API DTOs | IMPLEMENTED | crates/core and crates/contracts; tests exist but were not run in this environment |
| Moon UI | PARTIAL | Source exports button/card/badge/separator/spinner/visually-hidden/label/input/textarea/checkbox/switch/select; compile and render verification pending |
| Form and feedback components | PARTIAL | Native field controls and badge are implemented in source; Alert/Toast and runtime verification remain outstanding |
| Dialog | MISSING | Route explicitly documents deferral for focus/accessibility |
| Config crate and validated config | MISSING | No crates/config; static frontend has no server runtime config |
| Typed API client | MISSING | No crates/api-client; no backend endpoint is defined |
| Loading/empty/error states | PARTIAL | Not yet standardized as reusable patterns |
| Structured observability | MISSING | No documented tracing convention or app-level error boundary |
| Browser E2E | MISSING | docs/TESTING.md states E2E is not included |
| CI | PARTIAL | CI workflow exists; Docker build validation and immutable action pinning are not established |
| Security audit | PARTIAL | Scheduled cargo-audit workflow exists; lockfile is generated in workflow |
| Docker/Coolify | PARTIAL | Dockerfile, Compose and deployment guide exist; image/runtime are not verified here |
| Autonomous plan | IMPLEMENTED | .plan contains the master plan, dependency graph, task index, milestones, task specs, and progress tracking |

No duplicate business-domain implementation was found in the inspected app; product-specific logic is intentionally absent.

## 5. Product principles

1. Rust and Dioxus remain the primary implementation stack.
2. Initial supported product target is web only.
3. Shared crates must remain product-neutral.
4. Contracts are transport/UI independent and serialized deterministically.
5. Configuration exposed to WASM is public by definition; secrets stay server-side.
6. Add dependencies and crates only when a concrete ownership boundary requires them.
7. Accessible behavior is part of a component contract, not a later styling task.
8. A source file or workflow is not proof of a passing build or deployment.

## 6. Capability roadmap

| Capability | Class | Priority | Boundary |
|---|---|---:|---|
| Workspace, routes, core types, DTOs | CORE | P0 | Existing workspace |
| Validated public configuration | ESSENTIAL | P1 | Config crate only if actual consumers justify it |
| Typed API transport and error mapping | ESSENTIAL | P1 | api-client depends on contracts, never UI |
| Accessible UI primitives and forms | CORE / ESSENTIAL | P1 | crates/ui |
| Loading, empty, error and not-found patterns | ESSENTIAL | P1 | app composition + shared UI where reusable |
| Browser E2E and accessibility checks | ESSENTIAL | P1 | test tooling, no production dependency |
| Tracing and error recovery conventions | ESSENTIAL | P2 | app/runtime boundaries |
| Dialog, toast, tabs and advanced controls | ADVANCED | P3 | only with keyboard/focus tests |
| Product-specific authentication, data models, business workflows | OUT OF SCOPE | — | owned by consuming products/backends |
| Desktop/mobile targets | FUTURE | P4 | revisit only after real consumers and Dioxus support are validated |

## 7. Target architecture and ownership

```text
apps/web
  ├── crates/ui
  ├── crates/api-client
  ├── crates/contracts
  └── crates/core

crates/api-client ──> crates/contracts ──> Serde
crates/ui ──> Dioxus (and core only if justified)
crates/core ──> minimal foundational dependencies
```

- **apps/web:** bootstrap, routes, layouts, page composition, browser-specific integration.
- **crates/core:** identifiers and small framework-independent primitives.
- **crates/contracts:** stable request/response DTOs, pagination, public error shape.
- **crates/api-client (planned):** HTTP transport, serialization, timeout and typed error mapping.
- **crates/ui:** reusable accessible Dioxus components and component contracts.
- **xtask:** Cargo aliases and developer task orchestration.
- **deploy:** static web image and Compose runtime.

Do not add database, auth-server, event-bus, icon, token, theme, or testing crates until a concrete consumer and independent public API justify them.

## 8. Integration blueprint

| Capability | Owner | Consumers / interface | Failure boundary |
|---|---|---|---|
| Public DTOs | contracts | API client and app | Decode/validation errors become typed client errors |
| HTTP transport | api-client (planned) | App calls typed methods | Timeout, network, non-success, and decode failures remain distinguishable |
| UI primitives | ui | App pages | UI receives values/callbacks; no transport or domain ownership |
| Product domain | Consuming product | Product-owned services and contracts | Must not be added to Moonbase shared crates |
| Static delivery | deploy | Browser via Nginx | Health check tests static server availability, not backend health |

There is currently no API origin, backend, authentication service, or data store in this repository. Do not invent endpoints or authentication behavior. A consuming product must supply its API contract and deployment boundary.

## 9. UX and design blueprint

- Preserve the neutral semantic CSS token approach and light/dark theme.
- Do not turn the sample into a generic data dashboard.
- Component APIs must document variants, sizes, disabled/loading states, labels, keyboard behavior, focus behavior, responsive behavior, and CSS ownership.
- Forms must expose labels and validation feedback without relying on color alone.
- Loading, empty, error, and not-found states must be distinguishable and recoverable.
- Dialog work is gated on focus entry/return, Escape behavior, accessible naming, and keyboard tests.

## 10. Operational blueprint

- Installation and development: see docs/DEVELOPMENT.md.
- Validation: see docs/TESTING.md and .plan/ execution gates.
- Public configuration/secrets: see docs/SECURITY.md.
- Static deployment and port mapping: see docs/DEPLOYMENT.md.
- The app currently has no server-side runtime configuration or secrets.
- Production readiness requires actual CI, browser, security, and Docker evidence; documentation alone is insufficient.

## 11. Explicit non-goals

No product-specific features, auth backend, database, SSR/hydration, Kubernetes, telemetry vendor, or production domain configuration is included in this baseline. These require separate evidence and ownership decisions.

## 12. Acceptance direction

The target is accepted only when workspace and WASM builds, unit/integration tests, browser checks, lint/format, dependency audit, and Docker build/runtime checks have real successful evidence; docs match behavior; and no secret or product-domain logic leaks into the shared starter. See [.plan/MASTER-PLAN.md](../.plan/MASTER-PLAN.md).
