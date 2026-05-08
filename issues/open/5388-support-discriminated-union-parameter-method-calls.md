---
id: 5388
title: "Support discriminated union parameter method calls"
type: feature
area: ir/lowering
class: implementation-ready
priority: P1
depends_on: []
blocks: []
created: 2026-05-07
updated: 2026-05-07
---

## Summary

Support or precisely diagnose method calls on function parameters whose type is
a discriminated union of object shapes, such as `item.method("")` after
checking `item.kind === "a"`.

## Problem

`contextuallyTypedByDiscriminableUnion.ts` tokenizes and parses successfully,
including the `ADT` discriminated union alias, the `invoke(item: ADT)`
parameter, the `item.kind === "a"` branch, and the `item.method(...)` calls.
Lowering then rejects the first method call because `item` has no known
runtime receiver class even though the TypeScript type annotation contains the
callable member shapes.

Current diagnostic:

```text
UnsupportedSyntax: issue-211: unknown receiver class for method `method` at 238..253
```

## Current failure

Use the validation command below to reproduce the representative failure.

Source shape:

```ts
type ADT = { kind: "a", method: (s: string) => number } |
           { kind: "b", method: (n: number) => string };

function invoke(item: ADT) {
    if (item.kind === "a") {
        item.method("");
    }
    else {
        item.method(42);
    }
}
```

Compiler evidence:

```text
tokens: ok
ast: ok; includes discriminated union type alias, invoke(item: ADT), strict
     equality narrowing shape item.kind === "a", and item.method calls
resolved/lowered: issue-211 unknown receiver class for method `method`
visible symbols: function invoke, params item: ADT
TypeScript AST: PropertyAccessExpression item.method at line 14
TypeScript oracle: ok, diagnostics []
```

## Desired final state

The compiler no longer reports the generic unknown receiver class diagnostic
for method calls on discriminated union parameter receivers. The representative
path either supports the method-call shape enough to expose the next semantic
blocker or reports a source-spanned diagnostic that names the unsupported
discriminated union method receiver.

## Scope

In scope:

- [ ] Preserve or recover enough object-union method metadata for parameters
  annotated as discriminated union aliases.
- [ ] Classify `item.method(...)` before the generic issue-211 unknown receiver
  class diagnostic.
- [ ] Keep branch narrowing shape `item.kind === "a"` visible enough that the
  method receiver diagnostic or lowering decision is tied to the annotated
  union, not to an unknown runtime class.

Out of scope:

- Full TypeScript control-flow narrowing.
- Runtime implementation of arbitrary erased object-union values.
- Interface-typed erased locals, covered by
  `issues/open/5222a-parse-ambient-generic-variable-type-annotations.md`.
- Broad method-call support outside this discriminated union parameter shape.

## Affected paths

Expected:

- `crates/ir/src/`
- `crates/frontend/src/`
- `crates/cli/tests/`
- focused fixtures

Do not touch:

- `crates/backend-wasm/` unless lowering already produces a supported runtime
  representation
- unrelated array, string, number, or class method builtins

## Acceptance criteria

- [ ] `contextuallyTypedByDiscriminableUnion.ts` no longer reports
  `unknown receiver class for method method` at `item.method("")`.
- [ ] A focused fixture covers a parameter typed as a discriminated union alias:
  `function invoke(item: ADT) { if (item.kind === "a") item.method(""); }`.
- [ ] The object literal invocation shapes later in the reference file still
  parse and continue to the same or later compilation phase.
- [ ] Existing broad unsupported method-call diagnostics remain unchanged for
  receivers without annotated object/interface metadata.

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run -p ts2wasm-cli -E 'test(method) or test(union) or test(discrimin)'
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/contextuallyTypedByDiscriminableUnion.ts
```

Impacted commands:

```sh
python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/contextuallyTypedByDiscriminableUnion.ts --detail --no-dashboard-data
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

Split from generated bucket
`issues/open/1539-implement-contextuallyTypedByDiscriminableUnion-parser-syntax.md`.

Related but distinct:

- `issues/open/5222a-parse-ambient-generic-variable-type-annotations.md`
  owns method calls on interface-typed erased locals such as
  `var s: Sequence<string>; s.groupBy(...)`.
- `issues/open/435-implement-method-call.md` is a broad method-call bucket and
  does not provide fixed acceptance for this reference path.

## Completion evidence

Fill when implemented.
