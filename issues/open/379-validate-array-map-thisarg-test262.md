---
id: 379
title: "Validate Array.prototype.map thisArg against Test262"
type: test
area: reference/tests
class: verification-ready
priority: P2
depends_on: [339]
blocks: []
created: 2026-05-01
updated: 2026-05-01
---

## Summary

Validate the implemented `Array.prototype.map` callback `thisArg` slice against selected Test262 cases once a local Test262 checkout is available.

Issue 339 now has Node/iwasm fixture coverage for named callback and inline `function` callback `thisArg`, but this worktree does not contain `reference/test262` and has no `TS2WASM_REFERENCE_ROOT` configured.

## Problem

Problem: selected Test262 `Array.prototype.map` thisArg evidence cannot be recorded in the current child worktree because the Test262 reference checkout is absent.

## Current failure

Current environment evidence from issue 339 closure:

```text
find reference/test262/test/built-ins/Array/prototype/map -maxdepth 1 -type f ...
result: reference/test262/test/built-ins/Array/prototype/map: No such file or directory

printenv TS2WASM_REFERENCE_ROOT
result: empty
```

## Desired final state

Selected Test262 `Array.prototype.map` callback `thisArg` cases are identified, run through the repository reference runner, and recorded as pass or split into exact implementation follow-ups if they expose behavior outside issue 339's dense-array callback-this slice.

## Scope

In scope:

- [ ] Identify selected Test262 `Array.prototype.map` cases that exercise callback `thisArg`.
- [ ] Run the selected cases through `mise run reference-triage` or an equivalent path-filtered `reference-coverage` command.
- [ ] Record exact pass/failure evidence in this issue.
- [ ] Split any newly exposed sparse-array or generic-call failure to issue 338 or issue 340 rather than broadening this verification issue.

Out of scope:

- Sparse array hole semantics; use issue 338.
- Broader generic `Array.prototype.map.call(...)` semantics; use issue 340.
- Changing the issue 339 dense receiver implementation unless Test262 exposes a direct callback-this regression.

## Affected paths

Expected:

- `issues/open/379-validate-array-map-thisarg-test262.md`
- `issues/index.md`
- `current-state.md` only if new factual behavior is discovered

Do not touch:

- Sparse array representation redesign
- Generic call implementation
- Unrelated Array.map dense behavior rewrites

## Acceptance criteria

- [ ] At least one selected Test262 callback `thisArg` case for `Array.prototype.map` is run with local reference-root evidence.
- [ ] Passing cases are recorded with exact command output summary.
- [ ] Failing cases are classified as already-covered issue 338/340 scope or split into a concrete new issue.
- [ ] No code changes are made unless the selected case is a direct regression of the issue 339 dense receiver callback-this slice.

## Validation

Required commands:

```sh
mise run reference-triage -- test262 <selected-array-map-thisarg-case>
mise run update-issue-index -- --check
mise run check issues
```

Impacted commands:

```sh
mise run reference-coverage -- test262 --path-filter <selected-array-map-thisarg-directory-or-file> --detail
```

Not run:

- none

## Docs / current-state / issue sync

Final-state docs:

- [x] not affected

Current state:

- [ ] not affected unless validation changes known current behavior

Follow-up issues:

- [ ] none unless selected Test262 evidence exposes a new non-338/non-340 gap

## Notes

This is a verification split from issue 339, not a license to broaden Array.map runtime semantics. Use issue 338 for sparse holes and issue 340 for generic call behavior.

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
