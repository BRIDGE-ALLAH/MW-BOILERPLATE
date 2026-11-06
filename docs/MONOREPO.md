# Monorepo Guide

## Current workspace

The Cargo workspace is intentionally compact. A directory or crate is added only when it owns an independently useful interface.

| Package | Path | Owns | Must not own |
|---|---|---|---|
| mw-web | apps/web | Dioxus bootstrap, routes, pages, browser composition | Reusable UI library, canonical DTOs, product business rules |
| mw-core | crates/core | Framework-independent identifiers and primitives | Dioxus, transport, app code |
| mw-contracts | crates/contracts | Serde DTOs and API boundary types | UI, database implementation, HTTP transport |
| mw-ui | crates/ui | Reusable Dioxus components | API calls, product domain logic |
| mw-xtask | xtask | Cargo command aliases and task delegation | Application runtime behavior |

## Current dependency direction

```text
mw-web ──> mw-ui
   ├─────> mw-contracts
   └─────> mw-core

mw-ui ──> Dioxus
mw-contracts ──> Serde
mw-core ──> minimal foundational dependencies
mw-xtask ──> command execution only
```

The root Cargo.toml centralizes shared dependency versions. Workspace members should use `workspace = true` for shared dependencies.

## Planned boundaries

An API client may be introduced when typed HTTP communication is implemented. It must depend on contracts and core, not UI. A config crate is justified only when configuration has multiple consumers or validation cannot remain local without duplication. Theme, icon, token, and testing crates remain deferred until they have independent consumers.

## Change rules

1. Put a type in core only if it is framework/transport independent and shared.
2. Put serialized public DTOs in contracts.
3. Put network behavior in an API client, never in contracts or UI.
4. Keep Dioxus imports out of core and contracts.
5. Keep app route/page composition in apps/web.
6. Avoid circular dependencies and feature combinations that pull server-only code into WASM.
7. Document public API changes and test serialization compatibility.

## Verification

Run from the repository root:

```bash
cargo check --workspace
cargo test --workspace
cargo tree --workspace
```

The commands above are repository/Cargo commands. They were not executed in the authoring environment during the documentation audit.
