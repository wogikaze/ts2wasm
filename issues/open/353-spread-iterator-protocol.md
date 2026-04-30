---
id: 353
title: "Implement iterator protocol integration for spread operator"
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

Implement the ECMAScript iterator protocol so the spread operator can expand any iterable value (custom iterables, generators, Map, etc.) beyond the currently supported literal and known-local cases.

## Problem

The current spread implementation handles dense array literals, ASCII string literals, Set locals, and known dense array locals. It does not implement the general iterator protocol (`Symbol.iterator`, `.next()`, `{value, done}`), so custom iterables, generators, and Map spread silently fail or trap.

Problem: General iterator protocol is not implemented for spread operator.

## Current failure

```sh
tmp=/tmp/ts2wasm-353-iterator-spread.ts
printf 'function* gen() { yield 1; yield 2; }\nconst arr = [...gen()];\nconsole.log(arr);\n' > "$tmp"
cargo run -q -p ts2wasm-cli -- build "$tmp" -o /tmp/ts2wasm-353-iterator-spread.wasm
```

Current result: `[UnsupportedSyntax] issue-274: general iterator protocol spread is not supported`

## Desired final state

Any object implementing `Symbol.iterator` can be spread in array literals, call arguments, and other spread contexts. The iterator protocol (`next()`, `value`, `done`) is supported at runtime.

## Scope

In scope:

- [ ] Runtime `Symbol.iterator` lookup helper
- [ ] Runtime iterator protocol execution (`next()`, `value`, `done` handling)
- [ ] Array literal spread over general iterables
- [ ] Call argument spread over general iterables
- [ ] Node/iwasm differential fixtures for custom iterable spread

Out of scope:

- Sparse array spread (issue 354)
- Dynamic object property enumeration spread (issue 355)
- Map/Set specific optimizations beyond general iterator protocol

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

- [ ] Node/iwasm differential fixture for generator function spread
- [ ] Node/iwasm differential fixture for custom `[Symbol.iterator]` object spread
- [ ] Runtime helper tests cover iterator protocol edge cases (empty iterator, iterator with return())
- [ ] Existing supported spread slices (array literals, strings, Set, known locals) remain passing
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
TS2WASM_REFERENCE_ROOT=./reference mise run reference-coverage -- test262 --limit 300
```

## Docs / current-state / issue sync

Final-state docs:

- [ ] updated: `docs/14-runtime-abi.md` if iterator protocol ABI is added
- [ ] updated: `docs/language-reference/javascript-features.md` for spread coverage

Current state:

- [ ] updated: `current-state.md` if iterator capability changes

Follow-up issues:

- [ ] none

## Notes

Parent issue: 274

The iterator protocol is: get `obj[Symbol.iterator]`, call it to get an iterator, call `.next()` repeatedly until `done: true`, collecting `value` properties. The spread operator in array literals and call arguments must support this for any iterable. String iteration is a special case that can reuse the existing ASCII string spread optimization.

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

- `Symbol` support in the runtime may need prerequisite work
- Generator function support may require generator state machine lowering
