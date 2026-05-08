---
id: 1525
title: "Implement Contextualtypingofconditionalexpression"
type: spike
area: frontend/semantics
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
> Evidence: Empty completion evidence. No feat/fix commit for #1525.

## Summary

Triage contextualTypingOfConditionalExpression across 2 failing reference test cases and split this bucket into implementation-ready child issues.

## Problem

Reference test results show 2 cases fail in directory `contextualTypingOfConditionalExpression` with diagnostics: type-system. The compiler cannot handle these syntax/semantics, preventing compilation of code in this category.

Problem: contextualTypingOfConditionalExpression has 2 reference failures and needs smart-triage evidence before implementation starts.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/contextualTypingOfConditionalExpression2.ts
```

Coverage window:

```sh
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/contextualTypingOfConditionalExpression2.ts --detail
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
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/contextualTypingOfConditionalExpression2.ts --detail
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/contextualTypingOfConditionalExpression2.ts
```

Not run:

- none

## Docs / current-state / issue sync

Final-state docs:

- [x] not affected

Current state:

- [x] not affected

Follow-up issues:

- [x] added: `issues/open/5381-parse-arrow-functions-in-ternary-branches.md`
- [x] added: `issues/open/5382-parse-typed-arrow-ternary-branches.md`

## Notes

## Affected test files

- `reference/typescript/tests/cases/compiler/contextualTypingOfConditionalExpression2.ts`
- `reference/typescript/tests/cases/compiler/contextualTypingOfConditionalExpression.ts`

## Duplicate detection

- No exact existing owner found. `issues/open/5160-lower-plain-ternary-conditional-expressions.md`
  is related but starts after parser success; this bucket currently stops
  during AST construction on arrow-function ternary branches.

## Smart triage

Generated 2026-05-07.

`contextualTypingOfConditionalExpression.ts`:

```text
command:
env TS2WASM_BINARY=/home/wogikaze/wgkz/ts2wasm/target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/contextualTypingOfConditionalExpression.ts

result:
UnsupportedSyntax: expected Semicolon, got Some(Arrow) at 106..108
source context: true ? (a) => a.toExponential() : (b) => b.toFixed()
tokens: ok
AST: fails on the alternate branch arrow
TypeScript AST: ConditionalExpression with ArrowFunction branch nodes
```

`contextualTypingOfConditionalExpression2.ts`:

```text
command:
env TS2WASM_BINARY=/home/wogikaze/wgkz/ts2wasm/target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/contextualTypingOfConditionalExpression2.ts

result:
UnsupportedSyntax: expected RightParen, got Some(Colon) at 209..210
source context: true ? (a: C) => a.foo : (b: number) => { }
tokens: ok
AST: fails on the colon inside the typed alternate branch parameter
TypeScript AST: ConditionalExpression with ArrowFunction branch nodes
```

The generated bucket was split to
`issues/open/5381-parse-arrow-functions-in-ternary-branches.md` for the
untyped branch case and
`issues/open/5382-parse-typed-arrow-ternary-branches.md` for the typed branch
case.

## Completion evidence

Fill only when moving to `done/`.

Commits:

- child issues:
  `issues/open/5381-parse-arrow-functions-in-ternary-branches.md`,
  `issues/open/5382-parse-typed-arrow-ternary-branches.md`

Validation result:

```text
command:
env TS2WASM_BINARY=/home/wogikaze/wgkz/ts2wasm/target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/contextualTypingOfConditionalExpression --detail --no-dashboard-data
result:
pass; executed=2, unsupported=2, UnsupportedSyntax/type-system
date:
2026-05-07

command:
env TS2WASM_BINARY=/home/wogikaze/wgkz/ts2wasm/target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/contextualTypingOfConditionalExpression.ts
result:
pass; reproduced parser failure at untyped alternate arrow branch
date:
2026-05-07

command:
env TS2WASM_BINARY=/home/wogikaze/wgkz/ts2wasm/target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/contextualTypingOfConditionalExpression2.ts
result:
pass; reproduced parser failure at typed alternate arrow branch
date:
2026-05-07
```

Remaining risks:

- Implementation remains open in 5381 and 5382. After parsing succeeds, 5160
  or later contextual function assignability diagnostics may become visible.
