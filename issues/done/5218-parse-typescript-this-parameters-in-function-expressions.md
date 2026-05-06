---
id: 5218
title: "Parse TypeScript this parameters in function expressions"
type: feature
area: frontend/parser
class: implementation-ready
priority: P1
depends_on: []
blocks: []
created: 2026-05-06
updated: 2026-05-06
---

## Summary

Accept erased TypeScript `this` parameters in function expression parameter
lists, such as `function (this: any) { ... }`.

## Problem

After unsigned 32-bit hex literal lexing is fixed for
`binaryArithmeticControlFlowGraphNotTooLarge.ts`, the next frontend blocker is
the parser treating the contextual `this` parameter as an ordinary binding
identifier.

Problem: function expressions with TypeScript `this` parameters currently fail with `issue-247: expected binding identifier or pattern, got Some(This)`.

## Current failure

Reproduction:

```sh
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/binaryArithmeticControlFlowGraphNotTooLarge.ts
```

Current diagnostic after issue 5171:

```text
UnsupportedSyntax: issue-247: expected binding identifier or pattern, got Some(This) at 118..122
```

Representative source:

```ts
const foo = function (this: any) {
    var a, b, c, d, ab, bc, cd, da, blocks = this.blocks;
};
```

TypeScript oracle evidence accepts `this: any` as a function parameter metadata
slot and does not bind it as a runtime parameter.

## Desired final state

The parser erases TypeScript `this` parameters in supported function expression
and function declaration parameter lists, while preserving runtime parameter
order for the remaining parameters.

## Scope

In scope:

- [x] Parse and erase leading `this: Type` parameters in function expressions
- [x] Parse and erase leading `this: Type` parameters in function declarations if the same parser path is shared
- [x] Preserve rejection for non-leading `this` parameters if unsupported
- [x] Add focused parser tests for `function (this: any) {}` and a remaining runtime parameter
- [x] Re-run the representative triage and confirm it advances past the current issue-247 diagnostic

Out of scope:

- JSDoc semantics
- Type-checking call receivers from `this` parameter annotations
- Broad control-flow or bitwise lowering for `binaryArithmeticControlFlowGraphNotTooLarge.ts`

## Affected paths

Expected:

- `crates/frontend/src/parser/`
- `crates/frontend/src/parser/tests.rs`

Do not touch:

- backend lowering for bitwise operators
- control-flow graph limits

## Acceptance criteria

- [x] `function (this: any) {}` parses without creating a runtime parameter named `this`
- [x] `function (this: any, value: number) { return value; }` preserves `value` as the first runtime parameter
- [x] `binaryArithmeticControlFlowGraphNotTooLarge.ts` no longer reports `issue-247: expected binding identifier or pattern, got Some(This)`
- [x] Issue index and readiness checks pass

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run -p ts2wasm-frontend this_parameter
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/binaryArithmeticControlFlowGraphNotTooLarge.ts
python scripts/manager.py update-issue-index --check
python scripts/manager.py check issues
python scripts/manager.py check issue-readiness -- --fail-ready-below 80
git diff --check
```

Impacted commands:

```sh
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/APISample_jsdoc.ts
```

Not run:

- none

## Docs / current-state / issue sync

Final-state docs:

- [x] not affected

Current state:

- [x] not affected

Follow-up issues:

- [x] created: `issues/open/5219-support-explicit-this-parameter-function-expression-lowering.md`

## Notes

Split while closing issue 5171. Existing generated bucket 544 contains the same
`Some(This)` shape in a JSDoc reference file, but it is a blocked broad bucket;
this issue is the executable parser slice.

## Completion evidence

Fill only when moving to `done/`.

Commits:

- pending local commit

Validation result:

```text
command: cargo fmt --all --check
result: pass
date: 2026-05-06

command: cargo nextest run -p ts2wasm-frontend this_parameter
result: pass
date: 2026-05-06

command: python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/binaryArithmeticControlFlowGraphNotTooLarge.ts
result: pass for issue 5218; advanced past issue-247 Some(This) to issue-062e, tracked by issue 5219
date: 2026-05-06

command: python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/APISample_jsdoc.ts
result: pass for issue 5218; no longer reports issue-247 Some(This)
date: 2026-05-06
```

Remaining risks:

- Runtime/lowering support for explicit-this function expressions is tracked by issue 5219.
