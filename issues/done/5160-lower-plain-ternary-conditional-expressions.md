---
id: 5160
title: "Lower plain ternary conditional expressions"
type: feature
area: frontend/semantics
class: implementation-ready
priority: P2
depends_on: []
blocks: []
created: 2026-05-06
updated: 2026-05-06
---

## Summary

The parser already creates `Expr::Ternary` for conditional expressions, but builtin resolution rejects every ternary with `UnsupportedSyntax: ternary operator not yet supported`. This blocks `bestChoiceType.ts` after the earlier `|| []` and `.map(...)` expressions parse successfully.

## Problem

Problem: `reference/typescript/tests/cases/compiler/bestChoiceType.ts` currently stops in builtin resolution on `let y = x ? x : [];`, even though TypeScript accepts the file with no diagnostics.

## Current failure

Reference triage:

```sh
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/bestChoiceType.ts
```

Current compiler diagnostic:

```text
UnsupportedSyntax: ternary operator not yet supported at 317..327
```

Representative source:

```ts
function f2() {
    let x = ''.match(/ /);
    let y = x ? x : [];
    let z = y.map(s => s.toLowerCase());
}
```

Current compiler evidence:

- Tokens succeed.
- AST succeeds and records `x ? x : []` as a ternary expression.
- Name resolution reaches bindings `x` and `y`.
- Builtin resolution rejects `Expr::Ternary` in `crates/ir/src/builtin_resolver.rs`.

TypeScript oracle evidence:

```text
bestChoiceType.ts: no diagnostics
y: RegExpMatchArray | never[]
z: any[]
```

TypeScript AST path at the failing expression:

```text
FunctionDeclaration -> Block -> FirstStatement -> VariableDeclarationList -> VariableDeclaration -> ConditionalExpression -> Identifier(x)
```

## Desired final state

Plain ternary conditional expressions lower through resolver/IR/backend for the supported runtime subset. The representative `bestChoiceType.ts` case should advance past the current ternary unsupported diagnostic.

## Scope

In scope:

- [x] Resolve `Expr::Ternary` into an IR form or existing conditional lowering that preserves JavaScript truthiness of the condition.
- [x] Lower simple expression branches such as `x ? x : []`.
- [x] Add focused coverage for `let y = x ? x : []` and the existing `fixtures/core-semantics/ternary.ts` shape.
- [x] Re-run the representative reference triage and confirm the current `ternary operator not yet supported` blocker is gone.

Out of scope:

- Nullish coalescing and mixed `??` ternary precedence; test262 coalesce metadata remains separate.
- Contextual TypeScript best-choice type inference beyond preserving runtime behavior.
- Source map validation for ternary expressions.
- Tail-call optimization behavior for conditional expressions.

## Affected paths

Expected:

- `crates/ir/src/builtin_resolver.rs`
- `crates/ir/src/builtin_resolved.rs`
- `crates/ir/src/lowered/`
- `crates/backend-wasm/src/`
- `crates/frontend/src/parser/tests.rs`
- `fixtures/core-semantics/ternary.ts`
- `crates/cli/tests/common/m2_node_diff_fixture_tests.rs`

Do not touch:

- TypeScript checker/oracle scripts unless triage output changes unexpectedly.

## Acceptance criteria

- [x] `fixtures/core-semantics/ternary.ts` no longer reports `ternary operator not yet supported` and matches Node output.
- [x] `python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/bestChoiceType.ts` no longer reports `UnsupportedSyntax: ternary operator not yet supported`.
- [x] A focused unit or fixture test covers `let y = x ? x : [];`.
- [x] Existing parser ternary AST coverage remains passing.

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run -p ts2wasm-frontend
cargo nextest run -p ts2wasm-ir
cargo nextest run -p ts2wasm-cli ternary
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/bestChoiceType.ts
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

- [x] created/updated: `issues/open/5208-support-regexp-match-fallback-array-map-receiver.md`

## Notes

Split from generated bucket `1043` on 2026-05-06. Other generated ternary buckets may become duplicates once this implementation-ready slice lands, but they still need current triage before closure.

## Completion evidence

Fill only when moving to `done/`.

Commits:

- this commit: frontend: lower plain ternary expressions

Validation result:

```text
command: cargo fmt --all --check
result: pass
date: 2026-05-06

command: cargo nextest run -p ts2wasm-frontend
result: pass (172 passed)
date: 2026-05-06

command: cargo nextest run -p ts2wasm-ir
result: pass (26 passed)
date: 2026-05-06

command: cargo nextest run -p ts2wasm-cli ternary
result: pass (4 passed)
date: 2026-05-06

command: cargo nextest run -p ts2wasm-cli --test m2_node_diff ternary
result: pass (2 passed)
date: 2026-05-06

command: python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/bestChoiceType.ts
result: pass for issue 5160; ternary unsupported blocker is gone, next blocker split to issue 5208
date: 2026-05-06

command: python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/bestCommonTypeWithContextualTyping.ts
result: pass; BuildPass, ternary expression resolves and lowers
date: 2026-05-06
```

Remaining risks:

- issue 5208 owns the next `bestChoiceType.ts` receiver-classification blocker after ternary lowering.
