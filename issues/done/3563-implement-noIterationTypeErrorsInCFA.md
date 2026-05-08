---
id: 3563
title: "Implement Noiterationtypeerrorsincfa"
type: spike
area: frontend/syntax
class: done
priority: P1
depends_on: [5169]
blocks: []
created: 2026-05-01
updated: 2026-05-08
---

## Summary

Triage noIterationTypeErrorsInCFA across 1 failing reference test cases and split this bucket into implementation-ready child issues.

## Problem

Fresh triage shows this bucket is not blocked by import/export or iteration
typing yet. The current parser stops earlier at the semicolonless assignment
expression statement `dds = [dds]` before the closing `}` of the `if` block.
That exact expression-statement ASI boundary is tracked by issue 5169.

Problem: `noIterationTypeErrorsInCFA.ts` is superseded by issue 5169 until
expression-statement ASI before a closing block is accepted.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/noIterationTypeErrorsInCFA.ts
```

Coverage window:

```sh
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/noIterationTypeErrorsInCFA.ts --detail
```

## Desired final state

This generated bucket is either split into implementation-ready child issues or superseded by an existing open/done issue with matching evidence. Do not implement directly from this bucket.

## Scope

In scope:

- [x] Inspect the smart triage report below
- [x] Confirm whether existing open/done issues already cover this bucket
- [x] Supersede this bucket with issue 5169
- [x] Preserve exact reproduction commands and representative AST/diagnostic evidence

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
- [x] Issue 5169 contains the exact parser boundary family
- [x] This issue includes failing path, diagnostic code, source context, visible symbols, and parser/TypeScript AST evidence
- [x] Completion evidence names the diagnostic/stdout change required after issue 5169

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run
```

Impacted commands:

```sh
mise run reference-coverage -- tsc --limit 2
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/noIterationTypeErrorsInCFA.ts --detail
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/noIterationTypeErrorsInCFA.ts
```

Not run:

- broad Rust gates; no source implementation changed

## Docs / current-state / issue sync

Final-state docs:

- [x] not affected

Current state:

- [x] not affected

Follow-up issues:

- [x] superseded by `issues/open/5169-parse-asi-after-expression-statement.md`

## Notes

## Affected test files

- `reference/typescript/tests/cases/compiler/noIterationTypeErrorsInCFA.ts`

## Duplicate detection

- none found by path/title/feature scan

## Smart triage

Fresh triage on 2026-05-08:

- Diagnostic: `UnsupportedSyntax` / `parser-or-frontend-unsupported`
- Message: `expected Semicolon, got Some(RightBrace) at 172..173`
- Source context: `dds = [dds]` followed by `}` before the `for (let n of dds)`
  loop.
- Visible symbols before failure include exported function `doRemove`.
- Tokens succeed through the interface, exported function, typed parameter,
  `if`, array assignment, and following `RightBrace`. AST construction stops
  before reaching the `for-of` loop.
- TypeScript oracle reports no diagnostics and binds loop variable `n: F`, so
  iteration CFA remains unproven until issue 5169 advances past the ASI
  parser boundary.

## Completion evidence

Superseded by issue 5169.

Commits:

- `...`

Validation result:

```text
command: env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/noIterationTypeErrorsInCFA.ts --detail --no-dashboard-data
result: pass; executed=1, build_pass=0, unsupported=1, UnsupportedSyntax at expression-statement ASI before RightBrace
date: 2026-05-08

command: env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/noIterationTypeErrorsInCFA.ts
result: pass; current blocker is `expected Semicolon, got Some(RightBrace)` after `dds = [dds]`, superseded by issue 5169
date: 2026-05-08
```

Remaining risks:

- After issue 5169 accepts expression-statement ASI before `}`, this path may
  expose the intended `for-of` / CFA behavior.
