---
id: 1156
title: "Implement Circularinlinemappedgenerictupletypenocrash"
type: spike
area: frontend/semantics
class: done
priority: P1
depends_on: [5241]
blocks: []
created: 2026-05-01
updated: 2026-05-06
completed: 2026-05-06
---

## Summary

Triage circularInlineMappedGenericTupleTypeNoCrash across 1 failing reference test cases and split this bucket into implementation-ready child issues.

## Problem

Reference test results show 1 case failing in directory `circularInlineMappedGenericTupleTypeNoCrash` with diagnostics: type-system. Fresh triage shows the current blocker is earlier than type semantics: the parser rejects a spread argument inside a `new` expression.

Problem: `circularInlineMappedGenericTupleTypeNoCrash.ts` currently fails with `unsupported expression: ... DotDotDot` at `new Foo<T>(...this.elements, ...)`, before AST construction reaches mapped tuple or type-instantiation behavior.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/circularInlineMappedGenericTupleTypeNoCrash.ts
```

Coverage window:

```sh
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/circularInlineMappedGenericTupleTypeNoCrash.ts --detail
```

## Desired final state

This generated bucket is closed after splitting the current parser blocker into implementation-ready child issue 5241. Do not implement directly from this bucket.

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

- [x] Duplicate candidates below are confirmed as no-match for an implementation-ready owner
- [x] Child issue 5241 contains an exact `python scripts/manager.py reference-triage ...` command
- [x] Child issue 5241 includes failing path, diagnostic code, source context, visible symbols, and parser/TypeScript AST evidence
- [x] Child issue 5241 acceptance names the exact fixture/reference path and diagnostic/stdout change

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
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/circularInlineMappedGenericTupleTypeNoCrash.ts --detail
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/circularInlineMappedGenericTupleTypeNoCrash.ts
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

- [x] created: `issues/done/5241-w2-number-model-sentinels.md`

## Notes

## Affected test files

- `reference/typescript/tests/cases/compiler/circularInlineMappedGenericTupleTypeNoCrash.ts`

## Duplicate detection

Fresh duplicate scan found broad spread issues but no exact implementation-ready
owner for constructor-call spread parsing:

- `issues/open/274-implement-spread-operator.md` is the broad spread meta issue.
- `issues/done/353-spread-iterator-protocol.md` owns general iterator protocol semantics.
- `issues/done/039-implement-spread-arguments.md` completed ordinary call spread syntax.

Issue 5241 is narrower: parse spread arguments in `new` expressions so the
representative case advances past the raw parser boundary.

## Smart triage

### Smart triage: type-system label, parser boundary

- Issue class: `triage-needed`
- Feature label: `type-system`
- Diagnostic: `UnsupportedSyntax` / `parser-or-frontend-unsupported`
- Path: `reference/typescript/tests/cases/compiler/circularInlineMappedGenericTupleTypeNoCrash.ts`

Reproduction:

```sh
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/circularInlineMappedGenericTupleTypeNoCrash.ts
```

Coverage reproduction:

```sh
python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/circularInlineMappedGenericTupleTypeNoCrash.ts --detail --no-dashboard-data
```

Source context:

```ts
class Foo<Elements extends readonly unknown[]> {
  public readonly elements: { [P in keyof Elements]: { bar: Elements[P] } };

  public constructor(
    ...elements: { [P in keyof Elements]: { bar: Elements[P] } }
  ) {
    this.elements = elements;
  }

  public add(): Foo<[...Elements, "abc"]> {
    return new Foo<[...Elements, "abc"]>(...this.elements, { bar: "abc" });
  }
}
```

Coverage result:

```text
unsupported=1
unsupported_diagcodes=UnsupportedSyntax:1
unsupported_features=type-system:1
```

Compiler evidence:

```text
tokens: ok; includes New, Foo, type arguments, LeftParen, DotDotDot, This, Dot, elements
ast: fails before construction with unsupported expression DotDotDot at 397..401
resolved: same parser failure
visible symbols before failure: class Foo
```

TypeScript oracle evidence:

```text
TypeScript reports TS2589 at the new expression, proving it parses through the constructor spread and reaches type instantiation.
Oracle AST path reaches NewExpression -> SpreadElement -> PropertyAccessExpression this.elements.
```

## Completion evidence

Fill only when moving to `done/`.

Commits:

- filled by closing commit

Validation result:

```text
command: python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/circularInlineMappedGenericTupleTypeNoCrash.ts --detail --no-dashboard-data
result: fail on the main checkout; unsupported=1, unsupported_diagcodes=UnsupportedSyntax:1, unsupported_features=type-system:1
date: 2026-05-06

command: python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/circularInlineMappedGenericTupleTypeNoCrash.ts
result: fail; current blocker is raw DotDotDot parser failure in a new-expression argument list, split into issue 5241
date: 2026-05-06
```

Remaining risks:

- Fixing issue 5241 may expose later constructor-spread runtime, iterator-protocol, or mapped tuple type semantics.
