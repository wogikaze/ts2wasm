---
id: 1113
title: "Implement Capturedshorthandpropertyassignmentnocheck"
type: spike
area: frontend/syntax
class: done
priority: P1
depends_on: []
blocks: []
created: 2026-05-01
updated: 2026-05-06
completed: 2026-05-06
---
> **Reopened by false-done audit** (2026-05-07)
> Classification: must-reopen
> Reason: Batch-migrated to issues/open/ without implementation commits.
> Evidence: Empty completion evidence. No feat/fix commit for #1113.

## Summary

Triage capturedShorthandPropertyAssignmentNoCheck across 1 failing reference test cases and split this bucket into implementation-ready child issues.

## Problem

Reference test results show 1 cases fail in directory `capturedShorthandPropertyAssignmentNoCheck` with diagnostics: parser-syntax. The compiler cannot handle these syntax/semantics, preventing compilation of code in this category.

Problem: capturedShorthandPropertyAssignmentNoCheck has 1 reference failures and needs smart-triage evidence before implementation starts.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/capturedShorthandPropertyAssignmentNoCheck.ts
```

Coverage window:

```sh
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/capturedShorthandPropertyAssignmentNoCheck.ts --detail
```

## Desired final state

This generated bucket is either split into implementation-ready child issues or superseded by an existing open/done issue with matching evidence. Do not implement directly from this bucket.

## Scope

In scope:

- [x] Inspect the smart triage report below
- [x] Confirm whether existing open/done issues already cover this bucket
- [x] Supersede this stale parser bucket with the existing issue-211 method-call tracking
- [x] Preserve exact reproduction commands and representative AST/diagnostic evidence in this issue before closure

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
- [x] This closure records an exact `mise run reference-triage -- ...` command
- [x] This issue includes failing path, diagnostic code, source context, visible symbols, and parser/TypeScript AST evidence
- [x] Superseding issue evidence names the exact diagnostic/stdout change

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run
```

Impacted commands:

```sh
mise run reference-coverage -- tsc --limit 2
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/capturedShorthandPropertyAssignmentNoCheck.ts --detail
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/capturedShorthandPropertyAssignmentNoCheck.ts
```

Not run:

- none

## Docs / current-state / issue sync

Final-state docs:

- [x] not affected

Current state:

- [x] not affected

Follow-up issues:

- [x] superseded by `issues/open/211-complete-this-receiver-binding-semantics.md`
- [x] related umbrella: `issues/open/435-implement-method-call.md`

## Notes

## Affected test files

- `reference/typescript/tests/cases/compiler/capturedShorthandPropertyAssignmentNoCheck.ts`

## Duplicate detection

- none found by path/title/feature scan

## Smart triage

Fresh triage shows the original parser-syntax bucket is stale. The parser now
accepts the shorthand object literal `({ value })` and builds an AST. The
current blocker is the existing issue-211 method-call boundary for calling a
function-valued local parameter.

### Smart triage: capturedShorthandPropertyAssignmentNoCheck

- Issue class: `triage-needed`
- Feature label: `method-call`
- Diagnostic: `UnsupportedSyntax` / `parser-or-frontend-unsupported`
- Path: `reference/typescript/tests/cases/compiler/capturedShorthandPropertyAssignmentNoCheck.ts`

Reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/capturedShorthandPropertyAssignmentNoCheck.ts
```

Failure location:

```json
{
  "code": "UnsupportedSyntax",
  "message": "issue-211: function-valued local calls such as extracted method `fn(...)` are not supported; call receiver.method(...) directly at 138..142",
  "line": 7,
  "column": 30
}
```

Source context:

```text
4 | for (const value of [1, 2, 3]) {
5 |     fns.push(() => ({ value }));
6 | }
7 | const result = fns.map(fn => fn());
8 | console.log(result)
```

Compiler evidence:

```text
tokens: ok
ast: ok; arrow body object literal contains shorthand property `value`
resolved/lowered: issue-211 function-valued local call at `fn()`
```

TypeScript AST sees `CallExpression -> ArrowFunction -> CallExpression "fn()"`.
The current blocker is not parser-syntax and is superseded by
`issues/open/211-complete-this-receiver-binding-semantics.md`, which explicitly
keeps dynamic/function-valued local calls issue-linked as unsupported, and by
the broader method-call umbrella `issues/open/435-implement-method-call.md`.

## Completion evidence

capturedShorthandPropertyAssignmentNoCheck triage is complete. No new parser
child issue is needed because the original shorthand object literal syntax now
parses; the remaining blocker is the established issue-211 method-call
diagnostic.

Commits:

- close commit records fresh triage and supersedes this stale parser bucket

Validation result:

```text
command: python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/capturedShorthandPropertyAssignmentNoCheck.ts --detail --no-dashboard-data
result: pass on the main checkout; 1 executed, current failure is UnsupportedSyntax method-call
date: 2026-05-06

command: python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/capturedShorthandPropertyAssignmentNoCheck.ts
result: pass; AST succeeds and lowering reports issue-211 at fn()
date: 2026-05-06
```

Remaining risks:

- none
