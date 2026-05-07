---
id: 5379
title: "Lower array binding object default initializers"
type: feature
area: ir/lowering
class: implementation-ready
priority: P2
depends_on: []
blocks: []
created: 2026-05-07
updated: 2026-05-07
---

## Summary

Lower array destructuring bindings whose default initializer is an object
literal, or keep a precise source-spanned issue-251 diagnostic for the remaining
unsupported runtime slice.

## Problem

`contextualTypingArrayDestructuringWithDefaults.ts` parses these top-level
bindings:

```ts
type I = { a: "a" };
let [ c0 = {a: "a"} ]: [I?] = [];
let [ x1, c1 = {a: "a"} ]: [number, I?] = [1];
let [ c_ = {a: "a"} ]: I[] = [];
```

Name resolution currently rejects the first binding with:

```text
UnsupportedRuntimeSubset: issue-251: only literal default binding initializers are supported in this runtime slice at 58..91
```

TypeScript accepts these bindings and reaches a later TS2322 diagnostic in the
function body.

Problem: array binding patterns with object-literal default initializers are not
lowered, so contextual typing destructuring cases stop at the issue-251 runtime
subset guard.

## Current failure

```sh
env TS2WASM_BINARY=/home/wogikaze/wgkz/ts2wasm/target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/contextualTypingArrayDestructuringWithDefaults.ts
```

Compiler evidence:

- tokens: ok through array binding defaults and tuple annotations
- ast: ok; bindings are represented as names like `[c0 = {a: "a"}]`
- resolved/name resolution: issue-251 at the first array binding default
- TypeScript oracle: reaches later TS2322 in the function body after accepting
  the top-level destructuring defaults

## Desired final state

The representative top-level array binding defaults no longer report the
generic issue-251 literal-default guard. They either lower correctly or report a
narrower diagnostic that names array binding object defaults.

## Scope

In scope:

- [x] Lower `let [c0 = { a: "a" }] = [];` without issue-251.
- [x] Lower an elided/preceded element form such as
  `let [x1, c1 = { a: "a" }] = [1];`.
- [x] Preserve unsupported diagnostics for broader effectful or non-literal
  default binding initializers.
- [x] Add focused coverage for array binding object-literal defaults.

Out of scope:

- Object binding parameter defaults, tracked by issue 5373.
- Full TypeScript contextual typing or tuple optional element semantics.
- Object rest binding and computed binding aliases.

## Affected paths

Expected:

- `crates/ir/src/binding_pattern.rs`
- `crates/ir/src/name_resolver.rs`
- `crates/ir/src/lowered/`
- `crates/cli/tests/common/`
- `fixtures/core-semantics/`

Do not touch:

- backend/runtime ABI unless lowering proves the blocker has advanced there

## Acceptance criteria

- [x] `contextualTypingArrayDestructuringWithDefaults.ts` no longer reports the
  current issue-251 diagnostic at `58..91`.
- [x] A focused fixture covers `[c0 = { a: "a" }] = []`.
- [x] A focused fixture covers `[x1, c1 = { a: "a" }] = [1]`.
- [x] Existing unsupported cases for effectful/default binding initializers
  still report source-spanned issue-251 diagnostics.

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run -p ts2wasm-ir
cargo nextest run -p ts2wasm-cli
env TS2WASM_BINARY=/home/wogikaze/wgkz/ts2wasm/target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/contextualTypingArrayDestructuringWithDefaults.ts
```

Impacted commands:

```sh
env TS2WASM_BINARY=/home/wogikaze/wgkz/ts2wasm/target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/contextualTypingArrayDestructuringWithDefaults.ts --detail --no-dashboard-data
python scripts/manager.py check-issue-readiness -- --fail-ready-below 80
```

## Docs / current-state / issue sync

Final-state docs:

- [x] not affected

Current state:

- [x] not affected

Follow-up issues:

- [x] none

## Notes

Split from
`issues/open/1521-implement-contextualTypingArrayDestructuringWithDefaults.md`.

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
