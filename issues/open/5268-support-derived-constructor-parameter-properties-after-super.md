---
id: 5268
title: "Support derived constructor parameter properties after super"
type: feature
area: ir
class: implementation-ready
priority: P1
depends_on: []
blocks: []
created: 2026-05-06
updated: 2026-05-06
---

## Summary

Support the remaining TypeScript parameter-property form where a derived class
constructor has a leading `super()` call and parameter properties that must be
initialized after `super()`.

This narrows the current issue-226 boundary from `classUpdateTests.ts`.

## Problem

Basic parameter properties are implemented by issue 226, but the compiler still
rejects derived constructors with parameter properties even when the constructor
body starts with `super()`.

Problem: `reference/typescript/tests/cases/compiler/classUpdateTests.ts` reports
`issue-226: parameter properties in derived constructors require a leading
super(...) call` at `constructor(private p1:number)`.

## Current failure

Reproduction:

```sh
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/classUpdateTests.ts
```

Current diagnostic:

```text
error: [UnsupportedTypeScriptSyntax] issue-226: parameter properties in derived constructors require a leading super(...) call at 906..916
```

Source context:

```ts
class L extends G {
    constructor(private p1:number) {
        super(); // NO ERROR
    }
}
```

Smart triage evidence:

```text
tokens: ok
AST: fails on derived constructor parameter property at 906..916
resolved: same unsupported-feature boundary
TypeScript oracle: parses; reports later type diagnostics for private member compatibility
```

## Desired final state

Derived constructors with supported parameter properties and a leading
`super(...)` call lower by running `super(...)` before writing the generated
instance fields for those parameter properties.

## Scope

In scope:

- [ ] Accept derived constructor parameter properties when the constructor body
      starts with `super(...)`.
- [ ] Lower generated parameter-property field assignments after the leading
      `super(...)` call.
- [ ] Preserve the existing diagnostic for derived constructors where any
      parameter-property initialization would occur before `super(...)`.
- [ ] Add focused coverage for both accepted and rejected derived constructor
      parameter-property forms from `classUpdateTests.ts`.

Out of scope:

- Full TypeScript visibility/type compatibility diagnostics.
- General derived-constructor `this` analysis beyond the parameter-property
  ordering slice.
- Parameter property default-value support already covered by issue 226.

## Affected paths

Expected:

- `crates/frontend/src/`
- `crates/ir/src/`
- `fixtures/`

Do not touch:

- runtime ABI
- package/module resolution

## Acceptance criteria

- [ ] `python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/classUpdateTests.ts` no longer reports `issue-226` for class `L`.
- [ ] A focused regression accepts `class L extends G { constructor(private p1:number) { super(); } }`.
- [ ] A focused regression still rejects a derived parameter-property constructor where other statements precede `super()`.
- [ ] Any next blocker from the same reference path is recorded in this issue or split to a follow-up if outside this scope.

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run -p ts2wasm-frontend
cargo nextest run -p ts2wasm-ir
python scripts/manager.py update-issue-index --check
python scripts/manager.py check-issue-health
python scripts/manager.py check-issue-readiness -- --fail-ready-below 80
git diff --check
```

Impacted commands:

```sh
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/classUpdateTests.ts
python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/classUpdateTests.ts --detail --no-dashboard-data
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

Split from generated bucket `issues/open/1242-implement-classUpdateTests.md`.
Related done issue: `issues/done/226-implement-parameter-properties.md`.

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
