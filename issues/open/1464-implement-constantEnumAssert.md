---
id: 1464
title: "Implement Constantenumassert"
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
> Evidence: Empty completion evidence. No feat/fix commit for #1464.

## Summary

Closed as superseded. Fresh triage shows
`reference/typescript/tests/cases/compiler/constantEnumAssert.ts` currently
stops at the same top-level `const enum E4 { ... }` parser misclassification
already owned by `issues/open/5184-parse-const-enum-declarations.md`.

## Problem

Reference test results previously grouped this file under const enum assertions.
Current compiler behavior parses the preceding ordinary enum declarations far
enough to reach the first `const enum`, then stops before enum-member const
assertions and the later TS1355 diagnostic can be triaged.

Problem: `constantEnumAssert.ts` is blocked by const-enum declaration parsing
before const assertion behavior can be triaged.

## Current failure

Representative reproduction:

```sh
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/constantEnumAssert.ts
```

Coverage window:

```sh
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/constantEnumAssert.ts --detail --no-dashboard-data
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
- Const assertion semantics.
- Enum member literal typing.
- TS1355 diagnostic parity for invalid const assertions.

## Affected paths

Expected:

- `crates/frontend/src/parser/`
- `crates/frontend/src/ast.rs`
- `fixtures/`
- focused parser tests

Do not touch:

- const assertion or enum typechecker logic until const-enum parsing advances

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
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/constantEnumAssert.ts --detail --no-dashboard-data
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/constantEnumAssert.ts
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

- `reference/typescript/tests/cases/compiler/constantEnumAssert.ts`

## Duplicate detection

Current first blocker is covered by
`issues/open/5184-parse-const-enum-declarations.md`.

Resolution:

```text
Superseded by issue 5184. The active diagnostic is the const-enum parser
misclassification at `const enum E4`.
```

## Smart triage

### Smart triage: Triage enum: constantEnumAssert

- Issue class: `triage-needed`
- Feature label: `enum`
- Diagnostic: `UnsupportedSyntax` / `parser-or-frontend-unsupported`
- Path: `reference/typescript/tests/cases/compiler/constantEnumAssert.ts`

Reproduction:

```sh
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/constantEnumAssert.ts
```

Coverage reproduction:

```sh
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/constantEnumAssert.ts --detail --no-dashboard-data
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
UnsupportedSyntax: const declarations require an initializer at 145..149
```

Source context:

```ts
enum E3 {
    a = 1,
    b = a << 1,
    c = a << 2,
}

const enum E4 {
    a,
    b
}
```

Compiler evidence:

- Tokenization succeeds through ordinary `enum E1`, `enum E2`, `enum E3`, then
  `Const`, `Ident("enum")`, `Ident("E4")`.
- AST and resolved construction both fail before representing the const enum
  declaration.
- No visible symbols are available before the parser failure.

TypeScript oracle evidence:

- TypeScript parses ordinary enum declarations `E1`, `E2`, `E3` and
  `EnumDeclaration "const enum E4 { ... }"`.
- TypeScript then parses const/object declarations using `as const` with enum
  member references.
- The oracle reports later TS1355 for invalid `E5.a as const`; that later
  const-assertion diagnostic is not reached before this parser boundary.

## Completion evidence

Commits:

- `...`

Validation result:

```text
command: env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/constantEnumAssert.ts
result: pass; current first blocker is the same const-enum parser support tracked by issue 5184
date: 2026-05-07

command: env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/constantEnumAssert.ts --detail --no-dashboard-data
result: pass; executed=1 unsupported=1 unsupported_diagcodes=UnsupportedSyntax:1 unsupported_features=enum:1
date: 2026-05-07
```

Remaining risks:

- After issue 5184 advances const-enum parsing, this file may expose const
  assertion semantics, enum member literal typing, or TS1355 diagnostic fidelity
  as later blockers.
