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
completed: 2026-04-29
---

Problem: Issue 062 mixes unrelated function surfaces, so implementation starts with scope design instead of code.

## Summary

Rewrite the function work queue into independent child issues for dynamic Function diagnostics, ordinary call semantics, `this`/`arguments`, closures, and function object metadata.

## Scope

In scope:

- [x] Add or update child issues with one function semantic surface each.
- [x] Ensure dynamic `Function(...)` remains diagnostic-only unless a policy issue exists.
- [x] Cross-link issue 063 if its affected cases belong to the dynamic Function child.

Out of scope:

- Implementing function runtime semantics.
- Parser changes.

## Affected paths

Expected:

- `issues/done/062-implement-function.md`
- `issues/done/063-implement-function-resolution.md`
- `issues/open/`
- `issues/done/`
- `issues/index.md`

Do not touch:

- `crates/`

## Acceptance criteria

- [x] Issue 062 is only a blocked parent epic.
- [x] At least three child issues have single-surface scope and validation commands.
- [x] Issue 063 is either merged, superseded, or explicitly owned by one child issue.

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

Issue 062 was rewritten as a blocked parent epic, issues 062b through 062f were
created as callable single-surface child issues, and issue 063 was closed as
superseded by issue 062b.

Commits:

- closing commit on branch `agent/062a-function-split-20260428T235026Z` (hash recorded in cycle report)

Validation result:

```text
command: mise run update-issue-index; mise run update-issue-index -- --check; mise run check issues; mise run check issue-index; mise run check-agent-state
result: index update/check and agent-state passed; issue health commands failed only on pre-existing missing report paths in issue 052 and done issue 228, with no remaining 062a/063 errors
date: 2026-04-29
```

Remaining risks:

- none
