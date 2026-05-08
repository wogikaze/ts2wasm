---
id: 061a
title: "Merge Date reference issue into Date epic"
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

Problem: Issue 061 duplicates issue 050 and keeps Date reference cases as a separate Ready candidate.

## Summary

Preserve issue 061 affected-test evidence under the Date epic/children, then close issue 061 as superseded.

## Scope

In scope:

- [x] Copy or cross-link useful affected-test evidence from issue 061 into issue 050 or its child issues.
- [x] Mark issue 061 done as superseded once evidence is preserved.
- [x] Regenerate the issue index.

Out of scope:

- Implementing Date behavior.
- Changing runtime or resolver code.

## Affected paths

Expected:

- `issues/done/050-implement-date.md`
- `issues/done/061-implement-date.md`
- `issues/done/`
- `issues/index.md`

Do not touch:

- `crates/`

## Acceptance criteria

- [x] There is only one Date parent epic in the open queue.
- [x] Issue 061 evidence remains discoverable from issue 050 or a Date child issue.
- [x] Ready queue no longer contains a duplicate Date reference issue.

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

Issue 061 affected-test evidence was summarized in issue 050, and issue 061 was moved to `issues/done/` as superseded.

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

## False-done audit

Date: 2026-05-05

Classification: truly-done.

Audit result: retained in `issues/done/`. The issue has repo-local completion evidence, is not marked `blocked` or `triage-needed`, and `python scripts/manager.py check issues` validates the resulting issue graph.

Evidence files:
- `issues/done/061a-merge-date-reference-issue-into-date-epic.md`
- `issues/index.md` after regeneration

Future-work tracking: no untracked future-work item was identified in this issue during this metadata/evidence audit.
