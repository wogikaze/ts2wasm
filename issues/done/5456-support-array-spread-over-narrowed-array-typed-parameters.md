---
id: 5456
title: "Support array spread over narrowed array-typed parameters"
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

Support array literal spread over a function parameter whose TypeScript type is
known to be array-like after a `typeof value !== "undefined"` guard.

Split from generated bucket
`issues/done/3467-implement-narrowingTypeofUndefined-name-resolution.md`.

## Problem

Problem: `narrowingTypeofUndefined2.ts` parses, resolves, and lowers through
the `typeof arg !== "undefined"` guarded block until it reaches:

```ts
const m = [...arg];
```

The parameter is constrained as `T extends Array<unknown> | undefined`, and
TypeScript narrows it to an array-compatible value inside the guarded block.
The current compiler still reports the generic issue-274 spread boundary:

```text
UnsupportedSyntax: issue-274: array literal spread is only supported for literal arrays and ASCII literal-derived strings in this milestone
```

## Current failure

```sh
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/narrowingTypeofUndefined2.ts
```

Representative source:

```ts
declare function takeArray(arr: Array<unknown>): void;

function fn<T extends Array<unknown> | undefined>(arg: T) {
    if (typeof arg !== "undefined") {
        takeArray(arg);
        const n: Array<unknown> = arg;
        for (const p of arg) {  }
        const m = [...arg];
    }
}
```

Compiler evidence:

```text
tokens: ok
ast: ok; function fn, typeof guard, call, typed const, for-of, and array spread parse
resolved: ok through builtins
lower_program: issue-274 array literal spread boundary for [...arg]
TypeScript oracle: ok, diagnostics=[]
```

## Desired final state

The compiler classifies `arg` as an array-like spread operand inside the
`typeof arg !== "undefined"` branch and lowers `[...arg]` through the existing
supported array-spread path. The representative reference should advance past
the current issue-274 spread diagnostic to the next narrowing, generic, or
runtime blocker.

## Scope

In scope:

- [ ] Preserve or infer enough parameter type information for
  `T extends Array<unknown> | undefined`.
- [ ] Use the `typeof arg !== "undefined"` guard to avoid treating `arg` as
  possibly undefined at the spread site.
- [ ] Lower `[...arg]` when `arg` is a parameter known to be array-like.
- [ ] Add focused coverage for `function fn<T extends Array<unknown> |
  undefined>(arg: T) { if (typeof arg !== "undefined") { const m = [...arg]; } }`.
- [ ] Re-run the representative triage and record any later blocker.

Out of scope:

- General iterator protocol integration, tracked by
  `issues/open/353-spread-iterator-protocol.md`.
- Broad spread operator tracking, tracked by
  `issues/open/274-implement-spread-operator.md`.
- Custom iterable, generator, Map, or non-array parameter spread.
- Full TypeScript generic constraint solving beyond the array-or-undefined
  shape in the representative.
- Parser support for spread arguments in `new` expressions, tracked by
  `issues/open/5241-parse-spread-arguments-in-new-expressions.md`.

## Affected paths

Expected:

- `crates/ir/src/`
- `crates/frontend/src/`
- focused CLI/reference fixtures

Do not touch:

- iterator protocol runtime helpers unless a focused fixture proves that this
  array-typed parameter case cannot use existing array spread lowering
- unrelated object spread or call spread paths

## Acceptance criteria

- [ ] `[...arg]` no longer reports issue-274 when `arg` is narrowed from
  `T extends Array<unknown> | undefined` by `typeof arg !== "undefined"`.
- [ ] A focused fixture covers array literal spread over a narrowed array-typed
  parameter.
- [ ] Existing literal array, dense array local, Set local, and string spread
  slices remain passing.
- [ ] Custom iterable and generator spread continue to route to issue 353 or
  the existing iterator-protocol diagnostic.
- [ ] `narrowingTypeofUndefined2.ts` no longer reports the current issue-274
  diagnostic for `const m = [...arg]`.

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run -p ts2wasm-cli -E 'test(spread) or test(narrow) or test(array)'
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/narrowingTypeofUndefined2.ts
```

Impacted commands:

```sh
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/narrowingTypeofUndefined2.ts --detail --no-dashboard-data
python scripts/manager.py update-issue-index --check
python scripts/manager.py check-issue-health
python scripts/manager.py check-issue-readiness -- --fail-ready-below 80
git diff --check
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

Related but distinct:

- Issue 274 is a broad spread meta issue and should not be selected directly
  when this narrow array-typed parameter slice exists.
- Issue 353 owns general iterator protocol semantics for custom iterables,
  generators, and Map/custom spread operands.

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
