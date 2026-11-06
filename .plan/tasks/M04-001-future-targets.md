# M04-001 — Evaluate Consumer-Backed Future Targets
**Priority:** P4 | **Agent:** ARCHITECT_AGENT | **Status:** READY | **Execution:** SEQUENTIAL
## Objective
Decide whether desktop/mobile, SSR/hydration, theme/token/icon crates, or shared test utilities are justified by actual consumer needs.
## Context
Read product blueprint, monorepo/dependency rules, consumer requirements available in repository, Dioxus official compatibility evidence, and final M03 report.
## Dependencies
M03-004.
## Scope
Assessment and decision record only unless a separate authorized task is created.
## Steps
1. Inventory concrete consumer requests and usage evidence.
2. Compare Dioxus stable platform support and maintenance costs.
3. Evaluate dependency, security, build, and migration implications.
4. Recommend adopt/defer/reject for each candidate.
5. Create new tasks only for accepted, bounded work.
## Acceptance
- Every recommendation cites a real consumer need or evidence.
- No speculative crate or platform target is added.
- Decision and follow-up tasks are recorded.
## Verification
Review against current docs and supported Dioxus release notes.
## Recovery
If no consumer evidence exists, defer candidates and close with rationale.
## Definition of done
A concise decision record and updated roadmap exist; no unsupported implementation claim.