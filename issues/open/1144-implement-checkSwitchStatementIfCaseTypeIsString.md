---
id: 1144
title: "Implement Checkswitchstatementifcasetypeisstring"
type: spike
area: frontend/syntax
class: done
priority: P1
depends_on: [5234]
blocks: []
created: 2026-05-01
updated: 2026-05-06
completed: 2026-05-06
---

## Summary

Triage checkSwitchStatementIfCaseTypeIsString across 1 failing reference test cases and split this bucket into implementation-ready child issues.

## Problem

Reference test results show 1 cases fail in directory `checkSwitchStatementIfCaseTypeIsString` with diagnostics: arrow-function. The compiler cannot handle these syntax/semantics, preventing compilation of code in this category.

Problem: checkSwitchStatementIfCaseTypeIsString has 1 reference failures and needs smart-triage evidence before implementation starts.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/checkSwitchStatementIfCaseTypeIsString.ts
```

Coverage window:

```sh
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/checkSwitchStatementIfCaseTypeIsString.ts --detail
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
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/checkSwitchStatementIfCaseTypeIsString.ts --detail
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/checkSwitchStatementIfCaseTypeIsString.ts
```

Not run:

- none

## Docs / current-state / issue sync

Final-state docs:

- [x] not affected

Current state:

- [x] not affected

Follow-up issues:

- [x] created: `issues/open/5234-w0-implement-host-deny-and-auditable-e2e-manifest-verification.md`

## Notes

## Affected test files

- `reference/typescript/tests/cases/compiler/checkSwitchStatementIfCaseTypeIsString.ts`

## Duplicate detection

Fresh duplicate scan found related Array/method-call buckets but no exact open
issue for array-shaped function/class-method parameter annotations:

- `issues/open/313-implement-array-builtin.md` is a broad Array builtin
  umbrella, not an implementation-ready typed-parameter receiver slice.
- `issues/open/673-implement-arrayEvery.md` and
  `issues/open/677-implement-arrayFlatMap.md` hit similar issue-211 unknown
  receiver boundaries for array locals, but they are generated buckets and do
  not cover class method parameter annotation tracking.
- `issues/open/5222-parse-ambient-generic-variable-type-annotations.md`
  is related for interface-typed receivers, but not array-shaped parameters and
  built-in callback methods.

## Smart triage

Fresh triage shows tokens and AST already succeed for the class method, arrow
callback, and switch statement. The current blocker is `x.forEach(...)` on a
parameter whose TypeScript type is `Array<string>` but whose runtime/lowering
receiver is not tracked as a known array.

### Smart triage: checkSwitchStatementIfCaseTypeIsString

- Issue class: `triage-needed`
- Feature label: `class`
- Diagnostic: `UnsupportedSyntax` / `parser-or-frontend-unsupported`
- Current compiler message: `issue-211: unknown receiver class for method forEach`
- Path: `reference/typescript/tests/cases/compiler/checkSwitchStatementIfCaseTypeIsString.ts`

Reproduction:

```sh
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/checkSwitchStatementIfCaseTypeIsString.ts
```

Coverage:

```sh
python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/checkSwitchStatementIfCaseTypeIsString.ts --detail --no-dashboard-data
```

Coverage result:

```text
unsupported=1
unsupported_diagcodes=UnsupportedSyntax:1
unsupported_features=unknown-unsupported:1
```

Source context:

```ts
declare function use(a: any): void;

class A {
    doIt(x: Array<string>): void {
        x.forEach((v) => {
            switch(v) {
                case "test": use(this);
            }
        });
    }
}
```

Compiler evidence:

```text
tokens: ok
ast: ok; method body contains Call(callee=Member(Ident("x"), "forEach"), args=[ArrowFn])
resolved/lowered: UnsupportedSyntax issue-211 unknown receiver class for method forEach
TypeScript oracle: ok, no diagnostics; parameter x has type string[]
```

Split result:

- `issues/open/5234-w0-implement-host-deny-and-auditable-e2e-manifest-verification.md`

## Completion evidence

Fill only when moving to `done/`.

Commits:

- filled by closing commit

Validation result:

```text
command: python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/checkSwitchStatementIfCaseTypeIsString.ts --detail --no-dashboard-data
result: pass; reproduced current unsupported bucket
date: 2026-05-06

command: python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/checkSwitchStatementIfCaseTypeIsString.ts
result: pass; reproduced issue-211 forEach receiver boundary and split to issue 5234
date: 2026-05-06
```

Remaining risks:

- Implementing issue 5234 may expose switch narrowing or arrow lexical `this` callback semantics.
