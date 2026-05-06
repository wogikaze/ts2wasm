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

- [ ] Parse `**=` as an assignment expression for identifier left-hand sides.
- [ ] Preserve existing `**` exponentiation parsing.
- [ ] Add focused parser/frontend coverage for `num **= BigInt(2);`.
- [ ] Re-run representative triage and confirm the current `PowerEqual` parser blocker is gone.

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

- [ ] `num **= BigInt(2);` parses without `expected Semicolon, got Some(PowerEqual)`.
- [ ] `BigInt(1) ** BigInt(1);` remains parsed as exponentiation.
- [ ] `python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/bigIntWithTargetES2016.ts` no longer reports the `PowerEqual` parser diagnostic.
- [ ] `python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/bigIntWithTargetLessThanES2016.ts` no longer reports the `PowerEqual` parser diagnostic.
- [ ] A focused parser/frontend regression covers the `PowerEqual` token path.

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

- [ ] not affected

Current state:

- [ ] not affected

Follow-up issues:

- [ ] none

## Notes

Split from generated bucket `1046` on 2026-05-06 and expanded with generated bucket `1047` after both triage runs stopped at the same `PowerEqual` parser boundary. Issue 376 already covers dynamic BigInt `**`; this issue is specifically the parser boundary for `**=`.

## Completion evidence

Fill only when moving to `done/`.

Commits:

- `<hash>` (set during commit)

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
