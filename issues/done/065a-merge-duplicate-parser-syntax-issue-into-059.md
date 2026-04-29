---
id: 065a
title: "Merge duplicate parser syntax issue into 059"
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

Problem: Issue 065 duplicates the parser syntax epic in issue 059 and makes the Ready queue present the same work twice.

## Summary

Move useful affected-test evidence from issue 065 into issue 059 child planning, then close issue 065 as superseded.

## Scope

In scope:

- [x] Cross-link issue 065 affected-test families to issue 059 child syntax slices.
- [x] Mark issue 065 done as superseded once evidence is preserved.
- [x] Regenerate the issue index.

Out of scope:

- Parser implementation.
- Reference coverage reruns.

## Affected paths

Expected:

- `issues/done/059-implement-parser-syntax-extensions.md`
- `issues/done/065-implement-parser-syntax.md`
- `issues/done/`
- `issues/index.md`

Do not touch:

- `crates/`

## Acceptance criteria

- [x] There is only one parser syntax parent epic in the open queue.
- [x] Issue 065 evidence remains discoverable from issue 059 or a child issue.
- [x] Ready queue no longer contains a duplicate parser syntax issue.

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

Issue 065 affected-test evidence was summarized in issue 059, and issue 065 was moved to `issues/done/` as superseded.

Commits:

- closing commit on branch `agent/061a-065a-issue-dedupe-20260428T233550Z` (hash recorded in cycle report)

Validation result:

```text
command: mise run update-issue-index; mise run update-issue-index -- --check; mise run check issues; mise run check issue-index
result: index update/check passed; issue-health commands returned nonzero only for unrelated pre-existing missing reports in issues 052 and 228
date: 2026-04-29
```

Remaining risks:

- none
