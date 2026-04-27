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
---

## Summary

Implement JavaScript `switch` fall-through behavior instead of treating each case as implicitly broken.

## Problem

Issue 033 completed basic matching/default behavior but explicitly recorded that every case currently breaks automatically. This differs from ECMAScript semantics and must be tracked separately from basic switch support.

## Desired final state

After the matching case is found, execution continues through subsequent cases until a `break`, `return`, `throw`, or the end of the switch body, matching Node.js behavior.

## Scope

In scope:

- [ ] Preserve fall-through across case clauses when no control transfer exits the case.
- [ ] Keep default-case matching behavior compatible with ECMAScript ordering.
- [ ] Add Node differential fixtures for fall-through, default placement, and explicit `break`.
- [ ] Update docs/current-state/issues when semantic status changes.

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

- [ ] Switch fall-through matches Node.js for cases without `break`.
- [ ] Existing switch fixtures with explicit/default behavior still pass.
- [ ] Regression fixtures distinguish build success from semantic differential success.
- [ ] Docs/current-state/issues are synchronized after behavior changes.

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

- [ ] update `docs/language-reference/javascript-features.md`

Current state:

- [ ] update `current-state.md`

Follow-up issues:

- [ ] none

## Notes

Created from issue 203 audit of `issues/done/033-implement-switch-statement.md`.

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
