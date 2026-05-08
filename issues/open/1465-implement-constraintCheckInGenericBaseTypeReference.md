---
id: 1465
title: "Implement Constraintcheckingenericbasetypereference"
type: spike
area: frontend/semantics
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
> Reason: Batch-migrated to issues/open/ without implementation commits.
> Evidence: Empty completion evidence. No feat/fix commit for #1465.

## Summary

Closed as superseded. Fresh triage shows
`reference/typescript/tests/cases/compiler/constraintCheckInGenericBaseTypeReference.ts`
currently stops at the typed modified static class field parser boundary already
owned by `issues/open/5288-parse-typed-modified-static-class-fields.md`.

## Problem

Reference test results previously grouped this file under generic constraint
checking. Current compiler behavior does not reach generic base type constraint
semantics because class member parsing rejects `public static People: Derived`
before it can erase the TypeScript type annotation.

Problem: `constraintCheckInGenericBaseTypeReference.ts` is blocked by typed
modified static class field parsing before generic constraint behavior can be
triaged.

## Current failure

Representative reproduction:

```sh
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/constraintCheckInGenericBaseTypeReference.ts
```

Coverage window:

```sh
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/constraintCheckInGenericBaseTypeReference.ts --detail --no-dashboard-data
```

## Desired final state

This generated bucket is closed as superseded by
`issues/open/5288-parse-typed-modified-static-class-fields.md`. Do not implement
directly from this bucket.

## Scope

In scope:

- [x] Inspect fresh smart triage for the affected file.
- [x] Confirm the current first blocker is covered by issue 5288.
- [x] Preserve exact reproduction commands and representative diagnostic/AST
      evidence in this closed issue and the owner issue.

Out of scope:

- Direct implementation from this generated bucket.
- Generic constraint checking.
- Generic base type reference semantics.
- Static field runtime lowering.

## Affected paths

Expected:

- `crates/frontend/src/parser/statements_class.rs`
- `crates/frontend/src/parser/tests.rs`
- focused parser fixtures/tests

Do not touch:

- typechecker/generic constraint logic until this parser boundary advances

## Acceptance criteria

- [x] Existing issue 5288 is confirmed as the current first-blocker owner.
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
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/constraintCheckInGenericBaseTypeReference.ts --detail --no-dashboard-data
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/constraintCheckInGenericBaseTypeReference.ts
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

- [x] none; current first blocker is already tracked by issue 5288

## Notes

## Affected test files

- `reference/typescript/tests/cases/compiler/constraintCheckInGenericBaseTypeReference.ts`

## Duplicate detection

Current first blocker is covered by
`issues/open/5288-parse-typed-modified-static-class-fields.md`.

Resolution:

```text
Superseded by issue 5288. The active diagnostic is the modified static class
field parser boundary at `public static People: Derived`.
```

## Smart triage

### Smart triage: Triage type system: constraintCheckInGenericBaseTypeReference

- Issue class: `triage-needed`
- Feature label: `type-system`
- Diagnostic: `UnsupportedSyntax` / `parser-or-frontend-unsupported`
- Path: `reference/typescript/tests/cases/compiler/constraintCheckInGenericBaseTypeReference.ts`

Reproduction:

```sh
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/constraintCheckInGenericBaseTypeReference.ts
```

Coverage reproduction:

```sh
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/constraintCheckInGenericBaseTypeReference.ts --detail --no-dashboard-data
```

Coverage result:

```text
executed=1
build_pass=0
unsupported=1
unsupported_diagcodes=UnsupportedSyntax:1
unsupported_features=type-system:1
```

Current diagnostic:

```text
UnsupportedSyntax: expected LeftParen, got Some(Ident("People")) at 307..313
```

Source context:

```ts
class Container {
    public static People: Derived
}
```

Compiler evidence:

- Tokenization succeeds through `Class`, `Ident("Container")`, `{`,
  `Ident("public")`, `Static`, `Ident("People")`, `Colon`,
  `Ident("Derived")`.
- AST and resolved construction fail before representing the static property
  declaration.
- Visible symbols include the surrounding classes `Constraint`, `GenericBase`,
  `Derived`, `TypeArg`, and `Container`.

TypeScript oracle evidence:

- TypeScript parses `class Container { public static People: Derived }`.
- TypeScript AST path at the current blocker is
  `ClassDeclaration -> PropertyDeclaration -> Identifier "People"`.
- The oracle reports no TypeScript diagnostics for this file; generic
  constraint checking is not reached by the current compiler.

## Completion evidence

Commits:

- `...`

Validation result:

```text
command: env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/constraintCheckInGenericBaseTypeReference.ts
result: pass; current first blocker is the typed modified static class field parser support tracked by issue 5288
date: 2026-05-07

command: env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/constraintCheckInGenericBaseTypeReference.ts --detail --no-dashboard-data
result: pass; executed=1 unsupported=1 unsupported_diagcodes=UnsupportedSyntax:1 unsupported_features=type-system:1
date: 2026-05-07
```

Remaining risks:

- After issue 5288 advances typed modified static field parsing, this file may
  expose generic constraint checking or generic base type reference semantics as
  later blockers.
