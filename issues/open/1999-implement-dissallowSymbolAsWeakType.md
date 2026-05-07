---
id: 1999
title: "Report symbol WeakSet.add diagnostics"
type: feature
area: frontend/resolver
class: implementation-ready
priority: P1
depends_on: []
blocks: []
created: 2026-05-01
updated: 2026-05-07
---

## Summary

Handle the first `symbol` weak-collection negative blocker by resolving the
`Symbol` call and `WeakSet.add` far enough to report a TypeScript-style
diagnostic instead of stopping at resolver/lowering lookup failures.

## Problem

Fresh triage on 2026-05-07 shows both
`acceptSymbolAsWeakType.ts` and `dissallowSymbolAsWeakType.ts` parse and reach
the resolver/lowerer, but stop before the first TypeScript diagnostic for using
a `symbol` value with `WeakSet.add`.

Problem: weak collection symbol negative tests currently fail with
`UnresolvedFunction: Symbol` / `method WeakSet.add not found` instead of a
source-spanned TS2345-style diagnostic for `ws.add(s)`.

## Current failure

Representative reproduction:

```sh
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/dissallowSymbolAsWeakType.ts
```

Coverage window:

```sh
python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/dissallowSymbolAsWeakType.ts --detail --no-dashboard-data
```

## Desired final state

The compiler should recognize the relevant builtin call and `WeakSet.add`
surface well enough to emit the expected negative diagnostic for a symbol
argument. The exact reference files should no longer stop at `Symbol` function
resolution or `WeakSet.add` method lookup.

## Scope

In scope:

- [ ] Resolve the first `Symbol('s')` / `WeakSet.add` lookup path for the focused weak-set symbol fixture.
- [ ] Emit a source-spanned diagnostic for `ws.add(s)` where TypeScript requires an object argument and `s` is symbol-typed.

Out of scope:

- General collection runtime implementation beyond the first `WeakSet.add` diagnostic in these reference files.
- `WeakMap` methods, `WeakRef`, and `FinalizationRegistry`; split follow-up issues if they become the next blockers.
- Full lib.esnext / lib.es2022 declaration modeling outside this focused builtin/method diagnostic slice.
- Runtime support for weak references or finalization.

## Affected paths

Expected:

- `crates/frontend/src/`
- `crates/ir/src/`
- `crates/cli/tests/`
- focused fixtures under the existing CLI/frontend test layout

Do not touch:

- unrelated runtime/backend code unless the resolver produces a supported runtime shape
- broad collection/runtime ABI work outside this diagnostic slice

## Acceptance criteria

- [ ] `dissallowSymbolAsWeakType.ts` no longer reports `unresolved function: Symbol` or `method WeakSet.add not found`.
- [ ] A focused fixture covers `const s: symbol = Symbol("s"); const ws = new WeakSet([s]); ws.add(s);` and reports the symbol/object mismatch at `s`.

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run -p ts2wasm-cli -E 'test(symbol) or test(weak) or test(diagnostic)'
```

Impacted commands:

```sh
python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/dissallowSymbolAsWeakType.ts --detail --no-dashboard-data
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/dissallowSymbolAsWeakType.ts
```

Not run:

- implementation gates; this issue is being refined into an implementation-ready owner

## Docs / current-state / issue sync

Final-state docs:

- [ ] not affected

Current state:

- [ ] not affected

Follow-up issues:

- [ ] none

## Notes

## Affected test files

- `reference/typescript/tests/cases/compiler/dissallowSymbolAsWeakType.ts`

## Duplicate detection

- none found by path/title/feature scan

## Smart triage

Generated 2026-05-07.

```text
commands:
env TS2WASM_BINARY=/home/wogikaze/wgkz/ts2wasm/target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/dissallowSymbolAsWeakType.ts

result:
UnresolvedFunction / function-resolution

current diagnostic:
unresolved function: `Symbol`

lowerer evidence:
tokens: ok
ast: ok
resolved/lowered: fails with `method WeakSet.add not found`

TypeScript oracle:
TS2769 on `new WeakSet([s])`; TS2345 on `ws.add(s)` with a `symbol` argument.
Later WeakMap, WeakRef, and FinalizationRegistry diagnostics are out of scope
for this first blocker slice.

decision:
Own the first `dissallowSymbolAsWeakType.ts` weak-set symbol blocker. Issue 560
has the same first blocker family and is closed as superseded by this owner.
```

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

- This issue intentionally scopes to the first `WeakSet.add` diagnostic and
  resolver/lowerer classification, not runtime weak collection support.
