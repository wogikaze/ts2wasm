---
id: 5215
title: "Support loop-local arrow calls from arrow closures"
type: feature
area: ir/lowering
class: implementation-ready
priority: P2
depends_on: [5001]
blocks: []
created: 2026-05-06
updated: 2026-05-06
---

## Summary

Support calls from one loop-local arrow closure to another local arrow binding,
such as `let lambda2 = () => lambda1(len)`.

## Problem

Problem: capturedVarInLoop parses and resolves, but lowering rejects
`lambda1(len)` as a generic issue-211 function-valued local call even though
`lambda1` is initialized to a local arrow function and TypeScript accepts the
file.

Current diagnostic:

```text
UnsupportedSyntax: issue-211: function-valued local calls such as extracted method `lambda1(...)` are not supported
```

## Current failure

Use the validation command below to reproduce the representative failure.

Source shape:

```text
for (var i = 0; i < 10; i++) {
    var str = 'x', len = str.length;
    let lambda1 = (y) => { };
    let lambda2 = () => lambda1(len);
}
```

Observed failure:

```text
capturedVarInLoop.ts: issue-211 function-valued local call at lambda1(len)
```

Compiler evidence:

```text
tokens: ok
ast: ok; lambda1 is an ArrowFn binding and lambda2 body is Call(Ident lambda1, Ident len)
resolved/lowered: issue-211 at lambda1(len)
```

TypeScript oracle reports no diagnostics.

## Desired final state

Calls to a known local arrow binding from another local arrow closure are
classified before the generic issue-211 extracted-method diagnostic.

## Scope

In scope:

- [ ] Support the `lambda2 = () => lambda1(len)` loop-local arrow-call shape.

Out of scope:

- Reassigned callable locals.
- Arbitrary function-valued locals whose initializer is not a known arrow
  function.
- General extracted method calls that issue 211 intentionally keeps
  unsupported.

## Affected paths

Expected:

- `crates/ir/src/`
- `crates/frontend/src/`
- focused compiler fixtures

Do not touch:

- unrelated method receiver behavior
- `crates/runtime-abi/`

## Acceptance criteria

- [ ] `capturedVarInLoop.ts` no longer reports generic issue-211 at
  `lambda1(len)`.
- [ ] A focused fixture covers one local arrow binding calling a sibling local
  arrow binding from inside another arrow body.
- [ ] Existing reassigned callable-local and extracted-method issue-211
  diagnostics remain unchanged.

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run -p ts2wasm-cli -E 'test(function) or test(closure) or test(node_diff)'
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/capturedVarInLoop.ts
```

Impacted commands:

```sh
python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/capturedVarInLoop.ts --detail
```

Not run:

- none

## Completion evidence

Fill when implemented.
