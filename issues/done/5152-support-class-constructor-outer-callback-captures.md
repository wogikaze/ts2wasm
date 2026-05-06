---
id: 5152
title: "Support class constructor outer callback captures"
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

Implement the narrow class-constructor lexical capture slice where a constructor calls an outer callback function and nested arrow callbacks reference constructor `this`.

## Problem

The representative TypeScript reference case parses successfully, but name/builtin resolution stops when the class constructor calls outer local `foo`. The current issue-289 diagnostic says constructor lexical captures require environment support even though TypeScript accepts the source and the callback body only stores constructor `this` into a local.

Problem: class constructors cannot currently call outer callback locals when nested arrow callbacks capture constructor `this`.

## Current failure

Representative reproduction:

```sh
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/badThisBinding.ts
```

Current diagnostic:

```text
error: [UnsupportedSyntax] issue-289: class constructor `constructor` references outer local `foo`; class constructor lexical captures require environment support at 132..135
```

Source context:

```text
class Greeter {
    constructor() {
        foo(() => {
            bar(() => {
                var x = this;
            });
        });
    }
}
```

TypeScript oracle evidence:

```text
TypeScript diagnostics: none.
AST path: ClassDeclaration -> Constructor -> CallExpression `foo(...)`.
```

## Desired final state

The supported class-constructor lowering path can resolve and lower direct calls to outer callback locals from constructors, preserving constructor `this` for nested arrow callbacks in this fixed reference shape.

## Scope

In scope:

- [x] Allow class constructor bodies to reference direct outer callback locals such as `foo` and `bar` in the representative shape.
- [x] Preserve lexical `this` inside nested arrow callbacks under the constructor body.
- [x] Add a focused regression for `class Greeter { constructor() { foo(() => { bar(() => { var x = this; }); }); } }`.
- [x] Re-run the representative triage and record any next blocker separately if it is outside this capture slice.

Out of scope:

- Escaped class lexical environments beyond direct constructor callback calls.
- General class method capture semantics already covered by prior issue-289/301 work.
- Full TypeScript `this` type checking.

## Affected paths

Expected:

- `crates/ir/src/builtin_resolver_outer.rs`
- `crates/ir/src/lowered/resolver_extra.rs`
- `crates/frontend/src/parser/tests.rs`
- `fixtures/`

Do not touch:

- package/module resolution
- unrelated class syntax parsing

## Acceptance criteria

- [x] A focused class-constructor callback capture test no longer emits the issue-289 constructor outer-local diagnostic.
- [x] The nested arrow callback sees the constructor `this` binding instead of creating a separate callback `this`.
- [x] `python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/badThisBinding.ts` no longer reports `class constructor lexical captures require environment support`.
- [x] Any later unsupported runtime/lowering blocker from the same reference path is split separately.

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run -p ts2wasm-ir class_constructor_outer_callback_captures
python scripts/manager.py update-issue-index --check
python scripts/manager.py check-issue-health
python scripts/manager.py check-issue-readiness -- --fail-ready-below 80
git diff --check
```

Impacted commands:

```sh
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/badThisBinding.ts
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/badThisBinding.ts --detail
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

Split from generated bucket `issues/done/1031-implement-badThisBinding.md`.

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


## False-done audit

Date: 2026-05-07

Classification: truly-done.

Audit result: retained in issues/done/. Implementation commits confirmed.
