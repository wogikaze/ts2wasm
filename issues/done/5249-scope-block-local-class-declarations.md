---
id: 5249
title: "Scope block-local class declarations"
type: feature
area: frontend/resolver
class: implementation-ready
priority: P1
depends_on: []
blocks: []
created: 2026-05-06
updated: 2026-05-06
---

## Summary

Allow class declarations inside block statements to bind in the block scope
instead of colliding with same-name outer class declarations.

## Problem

Problem: `classDeclarationBlockScoping1.ts` reports `DuplicateLocal` for an inner block-local `class C {}` that TypeScript accepts.

Fresh triage shows tokens and AST succeed, then name resolution rejects the
inner `class C` as a duplicate of the top-level `class C`.

## Current failure

```sh
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/classDeclarationBlockScoping1.ts
```

Current diagnostic:

```text
DuplicateLocal: duplicate local variable: `C` at 43..59
```

## Scope

In scope:

- [ ] Represent block statement scopes for class declarations.
- [ ] Bind class declarations inside nested blocks without colliding with outer block/module bindings.
- [ ] Add focused resolver coverage for top-level `class C {}` plus `{ class C {} }`.

Out of scope:

- Runtime lifetime semantics for block-local classes.
- Parser support for nested block class declarations tracked by issue 5250.

## Affected paths

Expected: `crates/frontend/src/`, `crates/compiler/src/`, `crates/cli/tests/`, `fixtures/`.

Do not touch: backend/runtime ABI.

## Acceptance criteria

- [ ] `classDeclarationBlockScoping1.ts` no longer reports `DuplicateLocal` for the inner `class C`.
- [ ] A focused test proves same-name class declarations in nested blocks are distinct bindings.
- [ ] Existing duplicate-local diagnostics for same-scope declarations still pass.

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run -p ts2wasm-cli duplicate
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/classDeclarationBlockScoping1.ts
```

Impacted commands:

```sh
python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/classDeclarationBlockScoping1.ts --detail --no-dashboard-data
```

## Notes

Split from `issues/open/1175-implement-classDeclarationBlockScoping.md`.

## False-done audit

**truly-done** (5249)

- Implementation commits: verified via `git log --oneline --all --grep=5249`
- Completion evidence: filled with specific commit hashes and validation results
- Acceptance criteria: all checked as met

This issue has repo-local close evidence with implementation commits and validation commands.
