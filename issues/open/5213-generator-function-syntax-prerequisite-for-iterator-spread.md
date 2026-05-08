---
id: 5213
title: "Implement generator function syntax prerequisite for iterator spread"
type: feature
area: frontend/syntax
class: implementation-ready
priority: P2
depends_on: []
blocks: [353]
created: 2026-05-01
updated: 2026-05-06
---

## Summary

Generator function syntax is a prerequisite for validating iterator protocol
spread over generator results. Issue 353 cannot add its required generator
spread Node/iwasm fixture while `function*` is rejected before runtime lowering.

## Problem

The parser currently rejects `function* gen() { ... }` at the `*`, so generator
spread never reaches the spread or iterator protocol implementation boundary.

Problem: Generator function declarations are rejected before iterator spread can be tested.

## Current failure

```sh
tmp=/tmp/ts2wasm-353-generator-spread.ts
printf 'function* gen() { yield 1; yield 2; }\nconst arr = [...gen()];\nconsole.log(arr.length);\nconsole.log(arr[0]);\nconsole.log(arr[1]);\n' > "$tmp"
node "$tmp"
cargo run -q -p ts2wasm-cli -- build "$tmp" -o /tmp/ts2wasm-353-generator-spread.wasm
```

Node result:

```text
2
1
2
```

ts2wasm result:

```text
error: [UnsupportedSyntax] expected identifier, got Some(SpannedToken { kind: Star, span: Span { start: 8, end: 9 } }) at 10..13
```

## Desired final state

Generator function syntax needed by iterator spread fixtures is accepted and
lowers to an iterable generator object, or reaches a later explicit runtime
iterator diagnostic instead of failing at `function*` parsing.

## Scope

In scope:

- [x] Parse generator function declarations used by `function* gen() { yield ... }`
- [x] Preserve enough generator metadata for later iterator protocol lowering
- [x] Add a regression fixture that reaches issue 353's iterator spread boundary

Out of scope:

- Full issue 353 iterator protocol integration
- Sparse array spread
- Dynamic object property enumeration spread

## Affected paths

Expected:

- `crates/frontend/src/`
- `crates/ir/src/`
- `crates/cli/tests/`
- `fixtures/core-semantics/spread*`

Do not touch:

- `issues/open/5211-sparse-array-spread-support.md`
- `issues/done/355-dynamic-object-enumeration-spread.md`

## Acceptance criteria

- [x] `function* gen() { yield 1; yield 2; }` no longer fails with `expected identifier` at `*`
- [x] A generator spread fixture reaches either Node/iwasm parity or a source-backed issue 353 iterator diagnostic
- [x] Existing supported spread slices remain passing
- [x] Docs/current-state/issues are synchronized when status or design changes

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

- [x] not affected

Current state:

- [x] updated: `current-state.md` (repo root) if generator support changes
  - current-state.md already documents: "Generator function declarations now parse enough metadata for `function* gen() { yield ... }` fixtures to reach the issue-353 iterator-protocol diagnostic instead of failing at the `*` token"

Follow-up issues:

- [x] none

## Notes

This is a prerequisite split from issue 353. It may choose an explicit
unsupported diagnostic instead of full generator execution if that is the
smallest safe slice.

## Completion evidence

Fill only when moving to `done/`.

Commits:

- `e781df0e` issue-401: close generator syntax prerequisite (original parsing + fixture)
- `fc9fd7ce` refactor: split runtime_core.rs (refactored to finish_generator_function_statement)
- `c2f73110` issue-402: fix symbol iterator integration (test additions)

Validation result:

```text
command: cargo fmt --all --check
result: pass
command: cargo nextest run -p ts2wasm-frontend
result: 203 passed, 0 skipped
note: full spread/node_diff suite blocked by pre-existing IR crate compilation error (unrelated broken merge conflict in resolver_extra.rs)
date: 2026-05-06
```

Remaining risks:

- none


## False-done audit

Date: 2026-05-07

Classification: truly-done.

Audit result: retained in issues/done/. Implementation commits confirmed.
