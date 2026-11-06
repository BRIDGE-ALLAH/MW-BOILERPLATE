# MW-BOILERPLATE (Moonbase)

Reusable, Rust-first Dioxus web application foundation for MoonWitness, A-L-L-A-H, and future independent products. Moonbase standardizes workspace boundaries, serializable contracts, shared UI conventions, developer workflows, validation, and static web deployment. Consuming products retain their own business logic, visual identity, backend, data, and secrets.

## Product vision and capabilities

The goal is a compact foundation that teams and AI coding agents can extend safely without repeatedly rebuilding the same engineering baseline.

**Available in the current repository**
- Cargo workspace with web, core, contracts, UI, and xtask packages.
- Dioxus web shell, typed routes, responsive navigation, and light/dark theme toggle.
- Framework-independent core identifier and Serde pagination/error DTOs.
- Moon UI primitives and native controls: button, card, badge, separator, spinner, visually-hidden text, label, input, textarea, checkbox, switch, and select (source committed; compile verification pending).
- Cargo task aliases, GitHub Actions validation/security workflows, Docker and Compose deployment files.

**Partial, planned, or not yet verified**
- Moon UI is an initial two-component foundation; form, feedback, and dialog components remain planned.
- There is no API client, backend, database, authentication service, or product-specific business logic.
- Browser E2E tests are not configured yet.
- Docker image build/runtime and production bundle output path have not been validated in this authoring environment.
- CI must report a successful run before any revision is considered validated.

See [Product Blueprint](docs/PRODUCT-BLUEPRINT.md) for the evidence-based capability inventory and target architecture.

## Technology

- Rust workspace; pinned toolchain is in `rust-toolchain.toml`.
- Dioxus and Dioxus Router; dependency versions are centralized in root `Cargo.toml`.
- Native CSS with semantic custom properties and light/dark themes.
- Serde contracts and framework-independent core types.
- Docker multi-stage build with Nginx static runtime.

The checked-in manifests declare Dioxus 0.7.10 and Rust 1.98.1. Confirm the currently supported Rust/Dioxus/CLI combination using the compatibility task in [.plan](.plan/README.md) before changing pins.

## Workspace

| Path | Responsibility |
|---|---|
| `apps/web` | Dioxus bootstrap, routes, page/layout composition |
| `crates/core` | Framework-independent foundational types |
| `crates/contracts` | Serializable API DTOs |
| `crates/ui` | Reusable Dioxus components |
| `xtask` | Cargo aliases for common developer tasks |
| `deploy` | Dockerfile and minimal Compose deployment |

Dependency direction and package ownership are defined in [docs/MONOREPO.md](docs/MONOREPO.md) and [docs/DEPENDENCY_RULES.md](docs/DEPENDENCY_RULES.md).

## Prerequisites and installation

1. Install the Rust toolchain selected by `rust-toolchain.toml` (rustup is recommended).
2. Install the Dioxus CLI:

   ```bash
   cargo install dioxus-cli --locked
   ```

3. Clone the repository and enter its root directory.
4. Start the web development server:

   ```bash
   cargo dev
   ```

`cargo dev` delegates to `dx serve --package mw-web` and supports Dioxus rebuild/browser hot reload. No Node.js or pnpm is required. The first run may download the pinned Rust target and Cargo dependencies.

## Configuration and environment

The current static frontend does not require an environment file or runtime secret configuration. No `.env.example` is provided because no consumed environment variables are defined yet.

Never put credentials, private API keys, or server secrets into Dioxus assets, compile-time environment variables, or files copied into the static web bundle. Any value embedded in browser JavaScript/WASM is public. A future public API origin must be explicitly documented and validated; server secrets belong in a separately owned backend. See [Security](docs/SECURITY.md).

## Development commands

| Command | Purpose |
|---|---|
| `cargo dev` | Start web dev server with hot reload |
| `cargo dev -- --port 8080` | Pass options through to Dioxus CLI |
| `cargo build-web` | Build optimized web bundle |
| `cargo check-all` | Check workspace |
| `cargo test-all` | Run workspace tests |
| `cargo lint` | Run Clippy with warnings denied |
| `cargo fmt --all` | Format Rust code |

Direct Dioxus commands:

