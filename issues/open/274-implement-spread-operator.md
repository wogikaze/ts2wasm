---
id: 274
title: Implement spread operator
type: feature
area: frontend/semantics
class: implementation-ready
priority: P2
tracking: feature:spread-operator
---

## Summary

Spread operator (`...`) is not implemented. This prevents expanding iterables into function arguments, array literals, and object literals.

## Evidence

AtCoder ABC451 D problem uses spread operator for array operations:

```typescript
const arr = [...iterable];
```

Test262 test case: `reference/test262/test/language/expressions/call/spread-sngl-literal.js`

```javascript
var callCount = 0;

(function() {
  assert.sameValue(arguments.length, 3);
  assert.sameValue(arguments[0], 3);
  assert.sameValue(arguments[1], 4);
  assert.sameValue(arguments[2], 5);
  callCount += 1;
}(...[3, 4, 5]));

assert.sameValue(callCount, 1);
```

Current behavior: UnsupportedSyntax error for spread operator.

## Acceptance criteria

1. Parser accepts spread operator syntax
2. Spread in function arguments works
3. Spread in array literals works
4. Spread in object literals works
5. Iterator protocol integration
6. Handles sparse arrays correctly
7. Test262 spread operator tests pass

## Validation

```bash
cargo nextest run
cargo fmt --all --check
```

## Notes

- Spread operator is part of ES6 specification
- Requires iterator protocol implementation
- Should work with Array, Set, Map, and custom iterables
- Consider rest parameter syntax in parallel (related feature)

## Progress

- 2026-04-29: Added the smallest safe slice for issue 274:
  - parser accepts spread syntax in array literals, object literals, and existing call arguments;
  - literal-array call spread such as `sum(...[3, 4, 5])` remains executable and is covered by a Node/iwasm fixture;
  - dynamic call spread, array literal spread, and object literal spread now fail with explicit `issue-274` unsupported diagnostics instead of falling through to generic spread/lowering errors.
- Remaining acceptance criteria are not complete: iterator protocol integration, sparse arrays, object property enumeration, Set/Map/custom iterable spread, and broad Test262 spread coverage still require follow-up implementation.
- 2026-04-29: Added a follow-up executable array-literal slice:
  - array literal spread over dense array literals, such as `[1, ...[2, 3], 4]`, lowers to the existing dense `ArrayNew` representation and matches Node/iwasm output;
  - dynamic array literal spread, object literal spread, and dynamic call spread remain explicitly guarded by `issue-274` unsupported diagnostics.
- 2026-04-29: Added the next executable object-literal slice:
  - object literal spread over static object literal operands, such as `{ left: 0, ...{ a: 1, b: 2 }, b: 3 }`, lowers by flattening those literal properties into the existing `ObjectNew` representation and matches Node/iwasm output;
  - dynamic object spread, including spreading a local object value, remains explicitly guarded by an `issue-274` unsupported diagnostic because the runtime still lacks general own-enumerable property copy/enumeration semantics.
- 2026-04-29: Added a focused function-expression call slice:
  - anonymous function expressions now parse in call position, and direct calls such as `(function(a, b, c) { console.log(a + b + c); }(...[3, 4, 5]))` lower to a generated function call with literal-array spread expansion and match Node/iwasm output;
  - broader function-expression spread calls that use `this`, `arguments`, rest parameters, or mutable captured outer locals remain explicitly guarded. The representative test262 case `language/expressions/call/spread-sngl-literal.js` now reaches the spread/IIFE boundary and reports `issue-274: direct function-expression spread calls with this or arguments require broader call-expression runtime support`.
- 2026-04-29: Added a Set-to-array spread slice:
  - array literal spread over a known `Set` local, such as `let copy = [...set]`, lowers through the existing `SetValuesArray` runtime helper and preserves insertion order under Node/iwasm differential coverage;
  - mixed array literals such as `[0, ...set]`, dynamic iterable spread, Map/custom iterable spread, and general iterator protocol semantics remain out of scope.
- 2026-04-29: Extended the Set-to-array spread slice to mixed dense array literals:
  - literals such as `[0, ...set, 4]` lower as dense array segments concatenated with `SetValuesArray(set)`, preserving Set insertion order under Node/iwasm differential coverage;
  - Map/custom iterator/general iterator protocol, object spread beyond the existing static object-literal slice, and dynamic non-Set iterable spread remain guarded by `issue-274` diagnostics.
- 2026-04-30: Added a dense array local spread slice:
  - array literals such as `[0, ...base, 3]` where `base` is a known dense array local lower through the existing `ArrayConcat` path and match Node/iwasm output;
  - string/custom iterable spread, sparse arrays, object spread beyond static object literals, and general iterator protocol semantics remain guarded by `issue-274` diagnostics.
