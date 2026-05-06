---
id: 5173
title: "Avoid stack overflow on deep binary expressions"
type: feature
area: ir/builtin-resolver
class: implementation-ready
priority: P1
depends_on: []
blocks: []
created: 2026-05-06
updated: 2026-05-06
---

## Summary

Make builtin resolution handle very deep left-associated binary `+` expression chains without overflowing the Rust stack.

## Problem

`binderBinaryExpressionStress.ts` and its `.js` companion tokenize successfully, then abort during `resolve_builtins` with `thread 'main' has overflowed its stack`. The crash happens before any ordinary diagnostic can be reported.

Problem: recursive expression folding in builtin resolution cannot process the deep binary-expression stress references.

## Current failure

Representative reference triage:

```sh
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/binderBinaryExpressionStress.ts
```

Current compiler diagnostic:

```text
thread 'main' has overflowed its stack
fatal runtime error: stack overflow, aborting
```

Compiler evidence:

- Token dumps succeed and are truncated only for size.
- AST dump is unavailable because the process aborts.
- Resolved dump reaches `validate_ast`, `module_graph`, `resolve_names`, and then aborts in `resolve_builtins`.
- The sibling file `binderBinaryExpressionStressJs.ts` has the same signature.

## Desired final state

Builtin resolution processes the deep binary expression without stack overflow, or returns a controlled source-spanned diagnostic if a later semantic boundary is still unsupported.

## Scope

In scope:

- [x] Replace or guard the recursive builtin fold path for left-associated binary expression chains.
- [x] Add focused regression coverage for a deep `+` chain that previously overflowed.
- [x] Confirm both stress reference files no longer abort during `resolve_builtins`.

Out of scope:

- General parser recursion redesign outside the demonstrated binary-chain path.
- Optimizing final emitted code for the full stress benchmark.
- TypeScript oracle timeout behavior for these large files.

## Affected paths

Expected:

- `crates/ir/src/builtin_resolver.rs`
- focused IR/compiler tests for deep binary expressions
- `scripts/run/reference-triage.py` only if crash classification needs refinement

Do not touch:

- unrelated binary operator semantics
- frontend lexer/parser unless new evidence shows the crash occurs before builtin resolution

## Acceptance criteria

- [x] A focused test exercises a deep left-associated `+` chain through builtin resolution.
- [x] The focused test no longer panics or aborts with stack overflow.
- [x] `python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/binderBinaryExpressionStress.ts` no longer reports `thread 'main' has overflowed its stack`.
- [x] `python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/binderBinaryExpressionStressJs.ts` no longer reports `thread 'main' has overflowed its stack`.

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run -p ts2wasm-ir deep_binary
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/binderBinaryExpressionStress.ts
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/binderBinaryExpressionStressJs.ts
```

Impacted commands:

```sh
python scripts/manager.py check-issue-readiness -- --fail-ready-below 80
```

Not run:

- none

## Docs / current-state / issue sync

Final-state docs:

- [x] not affected

Current state:

- [x] not affected

Follow-up issues:

- [x] none

## Notes

Split from generated buckets `1058` and `1059` on 2026-05-06.

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

## Completion evidence

Commits:

- `<hash>`

Validation result:

```text
command: cargo nextest run -p ts2wasm-ir
result: 27 passed, 0 failed
date: 2026-05-06

command: cargo nextest run -p ts2wasm-compiler
result: 65 passed, 0 failed
date: 2026-05-06
```

Remaining risks:

- lowering pass (lower_expr) and backend emission (emit_expr) are still recursive for deep binary chains; a deep expression that survives builtin resolution may overflow during lowering or emission. This is tracked as a follow-up.
