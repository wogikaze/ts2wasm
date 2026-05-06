---
id: 5234
title: "Track array-typed parameters for callback methods"
type: feature
area: frontend/semantics
class: implementation-ready
priority: P1
depends_on: []
blocks: []
created: 2026-05-06
updated: 2026-05-06
---

## Summary

Preserve enough TypeScript array type annotation information on function and
class-method parameters for array callback methods such as `x.forEach(...)` to
use the existing known-array receiver path.

## Problem

`checkSwitchStatementIfCaseTypeIsString.ts` parses the class method, arrow
callback, and switch statement successfully, but lowering rejects
`x.forEach(...)` because `x: Array<string>` is not tracked as a known array
receiver after type erasure.

Problem: class method parameter `x: Array<string>` reports `UnsupportedSyntax: issue-211: unknown receiver class for method forEach`.

## Current failure

Reproduction:

```sh
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/checkSwitchStatementIfCaseTypeIsString.ts
```

Source context:

```ts
declare function use(a: any): void;

class A {
    doIt(x: Array<string>): void {
        x.forEach((v) => {
            switch(v) {
                case "test": use(this);
            }
        });
    }
}
```

Compiler evidence:

```text
tokens: ok
ast: ok; method body contains Call(callee=Member(Ident("x"), "forEach"), args=[ArrowFn])
resolved/lowered: UnsupportedSyntax issue-211 unknown receiver class for method forEach
TypeScript oracle: ok, no diagnostics; parameter x has type string[]
```

## Desired final state

The frontend or resolver records `Array<T>` / `T[]` parameter annotations as
array-shaped locals, so supported array callback methods on those parameters
reach the existing `lower_array_callback_method` path instead of the generic
unknown receiver diagnostic.

## Scope

In scope:

- [ ] Preserve array-shaped parameter annotation metadata for function and class method parameters.
- [ ] Mark parameters annotated as `Array<T>`, `ReadonlyArray<T>`, or `T[]` as known array locals where existing array callback method lowering can use them.
- [ ] Add a focused fixture for `class A { m(x: Array<string>) { x.forEach(v => use(v)); } }`.
- [ ] Re-run the representative reference triage and confirm `x.forEach(...)` no longer reports unknown receiver class.

Out of scope:

- Full TypeScript type checking or generic element type propagation.
- Full `switch` type narrowing.
- New Array callback runtime semantics beyond the methods already supported by `lower_array_callback_method`.
- Non-array interface typed method calls, tracked separately by `issues/open/5222-support-interface-typed-method-calls-on-erased-locals.md`.

## Affected paths

Expected:

- `crates/frontend/src/ast.rs`
- `crates/frontend/src/parser.rs`
- `crates/ir/src/name_resolver.rs`
- `crates/ir/src/lowered/`
- focused parser/IR/CLI tests

Do not touch:

- backend/runtime Array method implementations unless the existing callback method path lacks required support after receiver tracking is fixed

## Acceptance criteria

- [ ] `x: Array<string>` and `x: string[]` parameters are recognized as array-shaped locals for supported callback methods.
- [ ] `x.forEach(v => use(v))` no longer reports `issue-211: unknown receiver class for method forEach` when `x` has an array-shaped parameter annotation.
- [ ] Existing unknown receiver diagnostics remain for untyped locals and non-array receiver annotations.
- [ ] `python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/checkSwitchStatementIfCaseTypeIsString.ts` advances past the current `forEach` receiver boundary.

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run -p ts2wasm-frontend
cargo nextest run -p ts2wasm-ir
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/checkSwitchStatementIfCaseTypeIsString.ts
```

Impacted commands:

```sh
python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/checkSwitchStatementIfCaseTypeIsString.ts --detail --no-dashboard-data
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

Split from generated bucket `issues/done/1144-implement-checkSwitchStatementIfCaseTypeIsString.md`.

Related but broader:

- `issues/open/313-implement-array-builtin.md`
- `issues/open/673-implement-arrayEvery.md`
- `issues/open/677-implement-arrayFlatMap.md`
- `issues/open/5222-support-interface-typed-method-calls-on-erased-locals.md`

## Completion evidence

Fill when implemented.
