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

- [ ] Resolve local arrow closure calls without going through method extraction.
- [ ] Support self-recursive assigned arrow functions for the current closure representation.
- [ ] Add Node/iwasm differential coverage for a base-case recursive arrow fixture.

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

- [ ] `fixtures/core-semantics/arrow-assigned-recursive-unsupported.ts` becomes a Node/iwasm differential fixture or is replaced by an equivalent supported fixture.
- [ ] A recursive arrow base case terminates and returns the same value as Node.
- [ ] Reassignment or unsupported callable-local cases remain diagnostic-backed.

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run -E 'test(recursive) or test(function) or test(node_diff)'
mise run update-issue-index -- --check
mise run check issues
```

## Notes

Split from issue 273 on 2026-04-29. The current diagnostic fixture is
`fixtures/core-semantics/arrow-assigned-recursive-unsupported.ts`.
