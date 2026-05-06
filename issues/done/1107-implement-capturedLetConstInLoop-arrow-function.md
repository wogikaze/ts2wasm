---
id: 1107
title: "Implement Capturedletconstinloop Arrow Function"
type: spike
area: frontend/syntax
class: blocked
priority: P1
depends_on: []
blocks: []
created: 2026-05-01
updated: 2026-05-06
---

## Summary

Triage capturedLetConstInLoop-arrow-function across 2 failing reference test cases and split this bucket into implementation-ready child issues.

## Problem

Reference test results show 2 cases fail in directory `capturedLetConstInLoop-arrow-function` with diagnostics: arrow-function. The compiler cannot handle these syntax/semantics, preventing compilation of code in this category.

Problem: capturedLetConstInLoop-arrow-function has 2 reference failures and needs smart-triage evidence before implementation starts.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/capturedLetConstInLoop10.ts
```

Coverage window:

```sh
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/capturedLetConstInLoop10.ts --detail
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
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/capturedLetConstInLoop10.ts --detail
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/capturedLetConstInLoop10.ts
```

Not run:

- none

## Docs / current-state / issue sync

Final-state docs:

- [x] not affected

Current state:

- [x] not affected

Follow-up issues:

- [x] none; both affected reference files now build

## Notes

## Affected test files

- `reference/typescript/tests/cases/compiler/capturedLetConstInLoop10.ts`
- `reference/typescript/tests/cases/compiler/capturedLetConstInLoop10_ES6.ts`

## Duplicate detection

- `issues/done/415-implement-arrow-function.md` - Implement arrow functions (same feature label, title overlap)
- `issues/done/430-implement-function.md` - Implement function support (same feature label, title overlap)
- `issues/done/456-implement-APISample-arrow-function.md` - Implement Apisample Arrow Function (same feature label, title overlap)
- `issues/open/542-implement-APISample-arrow-function.md` - Implement Apisample Arrow Function (same feature label, title overlap)
- `issues/done/036-implement-arrow-function.md` - Implement arrow function (same feature label, title overlap)
- `issues/done/062d-function-this-and-arguments.md` - Implement function this and arguments semantics (same feature label, title overlap)
- `issues/done/210-implement-arrow-function-closure-lexical-this.md` - Implement arrow function closure and lexical this semantics (same feature label, title overlap)

## Smart triage

### Smart triage: Build pass: capturedLetConstInLoop10

- Issue class: `none`
- Feature label: `build-pass`
- Diagnostic: `BuildPass` / `pass`
- Path: `reference/typescript/tests/cases/compiler/capturedLetConstInLoop10.ts`

Reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/capturedLetConstInLoop10.ts
```

Result:

```text
ts2wasm build succeeded
```

### Smart triage: Build pass: capturedLetConstInLoop10_ES6

- Issue class: `none`
- Feature label: `build-pass`
- Diagnostic: `BuildPass` / `pass`
- Path: `reference/typescript/tests/cases/compiler/capturedLetConstInLoop10_ES6.ts`

Reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/capturedLetConstInLoop10_ES6.ts
```

Result:

```text
ts2wasm build succeeded
```

Evidence:

- Tokens, AST, and resolved dumps succeed for both affected files.
- TypeScript oracle reports no diagnostics for both affected files.
- No implementation-ready child issue is needed because no compiler blocker
  remains in this generated bucket.

## Completion evidence

Fill only when moving to `done/`.

Commits:

- `...` pending

Validation result:

```text
command: python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/capturedLetConstInLoop10.ts
result: pass; build succeeded
date: 2026-05-06

command: python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/capturedLetConstInLoop10_ES6.ts
result: pass; build succeeded
date: 2026-05-06
```

Remaining risks:

- none
