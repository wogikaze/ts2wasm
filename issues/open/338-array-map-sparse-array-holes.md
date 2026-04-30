---
id: 338
title: "Sparse array holes handling for Array.prototype.map"
type: feature
area: runtime/builtins
class: ready
priority: P2
depends_on: [334]
blocks: []
created: 2026-04-30
updated: 2026-04-30
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

- [ ] updated: `current-state.md` when sparse array map behavior is implemented

## Notes

Sparse array representation may require changes to the array layout contract.
Coordinate with runtime-abi team if array representation changes are needed.

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
