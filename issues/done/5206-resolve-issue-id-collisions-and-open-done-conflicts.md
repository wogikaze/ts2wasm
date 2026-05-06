---
id: 5206
title: "Resolve issue ID collisions and open/done conflicts"
type: cleanup
area: issues
class: docs-ready
priority: P1
depends_on: []
blocks: []
created: 2026-05-06
updated: 2026-05-06
completed: 2026-05-06
---

## Summary

Repair the issue queue health failures caused by duplicate open issue IDs and issue IDs that exist in both `issues/open/` and `issues/done/`.

## Problem

While closing issue 5134, `python scripts/manager.py check-issue-health` reported duplicate open IDs and open/done conflicts that are unrelated to the harness feature admission change.

Problem: issue health currently fails because issue IDs 5191, 5192, 5193 and several older reopened IDs are duplicated across queue states.

## Current failure

Reproduction:

```sh
python scripts/manager.py check-issue-health
```

Current diagnostics include:

```text
duplicate id prefix in issues/open: 5191
duplicate id prefix in issues/open: 5192
duplicate id prefix in issues/open: 5193
id present in both issues/open/ and issues/done/: 338
id present in both issues/open/ and issues/done/: 354
id present in both issues/open/ and issues/done/: 368
id present in both issues/open/ and issues/done/: 401
id present in both issues/open/ and issues/done/: 402
```

## Desired final state

Every open issue has a unique ID, reopened/follow-up work does not reuse done issue IDs, and `python scripts/manager.py check-issue-health` passes.

## Scope

In scope:

- [x] Renumber duplicate open issue files to unused IDs.
- [x] Update affected issue frontmatter, dependency references, and issue index.
- [x] Decide whether open/done conflicts are true reopenings or follow-up issues, then make the queue state explicit without duplicate IDs.

Out of scope:

- Implementing the underlying compiler/runtime issues represented by those files.
- Reverting another agent's completed issue work.

## Affected paths

Expected:

- `issues/open/`
- `issues/done/`
- `issues/index.md`

Do not touch:

- `crates/`
- `scripts/`

## Acceptance criteria

- [x] `python scripts/manager.py check-issue-health` passes.
- [x] `python scripts/manager.py update-issue-index --check` passes.
- [x] `python scripts/manager.py check-issue-readiness -- --fail-ready-below 80` passes or records any remaining independent readiness blocker as a separate issue.
- [x] No issue content is dropped while renumbering.

## Validation

Required commands:

```sh
python scripts/manager.py update-issue-index
python scripts/manager.py update-issue-index --check
python scripts/manager.py check-issue-health
python scripts/manager.py check-issue-readiness -- --fail-ready-below 80
git diff --check
```

Impacted commands:

```sh
mise run check issues
```

Not run:

- none

## Docs / current-state / issue sync

Final-state docs:

- [x] not affected

Current state:

- [x] not affected

Follow-up issues:

- [x] none

## Notes

Discovered while closing issue 5134.

## Completion evidence

Commits:

- close commit: issues: restore issue queue health

Validation result:

```text
command: python scripts/manager.py update-issue-index
result: pass; index regenerated
date: 2026-05-06

command: python scripts/manager.py update-issue-index --check
result: pass
date: 2026-05-06

command: python scripts/manager.py check-issue-health
result: pass
date: 2026-05-06

command: python scripts/manager.py check-issue-readiness -- --fail-ready-below 80
result: pass
date: 2026-05-06

command: mise run check issues
result: pass
date: 2026-05-06

command: cargo fmt --all --check
result: pass
date: 2026-05-06

command: cargo check -p ts2wasm-ir -p ts2wasm-compiler
result: pass; existing compiler dead_code warning remains
date: 2026-05-06

command: git diff --check
result: pass
date: 2026-05-06
```

Remaining risks:

- Full `cargo nextest run` still has backend residual-expression failures tracked by issue 5205.

## False-done audit

Date: 2026-05-06

Classification: truly-done.

Audit result: retained in `issues/done/`. This issue has repo-local close evidence
(completion evidence with validation commands) or proper superseded classification
with child issues in `issues/open/`. The acceptance criteria documented in the issue
are satisfied by the repo-local evidence cited in the completion evidence section.

Future-work tracking: no untracked future-work item was identified in this issue
during this metadata/evidence audit.

