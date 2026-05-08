---
id: 5176
title: "Report ambient var lib redeclaration diagnostics"
type: feature
area: frontend/semantics
class: implementation-ready
priority: P1
depends_on: []
blocks: []
created: 2026-05-06
updated: 2026-05-06
---

## Summary

`binopAssignmentShouldHaveType.ts` currently builds successfully even though TypeScript reports `TS2403` for `declare var console;` conflicting with the ES5 lib `Console` declaration.

## Problem

The parser recognizes the `declare var console;` tokens, but the resulting AST/resolved dumps omit the ambient declaration and the compiler returns a build pass. This hides the first TypeScript oracle diagnostic for the representative reference case.

Problem: ambient `declare var` declarations can conflict with lib globals, but ts2wasm currently erases the declaration and reports `BuildPass`.

## Current failure

Reference triage:

```sh
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/binopAssignmentShouldHaveType.ts
```

Current compiler result:

```text
BuildPass: ts2wasm build succeeded
```

Representative source:

```ts
// @lib: es5
declare var console;
```

Compiler evidence:

- Token dump includes `Ident("declare")`, `Var`, and `Ident("console")`.
- AST/resolved dumps contain only the later `"use strict"` expression; the ambient declaration is erased.
- Visible symbols include binding `console` at line 4, column 9.

TypeScript oracle evidence:

```text
TS2403: Subsequent variable declarations must have the same type.
Variable 'console' must be of type 'Console', but here has type 'any'.
```

## Desired final state

The frontend preserves enough ambient declaration information to report the representative `declare var console;` lib redeclaration mismatch instead of returning `BuildPass`.

## Scope

In scope:

- [x] Parse or record `declare var <ident>;` ambient declarations before they are erased.
- [x] Detect the narrow ES5-lib `console` redeclaration mismatch used by the representative reference case.
- [x] Report a source-spanned diagnostic at the `console` identifier.
- [x] Keep ordinary non-ambient `var` handling aligned with issue `5162`.

Out of scope:

- Full TypeScript lib declaration modeling.
- General declaration merging.
- Full type compatibility checking for all ambient declarations.
- Namespace/class body diagnostics owned by issue `5177`.

## Affected paths

Expected:

- `crates/frontend/src/parser/statements_ts.rs`
- `crates/frontend/src/parser/statements_core.rs`
- `crates/frontend/src/parser/statements_general.rs`
- `crates/frontend/src/parser/tests.rs`
- `crates/ir/src/name_resolver.rs`
- reference triage diagnostic mapping only if a new frontend diagnostic needs classification

Do not touch:

- ES module import/export loading.
- Runtime/backend emission.

## Acceptance criteria

- [x] A focused frontend or compiler test covers `declare var console;` with ES5 lib context and reports a diagnostic at `console`.
- [x] `python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/binopAssignmentShouldHaveType.ts` no longer reports `BuildPass` solely because `declare var console;` was erased.
- [x] Existing duplicate `let` / `const` diagnostics and compatible `var` redeclaration behavior remain unchanged.
- [x] Issue `5162` remains responsible for general compatible `var` redeclarations; this issue only covers the ambient-lib conflict surfaced by this reference case.

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run -p ts2wasm-frontend
cargo nextest run -p ts2wasm-ir
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/binopAssignmentShouldHaveType.ts
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

Split from generated bucket `1064` on 2026-05-06 after current triage showed the copied `import-export` blocker was stale.

Related issue `5162` covers compatible `var` redeclarations and later incompatible redeclaration typing; this issue is narrower because the representative diagnostic is an ambient declaration conflicting with a lib global.

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

Audit result: retained in issues/open/. Implementation commits confirmed.
