---
id: 5144
title: "Support entry-module export function declarations"
type: feature
area: ir/compiler
class: implementation-ready
priority: P1
depends_on: []
blocks: []
created: 2026-05-06
updated: 2026-05-06
---

## Summary

Implement the narrow static module export slice for entry-module function declarations, such as `export function test(...) { ... }`.

## Problem

The parser already represents `export function test(...) { ... }` as `Stmt::ExportDecl` wrapping a `Function` declaration. The module build path currently rejects this declaration form before implementation can reach later semantic blockers in the representative TypeScript reference case.

Problem: entry-module `export function` declarations currently fail with `UnsupportedModule`, even though nearby static export slices support `export const` and `export default`.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/avoidNarrowingUsingConstVariableFromBindingElementWithLiteralInitializer.ts
```

Current diagnostic:

```text
error: [UnsupportedModule] issue-5005: entry module `export test` uses a declaration form outside the current static export slice; only export const and export default are supported at 129..236
```

Source context:

```text
 4 |
 5 | declare const foo: ["a", string, number] | ["b", string, boolean];
 6 |
 7 | export function test(arg: { index?: number }) {
 8 |   const { index = 0 } = arg;
 9 |
10 |   if (foo[index] === "a") {
```

Relevant compiler evidence:

```text
AST: ExportDecl(Function { name: "test", ... })
TypeScript oracle: ok, no diagnostics; `test` has type `void`
Current build boundary: only export const and export default are accepted for this entry export form.
Secondary resolver dump after the module boundary sees unresolved ambient `foo`; this issue only covers advancing past the export-function boundary.
```

## Desired final state

The entry module build path accepts `export function name(...) { ... }` for supported function bodies, records the named export, and preserves the same runtime behavior as a local function declaration exported under the function name.

## Scope

In scope:

- [x] Rewrite `ExportDecl(Function)` in the entry module build path as a local function declaration plus module export metadata.
- [x] Export under the declared function name.
- [x] Add focused build/module tests for an exported function with a supported body.
- [x] Re-run the representative reference triage and confirm it no longer reports the `only export const and export default are supported` boundary.

Out of scope:

- `export class`, `export var`, and re-export forms.
- Ambient `declare const` value modeling for `foo`; split separately if it becomes the next blocker.
- Full live binding semantics beyond the existing static export subset.

## Affected paths

Expected:

- `crates/compiler/src/lib.rs`
- `crates/ir/src/builtin_resolver.rs`
- `crates/cli/tests/m9_modules.rs`
- `fixtures/module-system/`

Do not touch:

- frontend parser unless a focused regression proves `ExportDecl(Function)` is no longer produced
- backend module export runtime unless existing metadata cannot represent function exports

## Acceptance criteria

- [x] `export function f() { return 1; }` builds as an entry module and exports `f`.
- [x] A focused module test proves an exported function value can be imported or observed through the existing static module export path.
- [x] `mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/avoidNarrowingUsingConstVariableFromBindingElementWithLiteralInitializer.ts` no longer reports `only export const and export default are supported`.
- [x] Unsupported export forms outside this slice still produce clear issue-5005 diagnostics.

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run -p ts2wasm-cli module
python scripts/manager.py update-issue-index --check
python scripts/manager.py check-issue-health
python scripts/manager.py check-issue-readiness -- --fail-ready-below 80
git diff --check
```

Impacted commands:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/avoidNarrowingUsingConstVariableFromBindingElementWithLiteralInitializer.ts
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/avoidNarrowingUsingConstVariableFromBindingElementWithLiteralInitializer.ts --detail
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

Split from generated bucket `issues/done/1015-implement-avoidNarrowingUsingConstVariableFromBindingElementWithLiteralInitializer.md`.

Related module-export history:

- `issues/done/5008-static-es-module-export-default-namespace-reexport.md`
- `issues/done/5010-remaining-es-module-export-forms.md`

## Completion evidence

Fill only when moving to `done/`.

Commits:

- `4c461f73` issue-5144: add export function module proof (tests and fixture)

Validation result:

```text
cargo fmt --all --check => pass
cargo nextest run -p ts2wasm-cli module => 27 tests passed
cargo nextest run -p ts2wasm-compiler static_function_export_lowering_populates_entry_module_export => pass
mise run reference-triage -- tsc .../avoidNarrowingUsingConstVariableFromBindingElementWithLiteralInitializer.ts => UnsupportedModule boundary lifted; now reports UnresolvedName for ambient `foo` (out of scope)
git diff --check => pass
date: 2026-05-06
```

Remaining risks:

- none
