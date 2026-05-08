---
id: 5470
title: "Support array spread over array fallback expressions"
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

Support array literal spread when the operand is a simple array fallback
expression such as `e || []`, where `e` is a local known to hold a dense array.

Split from generated bucket `3525`.

## Problem

`noCrashOnNoLib.ts` parses and resolves through an exported function with a
local array and a while loop:

```ts
export function f() {
    let e: {}[] = [];
    while (true) {
      e = [...(e || [])];
    }
}
```

The compiler reaches lowering and reports:

```text
UnsupportedSyntax: issue-274: array literal spread is only supported for literal arrays and ASCII literal-derived strings in this milestone
```

Problem: the array spread lowering recognizes dense array literals and dense
array locals, but does not recognize a fallback expression whose branches are a
dense array local and an empty array literal.

## Current failure

Reproduction:

```sh
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/noCrashOnNoLib.ts
```

Observed result:

```text
Diagnostic: UnsupportedSyntax / parser-or-frontend-unsupported
Feature label: spread
Message: issue-274: array literal spread is only supported for literal arrays and ASCII literal-derived strings in this milestone
```

Compiler evidence:

```text
tokens: ok through export function, typed let e, while loop, and `[...(e || [])]`
ast: ok; array literal has Spread(Binary(Ident e, Or, Array []))
resolved: ok through builtins
lower_program: issue-274 array literal spread boundary
```

TypeScript oracle:

```text
ok; diagnostics=[]
hint: e has type {}[]
```

## Desired final state

The compiler lowers `[...(e || [])]` through the existing dense array spread
path when `e` is a local dense array value and the fallback branch is `[]`. The
representative reference should advance past the current issue-274 diagnostic
or build successfully.

## Scope

In scope:

- [ ] Recognize `Ident || []` spread operands when the identifier is a known dense array local.
- [ ] Lower `[...(arrayLocal || [])]` using the existing dense array concat/copy path.
- [ ] Add focused coverage for `let e = []; e = [...(e || [])];`.
- [ ] Re-triage `noCrashOnNoLib.ts` and record any next diagnostic.

Out of scope:

- General iterator protocol integration, tracked by `issues/open/353-spread-iterator-protocol.md`.
- Broad spread meta tracking, tracked by `issues/open/274-implement-spread-operator.md`.
- Array spread over narrowed array-typed parameters, tracked by `issues/open/5456-support-array-spread-over-narrowed-array-typed-parameters.md`.
- Arbitrary boolean expressions, non-array fallbacks, Map/custom iterables, and object spread.

## Affected paths

Expected:

- `crates/ir/src/`
- `crates/cli/tests/`
- focused spread/reference fixtures

Do not touch:

- iterator protocol runtime helpers
- object spread or call spread paths
- parser code unless the AST evidence changes

## Acceptance criteria

- [ ] `[...(e || [])]` no longer reports issue-274 when `e` is a dense array local.
- [ ] A focused fixture covers a local array fallback expression inside array literal spread.
- [ ] Existing dense array local, literal array, Set local, and string spread slices remain passing.
- [ ] Non-array fallback expressions still produce clear unsupported diagnostics.
- [ ] `noCrashOnNoLib.ts` no longer reports the current issue-274 diagnostic for `[...(e || [])]`.

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run -p ts2wasm-cli -E 'test(spread) or test(array)'
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/noCrashOnNoLib.ts
```

Impacted commands:

```sh
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/noCrashOnNoLib.ts --detail --no-dashboard-data
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
