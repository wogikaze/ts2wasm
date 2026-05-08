---
id: 1467
title: "Implement Constraintsthatreferenceothercontstraints"
type: spike
area: frontend/syntax
class: done
priority: P1
depends_on: []
blocks: []
created: 2026-05-01
updated: 2026-05-07
completed: 2026-05-07
status: done
---
> **Reopened by false-done audit** (2026-05-07)
> Classification: must-reopen
> Reason: Batch-migrated to issues/done/ without implementation commits.
> Evidence: Empty completion evidence. No feat/fix commit for #1467.

## Summary

Closed as superseded. Fresh triage shows
`reference/typescript/tests/cases/compiler/constraintsThatReferenceOtherContstraints1.ts`
currently stops at the generic type-argument skipper boundary already owned by
`issues/open/5309-skip-generic-type-arguments-in-type-annotations.md`.

## Problem

Reference test results previously grouped this file under constraints that
reference other constraints. Current compiler behavior does not reach generic
constraint semantics because the type annotation skipper stops at the comma
inside `Foo<Object, Object>` in a class property declaration.

Problem: `constraintsThatReferenceOtherContstraints1.ts` is blocked by generic
type arguments inside type annotations before constraint behavior can be
triaged.

## Current failure

Representative reproduction:

```sh
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/constraintsThatReferenceOtherContstraints1.ts
```

Coverage window:

```sh
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/constraintsThatReferenceOtherContstraints1.ts --detail --no-dashboard-data
```

## Desired final state

This generated bucket is closed as superseded by
`issues/open/5309-skip-generic-type-arguments-in-type-annotations.md`. Do not
implement directly from this bucket.

## Scope

In scope:

- [x] Inspect fresh smart triage for the affected file.
- [x] Confirm the current first blocker is covered by issue 5309.
- [x] Preserve exact reproduction commands and representative diagnostic/AST
      evidence in this closed issue and the owner issue.

Out of scope:

- Direct implementation from this generated bucket.
- Generic constraint checking.
- Structural type checking for object type literals.
- Definite assignment diagnostics for class fields.

## Affected paths

Expected:

- `crates/frontend/src/parser/tokens.rs`
- `crates/frontend/src/parser/statements_class.rs`
- `crates/frontend/src/parser/tests.rs`

Do not touch:

- typechecker/generic constraint logic until the annotation parser advances

## Acceptance criteria

- [x] Existing issue 5309 is confirmed as the current first-blocker owner.
- [x] This closed issue includes failing path, diagnostic code, source context,
      token evidence, and TypeScript AST evidence.
- [x] Completion evidence names the exact reference path and current
      diagnostic/stdout change.

## Validation

Required commands:

```sh
python scripts/manager.py update-issue-index --check
python scripts/manager.py check-issue-health
python scripts/manager.py check-issue-readiness -- --fail-ready-below 80
git diff --check
```

Impacted commands:

```sh
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/constraintsThatReferenceOtherContstraints1.ts --detail --no-dashboard-data
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/constraintsThatReferenceOtherContstraints1.ts
```

Not run:

- `cargo fmt --all --check`; issue cleanup only, no Rust code changed
- `cargo nextest run`; issue cleanup only, no implementation changed

## Docs / current-state / issue sync

Final-state docs:

- [x] not affected

Current state:

- [x] not affected

Follow-up issues:

- [x] none; current first blocker is already tracked by issue 5309

## Notes

## Affected test files

- `reference/typescript/tests/cases/compiler/constraintsThatReferenceOtherContstraints1.ts`

## Duplicate detection

Current first blocker is covered by
`issues/open/5309-skip-generic-type-arguments-in-type-annotations.md`.

Resolution:

```text
Superseded by issue 5309. The active diagnostic is the generic type-argument
skipper boundary at `Foo<Object, Object>` inside a class property annotation.
```

## Smart triage

### Smart triage: Triage parser syntax: constraintsThatReferenceOtherContstraints1

- Issue class: `triage-needed`
- Feature label: `parser-syntax`
- Diagnostic: `UnsupportedSyntax` / `parser-or-frontend-unsupported`
- Path: `reference/typescript/tests/cases/compiler/constraintsThatReferenceOtherContstraints1.ts`

Reproduction:

```sh
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/constraintsThatReferenceOtherContstraints1.ts
```

Coverage reproduction:

```sh
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/constraintsThatReferenceOtherContstraints1.ts --detail --no-dashboard-data
```

Coverage result:

```text
executed=1
build_pass=0
unsupported=1
unsupported_diagcodes=UnsupportedSyntax:1
unsupported_features=unknown-unsupported:1
```

Current diagnostic:

```text
UnsupportedSyntax: expected property name, got Comma at 141..147
```

Source context:

```ts
class Foo<T, U extends T> { }
class Bar<T extends Object, U extends T> {
    data: Foo<Object, Object>;
}

var x: Foo< { a: string }, { a: string; b: number }>;
```

Compiler evidence:

- Tokenization succeeds through `Foo<Object, Object>` and includes the comma
  inside the type argument list.
- AST and resolved construction fail before representing the class property
  declaration.
- Visible symbols include classes `Foo` and `Bar` before the parser failure.

TypeScript oracle evidence:

- TypeScript parses `data: Foo<Object, Object>;` as
  `ClassDeclaration -> PropertyDeclaration -> TypeReference`.
- TypeScript also parses the later variable annotation
  `Foo<{ a: string }, { a: string; b: number }>` and reports later TS2564 for
  `data` definite assignment.
- Constraint diagnostics are not reached before this parser boundary.

## Completion evidence

Commits:

- `...`

Validation result:

```text
command: env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/constraintsThatReferenceOtherContstraints1.ts
result: pass; current first blocker is the same generic type-argument skipper support tracked by issue 5309
date: 2026-05-07

command: env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/constraintsThatReferenceOtherContstraints1.ts --detail --no-dashboard-data
result: pass; executed=1 unsupported=1 unsupported_diagcodes=UnsupportedSyntax:1 unsupported_features=unknown-unsupported:1
date: 2026-05-07
```

Remaining risks:

- After issue 5309 advances generic type-argument skipping in annotations, this
  file may expose generic constraint checking, object type literal parsing in
  annotations, or class field definite-assignment diagnostic fidelity.
