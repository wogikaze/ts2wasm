---
id: 5476
title: "Lower integer number bitwise-not"
type: feature
area: ir/lowered
class: implementation-ready
priority: P1
depends_on: []
blocks: []
created: 2026-05-08
updated: 2026-05-08
---

## Summary

Lower the first ordinary Number unary bitwise-not slice for integer operands,
so `~0`, `~1`, and `~-1` no longer stop with the generic
`UnaryOp::BitwiseNot` unsupported diagnostic.

## Problem

The frontend already tokenizes `~` as `Token::Tilde` and parses it as
`UnaryOp::BitwiseNot`. The representative TSC case reaches `lower_program`,
then fails because `lower_unary_op` rejects `UnaryOp::BitwiseNot`:

```text
UnsupportedSyntax: unary operator BitwiseNot not yet supported
```

Problem: integer Number `~expr` has parser and AST support but no lowering path.

## Current failure

Representative reproduction:

```sh
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/noImplicitAnyLoopCrash.ts
```

Focused coverage:

```sh
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/noImplicitAnyLoopCrash.ts --detail --no-dashboard-data
```

Current diagnostic:

```text
error: [UnsupportedSyntax] unary operator BitwiseNot not yet supported
```

Source context:

```ts
let foo = () => {};
let bar;
while (1) {
    bar = ~foo(...bar);
}
```

Relevant compiler evidence:

```text
tokens: ok; includes Tilde at 104..105
AST: While body contains Assign { name: "bar", expr: Unary { op: BitwiseNot, expr: Call(foo, [Spread(bar)]) } }
Pipeline: validate_ast -> module_graph -> resolve_names -> resolve_builtins -> build_typed_ir -> lower_program
Failure: lower_program reports `unary operator BitwiseNot not yet supported`
TypeScript oracle: reports TS2556 on the spread argument after accepting the `~` syntax
```

## Desired final state

The compiler lowers integer Number `~expr` through JavaScript 32-bit bitwise
semantics for the focused integer slice.

## Scope

In scope:

- [ ] Add the narrow lowering path needed for integer Number `~`.
- [ ] Add one focused Node/iwasm fixture for `~0`, `~1`, and `~-1`.
- [ ] Re-run `noImplicitAnyLoopCrash.ts` triage and confirm the BitwiseNot diagnostic is gone.

Out of scope:

- BigInt `~`, already tracked and implemented separately by BigInt bitwise issues.
- Number coercion cases outside integer literals/known integer values.
- General dynamic call spread or iterator protocol behavior in `foo(...bar)`.
- Other unary operators.

## Affected paths

Expected:

- `crates/ir/src/lowered/`
- `crates/backend-wasm/src/`
- `crates/cli/tests/`
- `fixtures/core-semantics/`

Do not touch:

- frontend lexer/parser
- BigInt bitwise helpers
- spread operator implementation

## Acceptance criteria

- [ ] A focused Node/iwasm fixture proves `~0`, `~1`, and `~-1` match Node.
- [ ] `lower_unary_op` no longer returns `UnsupportedSyntax` for `UnaryOp::BitwiseNot` on the focused integer slice.
- [ ] `env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/noImplicitAnyLoopCrash.ts` no longer reports `unary operator BitwiseNot not yet supported`; any next spread/semantic boundary is recorded.

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run -p ts2wasm-cli bitwise
python scripts/manager.py update-issue-index --check
python scripts/manager.py check-issue-health
python scripts/manager.py check-issue-readiness -- --fail-ready-below 80
git diff --check
```

Impacted commands:

```sh
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/noImplicitAnyLoopCrash.ts
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/noImplicitAnyLoopCrash.ts --detail --no-dashboard-data
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
`issues/done/3544-implement-noImplicitAnyLoopCrash.md`.

`lower_unary_op` currently maps `UnaryOp::Void` to `LoweredUnaryOp::Void` but
returns `UnsupportedSyntax` for `UnaryOp::BitwiseNot`. BigInt bitwise-not has
separate resolver/runtime paths; do not route ordinary Number `~` through them.

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
