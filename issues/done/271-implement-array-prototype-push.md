---
id: 271
title: Implement Array.prototype.push
type: feature
area: runtime/builtins
class: implementation-ready
priority: P2
tracking: feature:array-prototype-methods
completed: 2026-04-29
---

## Summary

Array.prototype.push is covered for the current supported boundary: direct array receivers, Test262-style assignment to an array-like object's `push` property followed by method call, and `Array.prototype.push.call` on an array-like object.

## Evidence

AtCoder ABC451 D problem uses array.push for building arrays:

```typescript
const arr = [];
arr.push(value);
```

Test262 test case: `reference/test262/test/built-ins/Array/prototype/push/S15.4.4.7_A2_T1.js`

```javascript
var obj = {};
obj.push = Array.prototype.push;

if (obj.length !== undefined) {
  throw new Test262Error('#0: var obj = {}; obj.length === undefined. Actual: ' + (obj.length));
} else {
  var push = obj.push(-1);
  if (push !== 1) {
    throw new Test262Error('#1: var obj = {}; obj.push = Array.prototype.push; obj.push(-1) === 1. Actual: ' + (push));
  }
  if (obj.length !== 1) {
    throw new Test262Error('#2: var obj = {}; obj.push = Array.prototype.push; obj.push(-1); obj.length === 1. Actual: ' + (obj.length));
  }
  if (obj["0"] !== -1) {
    throw new Test262Error('#3: var obj = {}; obj.push = Array.prototype.push; obj.push(-1); obj["0"] === -1. Actual: ' + (obj["0"]));
  }
}
```

## Current state

Direct array receiver calls are implemented for the current fixed-capacity array representation:

- `RuntimeFn::ArrayPush` exists.
- `emit_array_push` exists.
- `fixtures/builtins-and-io/array-push.ts` is covered by `build_smoke_array_push_method`.

The reference-backed `Array.prototype.push` boundary is now covered by `fixtures/builtins-and-io/array-prototype-push-array-like.ts`, which validates the Test262-style array-like case plus `Array.prototype.push.call(obj, 2, 3)` under Node/iwasm differential testing.

Current narrow reproduction:

```typescript
let obj = { length: 0 };
obj.push = Array.prototype.push;
console.log(obj.push(7));
console.log(obj.length);
console.log(obj[0]);
```

Current result:

```text
1
1
7
```

## Acceptance criteria

1. [x] Direct `arr.push(value)` is available on supported Array objects.
2. [x] Direct `arr.push(value)` appends to the end of supported arrays.
3. [x] Direct `arr.push(value)` returns the new length.
4. [x] Multiple arguments are handled correctly for direct identifier array receivers.
5. [x] `Array.prototype.push` exists as an observable builtin.
6. [x] Array-like objects via `Array.prototype.push`/method extraction/call-style behavior are supported or explicitly split with diagnostics.
7. [x] Reference-backed Node/iwasm differential coverage exists for the completed slice.

## Validation

Required commands:

```bash
cargo fmt --all --check
cargo nextest run -p ts2wasm-cli array_push
mise run update-issue-index -- --check
mise run check issues
```

## Notes

- Do not close this issue using direct `arr.push` smoke coverage alone.
- A safe next slice may either implement `Array.prototype.push` for the reference-backed array-like case or split the capacity/grow and prototype-call boundaries into smaller issues.
- Earlier unsafe work that weakened `Array.push` expectations or added hidden spare-capacity mutation without a clear grow contract was rejected.

## Progress 2026-04-29

- Direct identifier receiver `arr.push(a, b)` lowers through an `ArrayPushMany` runtime call that expands to repeated `$array_push` calls and returns the final length.
- Added `fixtures/builtins-and-io/array-push-multi-arg.ts` with Node/iwasm differential coverage for final length, updated `.length`, and appended indexes.
- Validation: `cargo fmt --all --check`; `cargo nextest run -p ts2wasm-cli array_push` (3 passed, 341 skipped).
- Added `fixtures/builtins-and-io/array-prototype-push-array-like.ts` for `obj.push = Array.prototype.push; obj.push(-1)` and `Array.prototype.push.call(obj, 2, 3)`.
- Runtime `$array_push` now handles object receivers by reading/writing the array-like `length` property and decimal index properties.
- Validation: `cargo fmt --all --check`; `cargo nextest run -p ts2wasm-cli array_push` (5 passed, 346 skipped).
- Remaining limitation: real array growth/reallocation beyond literal allocation remains outside this issue's completed array-like/prototype boundary.

## Completion evidence

Commits:

- `2063987b issue-271: implement array prototype push boundary`

Validation result:

```text
command: cargo fmt --all --check
result: PASS
date: 2026-04-29

command: cargo nextest run -p ts2wasm-cli array_push
result: PASS (5 passed, 346 skipped)
date: 2026-04-29

command: cargo nextest run -p ts2wasm-cli array_push_prototype_array_like
result: PASS (2 passed, 349 skipped)
date: 2026-04-29

command: node fixtures/builtins-and-io/array-prototype-push-array-like.ts
result: PASS; stdout: undefined / 1 / 1 / -1 / 3 / 3 / 2 / 3
date: 2026-04-29

command: cargo nextest run
result: PASS (543 passed, 4 skipped)
date: 2026-04-29

command: mise run update-issue-index -- --check
result: PASS
date: 2026-04-29

command: mise run check issues
result: PASS after restoring ignored generated artifact `artifacts/coverage/results/test262-results.jsonl` locally for issue-health path validation
date: 2026-04-29
```

## False-done audit

Date: 2026-05-05

Classification: truly-done.

Audit result: retained in `issues/done/`. The issue has repo-local completion evidence, is not marked `blocked` or `triage-needed`, and `python scripts/manager.py check issues` validates the resulting issue graph.

Evidence files:
- `issues/done/271-implement-array-prototype-push.md`
- `issues/index.md` after regeneration

Future-work tracking: no untracked future-work item was identified in this issue during this metadata/evidence audit.
