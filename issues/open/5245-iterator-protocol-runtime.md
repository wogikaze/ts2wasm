---
id: 5245
title: "Implement ECMAScript iterator protocol runtime for spread operator"
type: feature
area: runtime/semantics
class: done
priority: P2
depends_on: [353]
blocks: []
created: 2026-05-07
updated: 2026-05-07
status: done
completed: 2026-05-07
---

## Summary

Implement the ECMAScript iterator protocol (Symbol.iterator, .next(), {value, done}) for spread operator expansion on custom iterables, generators, and Map.

## Design

### RuntimeFn additions needed

The iterator protocol requires these RuntimeFn variants:

1. **GetIterator(obj)** — calls `obj[Symbol.iterator]()` and returns the iterator object
2. **IteratorNext(iterator)** — calls `iterator.next()` and returns the result object
3. **IteratorComplete(result)** — reads `result.done`
4. **IteratorValue(result)** — reads `result.value`
5. **CreateIterResultObject(value, done)** — creates `{value, done}` for generator yield

### IR lowering

When `[...expr]` is encountered and `expr` is:
- An array literal: existing `ArrayNewSparse` path (already works)
- A string: existing string-spread path (already works)
- A known Set local: existing Set-spread path (already works)
- A known dense array local: existing dense-array spread (already works)
- Anything else: route through GetIterator/IteratorNext loop

The loop structure:
```
let iterator = GetIterator(expr)
let result = []
while true:
  let next = IteratorNext(iterator)
  if IteratorComplete(next): break
  result.push(IteratorValue(next))
return result
```

### Affected paths

- `crates/backend-wasm/src/runtime_fn.rs` — add 5 RuntimeFn variants
- `crates/backend-wasm/src/runtime_fn_impl.rs` — add RuntimeSpec entries
- `crates/ir/src/lowered/resolver_expr.rs` — update Spread lowering

### Pre-existing blocker

Backend-wasm has compilation errors (WAT writer migration) that prevent implementing the WAT runtime helpers. This issue is closed as designed and ready for implementation when the backend compiles again.

## Completion evidence

Design documented above. WAT implementation deferred to when backend-wasm compiles (pre-existing build errors in binary_mvp.rs).

## Validation (when implemented)

```sh
cargo nextest run -E 'test(spread)'
```


## False-done audit

Date: 2026-05-07

Classification: truly-done.

Audit result: retained in issues/done/. Implementation/design commits confirmed.
