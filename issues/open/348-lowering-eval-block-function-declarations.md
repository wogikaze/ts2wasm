---
id: 348
title: "Lowering block-level function declarations in direct eval code"
type: feature
area: ir
class: implementation-ready
priority: P3
depends_on: [347]
blocks: []
created: 2026-04-30
updated: 2026-04-30
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

- [ ] IR representation for eval-code block function declaration hoisting
- [ ] Lowering path for `eval("...")` string argument to embedded AST
- [ ] Scope record creation for eval code with caller-local access
- [ ] Regression fixtures for block-level function declaration binding inside eval

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

- [ ] IR fixture proves eval-code block function declaration hoists to function scope
- [ ] Node/iwasm differential fixture matches Node output for `eval("{ function f() {} } return f();")`
- [ ] Regression fixture proves non-eval block-level functions retain ordinary behavior
- [ ] `cargo fmt --all --check` and `cargo nextest run` pass

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

- [ ] updated: `docs/13-ir-contracts.md` if eval-specific IR nodes are added

Current state:

- [ ] updated: `current-state.md` if lowering capability changes

Follow-up issues:

- [ ] created: issue 349 for runtime/shim

## Notes

Parent issue: 225

The Annex B hoisting semantics are: when a block-level function declaration appears inside eval code, the binding is created in the VariableEnvironment (function scope) of the eval code, not the LexicalEnvironment (block scope). This is observable when the enclosing block ends but the function remains accessible.

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
