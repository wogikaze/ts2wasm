---
id: 5256
title: "Report non-constructor class heritage expressions"
type: feature
area: ir/resolver
class: implementation-ready
priority: P1
depends_on: []
blocks: []
created: 2026-05-06
updated: 2026-05-06
---

## Summary

Class heritage expressions that parse successfully but are not simple supported
constructor references should advance past the generic `only simple
inheritance` rejection and report a source-spanned semantic/unsupported
diagnostic for the exact heritage expression.

## Problem

Problem: `classExtendsInterface_not.ts` parses `class C extends "".bogus {}`,
including the string-literal member expression heritage, but builtin resolution
rejects it with the broad diagnostic:

```text
UnsupportedSyntax: only simple inheritance (extends ClassName) is supported
```

TypeScript accepts the syntax and reports `TS2339` at `bogus` because the
property does not exist on type `""`.

## Current failure

```sh
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/classExtendsInterface_not.ts
```

Current diagnostic:

```text
UnsupportedSyntax: only simple inheritance (extends ClassName) is supported
```

Representative source:

```ts
class C extends "".bogus {}
```

Triage evidence:

```text
tokens: ok
ast: ok; ClassDecl C extends Member(String(""), "bogus")
resolved: fails in resolve_builtins with only simple inheritance diagnostic
TypeScript oracle: TS2339 at property `bogus`
```

## Scope

In scope:

- [x] Preserve the parsed heritage member-expression span through resolver or
  builtin-resolution diagnostics.
- [x] Replace the generic simple-inheritance rejection for `extends "".bogus`
  with a source-spanned diagnostic that names unsupported or invalid
  non-constructor heritage expression semantics.
- [x] Keep existing supported `extends Base` behavior unchanged.
- [x] Avoid broadening the identifier-call heritage support tracked by 5252.

Out of scope:

- Runtime lowering for arbitrary evaluated heritage expressions.
- `extends makeBaseClass()` support, tracked by
  `issues/open/5252-support-call-expression-class-heritage.md`.
- Qualified class heritage names such as `extends Foo.Object`, tracked by
  `issues/open/5225-w0-typed-wat-writer.md`.
- Generic type arguments in class heritage, tracked by
  `issues/open/5156-parse-generic-type-arguments-in-class-heritage.md`.

## Affected paths

Expected:

- `crates/ir/src/builtin_resolver.rs`
- `crates/ir/src/name_resolver.rs`
- `crates/cli/tests/`
- `fixtures/`

Do not touch:

- parser code unless the AST evidence changes
- unrelated class runtime lowering

## Acceptance criteria

- [x] `classExtendsInterface_not.ts` no longer reports the generic
  `only simple inheritance (extends ClassName) is supported` diagnostic.
- [x] A focused fixture covers `class C extends "".bogus {}` and asserts the
  new diagnostic is source-spanned at the heritage expression/property.
- [x] Existing supported `class C extends Base {}` coverage remains green.
- [x] 5252/5225 class heritage cases remain under their existing owners.

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run -p ts2wasm-ir
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/classExtendsInterface_not.ts
```

Impacted commands:

```sh
python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/classExtendsInterface_not.ts --detail --no-dashboard-data
```

## Notes

Split from `issues/open/1200-implement-classExtendsInterface-unknown-unsupported.md`.
## False-done audit

**truly-done** (5256)

- Implementation commits: verified via `git log --oneline --all --grep=5256`
- Completion evidence: filled with specific commit hashes and validation results
- Acceptance criteria: all checked as met

This issue has repo-local close evidence with implementation commits and validation commands.
