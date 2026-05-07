---
id: 5348
title: "Resolve const declarations before use diagnostics"
type: bug
area: frontend/resolver
class: implementation-ready
priority: P1
depends_on: []
blocks: []
created: 2026-05-07
updated: 2026-05-07
---

## Summary

Register lexical `const` bindings before resolving same-scope uses so const
references reach TypeScript-style use-before-declaration diagnostics instead of
generic `UnresolvedName`.

## Problem

`constDeclarations-useBeforeDefinition2.ts` parses successfully, but
`resolve_names` rejects `c;` before the later `const c = 0;`. TypeScript knows
the binding and reports TS2448/TS2454. `constDeclarations.ts` has a related
same-header case where `for (const c5 = 0, c6 = 0; c5 < c6; )` rejects `c6`
in the condition even though TypeScript resolves the second const declarator.

Problem: lexical const bindings are not registered early enough for
same-scope forward references and same-`for` initializer declarator references.

## Current failure

```sh
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/constDeclarations-useBeforeDefinition2.ts
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/constDeclarations.ts
```

Current diagnostics:

```text
UnresolvedName: unresolved name: `c` at 83..84
UnresolvedName: unresolved name: `c6` at 239..241
```

Representative source:

```ts
c;
const c = 0;

for (const c5 = 0, c6 = 0; c5 < c6; ) { break; }
```

TypeScript oracle:

```text
TS2448: Block-scoped variable 'c' used before its declaration.
TS2454: Variable 'c' is used before being assigned.
```

`constDeclarations.ts` is accepted by TypeScript; hints include `c6` as a
binding in the `for` initializer.

## Desired final state

The resolver knows lexical const declarations in the relevant scope before
resolving reads. The representative cases no longer stop at `UnresolvedName`;
the use-before-declaration case reaches a narrower TS2448/TS2454-style
diagnostic boundary, and the `for` initializer case resolves `c6`.

## Scope

In scope:

- [ ] Predeclare same-scope lexical `const` names before resolving expression reads.
- [ ] Register all declarators in a multi-declarator `const` list before resolving the `for` condition.
- [ ] Preserve duplicate lexical binding diagnostics.

Out of scope:

- Full TypeScript definite-assignment analysis.
- Runtime behavior for invalid use-before-declaration programs.
- Parser fixes for other `constDeclarations-*` buckets.

## Affected paths

Expected:

- `crates/ir/src/name_resolver.rs`
- `crates/ir/src/name_resolver_tests.rs`
- focused fixtures or CLI tests

Do not touch:

- backend/runtime lowering

## Acceptance criteria

- [ ] `constDeclarations-useBeforeDefinition2.ts` no longer reports generic `UnresolvedName` for `c`.
- [ ] `constDeclarations.ts` no longer reports generic `UnresolvedName` for `c6`.
- [ ] A focused resolver test covers `c; const c = 0;`.
- [ ] A focused resolver test covers `for (const a = 0, b = 0; a < b;) {}`.

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run -p ts2wasm-ir
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/constDeclarations-useBeforeDefinition2.ts
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/constDeclarations.ts
```

Impacted commands:

```sh
python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/constDeclarations --detail --no-dashboard-data
python scripts/manager.py check-issue-readiness -- --fail-ready-below 80
```

Not run:

- none

## Docs / current-state / issue sync

Final-state docs:

- [ ] not affected

Current state:

- [ ] not affected

Follow-up issues:

- [ ] none

## Notes

Split from generated bucket
`issues/open/1441-implement-constDeclarations-name-resolution.md`.

## Completion evidence

Fill only when implemented.
