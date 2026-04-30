---
id: 355
title: "Implement dynamic object property enumeration spread"
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

Implement general object property enumeration for the spread operator in object literals, supporting runtime-computed objects (locals, function returns, etc.) beyond the current static object-literal flattening.

## Problem

The current object spread implementation only flattens known static object literals and simple aliases. It cannot spread runtime-computed objects that require enumerating own enumerable properties at runtime, such as function return values or mutated object locals.

Problem: Dynamic object property enumeration is not implemented for spread.

## Current failure

```sh
tmp=/tmp/ts2wasm-355-dynamic-object-spread.ts
printf 'function makeObj() { return { a: 1, b: 2 }; }\nconst obj = { c: 3, ...makeObj(), d: 4 };\nconsole.log(obj);\n' > "$tmp"
cargo run -q -p ts2wasm-cli -- build "$tmp" -o /tmp/ts2wasm-355-dynamic-object-spread.wasm
```

Current result: `[UnsupportedSyntax] issue-274: dynamic object spread requires general property enumeration`

## Desired final state

Object literal spread can enumerate own enumerable properties of any runtime object value, including function returns, local variables, and built-in objects, producing Node-compatible output.

## Scope

In scope:

- [ ] Runtime own-enumerable property enumeration helper
- [ ] Object literal spread lowering for runtime-computed operands
- [ ] Property descriptor handling (enumerable, data properties vs accessors)
- [ ] Node/iwasm differential fixtures for dynamic object spread

Out of scope:

- General iterator protocol (issue 353)
- Sparse array spread (issue 354)
- Object spread specific edge cases (prototype chain properties, symbol keys) unless they arise from the implementation

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

- [ ] Node/iwasm differential fixture for function return object spread
- [ ] Node/iwasm differential fixture for local variable object spread
- [ ] Node/iwasm differential fixture for mutated object spread
- [ ] Existing static object-literal spread slices remain passing
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

- [ ] updated: `docs/language-reference/javascript-features.md` for object spread coverage

Current state:

- [ ] updated: `current-state.md` if object spread capability changes

Follow-up issues:

- [ ] none

## Notes

Parent issue: 274

Object spread `{ ...obj }` must enumerate `obj`'s own enumerable string-keyed properties and copy them as data properties to the new object. This requires a runtime helper that can iterate over object keys. The current runtime has `Object.keys` support which can serve as the enumeration primitive.

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

- Object property enumeration performance may be slower than the current static flattening
- Getter/setter properties require descriptor-aware copying rather than simple value copy
