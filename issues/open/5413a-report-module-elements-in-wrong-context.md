---
id: 5413a
title: "Report nested namespace wrong-context diagnostic"
type: bug
area: frontend/syntax
class: implementation-ready
priority: P1
depends_on: []
blocks: []
created: 2026-05-08
updated: 2026-05-08
---

## Summary

Report a specific wrong-context diagnostic for `namespace M { }` when it appears
inside a nested statement block.

## Problem

`moduleElementsInWrongContext3.ts` currently builds successfully, but TypeScript
first reports TS1235 for the nested namespace declaration:

```ts
namespace P {
    {
        namespace M { }
    }
}
```

Problem: the frontend accepts a nested namespace declaration in a statement
block where TypeScript reports a wrong-context diagnostic.

## Current failure

Reference triage:

```sh
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/moduleElementsInWrongContext3.ts
```

Current compiler result:

```text
BuildPass: ts2wasm build succeeded
```

TypeScript oracle first diagnostic:

```text
TS1235: A namespace declaration is only allowed at the top level of a namespace or module.
```

## Desired final state

The frontend rejects `namespace M { }` inside a nested statement block and
preserves the span of the offending `namespace` keyword.

## Scope

In scope:

- [ ] Detect `namespace Name {}` inside a nested statement block.
- [ ] Preserve the span of the nested `namespace` keyword.
- [ ] Add focused diagnostic coverage for the reduced source or
      `moduleElementsInWrongContext3.ts`.

Out of scope:

- CommonJS `export =` parsing or runtime/module loading.
- Wrong-context diagnostics for imports, static exports, default exports, or
  ambient modules.
- Namespace merge semantics outside this nested-block check.

## Affected paths

Expected:

- `crates/frontend/src/parser/`
- `crates/frontend/src/`
- focused frontend/CLI diagnostic tests

Do not touch:

- backend emit
- package/module resolution
- runtime ABI

## Acceptance criteria

- [ ] `moduleElementsInWrongContext3.ts` no longer silently builds; its first
      observable result is a nested namespace wrong-context diagnostic.
- [ ] A focused test preserves the span for the nested `namespace` keyword.
- [ ] Valid top-level namespace declarations still parse.

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run -p ts2wasm-frontend
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/moduleElementsInWrongContext3.ts
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

Split from generated bucket
`issues/open/3335-implement-moduleElementsInWrongContext.md`. The bare-block
and function-body variants still stop at `export = M;`; issue 5186 owns that
parser boundary. Later import/export wrong-context diagnostics should be split
after this first diagnostic lands.

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
