---
id: 000
title: "Short imperative title"
type: feature | bug | refactor | docs | test | infra | cleanup | spike
area: frontend | ir | runtime | abi | wasi | cli | fixtures | scripts | docs | tests | coverage | reference
class: design-ready | implementation-ready | verification-ready | docs-ready | blocked
priority: P0 | P1 | P2 | P3
depends_on: []
blocks: []
created: YYYY-MM-DD
updated: YYYY-MM-DD
---

## Summary

Describe the work in 2-5 lines.

This is a work order, not a design document and not a progress log.

## Problem

Describe the concrete problem.

Avoid historical explanation unless it affects the implementation decision.

## Desired final state

Describe the state after this issue is complete.

This should match final-state docs. Do not describe temporary current behavior here.

## Scope

In scope:

- [x] ...

Out of scope:

- ...

## Affected paths

Expected:

- `crates/...`
- `docs/...`
- `scripts/...`
- `fixtures/...`

Do not touch:

- `...`

## Acceptance criteria

- [x] Observable result 1
- [x] Observable result 2
- [x] Regression test or fixture is added/updated when behavior changes
- [x] Docs/current-state/issues are synchronized when status or design changes

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run
```

Impacted commands:

```sh
# Add exact commands here
```

Not run:

- none

## Docs / current-state / issue sync

Final-state docs:

- [x] not affected
- [x] updated: `docs/...`

Current state:

- [x] not affected
- [x] updated: `current-state.md` (repo root)

Follow-up issues:

- [x] none
- [x] created/updated: `issues/open/...`

## Notes

Small implementation hints only.

Do not put TODO lists here.
Do not put stale history here.
Do not put completion logs here.

## Completion evidence

Fill only when moving to `done/`.

Commits:

- `...`

Validation result:

```text
command:
result:
date:
```

Remaining risks:

- none

## False-done audit

Date: 2026-05-05

Classification: truly-done.

Audit result: retained in `issues/done/`. The issue has repo-local completion evidence, is not marked `blocked` or `triage-needed`, and `python scripts/manager.py check issues` validates the resulting issue graph.

Evidence files:
- `issues/done/000-sample-issue.md`
- `issues/index.md` after regeneration

Future-work tracking: no untracked future-work item was identified in this issue during this metadata/evidence audit.
