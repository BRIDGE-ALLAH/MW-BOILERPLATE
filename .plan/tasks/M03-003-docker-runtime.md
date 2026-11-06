# M03-003 — Verify Docker and Coolify Runtime
**Priority:** P3 | **Agent:** DEVOPS_AGENT | **Status:** READY | **Execution:** SEQUENTIAL
## Objective
Prove the static Dioxus bundle is copied to the correct Nginx path and served with a functioning health check.
## Context
Read deploy/Dockerfile, deploy/docker-compose.yml, .dockerignore, docs/DEPLOYMENT.md and Dioxus CLI output from M00-001.
## Dependencies
M03-002.
## Scope
Container build/runtime and deployment docs. No production deployment.
## Steps
1. Build image locally.
2. Inspect actual generated output and Docker COPY source.
3. Run container and verify HTTP response and health status.
4. Run Compose and verify port mapping/restart policy.
5. Confirm Coolify build context/Compose path and proxy/TLS assumptions.
6. Update docs with only verified facts.
## Acceptance
- Docker image builds reproducibly.
- Runtime serves app on container port 80.
- Health check reports healthy after startup.
- Compose starts with documented command.
- No secrets or build toolchain remain in runtime image.
## Verification
`docker build -f deploy/Dockerfile -t mw-boilerplate:local .`; `docker run --rm -d --name mw-boilerplate-check -p 18080:80 mw-boilerplate:local`; `curl -fsS http://127.0.0.1:18080/`; inspect `docker inspect` health; cleanup only the test container created by this task.
## Recovery
If COPY path is wrong, inspect dx output and correct Dockerfile; if Docker unavailable, mark BLOCKED and retain exact evidence.
## Definition of done
Build, HTTP, health, Compose and docs evidence are recorded; no production deployment performed.