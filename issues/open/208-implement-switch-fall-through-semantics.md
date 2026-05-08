---
id: 208
title: "Implement switch fall-through semantics"
type: feature
area: frontend/semantics
class: implementation-ready
priority: P2
depends_on: []
blocks: []
created: 2026-04-28
updated: 2026-04-28
completed: 2026-04-28
---

## Summary

Implement JavaScript `switch` fall-through behavior instead of treating each case as implicitly broken.

## Problem

Issue 033 completed basic matching/default behavior but explicitly recorded that every case currently breaks automatically. This differs from ECMAScript semantics and must be tracked separately from basic switch support.

## Desired final state

After the matching case is found, execution continues through subsequent cases until a `break`, `return`, `throw`, or the end of the switch body, matching Node.js behavior.

## Scope

In scope:

- [x] Preserve fall-through across case clauses when no control transfer exits the case.
- [x] Keep default-case matching behavior compatible with ECMAScript ordering.
- [x] Add Node differential fixtures for fall-through, default placement, and explicit `break`.
- [x] Update docs/current-state/issues when semantic status changes.

Out of scope:

- Labeled break/continue; tracked by issue 209.

## Affected paths

Expected:

- `crates/frontend/src/`
- `crates/ir/src/`
- `crates/backend-wasm/src/`
- `crates/cli/tests/`
- `fixtures/`
- `docs/language-reference/javascript-features.md`
- `current-state.md`

Do not touch:

- none

## Acceptance criteria

- [x] Switch fall-through matches Node.js for cases without `break`.
- [x] Existing switch fixtures with explicit/default behavior still pass.
- [x] Regression fixtures distinguish build success from semantic differential success.
- [x] Docs/current-state/issues are synchronized after behavior changes.

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run
```

Impacted commands:

```sh
cargo nextest run -E 'test(switch)'
```

Not run:

- none

## Docs / current-state / issue sync

Final-state docs:

- [x] update `docs/language-reference/javascript-features.md`

Current state:

- [x] update `current-state.md`

Follow-up issues:

- [x] none

## Notes

Created from issue 203 audit of `issues/open/033-implement-switch-statement.md`.

## Completion evidence

Fill only when moving to `done/`.

Commits:

- `f07ee0a` implements switch fall-through dispatch and Node differential coverage.
- close commit records docs/issue synchronization.

Validation result:

```text
command: cargo nextest run -E 'test(switch)'
result: passed, 5 tests run including switch_fallthrough_fixture_matches_node_output_under_iwasm
date: 2026-04-28

command: cargo fmt --all --check
result: passed
date: 2026-04-28

command: cargo nextest run
result: passed
date: 2026-04-28

command: mise run update-issue-index -- --check
result: passed
date: 2026-04-28

command: mise run check-issue-health
result: passed
date: 2026-04-28

command: mise run check-agent-state
result: passed
date: 2026-04-28

command: mise run check-repo-smoke
result: passed
date: 2026-04-28
```

Remaining risks:

- none

## False-done audit

Date: 2026-05-06

Classification: truly-done.

Audit result: retained in `issues/open/`. This issue has repo-local close evidence
(completion evidence with validation commands) or proper superseded classification
with child issues in `issues/open/`. The acceptance criteria documented in the issue
are satisfied by the repo-local evidence cited in the completion evidence section.

Future-work tracking: no untracked future-work item was identified in this issue
during this metadata/evidence audit.

