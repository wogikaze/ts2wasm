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
updated: 2026-05-01
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

- [ ] Parse generator function declarations used by `function* gen() { yield ... }`
- [ ] Preserve enough generator metadata for later iterator protocol lowering
- [ ] Add a regression fixture that reaches issue 353's iterator spread boundary

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

- [ ] `function* gen() { yield 1; yield 2; }` no longer fails with `expected identifier` at `*`
- [ ] A generator spread fixture reaches either Node/iwasm parity or a source-backed issue 353 iterator diagnostic
- [ ] Existing supported spread slices remain passing
- [ ] Docs/current-state/issues are synchronized when status or design changes

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

- [ ] not affected

Current state:

- [ ] updated: `current-state.md` (repo root) if generator support changes

Follow-up issues:

- [ ] none

## Notes

This is a prerequisite split from issue 353. It may choose an explicit
unsupported diagnostic instead of full generator execution if that is the
smallest safe slice.

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
