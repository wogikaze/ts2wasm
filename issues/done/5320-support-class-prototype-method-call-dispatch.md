---
id: 5320
title: "Support class prototype method call dispatch"
type: feature
area: ir/runtime
class: implementation-ready
priority: P1
depends_on: []
blocks: []
created: 2026-05-07
updated: 2026-05-07
---

## Summary

Support direct `Class.prototype.method.call(receiver)` dispatch for class
prototype methods.

## Problem

Problem: `classFieldSuperAccessibleJs2.ts` parses and lowers through ordinary
class inheritance and `super.foo()`, then stops at the final
`D.prototype.foo.call(obj)` with `issue-211`.

## Current failure

```sh
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/classFieldSuperAccessibleJs2.ts
```

Observed 2026-05-07:

```text
lower_program: UnsupportedSyntax issue-211: D.prototype.foo.call is not supported at 399..424
TypeScript oracle: ok, diagnostics=[]
```

Representative source:

```ts
const obj = new D();
obj.foo();
D.prototype.foo.call(obj);
```

## Desired final state

The lowering/runtime path recognizes `D.prototype.foo.call(obj)` as an explicit
receiver-bound call to class method `D.prototype.foo` with `this = obj`.

## Scope

In scope:

- [x] Detect direct `ClassName.prototype.method.call(receiver)` calls for known
  class declarations.
- [x] Dispatch the prototype method with the explicit receiver as `this`.
- [x] Add focused coverage for `D.prototype.foo.call(obj)`.

Out of scope:

- Arbitrary extracted method calls or dynamic `.call` targets.
- Generic `Function.prototype.call.bind(...)` patterns.
- Broad method-call issue-211 cleanup outside class prototype methods.

## Affected paths

Expected:

- `crates/ir/src/lowered/resolver_expr.rs`
- `crates/ir/src/`
- `crates/backend-wasm/src/`
- focused CLI/IR tests or fixtures

Do not touch:

- parser code unless the existing AST evidence changes
- unrelated builtins or array prototype `.call` behavior

## Acceptance criteria

- [x] `classFieldSuperAccessibleJs2.ts` no longer reports
  `issue-211: D.prototype.foo.call is not supported`.
- [x] A focused fixture covers `D.prototype.foo.call(obj)` and verifies receiver
  binding.
- [x] Existing unsupported dynamic/function-valued issue-211 diagnostics remain
  source-spanned.

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run -p ts2wasm-ir -p ts2wasm-cli -E 'test(class) or test(method) or test(receiver)'
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/classFieldSuperAccessibleJs2.ts
```

Impacted commands:

```sh
python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/classFieldSuperAccessibleJs2.ts --detail --no-dashboard-data
python scripts/manager.py check-issue-readiness -- --fail-ready-below 80
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

Split from stale generated bucket
`issues/open/1208-implement-classFieldSuperAccessibleJs.md`.

Related but not duplicate:

- `issues/open/435-implement-method-call.md` is a broad method-call bucket; this
  issue is the narrow class-prototype `.call` shape emitted by issue-211.

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
