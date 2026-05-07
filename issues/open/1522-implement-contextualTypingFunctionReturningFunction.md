---
id: 1522
title: "Implement Contextualtypingfunctionreturningfunction"
type: spike
area: frontend/syntax
class: blocked
priority: P1
depends_on: []
blocks: []
created: 2026-05-01
updated: 2026-05-07
---
> **Reopened by false-done audit** (2026-05-07)
> Classification: must-reopen
> Reason: Batch-migrated to issues/done/ without implementation commits.
> Evidence: Empty completion evidence. No feat/fix commit for #1522.

## Summary

Triage contextualTypingFunctionReturningFunction across 2 failing reference test cases and split this bucket into implementation-ready child issues.

## Problem

Reference test results show 2 cases fail in directory `contextualTypingFunctionReturningFunction` with diagnostics: arrow-function. The compiler cannot handle these syntax/semantics, preventing compilation of code in this category.

Problem: contextualTypingFunctionReturningFunction has 2 reference failures and needs smart-triage evidence before implementation starts.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/contextualTypingFunctionReturningFunction.ts
```

Coverage window:

```sh
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/contextualTypingFunctionReturningFunction.ts --detail
```

## Desired final state

This generated bucket is either split into implementation-ready child issues or superseded by an existing open/done issue with matching evidence. Do not implement directly from this bucket.

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
- [x] At least one child issue contains an exact `mise run reference-triage -- ...` command
- [x] Child issue includes failing path, diagnostic code, source context, visible symbols, and parser/TypeScript AST evidence
- [x] Child issue acceptance names the exact fixture/reference path and diagnostic/stdout change

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run
```

Impacted commands:

```sh
mise run reference-coverage -- tsc --limit 4
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/contextualTypingFunctionReturningFunction.ts --detail
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/contextualTypingFunctionReturningFunction.ts
```

Not run:

- none

## Docs / current-state / issue sync

Final-state docs:

- [x] not affected

Current state:

- [x] not affected

Follow-up issues:

- [x] superseded by `issues/open/5273-parse-nested-zero-argument-arrow-returns.md`

## Notes

## Affected test files

- `reference/typescript/tests/cases/compiler/contextualTypingFunctionReturningFunction.ts`
- `reference/typescript/tests/cases/compiler/contextualTypingFunctionReturningFunction2.ts`

## Duplicate detection

- Superseded by `issues/open/5273-parse-nested-zero-argument-arrow-returns.md`.
  Both affected contextual typing cases stop at nested arrow parsing for
  `() => n => ...`, matching 5273's parser-owned feature family.

## Smart triage

Generated 2026-05-07.

`contextualTypingFunctionReturningFunction.ts`:

```text
command:
env TS2WASM_BINARY=/home/wogikaze/wgkz/ts2wasm/target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/contextualTypingFunctionReturningFunction.ts

result:
UnsupportedSyntax: expected Comma, got Some(Arrow) at 164..166
smart triage feature_label: arrow-function
source context: object literal property `b: () => n => {}`
tokenization: ok
AST: fails at the inner arrow
TypeScript oracle: ok, no diagnostics
TypeScript AST: nested ArrowFunction nodes under PropertyAssignment
```

`contextualTypingFunctionReturningFunction2.ts`:

```text
command:
env TS2WASM_BINARY=/home/wogikaze/wgkz/ts2wasm/target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/contextualTypingFunctionReturningFunction2.ts

result:
UnsupportedSyntax: expected Comma, got Some(Arrow) at 144..146
smart triage feature_label: arrow-function
source context: call argument `f(() => n => n)`
tokenization: ok
AST: fails at the inner arrow
TypeScript oracle: ok, no diagnostics
TypeScript AST: nested ArrowFunction nodes under CallExpression argument
```

Duplicate candidates include
`issues/open/5273-parse-nested-zero-argument-arrow-returns.md`, which now owns
both exact reference paths and acceptance criteria.

## Completion evidence

Fill only when moving to `done/`.

Commits:

- Superseding issue: `issues/open/5273-parse-nested-zero-argument-arrow-returns.md`

Validation result:

```text
command:
env TS2WASM_BINARY=/home/wogikaze/wgkz/ts2wasm/target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/contextualTypingFunctionReturningFunction --detail --no-dashboard-data
result:
pass; executed=2, unsupported=2, both UnsupportedSyntax/unknown-unsupported
date:
2026-05-07

command:
env TS2WASM_BINARY=/home/wogikaze/wgkz/ts2wasm/target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/contextualTypingFunctionReturningFunction.ts
result:
pass; reproduces parser-owned nested arrow failure and points to 5273 duplicate candidate
date:
2026-05-07

command:
env TS2WASM_BINARY=/home/wogikaze/wgkz/ts2wasm/target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/contextualTypingFunctionReturningFunction2.ts
result:
pass; reproduces parser-owned nested arrow failure and points to 5273 duplicate candidate
date:
2026-05-07
```

Remaining risks:

- The parser feature remains open in 5273; this bucket is closed only as
  duplicate/superseded triage cleanup.
