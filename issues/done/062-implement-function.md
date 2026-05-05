---
id: 062
title: "Implement function support (dup)"
type: feature
area: frontend/semantics
class: blocked
priority: P1
depends_on: []
blocks: []
status: done
created: 2026-04-26
updated: 2026-05-05
completed: 2026-04-29
---

## Summary

Epic for function support. The implementation work is split into callable child
issues because function support spans unrelated semantic surfaces.

Problem: Function support spans dynamic Function diagnostics, ordinary call
semantics, `this`/`arguments`, closures, and function object behavior; direct
selection is too broad.

Queue design note:

- This is an epic-level issue and must not be selected directly from the Ready queue.
- Use child slices with a single function semantic surface and Node/iwasm differential evidence.
- Function constructor / eval-like behavior should remain diagnostic-only unless an explicit dynamic evaluation policy exists.
- Child issues:
  - issue 062b: dynamic `Function(...)` / `new Function(...)` diagnostics and policy
  - issue 062c: ordinary function declarations and direct calls
  - issue 062d: function receiver `this` and `arguments`
  - issue 062e: closures and captured lexical environments
  - issue 062f: function object metadata
  - issue 062g: heap closure object ABI and rooting for escaping closures
- Issue 063 was closed as superseded by issue 062b; its Annex B dynamic
  Function constructor cases are owned there.

## Problem

Reference test results show 47 cases fail with function diagnostic. The compiler cannot handle these syntax/semantics, preventing compilation of code in this category.

## Desired final state

function feature is correctly implemented according to JavaScript/TypeScript specifications. Related diagnostics are only emitted for genuinely unsupported cases.

## Scope

In scope for the epic:

- [x] Track the child issue list.
- [x] Keep shared function support constraints discoverable.
- [x] Close only after all child issues are closed or explicitly superseded.

Implementation scope belongs to child issues:

- [x] Dynamic Function constructor diagnostics and policy: issue 062b
- [x] Ordinary function declarations and direct calls: issue 062c
- [x] Function receiver `this` and `arguments`: issue 062d
- [x] Closures and captured lexical environments: issue 062e
- [x] Function object metadata: issue 062f
- [x] Heap closure object ABI/rooting for escaping returned closures: issue 062g

Out of scope:

- [x] Related features remain separate issues/scopes when explicitly out of the
      function epic, including dynamic `eval`, Annex B semantics, generators,
      async functions, and broader mutable closure environments.

## Affected paths

Expected:

- `crates/frontend/src/`
- `crates/cli/src/`
- `fixtures/`
- `scripts/run/reference-triage.py`

Do not touch:

- unrelated runtime/backend code unless `reference-triage` proves the failure is not frontend-owned

## Acceptance criteria

