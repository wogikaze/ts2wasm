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
---

Problem: Issue 065 duplicates the parser syntax epic in issue 059 and makes the Ready queue present the same work twice.

## Summary

Move useful affected-test evidence from issue 065 into issue 059 child planning, then close issue 065 as superseded.

## Scope

In scope:

- [ ] Cross-link issue 065 affected-test families to issue 059 child syntax slices.
- [ ] Mark issue 065 done as superseded once evidence is preserved.
- [ ] Regenerate the issue index.

Out of scope:

- Parser implementation.
- Reference coverage reruns.

## Affected paths

Expected:

- `issues/open/059-implement-parser-syntax-extensions.md`
- `issues/open/065-implement-parser-syntax.md`
- `issues/done/`
- `issues/index.md`

Do not touch:

- `crates/`

## Acceptance criteria

- [ ] There is only one parser syntax parent epic in the open queue.
- [ ] Issue 065 evidence remains discoverable from issue 059 or a child issue.
- [ ] Ready queue no longer contains a duplicate parser syntax issue.

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
