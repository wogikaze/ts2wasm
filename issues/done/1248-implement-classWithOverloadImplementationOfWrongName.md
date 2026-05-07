---
id: 1248
title: "Implement Classwithoverloadimplementationofwrongname"
type: spike
area: frontend/syntax
class: done
priority: P1
depends_on: []
blocks: []
created: 2026-05-01
updated: 2026-05-07
completed: 2026-05-07
status: done
---

## Summary

Triage `classWithOverloadImplementationOfWrongName` across 1 failing reference
test case and split the current duplicate-function blocker into
implementation-ready child issue 5327.

## Problem

Reference test results previously showed 1 case failing in directory
`classWithOverloadImplementationOfWrongName` with diagnostics: parser-syntax.
Fresh triage shows tokens and AST now succeed; the current blocker is the
generic `DuplicateFunction` diagnostic for class method overload declarations.

Problem: `classWithOverloadImplementationOfWrongName2.ts` reports
`DuplicateFunction: duplicate method definition: C.foo` instead of
TypeScript-compatible diagnostics for a wrong overload implementation name and a
missing immediately following implementation. This is now tracked by issue 5327.

## Current failure

Representative reproduction:

```sh
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/classWithOverloadImplementationOfWrongName2.ts
```

Coverage window:

```sh
python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/classWithOverloadImplementationOfWrongName2.ts --detail --no-dashboard-data
```

## Desired final state

This generated bucket is either split into implementation-ready child issues or superseded by an existing open/done issue with matching evidence. Do not implement directly from this bucket.

## Scope

In scope:

- [x] Inspect the smart triage report below
- [x] Confirm existing issue 5198 covers valid class method overload merging but not this invalid wrong-name implementation shape
- [x] Split the observable behavior into child issue 5327
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

- [x] Duplicate candidates below are confirmed as no-match for the exact invalid overload behavior
- [x] Child issue 5327 contains an exact `python scripts/manager.py reference-triage ...` command
- [x] Child issue includes failing path, diagnostic code, source context, visible symbols, and parser/TypeScript AST evidence
- [x] Child issue acceptance names the exact reference path and diagnostic/stdout change

## Validation

Required commands:

```sh
python scripts/manager.py update-issue-index --check
python scripts/manager.py check-issue-health
python scripts/manager.py check-issue-readiness -- --fail-ready-below 80
git diff --check
```

Impacted commands:

```sh
mise run reference-coverage -- tsc --limit 2
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/classWithOverloadImplementationOfWrongName2.ts --detail
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/classWithOverloadImplementationOfWrongName2.ts
```

Not run:

- `cargo fmt --all --check`; issue split/cleanup only, no Rust code changed
- `cargo nextest run`; issue split/cleanup only, no implementation changed

## Docs / current-state / issue sync

Final-state docs:

- [x] not affected

Current state:

- [x] not affected

Follow-up issues:

- [x] created: `issues/open/5327-report-class-method-overload-wrong-implementation-name.md`

## Notes

## Affected test files

- `reference/typescript/tests/cases/compiler/classWithOverloadImplementationOfWrongName2.ts`

## Duplicate detection

- `issues/done/5198-support-class-method-overload-signatures-for-element-access-calls.md` is related but handles valid class method overload signature merging with a same-name implementation.
- `issues/done/5200-validate-top-level-function-overload-implementations.md` is related but only handles top-level function overload implementation grouping.
- `issues/open/2043-implement-duplicateIdentifierRelatedSpans-duplicate-function.md`, `issues/open/2600-implement-getAndSetNotIdenticalType-duplicate-function.md`, and `issues/open/4258-implement-staticVisibility-duplicate-function.md` are no-match generated duplicate-function buckets for other reference paths.
- No existing open/done issue owned the invalid class method overload implementation-name/order diagnostics, so this bucket was split to issue 5327.

## Smart triage

### Smart triage: classWithOverloadImplementationOfWrongName2

- Issue class: `triage-needed`
- Feature label: `duplicate-function`
- Diagnostic: `DuplicateFunction` / `compiler-diagnostic`
- Current compiler message: `duplicate method definition: C.foo`
- Path: `reference/typescript/tests/cases/compiler/classWithOverloadImplementationOfWrongName2.ts`

Reproduction:

```sh
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/classWithOverloadImplementationOfWrongName2.ts
```

Coverage:

```sh
python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/classWithOverloadImplementationOfWrongName2.ts --detail --no-dashboard-data
```

Coverage result:

```text
executed=1
unsupported=1
unsupported_diagcodes=DuplicateFunction:1
unsupported_features=duplicate-function:1
```

Source context:

```ts
class C {
    foo(): string;
    bar(x): any { }
    foo(x): number;
}
```

Compiler evidence:

```text
tokens: ok
ast: ok; ClassDecl C contains bodyless method `foo`, concrete method `bar`, and bodyless method `foo`
resolved/lowering: DuplicateFunction duplicate method definition: `C.foo`
```

TypeScript oracle evidence:

```text
TS2389: Function implementation name must be 'foo' at `bar`.
TS2391: Function implementation is missing or not immediately following the declaration at the later bodyless `foo`.
```

Split child:

- `issues/open/5327-report-class-method-overload-wrong-implementation-name.md`

## Completion evidence

Fill only when moving to `done/`.

Commits:

- Split to issue 5327; see local commit for this issue cleanup.

Validation result:

```text
command: python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/classWithOverloadImplementationOfWrongName2.ts
result: pass on the main checkout with TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm; reproduced DuplicateFunction for class method overload wrong implementation name/order and split child issue 5327
date: 2026-05-07

command: python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/classWithOverloadImplementationOfWrongName2.ts --detail --no-dashboard-data
result: pass on the main checkout with TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm; executed=1, unsupported=1, unsupported_features=duplicate-function
date: 2026-05-07
```

Remaining risks:

- none; remaining implementation work is tracked by issue 5327
