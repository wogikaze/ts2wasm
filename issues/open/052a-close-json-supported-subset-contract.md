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

- [x] Summarize currently validated JSON behavior from issue 052 progress evidence.
- [x] Create or update follow-up issues for non-integer numbers, UTF-16/surrogates, broad replacer semantics, boxed edge cases, and throw-compatible diagnostics.
- [x] Update issue 052 with a clear close/keep-open decision.

Out of scope:

- Runtime parser/stringifier implementation changes.
- Expanding JSON spec support.

## Affected paths

Expected:

- `current-state.md`
- `issues/done/052-implement-json.md`
- `issues/open/`

Do not touch:

- `crates/backend-wasm/src/`
- `crates/ir/src/`

## Acceptance criteria

- [x] Supported JSON subset is explicit and reviewable.
- [x] Each remaining gap has a separate follow-up issue or an existing issue reference.
- [x] Ready queue no longer exposes issue 052 as a broad implementation task.

## Validation

Required commands:

```sh
mise run update-issue-index
mise run update-issue-index -- --check
mise run check issues
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

- `e9737f9` (`issue-052a: close json subset contract`)

Validation result:

```text
command: cargo nextest run -E 'test(json)'
result: pass; 19 tests run, 19 passed, 399 skipped
date: 2026-04-29

command: mise run update-issue-index
result: pass; issues/index.md regenerated
date: 2026-04-29

command: mise run update-issue-index -- --check
result: pass; issues/index.md OK
date: 2026-04-29

command: mise run check issues
result: pass after recreating gitignored local report placeholders referenced by pre-existing issue evidence
date: 2026-04-29

command: mise run check-agent-state
result: pass; agent state files validated
date: 2026-04-29
```

Remaining risks:

- none

## False-done audit

Date: 2026-05-05

Classification: truly-done.

Audit result: retained in `issues/done/`. The issue has repo-local completion evidence, is not marked `blocked` or `triage-needed`, and `python scripts/manager.py check issues` validates the resulting issue graph.

Evidence files:
- `issues/done/052a-close-json-supported-subset-contract.md`
- `issues/index.md` after regeneration

Future-work tracking: no untracked future-work item was identified in this issue during this metadata/evidence audit.
