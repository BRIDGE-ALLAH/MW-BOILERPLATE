# Architecture

## Product boundary

Moonbase is a product-neutral Rust/Dioxus web foundation. It is shared by independent consumers but does not own their domain logic, backend, database, authentication, data, or visual brand. The [Product Blueprint](PRODUCT-BLUEPRINT.md) is the canonical target-state narrative and capability inventory.

## Workspace ownership

- `apps/web`: Dioxus bootstrap, typed routes, page/layout composition, browser-specific integration.
- `crates/ui`: reusable Dioxus components; no API transport or product business rules.
- `crates/contracts`: serializable API DTOs; independent of UI, HTTP, and database implementations.
- `crates/core`: foundational types independent of app, UI, and transport.
- `xtask`: Cargo task aliases and development command delegation.
- `deploy`: static bundle container and Compose deployment.

See [Monorepo Guide](MONOREPO.md) and [Dependency Rules](DEPENDENCY_RULES.md) for package contracts.

## Dependency direction

```text
apps/web ──> crates/ui
   ├─────> crates/contracts
   └─────> crates/core

future api-client ──> contracts + core
contracts ──> Serde
ui ──> Dioxus
core ──> minimal foundational dependencies
```

No shared crate may depend on apps/web. Contracts must not depend on UI or transport. UI must not depend on API implementation. The API client is planned, not implemented.

## Runtime

The current target is browser WebAssembly served as a static web application. Dioxus Router handles routes. CSS custom properties provide light/dark theme tokens. Server-side rendering, hydration, authentication, persistence, and API transport are not implemented in the current baseline.

## Operational boundary

The production container is intended to build the Dioxus web bundle and serve it through Nginx. It is not an application server and has no database or secret-bearing runtime process. The Docker build output path and runtime health check still require execution evidence; see [Deployment](DEPLOYMENT.md).

## Evolution

Add crates only for real independent responsibilities and consumers. Desktop/mobile, SSR, token/icon/theme crates, and shared testing utilities remain deferred until supported by concrete use cases and compatibility evidence.
