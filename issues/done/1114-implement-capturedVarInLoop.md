---
id: 1114
title: "Implement Capturedvarinloop"
type: spike
area: frontend/syntax
class: done
priority: P1
depends_on: [5001]
blocks: []
created: 2026-05-01
updated: 2026-05-06
completed: 2026-05-06
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
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/capturedVarInLoop.ts --detail
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/capturedVarInLoop.ts
```

Not run:

- none

## Docs / current-state / issue sync

Final-state docs:

- [x] not affected

Current state:

- [x] not affected

Follow-up issues:

- [x] `issues/done/5215-fix-array-includes-wat-stack-mismatch.md`

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
`issues/done/5215-fix-array-includes-wat-stack-mismatch.md` owns
this lowering slice.

## Completion evidence

capturedVarInLoop triage is complete. The single current blocker is represented
by focused implementation issue 5215.

Commits:

- `cc3872b8` issues: split captured var loop arrow call blocker

Validation result:

```text
command: python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/capturedVarInLoop.ts --detail --no-dashboard-data
result: pass on the main checkout; 1 executed, current failure is UnsupportedSyntax method-call
date: 2026-05-06

command: python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/capturedVarInLoop.ts
result: pass; AST succeeds and lowering reports issue-211 at lambda1(len)
date: 2026-05-06

command: python scripts/manager.py update-issue-index
result: pass
date: 2026-05-06

command: python scripts/manager.py update-issue-index --check
result: pass
date: 2026-05-06

command: python scripts/manager.py check-issue-health
result: pass
date: 2026-05-06

command: python scripts/manager.py check-issue-readiness -- --fail-ready-below 80
result: pass; child issue 5215 is M-sized and ready
date: 2026-05-06

command: git diff --check
result: pass
date: 2026-05-06
```

Remaining risks:

- none
