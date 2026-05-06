---
id: 354
title: "Implement sparse array spread support"
type: feature
area: runtime/semantics
class: implementation-ready
priority: P2
depends_on: [274]
blocks: []
created: 2026-04-30
updated: 2026-05-01
---

## Summary

Implement spread operator behavior for sparse arrays using the shared sparse
array representation contract. Array/call spread observes holes through
iterator/Get semantics, so holes become present `undefined` values in the
destination rather than preserved absent slots.

## Problem

The current spread implementation only supports dense arrays. When spreading a
sparse array (e.g., `const sparse = [1, , 3]; const arr = [...sparse];`), the
hole must be read as `undefined` and stored as a present destination element.
The current implementation either produces incorrect dense arrays or rejects
sparse array spread.

Problem: Sparse array spread needs shared hole representation support and
iterator/Get-compatible hole materialization to present `undefined`.

## Current failure

```sh
tmp=/tmp/ts2wasm-354-sparse-spread.ts
printf 'const sparse = [1, , 3];\nconst arr = [...sparse];\nconsole.log(0 in arr, 1 in arr, 2 in arr);\n' > "$tmp"
cargo run -q -p ts2wasm-cli -- build "$tmp" -o /tmp/ts2wasm-354-sparse-spread.wasm
```

Current result: `[UnsupportedSyntax] issue-274: sparse array spread is not supported`

## Desired final state

Sparse array spread produces arrays with present `undefined` values for source
holes. Array methods that interact with spread results (e.g., `.map()` on a
spread sparse array) observe those positions as present, matching Node behavior.

## Scope

In scope:

- [ ] Sparse array representation in array literal spread
- [ ] Hole materialization to present `undefined` when spreading sparse arrays
- [ ] Call argument spread with sparse arrays (holes map to undefined in arguments)
- [ ] Node/iwasm differential fixtures for sparse array spread

Out of scope:

- General iterator protocol (issue 353)
- Dynamic object property enumeration spread (issue 355)
- Sparse array creation syntax (`[1, , 3]`) improvements beyond spread context

## Affected paths

Expected:

- `crates/runtime-abi/`
- `crates/backend-wasm/src/`
- `crates/ir/src/`
- `crates/cli/tests/`
- `fixtures/core-semantics/spread*`

Do not touch:

- `crates/frontend/src/`

## Acceptance criteria

- [ ] Node/iwasm differential fixture proves sparse array spread materializes holes as present `undefined`
- [ ] Node/iwasm differential fixture proves `0 in [...sparse]` / `1 in [...sparse]` / `2 in [...sparse]` match Node
- [ ] Node/iwasm differential fixture proves call spread with sparse array maps holes to undefined
- [ ] Existing dense array spread slices remain passing
- [ ] `cargo fmt --all --check` and `cargo nextest run` pass

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run -E 'test(spread) or test(node_diff)'
mise run update-issue-index -- --check
mise run check issues
```

Impacted commands:

```sh
cargo test -p ts2wasm-cli spread
```

## Docs / current-state / issue sync

Final-state docs:

- [ ] updated: `docs/language-reference/javascript-features.md` for sparse array spread

Current state:

- [ ] updated: `current-state.md` if sparse array capability changes

Follow-up issues:

- [ ] none

## Notes

Parent issue: 274

Sparse array representation is defined by `docs/13-ir-contracts.md` and
`docs/14-runtime-abi.md`. In ECMAScript, sparse array holes are not the same as
`undefined` values while they remain array properties. When a sparse array is
spread into a new array literal or call arguments, spread consumes iterator/Get
semantics: source holes are read as `undefined`, and the destination
element/argument is present.

Exact fixture targets:

- Fixture name `spread-sparse-array-materializes-undefined.ts` under the existing core-semantics spread fixture group: cover
  `0 in arr`, `1 in arr`, `2 in arr`, `arr[1] === undefined`, and `arr.length`.
- Fixture name `spread-sparse-call-undefined.ts` under the existing core-semantics spread fixture group: cover a call where
  the hole argument is observed as `undefined`.

Targeted validation: `cargo fmt --all --check`; `cargo nextest run -E 'test(spread) or test(node_diff)'`; `mise run update-issue-index -- --check`; `mise run check issues`.

## Progress

2026-05-01:

- Added Node/iwasm differential fixtures `spread-sparse-array-materializes-undefined.ts` and `spread-sparse-call-undefined.ts`.
- Focused sparse validation passed: `cargo test -p ts2wasm-cli spread_operator_sparse -- --nocapture` ran both sparse tests successfully.
- Required broad spread validation is blocked by existing Set spread failures outside issue 354: `spread-array-set.ts` prints `undefined` values where Node prints `2` and `1`, and `spread-call-set-local.ts` traps with `Exception: unreachable`.
- Required nextest selector is also blocked before running tests by a frontend lib-test compile error in `crates/frontend/src/parser/tests.rs`: `use of undeclared type ArrayLiteralElement`.
- Issue remains open until the required broad gates pass or those unrelated blockers are dispositioned.

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

- Sparse array representation in the runtime may need backend changes for hole tracking
