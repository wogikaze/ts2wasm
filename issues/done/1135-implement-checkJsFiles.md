---
id: 1135
title: "Implement Checkjsfiles"
type: spike
area: frontend/syntax
class: done
priority: P1
depends_on: [5227]
blocks: []
created: 2026-05-01
updated: 2026-05-06
completed: 2026-05-06
---

## Summary

Triage checkJsFiles across 1 failing reference test cases and split this bucket into implementation-ready child issues.

## Problem

Reference test results show 1 cases fail in directory `checkJsFiles` with diagnostics: method-call. The compiler cannot handle these syntax/semantics, preventing compilation of code in this category.

Problem: checkJsFiles has 1 reference failures and needs smart-triage evidence before implementation starts.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/checkJsFiles_skipDiagnostics.ts
```

Coverage window:

```sh
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/checkJsFiles_skipDiagnostics.ts --detail
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
- [x] At least one child issue contains an exact `reference-triage` command
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
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/checkJsFiles_skipDiagnostics.ts --detail
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/checkJsFiles_skipDiagnostics.ts
```

Not run:

- none

## Docs / current-state / issue sync

Final-state docs:

- [x] not affected

Current state:

- [x] not affected

Follow-up issues:

- [x] created: `issues/open/5227-honor-ts-ignore-for-js-call-diagnostics.md`

## Notes

## Affected test files

- `reference/typescript/tests/cases/compiler/checkJsFiles_skipDiagnostics.ts`

## Duplicate detection

Fresh duplicate scan found broad function-resolution and issue-211 local-call
issues, but no exact child for JavaScript checkJs `// @ts-ignore` suppression
of call-expression diagnostics.

No-match rationale:

- `issues/done/211-complete-this-receiver-binding-semantics.md` intentionally
  leaves dynamic/function-valued local calls as issue-linked unsupported forms.
  This bucket's first blocker is not making `x()` executable; it is honoring a
  TypeScript directive that suppresses the diagnostic for selected calls.
- `issues/open/431-implement-function-resolution.md` is a broad generated
  test262 function-resolution bucket and is not an executable work order.
- `issues/done/057-implement-function-resolution.md` covers ordinary function
  declaration/name resolution and does not cover JavaScript comment directive
  suppression.

## Smart triage

Fresh triage shows the original method-call bucket is now a narrower
diagnostic-suppression blocker. Tokens and AST succeed for the representative,
and the lowering stage stops at the first `x()` call even though it is covered
by `/// @ts-ignore`.

### Smart triage: checkJsFiles_skipDiagnostics

- Issue class: `triage-needed`
- Feature label: `function-resolution`
- Diagnostic: `UnresolvedFunction` / `resolver-symbol`
- Current compiler message: `issue-211: function-valued local calls such as extracted method x(...) are not supported`
- Path: `reference/typescript/tests/cases/compiler/checkJsFiles_skipDiagnostics.ts`

Reproduction:

```sh
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/checkJsFiles_skipDiagnostics.ts
```

Coverage:

```sh
python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/checkJsFiles_skipDiagnostics.ts --detail --no-dashboard-data
```

Coverage result:

```text
unsupported=1
unsupported_diagcodes=UnresolvedFunction:1
unsupported_features=function-resolution:1
```

Source context:

```ts
// @target: es2015
// @allowJs: true
// @checkJs: true
// @noEmit: true

// @fileName: a.js
var x = 0;

/// @ts-ignore
x();
```

Visible symbols before failure:

```json
[
  {
    "kind": "binding",
    "name": "x",
    "line": 7,
    "column": 1,
    "initializer": "0"
  }
]
```

Compiler evidence:

```text
tokens: ok; all x(...) call tokens are present
ast: ok; all x(...) calls are represented as Expr(Call(Ident("x"), ...))
resolved/lowered: issue-211 at the first ignored x() call, byte span 130..133
TypeScript oracle: TS2349 only for the block-comment pseudo-directive calls at lines 35 and 41
```

Split result:

- `issues/open/5227-honor-ts-ignore-for-js-call-diagnostics.md`

## Completion evidence

Fill only when moving to `done/`.

Commits:

- filled by closing commit

Validation result:

```text
command: python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/checkJsFiles_skipDiagnostics.ts --detail --no-dashboard-data
result: pass; reproduced current UnresolvedFunction/function-resolution blocker
date: 2026-05-06

command: python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/checkJsFiles_skipDiagnostics.ts
result: pass; reproduced issue-211 at first @ts-ignore-covered x() and split to issue 5227
date: 2026-05-06
```

Remaining risks:

- Implementing issue 5227 may expose the next unsuppressed callability diagnostic in this fixture.
