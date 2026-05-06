---
id: 1239
title: "Implement Classstaticpropertyaccess"
type: spike
area: frontend/syntax
class: triage-needed
priority: P1
depends_on: []
blocks: []
created: 2026-05-01
updated: 2026-05-06
---

## Summary

Closed after splitting the current blocker to
`issues/open/5267-parse-string-literal-class-member-names.md`. Fresh triage
shows the first failure is a parser gap for quoted string-literal class member
names after `public static`.

## Problem

Reference test results show 1 case fails in directory
`classStaticPropertyAccess`. Fresh triage confirms the current blocker is the
quoted static method name `public static "\""() {}`.

Problem: `classStaticPropertyAccess.ts` reports `expected LeftParen, got
Some(String("\""))` at the string-literal class member name.

## Current failure

Representative reproduction:

```sh
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/classStaticPropertyAccess.ts
```

Coverage window:

```sh
python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/classStaticPropertyAccess.ts --detail --no-dashboard-data
```

## Desired final state

This generated bucket is either split into implementation-ready child issues or superseded by an existing open/done issue with matching evidence. Do not implement directly from this bucket.

## Scope

In scope:

- [x] Inspect the smart triage report below
- [x] Confirm existing parser issues are related but do not own this exact quoted member-name behavior
- [x] Split one observable behavior into child issue 5267
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

- [x] Duplicate candidates below are confirmed as no-match for the exact behavior
- [x] Child issue 5267 contains an exact `python scripts/manager.py reference-triage ...` command
- [x] Child issue includes failing path, diagnostic code, source context, visible symbols, and parser/TypeScript AST evidence
- [x] Child issue acceptance names the exact reference path and diagnostic change

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run
```

Impacted commands:

```sh
mise run reference-coverage -- tsc --limit 2
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/classStaticPropertyAccess.ts --detail
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/classStaticPropertyAccess.ts
```

Not run:

- none

## Docs / current-state / issue sync

Final-state docs:

- [x] not affected

Current state:

- [x] not affected

Follow-up issues:

- [x] created: `issues/open/5267-parse-string-literal-class-member-names.md`

## Notes

## Affected test files

- `reference/typescript/tests/cases/compiler/classStaticPropertyAccess.ts`

Source context:

```ts
class A {
    public static "\""() {}
    public static x: number = 1;
    public static y: number = 1;
    private static _b: number = 2;
}
```

## Duplicate detection

- `issues/open/059-implement-parser-syntax-extensions.md` is the broader parser
  syntax epic and should not be selected directly.
- `issues/open/5251-parse-computed-class-member-names-in-class-declarations.md`
  is related but covers computed member names, not quoted string-literal names.
- No existing open issue owned the exact `public static "\""() {}` parser
  failure, so this bucket was split to issue 5267.

## Smart triage

Fresh commands:

```sh
python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/classStaticPropertyAccess.ts --detail --no-dashboard-data
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/classStaticPropertyAccess.ts
```

Observed result on 2026-05-06:

```text
coverage: unsupported=1
unsupported_diagcodes: UnsupportedSyntax:1
unsupported_features: unknown-unsupported:1

Diagnostic: UnsupportedSyntax
Message: expected LeftParen, got Some(String("\"")) at 74..78
Source: public static "\""() {}
tokens: ok; String("\"") token follows public static
AST: fails before class member construction
resolved: same parser failure
TypeScript oracle: parses; expected diagnostics TS2576, TS2341, TS2339
Child issue: 5267
```

## Completion evidence

Fill only when moving to `done/`.

Commits:

- Split to `issues/open/5267-parse-string-literal-class-member-names.md`; see local commit for this issue cleanup.

Validation result:

```text
command: python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/classStaticPropertyAccess.ts
result: pass; reproduced quoted string-literal class member parser failure and split child issue 5267
date: 2026-05-06
```

Remaining risks:

- none; remaining implementation work is tracked by issue 5267
