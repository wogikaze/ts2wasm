---
id: 1204
title: "Implement Classextendsmultiplebaseclasses"
type: spike
area: frontend/syntax
class: done
priority: P1
depends_on: []
blocks: []
created: 2026-05-01
updated: 2026-05-07
completed: 2026-05-07
---

## Summary

Triage classExtendsMultipleBaseClasses across 1 failing reference test cases and split this bucket into implementation-ready child issues.

## Problem

Fresh smart triage confirms this is a narrow parser/diagnostic boundary for a
class `extends` clause with two base expressions. The compiler currently stops
at the comma, while TypeScript reports TS1174.

Problem: classExtendsMultipleBaseClasses was a stale generated bucket and
needed smart-triage evidence before implementation starts.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/classExtendsMultipleBaseClasses.ts
```

Coverage window:

```sh
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/classExtendsMultipleBaseClasses.ts --detail
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
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/classExtendsMultipleBaseClasses.ts --detail
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/classExtendsMultipleBaseClasses.ts
```

Not run:

- none

## Docs / current-state / issue sync

Final-state docs:

- [x] not affected

Current state:

- [x] not affected

Follow-up issues:

- [x] created: `issues/open/5317-report-multiple-class-heritage-bases.md`

## Notes

## Affected test files

- `reference/typescript/tests/cases/compiler/classExtendsMultipleBaseClasses.ts`

## Duplicate detection

- none found by path/title/feature scan

## Smart triage

Generated 2026-05-07:

```sh
TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm \
  python scripts/manager.py reference-triage tsc \
  reference/typescript/tests/cases/compiler/classExtendsMultipleBaseClasses.ts
```

Result:

```text
diagnostic: UnsupportedSyntax / parser-or-frontend-unsupported
message: expected LeftBrace, got Some(Comma) at 63..64
source: class C extends A,B { }
visible symbols: class A, class B, class C
TypeScript oracle:
TS1174 Classes can only extend a single class.
```

TypeScript AST evidence:

```text
ClassDeclaration -> HeritageClause "extends A,B" -> ExpressionWithTypeArguments "A"
```

Duplicate review:

- `issues/open/5260-report-class-heritage-trailing-comma.md` is related but
  only covers `class D extends C, {}` trailing comma and excludes multiple
  heritage clauses.
- No exact open/done owner was found for TS1174 multiple class heritage bases.

## Completion evidence

This generated bucket was split and closed as superseded by issue 5317.

Commits:

- this commit

Validation result:

```text
command: TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/classExtendsMultipleBaseClasses.ts
result: pass; parser comma failure reclassified as TS1174 multiple class heritage diagnostic work
date: 2026-05-07

command: TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/classExtendsMultipleBaseClasses.ts --detail --no-dashboard-data
result: pass; single-file window reports UnsupportedSyntax:1
date: 2026-05-07
```

Remaining risks:

- Implementation remains open in issue 5317.
