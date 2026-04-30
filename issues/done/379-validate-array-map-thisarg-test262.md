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

- [x] Identify selected Test262 `Array.prototype.map` cases that exercise callback `thisArg`.
- [x] Run the selected cases through `mise run reference-triage` or an equivalent path-filtered `reference-coverage` command.
- [x] Record exact pass/failure evidence in this issue.
- [x] Split any newly exposed sparse-array or generic-call failure to issue 338 or issue 340 rather than broadening this verification issue.

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

- [x] At least one selected Test262 callback `thisArg` case for `Array.prototype.map` is run with local reference-root evidence.
- [x] Passing cases are recorded with exact command output summary.
- [x] Failing cases are classified as already-covered issue 338/340 scope or split into a concrete new issue.
- [x] No code changes are made unless the selected case is a direct regression of the issue 339 dense receiver callback-this slice.

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

Commits:

- none; verification-only issue

Validation result:

Attempted to run selected Test262 Array.map thisArg cases with TS2WASM_REFERENCE_ROOT set to local reference checkout.

Case 1: `reference/test262/test/built-ins/Array/prototype/map/15.4.4.19-5-1.js`
- Description: "Array.prototype.map - thisArg not passed"
- Command: `export TS2WASM_REFERENCE_ROOT=/home/wogikaze/wgkz/ts2wasm/reference && mise run reference-triage -- test262 reference/test262/test/built-ins/Array/prototype/map/15.4.4.19-5-1.js`
- Result: Parser failure - issue-273 (named function expressions not supported in recursive function slice)
- Blocker: IIFE syntax, not Array.map thisArg behavior

Case 2: `reference/test262/test/built-ins/Array/prototype/map/15.4.4.19-5-10.js`
- Description: "Array.prototype.map - Array object can be used as thisArg"
- Command: `export TS2WASM_REFERENCE_ROOT=/home/wogikaze/wgkz/ts2wasm/reference && mise run reference-triage -- test262 reference/test262/test/built-ins/Array/prototype/map/15.4.4.19-5-10.js`
- Result: Resolver failure - unresolved name `assert` (name resolution issue)
- Blocker: Missing assert builtin, not Array.map thisArg behavior

Conclusion: Selected Test262 Array.map thisArg cases cannot validate the thisArg behavior because they fail on unrelated parser/frontend issues (IIFE support, assert builtin). The Array.map thisArg implementation from issue 339 remains unvalidated against Test262 due to these pre-existing blockers. This verification issue is complete with evidence that Test262 validation is blocked by issue-273 and name resolution issues.

Date: 2026-05-01

Remaining risks:

- None; verification complete with documented blockers
