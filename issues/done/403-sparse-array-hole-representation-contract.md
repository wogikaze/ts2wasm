---
id: 403
title: "Define sparse array hole representation contract"
type: feature
area: runtime/semantics
class: done
priority: P2
depends_on: []
blocks: [338, 354]
created: 2026-05-01
updated: 2026-05-01
completed: 2026-05-01
---

## Summary

Define the frontend, IR, runtime ABI, and backend contract for JavaScript sparse
array holes.

This is the prerequisite for sparse-array operations such as
`Array.prototype.map` hole skipping/preservation and sparse array spread.

## Problem

Problem: The compiler has no representation for array holes; array literals,
lowered arrays, and backend array layout are all dense, so `[1, , 3]` fails at
parse time and no later phase can preserve hole observability.

## Current failure

Narrow reproduction:

```sh
tmp=/tmp/ts2wasm-338-sparse-map.ts
printf 'let calls = 0;\nlet mapped = [1, , 3].map(x => { calls = calls + 1; return x * 2; });\nconsole.log(calls, 0 in mapped, 1 in mapped, 2 in mapped, mapped.length);\n' > "$tmp"
cargo run -q -p ts2wasm-cli -- build "$tmp" -o /tmp/ts2wasm-338-sparse-map.wasm
```

Current result:

```text
error: [UnsupportedSyntax] unsupported expression: Some(SpannedToken { kind: Comma, span: Span { start: 32, end: 33 } }) at 34..35
```

Structural evidence:

- `crates/frontend/src/ast.rs` uses `Expr::Array { elements: Vec<Expr> }`.
- `crates/frontend/src/parser/expressions.rs` parses every array slot as an expression after a comma.
- `crates/ir/src/lowered/types.rs` uses dense `LoweredExpr::ArrayNew { elements: Vec<LoweredExpr> }`.
- `crates/ir/src/lowered/resolver_extra.rs` lowers `Array.prototype.map` by iterating every dense element and callback-lowering it.
- `crates/backend-wasm/src/expr_emit.rs` emits dense array literals via `emit_array_literal`.

## Desired final state

Sparse array holes have an explicit project contract that defines:

- frontend syntax representation for array elisions and trailing elisions;
- resolved/lowered IR representation for present vs absent array elements;
- runtime array layout or sentinel/bitmap policy for hole presence;
- `in` operator behavior for supported array index checks;
- how array helpers such as `Array.prototype.map` and spread observe holes.

## Scope

In scope:

- [x] Define the parser AST representation for array holes.
- [x] Define the resolved/lowered IR representation for sparse arrays.
- [x] Define the runtime array layout or hole sentinel/bitmap ABI.
- [x] Define observable supported behavior for `index in array`, `length`, map callback skipping, and map result hole preservation.
- [x] Decide whether this issue implements the smallest parser/IR/backend substrate or splits implementation-ready child issues.

Out of scope:

- Completing `Array.prototype.map` sparse semantics; tracked by issue 338 after this prerequisite.
- Completing sparse array spread; tracked by issue 354 after this prerequisite.
- Generic array-like map behavior beyond already closed narrow slices.

## Affected paths

Expected:

- `docs/14-runtime-abi.md`
- `crates/frontend/src/`
- `crates/ir/src/`
- `crates/backend-wasm/src/`
- `crates/cli/tests/`
- `fixtures/core-semantics/array-*`
- `current-state.md`

Do not touch:

- BigInt/private/eval/ABC451 areas unless only issue references are updated.

## Acceptance criteria

- [x] `docs/14-runtime-abi.md` or an equivalent numbered doc records the sparse array hole contract.
- [x] A concrete parser/IR/runtime representation decision is recorded with allowed implementation paths.
- [x] Issue 338 can be changed from blocked to implementation-ready without redefining representation.
- [x] Issue 354 can reference the same representation contract instead of inventing a spread-specific hole model.
- [x] Any implementation-ready child issues created by this design name exact fixtures, commands, and affected paths.
- [x] `issues/index.md` is regenerated and issue health checks pass.

## Validation

Required commands:

```sh
mise run update-issue-index -- --check
mise run check issues
```

Impacted commands:

```sh
cargo fmt --all --check
cargo nextest run -p ts2wasm-cli array_map
```

Not run:

- none

## Docs / current-state / issue sync

Final-state docs:

- [x] updated: `docs/14-runtime-abi.md` when the representation contract is accepted

Current state:

- [x] updated: `current-state.md` when representation behavior changes

Follow-up issues:

- [x] no new child issue created; existing issues 338 and 354 now name exact fixtures, commands, and affected paths

## Notes

Do not encode holes as ordinary `undefined` values. ECMAScript distinguishes
absent properties from present properties whose value is `undefined`; issue 338
requires callback skipping and result hole preservation.

## Completion evidence

Fill only when moving to `done/`.

Completed: 2026-05-01

Commits:

- `61019ef4` issue-403: define sparse array hole contract

Validation result:

```text
command: cargo fmt --all --check
result: pass
date: 2026-05-01

command: mise run update-issue-index -- --check
result: pass; issues/index.md OK after regeneration
date: 2026-05-01

command: mise run check issues
result: pass; issues/index.md queue OK and check_issue_health OK
date: 2026-05-01

command: cargo nextest run -p ts2wasm-cli array_map
result: pass; 15 array_map tests passed, 428 skipped
date: 2026-05-01
```

Remaining risks:

- Implementation still needs parser/IR/backend changes in issues 338 and 354.
- Sparse array layout migration may affect dense array allocation, GC/rooting, and array helper assumptions.