- [x] All child issues listed in the queue design note are closed or explicitly superseded.
- [x] Related function diagnostics are reduced in reference tests through child issues.
- [x] Regression coverage exists for each supported function semantic surface.
- [x] Docs/current-state are updated by child issues when semantics change.

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run
```

Impacted commands:

```sh
mise run reference-coverage -- test262 --limit 94
```

Not run:

- none

## Docs / current-state / issue sync

Final-state docs:

- [x] updated: `docs/language-reference/javascript-features.md`

Current state:

- [x] updated: `current-state.md` (repo root)

Follow-up issues:

- [x] none for the 062 epic close. Remaining unsupported behavior is outside
      this parent issue and tracked by separate issues/scopes.

## Notes

2026-04-29 split note:

- Issue 062 is now a blocked parent epic only.
- The callable implementation/verification surfaces are split into issues
  062b, 062c, 062d, 062e, and 062f.
- Issue 063 is superseded by issue 062b because its affected tests are Annex B
  dynamic Function constructor cases.

2026-04-28 child progress (`062-function-constructor-diagnostics-20260428T050359Z`):

- Added an issue-linked diagnostic slice for dynamic `Function(...)` and `new Function(...)` constructor usage.
- The diagnostic intentionally reports unsupported runtime code evaluation and does not implement dynamic evaluation semantics.
- Added resolver and CLI regression coverage plus direct unsupported fixtures.
- Issue remains open because the broader function syntax/semantics acceptance criteria are not complete.

2026-04-29 epic close verification:

- issue 062b is done for diagnostic-only dynamic `Function(...)` / `new Function(...)` policy.
- issue 062c is done for ordinary declarations, direct calls, positional arguments, and returns.
- issue 062d is done for supported receiver binding plus basic `arguments.length` and indexed reads.
- issue 062e is done for immutable ordinary closure capture, returned closures, and mutation diagnostics.
- issue 062f is done for supported `name` / `length` function object metadata.
- issue 062g is done for heap closure object ABI, dispatch, and GC rooting for escaping returned closures.
- Dynamic `eval`, Annex B semantics, generators, async functions, mutable captured environments, and broader closure dispatch remain outside this epic and are covered by separate issue scopes.

## Affected test files

- `reference/test262/test/annexB/built-ins/Function/createdynfn-no-line-terminator-html-close-comment-params.js`
- `reference/test262/test/annexB/built-ins/String/prototype/anchor/attr-tostring-err.js`
- `reference/test262/test/annexB/built-ins/String/prototype/anchor/this-val-tostring-err.js`
- `reference/test262/test/annexB/built-ins/String/prototype/big/this-val-tostring-err.js`
- `reference/test262/test/annexB/built-ins/String/prototype/blink/this-val-tostring-err.js`
- `reference/test262/test/annexB/built-ins/String/prototype/bold/this-val-tostring-err.js`
- `reference/test262/test/annexB/built-ins/String/prototype/fixed/this-val-tostring-err.js`
- `reference/test262/test/annexB/built-ins/String/prototype/fontcolor/attr-tostring-err.js`
- `reference/test262/test/annexB/built-ins/String/prototype/fontcolor/this-val-tostring-err.js`
- `reference/test262/test/annexB/built-ins/String/prototype/fontsize/attr-tostring-err.js`
- ... and 37 more files

## Completion evidence

Fill only when moving to `done/`.

Commits:

- `062b`: done; dynamic Function constructor diagnostics and policy
- `062c`: `446224c`
- `062d`: `bf80f0b`
- `062e`: `a558aca269c61f0ba64f82d6799d729874930b0f`, `b1e9a98c8fc94ccf794998ba97376045e7438cb9`, `115d5cf74a9d19840303ff951463264529deb415`, `29d57aced2fdcc3273ead0997bac39797780e0e5`
- `062f`: `6448031`
- `062g`: `50e36ded2d68eb09dc29d5ed7fcd7723bc49c867`, `b1e9a98c8fc94ccf794998ba97376045e7438cb9`, `115d5cf74a9d19840303ff951463264529deb415`, `29d57aced2fdcc3273ead0997bac39797780e0e5`

Validation result:

```text
command: cargo fmt --all --check
result: passed
date: 2026-04-29

command: cargo nextest run -E 'test(function) or test(arguments) or test(closure) or test(node_diff)'
result: passed (33 tests run, 33 passed, 476 skipped)
date: 2026-04-29

command: mise run update-issue-index
result: passed
date: 2026-04-29

command: mise run update-issue-index -- --check
result: passed
date: 2026-04-29

command: mise run check issues
result: passed
date: 2026-04-29

command: cargo nextest run
result: passed (505 tests run, 505 passed, 4 skipped)
date: 2026-04-29
```

Remaining risks:

- Dynamic `Function(...)` and `new Function(...)` remain diagnostic-only by policy.
- Dynamic `eval` and Annex B function declaration semantics remain separate issue 225 work.
- Mutable captured environments, generators, async functions, and broader closure dispatch forms remain outside this epic close and are tracked by separate issue scopes.

## Reopened by audit

Date: 2026-05-05

Classification: must-reopen.

Reopen reason: frontmatter still says `class: blocked`, which is incompatible with a completed issue unless explicit supersedence/closure evidence is present.

Violated acceptance: the issue cannot provide repo-local close evidence for its checked acceptance criteria while it remains in this state. Acceptance checkboxes were reset for re-verification.

Evidence files:
- `issues/done/062-implement-function.md` before this move
- `issues/done/062-implement-function.md` after this move

Split follow-up: none created in this audit wave; this reopened issue remains the tracking item.

## False-done audit correction

Date: 2026-05-05

Classification: truly-done duplicate/superseded tracking issue.

Audit result: returned to `issues/done/` after review feedback. The title contains `(dup)`, which marks this issue as a duplicate/superseded tracker; duplicate closure issues must not remain in `issues/open/` unless the duplicate mapping itself is invalid.

Evidence files:
- `issues/done/062-implement-function.md` after this correction
- `issues/index.md` after regeneration
