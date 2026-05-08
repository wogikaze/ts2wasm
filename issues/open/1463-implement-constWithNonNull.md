---
id: 1463
title: "Implement Constwithnonnull"
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
> Evidence: Empty completion evidence. No feat/fix commit for #1463.

## Summary

Closed as superseded. Fresh triage shows
`reference/typescript/tests/cases/compiler/constWithNonNull.ts` currently stops
at the ambient `declare const x` name-resolution boundary already owned by
`issues/done/5161-model-ambient-value-declarations-for-name-resolution.md`.

## Problem

The generated bucket was originally labeled parser-syntax. Current triage parses
the non-null update expression shape, but the parser erases `declare const x`
without leaving resolver-visible ambient value metadata. Name resolution then
reports `UnresolvedName` for `x` in `x!++`.

Problem: `constWithNonNull.ts` is blocked by ambient `declare const`
name-resolution before the later non-null increment / const-assignment
diagnostic can be triaged.

## Current failure

Representative reproduction:

```sh
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/constWithNonNull.ts
```

Coverage window:

```sh
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/constWithNonNull.ts --detail --no-dashboard-data
```

## Desired final state

This generated bucket is closed as superseded by
`issues/done/5161-model-ambient-value-declarations-for-name-resolution.md`. Do
not implement directly from this bucket.

## Scope

In scope:

- [x] Inspect fresh smart triage for the affected file.
- [x] Confirm the current first blocker is covered by issue 5161.
- [x] Preserve exact reproduction commands and representative diagnostic/AST
      evidence in this closed issue and the owner issue.

Out of scope:

- Direct implementation from this generated bucket.
- Non-null assertion semantics.
- Increment assignment diagnostics.
- TS2588 "Cannot assign to const" diagnostic parity.

## Affected paths

Expected:

- `crates/frontend/src/parser/statements_ts.rs`
- `crates/frontend/src/ast.rs`
- `crates/ir/src/name_resolver.rs`
- `crates/ir/src/name_resolver_tests.rs`
- focused parser/resolver tests

Do not touch:

- backend/runtime lowering unless ambient resolution advances to runtime emit

## Acceptance criteria

- [x] Existing issue 5161 is confirmed as the current first-blocker owner.
- [x] This closed issue includes failing path, diagnostic code, source context,
      token evidence, compiler AST/resolved evidence, and TypeScript AST
      evidence.
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
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/constWithNonNull.ts --detail --no-dashboard-data
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/constWithNonNull.ts
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

- [x] none; current first blocker is already tracked by issue 5161

## Notes

## Affected test files

- `reference/typescript/tests/cases/compiler/constWithNonNull.ts`

## Duplicate detection

Current first blocker is covered by
`issues/done/5161-model-ambient-value-declarations-for-name-resolution.md`.

Resolution:

```text
Superseded by issue 5161. The active diagnostic is the ambient `declare const`
name-resolution gap at the later `x!++` reference.
```

## Smart triage

### Smart triage: Triage name resolution: constWithNonNull

- Issue class: `triage-needed`
- Feature label: `name-resolution`
- Diagnostic: `UnresolvedName` / `resolver-symbol`
- Path: `reference/typescript/tests/cases/compiler/constWithNonNull.ts`

Reproduction:

```sh
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/constWithNonNull.ts
```

Coverage reproduction:

```sh
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/constWithNonNull.ts --detail --no-dashboard-data
```

Coverage result:

```text
executed=1
build_pass=0
unsupported=1
unsupported_diagcodes=UnresolvedName:1
unsupported_features=name-resolution:1
```

Current diagnostic:

```text
UnresolvedName: unresolved name: `x` at 73..74
```

Source context:

```ts
// Fixes #21848

declare const x: number | undefined;
x!++;
```

Compiler evidence:

- Tokens include `Ident("declare")`, `Const`, `Ident("x")`, type annotation
  tokens, then `Ident("x")`, `Bang`, `Increment`.
- AST construction succeeds but only keeps the expression statement
  `Unary { op: Increment, expr: Ident { name: "x" } }`; the ambient const is
  erased.
- Resolved construction fails in `resolve_names` with `UnresolvedName` for
  `x`.
- Visible symbol extraction reports `x` at line 4, column 9 before the resolver
  failure.

TypeScript oracle evidence:

- TypeScript parses `declare const x: number | undefined;` and `x!++;`.
- TypeScript AST path at the use is
  `ExpressionStatement -> PostfixUnaryExpression -> NonNullExpression -> Identifier`.
- The oracle reports later TS2588:
  `Cannot assign to 'x' because it is a constant.`

## Completion evidence

Commits:

- `...`

Validation result:

```text
command: env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/constWithNonNull.ts
result: pass; current first blocker is the same ambient value name-resolution support tracked by issue 5161
date: 2026-05-07

command: env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/constWithNonNull.ts --detail --no-dashboard-data
result: pass; executed=1 unsupported=1 unsupported_diagcodes=UnresolvedName:1 unsupported_features=name-resolution:1
date: 2026-05-07
```

Remaining risks:

- After issue 5161 advances ambient value name resolution, this file may expose
  non-null expression assignment semantics or TS2588 const assignment
  diagnostic fidelity as later blockers.
