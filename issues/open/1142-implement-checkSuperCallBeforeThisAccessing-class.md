---
id: 1142
title: "Implement Checksupercallbeforethisaccessing Class"
type: spike
area: frontend/syntax
class: done
priority: P1
depends_on: [5233]
blocks: []
created: 2026-05-01
updated: 2026-05-06
completed: 2026-05-06
---
> **Reopened by false-done audit** (2026-05-07)
> Classification: must-reopen
> Reason: Batch-migrated to issues/open/ without implementation commits.
> Evidence: Empty completion evidence. No feat/fix commit for #1142.

## Summary

Triage checkSuperCallBeforeThisAccessing-class across 1 failing reference test cases and split this bucket into implementation-ready child issues.

## Problem

Reference test results show 1 cases fail in directory `checkSuperCallBeforeThisAccessing-class` with diagnostics: class. The compiler cannot handle these syntax/semantics, preventing compilation of code in this category.

Problem: checkSuperCallBeforeThisAccessing-class has 1 reference failures and needs smart-triage evidence before implementation starts.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/checkSuperCallBeforeThisAccessing9.ts
```

Coverage window:

```sh
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/checkSuperCallBeforeThisAccessing9.ts --detail
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
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/checkSuperCallBeforeThisAccessing9.ts --detail
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/checkSuperCallBeforeThisAccessing9.ts
```

Not run:

- none

## Docs / current-state / issue sync

Final-state docs:

- [x] not affected

Current state:

- [x] not affected

Follow-up issues:

- [x] created: `issues/open/5233-w0-harden-reference-coverage-prerequisites.md`

## Notes

## Affected test files

- `reference/typescript/tests/cases/compiler/checkSuperCallBeforeThisAccessing9.ts`

## Duplicate detection

Fresh duplicate scan found broad class/super buckets, but no exact open issue
for converting `super()` in a non-derived class from unsupported syntax into a
TS2335-style diagnostic:

- `issues/open/449-implement-super.md` is a broad test262 `super` triage bucket.
- `issues/done/047-implement-super-keyword.md` covers legal `super()` calls.
- `issues/open/421-implement-class.md` is broad class syntax and not an exact
  diagnostic work order.
- Private/static class element issues do not cover this constructor `super()`
  diagnostic.

## Smart triage

Fresh triage shows parser and AST construction are already successful. The
current blocker is the lowering diagnostic for a `super()` call in a class with
no `extends` clause.

### Smart triage: checkSuperCallBeforeThisAccessing9

- Issue class: `triage-needed`
- Feature label: `class`
- Diagnostic: `UnsupportedSyntax` / `parser-or-frontend-unsupported`
- Current compiler message: `super(...) used in class without extends`
- Path: `reference/typescript/tests/cases/compiler/checkSuperCallBeforeThisAccessing9.ts`

Reproduction:

```sh
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/checkSuperCallBeforeThisAccessing9.ts
```

Coverage:

```sh
python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/checkSuperCallBeforeThisAccessing9.ts --detail --no-dashboard-data
```

Coverage result:

```text
unsupported=1
unsupported_diagcodes=UnsupportedSyntax:1
unsupported_features=unknown-unsupported:1
```

Source context:

```ts
// @filename: noSuperInJSDocExtends.js
class Based { }
/** @extends {Based} */
class Derived2 {
    constructor() {
        super();
    }
}
```

Compiler evidence:

```text
tokens: ok
ast: ok; Derived2 has no extends and constructor body contains Call(callee=Ident("super"))
resolved/lowered: UnsupportedSyntax: super(...) used in class without extends
TypeScript oracle: TS2335 "'super' can only be referenced in a derived class."
```

Split result:

- `issues/open/5233-w0-harden-reference-coverage-prerequisites.md`

## Completion evidence

Fill only when moving to `done/`.

Commits:

- filled by closing commit

Validation result:

```text
command: python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/checkSuperCallBeforeThisAccessing9.ts --detail --no-dashboard-data
result: pass; reproduced current unsupported bucket
date: 2026-05-06

command: python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/checkSuperCallBeforeThisAccessing9.ts
result: pass; reproduced non-derived super() UnsupportedSyntax boundary and split to issue 5233
date: 2026-05-06
```

Remaining risks:

- Implementing issue 5233 may expose `this` property diagnostics or broader JSDoc/class-checking differences.
