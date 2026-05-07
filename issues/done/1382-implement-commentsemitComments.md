---
id: 1382
title: "Implement Commentsemitcomments"
type: spike
area: frontend/syntax
class: triage-needed
priority: P1
depends_on: []
blocks: [5279]
created: 2026-05-01
updated: 2026-05-07
---

## Summary

Closed as superseded by
`issues/open/5279-report-function-typed-local-call-definite-assignment.md`.

Fresh triage shows this generated bucket is not currently blocked by comment
emit behavior. The first reported blocker is an unspanned function-resolution
failure for `fooVar()`, matching the function-typed local call
definite-assignment issue.

## Problem

Reference test results originally showed 1 case failing in directory
`commentsemitComments` with diagnostics: parser-syntax. Fresh focused triage
on 2026-05-07 shows tokens and AST succeed, then the compiler reports an
unspanned `UnresolvedFunction` for the function-typed local call `fooVar()`.

Problem: `commentsemitComments.ts` currently reports `UnresolvedFunction` for
`fooVar()` before comment emit behavior is reached.

## Current failure

Representative reproduction:

```sh
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/commentsemitComments.ts
```

Coverage window:

```sh
python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/commentsemitComments.ts --detail --no-dashboard-data
```

Observed 2026-05-07:

```text
coverage: build_pass=0, unsupported=1
unsupported_diagcodes=UnresolvedFunction:1
unsupported_features=function-resolution:1
```

## Desired final state

This generated bucket is closed. Implementation should proceed through
`issues/open/5279-report-function-typed-local-call-definite-assignment.md`;
comment emit behavior should be rechecked after the function-typed local call
blocker advances.

## Scope

In scope:

- [x] Inspect the smart triage report below
- [x] Confirm whether existing open/done issues already cover this bucket
- [x] Supersede this bucket with the existing function-typed local call issue
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
- [x] Superseding issue contains the exact function-typed local call definite-assignment family
- [x] This issue includes failing path, diagnostic code, source context, visible symbols, parser/TypeScript AST evidence, and TypeScript oracle evidence
- [x] Completion evidence names the exact reference path and diagnostic/stdout change

## Validation

Required commands:

```sh
python scripts/manager.py update-issue-index
python scripts/manager.py update-issue-index --check
python scripts/manager.py check-issue-health
python scripts/manager.py check-issue-readiness -- --fail-ready-below 80
git diff --check
```

Impacted commands:

```sh
python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/commentsemitComments.ts --detail --no-dashboard-data
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/commentsemitComments.ts
```

Not run:

- `cargo fmt --all --check` (issue metadata only)
- `cargo nextest run` (issue metadata only)

## Docs / current-state / issue sync

Final-state docs:

- [x] not affected

Current state:

- [x] not affected

Follow-up issues:

- [x] superseded by: `issues/open/5279-report-function-typed-local-call-definite-assignment.md`

## Notes

## Affected test files

- `reference/typescript/tests/cases/compiler/commentsemitComments.ts`

## Duplicate detection

- `issues/open/5279-report-function-typed-local-call-definite-assignment.md`
  owns the exact first blocker: `var fooVar: () => void; fooVar();` should
  report a source-spanned TS2454-like diagnostic instead of unspanned
  `UnresolvedFunction`.
- `issues/done/5198-support-class-method-overload-signatures-for-element-access-calls.md`
  is adjacent for the later `c.foo1` overload group, but it is not the first
  diagnostic reported by current coverage.

## Smart triage

Generated 2026-05-07.

```text
### Smart triage: Triage function resolution: commentsemitComments

- Issue class: triage-needed
- Feature label: function-resolution
- Diagnostic: UnresolvedFunction / resolver-symbol
- Path: reference/typescript/tests/cases/compiler/commentsemitComments.ts
```

Source context:

```text
12 | /** variable with function type comment*/
13 | var fooVar: () => void;
14 | foo(50);
15 | fooVar();
```

Compiler evidence:

```text
tokens: ok
ast: ok; Let fooVar with function type annotation erased to Undefined, then Call fooVar()
coverage: UnresolvedFunction for fooVar with no source span
visible symbols include binding fooVar
```

TypeScript oracle:

```text
TS2454: Variable 'fooVar' is used before being assigned.
hint: fooVar has type () => void
```

## Completion evidence

Fill only when moving to `done/`.

Commits:

- this commit

Validation result:

```text
command: python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/commentsemitComments.ts --detail --no-dashboard-data
result: build_pass=0, unsupported=1, unsupported_diagcodes=UnresolvedFunction:1
date: 2026-05-07

command: python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/commentsemitComments.ts
result: function-typed local call definite-assignment blocker; superseded by issue 5279
date: 2026-05-07
```

Remaining risks:

- Later class method overload and comment emit behavior in this reference file
  have not been reached by the current first blocker.
