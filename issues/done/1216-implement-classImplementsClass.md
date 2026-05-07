---
id: 1216
title: "Implement Classimplementsclass"
type: spike
area: frontend/syntax
class: blocked
priority: P1
depends_on: [5261]
blocks: []
created: 2026-05-01
updated: 2026-05-06
---

## Summary

Closed by splitting the only remaining current blocker in the
`classImplementsClass` reference family to
`issues/open/5261-report-class-typed-missing-instance-method-calls.md`.

## Problem

Reference test results previously showed 7 cases failing in directory
`classImplementsClass` with diagnostics: parser-syntax. Fresh coverage now shows
6 build-pass files and 1 remaining unsupported file.

Problem: `classImplementsClass6.ts` reaches lowering and reports a generic
`issue-211: unknown receiver class for method bar` for `c.bar()` instead of a
class-typed missing instance method diagnostic.

## Current failure

Representative reproduction:

```sh
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/classImplementsClass6.ts
```

Coverage window:

```sh
python scripts/manager.py reference-coverage tsc --path-filter classImplementsClass --detail --no-dashboard-data
```

## Desired final state

This generated bucket is either split into implementation-ready child issues or superseded by an existing open/done issue with matching evidence. Do not implement directly from this bucket.

## Scope

In scope:

- [x] Inspect the smart triage report below
- [x] Confirm existing open/done issues do not cover the exact current boundary
- [x] Split one observable behavior into an implementation-ready child issue
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

- [x] Duplicate candidates below are confirmed as no-match for the exact current boundary
- [x] Child issue contains an exact `python scripts/manager.py reference-triage ...` command
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
mise run reference-coverage -- tsc --limit 14
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/classImplementsClass1.ts --detail
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/classImplementsClass1.ts
```

Not run:

- none

## Docs / current-state / issue sync

Final-state docs:

- [x] not affected

Current state:

- [x] not affected

Follow-up issues:

- [x] `issues/open/5261-report-class-typed-missing-instance-method-calls.md`

## Notes

## Affected test files

- `reference/typescript/tests/cases/compiler/classImplementsClass1.ts`
- `reference/typescript/tests/cases/compiler/classImplementsClass2.ts`
- `reference/typescript/tests/cases/compiler/classImplementsClass3.ts`
- `reference/typescript/tests/cases/compiler/classImplementsClass4.ts`
- `reference/typescript/tests/cases/compiler/classImplementsClass5.ts`
- `reference/typescript/tests/cases/compiler/classImplementsClass6.ts`
- `reference/typescript/tests/cases/compiler/classImplementsClass7.ts`

Fresh coverage on 2026-05-06:

```text
build_pass=6
unsupported=1
unsupported_diagcodes=UnsupportedSyntax:1
unsupported_features=unknown-unsupported:1

build-pass: classImplementsClass1.ts, classImplementsClass2.ts,
classImplementsClass3.ts, classImplementsClass4.ts,
classImplementsClass5.ts, classImplementsClass7.ts
unsupported: classImplementsClass6.ts
```

## Duplicate detection

- `issues/done/5222-parse-ambient-generic-variable-type-annotations.md`
  is related but not exact: it covers interface-typed erased receivers such as
  `var s: Sequence<string>; s.groupBy(...)`, while this bucket has class-typed
  ambient locals `declare var c: C; c.bar()`.
- broad method-call/class buckets are not exact matches because this current
  boundary is a class-typed missing/static-only instance method diagnostic.

## Smart triage

Fresh commands:

```sh
python scripts/manager.py reference-coverage tsc --path-filter classImplementsClass --detail --no-dashboard-data
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/classImplementsClass6.ts
```

Observed result on 2026-05-06:

```text
Diagnostic: UnsupportedSyntax
Message: issue-211: unknown receiver class for method `bar` at 279..286
Feature label: class
Source context:
  c.bar();  // error
  c2.bar(); // should error

tokens: ok
AST: ok; ClassDecl A has static::bar and foo, ClassDecl C implements A has foo,
     ClassDecl C2 extends A, ambient c/c2, calls c.bar() and c2.bar()
resolved/lowered: issue-211 unknown receiver class for method `bar`
TypeScript oracle: TS2339 on c.bar and TS2576 on c2.bar
```

Split issue:

- `issues/open/5261-report-class-typed-missing-instance-method-calls.md`

## Completion evidence

Fill only when moving to `done/`.

Commits:

- Split to `issues/open/5261-report-class-typed-missing-instance-method-calls.md`; see local commit for this issue cleanup.

Validation result:

```text
command: python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/classImplementsClass6.ts
result: pass; current blocker split to issue 5261
date: 2026-05-06
```

Remaining risks:

- Six sibling references are build-pass, but TypeScript still reports semantic
  diagnostics in some of them. This closure only removes the generated blocker
  bucket; semantic parity is tracked separately.
