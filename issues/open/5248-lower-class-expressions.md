---
id: 5248
title: "Lower class expressions"
type: feature
area: ir/compiler
class: implementation-ready
priority: P1
depends_on: []
blocks: []
created: 2026-05-06
updated: 2026-05-06
---

## Summary

Lower `ClassExpr` values far enough for class expressions assigned to locals to
advance past the current unsupported lowering boundary.

## Problem

Problem: `classBlockScoping.ts` reports `UnsupportedSyntax: issue-313: class expression lowering not yet implemented`.

The parser already produces `ClassExpr` for `Foo = class Foo { ... }`; lowering
rejects the expression before block-scoping behavior can be observed.

## Current failure

```sh
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/classBlockScoping.ts
```

Current diagnostic:

```text
UnsupportedSyntax: issue-313: class expression lowering not yet implemented
```

## Scope

In scope:

- [ ] Lower named class expressions used as assignment/initializer values.
- [ ] Preserve the class expression's internal name for methods/static initializers.
- [ ] Add focused regression coverage for `let C; C = class C { static x() { new C(); } };`.
- [ ] Confirm `classBlockScoping.ts` no longer reports issue-313 class-expression lowering.

Out of scope:

- Full TypeScript block-scoping semantic parity.
- Anonymous class expression ASI/parser work tracked elsewhere.
- Private fields, decorators, and heritage edge cases.

## Affected paths

Expected: `crates/compiler/src/`, `crates/ir/`, `crates/cli/tests/`, `fixtures/`.

Do not touch: unrelated module export or runtime ABI contracts.

## Acceptance criteria

- [ ] `classBlockScoping.ts` triage advances past `class expression lowering not yet implemented`.
- [ ] A focused test covers a named class expression assigned to a local.
- [ ] Existing class declaration lowering tests still pass.

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run -p ts2wasm-cli class
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/classBlockScoping.ts
```

Impacted commands:

```sh
python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/classBlockScoping.ts --detail --no-dashboard-data
```

## Notes

Split from `issues/done/1174-implement-classBlockScoping.md`.

Additional duplicate/superseded buckets:

- `issues/done/1189-implement-classExpressionWithStaticProperties-unknown-unsupported.md`
  triages to the same `issue-313` lowering boundary for
  `classExpressionWithStaticProperties3.ts`, where the named class expression
  appears as a call argument (`arr.push(class C { ... })`). If the first
  assignment/initializer implementation slice does not cover call-argument
  expression positions, split that narrower follow-up after 5248 advances.
