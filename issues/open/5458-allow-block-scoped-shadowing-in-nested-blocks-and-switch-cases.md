---
id: 5458
title: "Allow block-scoped shadowing in nested blocks and switch cases"
type: feature
area: frontend/resolver
class: implementation-ready
priority: P2
depends_on: []
blocks: []
created: 2026-05-08
updated: 2026-05-08
---

## Summary

Allow `let` declarations in nested block scopes and switch case scopes to
shadow outer `var` declarations without reporting `DuplicateLocal`.

Split from generated bucket
`issues/open/3475-implement-nestedBlockScopedBindings.md`.

## Problem

Problem: `nestedBlockScopedBindings11.ts` parses successfully, but AST
validation reports a false duplicate for a `let x` inside a nested block after
an outer `var x`.

The representative also contains a switch case with outer `var y` and inner
`let y`; both are accepted by TypeScript and should be represented as distinct
block-scoped bindings.

Current compiler diagnostic:

```text
DuplicateLocal: duplicate local binding: `x` at 35..41
```

## Current failure

```sh
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/nestedBlockScopedBindings11.ts
```

Representative source:

```ts
var x;
{
    let x;
    () => x;
}

var y;
switch (1) {
    case 1:
        let y;
        () => y;
        break;
}
```

Compiler evidence:

```text
tokens: ok; Var x, block Let x, arrow capture, Switch, case Let y, arrow capture
ast: ok; the inner block and switch case statements are preserved
resolved: fails during validate_ast
diagnostic: DuplicateLocal / compiler-diagnostic
message: duplicate local binding: `x` at 35..41
TypeScript oracle: ok, diagnostics=[]
```

Coverage evidence:

```sh
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/nestedBlockScopedBindings11.ts --detail --no-dashboard-data
```

```text
executed=1
build_pass=0
unsupported=1
unsupported_diagcodes=DuplicateLocal:1
unsupported_features=duplicate-local:1
```

## Desired final state

The frontend distinguishes nested block and switch-case lexical scopes during
duplicate-local validation and name resolution. The representative should
advance past the false `DuplicateLocal` for `x`; if it then reaches the switch
case `y` or arrow-capture lowering boundary, that next blocker should be
recorded separately.

## Scope

In scope:

- [ ] Treat a top-level `var x` and a nested block `let x` as distinct scopes
  for duplicate-local validation.
- [ ] Treat a top-level `var y` and a switch case `let y` as distinct scopes
  for duplicate-local validation.
- [ ] Preserve valid same-scope `let` / `const` duplicate diagnostics.
- [ ] Add focused validation or parser/resolver coverage for the representative
  nested block and switch case shapes.
- [ ] Re-run `nestedBlockScopedBindings11.ts` triage and record any next
  blocker.

Out of scope:

- General duplicate-local diagnostic formatting, tracked by broader duplicate
  local issues such as `issues/open/343-implement-duplicate-local-detection.md`.
- Compatible same-scope `var` redeclarations, tracked by
  `issues/open/5162-allow-compatible-var-redeclarations.md`.
- Block-local class declaration shadowing, tracked by
  `issues/open/5249-scope-block-local-class-declarations.md`.
- Arrow closure environment lowering beyond the existing arrow-function scope
  support.

## Affected paths

Expected:

- `crates/frontend/src/`
- `crates/ir/src/`
- focused parser/resolver or CLI tests

Do not touch:

- backend/runtime code unless the representative advances past validation and
  proves a lowering-owned blocker
- broad duplicate-local issue queue files except for lifecycle synchronization

## Acceptance criteria

- [ ] `var x; { let x; () => x; }` no longer reports `DuplicateLocal`.
- [ ] `var y; switch (1) { case 1: let y; () => y; break; }` no longer
  reports `DuplicateLocal`.
- [ ] A focused test preserves same-scope `let x; let x;` or equivalent
  duplicate-local diagnostics.
- [ ] `nestedBlockScopedBindings11.ts` no longer reports
  `DuplicateLocal: duplicate local binding: x` at the inner block `let x`.
- [ ] If the representative advances to a new blocker, the issue records that
  blocker before closure.

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run -p ts2wasm-frontend duplicate
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/nestedBlockScopedBindings11.ts
```

Impacted commands:

```sh
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/nestedBlockScopedBindings11.ts --detail --no-dashboard-data
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

- `issues/open/3164-implement-letDeclarations-duplicate-local.md` is a broad
  generated bucket for `letDeclarations-*` duplicate-local cases, not this
  nested block/switch representative.
- `issues/open/343-implement-duplicate-local-detection.md` owns broad duplicate
  diagnostics for actual same-scope duplicates.

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
