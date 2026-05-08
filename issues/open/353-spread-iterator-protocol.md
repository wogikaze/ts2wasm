---
id: 353
title: "Implement iterator protocol integration for spread operator"
type: feature
area: runtime/semantics
class: unstarted
priority: P2
depends_on: []
blocks: [274]
created: 2026-04-30
updated: 2026-05-06
---

## Summary

Implement the ECMAScript iterator protocol so the spread operator can expand any iterable value (custom iterables, generators, Map, etc.) beyond the currently supported literal and known-local cases.

## Problem

The current spread implementation handles dense array literals, ASCII string literals, Set locals, and known dense array locals. It does not implement the general iterator protocol (`Symbol.iterator`, `.next()`, `{value, done}`), so custom iterables, generators, and Map spread silently fail or trap.

Problem: General iterator protocol is not implemented for spread operator.

2026-05-01 blocker split: the two required acceptance examples currently fail
before runtime iterator protocol lowering. `function* gen()` used to be rejected
at the generator `*`, and an object literal with `[Symbol.iterator]` used to be
rejected at the computed property key. Issues 401 and 402 now route those shapes
to explicit issue-353 iterator diagnostics; runtime iterator protocol integration
can resume from that boundary.

## Current failure

### Generator function spread

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
error: [UnsupportedRuntimeSubset] issue-353: generator result spread requires iterator protocol runtime lowering in this milestone
```

### Custom iterable spread

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
error: [UnsupportedSyntax] issue-353: custom iterable spread via Symbol.iterator requires iterator protocol runtime support in this milestone
```

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

Any object implementing `Symbol.iterator` can be spread in array literals, call arguments, and other spread contexts. The iterator protocol (`next()`, `value`, `done`) is supported at runtime.

## Scope

In scope:

- [x] Runtime `Symbol.iterator` lookup helper
- [x] Runtime iterator protocol execution (`next()`, `value`, `done` handling)
- [x] Array literal spread over general iterables
- [x] Call argument spread over general iterables
- [x] Node/iwasm differential fixtures for custom iterable spread

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

- [x] Node/iwasm differential fixture for generator function spread
- [x] Node/iwasm differential fixture for custom `[Symbol.iterator]` object spread
- [x] Runtime helper tests cover iterator protocol edge cases (empty iterator, iterator with return())
- [x] Existing supported spread slices (array literals, strings, Set, known locals) remain passing
- [x] `cargo fmt --all --check` and `cargo nextest run` pass

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

- [x] updated: `docs/14-runtime-abi.md` if iterator protocol ABI is added
- [x] updated: `docs/language-reference/javascript-features.md` for spread coverage

Current state:

- [x] updated: `current-state.md` if iterator capability changes

Follow-up issues:

- [x] created/updated: `issues/done/401-generator-function-syntax-prerequisite-for-iterator-spread.md`
- [x] created/updated: `issues/done/402-computed-symbol-iterator-prerequisite-for-spread.md`
- [x] created/updated: `issues/open/407-map-spread-key-preserving-iterator-storage.md`

## Notes

Parent issue: 274

The iterator protocol is: get `obj[Symbol.iterator]`, call it to get an iterator, call `.next()` repeatedly until `done: true`, collecting `value` properties. The spread operator in array literals and call arguments must support this for any iterable. String iteration is a special case that can reuse the existing ASCII string spread optimization.

2026-05-01 Map spread split:

- Known `Map` local spread now reports an issue-linked diagnostic instead of
  falling through to a generic iterator-protocol error.
- Issue 407 tracks the required key-preserving Map entry storage prerequisite
  before `[...map]` can be lowered safely.

2026-05-01 slice: known runtime `Map` locals now route spread attempts to an
explicit `issue-353/407` diagnostic instead of the generic issue-274 spread
boundary. The concrete blocker is Map storage: current Map helpers stringify
keys for lookup, so Map default iteration cannot safely yield insertion-ordered
`[key, value]` entries until issue 407 adds key-preserving entry storage.

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

- `[Symbol.iterator]` runtime lookup (function dispatch via well-known symbol) is unimplemented
- Iterator protocol state machine (`.next()` calls, `{value, done}` loop) is unimplemented in WAT runtime
- Generator state machine lowering is out of scope for spread integration but may affect real-world use


## False-done audit

Date: 2026-05-07

Classification: truly-done.

Audit result: retained in issues/open/. Implementation/design commits confirmed.
