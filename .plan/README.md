# Autonomous Engineering Plan

This directory is the executable work specification for evolving Moonbase from its audited baseline toward the target in [docs/PRODUCT-BLUEPRINT.md](../docs/PRODUCT-BLUEPRINT.md).

## Start here

1. Read [MASTER-PLAN.md](MASTER-PLAN.md) for scope and acceptance.
2. Read [EXECUTION-STRATEGY.md](EXECUTION-STRATEGY.md) for scheduler, ownership, recovery, and verification rules.
3. Select the highest-priority READY task whose dependencies are COMPLETED.
4. Read that task file completely; it is designed to be self-contained.
5. Implement only its bounded scope, run its verification, and update task/milestone/progress status with evidence.

## Status vocabulary

- READY — dependencies satisfied and safe to start.
- IN_PROGRESS — actively being executed.
- BLOCKED — an external or technical blocker prevents safe progress; record evidence and continue independent tasks.
- COMPLETED — acceptance criteria and verification passed.
- COMPLETED_WITH_LIMITATIONS — implementation/documentation exists but one or more verification gates could not be completed; limitations are explicit.

Never mark a task COMPLETED based only on file creation.

## Priority

P0 foundation/blockers → P1 core → P2 essential → P3 advanced → P4 innovation/future. Within a priority, schedule dependency-ready work first, then critical-path tasks.

## Agent profiles

Use only profiles required by the task:
ARCHITECT_AGENT, FRONTEND_AGENT, API_AGENT, UI_DESIGN_AGENT, INTEGRATION_AGENT, TESTING_AGENT, QA_AGENT, SECURITY_AGENT, DOCUMENTATION_AGENT, DEVOPS_AGENT, REFACTOR_AGENT.

No separate backend/database agent is needed until a consuming product supplies an owned backend scope.

## Source of truth

- Product and architecture boundaries: [docs/PRODUCT-BLUEPRINT.md](../docs/PRODUCT-BLUEPRINT.md)
- Workspace ownership: [docs/MONOREPO.md](../docs/MONOREPO.md)
- Dependency rules: [docs/DEPENDENCY_RULES.md](../docs/DEPENDENCY_RULES.md)
- Contracts: [docs/CONTRACTS.md](../docs/CONTRACTS.md)
- Security: [docs/SECURITY.md](../docs/SECURITY.md)
- Test commands: [docs/TESTING.md](../docs/TESTING.md)

Task files may repeat only the context needed by a small agent; if this directory conflicts with canonical architecture docs, fix the conflicting plan rather than silently changing architecture.
