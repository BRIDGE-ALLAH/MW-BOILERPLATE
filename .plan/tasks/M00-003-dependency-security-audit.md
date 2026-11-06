# M00-003 — Dependency and Security Baseline Audit
**Priority:** P0 | **Agent:** SECURITY_AGENT | **Status:** READY | **Execution:** PARALLEL
## Objective
Inventory direct/transitive dependencies, advisories, unsafe code, secrets exposure, and workflow permissions.
## Context
Read Cargo manifests/lockfile, .github/workflows, .gitignore, .dockerignore, deploy files, docs/SECURITY.md, docs/DEPENDENCY_RULES.md.
## Dependencies
M00-001.
## Scope
Audit and minimal dependency/security fixes. Do not rotate or modify secrets.
## Steps
1. Generate/inspect the dependency tree and lockfile.
2. Run cargo audit using a current cargo-audit release.
3. Inspect repository-tracked files for likely credentials; never print secret values.
4. Review unsafe blocks, browser-exposed configuration, action permissions, and PR trust boundaries.
5. Classify findings by severity, file, exploitability, and remediation.
6. Fix only bounded, evidenced issues; create follow-up tasks for larger changes.
## Acceptance
- Audit report records date, tool version, findings and disposition.
- No secret values are exposed or added.
- Direct/transitive dependency risks are classified.
- Security docs reflect actual controls and limitations.
## Verification
`cargo tree --workspace`; `cargo audit`; `git grep -nE '(BEGIN .*PRIVATE KEY|gh[pousr]_[A-Za-z0-9]{20,})'` (review matches without echoing values).
## Recovery
If advisory DB/network is unavailable, record the exact failure and retain status BLOCKED for audit acceptance while continuing static review.
## Definition of done
All critical/high findings are fixed or explicitly tracked with owner and acceptance criteria.