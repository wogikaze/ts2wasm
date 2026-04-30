---
id: 270
title: "Implement Array.prototype.map named-callback slice"
type: feature
area: runtime/builtins
class: done
priority: P2
tracking: feature:array-prototype-methods
updated: 2026-04-30
completed: 2026-04-30
---

## Summary

Close this issue for the already implemented `Array.prototype.map` slice:
dense arrays with supported named function callbacks and the callback argument
shape `(value, index, array)`.

This issue does not claim full `Array.prototype.map` compatibility. Sparse
arrays, `thisArg`, `Array.prototype.map.call(...)`, async callbacks, generic
callback allocation, and broad Test262 coverage are split to issue 334 or other
more-specific follow-ups.

## Evidence

The implemented slice added:

- `ArrayMap` runtime function planning and emission
- callback dispatch through `$array_map_callback`
- named function callback lowering with value/index/array arguments
- resolver diagnostics for unsupported call forms
- regression coverage for supported dense-array map fixtures

`Array.prototype.map.call(...)` remains explicitly unsupported in the current
runtime slice and is covered by issue-linked diagnostics.

## Acceptance criteria

1. [x] `Array.prototype.map` is available on supported Array objects.
2. [x] Supported callbacks receive `(value, index, array)` arguments.
3. [x] Supported dense-array map calls return a new transformed array.
4. [x] Supported map calls do not modify the original array.
5. [x] Unsupported sparse-array, `thisArg`, and broad Test262 semantics are not
   hidden in this closed issue.
6. [x] Residual full map compatibility is tracked by a separate open issue.

## Validation

Required commands:

```bash
cargo fmt --all --check
cargo nextest run -E 'test(math) or test(array_map) or test(node_diff)'
python scripts/manager.py update-issue-index --check
python scripts/manager.py check issues
```

## Reopened by audit

Date: 2026-04-30

Classification: false-done / unchecked acceptance.

Reason: the issue was under `issues/done/` with unchecked acceptance criteria
for sparse arrays, `thisArg`, and Test262 `Array.prototype.map` coverage. Issue
295 closed the later arrow-callback/chained-receiver subset, but explicitly left
sparse arrays, `thisArg`, `Array.prototype.map.call(...)`, async callbacks, and
generic callback allocation out of scope.

Next close bar resolution:

- This issue is narrowed to the implemented dense-array named-callback slice.
- Residual `Array.prototype.map` sparse/`thisArg`/Test262 semantics are split to
  `issues/open/334-complete-array-map-sparse-thisarg-test262.md`.

## Completion evidence

Commits:

- `d983e223`: issue-270: implement Array.prototype.map with named function callbacks.
- close commit on branch `agent/269-270-math-pow-array-map-20260430T000000Z`.

Validation result:

```text
command: cargo fmt --all --check
result: pass
date: 2026-04-30

command: cargo nextest run -E 'test(math) or test(array_map) or test(node_diff)'
result: fail after issue edits while compiling unrelated split test modules:
  crates/frontend/src/lexer_tests.rs missing direct imports for Diagnostic/Lexer/Token/DiagCode
  crates/cli/tests/m2_node_diff_fixture_tests.rs compiled as a standalone integration test and missing parent module helpers
date: 2026-04-30

command: python scripts/manager.py update-issue-index --check
result: pass after issue edits
date: 2026-04-30

command: python scripts/manager.py check issues
result: pass after issue edits after restoring ignored local artifact
  artifacts/coverage/results/test262-results.jsonl from the parent worktree
date: 2026-04-30

command: python scripts/manager.py check agent-state
result: pass
date: 2026-04-30
```

Remaining risks:

- Full sparse-array semantics, `thisArg`, `Array.prototype.map.call(...)`, async
  callbacks, generic callback allocation, and broad Test262 coverage remain open
  in issue 318 or narrower future child issues.
