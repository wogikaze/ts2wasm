---
id: 1114
title: "Implement Capturedvarinloop"
type: spike
area: frontend/syntax
class: triage-needed
priority: P1
depends_on: [5001]
blocks: []
created: 2026-05-01
updated: 2026-05-06
---

## Summary

Triage capturedVarInLoop across 1 failing reference test cases and split this bucket into implementation-ready child issues.

## Problem

Reference test results show 1 cases fail in directory `capturedVarInLoop` with diagnostics: method-call. The compiler cannot handle these syntax/semantics, preventing compilation of code in this category.

Problem: capturedVarInLoop has 1 reference failures and needs smart-triage evidence before implementation starts.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/capturedVarInLoop.ts
```

Coverage window:

```sh
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/capturedVarInLoop.ts --detail
```

## Desired final state

This generated bucket is either split into implementation-ready child issues or superseded by an existing open/done issue with matching evidence. Do not implement directly from this bucket.

## Scope

In scope:

- [ ] Inspect the smart triage report below
- [ ] Confirm whether existing open/done issues already cover this bucket
- [ ] Split one feature family, one observable behavior, or one fixed reference window into child issues
- [ ] Preserve exact reproduction commands and representative AST/diagnostic evidence in each child issue

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

- [ ] Duplicate candidates below are confirmed as no-match or this issue is superseded
- [ ] At least one child issue contains an exact `mise run reference-triage -- ...` command
- [ ] Child issue includes failing path, diagnostic code, source context, visible symbols, and parser/TypeScript AST evidence
- [ ] Child issue acceptance names the exact fixture/reference path and diagnostic/stdout change

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run
```

Impacted commands:

```sh
mise run reference-coverage -- tsc --limit 2
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/capturedVarInLoop.ts --detail
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/capturedVarInLoop.ts
```

Not run:

- none

## Docs / current-state / issue sync

Final-state docs:

- [ ] not affected

Current state:

- [ ] updated: `current-state.md` (repo root)

Follow-up issues:

- [ ] `issues/open/5215-support-loop-local-arrow-calls-from-arrow-closures.md`

## Notes

## Affected test files

- `reference/typescript/tests/cases/compiler/capturedVarInLoop.ts`

## Duplicate detection

- none found by path/title/feature scan

## Smart triage

Fresh triage shows this bucket is a method-call/lowering blocker, not
parser-syntax. The parser and AST succeed.

### Smart triage: capturedVarInLoop

- Issue class: `triage-needed`
- Feature label: `method-call`
- Diagnostic: `UnsupportedSyntax` / `parser-or-frontend-unsupported`
- Path: `reference/typescript/tests/cases/compiler/capturedVarInLoop.ts`

Reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/capturedVarInLoop.ts
```

Failure location:

```json
{
  "code": "UnsupportedSyntax",
  "message": "issue-211: function-valued local calls such as extracted method `lambda1(...)` are not supported; call receiver.method(...) directly at 169..181",
  "line": 6,
  "column": 30
}
```

Source context:

```text
3 | for (var i = 0; i < 10; i++) {
4 |     var str = 'x', len = str.length;
5 |     let lambda1 = (y) => { };
6 |     let lambda2 = () => lambda1(len);
7 | }
```

Compiler evidence:

```text
tokens: ok
ast: ok; lambda1 is an ArrowFn binding, lambda2 body is Call(Ident lambda1, Ident len)
resolved/lowered: issue-211 function-valued local call at lambda1(len)
```

TypeScript oracle reports no diagnostics. Child issue
`issues/open/5215-support-loop-local-arrow-calls-from-arrow-closures.md` owns
this lowering slice.

## Completion evidence

Fill only when moving to `done/`.

Commits:

- `...`

Validation result:

```text
command:
result:
date:
```

Remaining risks:

- none
