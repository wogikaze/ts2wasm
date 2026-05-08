---
id: 5404
title: "Bind dotted ambient namespace qualified access"
type: feature
area: frontend/resolver
class: implementation-ready
priority: P1
depends_on: [5370]
blocks: []
created: 2026-05-08
updated: 2026-05-08
---

## Summary

Bind same-file dotted ambient namespaces so `declare namespace Foo.Bar { ... }`
creates resolver metadata for later `Foo.Bar.foo` access, without emitting a
runtime namespace object.

## Problem

Problem: `ambientModules.ts` currently reaches `UnresolvedName` for `Foo` in
`Foo.Bar.foo = 5`.

## Current failure

```sh
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/ambientModules.ts
```

```text
source: declare namespace Foo.Bar { export var foo; }; Foo.Bar.foo = 5;
coverage: unsupported_diagcodes=UnresolvedName:1, unsupported_features=name-resolution:1
triage: UnresolvedName unresolved name: `Foo` at 90..93
tokens/ast: ok through dotted declare namespace and `Foo.Bar.foo = 5`
oracle: TypeScript diagnostics=[]
```

## Desired final state

`Foo.Bar.foo` binds past the ambient namespace path or advances to a narrower
source-spanned namespace/member diagnostic.

## Scope

In scope:

- [ ] Bind `declare namespace Foo.Bar { export var foo; }` for same-file qualified access.
- [ ] Preserve ambient erasure: no runtime namespace initialization.
- [ ] Re-run `ambientModules.ts` and record any next blocker.

Out of scope:

- Base ambient namespace root binding, tracked by issue 5370.
- Non-ambient namespaces, tracked by issue 5287.
- Full namespace runtime object materialization.

## Affected paths

Expected: `crates/frontend/src/`, `crates/ir/src/`, focused resolver tests.

## Acceptance criteria

- [ ] `ambientModules.ts` no longer reports `UnresolvedName` for `Foo` at `90..93`.
- [ ] A focused regression covers `declare namespace Foo.Bar { export var foo; } Foo.Bar.foo = 5;`.
- [ ] Ambient namespace declarations remain erased.

## Validation

Required:

```sh
cargo fmt --all --check
cargo nextest run -p ts2wasm-ir -E 'test(namespace) or test(resolve)'
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/ambientModules.ts
python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/ambientModules.ts --detail --no-dashboard-data
```

## Docs / current-state / issue sync

Final-state docs: not affected.
Current state: not affected.
Follow-up issues: none.

## Notes

Split from `issues/done/620-implement-ambientModules.md` on 2026-05-08.
Depends on 5370 because root ambient namespace binding must exist first.

## Completion evidence

Fill only when implemented.
