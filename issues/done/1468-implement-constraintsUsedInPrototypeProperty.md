---
id: 1468
title: "Implement Constraintsusedinprototypeproperty"
type: spike
area: frontend/syntax
class: superseded
priority: P1
depends_on: [5000]
blocks: []
created: 2026-05-01
updated: 2026-05-07
---

## Summary

Superseded by issue 5192, which already tracks first-class class constructor
values and names constructor/prototype/class-value support as the missing runtime
boundary.

## Problem

Fresh triage shows the representative file parses successfully and exposes a
top-level class symbol, but the compiler still loses the class declaration when
the class is used as a runtime constructor value for `Foo.prototype`.

## Current failure

Representative reproduction:

```sh
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/constraintsUsedInPrototypeProperty.ts
```

Coverage window:

```sh
python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/constraintsUsedInPrototypeProperty.ts --detail --no-dashboard-data
```

Current diagnostic:

```text
error: [UnresolvedName] unresolved name: `Foo`
```

Source context:

```ts
// @target: es2015
class Foo<T extends number, U, V extends string> { }
Foo.prototype; // Foo<any, any, any>
```

Triage evidence:

```text
tokens: ok
ast: ok; ClassDecl Foo with TypeScript generic parameters erased, followed by Member Ident("Foo").prototype
visible symbols: class Foo at line 2 column 1
resolved pipeline: validate_ast, module_graph, resolve_names, resolve_builtins, build_typed_ir, then lower_program reports UnresolvedName Foo
TypeScript oracle: ok, diagnostics: []
```

Coverage result:

```text
executed=1
build_pass=0
unsupported=1
unsupported_diagcodes=UnresolvedName:1
unsupported_features=name-resolution:1
```

## Desired final state

Issue 5192 implements the underlying class constructor value support, including
direct `C.prototype` reads, so this generated bucket no longer needs a separate
blocked issue.

## Scope

Superseded by:

- `issues/done/5192-support-first-class-class-constructor-values.md`

Rationale:

- `Foo.prototype` requires the class declaration to be represented as a runtime constructor value.
- The existing implementation-ready owner already covers constructor/prototype/class-value support.
- The parser and TypeScript oracle evidence show this is not a frontend syntax bucket.

## Acceptance criteria

- [x] Duplicate candidates below are confirmed as no-match or this issue is superseded
- [x] This superseded issue preserves an exact `reference-triage` command for the bucket
- [x] This superseded issue includes failing path, diagnostic code, source context, visible symbols, and parser/TypeScript AST evidence
- [x] Existing implementation owner covers the class constructor/prototype/class-value runtime boundary

## Validation

Fresh evidence commands:

```sh
TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/constraintsUsedInPrototypeProperty.ts --detail --no-dashboard-data
TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/constraintsUsedInPrototypeProperty.ts
```

Issue lifecycle validation:

```sh
python scripts/manager.py update-issue-index
python scripts/manager.py update-issue-index --check
python scripts/manager.py check-issue-health
python scripts/manager.py check-issue-readiness -- --fail-ready-below 80
git diff --check
```

Not run:

- full compiler/runtime gate; issue metadata only

## Docs / current-state / issue sync

Final-state docs:

- [x] not affected

Current state:

- [x] not affected

Follow-up issues:

- [x] existing: issue 5192

## Notes

## Affected test files

- `reference/typescript/tests/cases/compiler/constraintsUsedInPrototypeProperty.ts`

## Duplicate detection

- none found by path/title/feature scan

## Smart triage

Superseded by the fresh triage evidence above.

## Completion evidence

Commits:

- filled by local commit that moves this issue to `done/`

Validation result:

```text
command: python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/constraintsUsedInPrototypeProperty.ts --detail --no-dashboard-data
result: pass; executed=1 build_pass=0 unsupported=1 unsupported_diagcodes=UnresolvedName:1 unsupported_features=name-resolution:1
date: 2026-05-07

command: python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/constraintsUsedInPrototypeProperty.ts
result: pass; parser and TypeScript oracle ok; lower_program reports UnresolvedName Foo for Foo.prototype
date: 2026-05-07
```

Remaining risks:

- implementation remains tracked by issue 5192
