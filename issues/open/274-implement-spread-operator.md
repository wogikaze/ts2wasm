---
id: 274
title: Implement spread operator
type: meta
area: frontend/semantics
class: ready
priority: P2
tracking: feature:spread-operator
blocks: [353, 354, 355]
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

This meta issue is complete when all child issues are moved to `done/`.

Already completed (not tracked by child issues):
1. Parser accepts spread operator syntax
2. Spread in function arguments works (literal arrays, dense array locals, ASCII strings, Set locals)
3. Spread in array literals works (dense array literals, Set locals, dense array locals, ASCII strings)
4. Spread in object literals works (static object literals, known object-literal locals)

Child issues:
- [ ] Issue 353: Implement iterator protocol integration for spread operator
- [ ] Issue 354: Implement sparse array spread support
- [ ] Issue 355: Implement dynamic object property enumeration spread

Remaining acceptance criteria:
5. Iterator protocol integration (issue 353)
6. Handles sparse arrays correctly (issue 354)
7. Test262 spread operator tests pass (all child issues)

## Validation

```bash
cargo nextest run
cargo fmt --all --check
```

## Notes

- Spread operator is part of ES6 specification
- Requires iterator protocol implementation (issue 353)
- Should work with Array, Set, Map, and custom iterables
- Consider rest parameter syntax in parallel (related feature)

Child issues:
- Issue 353: General iterator protocol (`Symbol.iterator`, `.next()`, `{value, done}`)
- Issue 354: Sparse array hole preservation in array and call spread
- Issue 355: Runtime object property enumeration for dynamic object spread

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
- 2026-04-29: Added a dense array local spread slice:
  - array literals such as `[0, ...base, 3]` where `base` is a known dense array local lower through the existing `ArrayConcat` path and match Node/iwasm output;
  - string/custom iterable spread, sparse arrays, object spread beyond static object literals, and general iterator protocol semantics remain guarded by `issue-274` diagnostics.
- 2026-04-29: Added a dense array local call-spread slice:
  - direct fixed-arity function calls such as `sum(...values)` where `values` is a known dense array local lower each formal argument through existing `ArrayGet` runtime reads and match Node/iwasm output;
  - string/custom iterable call spread, rest/`arguments`-observing callees, sparse arrays, and general iterator protocol semantics remain guarded by `issue-274` diagnostics.
- 2026-04-29: Added an ASCII string literal call-spread slice:
  - direct fixed-arity function calls such as `join(..."abc")` lower to one-character string arguments and match Node/iwasm output;
  - string locals, non-ASCII string iterator parity, custom iterable call spread, rest/`arguments`-observing callees, sparse arrays, and general iterator protocol semantics remain guarded by `issue-274` diagnostics.
- 2026-04-29: Added an ASCII literal-derived string local call-spread slice:
  - direct fixed-arity function calls such as `join(...copy)` where `copy` is assigned from a known ASCII string literal local lower to one-character string arguments and match Node/iwasm output;
  - non-ASCII string iterator parity, runtime-computed string locals, custom iterable call spread, sparse arrays, and general iterator protocol semantics remain guarded by `issue-274` diagnostics.
- 2026-04-29: Added an ASCII literal-derived string array-spread slice:
  - array literals such as `[..."ab", ...copy]`, where `copy` is assigned from a known ASCII string literal local, lower to one-character string elements and match Node/iwasm output;
  - runtime-computed string locals, non-ASCII string iterator parity, custom iterable spread, sparse arrays, and general iterator protocol semantics remain guarded by `issue-274` diagnostics.
- 2026-04-29: Added an ASCII static-concat string spread slice:
  - call and array spreads over locals derived from statically known string `+` concatenation, such as `let letters = "a" + "b"; join(...letters)` and `[...letters]`, lower to one-character string values and match Node/iwasm output;
  - runtime-computed string locals, non-ASCII string iterator parity, custom iterable spread, sparse arrays, and general iterator protocol semantics remain guarded by `issue-274` diagnostics.
- 2026-04-29: Added a known object-literal local spread slice:
  - object literals such as `{ z: 0, ...base, b: 3 }`, where `base` is a local initialized from static primitive object-literal properties and has no intervening assignment/property write, lower by flattening the tracked properties and match Node/iwasm output;
  - mutated object locals, dynamic property values, object property enumeration, and general dynamic object spread remain guarded by `issue-274` diagnostics.
- 2026-04-29: Added a dense array alias spread slice:
  - array and call spreads over a simple alias of a known dense array local, such as `let values = base; [0, ...values, 6]` and `sum(...values)`, now preserve the existing runtime array value and match Node/iwasm output;
  - sparse arrays, Map/custom iterator/general iterator protocol, runtime-computed or non-ASCII string spread, and dynamic object spread remain guarded by `issue-274` diagnostics.
- 2026-04-29: Added a simple object-spread alias slice:
  - object literals such as `{ z: 0, ...values, b: 3 }`, where `values` is a simple alias of a known static object-literal local and neither local is assigned or property-mutated before spread, lower by flattening the tracked properties and match Node/iwasm output;
  - alias-source assignment/property mutation conservatively invalidates the tracked static object-spread alias instead of copying stale properties; dynamic object enumeration and mutated object spread remain guarded by `issue-274` diagnostics.
- 2026-04-30: Added a known Set local call-spread slice:
  - fixed-arity direct calls such as `join(...letters)`, where `letters` is a known `Set` local, lower each formal argument through the existing `SetValuesArray` plus `ArrayGet` path and preserve insertion order under Node/iwasm differential coverage;
  - rest/`arguments`-observing callees, receiver-dependent calls, Map/custom iterator/general iterator protocol, sparse arrays, and dynamic non-Set iterable call spread remain guarded by existing issue-274 diagnostics.
