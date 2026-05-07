---
id: 5352
title: "Parse export namespace declarations"
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

Accept top-level TypeScript `export namespace Name { ... }` declarations far
enough to represent the namespace declaration or report a namespace-specific
boundary, instead of stopping at the generic issue-055 static export diagnostic.

This is the current blocker from
`constEnumNamespaceReferenceCausesNoImport2.ts`.

## Problem

The compiler tokenizes `export namespace ConstEnumOnlyModule { ... }`, but AST
construction stops at the `export` keyword with the generic static export
boundary before the namespace body, nested const enum, or later
`export = Foo.ConstEnumOnlyModule` can be triaged.

Problem: `export namespace Name { ... }` is treated as an unsupported static
export before the frontend can represent the namespace declaration.

## Current failure

Reproduction:

```sh
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/constEnumNamespaceReferenceCausesNoImport2.ts
```

Current diagnostic:

```text
UnsupportedModule: issue-055: unsupported static export; module resolution and loading are not implemented at 30..36
```

Source context:

```ts
export namespace ConstEnumOnlyModule {
  export const enum ConstFooEnum {
    Some,
    Values,
    Here
  }
}
```

Compiler token evidence:

```text
Export, Ident("namespace"), Ident("ConstEnumOnlyModule"), LeftBrace,
Export, Const, Ident("enum"), Ident("ConstFooEnum"), ...
```

TypeScript AST evidence:

```text
ModuleDeclaration "export namespace ConstEnumOnlyModule { ... }"
ImportDeclaration "import * as Foo from \"./foo\";"
ExportAssignment "export = Foo.ConstEnumOnlyModule;"
ImportEqualsDeclaration "import Foo = require(\"./reexport\");"
FunctionDeclaration "function check(x: Foo.ConstFooEnum): void { ... }"
```

## Desired final state

The frontend recognizes `export namespace Name { ... }` as an exported
namespace/module declaration or emits a precise namespace-specific unsupported
diagnostic with the namespace span preserved. The representative reference file
should no longer stop at generic issue-055 on the leading `export` keyword.

## Scope

In scope:

- [ ] Recognize top-level `export namespace Name { ... }` before the generic
      unsupported static export branch.
- [ ] Preserve the namespace name and declaration span for later diagnostics.
- [ ] Add focused parser/frontend coverage for
      `export namespace ConstEnumOnlyModule {}`.
- [ ] Re-run the representative triage and record the next blocker if it
      advances beyond the exported namespace boundary.

Out of scope:

- Nested `const enum` parsing, tracked by
  `issues/open/5184-parse-const-enum-declarations.md`.
- Export-assignment parsing and diagnostics, tracked separately by
  `issues/open/5186-parse-export-assignment-for-diagnostics.md` and
  `issues/open/5306-report-export-assignment-with-other-exports.md`.
- Import-equals/`require` module loading.
- Namespace value binding or namespace emit/lowering.
- `export as namespace Name;`, tracked by
  `issues/done/5231-w0-typed-wat-writer-expr-emit-migration.md`.

## Affected paths

Expected:

- `crates/frontend/src/parser/`
- `crates/frontend/src/ast.rs`
- `crates/cli/tests/`
- `fixtures/`

Do not touch:

- `crates/backend-wasm/`
- module graph/package resolution

## Acceptance criteria

- [ ] `constEnumNamespaceReferenceCausesNoImport2.ts` no longer reports
      generic `issue-055: unsupported static export` at the leading
      `export namespace ConstEnumOnlyModule`.
- [ ] A focused parser or CLI AST test proves
      `export namespace ConstEnumOnlyModule {}` is recognized or reported with
      a namespace-specific diagnostic.
- [ ] Existing unsupported static export diagnostics still apply to unrelated
      export forms.
- [ ] Any next blocker in
      `constEnumNamespaceReferenceCausesNoImport2.ts` is recorded here or split
      to a follow-up issue if outside this scope.

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run -p ts2wasm-frontend -E 'test(namespace) or test(export)'
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/constEnumNamespaceReferenceCausesNoImport2.ts
```

Impacted commands:

```sh
python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/constEnumNamespaceReferenceCausesNoImport2.ts --detail --no-dashboard-data
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
`issues/open/1451-implement-constEnumNamespaceReferenceCausesNoImport.md`.

Related but not duplicates:

- `issues/done/5231-w0-typed-wat-writer-expr-emit-migration.md` covers
  `export as namespace Name;`, not namespace declarations with bodies.
- `issues/done/5277-parse-export-enum-declarations-to-enum-boundary.md` covers
  `export enum`, not `export namespace`.
- `issues/open/5287-bind-namespace-declarations-for-qualified-value-access.md`
  and `issues/open/5294-resolve-sibling-namespaces-in-nested-namespace-scopes.md`
  cover namespace binding/resolution after namespace declarations parse.

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

- After this parser boundary advances, the representative file is expected to
  expose nested const-enum parsing, export-assignment diagnostics, import-equals
  resolution, or local module resolution as later blockers.
