---
id: 283
title: "Implement assigned arrow recursion"
type: feature
area: runtime/semantics
class: implementation-ready
priority: P2
depends_on: [273]
blocks: []
created: 2026-04-29
updated: 2026-04-29
status: done
completed: 2026-04-29
---

## Summary

Implement recursive calls through an arrow function stored in a local binding.

Problem: issue 273 closed ordinary function declaration and named function
expression recursion, but assigned arrow recursion still reports the existing
function-valued local call diagnostic.

## Current failure

```typescript
const fact = n => n === 1 || n * fact(n - 1);
console.log(fact(4));
```

Current result:

```text
issue-211: function-valued local calls such as extracted method `fact(...)` are not supported
```

## Desired final state

The compiler can call an arrow function through its assigned local binding when
the binding is the same arrow closure value, including self-recursive calls from
the arrow body.

## Scope

In scope:

- [x] Resolve local arrow closure calls without going through method extraction.
- [x] Support self-recursive assigned arrow functions for the current closure representation.
- [x] Add Node/iwasm differential coverage for a base-case recursive arrow fixture.

Out of scope:

- Arbitrary function-valued local calls beyond tracked arrow closure bindings.
- Mutual recursion through reassigned function locals.
- Stack overflow protection beyond existing runtime behavior.

## Affected paths

Expected:

- `crates/ir/src/`
- `crates/backend-wasm/src/`
- `crates/cli/tests/m2_node_diff.rs`
- `fixtures/core-semantics/*recursive*`

Do not touch:

- unrelated builtin/runtime APIs

## Acceptance criteria

- [x] `fixtures/core-semantics/arrow-assigned-recursive-unsupported.ts` becomes a Node/iwasm differential fixture or is replaced by an equivalent supported fixture.
- [x] A recursive arrow base case terminates and returns the same value as Node.
- [x] Reassignment or unsupported callable-local cases remain diagnostic-backed.

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run -E 'test(recursive) or test(function) or test(node_diff)'
mise run update-issue-index -- --check
mise run check issues
```

## Notes

Split from issue 273 on 2026-04-29. The former diagnostic fixture
`fixtures/core-semantics/arrow-assigned-recursive-unsupported.ts` is now a
Node/iwasm differential fixture. The reassignment diagnostic guard is
`fixtures/core-semantics/arrow-assigned-recursive-reassigned-unsupported.ts`.

## Completion evidence

Implementation:

- Declaration-bound arrow functions lower with a self-closure binding when their body calls the same local binding.
- Backend `&&` / `||` emission now short-circuits through `TruthyBool`, which lets the recursive arrow base case terminate without evaluating the recursive branch.
- Reassigned callable-local behavior remains diagnostic-backed by the `issue-211` fixture.

Validation result:

```text
command: cargo fmt --all --check
result: PASS
date: 2026-04-29

command: cargo nextest run -E 'test(recursive) or test(function) or test(node_diff)'
result: PASS; 29 tests run, 29 passed, 507 skipped
date: 2026-04-29

command: mise run update-issue-index -- --check
result: PASS before lifecycle move
date: 2026-04-29

command: mise run check issues
result: PASS before lifecycle move
date: 2026-04-29

command: cargo nextest run
result: PASS; 532 tests run, 532 passed, 4 skipped
date: 2026-04-29
```