```bash
dx serve --package mw-web
dx build --release --package mw-web
```

## Testing and validation

Run from the repository root:

```bash
cargo fmt --all --check
cargo check --workspace
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace
cargo check -p mw-web --target wasm32-unknown-unknown
dx build --release --package mw-web
cargo audit
```

`cargo audit` requires the cargo-audit tool and a resolvable Cargo.lock. Browser E2E and Docker checks are planned but are not yet represented as passing tests. See [Testing](docs/TESTING.md) and the task gates in [.plan](.plan/README.md).

## Deployment

The initial deployment model is a static Dioxus web bundle served by Nginx. Build and run locally:

```bash
docker build -f deploy/Dockerfile -t mw-boilerplate:local .
docker run --rm -p 8080:80 mw-boilerplate:local
```

Or use Compose:

```bash
docker compose -f deploy/docker-compose.yml up --build -d
```

The Compose service maps host port 8080 to container port 80. Coolify should use the repository root as build context and `deploy/docker-compose.yml` as the Compose file. TLS/domain routing is expected from the platform proxy. These commands describe the configured workflow; image build/runtime still require execution in a Docker-enabled environment. See [Deployment](docs/DEPLOYMENT.md).

## Integration boundaries

Moonbase currently owns the frontend shell, reusable primitives, foundational types, DTOs, and developer tooling. It does not own consumer authentication, product data, domain rules, backend endpoints, or database schema. A future typed API client must depend on contracts/core and remain independent from UI. No endpoint or authentication flow should be invented without a consumer-owned contract.

## Troubleshooting

- **`cargo dev` cannot find a task:** run it from the repository root and confirm `.cargo/config.toml` is present.
- **`dx` command not found:** install Dioxus CLI with `cargo install dioxus-cli --locked`, then reopen the terminal if PATH changed.
- **WASM target missing:** run `rustup target add wasm32-unknown-unknown --toolchain 1.98.1` (or use the toolchain configured by rust-toolchain.toml).
- **Build output or Docker COPY path differs:** inspect the actual output from `dx build --release --package mw-web` and reconcile `deploy/Dockerfile`; do not assume an output path across CLI upgrades.
- **Port 8080 is occupied:** pass a different Dioxus dev port or change the host-side Compose/Docker port mapping; container port remains 80.
- **CI result unavailable:** missing status is unknown, not success. Inspect the workflow run itself.

## Documentation

- [Product Blueprint](docs/PRODUCT-BLUEPRINT.md)
- [Architecture](docs/ARCHITECTURE.md)
- [Monorepo Guide](docs/MONOREPO.md)
- [Dependency Rules](docs/DEPENDENCY_RULES.md)
- [Contracts](docs/CONTRACTS.md)
- [Development](docs/DEVELOPMENT.md)
- [Testing](docs/TESTING.md)
- [Deployment](docs/DEPLOYMENT.md)
- [Security](docs/SECURITY.md)
- [Design System](design/DESIGN_SYSTEM.md)
- [Components](design/COMPONENTS.md)
- [Accessibility Contract](design/ACCESSIBILITY.md)
- [Autonomous Execution Plan](.plan/README.md)

## Roadmap

Prioritized execution is maintained in [.plan/MASTER-PLAN.md](.plan/MASTER-PLAN.md) and [.plan/TASK-INDEX.md](.plan/TASK-INDEX.md): P0 compatibility/audit, P1 configuration/API/error boundaries, P2 Moon UI and essential UX, P3 browser/security/CI/deployment verification, P4 consumer-backed future targets. Planned work is not represented as shipped functionality.

## Autonomous development

AI coding agents should start with [.plan/README.md](.plan/README.md), then read the master plan, execution strategy, and one dependency-ready task file. Tasks define context, file scope, agent profile, dependencies, acceptance criteria, verification commands, recovery behavior, and Definition of Done. Routine engineering does not require human approval; production deployment, secret changes, destructive operations, and externally breaking contract changes remain subject to authorization.

## Current status

This repository is a reusable starter foundation, not yet a fully production-accepted product. Completion requires successful workspace/WASM builds, tests, lint/format, browser E2E/accessibility, dependency audit, Docker build/runtime, and a final architecture/security reconciliation. Never infer a passing result from the presence of source files or workflows.
