# Architecture

## Workspace

- apps/web: Dioxus browser application, routes, and composition.
- crates/ui: reusable Dioxus UI primitives.
- crates/contracts: serializable API DTOs independent of UI and transport.
- crates/core: framework-independent foundational types.
- xtask: Cargo command aliases for development and common checks.

## Dependency direction

The web app composes the shared crates. UI depends on Dioxus; contracts depend on Serde; core stays independent of UI and transport. Shared crates must not import application modules.

## Runtime

The initial target is browser WebAssembly. Server-side rendering, authentication, persistence, and API transport are not implemented in this baseline and must be designed as separate capabilities.
