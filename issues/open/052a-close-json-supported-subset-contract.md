---
id: 052a
title: "Close JSON supported subset contract"
type: docs
area: runtime/builtins
class: docs-ready
priority: P1
depends_on: []
blocks: []
created: 2026-04-29
updated: 2026-04-29
---

Problem: JSON has many validated progress slices, but the parent issue still reads as a full-spec implementation target and is hard to close.

## Summary

Define the current JSON.parse/stringify supported subset, move remaining spec gaps to explicit child issues, and decide whether issue 052 can be closed as the subset milestone.

## Scope

In scope:

- [ ] Summarize currently validated JSON behavior from issue 052 progress evidence.
- [ ] Create or update follow-up issues for non-integer numbers, UTF-16/surrogates, broad replacer semantics, boxed edge cases, and throw-compatible diagnostics.
- [ ] Update issue 052 with a clear close/keep-open decision.

Out of scope:

- Runtime parser/stringifier implementation changes.
- Expanding JSON spec support.

## Affected paths

Expected:

- `current-state.md`
- `issues/open/052-implement-json.md`
- `issues/open/`

Do not touch:

- `crates/backend-wasm/src/`
- `crates/ir/src/`

## Acceptance criteria

- [ ] Supported JSON subset is explicit and reviewable.
- [ ] Each remaining gap has a separate follow-up issue or an existing issue reference.
- [ ] Ready queue no longer exposes issue 052 as a broad implementation task.

## Validation

Required commands:

```sh
mise run update-issue-index
mise run update-issue-index -- --check
mise run check-issue-health
```

Impacted commands:

```sh
cargo nextest run -E 'test(json)'
```

Not run:

- none

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
