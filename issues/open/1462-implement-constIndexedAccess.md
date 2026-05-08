---
id: 1462
title: "Implement Constindexedaccess"
type: spike
area: frontend/syntax
class: done
priority: P2
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
> Evidence: Empty completion evidence. No feat/fix commit for #1462.

## Summary

Closed as superseded. Fresh triage shows
`reference/typescript/tests/cases/compiler/constIndexedAccess.ts` currently
stops at the same top-level `const enum numbers { ... }` parser
misclassification already owned by
`issues/open/5184-parse-const-enum-declarations.md`.

## Problem

Reference test results previously grouped this file under indexed access. The
current compiler does not reach indexed access typing, enum member lookup, or
the later non-const enum comparison because parsing stops at the first
`const enum numbers` declaration.

Problem: `constIndexedAccess.ts` is blocked by const-enum declaration parsing
before indexed access behavior can be triaged.

## Current failure

Representative reproduction:

```sh
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/constIndexedAccess.ts
```

Coverage window:

```sh
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/constIndexedAccess.ts --detail --no-dashboard-data
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
- Indexed access typing.
- Const-enum member lookup and inlining.
- Ordinary enum indexed access behavior.
- Definite assignment diagnostics for `test`.

## Affected paths

Expected:

- `crates/frontend/src/parser/`
- `crates/frontend/src/ast.rs`
- `fixtures/`
- focused parser tests

Do not touch:

- indexed access typechecker/runtime code until const-enum parsing advances

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
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/constIndexedAccess.ts --detail --no-dashboard-data
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/constIndexedAccess.ts
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

- `reference/typescript/tests/cases/compiler/constIndexedAccess.ts`

## Duplicate detection

Current first blocker is covered by
`issues/open/5184-parse-const-enum-declarations.md`.

Resolution:

```text
Superseded by issue 5184. The active diagnostic is the const-enum parser
misclassification at the first top-level const enum declaration.
```

## Smart triage

### Smart triage: Triage unknown unsupported: constIndexedAccess

- Issue class: `triage-needed`
- Feature label: `unknown-unsupported`
- Diagnostic: `UnsupportedSyntax` / `parser-or-frontend-unsupported`
- Path: `reference/typescript/tests/cases/compiler/constIndexedAccess.ts`

Reproduction:

```sh
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/constIndexedAccess.ts
```

Coverage reproduction:

```sh
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/constIndexedAccess.ts --detail --no-dashboard-data
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
UnsupportedSyntax: const declarations require an initializer at 31..35
```

Source context:

```ts
// @target: es2015

const enum numbers {
    zero,
    one
}
```

Compiler evidence:

- Tokenization succeeds and starts with `Const`, `Ident("enum")`,
  `Ident("numbers")`, `{`, members `zero` and `one`, then `}`.
- AST and resolved construction both fail before representing the enum
  declaration.
- Visible symbol extraction reports a bogus binding named `enum` at line 3,
  column 1 before the parser failure.

TypeScript oracle evidence:

- TypeScript parses the first top-level declaration as
  `EnumDeclaration "const enum numbers { ... }"`.
- TypeScript then parses `interface indexAccess`, indexed accesses such as
  `test[numbers.zero]` and `test[numbers["zero"]]`, and the later
  `enum numbersNotConst`.
- The oracle reports later TS2454 definite-assignment diagnostics for `test`;
  those later semantics are not reached before this parser boundary.

## Completion evidence

Commits:

- `...`

Validation result:

```text
command: env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/constIndexedAccess.ts
result: pass; current first blocker is the same const-enum parser support tracked by issue 5184
date: 2026-05-07

command: env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/constIndexedAccess.ts --detail --no-dashboard-data
result: pass; executed=1 unsupported=1 unsupported_diagcodes=UnsupportedSyntax:1 unsupported_features=unknown-unsupported:1
date: 2026-05-07
```

Remaining risks:

- After issue 5184 advances const-enum parsing, this file may expose indexed
  access typing, const-enum member lookup/inlining, ordinary enum indexed
  access, or definite-assignment diagnostic fidelity as later blockers.
