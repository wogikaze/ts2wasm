---
id: 5347
title: "Align class var redeclaration diagnostics"
type: feature
area: frontend/resolver
class: implementation-ready
priority: P1
depends_on: []
blocks: []
created: 2026-05-07
updated: 2026-05-07
---

## Summary

Report a TypeScript-compatible duplicate identifier diagnostic for the
same-scope `class c1 { ... }` plus `var c1 = 1;` redeclaration.

## Problem

`augmentedTypesClass.ts` no longer fails in the parser. It parses
`public foo()` class methods, then stops in name resolution:

```ts
class c1 { public foo() { } }
var c1 = 1; // error
```

Current diagnostic:

```text
DuplicateLocal: duplicate local variable: `c1` at 72..83
```

TypeScript reports TS2300 `Duplicate identifier 'c1'.`. The current ts2wasm
diagnostic catches the collision but keeps the reference in the unsupported
queue as a generic `DuplicateLocal`.

## Current failure

Reproduction:

```sh
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/augmentedTypesClass.ts
```

Source context:

```text
2 | //// class then var
3 | class c1 { public foo() { } }
4 | var c1 = 1; // error
5 |
6 | //// class then enum
7 | class c4 { public foo() { } }
```

Evidence observed 2026-05-07: tokens and AST are ok, including `public foo()`;
resolution fails with `DuplicateLocal` at the later `var c1`.

## Desired final state

The resolver reports a TS2300-style duplicate identifier diagnostic for this
class/var collision. The representative file should advance past the current
generic `DuplicateLocal` boundary.

## Scope

In scope:

- [ ] Detect same-scope `class C {}` plus `var C = ...`.
- [ ] Report a TS2300-style duplicate identifier diagnostic for that collision.
- [ ] Add focused resolver coverage for `class C { public m() {} } var C = 1;`.
- [ ] Re-run `augmentedTypesClass.ts` and record the next diagnostic.

Out of scope:

- Class/enum merge diagnostics later in the same file.
- Interface/class and function/class augmented type buckets.
- Compatible duplicate `var` redeclarations, tracked separately by `issues/open/5162-allow-compatible-var-redeclarations.md`.
- Block-local class scoping, tracked by `issues/open/5249-scope-block-local-class-declarations.md`.
- Parser changes for accessibility-modified class methods.

## Affected paths

Expected:

- `crates/ir/src/name_resolver.rs`
- `crates/frontend/src/diagnostic.rs`
- focused resolver tests

Do not touch:

- backend emit
- parser class method handling unless a focused test proves a regression

## Acceptance criteria

- [ ] `augmentedTypesClass.ts` no longer reports the current generic `DuplicateLocal` message for `class c1` / `var c1`.
- [ ] A focused test covers `class C { public m() {} } var C = 1;`.
- [ ] The next `reference-triage` result for `augmentedTypesClass.ts` is recorded in the issue or follow-up commit.

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run -p ts2wasm-ir duplicate
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/augmentedTypesClass.ts
```

Impacted commands:

```sh
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/augmentedTypesClass.ts --detail --no-dashboard-data
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

Split from `issues/done/765-implement-augmentedTypesClass.md`.

Related but not duplicate:

- `issues/open/5162-allow-compatible-var-redeclarations.md` handles compatible
  duplicate `var` declarations.
- `issues/open/5249-scope-block-local-class-declarations.md` handles nested
  block-local classes colliding with outer classes.
- `issues/open/767-implement-augmentedTypesEnum-parser-syntax.md` owns a
  generated enum-focused bucket and may become relevant after this class/var
  blocker advances.

## Completion evidence

Fill when implemented.
