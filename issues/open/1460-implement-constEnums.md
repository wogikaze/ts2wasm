---
id: 1460
title: "Implement Constenums"
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
> Reason: Batch-migrated to issues/open/ without implementation commits.
> Evidence: Empty completion evidence. No feat/fix commit for #1460.

## Summary

Closed as superseded. Fresh triage shows
`reference/typescript/tests/cases/compiler/constEnums.ts` currently stops at the
same top-level `const enum Enum1 { ... }` parser misclassification already
owned by `issues/open/5184-parse-const-enum-declarations.md`.

## Problem

Reference test results previously grouped this file under a broad const-enums
bucket. Current compiler behavior does not reach enum member constant
evaluation, duplicate enum declaration merging, nested namespace const enums,
import-alias use, switch exhaustiveness, or comment-like enum member names
because parsing stops at the first `const enum Enum1` declaration.

Problem: `constEnums.ts` is blocked by const-enum declaration parsing before the
broader const-enum behavior can be triaged.

## Current failure

Representative reproduction:

```sh
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/constEnums.ts
```

Coverage window:

```sh
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/constEnums.ts --detail --no-dashboard-data
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
- Const-enum declaration merging.
- Enum member constant evaluation.
- Namespace const-enum export/import alias behavior.
- Switch exhaustiveness or later TypeScript diagnostics.
- Comment-like enum member names.

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
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/constEnums.ts --detail --no-dashboard-data
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/constEnums.ts
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

- `reference/typescript/tests/cases/compiler/constEnums.ts`

## Duplicate detection

Current first blocker is covered by
`issues/open/5184-parse-const-enum-declarations.md`.

Resolution:

```text
Superseded by issue 5184. The active diagnostic is the const-enum parser
misclassification at the first top-level const enum declaration.
```

## Smart triage

### Smart triage: Triage enum: constEnums

- Issue class: `triage-needed`
- Feature label: `enum`
- Diagnostic: `UnsupportedSyntax` / `parser-or-frontend-unsupported`
- Path: `reference/typescript/tests/cases/compiler/constEnums.ts`

Reproduction:

```sh
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/constEnums.ts
```

Coverage reproduction:

```sh
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/constEnums.ts --detail --no-dashboard-data
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
UnsupportedSyntax: const declarations require an initializer at 26..30
```

Source context:

```ts
// @target: es2015
const enum Enum1 {
   A0 = 100,
}
```

Compiler evidence:

- Tokenization succeeds and starts with `Const`, `Ident("enum")`,
  `Ident("Enum1")`, `{`, member `A0`, `=`, `Number(100)`, and `}`.
- AST and resolved construction both fail before representing the first enum
  declaration.
- Visible symbol extraction reports a bogus binding named `e` at line 2,
  column 1 before the parser failure.

TypeScript oracle evidence:

- TypeScript parses the first top-level declaration as
  `EnumDeclaration "const enum Enum1 { A0 = 100 }"`.
- TypeScript also parses later `EnumDeclaration` and `ModuleDeclaration`
  constructs for duplicate const enums, nested namespaces, import aliases, and
  switch tests.
- The oracle reports later TS2366 for a function missing a return; that later
  semantic diagnostic is not reached before the current parser boundary.

## Completion evidence

Commits:

- `...`

Validation result:

```text
command: env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/constEnums.ts
result: pass; current first blocker is the same const-enum parser support tracked by issue 5184
date: 2026-05-07

command: env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/constEnums.ts --detail --no-dashboard-data
result: pass; executed=1 unsupported=1 unsupported_diagcodes=UnsupportedSyntax:1 unsupported_features=enum:1
date: 2026-05-07
```

Remaining risks:

- After issue 5184 advances const-enum parsing, this file may expose enum member
  constant evaluation, const-enum declaration merging, namespace export/import
  alias handling, switch exhaustiveness, comment-like enum member names, or
  later TypeScript diagnostic fidelity as separate blockers.
