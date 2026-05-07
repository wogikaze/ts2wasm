---
id: 1361
title: "Implement Commentsclass"
type: spike
area: frontend/resolver
class: blocked
priority: P1
depends_on: []
blocks: [5192]
created: 2026-05-01
updated: 2026-05-06
---

## Summary

Closed as superseded by `issues/open/5192-support-first-class-class-constructor-values.md`.

## Problem

Reference test results show 1 case failing in directory `commentsClass` with
diagnostics: name-resolution. Fresh triage shows parser and AST construction now
succeed; the current blocker is the shared class runtime value boundary.

Problem: `commentsClass.ts` currently reports `issue-5011` when `c2` is used as
an expression value in `var i2_c = c2;`.

## Current failure

Representative reproduction:

```sh
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/commentsClass.ts
```

Coverage window:

```sh
python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/commentsClass.ts --detail --no-dashboard-data
```

Observed 2026-05-06:

```text
UnsupportedSyntax: issue-5011: class `c2` cannot be used as a value — class runtime is not yet supported at 187..189
```

## Desired final state

This generated bucket is closed. Implementation should proceed through
`issues/open/5192-support-first-class-class-constructor-values.md`.

## Scope

In scope:

- [x] Inspect the smart triage report below
- [x] Confirm whether existing open/done issues already cover this bucket
- [x] Supersede this bucket with the existing class runtime value issue
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
- [x] Superseding issue contains matching `issue-5011` class-value evidence
- [x] This issue includes failing path, diagnostic code, source context, visible symbols, AST evidence, and TypeScript oracle evidence
- [x] Superseding issue acceptance names the diagnostic change for class constructor values

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
python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/commentsClass.ts --detail --no-dashboard-data
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/commentsClass.ts
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

- [x] superseded by `issues/open/5192-support-first-class-class-constructor-values.md`

## Notes

## Affected test files

- `reference/typescript/tests/cases/compiler/commentsClass.ts`

## Duplicate detection

- `issues/open/5192-support-first-class-class-constructor-values.md` owns the
  shared class constructor value boundary. `commentsClass.ts` fails at
  `var i2_c = c2;`, which is the same `issue-5011` family.
- `issues/done/5011-class-runtime-value-semantics.md` documents the current
  structural diagnostic that prevents silent class value erasure.
- `issues/open/421-implement-class.md` is the broad class syntax issue and is
  too wide for this current blocker.

## Smart triage

Generated 2026-05-06.

```text
### Smart triage: Triage class: commentsClass

- Issue class: triage-needed
- Feature label: class
- Diagnostic: UnsupportedSyntax / parser-or-frontend-unsupported
- Path: reference/typescript/tests/cases/compiler/commentsClass.ts
```

Failure:

```text
issue-5011: class `c2` cannot be used as a value — class runtime is not yet supported at 187..189
```

Source context:

```ts
class c2 {
} // trailing comment1
var i2 = new c2();
var i2_c = c2;
class c3 {
```

Compiler evidence:

```text
tokens: ok
ast: ok, includes ClassDecl c2, Let i2 = New(Ident c2), Let i2_c = Ident c2
resolved: issue-5011 at identifier c2 in `var i2_c = c2;`
```

TypeScript oracle:

```text
ok: true
diagnostics: []
binding i2: c2
binding i2_c: typeof c2
```

## Completion evidence

Fill only when moving to `done/`.

Commits:

- this commit

Validation result:

```text
command: python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/commentsClass.ts --detail --no-dashboard-data
result: unsupported=1, unsupported_diagcodes=UnsupportedSyntax:1, unsupported_features=unknown-unsupported:1
date: 2026-05-06

command: python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/commentsClass.ts
result: parser/AST ok; resolved fails with issue-5011 class value use; superseded by issue 5192
date: 2026-05-06
```

Remaining risks:

- Comment handling is not independently validated until the class value boundary
  advances through issue 5192.
