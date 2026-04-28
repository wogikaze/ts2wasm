---
id: 001
title: "Short imperative title"
type: feature | bug | refactor | docs | test | infra | cleanup | spike
area: frontend | ir | runtime | abi | wasi | cli | fixtures | scripts | docs | tests | coverage | reference
class: triage-needed | design-ready | implementation-ready | verification-ready | docs-ready | blocked
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

Problem: Add a one-line concrete failure or gap summary for `issues/index.md` and readiness scoring.

## Current failure

Record the reproduction command, current stdout/stderr, failing fixture, diagnostic, or reference case that proves this issue exists.

## Desired final state

Describe the state after this issue is complete.

This should match final-state docs. Do not describe temporary current behavior here.

## Scope

In scope:

- [ ] ...

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

- [ ] Observable result 1
- [ ] Observable result 2
- [ ] Regression test or fixture is added/updated when behavior changes
- [ ] Docs/current-state/issues are synchronized when status or design changes

Avoid generic criteria such as "works correctly" or "diagnostics reduced"; name the exact fixture, diagnostic, stdout/stderr, generated artifact, or reference window that will change.

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

- [ ] not affected
- [ ] updated: `docs/...`

Current state:

- [ ] not affected
- [ ] updated: `current-state.md` (repo root)

Follow-up issues:

- [ ] none
- [ ] created/updated: `issues/open/...`

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
