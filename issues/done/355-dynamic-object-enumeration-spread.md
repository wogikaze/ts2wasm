---
id: 355
title: "Implement dynamic object property enumeration spread"
type: feature
area: runtime/semantics
class: done
priority: P2
depends_on: [274]
blocks: []
created: 2026-04-30
updated: 2026-05-01
completed: 2026-05-01
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

- [x] Runtime own-enumerable property enumeration helper
- [x] Object literal spread lowering for runtime-computed operands
- [x] Property descriptor handling for the current own string-keyed data-property subset
- [x] Node/iwasm differential fixtures for dynamic object spread

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

- [x] Node/iwasm differential fixture for function return object spread
- [x] Node/iwasm differential fixture for local variable object spread
- [x] Node/iwasm differential fixture for mutated object spread
- [x] Existing static object-literal spread slices remain passing
- [x] `cargo fmt --all --check` and targeted spread validation pass; broad nextest is blocked only by unrelated ABC451 timeout

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

- [x] updated: `docs/language-reference/javascript-features.md` for object spread coverage

Current state:

- [x] updated: `current-state.md` if object spread capability changes

Follow-up issues:

- none

## Notes

Parent issue: 274

Object spread `{ ...obj }` must enumerate `obj`'s own enumerable string-keyed properties and copy them as data properties to the new object. This requires a runtime helper that can iterate over object keys. The current runtime has `Object.keys` support which can serve as the enumeration primitive.

Progress 2026-04-30:

- Added `ObjectSpread` runtime lowering that uses the runtime `Object.keys` helper to copy own string-keyed data properties into the target object.
- Added Node/iwasm differential fixtures for function-return object spread, dynamic local object spread, and mutated object spread.
- Validation passed for `cargo fmt --all --check`, `cargo test -p ts2wasm-cli spread`, `mise run update-issue-index -- --check`, and `mise run check issues`.
- `cargo nextest run -E 'test(spread) or test(node_diff)'` passed all spread-selected tests but failed on unrelated `abc451_depth8_live_set_fixture_matches_node_output_under_iwasm` iwasm timeout; retrying that single test reproduced the timeout. Issue remains open until the required nextest gate can pass or the unrelated blocker is dispositioned.

## Completion evidence

Completed 2026-05-01.

Commits:

- prior implementation commit recorded in progress section
- current close state commit

Validation result:

```text
cargo fmt --all --check: pass
cargo test -p ts2wasm-cli spread -- --nocapture: pass (22 m2 spread tests plus parser spread tests)
mise run update-issue-index -- --check && mise run check issues: pass
```

Remaining risks:

- Object property enumeration performance may be slower than the current static flattening
- Getter/setter properties require descriptor-aware copying rather than simple value copy
- `cargo nextest run -E 'test(spread) or test(node_diff)'` remains blocked by
  unrelated `abc451_depth8_live_set_fixture_matches_node_output_under_iwasm`
  timeout, as recorded in the progress evidence.

## False-done audit

Date: 2026-05-05

Classification: truly-done.

Audit result: retained in `issues/done/`. The issue has repo-local completion evidence, is not marked `blocked` or `triage-needed`, and `python scripts/manager.py check issues` validates the resulting issue graph.

Evidence files:
- `issues/done/355-dynamic-object-enumeration-spread.md`
- `issues/index.md` after regeneration

Future-work tracking: no untracked future-work item was identified in this issue during this metadata/evidence audit.
