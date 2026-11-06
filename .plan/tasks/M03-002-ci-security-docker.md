# M03-002 — Harden CI, Dependency, and Docker Build Validation
**Priority:** P3 | **Agent:** DEVOPS_AGENT | **Status:** READY | **Execution:** PARALLEL
## Objective
Align CI with master prompt: reproducible build, dependency/security checks, and Docker build validation.
## Context
Read workflows, Dockerfile, Compose, .dockerignore, docs/TESTING.md, docs/DEPLOYMENT.md, M00-002 and M00-003 outcomes.
## Dependencies
M00-002, M00-003.
## Scope
GitHub Actions and build config only. Do not deploy production or modify secrets.
## Steps
1. Pin actions to reviewed immutable SHAs or documented reviewed tags.
2. Keep least-privilege permissions and avoid exposing secrets to untrusted PRs.
3. Add Docker build validation with appropriate caching and no registry push.
4. Align cargo-audit/lockfile policy.
5. Keep workflow count minimal and avoid duplicate checks.
## Acceptance
- Push/PR/manual CI paths are clear.
- Docker build runs in CI without publishing image.
- No secret is needed for PR validation.
- Security audit fails on actionable vulnerabilities per policy.
## Verification
Inspect workflow YAML; run relevant cargo checks; trigger workflow and inspect actual run status.
## Recovery
If Docker daemon/action unavailable, isolate Docker validation as a separate job and record status; do not mark gate passed.
## Definition of done
Workflow passes on branch and README/testing docs reference exact jobs.