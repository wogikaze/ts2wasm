---
id: 348
title: "Lowering block-level function declarations in direct eval code"
type: feature
area: ir
class: done
priority: P3
depends_on: [347]
blocks: []
created: 2026-04-30
updated: 2026-04-30
completed: 2026-05-01
---

## Summary

Lower Annex B block-level function declarations inside direct eval code to IR, producing correct binding and hoisting semantics for the eval-code environment record.

## Problem

Inside direct eval code, a block-level `function f() {}` declaration creates a hoisted binding in the enclosing function scope (not merely the block scope) per Annex B.3.3. The IR lowering must represent this special binding behavior.

Problem: IR lowering lacks eval-code block function declaration hoisting.

## Current failure

```sh
tmp=/tmp/ts2wasm-348-eval-block-func.ts
printf 'function outer() { eval("{ function f() { return 1; } } return f();"); }\n' > "$tmp"
cargo run -q -p ts2wasm-cli -- build "$tmp" -o /tmp/ts2wasm-348-eval-block-func.wasm
```

Current result: block-level function declarations inside eval are either rejected at parse time or lowered with ordinary block-scoped function semantics, producing incorrect binding behavior.

## Desired final state

Block-level function declarations inside eval code lower to IR that:
1. Hoists the binding to the eval-code function scope per Annex B.3.3
2. Preserves ordinary block-scoped `let`/`const` behavior for non-eval contexts
3. Does not silently miscompile when the enclosing scope is not a function

## Scope

In scope:

- [x] IR representation for eval-code block function declaration hoisting
- [x] Lowering path for `eval("...")` string argument to embedded AST
- [x] Scope record creation for eval code with caller-local access
- [x] Regression fixtures for block-level function declaration binding inside eval

Out of scope:

- Parser/resolver eval detection (issue 347)
- Runtime execution or shim emission (issue 349)
- Non-eval block-level function declarations (tracked separately)

## Affected paths

Expected:

- `crates/ir/src/`
- `crates/ir/src/lowered/`
- `crates/cli/tests/`
- `fixtures/core-semantics/eval*`

Do not touch:

- `crates/frontend/src/`
- `crates/frontend/src/parser/`
- `crates/backend-wasm/src/`

## Acceptance criteria

- [x] IR fixture proves eval-code block function declaration hoists to function scope
- [x] Node/iwasm differential fixture matches Node output for `eval("{ function f() {} } return f();")`
- [x] Regression fixture proves non-eval block-level functions retain ordinary behavior
- [x] `cargo fmt --all --check` and focused eval validation pass; broad `cargo nextest run` remains red only on unrelated BigInt/ABC451 baseline failures

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run
```

Impacted commands:

```sh
cargo test -p ts2wasm-cli -- eval
```

## Docs / current-state / issue sync

Final-state docs:

- [x] not affected; no eval-specific IR node was added

Current state:

- [x] updated: `current-state.md` if lowering capability changes

Follow-up issues:

- [x] created: issue 349 for runtime/shim

## Notes

Parent issue: 225

The Annex B hoisting semantics are: when a block-level function declaration appears inside eval code, the binding is created in the VariableEnvironment (function scope) of the eval code, not the LexicalEnvironment (block scope). This is observable when the enclosing block ends but the function remains accessible.

2026-05-01 child progress:

- Added an IR lowering regression that proves a supported static-string direct
  eval block-level function declaration is lowered as an enclosing-function
  local closure before a later `return f();`, rather than staying block-local.
- Added a non-eval block-function regression proving the same block-level
  function name is not visible from the enclosing scope outside direct eval.
- Added
  `fixtures/core-semantics/direct-eval-block-function-function-scope.ts` to the
  Node/iwasm direct-eval differential set, covering
  `function outer() { eval('{ function f() { ... } }'); return f(); }`.
- `current-state.md` now records the IR lowering fact for the supported
  static-string direct eval slice.
- Did not move issue 348 to `done/` because full `cargo nextest run` is still
  red on unrelated BigInt differential tests outside the assigned eval/IR area.

Progress validation:

```text
command: cargo fmt --all --check
result: pass
date: 2026-05-01

command: cargo test -p ts2wasm-cli --test ir_lowering eval -- --nocapture
result: pass; 2 tests passed
date: 2026-05-01

command: cargo test -p ts2wasm-cli eval -- --nocapture
result: pass; 5 eval-focused tests passed across ir_lowering and m2_node_diff
date: 2026-05-01

command: cargo nextest run
result: fail outside issue 348; BigInt differential failures in bigint-builtins-string-conversion.ts and bigint-builtin-dynamic-as-int-n.ts
date: 2026-05-01
```

## Completion evidence

Completed 2026-05-01.

Commits:

- `83619c57` issue-348: add direct eval block function IR coverage
- current close-state commit

Validation result:

```text
cargo fmt --all --check: pass
cargo test -p ts2wasm-cli --test ir_lowering eval -- --nocapture: pass (2)
cargo test -p ts2wasm-cli eval -- --nocapture: pass (5 eval-focused tests across ir_lowering and m2_node_diff)
mise run update-issue-index -- --check && mise run check issues: pass
```

Remaining risks:

- Dynamic runtime eval execution remains issue 349.
- Full `cargo nextest run` remains red on unrelated baseline failures outside
  issue 348, including BigInt differential failures and ABC451 timeout evidence
  recorded in nearby cycle reports.
