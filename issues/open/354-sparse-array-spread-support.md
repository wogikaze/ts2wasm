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
updated: 2026-04-30
---

## Summary

Implement spread operator behavior for sparse arrays, ensuring that holes are correctly preserved when spreading a sparse array into another array literal or call arguments.

## Problem

The current spread implementation only supports dense arrays. When spreading a sparse array (e.g., `const sparse = [1, , 3]; const arr = [...sparse];`), the hole must be preserved in the resulting array. The current implementation either produces incorrect dense arrays or rejects sparse array spread.

Problem: Sparse array hole preservation is unsupported in spread operator.

## Current failure

```sh
tmp=/tmp/ts2wasm-354-sparse-spread.ts
printf 'const sparse = [1, , 3];\nconst arr = [...sparse];\nconsole.log(0 in arr, 1 in arr, 2 in arr);\n' > "$tmp"
cargo run -q -p ts2wasm-cli -- build "$tmp" -o /tmp/ts2wasm-354-sparse-spread.wasm
```

Current result: `[UnsupportedSyntax] issue-274: sparse array spread is not supported`

## Desired final state

Sparse array spread produces arrays with correct holes. Array methods that interact with spread results (e.g., `.map()` on a spread sparse array) observe the holes correctly.

## Scope

In scope:

- [ ] Sparse array representation in array literal spread
- [ ] Hole preservation when spreading sparse arrays
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

- [ ] Node/iwasm differential fixture proves sparse array spread preserves holes
- [ ] Node/iwasm differential fixture proves `0 in [...sparse]` matches Node
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

In ECMAScript, sparse array holes are not the same as `undefined` values. When a sparse array is spread into a new array literal, the holes should be preserved in the destination. When spread into call arguments, holes are mapped to `undefined` because the arguments object is always dense.

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
