---
id: 239
title: "Design Date live-time capability policy"
type: docs
area: runtime/builtins
class: design-ready
priority: P1
depends_on: []
blocks: ["050"]
created: 2026-04-29
updated: 2026-04-29
---

## Summary

Define the auditable live-time capability policy required before `new Date()` or `Date.now()` can read host time.

Problem: `new Date()` and `Date.now()` currently have issue-linked unsupported diagnostics, but there is no policy work item that decides how host wall-clock time is declared, manifested, tested, and denied.

## Current failure

`issues/open/050-implement-date.md` records diagnostics for `Date.now()` and no-argument `new Date()` that require an auditable time capability policy before those entry points are enabled.

## Desired final state

The repository has a concrete policy for live host time use, including capability manifest representation, host-deny expectations, and deterministic test boundaries.

## Scope

In scope:

- [ ] Define whether live time is WASI, Node host, or another explicit host capability.
- [ ] Define manifest fields and audit reason strings for live time.
- [ ] Define host-deny behavior for programs that request live time.
- [ ] Define deterministic testing strategy for `new Date()` and `Date.now()`.
- [ ] Update issue 050 with the selected policy and next implementation child.

Out of scope:

- Implementing `new Date()` or `Date.now()`.
- Implementing timezone formatting.
- Implementing unrelated Date methods.

## Affected paths

Expected:

- `docs/`
- `issues/open/050-implement-date.md`
- `issues/open/`

Do not touch:

- `crates/backend-wasm/src/`
- `crates/ir/src/`

## Acceptance criteria

- [ ] Live time has an explicit capability and manifest policy.
- [ ] `new Date()` and `Date.now()` remain unsupported until an implementation child consumes this policy.
- [ ] Issue 050 remains blocked as the Date epic and links this child as the policy prerequisite.

## Validation

Required commands:

```sh
mise run update-issue-index
mise run update-issue-index -- --check
mise run check issues
```

Impacted commands:

```sh
mise run check manifest
```

Not run:

- none

## Docs / current-state / issue sync

Final-state docs:

- [ ] updated if the selected policy belongs in numbered docs

Current state:

- [ ] updated if current support facts change

Follow-up issues:

- [ ] created/updated for live-time implementation if policy is accepted

## Notes

Keep this as a policy/design issue. Do not add host time imports in this issue.

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
