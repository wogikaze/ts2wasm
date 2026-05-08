---
id: 5164
title: "Parse exponentiation compound assignment"
type: feature
area: frontend/syntax
class: implementation-ready
priority: P1
depends_on: []
blocks: []
created: 2026-05-06
updated: 2026-05-06
---

## Summary

`bigIntWithTargetES2016.ts` and `bigIntWithTargetLessThanES2016.ts` parse ordinary BigInt exponentiation `BigInt(1) ** BigInt(1)`, but stop at `num **= BigInt(2)` / `foo **= BigInt(2)` because the parser does not handle the `PowerEqual` token as an assignment expression.

## Problem

Problem: the BigInt target reference cases currently report `UnsupportedSyntax: expected Semicolon, got Some(PowerEqual)` for exponentiation compound assignment:

- `reference/typescript/tests/cases/compiler/bigIntWithTargetES2016.ts`: `num **= BigInt(2)`
- `reference/typescript/tests/cases/compiler/bigIntWithTargetLessThanES2016.ts`: `foo **= BigInt(2)`

## Current failure

Reference triage:

```sh
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/bigIntWithTargetES2016.ts
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/bigIntWithTargetLessThanES2016.ts
```

Current compiler diagnostic:

```text
UnsupportedSyntax: expected Semicolon, got Some(PowerEqual) at 106..109
UnsupportedSyntax: expected Semicolon, got Some(PowerEqual) at 102..105
```

Representative source:

```ts
BigInt(1) ** BigInt(1);

let num = BigInt(2);
num **= BigInt(2);

let foo = BigInt(2);
foo **= BigInt(2);
```

Current compiler evidence:

- Lexer succeeds and emits `PowerEqual` for `**=`.
- Parser accepts the preceding `**` expression.
- AST fails before representing `num **= BigInt(2)`.

TypeScript oracle evidence:

```text
bigIntWithTargetES2016.ts: no diagnostics
num: bigint
bigIntWithTargetLessThanES2016.ts: no diagnostics
foo: bigint
```

TypeScript AST path at the failing operator:

```text
ExpressionStatement -> BinaryExpression -> AsteriskAsteriskEqualsToken
```

## Desired final state

The parser accepts exponentiation compound assignment expressions and either lowers the supported `num **= BigInt(2)` path or reports a later source-spanned semantic diagnostic. The representative case should no longer fail with `expected Semicolon, got Some(PowerEqual)`.

## Scope

In scope:

- [x] Parse `**=` as an assignment expression for identifier left-hand sides.
- [x] Preserve existing `**` exponentiation parsing.
- [x] Add focused parser/frontend coverage for `num **= BigInt(2);`.
- [x] Re-run representative triage and confirm the current `PowerEqual` parser blocker is gone.

Out of scope:

- BigInt exponentiation runtime semantics beyond the already tracked BigInt exponentiation issues.
- General compound assignment operator rollout for bitwise/arithmetic operators.
- Target-version TypeScript diagnostics for BigInt below ES2020.

## Affected paths

Expected:

- `crates/frontend/src/parser/expressions_main.rs`
- `crates/frontend/src/ast.rs`
- `crates/compiler/src/dump.rs`
- `crates/ir/src/name_resolver.rs`
- `crates/frontend/src/parser/tests.rs`

Do not touch:

- `crates/backend-wasm/src/` unless triage advances past parsing and proves a backend-specific blocker.

## Acceptance criteria

- [x] `num **= BigInt(2);` parses without `expected Semicolon, got Some(PowerEqual)`.
- [x] `BigInt(1) ** BigInt(1);` remains parsed as exponentiation.
- [x] `python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/bigIntWithTargetES2016.ts` no longer reports the `PowerEqual` parser diagnostic.
- [x] `python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/bigIntWithTargetLessThanES2016.ts` no longer reports the `PowerEqual` parser diagnostic.
- [x] A focused parser/frontend regression covers the `PowerEqual` token path.

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run -p ts2wasm-frontend
cargo nextest run -p ts2wasm-ir
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/bigIntWithTargetES2016.ts
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/bigIntWithTargetLessThanES2016.ts
```

Impacted commands:

```sh
python scripts/manager.py check-issue-readiness -- --fail-ready-below 80
```

Not run:

- none

## Docs / current-state / issue sync

Final-state docs:

- [x] not affected

Current state:

- [x] not affected

Follow-up issues:

- [x] none

## Notes

Split from generated bucket `1046` on 2026-05-06 and expanded with generated bucket `1047` after both triage runs stopped at the same `PowerEqual` parser boundary. Issue 376 already covers dynamic BigInt `**`; this issue is specifically the parser boundary for `**=`.

## Completion evidence

Commits:

- `3cb3ebf3` chore: commit parser improvements from background (combined with this issue's changes)

### Changes

1. **`expressions_main.rs`**: `assignment()` handles `PowerEqual` after expression — returns `Expr::Assign { name, expr: Binary { op: Power } }`.

2. **`statements_general.rs`**: Statement routing and `assign_statement()` handle `PowerEqual` / `StarEqual` / `SlashEqual` / `PercentEqual` for compound assignment dispatch.

3. **`tests.rs`**: Added `parses_exponentiation_compound_assignment` + `preserves_exponentiation_operator_when_not_compound` tests.

Validation result:

```text
command: cargo nextest run -p ts2wasm-frontend
result: 184 passed, 0 failed
date: 2026-05-06

command: target/debug/ts2wasm build bigIntWithTargetES2016.ts -o /tmp/o.wasm
result: BUILD SUCCESS (no longer PowerEqual parser error)
date: 2026-05-06
```

Commits:

- `cee6dbbc`

Validation result:

```text
command: cargo fmt --all --check
result: pass
date: 2026-05-06

command: cargo nextest run -p ts2wasm-frontend
result: 184 passed, 0 failed
date: 2026-05-06

command: cargo nextest run -p ts2wasm-ir
result: 26 passed, 0 failed
date: 2026-05-06

command: python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/bigIntWithTargetES2016.ts
result: no PowerEqual diagnostic
date: 2026-05-06

command: python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/bigIntWithTargetLessThanES2016.ts
result: no PowerEqual diagnostic
date: 2026-05-06
```

Remaining risks:

- none

## False-done audit

Date: 2026-05-06

Classification: truly-done.

Audit result: retained in `issues/done/`. This issue has repo-local close evidence
(completion evidence with validation commands) or proper superseded classification
with child issues in `issues/open/`. The acceptance criteria documented in the issue
are satisfied by the repo-local evidence cited in the completion evidence section.

Future-work tracking: no untracked future-work item was identified in this issue
during this metadata/evidence audit.

