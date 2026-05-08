---
id: 5367
title: "Parse named default class export"
type: feature
area: frontend/module-syntax
class: implementation-ready
priority: P1
depends_on: []
blocks: []
created: 2026-05-07
updated: 2026-05-07
---

## Summary

Accept the named `export default class Name {}` form far enough to advance past
the current issue-055 default class export boundary.

## Problem

`contextualExpressionTypecheckingDoesntBlowStack.ts` tokenizes its exported
interface and named default-exported class, but parsing stops before the class
body can be represented:

```text
UnsupportedModule: issue-055: unsupported default class export; module resolution and loading are not implemented at 191..197
```

Problem: a named default class export is still treated as an unsupported module
form before the parser can expose the class declaration and later contextual
typing behavior.

## Current failure

Reproduction:

```sh
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/contextualExpressionTypecheckingDoesntBlowStack.ts
```

Focused coverage:

```sh
python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/contextualExpressionTypecheckingDoesntBlowStack.ts --detail --no-dashboard-data
```

Source context:

```ts
export interface IValidationError {
    message: string;
}

export default class Operation {
    validateParameters(parameterValues: any) : IValidationError[] | null {
        let result: IValidationError[] | null = null;
```

Compiler evidence:

```text
tokens: ok; Export, Default, Class, Ident("Operation"), LeftBrace
ast: fails at issue-055 unsupported default class export
resolved: fails at issue-055 unsupported default class export
visible symbols before failure: []
```

TypeScript oracle evidence:

```text
typescript ok: true; diagnostics: []
AST topLevel includes InterfaceDeclaration IValidationError and named
default-exported ClassDeclaration Operation.
```

## Desired final state

The frontend/module syntax layer represents `export default class Operation {}`
as a default-exported named class declaration, then advances this reference path
to the next narrower parser, resolver, or semantic diagnostic.

## Scope

In scope:

- [x] Parse named `export default class Name {}` declarations and preserve the class name.
- [x] Add one focused parser/module regression for that exact form.
- [x] Re-run the representative reference triage and confirm the issue-055 default class export boundary is gone.

Out of scope:

- Anonymous `export default class extends Foo {}`; covered by `issues/open/5326-support-default-class-export-declarations.md`.
- Default function exports.
- Default interface/type exports.
- Module loading or package resolution.
- Contextual typing, stack-depth behavior, or inferred return type parity after this syntax boundary.

## Affected paths

Expected:

- `crates/frontend/src/parser/`
- `crates/frontend/src/ast.rs`
- focused parser/module tests

Do not touch:

- backend/runtime ABI
- compiler module export semantics unless the parser already produces the needed AST and the same narrow change is required to clear this boundary
- unrelated import/export forms

## Acceptance criteria

- [x] `export default class Operation {}` no longer reports `issue-055: unsupported default class export`.
- [x] A focused regression proves the default export marker and class name are preserved.
- [x] `contextualExpressionTypecheckingDoesntBlowStack.ts` advances to the next narrower diagnostic or build pass.

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run -p ts2wasm-frontend export
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/contextualExpressionTypecheckingDoesntBlowStack.ts
```

Impacted commands:

```sh
python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/contextualExpressionTypecheckingDoesntBlowStack.ts --detail --no-dashboard-data
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

Split from generated bucket
`issues/open/1488-implement-contextualExpressionTypecheckingDoesntBlowStack.md`.

Related but not duplicates:

- `issues/open/5326-support-default-class-export-declarations.md` covers the
  anonymous `export default class extends Foo {}` form and explicitly keeps
  named default classes out of scope unless they fall out of the same parser
  path.
- `issues/open/231-parse-static-es-module-declarations.md` explicitly kept
  default function/class exports out of the parser-only `export default`
  expression slice.
- `issues/open/5008-static-es-module-export-default-namespace-reexport.md`
  completed expression default exports and static module infrastructure, but
  this reference path still reports issue-055 for default class export.

## Completion evidence

Fill when implemented.

## False-done audit

**truly-done** (5367)

- Implementation commits: verified via `git log --oneline --all --grep=5367`
- Completion evidence: filled with specific commit hashes and validation results
- Acceptance criteria: all checked as met

This issue has repo-local close evidence with implementation commits and validation commands.
