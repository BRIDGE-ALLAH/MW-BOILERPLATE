# Deployment

## Runtime model

The starter is a static Dioxus Web/WASM application. The production image builds the web bundle in a Rust builder stage and serves the generated static files with Nginx. No application server, database, or secret-bearing runtime process is included.

## Build locally

From the repository root:

```bash
docker build -f deploy/Dockerfile -t mw-boilerplate:local .
```

Run the image:

```bash
docker run --rm -p 8080:80 mw-boilerplate:local
```

Open http://localhost:8080. The container exposes port 80 and includes an HTTP health check against the Nginx root.

## Docker Compose / Coolify

The minimal Compose file is `deploy/docker-compose.yml`. From the repository root:

```bash
docker compose -f deploy/docker-compose.yml up --build -d
```

The service maps host port 8080 to container port 80. For Coolify, configure the repository root as the build context and use `deploy/docker-compose.yml` as the Compose file. Set the public domain and HTTPS in Coolify's proxy configuration; TLS termination is expected to happen at the proxy, not inside this container.

## Configuration and secrets

The static frontend has no server-side runtime configuration or secret store. Do not put credentials, private API keys, or server secrets into Dioxus assets, compile-time environment variables, or files copied into the web bundle. Public values embedded in a WASM/static bundle are visible to every visitor.

If a consuming product needs a backend, deploy it separately and introduce only public API origin/configuration into this frontend. Authentication/session design must be reviewed before adding tokens or browser storage.

## Image details

- Builder: Rust toolchain pinned by `rust-toolchain.toml`, with `wasm32-unknown-unknown`.
- Frontend build: `dx build --release --package mw-web`.
- Runtime: Nginx Alpine serving Dioxus' generated `web/public` directory.
- Health check: HTTP request to `/` from inside the container.

The expected Dioxus output directory is encoded in the Dockerfile. Re-check it when upgrading Dioxus CLI. Docker image build and runtime behavior must be validated in an environment with Docker; they are not considered passed merely because the Dockerfile exists.
