---
id: 3536
title: "Implement Noimplicitanyforin"
type: spike
area: frontend/syntax
class: superseded
priority: P1
depends_on: [5474]
blocks: []
created: 2026-05-01
updated: 2026-05-08
---

## Summary

Closed as a generated bucket. Fresh evidence splits the current
element-access `for-in` assignment-head parser gap to issue 5474.

## Problem

Fresh triage shows `noImplicitAnyForIn.ts` now reaches the final loop:

```ts
for (n[idx++] in m);
```

The parser rejects the `[` in the element-access left-hand side with
`UnsupportedSyntax: expected Equal, got Some(LeftBracket)`. TypeScript parses
the same source as `ForInStatement -> ElementAccessExpression n[idx++]` and
then reports TS2405 for the invalid left-hand-side type.

Problem: this generated bucket is superseded by issue 5474 for the remaining
element-access `for-in` assignment-head parser gap.

## Current failure

Representative reproduction:

```sh
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/noImplicitAnyForIn.ts
```

Coverage window:

```sh
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/noImplicitAnyForIn.ts --detail --no-dashboard-data
```

Observed 2026-05-08:

```text
executed=1 build_pass=0 unsupported=1 unsupported_diagcodes=UnsupportedSyntax:1 unsupported_features=unknown-unsupported:1
UnsupportedSyntax: expected Equal, got Some(LeftBracket) at 578..579
```

## Desired final state

This generated bucket is closed as superseded by
`issues/open/5474-parse-element-access-for-in-assignment-heads.md`. Do not
implement directly from this bucket.

## Scope

In scope:

- [x] Inspect the smart triage report below
- [x] Confirm whether existing open/done issues already cover this bucket
- [x] Split one feature family, one observable behavior, or one fixed reference window into child issues
- [x] Preserve exact reproduction commands and representative AST/diagnostic evidence in each child issue

Out of scope:

- Direct implementation from this generated bucket
- Broad multi-feature fixes without child issue split

## Affected paths

Expected:

- `crates/frontend/src/`
- `crates/cli/src/`
- `fixtures/`
- `scripts/run/reference-triage.py`

Do not touch:

- unrelated runtime/backend code unless `reference-triage` proves the failure is not frontend-owned

## Acceptance criteria

- [x] Duplicate candidates below are confirmed as no-match or this issue is superseded
- [x] Child issue contains an exact `python scripts/manager.py reference-triage ...` command
- [x] Child issue includes failing path, diagnostic code, source context, visible symbols, and parser/TypeScript AST evidence
- [x] Child issue acceptance names the exact fixture/reference path and diagnostic/stdout change

## Validation

Required commands:

```sh
git diff --check
python scripts/manager.py update-issue-index --check
python scripts/manager.py check-issue-health
python scripts/manager.py check-issue-readiness -- --fail-ready-below 80
```

Impacted commands:

```sh
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/noImplicitAnyForIn.ts --detail --no-dashboard-data
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/noImplicitAnyForIn.ts
```

Not run:

- `cargo fmt --all --check`; metadata-only issue split.
- `cargo nextest run`; metadata-only issue split.

## Docs / current-state / issue sync

Final-state docs:

- [x] not affected

Current state:

- [x] not affected

Follow-up issues:

- [x] created: `issues/open/5474-parse-element-access-for-in-assignment-heads.md`

## Notes

## Affected test files

- `reference/typescript/tests/cases/compiler/noImplicitAnyForIn.ts`

## Duplicate detection

- `issues/open/442-implement-parser-syntax.md` is a broad generated test262
  parser-syntax bucket, not an implementation-ready owner for the TypeScript
  `n[idx++]` for-in head.
- `issues/open/438-implement-negative-parse-syntaxerror.md` covers negative
  parse SyntaxError triage, not this valid TypeScript for-in head.
- Split to `issues/open/5474-parse-element-access-for-in-assignment-heads.md`.

## Smart triage

### Smart triage: Triage parser syntax: noImplicitAnyForIn

- Issue class: `triage-needed`
- Feature label: `parser-syntax`
- Diagnostic: `UnsupportedSyntax` / `parser-or-frontend-unsupported`
- Path: `reference/typescript/tests/cases/compiler/noImplicitAnyForIn.ts`

Current compiler message:

```text
expected Equal, got Some(LeftBracket) at 578..579
```

Source context:

```text
31 | var n = [[]] || [];
32 |
33 | for (n[idx++] in m);
```

Compiler evidence:

```text
tokens: ok through earlier declaration-form for-in loops
ast/resolved: fail before AST construction at the `[` in `n[idx++]`
visible symbols include x, i, j, _j, k, k1, k2, a, b, c, idx, m, n
```

TypeScript oracle:

```text
AST path: ForInStatement -> ElementAccessExpression `n[idx++]` -> PostfixUnaryExpression `idx++`
TS2405: The left-hand side of a 'for...in' statement must be of type 'string' or 'any'.
```

## Completion evidence

Fill only when moving to `done/`.

Commits:

- `...`

Validation result:

```text
command:
result:
date:
```

Remaining risks:

- none
