---
id: 5415a
title: "Support identifier update expressions in value positions"
type: feature
area: ir/lowering
class: implementation-ready
priority: P1
depends_on: []
blocks: []
created: 2026-05-08
updated: 2026-05-08
---

## Summary

Support identifier-target `x++`, `x--`, `++x`, and `--x` when the expression's
value is used, such as in `y <= x++`.

## Problem

Issue 268 implemented update operators for for-loop update slots where the
expression result is unused. `moduleExportsUnaryExpression.ts` uses update
operators inside comparisons and returns:

```ts
if (y <= x++) return y <= x++;
if (y <= ++x) return y <= ++x;
```

Current diagnostic:

```text
UnsupportedSyntax: issue-268: for-loop increment/decrement updates currently require an identifier target
```

Problem: identifier update expressions are parser-accepted in value positions,
but resolver/lowering still routes them through the narrow for-loop diagnostic
instead of implementing JavaScript result-value semantics.

## Current failure

```sh
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/moduleExportsUnaryExpression.ts
```

Evidence:

```text
source: if (y <= x++) return y <= x++;
tokens: ok
ast: ok; Binary(LessEqual, Ident("y"), Unary(Increment, Ident("x")))
smart triage: issue-268 at the postfix `x++`
TypeScript oracle: ok, diagnostics=[]
```

## Desired final state

Identifier-target update expressions in value positions mutate the local and
produce the correct JavaScript value. Postfix forms return the old value;
prefix forms return the new value.

## Scope

In scope:

- [ ] Support `x++` and `x--` as operands in binary expressions and return expressions.
- [ ] Support `++x` and `--x` as operands in binary expressions and return expressions.
- [ ] Preserve existing expression-statement and for-loop update behavior.
- [ ] Keep non-identifier update targets rejected with an issue-linked diagnostic.
- [ ] Re-run the representative reference and record the next blocker.

Out of scope:

- Member or element update targets such as `obj.x++` or `arr[i]++`.
- BigInt update semantics.
- Captured-variable closure semantics beyond this local identifier slice.
- Named export rewrite behavior after the update-expression blocker advances.

## Affected paths

Expected:

- `crates/ir/src/builtin_resolver.rs`
- `crates/ir/src/lowered/program.rs`
- `crates/ir/src/lowered/types.rs`
- focused IR/CLI tests

Do not touch:

- backend ABI unless a new lowered expression form requires emission support
- broad module graph resolution

## Acceptance criteria

- [ ] `let x = 1; let y = x++;` observes `y === 1` and `x === 2`.
- [ ] `let x = 1; let y = ++x;` observes `y === 2` and `x === 2`.
- [ ] `if (y <= x++) return y <= x++;` no longer reports issue-268.
- [ ] Existing for-loop update fixtures from issue 268 still pass.
- [ ] Non-identifier update targets still report a clear unsupported diagnostic.
- [ ] `moduleExportsUnaryExpression.ts` no longer reports the current issue-268 diagnostic.

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run -p ts2wasm-ir update
cargo nextest run -p ts2wasm-cli for_loop_increment_update_fixtures_match_node_output_under_iwasm
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/moduleExportsUnaryExpression.ts
```

Impacted commands:

```sh
python scripts/manager.py check-issue-readiness -- --fail-ready-below 80
```

## Docs / current-state / issue sync

Final-state docs: not affected.
Current state: not affected.
Follow-up issues: none.

## Notes

Split from generated bucket
`issues/open/3339-implement-moduleExportsUnaryExpression.md`.

Related but not duplicate:

- `issues/open/5181-support-prefix-update-expressions-in-call-arguments.md`
  handles prefix `++i` in call arguments only. This issue owns the first current
  blocker in 3339: postfix `x++` in a value-producing binary expression.

## Completion evidence

Fill only when implemented.
