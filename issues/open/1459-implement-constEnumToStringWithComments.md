---
id: 1459
title: "Implement Constenumtostringwithcomments"
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
> Evidence: Empty completion evidence. No feat/fix commit for #1459.

## Summary

Closed as superseded. Fresh triage shows
`reference/typescript/tests/cases/compiler/constEnumToStringWithComments.ts`
currently stops at the same top-level `const enum Foo { ... }` parser
misclassification already owned by
`issues/open/5184-parse-const-enum-declarations.md`.

## Problem

Reference test results previously grouped this file under a const-enum
`toString` bucket with `@removeComments: false`. Current compiler behavior does
not reach enum member constant values, property/string-index access,
`.toString()` calls, comment preservation, or const-enum inlining because
parsing stops at the first `const enum Foo` declaration.

Problem: `constEnumToStringWithComments.ts` is blocked by const-enum
declaration parsing before toString/with-comments behavior can be triaged.

## Current failure

Representative reproduction:

```sh
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/constEnumToStringWithComments.ts
```

Coverage window:

```sh
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/constEnumToStringWithComments.ts --detail --no-dashboard-data
```

## Desired final state

This generated bucket is closed as superseded by
`issues/open/5184-parse-const-enum-declarations.md`. Do not implement directly
from this bucket.

## Scope

In scope:

- [x] Inspect fresh smart triage for the affected file.
- [x] Confirm the current first blocker is covered by issue 5184.
- [x] Preserve exact reproduction commands and representative diagnostic/AST
      evidence in this closed issue and the owner issue.

Out of scope:

- Direct implementation from this generated bucket.
- Enum member constant evaluation.
- Const-enum inlining.
- `.toString()` behavior for enum member references.
- Comment preservation or emit behavior.

## Affected paths

Expected:

- `crates/frontend/src/parser/`
- `crates/frontend/src/ast.rs`
- `fixtures/`
- focused parser tests

Do not touch:

- unrelated runtime/backend code unless focused triage proves a backend-only
  blocker after parser support lands

## Acceptance criteria

- [x] Existing issue 5184 is confirmed as the current first-blocker owner.
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
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/constEnumToStringWithComments.ts --detail --no-dashboard-data
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/constEnumToStringWithComments.ts
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

- [x] none; current first blocker is already tracked by issue 5184

## Notes

## Affected test files

- `reference/typescript/tests/cases/compiler/constEnumToStringWithComments.ts`

## Duplicate detection

Current first blocker is covered by
`issues/open/5184-parse-const-enum-declarations.md`.

Resolution:

```text
Superseded by issue 5184. The active diagnostic is the const-enum parser
misclassification at the first top-level const enum declaration.
```

## Smart triage

### Smart triage: Triage enum: constEnumToStringWithComments

- Issue class: `triage-needed`
- Feature label: `enum`
- Diagnostic: `UnsupportedSyntax` / `parser-or-frontend-unsupported`
- Path: `reference/typescript/tests/cases/compiler/constEnumToStringWithComments.ts`

Reproduction:

```sh
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/constEnumToStringWithComments.ts
```

Coverage reproduction:

```sh
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/constEnumToStringWithComments.ts --detail --no-dashboard-data
```

Coverage result:

```text
executed=1
build_pass=0
unsupported=1
unsupported_diagcodes=UnsupportedSyntax:1
unsupported_features=enum:1
```

Current diagnostic:

```text
UnsupportedSyntax: const declarations require an initializer at 53..57
```

Source context:

```ts
// @target: es2015
// @removeComments: false
const enum Foo {
    X = 100,
    Y = 0.5,
    Z = 2.,
```

Compiler evidence:

- Tokenization succeeds and starts with `Const`, `Ident("enum")`,
  `Ident("Foo")`, `{`, then numeric enum members.
- AST and resolved construction both fail before representing the enum
  declaration.
- Visible symbol extraction reports a bogus binding named `en` at line 3,
  column 1 before the parser failure.

TypeScript oracle evidence:

- TypeScript parses the first top-level declaration as
  `EnumDeclaration "const enum Foo { ... }"`.
- The oracle reports no TypeScript diagnostics for the file.
- TypeScript also sees later statements such as
  `let x0 = Foo.X.toString();` and `let x1 = Foo["X"].toString();`, which are
  not reached by the current compiler.

## Completion evidence

Commits:

- `...`

Validation result:

```text
command: env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/constEnumToStringWithComments.ts
result: pass; current first blocker is the same const-enum parser support tracked by issue 5184
date: 2026-05-07

command: env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/constEnumToStringWithComments.ts --detail --no-dashboard-data
result: pass; executed=1 unsupported=1 unsupported_diagcodes=UnsupportedSyntax:1 unsupported_features=enum:1
date: 2026-05-07
```

Remaining risks:

- After issue 5184 advances const-enum parsing, this file may expose enum member
  constant evaluation, const-enum inlining, property/string-index enum access,
  `.toString()` behavior, or comment-preservation emit work as later blockers.
