---
id: 338
title: "Sparse array holes handling for Array.prototype.map"
type: feature
area: runtime/builtins
class: blocked
priority: P2
depends_on: [404]
blocks: []
created: 2026-04-30
updated: 2026-05-01
---

## Summary

Implement sparse array holes handling for `Array.prototype.map`. Sparse arrays
should skip holes during iteration and preserve them in the result array,
matching Node behavior.

## Problem

Problem: current `Array.prototype.map` implementation only supports dense
arrays. Sparse arrays with holes (e.g., `[1, , 3]`) are not handled correctly.

## Desired final state

`Array.prototype.map` correctly handles sparse arrays by:
- Skipping holes during iteration (not calling callback for hole indices)
- Preserving holes in result array at the same indices
- Matching Node behavior for sparse array operations

## Scope

In scope:

- [ ] Detect sparse arrays in map operations
- [ ] Skip holes during map iteration (no callback invocation for holes)
- [ ] Preserve holes in result array at correct indices
- [ ] Add sparse array map fixtures
- [ ] Validate with Test262 sparse array map tests

Out of scope:

- Dense array behavior (already implemented in issues 270, 295)
- thisArg (tracked by issue 339)
- Generic call (tracked by issue 340)

## Affected paths

Expected:

- `crates/ir/src/lowered/`
- `crates/backend-wasm/src/`
- `crates/cli/tests/`
- `fixtures/`

Do not touch:

- Dense array map implementation
- thisArg implementation
- Generic call implementation

## Acceptance criteria

