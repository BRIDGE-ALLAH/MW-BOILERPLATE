# Dependency Graph

```mermaid
flowchart TD
  A[M00-001 Toolchain compatibility] --> B[M00-002 Workspace and CI baseline]
  A --> C[M00-003 Dependency/security audit]
  D[M00-004 Docs and contracts reconciliation]
  A --> E[M01-001 Public config boundary]
  E --> F[M01-002 Typed API client]
  F --> G[M01-003 Error/recovery mapping]
  D --> H[M02-001 UI contracts and tokens]
  H --> I[M02-002 Primitives]
  H --> J[M02-003 Forms]
  H --> K[M02-004 Feedback and states]
  G --> K
  H --> L[M02-005 Dialog]
  I --> M[M02-006 Showcase integration]
  J --> M
  K --> M
  L --> M
  M --> N[M03-001 Browser E2E]
  B --> O[M03-002 CI/Docker validation]
  C --> O
  O --> P[M03-003 Runtime verification]
  N --> Q[M03-004 Final reconciliation]
  P --> Q
  Q --> R[M04-001 Future targets]
```

## Scheduling notes

- M00-003 can run alongside M00-002 after M00-001; M00-004 is already documented with limitations.
- M02-002, M02-003, and M02-005 may run in parallel only after M02-001 fixes shared component conventions and each task has separate file ownership.
- M02-004 depends on the error contract from M01-003.
- M02-006 is an integration gate; do not parallelize it with component API changes.
- M03-004 is a final integration/security gate.
- No dependency cycle is present in this directed acyclic graph.
