---
id: 5489
title: "Report mixed exported and local var/function merges"
type: feature
area: frontend/semantics
class: implementation-ready
priority: P1
depends_on: []
blocks: [3596]
created: 2026-05-08
updated: 2026-05-08
---

## Summary

Report a TS2395-equivalent diagnostic when an entry-module local variable
declaration and exported function overload declarations share the same merged
declaration name.

## Problem

`nonMergedOverloads.ts` currently fails with generic duplicate local/function
diagnostics before it can report TypeScript's all-exported-or-all-local merged
declaration rule.

Problem: local `var f` plus exported `function f` overload declarations report
generic duplicate diagnostics instead of TS2395-style mixed export/local merged
declaration diagnostics.

## Current failure

Focused triage:

```sh
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/nonMergedOverloads.ts
```

Current compiler diagnostics:

```text
coverage: DuplicateLocal / duplicate-local
triage: DuplicateLocal duplicate local variable: `f` at 82..90
resolved dump: DuplicateFunction duplicate function definition: `f`
```

TypeScript oracle:

```text
TS2395 Individual declarations in merged declaration 'f' must be all exported or all local.
```

Representative source:

```ts
var f = 10;

export function f();
export function f() {
}
```

## Desired final state

The representative fixture reports a source-spanned TS2395-equivalent
diagnostic for the mixed local/exported merged declarations rather than generic
`DuplicateLocal` or `DuplicateFunction`.

## Scope

In scope:

- [ ] Track exported/local state for same-name entry-module value declarations.
- [ ] Detect local `var` declarations merged with exported function overload
  declarations.
- [ ] Emit TS2395-equivalent diagnostics at the conflicting `f` identifiers.
- [ ] Preserve generic duplicate diagnostics for unrelated same-scope duplicate
  locals that do not form this mixed export/local merge.

Out of scope:

- Namespace-only mixed exported/local vars, tracked by issue 5436.
- Default function/namespace TS2395 diagnostics, tracked by issue 5442.
- Valid top-level function overload implementation grouping, tracked by issue
  5200 except as needed to inspect this representative's exported function
  overload pair.

## Affected paths

Expected:

- `crates/frontend/src/`
- `crates/ir/src/`
- focused CLI or semantic fixture

Do not touch:

- `crates/backend-wasm/`
- runtime ABI

## Acceptance criteria

- [ ] `nonMergedOverloads.ts` no longer reports generic `DuplicateLocal` at
  the `export function` keyword span `82..90`.
- [ ] `nonMergedOverloads.ts` reports a TS2395-equivalent diagnostic for the
  local `var f` and exported `function f` declarations.
- [ ] A focused regression covers `var f = 10; export function f(); export
  function f() {}`.
- [ ] Existing issue 5200 overload fixtures remain on their current path unless
  explicitly advanced by the same implementation.

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run -p ts2wasm-cli -E 'test(function) or test(export)'
```

Impacted commands:

```sh
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/nonMergedOverloads.ts --detail --no-dashboard-data
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/nonMergedOverloads.ts
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

Split from `issues/done/3596-implement-nonMergedOverloads.md`.

Related but not duplicates:

- `issues/open/5436-report-mixed-exported-local-namespace-vars.md` covers
  namespace member var declarations.
- `issues/open/5442-report-mixed-default-function-namespace-merge-diagnostic.md`
  covers default exported function plus local namespace merges.
- `issues/open/5200-validate-top-level-function-overload-implementations.md`
  covers valid non-ambient function overload groups without mixed export/local
  declarations.

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

- After TS2395 parity is implemented, the fixture may still need normal
  top-level overload grouping behavior from issue 5200.