- [ ] A sparse array map fixture (e.g., `[1, , 3].map(x => x * 2)`) matches Node output under `iwasm`.
- [ ] Holes are skipped during iteration (callback not called for hole indices).
- [ ] Holes are preserved in result array at correct indices.
- [ ] Existing dense-array map fixtures still pass.
- [ ] Selected Test262 sparse array map tests pass.

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run
python scripts/manager.py update-issue-index --check
python scripts/manager.py check issues
```

Impacted commands:

```sh
cargo nextest run -p ts2wasm-cli array_map
```

Not run:

- none

## Docs / current-state / issue sync

Final-state docs:

- [ ] not affected
- [ ] updated if sparse array representation changes

Current state:

- [x] updated: `current-state.md` records this issue as open sparse map execution work using the accepted sparse array representation contract

## Notes

Sparse array representation is defined by `docs/13-ir-contracts.md` and
`docs/14-runtime-abi.md`. Issue 403 accepted a frontend slot representation for
elisions, lowered present/hole slots, a presence-bitmap array contract, numeric
`index in array` presence checks, and map/spread observability. This issue
should implement map behavior without redefining the representation.

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

- Sparse array representation changes may affect other array operations

## Blocked evidence

2026-04-30 child-338-340:

- BLOCKED for this cycle. The frontend array literal parser stores `Expr::Array { elements: Vec<Expr> }` and expects an expression after each comma; it has no hole/elision node that could preserve `[1, , 3]`.
- Lowered arrays are dense `ArrayNew { elements }` values, and current backend array layout stores only length plus contiguous element values. There is no per-index presence bitmap/sentinel contract to preserve holes.
- Implementing sparse map correctly would require broader array representation/parser contract work outside this child slice.

Remaining:

- Not DONE. Needs a separate sparse array representation design before map can skip callback invocation for holes and preserve holes in results.

2026-05-01 child-338-sparse-array-map-blocker:

- BLOCKED. Narrow reproduction:
  `cargo run -q -p ts2wasm-cli -- build /tmp/ts2wasm-338-sparse-map.ts -o /tmp/ts2wasm-338-sparse-map.wasm`
  fails before lowering with
  `[UnsupportedSyntax] unsupported expression: Some(SpannedToken { kind: Comma, span: Span { start: 32, end: 33 } })`.
- Parser evidence: `crates/frontend/src/ast.rs` represents array literals as `Expr::Array { elements: Vec<Expr> }`, and `crates/frontend/src/parser/expressions.rs` pushes `self.expression()?` for each comma-separated element. There is no elision/hole node for `[1, , 3]`.
- IR/backend evidence: `crates/ir/src/lowered/types.rs` represents arrays as dense `LoweredExpr::ArrayNew { elements: Vec<LoweredExpr> }`; `crates/ir/src/lowered/resolver_extra.rs` maps `Array.prototype.map` by iterating every dense element and callback-lowering each one; `crates/backend-wasm/src/expr_emit.rs` emits dense array literals through `emit_array_literal`.
- Follow-up issue 403 was created for the sparse array representation contract. Issue 338 should remain blocked until that contract defines how frontend holes, lowered IR, runtime layout, `in` checks, and map result holes are represented.

2026-05-01 child-403-sparse-hole-contract:

- READY. Issue 403 moved the representation decision to
  `docs/13-ir-contracts.md` and `docs/14-runtime-abi.md`.
- Implement this issue against `ArrayLiteralElement::Hole` / lowered present-hole
  slots or an equivalent `ArrayNewSparse` path, presence-bit array layout, and
  numeric `index in array` checks. Do not encode holes as ordinary `undefined`.
- Exact fixture target: create fixture `array-map-sparse-holes.ts` under the existing core-semantics array fixture group; it should
  cover callback call count, `0 in mapped`, `1 in mapped`, `2 in mapped`, mapped
  values, and `mapped.length`.
- Targeted validation: `cargo fmt --all --check`; `cargo nextest run -p ts2wasm-cli array_map`; `mise run update-issue-index -- --check`; `mise run check issues`.

## Progress evidence

2026-05-01 child/338-sparse-map-20260501-074015:

- Implemented parser/AST slot preservation for sparse array literal holes.
- Added sparse lowered/backend path for hole-bearing array literals and sparse map results.
- Added `fixtures/core-semantics/array-map-sparse-holes.ts`; it covers callback call count through `thisArg`, `0 in mapped`, `1 in mapped`, `2 in mapped`, mapped values, and `mapped.length`.
- Validation passed: `cargo fmt --all --check`.
- Validation passed: `cargo nextest run -p ts2wasm-cli array_map` (16/16 passed, including the new sparse holes fixture and existing dense map fixtures).
- Selected Test262 sparse map validation was not run in this worktree because `reference/` contains no checked-out Test262 cases. Issue remains open until a Test262 representative can be run or the close requirement is explicitly waived by parent.
- Parent validation also ran
  `mise run reference-triage -- test262 reference/test262/test/built-ins/Array/prototype/map/15.4.4.19-8-b-1.js`
  from the main worktree with `reference/test262` present. The representative
  still reports `UnsupportedSyntax` through the existing issue-268
  increment/decrement diagnostic before it can validate sparse-map semantics.
  This keeps issue 338 open even though the curated sparse holes fixture passes.

2026-05-01 parent-cycle:

- Implemented identifier increment/decrement expression statements for the
  supported assignment-compatible subset and added
  `fixtures/core-semantics/increment-expression-statement.ts`.
- Validation passed:
  `cargo test -p ts2wasm-cli increment -- --nocapture`.
- Parent Test262 triage rerun:
  `mise run reference-triage -- test262 reference/test262/test/built-ins/Array/prototype/map/15.4.4.19-8-b-1.js`
  now advances past the previous issue-268 increment/decrement diagnostic and
  reaches `UnresolvedName: unresolved name: callCnt`.
- Issue remains open: the selected Test262 sparse map representative now needs
  mutable outer local capture/name-resolution support for callback-local
  mutation before sparse-map semantics can be fully validated against Test262.
  Follow-up issue 404 tracks that blocker.

2026-05-01 child-404-callback-captures-20260430T231258Z:

- Issue 404 narrowed the selected Test262 sparse map representative past
  `UnresolvedName: callCnt`.
- Parent rerun after issue 404 close:
  `mise run reference-triage -- test262 reference/test262/test/built-ins/Array/prototype/map/15.4.4.19-8-b-1.js`
  now reports `[UnsupportedSyntax] issue-207: instanceof right-hand side must be
  a supported class constructor \`Array\`` from the Test262 assertion harness.
- Issue 338 remains blocked for full selected-representative close evidence.
  Follow-up issue 405 tracks the harness `instanceof` RHS blocker.
