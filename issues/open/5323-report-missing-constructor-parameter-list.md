---
id: 5323
title: "Report missing constructor parameter list"
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

Report a TypeScript-compatible parser diagnostic for a class `constructor`
member without a parameter list or body.

## Problem

Problem: `classFieldsBrokenConstructorEmitNoCrash1.ts` currently reports a
generic parser error, `expected LeftParen, got Some(RightBrace)`, when a class
body ends with a bare `constructor` member.

## Current failure

```sh
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/classFieldsBrokenConstructorEmitNoCrash1.ts
```

Observed 2026-05-07:

```text
UnsupportedSyntax: expected LeftParen, got Some(RightBrace) at 112..113
TypeScript oracle:
TS2390 Constructor implementation is missing.
TS1005 '(' expected.
```

Representative source:

```ts
class Test {
  prop = 42;
  constructor
}
```

## Desired final state

The class member parser recognizes a bare `constructor` at class-body end and
emits a source-spanned diagnostic that names the missing constructor parameter
list/body instead of a generic `expected LeftParen` parser failure.

## Scope

In scope:

- [ ] Detect `constructor }` while parsing class members.
- [ ] Emit a source-spanned diagnostic at `constructor` or the following `}`.
- [ ] Add focused parser coverage for `class C { field = 1; constructor }`.

Out of scope:

- Full TypeScript parser recovery for every malformed constructor.
- Valid constructor overload declarations such as `constructor();`, tracked by
  broader class declaration triage buckets.
- Runtime class field initialization semantics.

## Affected paths

Expected:

- `crates/frontend/src/parser/statements_class.rs`
- `crates/frontend/src/diagnostic.rs`
- focused frontend parser tests or CLI fixtures

Do not touch:

- backend/runtime lowering
- unrelated class member syntax support

## Acceptance criteria

- [ ] `classFieldsBrokenConstructorEmitNoCrash1.ts` no longer reports generic
  `expected LeftParen, got Some(RightBrace)`.
- [ ] The new diagnostic names the missing constructor parameter list/body.
- [ ] Valid `constructor() {}` parsing remains unchanged.

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run -p ts2wasm-frontend -E 'test(class) or test(constructor) or test(parser)'
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/classFieldsBrokenConstructorEmitNoCrash1.ts
```

Impacted commands:

```sh
python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/classFieldsBrokenConstructorEmitNoCrash1.ts --detail --no-dashboard-data
python scripts/manager.py check-issue-readiness -- --fail-ready-below 80
```

Not run:

- none

## Docs / current-state / issue sync

Final-state docs:

- [ ] not affected

Current state:

- [ ] not affected

Follow-up issues:

- [ ] none

## Notes

Split from stale generated bucket
`issues/done/1211-implement-classFieldsBrokenConstructorEmitNoCrash.md`.

Related but not duplicate:

- `issues/done/546-implement-ClassDeclaration.md` is a broad class declaration
  triage bucket containing constructor overload declarations; this issue is the
  narrow malformed bare-constructor case.

## Completion evidence

Fill only when moving to `done/`.

Commits:

- `...`

Validation result:

```text
command:
result:
date:
```

Remaining risks:

- none
