---
id: 062
title: "Implement function support"
type: feature
area: frontend
class: blocked
priority: P1
depends_on: []
blocks: []
created: 2026-04-26
updated: 2026-04-29
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
- Issue 063 was closed as superseded by issue 062b; its Annex B dynamic
  Function constructor cases are owned there.

## Problem

Reference test results show 47 cases fail with function diagnostic. The compiler cannot handle these syntax/semantics, preventing compilation of code in this category.

## Desired final state

function feature is correctly implemented according to JavaScript/TypeScript specifications. Related diagnostics are only emitted for genuinely unsupported cases.

## Scope

In scope for the epic:

- [ ] Track the child issue list.
- [ ] Keep shared function support constraints discoverable.
- [ ] Close only after all child issues are closed or explicitly superseded.

Implementation scope belongs to child issues:

- [x] Dynamic Function constructor diagnostics and policy: issue 062b
- [ ] Ordinary function declarations and direct calls: issue 062c
- [ ] Function receiver `this` and `arguments`: issue 062d
- [ ] Closures and captured lexical environments: issue 062e
- [ ] Function object metadata: issue 062f

Out of scope:

- [ ] Related features (separate issues)

## Affected paths

Expected:

- `crates/frontend/src/`
- `crates/cli/src/`
- `fixtures/`

Do not touch:

- `crates/runtime-abi/`
- `crates/backend-wasm/`

## Acceptance criteria

- [ ] All child issues listed in the queue design note are closed or explicitly superseded.
- [ ] Related function diagnostics are reduced in reference tests through child issues.
- [ ] Regression coverage exists for each supported function semantic surface.
- [ ] Docs/current-state are updated by child issues when semantics change.

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

- [ ] not affected

Current state:

- [ ] updated: `current-state.md` (repo root)

Follow-up issues:

- [ ] none

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

- `...`

Validation result:

```text
command:
result:
date:
```

Remaining risks:

- none
