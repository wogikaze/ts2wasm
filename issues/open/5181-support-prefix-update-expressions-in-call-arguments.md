---
id: 5181
title: "Support prefix update expressions in call arguments"
type: feature
area: frontend/semantics
class: implementation-ready
priority: P1
depends_on: []
blocks: []
created: 2026-05-06
updated: 2026-05-06
---

## Summary

`blockScopedBindingsReassignedInLoop1.ts` parses `use(++i)` inside an arrow callback, but name/builtin resolution reports the for-loop update diagnostic from issue `268`.

## Problem

Issue `268` completed identifier increment/decrement support only for for-loop update slots. The representative TSC case uses the same identifier prefix update operator in an expression position: `use(++i)`. The AST preserves this as `Unary { op: PreIncrement, expr: Ident("i") }`, but the resolver/lowering path still rejects it with the for-loop-specific diagnostic.

Problem: identifier prefix update expressions in call arguments are parser-accepted but resolver/lowering-unsupported.

## Current failure

Reference triage:

```sh
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/blockScopedBindingsReassignedInLoop1.ts
```

Current compiler diagnostic:

```text
UnsupportedSyntax: issue-268: for-loop increment/decrement updates currently require an identifier target at 140..143
```

Representative source:

```ts
declare function use(n: number): void;

(function () {
  'use strict'
  for (let i = 0; i < 9; ++i) {
    (() => use(++i))();
  }
})();
```

Compiler evidence:

- Tokens include the loop update `++i` and the call argument `++i`.
- AST construction succeeds.
- The for-loop update `++i` is represented in `For.update`.
- The call argument is represented as `Unary { op: PreIncrement, expr: Ident("i") }`.
- Resolved pipeline fails in builtin/name resolution before lowering.
- Visible symbols include ambient function `use` and loop binding `i`.

TypeScript oracle evidence:

```text
TypeScript reports no diagnostics for the representative file.
```

TypeScript AST evidence at the failing expression:

```text
ExpressionStatement -> CallExpression -> ParenthesizedExpression -> ArrowFunction -> CallExpression -> PrefixUnaryExpression
```

## Desired final state

Identifier-target prefix update expressions in value positions are represented and lowered with JavaScript-compatible mutation and result-value semantics for the focused call-argument case. The representative reference case should no longer report the for-loop update diagnostic at `use(++i)`.

## Scope

In scope:

- [ ] Support `++i` as a call argument when `i` is a resolved local identifier.
- [ ] Preserve existing for-loop update support from issue `268`.
- [ ] Preserve the diagnostic for non-identifier update targets.
- [ ] Add focused coverage for `let i = 0; use(++i);`.
- [ ] Re-run representative triage and confirm the current issue-268 diagnostic at `use(++i)` is gone.

Out of scope:

- Postfix value-result semantics for `i++` in arbitrary expression positions.
- Member/index update targets such as `obj.x++` or `arr[i]++`.
- BigInt update semantics.
- Full block-scoped closure/lifetime behavior after this update expression advances.

## Affected paths

Expected:

- `crates/frontend/src/ast.rs`
- `crates/frontend/src/parser/expressions_main.rs`
- `crates/ir/src/builtin_resolver.rs`
- `crates/ir/src/lowered/program.rs`
- `crates/ir/src/lowered/types.rs`
- focused fixtures/tests for update expressions in value position

Do not touch:

- Backend closure object ABI.
- General scope-analysis diagnostics.
- Non-identifier update target support.

## Acceptance criteria

- [ ] `let i = 0; use(++i);` no longer reports `issue-268`.
- [ ] The supported `++i` expression mutates `i` before passing the resulting value to `use`.
- [ ] Existing `for (let i = 0; i < n; ++i)` coverage from issue `268` still passes.
- [ ] A non-identifier update target still reports an issue-linked unsupported diagnostic.
- [ ] `python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/blockScopedBindingsReassignedInLoop1.ts` no longer reports the current `issue-268` diagnostic at `use(++i)`.

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run -p ts2wasm-ir update
cargo nextest run -p ts2wasm-cli for_loop_increment_update_fixtures_match_node_output_under_iwasm
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/blockScopedBindingsReassignedInLoop1.ts
```

Impacted commands:

```sh
python scripts/manager.py check-issue-readiness -- --fail-ready-below 80
```

Not run:

- none

## Docs / current-state / issue sync

Final-state docs:

- [ ] not affected

Current state:

- [ ] not affected

Follow-up issues:

- [ ] none

## Notes

Split from generated bucket `1068` on 2026-05-06. The failure message references issue `268`, but completed issue `268` only claims for-loop update-slot support and leaves non-update-slot expression semantics outside its completed slice.

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

- Later triage may expose closure or block-scoped loop reassignment semantics after `use(++i)` is supported.
