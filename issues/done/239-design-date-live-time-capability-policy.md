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
completed: 2026-04-29
status: done
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

- [x] Define whether live time is WASI, Node host, or another explicit host capability.
- [x] Define manifest fields and audit reason strings for live time.
- [x] Define host-deny behavior for programs that request live time.
- [x] Define deterministic testing strategy for `new Date()` and `Date.now()`.
- [x] Update issue 050 with the selected policy and next implementation child.

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

- [x] Live time has an explicit capability and manifest policy.
- [x] `new Date()` and `Date.now()` remain unsupported until an implementation child consumes this policy.
- [x] Issue 050 remains blocked as the Date epic and links this child as the policy prerequisite.

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

- [x] updated: `docs/03-api-and-host-capability.md`, `docs/11-shared-definitions.md`

Current state:

- [x] not affected; implementation support facts did not change

Follow-up issues:

- [x] created: `issues/done/242-implement-date-live-time-wasi-clock.md`

## Notes

Keep this as a policy/design issue. Do not add host time imports in this issue.

## Completion evidence

Fill only when moving to `done/`.

Commits:

- `c1be503` issue-239: define date live-time policy

Validation result:

```text
command: mise run fmt
result: pass
date: 2026-04-29

command: mise run update-issue-index
result: pass; issues/index.md regenerated
date: 2026-04-29

command: mise run update-issue-index -- --check
result: pass; issues/index.md OK (up to date)
date: 2026-04-29

command: mise run check manifest
result: pass; check_manifest_imports OK for fixtures/basics-hello/hello.ts
date: 2026-04-29

command: mise run check-agent-state
result: pass; agent state files validated
date: 2026-04-29

command: mise run check issues
result: pass after restoring gitignored local report placeholders for pre-existing issue-health path references in issues 052 and 228
date: 2026-04-29
```

Remaining risks:

- none

## False-done audit

Date: 2026-05-06

Classification: truly-done.

Audit result: retained in `issues/done/`. This issue has repo-local close evidence
(completion evidence with validation commands) or proper superseded classification
with child issues in `issues/open/`. The acceptance criteria documented in the issue
are satisfied by the repo-local evidence cited in the completion evidence section.

Future-work tracking: no untracked future-work item was identified in this issue
during this metadata/evidence audit.

