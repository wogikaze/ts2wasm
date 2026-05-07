---
id: 5232
title: "Support entry-module export class declarations"
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

Implement the narrow static module export slice for entry-module class
declarations, such as `export class Foo {}`.

## Problem

`checkSuperCallBeforeThisAccess.ts` parses and builds an AST for the first
derived-class checks, but module build stops when it reaches an exported class
declaration in the entry module.

Problem: entry-module `export class Foo {}` currently reports `UnsupportedModule: issue-5005: entry module ... uses a declaration form outside the current static export slice`.

## Current failure

Reproduction:

```sh
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/checkSuperCallBeforeThisAccess.ts
```

Current diagnostic:

```text
error: [UnsupportedModule] issue-5005: entry module `export Foo` uses a declaration form outside the current static export slice; only export const and export default are supported at 1376..1431
```

Source context:

```ts
export class Foo {
    constructor(value: number) {
    }
}

export class BarCorrectlyFails extends Foo {
    constructor(something: boolean) {
        if (!something) {
            const value = this.bar();  // Error
            super(value);
        }
        else {
            super(1337);
        }
    }
    bar(): number { return 4; }
}
```

Compiler evidence:

```text
tokens: ok
ast: ok; exported class declarations are represented before module build
resolved/lowered: UnsupportedModule issue-5005 at entry-module export class
TypeScript oracle: reports TS17009/TS17011 this-before-super diagnostics after parsing the class bodies
```

## Desired final state

The entry module build path accepts `export class Name { ... }`, records the
class under the declared export name, and advances to the next semantic blocker
instead of rejecting the declaration as outside the static export slice.

## Scope

In scope:

- [ ] Rewrite `ExportDecl(ClassDecl)` in the entry module build path as a local class declaration plus module export metadata.
- [ ] Export under the declared class name.
- [ ] Add a focused module/build test for `export class Foo {}`.
- [ ] Re-run the representative reference triage and confirm it no longer reports the `only export const and export default are supported` boundary for `export class Foo`.

Out of scope:

- `export function` declarations, tracked by `issues/open/5144-support-entry-export-function-declarations.md`.
- Re-export forms, namespace exports, and `export var`.
- Full derived-class `this`/`super` flow diagnostics.
- Class runtime semantics beyond the existing class build support.

## Affected paths

Expected:

- `crates/compiler/src/lib.rs`
- `crates/cli/tests/m9_modules.rs`
- `fixtures/module-system/`

Do not touch:

- frontend parser unless a focused regression proves `ExportDecl(ClassDecl)` is no longer produced
- backend/runtime code unless existing module export metadata cannot represent class declarations

## Acceptance criteria

- [ ] `export class Foo {}` builds as an entry module and exports `Foo`.
- [ ] A focused module test proves an exported class value can be observed through the existing static module export path, or records the next class-runtime blocker with a narrower follow-up issue.
- [ ] `python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/checkSuperCallBeforeThisAccess.ts` no longer reports `issue-5005: entry module ... only export const and export default are supported` for `export class Foo`.
- [ ] `commentEmitOnParenthesizedAssertionInReturnStatement.ts` and
  `commentEmitOnParenthesizedAssertionInReturnStatement2.ts` no longer report
  issue-5005 for their entry-module `export class Foo` declarations.
- [ ] Unsupported export forms outside this slice still produce clear issue-5005 diagnostics.

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run -p ts2wasm-cli module
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/checkSuperCallBeforeThisAccess.ts
```

Impacted commands:

```sh
python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/checkSuperCallBeforeThisAccess.ts --detail --no-dashboard-data
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

Split from generated bucket `issues/done/1141-implement-checkSuperCallBeforeThisAccess.md`.

Related module-export history:

- `issues/done/5008-static-es-module-export-default-namespace-reexport.md`
- `issues/open/5144-support-entry-export-function-declarations.md`

2026-05-07 additional evidence:
`commentEmitOnParenthesizedAssertionInReturnStatement.ts` and
`commentEmitOnParenthesizedAssertionInReturnStatement2.ts` both tokenize and
parse `export class Foo { ... }`, including class field initializers and
parenthesized return expressions containing `as` or `satisfies` assertions.
Fresh triage reports `UnsupportedModule: issue-5005` for entry-module
`export Foo` in both files. Later comment emit, assertion preservation,
`satisfies`, and `this.client.getThing()` lowering behavior remains unproven
until this export-class boundary advances.
Also owns `issues/done/3361-implement-modulePrologueAMD.md`: fresh triage for
`modulePrologueAMD.ts` parses the `"use strict"` prologue and then stops at
entry-module `export class Foo {}` with issue-5005 before AMD prologue emit
parity becomes actionable.

## Completion evidence

Fill when implemented.
