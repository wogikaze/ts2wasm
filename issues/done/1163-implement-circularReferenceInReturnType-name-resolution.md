---
id: 1163
title: "Implement Circularreferenceinreturntype Name Resolution"
type: spike
area: frontend/resolver
class: done
priority: P1
depends_on: [5163]
blocks: []
created: 2026-05-01
updated: 2026-05-06
completed: 2026-05-06
status: done
---

## Summary

Triage circularReferenceInReturnType-name-resolution across 1 failing reference test cases and split this bucket into implementation-ready child issues.

## Problem

Reference test results show 1 case failing in directory `circularReferenceInReturnType-name-resolution`. Fresh triage no longer matches the generated name-resolution label: tokens and AST succeed, and the current build blocker is the lowerer rejecting a nested call expression whose callee is another call.

Problem: `circularReferenceInReturnType.ts` is not a standalone name-resolution implementation order in the current runner view. The current first blocker is `UnsupportedSyntax: only identifier calls are supported in expression context` for `fn2()(() => res2)`, covered by issue 5163.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/circularReferenceInReturnType.ts
```

Coverage window:

```sh
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/circularReferenceInReturnType.ts --detail
```

## Desired final state

This generated bucket is closed as superseded by `issues/open/5163-lower-nested-call-expression-callees.md` for the current nested-call callee lowering blocker. Do not implement directly from this bucket.

## Scope

In scope:

- [x] Inspect the smart triage report below
- [x] Confirm whether existing open/done issues already cover this bucket
- [x] Supersede this bucket with issue 5163's nested call expression callee work
- [x] Preserve exact reproduction commands and representative AST/diagnostic evidence in this closed issue

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
- [x] This closed issue contains an exact `python scripts/manager.py reference-triage ...` command
- [x] This closed issue includes failing path, diagnostic code, source context, visible symbols, and parser/TypeScript AST evidence
- [x] Completion evidence names the exact fixture/reference path and diagnostic/stdout change

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
python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/circularReferenceInReturnType.ts --detail --no-dashboard-data
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/circularReferenceInReturnType.ts
```

Not run:

- `cargo fmt --all --check`; issue cleanup only, no Rust code changed
- `cargo nextest run`; issue cleanup only, no implementation changed

## Docs / current-state / issue sync

Final-state docs:

- [x] not affected

Current state:

- [x] not affected

Follow-up issues:

- [x] folded into `issues/open/5163-lower-nested-call-expression-callees.md`

## Notes

## Affected test files

- `reference/typescript/tests/cases/compiler/circularReferenceInReturnType.ts`

## Duplicate detection

- `issues/open/5163-lower-nested-call-expression-callees.md` owns the current `only identifier calls are supported in expression context` lowering diagnostic for nested call callees.
- Name-resolution umbrella issues are not exact matches for the current first blocker because parser/AST succeed and the pipeline stops during lowering.
- Broad call-expression issue 420 is a parent bucket, not the focused implementation-ready owner.

## Smart triage

Fresh triage shows this generated name-resolution bucket is currently blocked
by nested call-expression lowering.

### Smart triage: circularReferenceInReturnType

- Issue class: `triage-needed`
- Feature label: `call-expression`
- Diagnostic: `UnsupportedSyntax` / `parser-or-frontend-unsupported`
- Current compiler message: `only identifier calls are supported in expression context at 267..284`
- Path: `reference/typescript/tests/cases/compiler/circularReferenceInReturnType.ts`

Reproduction:

```sh
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/circularReferenceInReturnType.ts
```

Coverage:

```sh
python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/circularReferenceInReturnType.ts --detail --no-dashboard-data
```

Coverage result:

```text
unsupported=1
unsupported_diagcodes=UnsupportedSyntax:1
unsupported_features=unknown-unsupported:1
```

Source context:

```ts
declare function fn1<T>(cb: () => T): string;
const res1 = fn1(() => res1);

declare function fn2<T>(): (cb: () => any) => (a: T) => void;
const res2 = fn2()(() => res2);

declare function fn3<T>(): <T2>(cb: (arg: T2) => any) => (a: T) => void;
const res3 = fn3()(() => res3);
```

Compiler evidence:

```text
tokens: ok
ast: ok; res2 initializer is Call { callee: Call { callee: Ident("fn2"), args: [] }, args: [ArrowFn(...)] }
resolved/lowered: UnsupportedSyntax: only identifier calls are supported in expression context at 267..284
```

TypeScript oracle evidence:

```text
typescript ok: true
diagnostics: []
binding res2 type: (a: unknown) => void
binding res3 type: (a: unknown) => void
```

Resolution:

```text
Issue 5163 already owns lowering or explicitly diagnosing call expressions whose callee is another expression. This bucket's generated name-resolution label is stale for the current first blocker.
```

## Completion evidence

Fill only when moving to `done/`.

The `circularReferenceInReturnType` name-resolution bucket is complete. The current failure is superseded by issue 5163's nested call-expression callee work.

Commits:

- superseded by `issues/open/5163-lower-nested-call-expression-callees.md`

Validation result:

```text
command: python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/circularReferenceInReturnType.ts --detail --no-dashboard-data
result: pass on the main checkout; 1 executed, current failure is UnsupportedSyntax/unknown-unsupported
date: 2026-05-06

command: python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/circularReferenceInReturnType.ts
result: pass; lower_program reports `only identifier calls are supported in expression context` for nested call `fn2()(() => res2)`
date: 2026-05-06
```

Remaining risks:

- Implementing issue 5163 may expose later circular return-type or `Parameters<typeof bar>[0]` semantic blockers in this reference case.
