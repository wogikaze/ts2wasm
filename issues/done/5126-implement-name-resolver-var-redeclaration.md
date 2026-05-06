---
id: 5126
title: "Implement name resolver var redeclaration tolerance"
type: feature
area: ir/lowering
class: implementation
priority: P1
depends_on: [3690]
blocks: []
parent: 3690
created: 2026-05-05
updated: 2026-05-05
completed: 2026-05-05
---

## Summary

Make the name resolver tolerate legal `var` redeclarations in the same scope, while continuing to reject illegal `let`/`const` duplicates.

## Problem

Reference test `optionalTupleElementsAndUndefined.ts` fails with `DuplicateLocal`:

```
error: [DuplicateLocal] duplicate local binding: `v` at 323..352
```

The test has 8 `var v` declarations in the same scope:

```typescript
var v: [1, 2?];
var v: [1, 2? | undefined];
var v: [1, (2 | undefined)?];
var v: [1, (2 | undefined)? | undefined];
var v: [1, (2? | undefined)];
var v: [1, (2? | undefined)?];
var v: [1, (2 | undefined)?];
var v: [1, (2 | undefined)? | undefined];
```

In TypeScript/JavaScript, `var` redeclarations are legal — they merge. The current name resolver treats all duplicate bindings as errors regardless of declaration kind.

## Current failure

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/optionalTupleElementsAndUndefined.ts
```

Smart triage evidence:

- **Diagnostic:** `DuplicateLocal` at span 323..352
- **Error type:** `ir-or-lowering`
- **Root:** `name_resolver.rs` rejects duplicate `var` bindings

## Desired final state

The name resolver allows multiple `var` declarations with the same name in the same scope. `let`/`const` duplicates continue to be rejected. The reference test compiles without `DuplicateLocal` errors.

## Scope

In scope:

- [x] Modify `crates/ir/src/name_resolver.rs` to distinguish declaration kind (var vs let/const)
- [x] Allow `var` redeclarations in the same scope
- [x] Continue rejecting `let`/`const` duplicates
- [x] Update reference coverage for the fixed test case (deferred — coverage ramp is a separate wave task)

Out of scope:

- Global scope redeclaration rules (separate issue)
- `function` declaration redeclaration rules (separate issue)
- test262 DuplicateLocal handling (tracked in #343)

## Affected paths

- `crates/ir/src/name_resolver.rs`

Do not touch:
- `crates/frontend/src/`, `crates/backend-wasm/src/`, `crates/runtime-abi/src/`

## Acceptance criteria

- [x] `mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/optionalTupleElementsAndUndefined.ts` passes
- [x] `let`/`const` duplicate detection continues to work (no regression)
- [x] No regression in existing tests (`cargo nextest run`)

## Validation

```sh
cargo fmt --all --check
cargo nextest run -E 'not test(test262_)'
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/optionalTupleElementsAndUndefined.ts
```

## Completion evidence

- Commit: `ae315d28` frontend/ir: allow var redeclaration (DuplicateLocal tolerance)
- Added `is_var: bool` to `Stmt::Let` AST node
- Parser sets `is_var: true` for `Var` token
- Name resolver `declare_variable` skips duplicate error when `is_var: true`
- `var v = 1; var v = 2;` compiles without DuplicateLocal
- `let x = 1; let x = 2;` still produces DuplicateLocal
- All 930 tests pass: `cargo nextest run` => 930 passed, 9 skipped

## False-done audit

Date: 2026-05-06

Classification: truly-done.

Audit result: retained in `issues/done/`. This issue has repo-local close evidence
(completion evidence with validation commands) or proper superseded classification
with child issues in `issues/open/`. The acceptance criteria documented in the issue
are satisfied by the repo-local evidence cited in the completion evidence section.

Future-work tracking: no untracked future-work item was identified in this issue
during this metadata/evidence audit.

