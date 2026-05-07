---
id: 5266
title: "Support class constructor new of later class binding"
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

Allow class constructor bodies to instantiate later class declarations in the
same lexical/module scope, covering the `classOrderBug.ts` reference shape.

This is a narrow issue-289 constructor lexical-capture slice, separate from the
callback-local constructor capture issue 5152.

## Problem

`classOrderBug.ts` parses and builds an AST, but name resolution rejects
`new foo()` inside `bar`'s constructor because constructor bodies cannot
currently reference outer/later class bindings through the class environment.

Problem: `reference/typescript/tests/cases/compiler/classOrderBug.ts` reports
`issue-289: class constructor constructor references outer local foo`.

## Current failure

Reproduction:

```sh
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/classOrderBug.ts
```

Current diagnostic:

```text
error: [UnsupportedSyntax] issue-289: class constructor `constructor` references outer local `foo`; class constructor lexical captures require environment support at 101..104
```

Source context:

```ts
class bar {
    public baz: foo;
    constructor() {
        this.baz = new foo();
    }
}

class baz {}
class foo extends baz {}
```

Smart triage evidence:

```text
tokens: ok
AST: ok; ClassDecl bar, ClassDecl baz, ClassDecl foo extends baz
resolved: fails in resolve_names on constructor `new foo()`
TypeScript oracle: ok, diagnostics=[]
```

## Desired final state

Constructor bodies can resolve and lower supported `new <ClassName>()` uses of
class declarations that appear later in the same module/reference file, without
emitting the issue-289 constructor lexical-capture diagnostic for this shape.

## Scope

In scope:

- [ ] Resolve constructor-body `new foo()` when `foo` is a class declaration in
      the same module scope, including later declarations.
- [ ] Preserve existing TDZ/runtime behavior decisions for unsupported
      expression-valued class uses; do not broaden to arbitrary class values.
- [ ] Add or update a focused regression for the `classOrderBug.ts` shape.
- [ ] Re-run the representative triage and split any later runtime/semantic
      blocker separately.

Out of scope:

- General first-class class constructor values; tracked by issue 5192.
- Constructor callback-local captures and nested arrow `this`; tracked by issue
  5152.
- General class method mutable capture environments.

## Affected paths

Expected:

- `crates/ir/src/`
- `crates/cli/src/`
- `fixtures/`

Do not touch:

- package/module resolution
- unrelated class parser behavior

## Acceptance criteria

- [ ] `python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/classOrderBug.ts` no longer reports `issue-289` for `new foo()`.
- [ ] A focused fixture/regression proves `class bar { constructor() { this.baz = new foo(); } } class baz {} class foo extends baz {}` resolves past constructor lexical capture.
- [ ] Existing diagnostics remain for constructor references to unsupported non-class outer locals.
- [ ] Any next blocker from the same reference path is recorded in this issue or split to a follow-up if outside this scope.

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run -p ts2wasm-ir
python scripts/manager.py update-issue-index --check
python scripts/manager.py check-issue-health
python scripts/manager.py check-issue-readiness -- --fail-ready-below 80
git diff --check
```

Impacted commands:

```sh
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/classOrderBug.ts
python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/classOrderBug.ts --detail --no-dashboard-data
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

Split from generated bucket `issues/done/1231-implement-classOrderBug.md`.
Related but distinct open issue: `issues/done/5152-support-class-constructor-outer-callback-captures.md`.

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
