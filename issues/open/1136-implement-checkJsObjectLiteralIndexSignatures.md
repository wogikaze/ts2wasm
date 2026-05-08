---
id: 1136
title: "Implement Checkjsobjectliteralindexsignatures"
type: spike
area: frontend/syntax
class: done
priority: P1
depends_on: [5228]
blocks: []
created: 2026-05-01
updated: 2026-05-06
completed: 2026-05-06
---
> **Reopened by false-done audit** (2026-05-07)
> Classification: must-reopen
> Reason: Batch-migrated to issues/done/ without implementation commits.
> Evidence: Empty completion evidence. No feat/fix commit for #1136.

## Summary

Triage checkJsObjectLiteralIndexSignatures across 1 failing reference test cases and split this bucket into implementation-ready child issues.

## Problem

Reference test results show 1 cases fail in directory `checkJsObjectLiteralIndexSignatures` with diagnostics: object-literal. The compiler cannot handle these syntax/semantics, preventing compilation of code in this category.

Problem: checkJsObjectLiteralIndexSignatures has 1 reference failures and needs smart-triage evidence before implementation starts.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/checkJsObjectLiteralIndexSignatures.ts
```

Coverage window:

```sh
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/checkJsObjectLiteralIndexSignatures.ts --detail
```

## Desired final state

This generated bucket is either split into implementation-ready child issues or superseded by an existing open/done issue with matching evidence. Do not implement directly from this bucket.

## Scope

In scope:

- [x] Inspect the smart triage report below
- [x] Confirm whether existing open/done issues already cover this bucket
- [x] Split this bucket to a focused computed object literal property issue
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
- [x] Child issue contains an exact `reference-triage` command
- [x] This issue includes failing path, diagnostic code, source context, visible symbols, and parser/TypeScript AST evidence
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
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/checkJsObjectLiteralIndexSignatures.ts --detail
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/checkJsObjectLiteralIndexSignatures.ts
```

Not run:

- none

## Docs / current-state / issue sync

Final-state docs:

- [x] not affected

Current state:

- [x] not affected

Follow-up issues:

- [x] created: `issues/open/5228-w0-wasm-binary-backend-mvp.md`

## Notes

## Affected test files

- `reference/typescript/tests/cases/compiler/checkJsObjectLiteralIndexSignatures.ts`

## Duplicate detection

Fresh duplicate scan found broad object-literal generated buckets and related
computed object literal parser issues. Issue 5209 covers binary computed keys,
and issue 5223 covers computed properties after object spread. This
representative is a smaller simple-identifier key slice.

Split to:

- `issues/open/5228-w0-wasm-binary-backend-mvp.md`

## Smart triage

### Smart triage: checkJsObjectLiteralIndexSignatures

- Issue class: `triage-needed`
- Feature label: `object-literal`
- Diagnostic: `UnsupportedSyntax` / `parser-or-frontend-unsupported`
- Path: `reference/typescript/tests/cases/compiler/checkJsObjectLiteralIndexSignatures.ts`

Reproduction:

```sh
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/checkJsObjectLiteralIndexSignatures.ts
```

Coverage:

```sh
python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/checkJsObjectLiteralIndexSignatures.ts --detail --no-dashboard-data
```

Coverage result:

```text
unsupported=1
unsupported_diagcodes=UnsupportedSyntax:1
unsupported_features=object-literal:1
```

Source context:

```ts
let n = Math.random();
let s = `${n}`;

const numericIndex = { [n]: 1 };
numericIndex[n].toFixed();

const stringIndex = { [s]: 1 };
stringIndex[s].toFixed();
```

Compiler evidence:

```text
tokens: ok; object literal computed key tokens are present
ast: fails with expected Dot, got Some(RightBracket) at 190..191
resolved/lowered: same parser failure
TypeScript oracle: no diagnostics; hints infer { [x: number]: number } and { [x: string]: number }
```

## Completion evidence

Fill only when moving to `done/`.

Commits:

- filled by closing commit

Validation result:

```text
command: python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/checkJsObjectLiteralIndexSignatures.ts --detail --no-dashboard-data
result: pass; reproduced UnsupportedSyntax/object-literal blocker
date: 2026-05-06

command: python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/checkJsObjectLiteralIndexSignatures.ts
result: pass; reproduced expected Dot / RightBracket parser blocker and split to issue 5228
date: 2026-05-06
```

Remaining risks:

- Implementing issue 5209 may expose later index-signature semantic or runtime blockers.
