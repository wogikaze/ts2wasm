---
id: 5214
title: "Implement computed Symbol.iterator prerequisite for spread"
type: feature
area: frontend/syntax
class: blocked
priority: P2
depends_on: []
blocks: [353]
created: 2026-05-01
updated: 2026-05-01
---

## Summary

Custom iterable spread requires object literals and property lookup involving
`[Symbol.iterator]`. Issue 353 cannot add its required custom iterable
Node/iwasm fixture while computed object keys are rejected before runtime
iterator lowering.

## Problem

The parser currently rejects an object literal key written as
`[Symbol.iterator]`, so custom iterable spread fails before the runtime can
look up `Symbol.iterator`, call it, and consume `.next()` results.

Problem: Computed `[Symbol.iterator]` object members are rejected before custom iterable spread can be tested.

## Current failure

```sh
tmp=/tmp/ts2wasm-353-custom-iterable-spread.ts
printf 'const iterable = { [Symbol.iterator]: function() { let i = 0; return { next: function() { i = i + 1; return { value: i, done: i > 2 }; } }; } };\nconst arr = [...iterable];\nconsole.log(arr.length);\nconsole.log(arr[0]);\nconsole.log(arr[1]);\n' > "$tmp"
node "$tmp"
cargo run -q -p ts2wasm-cli -- build "$tmp" -o /tmp/ts2wasm-353-custom-iterable-spread.wasm
```

Node result:

```text
2
1
2
```

ts2wasm result:

```text
error: [UnsupportedSyntax] expected identifier or string literal as object key, got Some(LeftBracket) at 19..20
```

## Desired final state

The compiler has a defined and implemented narrow path for computed
`[Symbol.iterator]` object members sufficient to build custom iterable spread
fixtures, or reports a later explicit issue 353 iterator diagnostic after the
computed key is recognized.

## Scope

In scope:

- [ ] Define the narrow representation for `Symbol.iterator` keys
- [ ] Accept object literal computed key syntax for `[Symbol.iterator]`
- [ ] Preserve enough property information for issue 353 iterator lookup
- [ ] Add a custom iterable spread fixture that reaches issue 353's iterator boundary

Out of scope:

- Full issue 353 iterator protocol integration
- General computed property name semantics unrelated to `Symbol.iterator`
- Sparse array spread
- Dynamic object property enumeration spread

## Affected paths

Expected:

- `crates/frontend/src/`
- `crates/ir/src/`
- `crates/runtime-abi/`
- `crates/backend-wasm/src/`
- `crates/cli/tests/`
- `fixtures/core-semantics/spread*`

Do not touch:

- `issues/open/5211-sparse-array-spread-support.md`
- `issues/done/355-dynamic-object-enumeration-spread.md`

## Acceptance criteria

- [ ] `{ [Symbol.iterator]: function() { ... } }` no longer fails with `expected identifier or string literal as object key`
- [ ] A custom iterable spread fixture reaches either Node/iwasm parity or a source-backed issue 353 iterator diagnostic
- [ ] The chosen `Symbol.iterator` representation is documented if it adds runtime ABI surface
- [ ] Existing supported spread slices remain passing

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

Not run:

- none

## Docs / current-state / issue sync

Final-state docs:

- [ ] updated: `docs/14-runtime-abi.md` if Symbol/iterator ABI surface is added

Current state:

- [ ] updated: `current-state.md` (repo root) if computed Symbol.iterator support changes

Follow-up issues:

- [ ] none

## Notes

This is a prerequisite split from issue 353. If the first implementation only
recognizes the syntax and emits an explicit issue 353 diagnostic, that is still
valid progress because it unblocks runtime-owned diagnostics from parser errors.

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
