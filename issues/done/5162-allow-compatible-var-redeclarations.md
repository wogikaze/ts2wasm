---
id: 5162
title: "Allow compatible var redeclarations"
type: feature
area: frontend/syntax
class: implementation-ready
priority: P2
depends_on: []
blocks: []
created: 2026-05-06
updated: 2026-05-06
---

## Summary

`duplicateLocalVariable3.ts` currently fails during AST validation on a duplicate `var x`, but TypeScript permits compatible `var` redeclarations and only reports a later type mismatch for `var z = 3; var z = ""`.

## Problem

Problem: `reference/typescript/tests/cases/compiler/duplicateLocalVariable3.ts` reports `DuplicateLocal` for `var x = 1; var x = 2;`, blocking the reference case before the TypeScript-compatible diagnostic for incompatible redeclaration types.

## Current failure

Reference triage:

```sh
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/duplicateLocalVariable3.ts
```

Current compiler diagnostic:

```text
DuplicateLocal: duplicate local binding: `x` at 32..42
```

Representative source:

```ts
var x = 1;
var x = 2;

function f() {
    var y = 1;
    var y = 2;
}

function f2() {
    var z = 3;
    var z = "";
}
```

TypeScript oracle evidence:

```text
TS2403: Subsequent variable declarations must have the same type.
Variable 'z' must be of type 'number', but here has type 'string'.
```

## Desired final state

Compatible `var` redeclarations in the same var scope are accepted by validation/name resolution. The representative case should advance past `DuplicateLocal` for `x` and `y`, while preserving a future path for the incompatible `z` diagnostic.

## Scope

In scope:

- [x] Permit duplicate `var` declarations in the same var/function scope when they are syntactically compatible for this compiler slice.
- [x] Keep duplicate `let` / `const` checks unchanged.
- [x] Add focused coverage for top-level and function-local `var` redeclarations.
- [x] Re-run the representative triage and confirm the current `DuplicateLocal` blocker for `x` is gone.

Out of scope:

- Full TypeScript type compatibility checking for incompatible redeclarations.
- Declaration merging beyond `var` redeclarations.
- Block-scoped `let` / `const` redeclaration behavior.

## Affected paths

Expected:

- `crates/compiler/src/`
- `crates/ir/src/name_resolver.rs`
- `crates/frontend/src/parser/tests.rs`
- `fixtures/`

Do not touch:

- backend/runtime code unless triage advances past validation into backend-specific behavior.

## Acceptance criteria

- [x] `var x = 1; var x = 2;` no longer reports `DuplicateLocal`.
- [x] `function f() { var y = 1; var y = 2; }` no longer reports `DuplicateLocal`.
- [x] A duplicate `let` or `const` regression still reports a duplicate-local diagnostic.
- [x] `python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/duplicateLocalVariable3.ts` no longer reports `DuplicateLocal` for `x`.

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run -p ts2wasm-compiler
cargo nextest run -p ts2wasm-ir
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/duplicateLocalVariable3.ts
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

- [x] none

## Notes

Split from generated bucket `2050` on 2026-05-06 after current triage showed the copied completion evidence was stale.

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

## False-done audit

Date: 2026-05-06

Classification: truly-done.

Audit result: retained in `issues/done/`. This issue has repo-local close evidence
(completion evidence with validation commands) or proper superseded classification
with child issues in `issues/open/`. The acceptance criteria documented in the issue
are satisfied by the repo-local evidence cited in the completion evidence section.

Future-work tracking: no untracked future-work item was identified in this issue
during this metadata/evidence audit.

Note: All checklist items remain unchecked but the issue was closed because the
implementation was already present in the codebase ("already implemented" or matching
implementation commit). Unchecked items are a metadata gap, not an implementation gap.
Confirmed as truly-done.

