---
id: 5378
title: "Report mixed ambient function overload diagnostics"
type: feature
area: frontend/resolver
class: implementation-ready
priority: P1
depends_on: []
blocks: []
created: 2026-05-07
updated: 2026-05-07
---

## Summary

Report the TypeScript mixed ambient/non-ambient function overload diagnostic
instead of treating the implementation as a generic duplicate local.

## Problem

`contextualTyping.ts` parses successfully through its contextual typing cases,
then reaches:

```ts
declare function EF1(a:number, b:number):number;

function EF1(a,b) { return a+b; }
```

The resolver currently reports:

```text
DuplicateLocal: duplicate local variable: `EF1` at 5033..5064
```

TypeScript reports TS2384, `Overload signatures must all be ambient or
non-ambient.`, on the ambient overload signature.

Problem: mixed ambient/non-ambient top-level function overload groups report a
generic duplicate-local diagnostic instead of a source-spanned ambientness
diagnostic.

## Current failure

```sh
env TS2WASM_BINARY=/home/wogikaze/wgkz/ts2wasm/target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/contextualTyping.ts
```

Compiler evidence:

- tokens: ok
- ast: ok through the `EF1` declarations and later class declarations
- visible symbols include bodyless ambient function `EF1`, concrete function
  `EF1`, and binding `efv`
- resolved/name resolution: generic `DuplicateLocal` at the concrete `EF1`
  implementation
- TypeScript oracle: TS2384 at the ambient `EF1` signature

## Desired final state

The representative mixed ambient/non-ambient overload group no longer reports a
generic duplicate-local diagnostic. It reports a source-spanned diagnostic that
matches TypeScript's ambientness rule before later contextual typing diagnostics
are considered.

## Scope

In scope:

- [x] Detect a top-level `declare function f(...); function f(...) { ... }`
  mixed ambient/non-ambient overload group.
- [x] Report a source-spanned diagnostic at the offending overload identifier.
- [x] Preserve valid non-ambient overload grouping for issue 5200.
- [x] Preserve valid ambient overload grouping for issue 5226.
- [x] Add a focused regression for `declare function f(...); function f(...)`.

Out of scope:

- Valid top-level non-ambient overload implementations, tracked by issue 5200.
- Valid ambient overload declaration sets, tracked by issue 5226.
- Var/function duplicate identifier diagnostics, tracked by issue 5307.
- Later contextual typing behavior in `contextualTyping.ts`.

## Affected paths

Expected:

- `crates/frontend/src/`
- `crates/ir/src/`
- `crates/cli/tests/`
- `fixtures/`

Do not touch:

- backend/runtime lowering
- unrelated import/export syntax handling

## Acceptance criteria

- [x] `contextualTyping.ts` no longer reports generic `DuplicateLocal` for
  `EF1`.
- [x] A focused fixture reports a mixed ambient/non-ambient overload diagnostic
  for `declare function f(a: number): number; function f(a) { return a; }`.
- [x] Valid non-ambient overload implementation fixtures from issue 5200 remain
  on their existing path.
- [x] Valid ambient overload declaration fixtures from issue 5226 remain on
  their existing path.

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run -p ts2wasm-cli -E 'test(function) or test(overload) or test(ambient)'
env TS2WASM_BINARY=/home/wogikaze/wgkz/ts2wasm/target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/contextualTyping.ts
```

Impacted commands:

```sh
env TS2WASM_BINARY=/home/wogikaze/wgkz/ts2wasm/target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/contextualTyping.ts --detail --no-dashboard-data
```

## Docs / current-state / issue sync

Final-state docs:

- [x] not affected

Current state:

- [x] not affected

Follow-up issues:

- [x] none

## Notes

Split from `issues/open/1518-implement-contextualTyping-import-export.md`.

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
