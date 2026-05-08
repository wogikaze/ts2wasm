---
id: 1531
title: "Implement Contextualtypingtwoinstancesofsametypeparameter"
type: spike
area: frontend/syntax
class: blocked
priority: P1
depends_on: [5385]
blocks: []
created: 2026-05-01
updated: 2026-05-07
---
> **Reopened by false-done audit** (2026-05-07)
> Classification: must-reopen
> Reason: Batch-migrated to issues/done/ without implementation commits.
> Evidence: Empty completion evidence. No feat/fix commit for #1531.

## Summary

Triage contextualTypingTwoInstancesOfSameTypeParameter across 1 failing reference test cases and split this bucket into implementation-ready child issues.

## Problem

Reference test results show 1 cases fail in directory `contextualTypingTwoInstancesOfSameTypeParameter` with diagnostics: parser-syntax. The compiler cannot handle these syntax/semantics, preventing compilation of code in this category.

Problem: contextualTypingTwoInstancesOfSameTypeParameter has 1 reference failures and needs smart-triage evidence before implementation starts.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/contextualTypingTwoInstancesOfSameTypeParameter.ts
```

Coverage window:

```sh
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/contextualTypingTwoInstancesOfSameTypeParameter.ts --detail
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
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/contextualTypingTwoInstancesOfSameTypeParameter.ts --detail
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/contextualTypingTwoInstancesOfSameTypeParameter.ts
```

Not run:

- none

## Docs / current-state / issue sync

Final-state docs:

- [x] not affected

Current state:

- [x] not affected

Follow-up issues:

- [x] created: `issues/open/5385-parse-arrow-body-assignment-expressions.md`

## Notes

## Affected test files

- `reference/typescript/tests/cases/compiler/contextualTypingTwoInstancesOfSameTypeParameter.ts`

## Duplicate detection

- No exact existing owner found.
- `issues/open/5208-support-regexp-match-fallback-array-map-receiver.md` is related
  but owns destructuring assignment bodies such as `() => [i] = [i + 1]`, not
  this plain identifier assignment body `y => x = y`.

## Smart triage

Generated 2026-05-07.

```text
command:
env TS2WASM_BINARY=/home/wogikaze/wgkz/ts2wasm/target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/contextualTypingTwoInstancesOfSameTypeParameter.ts

result:
Feature label: parser-syntax
Diagnostic: UnsupportedSyntax / parser-or-frontend-unsupported
error: expected Comma, got Some(Equal) at 113..114

source context:
function f6<T>(x: (a: T) => T) {
    return null;
}
f6(x => f6(y => x = y));

compiler evidence:
tokens: ok; includes `Ident x`, `Equal`, `Ident y` inside the nested arrow body
ast: fails at the assignment operator
TypeScript AST: ExpressionStatement -> CallExpression -> ArrowFunction -> CallExpression -> ArrowFunction -> BinaryExpression `x = y`
TypeScript oracle: ok, diagnostics []
```

## Completion evidence

Fill only when moving to `done/`.

Commits:

- split to `issues/open/5385-parse-arrow-body-assignment-expressions.md`

Validation result:

```text
command:
env TS2WASM_BINARY=/home/wogikaze/wgkz/ts2wasm/target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/contextualTypingTwoInstancesOfSameTypeParameter.ts --detail --no-dashboard-data
result:
pass; executed=1, unsupported=1, unsupported_diagcodes=UnsupportedSyntax:1, unsupported_features=unknown-unsupported:1
date:
2026-05-07

command:
env TS2WASM_BINARY=/home/wogikaze/wgkz/ts2wasm/target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/contextualTypingTwoInstancesOfSameTypeParameter.ts
result:
pass; reproduced parser failure at nested arrow body assignment `x = y`
date:
2026-05-07
```

Remaining risks:

- The reference path remains unsupported until issue 5385 parses arrow body
  assignment expressions.
