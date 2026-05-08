---
id: 5175
title: "Support export let declarations"
type: feature
area: frontend/module-syntax
class: implementation-ready
priority: P1
depends_on: []
blocks: []
created: 2026-05-06
updated: 2026-05-06
---

## Summary

Support narrow `export let` declaration forms that currently stop at the generic variable-export parser/module boundary, including the destructuring form from `bindingPatternOmittedExpressionNesting.ts` and the identifier declarations from `cacheResolutions.ts`.

## Problem

`bindingPatternOmittedExpressionNesting.ts` starts with `export let [,,[,[],,[],]] = undefined as any;`. `cacheResolutions.ts` contains repeated `export let x = 1;` declarations. In both cases, tokens succeed but the parser stops immediately with `issue-055: unsupported variable export` before the declaration can be parsed or triaged.

Problem: the parser has an `export const <ident> = ...` slice, but `export let` declarations still stop at the generic variable-export boundary.

## Current failure

Representative reference triage:

```sh
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/bindingPatternOmittedExpressionNesting.ts
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/cacheResolutions.ts
```

Current compiler diagnostic:

```text
UnsupportedModule: issue-055: unsupported variable export; module resolution and loading are not implemented at 64..70
UnsupportedModule: issue-055: unsupported variable export; module resolution and loading are not implemented at 114..120
```

Source context:

```ts
export let [,,[,[],,[],]] = undefined as any;
export let x = 1;
```

Compiler evidence:

- Tokens succeed for `export`, `let`, nested array elisions, `undefined as any`, and repeated `export let x = 1;` declarations.
- AST construction fails at `export` before building a declaration node.
- TypeScript oracle accepts `bindingPatternOmittedExpressionNesting.ts` with no diagnostics.
- TypeScript oracle accepts the `export let` syntax in `cacheResolutions.ts` and reports later TS2451 duplicate block-scoped variable diagnostics for `x`.

## Desired final state

The parser/module frontend accepts exported `let` declarations far enough that the current `unsupported variable export` blockers are gone.

## Scope

In scope:

- [x] Parse `export let <identifier> = <expr>;`.
- [x] Parse `export let <binding-pattern> = <expr>;` for array binding patterns with elisions.
- [x] Preserve the existing `export const <identifier> = <expr>` behavior.
- [x] Add focused coverage for `export let x = 1;`.
- [x] Add focused coverage for `export let [,,[,[],,[],]] = undefined as any;`.

Out of scope:

- General `export var` support.
- Named re-exports or namespace exports.
- Declaration emit fidelity after the parser advances.

## Affected paths

Expected:

- `crates/frontend/src/parser/statements_general.rs`
- `crates/frontend/src/parser/binding_patterns.rs`
- `crates/frontend/src/ast.rs`
- focused parser/module tests

Do not touch:

- full module graph loading
- unrelated import forms

## Acceptance criteria

- [x] A focused parser test accepts `export let [,,[,[],,[],]] = undefined as any;`.
- [x] A focused parser test accepts `export let x = 1;`.
- [x] Existing `export const value = 1;` tests still pass.
- [x] `python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/bindingPatternOmittedExpressionNesting.ts` no longer reports `unsupported variable export`.
- [x] `python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/cacheResolutions.ts` no longer reports `unsupported variable export`.

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run -p ts2wasm-frontend export_let
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/bindingPatternOmittedExpressionNesting.ts
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/cacheResolutions.ts
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

Split from generated bucket `1063` on 2026-05-06. Generated bucket `1088` was folded in on the same date after fresh triage showed the same `export let` variable-export boundary for `export let x = 1;`. Later declaration emit, duplicate binding diagnostics, or module export metadata gaps should be triaged separately after this parser boundary advances.
Also owns `issues/open/3310-implement-moduleAugmentationDisallowedExtensions.md`: fresh triage for `moduleAugmentationDisallowedExtensions.ts` stops at entry-section `export let a = 1;` before AST construction. Later blockers include initialized `export var x = 1;` (issue 5285), virtual imports between `@filename` sections (issue 5229), and `export = N1` (issue 5346).

## Completion evidence

Commits:

- `HEAD (final issue commit)`

Validation result:

```text
command: cargo fmt --all --check
result: pass
date: 2026-05-06

command: cargo nextest run -p ts2wasm-frontend export_let
result: 2 passed, 0 failed
date: 2026-05-06

command: cargo build -q -p ts2wasm-cli
result: pass (warning: compile_source_with_emit is dead code)
date: 2026-05-06

command: python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/bindingPatternOmittedExpressionNesting.ts
result: BuildPass; unsupported variable export no longer reported
date: 2026-05-06

command: python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/cacheResolutions.ts
result: BuildPass; parser advances to duplicate local binding evidence matching TypeScript TS2451
date: 2026-05-06
```

Remaining risks:

- none

## False-done audit

Date: 2026-05-06

Classification: truly-done.

Audit result: retained in `issues/open/`. This issue has repo-local close evidence
(completion evidence with validation commands) or proper superseded classification
with child issues in `issues/open/`. The acceptance criteria documented in the issue
are satisfied by the repo-local evidence cited in the completion evidence section.

Future-work tracking: no untracked future-work item was identified in this issue
during this metadata/evidence audit.

