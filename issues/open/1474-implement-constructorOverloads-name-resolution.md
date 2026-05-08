---
id: 1474
title: "Implement Constructoroverloads Name Resolution"
type: spike
area: frontend/resolver
class: superseded
priority: P1
depends_on: [5005]
blocks: [5358]
created: 2026-05-01
updated: 2026-05-07
---
> **Reopened by false-done audit** (2026-05-07)
> Classification: must-reopen
> Reason: Batch-migrated to issues/done/ without implementation commits.
> Evidence: Empty completion evidence. No feat/fix commit for #1474.

## Summary

Closed by splitting the current representative blocker to
`issues/done/5358-report-constructor-bodies-in-ambient-class-declarations.md`.

Fresh triage shows this generated `constructorOverloads-name-resolution` bucket
is stale: the representative no longer fails with `name-resolution`. The
current first compiler failure is a method-call fallthrough after ambient class
declarations were erased without reporting the TypeScript TS1183 diagnostic for
a constructor implementation body in a `declare class`.

## Problem

Reference test results originally showed one `name-resolution` case for
`constructorOverloads6.ts`. Current evidence shows:

- tokens: ok
- AST: runtime statements only, including `var f1 = new Foo("hey")` and `f1.bar1()`
- resolved/lowered: `UnsupportedSyntax: method Foo.bar1 not found`
- TypeScript oracle: TS1183 `An implementation cannot be declared in ambient contexts.`

Problem: this bucket needs a focused owner for the ambient-class constructor
body diagnostic, not a broad name-resolution issue.

## Current failure

Representative reproduction:

```sh
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/constructorOverloads6.ts
```

Focused coverage:

```sh
python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/constructorOverloads6.ts --detail --no-dashboard-data
```

Observed 2026-05-07:

```text
unsupported_diagcodes=UnsupportedSyntax:1
unsupported_features=unknown-unsupported:1
Diagnostic: UnsupportedSyntax / parser-or-frontend-unsupported
Feature label: method-call
Message: method `Foo.bar1` not found at 418..427
```

Source context:

```ts
declare class FooBase {
    constructor(s: string);
    constructor(n: number);
    constructor(x: any) {

    }
    bar1():void;
}

declare class Foo extends FooBase {
    constructor(s: string);
    constructor(n: number);
    constructor(x: any, y?:any);

    bar1():void;
}

var f1 = new Foo("hey");
f1.bar1();
```

TypeScript oracle:

```text
TS1183: An implementation cannot be declared in ambient contexts.
```

## Desired final state

This generated bucket is closed. Implementation proceeds through issue 5358,
which owns the exact ambient class constructor body diagnostic.

## Scope

In scope:

- [x] Inspect fresh triage for `constructorOverloads6.ts`
- [x] Confirm broad name-resolution and method-call issues are not exact owners
- [x] Split the ambient class constructor body diagnostic to issue 5358
- [x] Preserve reproduction commands and AST/oracle evidence

Out of scope:

- Direct implementation from this generated bucket
- General method-call support
- Constructor overload signature parsing outside ambient class diagnostics

## Affected paths

Expected:

- `crates/frontend/src/parser/statements_ts.rs`
- `crates/frontend/src/parser/statements_class.rs`
- `crates/frontend/src/parser/tests.rs`
- focused parser/diagnostic tests

Do not touch:

- backend/runtime method dispatch
- broad method-call lowering

## Acceptance criteria

- [x] Duplicate candidates below are confirmed as no-match or this issue is superseded
- [x] Child issue 5358 contains an exact `reference-triage` command
- [x] Child issue includes failing path, diagnostic code, source context, visible symbols, and parser/TypeScript AST evidence
- [x] Child issue acceptance names the exact reference path and diagnostic/stdout change

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
python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/constructorOverloads6.ts --detail --no-dashboard-data
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/constructorOverloads6.ts
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

- [x] created: `issues/done/5358-report-constructor-bodies-in-ambient-class-declarations.md`

## Notes

## Affected test files

- `reference/typescript/tests/cases/compiler/constructorOverloads6.ts`

## Duplicate detection

- `issues/open/435-implement-method-call.md` is a broad method-call bucket, but
  the oracle first diagnostic is the earlier ambient constructor implementation
  diagnostic.
- `issues/done/5261-report-class-typed-missing-instance-method-calls.md`
  handles known class-typed ambient locals whose requested method is missing;
  this representative has a method declared in `Foo` and should not reach that
  missing-method path before TS1183.
- `issues/done/5161-model-ambient-value-declarations-for-name-resolution.md`
  handles `declare var` / `declare let` / `declare const` value visibility,
  not `declare class` constructor bodies.
- `issues/done/5334-parse-class-constructor-overload-signatures.md` handles
  valid bodyless constructor overload signatures followed by an implementation
  in runtime classes; this representative's first oracle diagnostic is a body
  inside an ambient class declaration.

## Smart triage

Generated 2026-05-07.

```text
Path: reference/typescript/tests/cases/compiler/constructorOverloads6.ts
Compiler diagnostic: UnsupportedSyntax method `Foo.bar1` not found at 418..427
Feature label: method-call
TypeScript diagnostic: TS1183 An implementation cannot be declared in ambient contexts.
AST: runtime `new Foo(...)` bindings and `f1.bar1()` call; ambient declarations are erased
Visible symbols: FooBase, Foo, f1, f2, f3, f4
```

## Completion evidence

Commits:

- filled by local commit that moves this issue to `done/`

Validation result:

```text
command: python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/constructorOverloads6.ts --detail --no-dashboard-data
result: pass; current failure is UnsupportedSyntax/method-call, not name-resolution
date: 2026-05-07

command: python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/constructorOverloads6.ts
result: pass; reproduced method `Foo.bar1` not found and captured TS1183 oracle evidence
date: 2026-05-07
```

Remaining risks:

- implementation remains tracked by issue 5358
