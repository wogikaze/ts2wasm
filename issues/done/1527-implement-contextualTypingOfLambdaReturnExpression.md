---
id: 1527
title: "Implement Contextualtypingoflambdareturnexpression"
type: spike
area: frontend/syntax
class: triage-needed
priority: P1
depends_on: []
blocks: []
created: 2026-05-01
updated: 2026-05-07
---

## Summary

Triage contextualTypingOfLambdaReturnExpression across 1 failing reference test cases and split this bucket into implementation-ready child issues.

## Problem

Reference test results show 1 cases fail in directory `contextualTypingOfLambdaReturnExpression` with diagnostics: parser-syntax. The compiler cannot handle these syntax/semantics, preventing compilation of code in this category.

Problem: contextualTypingOfLambdaReturnExpression has 1 reference failures and needs smart-triage evidence before implementation starts.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/contextualTypingOfLambdaReturnExpression.ts
```

Coverage window:

```sh
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/contextualTypingOfLambdaReturnExpression.ts --detail
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
mise run reference-coverage -- tsc --limit 2
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/contextualTypingOfLambdaReturnExpression.ts --detail
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/contextualTypingOfLambdaReturnExpression.ts
```

Not run:

- none

## Docs / current-state / issue sync

Final-state docs:

- [x] not affected

Current state:

- [x] not affected

Follow-up issues:

- [x] superseded by `issues/open/5200-validate-top-level-function-overload-implementations.md`

## Notes

## Affected test files

- `reference/typescript/tests/cases/compiler/contextualTypingOfLambdaReturnExpression.ts`

## Duplicate detection

- Superseded by `issues/open/5200-validate-top-level-function-overload-implementations.md`.
  The current blocker is top-level function overload implementation grouping:
  bodyless overload signatures for `callb` are treated as duplicate local
  bindings before contextual lambda diagnostics can run.

## Smart triage

Generated 2026-05-07.

```text
command:
env TS2WASM_BINARY=/home/wogikaze/wgkz/ts2wasm/target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/contextualTypingOfLambdaReturnExpression.ts

result:
DuplicateLocal: duplicate local variable: `callb` at 82..90
feature_label: duplicate-local
source context:
function callb(lam: (l: number) => void);
function callb(lam: (n: string) => void);
function callb(a) { }
tokens: ok
AST: ok; three top-level Function statements named `callb`
resolved: DuplicateLocal at the second bodyless overload signature
TypeScript oracle: TS2339 at `a.length` in the contextual lambda calls
```

The generated parser-syntax bucket is stale. Current ownership is the
top-level function overload grouping issue 5200.

## Completion evidence

Fill only when moving to `done/`.

Commits:

- superseding issue: `issues/open/5200-validate-top-level-function-overload-implementations.md`

Validation result:

```text
command:
env TS2WASM_BINARY=/home/wogikaze/wgkz/ts2wasm/target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/contextualTypingOfLambdaReturnExpression.ts --detail --no-dashboard-data
result:
pass; executed=1, unsupported=1, DuplicateLocal/duplicate-local
date:
2026-05-07

command:
env TS2WASM_BINARY=/home/wogikaze/wgkz/ts2wasm/target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/contextualTypingOfLambdaReturnExpression.ts
result:
pass; reproduced DuplicateLocal at bodyless top-level overload signature and folded into 5200
date:
2026-05-07
```

Remaining risks:

- Implementation remains open in 5200. After overload grouping advances, the
  intended contextual lambda diagnostics may become visible.
