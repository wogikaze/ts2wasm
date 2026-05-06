---
id: 5193
title: "Parse ASI after ambient variable declarations"
type: feature
area: frontend/syntax
class: implementation-ready
priority: P1
depends_on: []
blocks: []
created: 2026-05-06
updated: 2026-05-06
---

## Summary

Accept automatic semicolon insertion after declaration-only ambient variable
statements, including exported `declare let` declarations in TypeScript
reference multi-file fixtures.

## Problem

The ambient declaration erasure boundary supports declaration-only
`declare var`/`let`/`const` statements, but the parser still reports
`issue-400` when an ambient variable declaration has no explicit semicolon and
the next token starts another statement.

Problem: declaration-only ambient variables followed by a newline and another statement can still report `issue-400` instead of accepting ASI.

## Current failure

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/cachedModuleResolution2.ts
```

Current diagnostic:

```text
error: [UnsupportedTypeScriptSyntax] issue-400: unterminated ambient variable declaration type at 7..14
```

Representative source:

```ts
// @filename: /a/b/node_modules/foo.d.ts
export declare let x: number

// @filename: /a/b/c/lib.ts
import {x} from "foo";
```

Additional representative:

```ts
declare var foo:{ ( ):void; }
declare var bar:{ new ( ):any; }

foo = bar; // error
bar = foo; // error
```

Triage notes:

- Tokens include `Export`, `declare`, `Let`, `x`, `:`, `number`, then `Import` with no semicolon token between them.
- `callConstructAssignment.ts` has the same ASI shape after ambient variable type literals: tokens include `declare var foo:{ ( ):void; }`, then `declare var bar:{ new ( ):any; }`, then assignment expressions; the parser reports the first `=` as an ambient initializer.
- TypeScript accepts ASI after the ambient variable declaration, then reports downstream duplicate identifier and missing-module diagnostics.
- Separate dumps can already construct an `AmbientValueDecl` and then reach the `issue-232` module specifier boundary for `foo`, so this issue is only about the first parser/build diagnostic.

## Desired final state

Declaration-only ambient variable declarations accept ASI at line breaks and
before TypeScript test-harness `@filename` virtual-file boundaries. The
representative case advances beyond the `issue-400` unterminated ambient
variable declaration diagnostic.

## Scope

In scope:

- [ ] Accept ASI after `declare var`, `declare let`, and `declare const`
- [ ] Accept ASI after exported declaration-only ambient variables, such as `export declare let x: number`
- [ ] Accept ASI after ambient variable type literals with call and construct signatures
- [ ] Preserve rejection for ambient variable declarations with initializers
- [ ] Add a focused parser/build regression fixture without relying on broad module resolution

Out of scope:

- Package or bare module resolution for `foo`
- TypeScript duplicate identifier and merged-declaration diagnostics
- General ASI for ambient function declarations, tracked separately by `issues/open/705-implement-asiAmbientFunctionDeclaration.md`

## Affected paths

Expected:

- `crates/frontend/src/`
- `crates/cli/src/`
- `fixtures/`
- `crates/cli/tests/`

Do not touch:

- `crates/backend-wasm/`
- module graph package resolution unless a focused test proves the parser has advanced first

## Acceptance criteria

- [ ] `export declare let x: number` followed by a newline parses as an erased ambient value declaration without `issue-400`
- [ ] `declare var foo:{ ( ):void; }` followed by `foo = bar;` parses the assignment as a separate expression, not an ambient initializer
- [ ] `declare const c: number` followed by EOF or another statement parses without requiring an explicit semicolon
- [ ] `declare var x = 1` remains rejected as an ambient initializer
- [ ] `cachedModuleResolution2.ts` no longer stops at `issue-400: unterminated ambient variable declaration type`
- [ ] `callConstructAssignment.ts` no longer stops at `issue-400: ambient variable declarations with initializers would affect runtime bindings`

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run -p ts2wasm-cli -E 'test(ambient)'
```

Impacted commands:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/cachedModuleResolution2.ts
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/callConstructAssignment.ts
cargo nextest run -p ts2wasm-cli -E 'test(ambient|parser|module)'
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

Issue 400 is closed for the ambient-erasure boundary. This issue is the narrower
parser completion slice for ASI on declaration-only ambient variables.

Generated bucket `1091` was folded in on 2026-05-06 after fresh triage showed
the same `issue-400` ambient variable ASI gap for call/construct signature type
literals.

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
