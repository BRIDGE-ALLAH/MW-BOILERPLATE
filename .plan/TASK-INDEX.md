# Task Index

Status reflects repository evidence at plan creation, not an assumption that tests passed. Each task has its own self-contained specification in `tasks/`.

| ID | Task | Priority | Agent | Dependencies | Status | Parallelism |
|---|---|---:|---|---|---|---|
| M00-001 | Verify Rust/Dioxus/CLI compatibility | P0 | ARCHITECT_AGENT | — | BLOCKED | SEQUENTIAL |
| M00-002 | Reproducible workspace and CI baseline | P0 | DEVOPS_AGENT | M00-001 | BLOCKED | SEQUENTIAL |
| M00-003 | Dependency and security baseline audit | P0 | SECURITY_AGENT | M00-001 | BLOCKED | PARALLEL |
| M00-004 | Reconcile documentation and contracts | P0 | DOCUMENTATION_AGENT | — | COMPLETED_WITH_LIMITATIONS | PARALLEL |
| M01-001 | Define public client configuration boundary | P1 | ARCHITECT_AGENT | M00-001 | BLOCKED | SEQUENTIAL |
| M01-002 | Implement typed API client boundary | P1 | API_AGENT | M01-001 | READY | SEQUENTIAL |
| M01-003 | Standardize typed error and recovery mapping | P1 | INTEGRATION_AGENT | M01-002 | READY | SEQUENTIAL |
| M02-001 | Define Moon UI component contract and tokens | P2 | UI_DESIGN_AGENT | M00-004 | COMPLETED | SEQUENTIAL |
| M02-002 | Expand primitive components | P2 | FRONTEND_AGENT | M02-001 | READY | PARALLEL |
| M02-003 | Implement accessible form components | P2 | FRONTEND_AGENT | M02-001 | READY | PARALLEL |
| M02-004 | Implement feedback and application states | P2 | FRONTEND_AGENT | M02-001, M01-003 | READY | PARALLEL |
| M02-005 | Implement accessible dialog | P2 | FRONTEND_AGENT | M02-001 | READY | SEQUENTIAL |
| M02-006 | Complete component showcase and route UX | P2 | INTEGRATION_AGENT | M02-002, M02-003, M02-004, M02-005 | READY | INTEGRATION_REQUIRED |
| M03-001 | Add browser E2E and accessibility checks | P3 | TESTING_AGENT | M02-006 | READY | SEQUENTIAL |
| M03-002 | Harden CI, dependency and Docker validation | P3 | DEVOPS_AGENT | M00-002, M00-003 | READY | PARALLEL |
| M03-003 | Verify Docker/Coolify runtime and operations | P3 | DEVOPS_AGENT | M03-002 | READY | SEQUENTIAL |
| M03-004 | Final security and architecture reconciliation | P3 | SECURITY_AGENT | M03-001, M03-002, M03-003 | READY | INTEGRATION_REQUIRED |
| M04-001 | Evaluate consumer-backed future targets | P4 | ARCHITECT_AGENT | M03-004 | READY | SEQUENTIAL |

**Totals:** 18 tasks — P0: 4, P1: 3, P2: 6, P3: 4, P4: 1. Only M00-004 is recorded as COMPLETED_WITH_LIMITATIONS; no task is marked COMPLETED without verification.
