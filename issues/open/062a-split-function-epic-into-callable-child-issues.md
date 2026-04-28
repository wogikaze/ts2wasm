---
id: 062a
title: "Split function epic into callable child issues"
type: cleanup
area: issues
class: docs-ready
priority: P1
depends_on: []
blocks: []
created: 2026-04-29
updated: 2026-04-29
---

Problem: Issue 062 mixes unrelated function surfaces, so implementation starts with scope design instead of code.

## Summary

Rewrite the function work queue into independent child issues for dynamic Function diagnostics, ordinary call semantics, `this`/`arguments`, closures, and function object metadata.

## Scope

In scope:

- [ ] Add or update child issues with one function semantic surface each.
- [ ] Ensure dynamic `Function(...)` remains diagnostic-only unless a policy issue exists.
- [ ] Cross-link issue 063 if its affected cases belong to the dynamic Function child.

Out of scope:

- Implementing function runtime semantics.
- Parser changes.

## Affected paths

Expected:

- `issues/open/062-implement-function.md`
- `issues/open/063-implement-function-resolution.md`
- `issues/open/`
- `issues/index.md`

Do not touch:

- `crates/`

## Acceptance criteria

- [ ] Issue 062 is only a blocked parent epic.
- [ ] At least three child issues have single-surface scope and validation commands.
- [ ] Issue 063 is either merged, superseded, or explicitly owned by one child issue.

## Validation

Required commands:

```sh
mise run update-issue-index
mise run update-issue-index -- --check
mise run check issues
```

Impacted commands:

```sh
mise run check issue-index
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
