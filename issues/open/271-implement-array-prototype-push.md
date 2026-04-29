---
id: 271
title: Implement Array.prototype.push
type: feature
area: runtime/builtins
class: implementation-ready
priority: P2
tracking: feature:array-prototype-methods
---

## Summary

Array.prototype.push is only partially covered. Direct `arr.push(value)` has an existing runtime helper and build smoke, but the issue evidence and acceptance criteria require the `Array.prototype.push` builtin boundary, including array-like `call`/method extraction behavior.

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

This does not close the issue because the reference-backed `Array.prototype.push` boundary is still missing. A 2026-04-29 attempted close added only an unconnected root fixture and did not validate the Test262-style array-like case.

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
error: [UnresolvedName] unresolved name: `Array`
```

## Acceptance criteria

1. [x] Direct `arr.push(value)` is available on supported Array objects.
2. [x] Direct `arr.push(value)` appends to the end of supported arrays.
3. [x] Direct `arr.push(value)` returns the new length.
4. [ ] Multiple arguments are handled correctly.
5. [ ] `Array.prototype.push` exists as an observable builtin.
6. [ ] Array-like objects via `Array.prototype.push`/method extraction/call-style behavior are supported or explicitly split with diagnostics.
7. [ ] Reference-backed Node/iwasm differential coverage exists for the completed slice.

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
