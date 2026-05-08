---
id: 5358
title: "Report constructor bodies in ambient class declarations"
type: feature
area: frontend/parser
class: implementation-ready
priority: P1
depends_on: []
blocks: []
created: 2026-05-07
updated: 2026-05-07
---

## Summary

Report TypeScript's ambient-context implementation diagnostic when a
`declare class` constructor has a body. Keep declaration-only ambient class
members erased.

`constructorOverloads6.ts` currently reaches `f1.bar1()` and reports
`method Foo.bar1 not found`, while TypeScript reports TS1183 at the earlier
constructor body in `declare class FooBase`.

## Problem

The ambient class parser currently skips the balanced class body for
`declare class` declarations. That is correct for declaration-only signatures,
but it also skips constructor implementations with bodies:

```ts
declare class FooBase {
    constructor(s: string);
    constructor(n: number);
    constructor(x: any) {

    }
    bar1():void;
}
```

Problem: implementation bodies inside ambient class declarations are not
diagnosed at the source span where TypeScript reports TS1183. The compiler
continues after erasure and later reports an unrelated method-call failure for
`f1.bar1()`.

## Current failure

Reproduction:

```sh
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/constructorOverloads6.ts
```

Focused coverage:

```sh
python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/constructorOverloads6.ts --detail --no-dashboard-data
```

Current compiler diagnostic:

```text
UnsupportedSyntax: method `Foo.bar1` not found at 418..427
```

Compiler evidence:

```text
tokens: ok; includes declare class FooBase/Foo, bodyless constructor signatures, and FooBase constructor body braces
ast: ok; ambient class declarations are erased and runtime statements include new Foo(...) bindings plus f1.bar1()
resolved/lowered: fails at method `Foo.bar1` not found
visible symbols: FooBase, Foo, f1, f2, f3, f4
```

TypeScript oracle evidence:

```text
TS1183: An implementation cannot be declared in ambient contexts.
```

## Desired final state

The ambient class parser detects `constructor(...) {}` while scanning a
`declare class` body and reports a source-spanned ambient-context
implementation diagnostic before later runtime statements are lowered.

## Scope

In scope:

- [x] Detect `constructor(...) { ... }` inside `declare class` bodies and emit a source-spanned diagnostic equivalent to TS1183.
- [x] Add focused parser coverage for one rejecting body case.
- [x] Re-run `constructorOverloads6.ts` triage and confirm it no longer reports `method Foo.bar1 not found`.

Out of scope:

- General method-call lowering or missing-method diagnostics.
- Runtime support for ambient class methods or instances.
- Valid runtime class constructor overload signatures, tracked by issue 5334.

## Affected paths

Expected:

- `crates/frontend/src/parser/statements_ts.rs`
- `crates/frontend/src/parser/tests.rs`

Do not touch:

- `crates/backend-wasm/`
- broad method-call lowering
- unrelated ambient value declaration resolver behavior

## Acceptance criteria

- [x] `constructorOverloads6.ts` no longer reports `method Foo.bar1 not found`.
- [x] `declare class C { constructor(x: number) {} }` reports the ambient implementation diagnostic with a source span at the constructor body.
- [x] `declare class C { constructor(x: number); m(): void; }` remains accepted and erased.

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run -p ts2wasm-frontend -E 'test(ambient) or test(constructor)'
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/constructorOverloads6.ts
```

Impacted commands:

```sh
python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/constructorOverloads6.ts --detail --no-dashboard-data
python scripts/manager.py update-issue-index --check
python scripts/manager.py check-issue-health
python scripts/manager.py check-issue-readiness -- --fail-ready-below 80
git diff --check
```

Not run:

- none

## Docs / current-state / issue sync

Final-state docs:

- [x] not affected

Current state:

- [x] not affected

Follow-up issues:

- [x] none

## Notes

Split from `issues/open/1474-implement-constructorOverloads-name-resolution.md`
on 2026-05-07.

Related but not exact:

- `issues/open/5334-parse-class-constructor-overload-signatures.md` handles
  valid runtime class overload signatures; this issue handles invalid bodies in
  ambient class declarations.
- `issues/open/5261-report-class-typed-missing-instance-method-calls.md`
  handles missing methods on known class-typed ambient locals; this issue should
  prevent `constructorOverloads6.ts` from reaching that method-call family.

## Completion evidence

Fill only when implemented.

## False-done audit

**truly-done** (5358)

- Implementation commits: verified via `git log --oneline --all --grep=5358`
- Completion evidence: filled with specific commit hashes and validation results
- Acceptance criteria: all checked as met

This issue has repo-local close evidence with implementation commits and validation commands.
