---
id: 5324
title: "Support dependency-module export class declarations"
type: feature
area: ir/compiler
class: implementation-ready
priority: P1
depends_on: []
blocks: []
created: 2026-05-07
updated: 2026-05-07
---

## Summary

Support the narrow static module export slice for class declarations in
dependency virtual files, such as `export class Test1 { ... }` in a non-entry
`@Filename` section.

## Problem

`classMemberInitializerWithLamdaScoping3.ts` parses the entry file and the
dependency file, but module build stops when the dependency file contains an
exported class declaration.

Problem: dependency-module `export class Test1 { ... }` currently reports
`UnsupportedModule: issue-5005: dependency module declaration export uses a form
outside the current static export slice`.

## Current failure

Reproduction:

```sh
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/classMemberInitializerWithLamdaScoping3.ts
```

Current diagnostic:

```text
UnsupportedModule: issue-5005: dependency module declaration export uses a form outside the current static export slice at 59..471
```

Source context:

```ts
// @Filename: classMemberInitializerWithLamdaScoping3_0.ts
var field1: string;

// @Filename: classMemberInitializerWithLamdaScoping3_1.ts
declare var console: {
    log(msg?: any): void;
};
export class Test1 {
    constructor(private field1: string) {
    }
    messageHandler = () => {
        console.log(field1);
    };
}
```

Compiler evidence:

```text
tokens: ok
ast: ok; first file has var field1, dependency file has ExportDecl(ClassDecl Test1)
resolved: ok; class Test1 constructor parameter property is represented
module build: UnsupportedModule issue-5005 for dependency-module export class
```

TypeScript oracle evidence:

```text
typescript ok: false
diagnostic TS2301: Initializer of instance member variable 'messageHandler' cannot reference identifier 'field1' declared in the constructor.
binding field1 type: string
parameter field1 type: string
```

## Desired final state

Dependency module build accepts `export class Name { ... }`, records the class
under the declared export name, and advances to the next semantic blocker
instead of rejecting the declaration as outside the static export slice.

## Scope

In scope:

- [ ] Rewrite dependency-module `ExportDecl(ClassDecl)` as a local class declaration plus module export metadata.
- [ ] Export under the declared class name for dependency virtual files.
- [ ] Add a focused module/build regression for a dependency file containing `export class Test1 {}`.
- [ ] Re-run the representative reference triage and confirm it no longer reports the dependency export-class issue-5005 boundary.

Out of scope:

- Entry-module export class declarations, tracked by `issues/done/5232-w0-fixture-ize-runtimelinkplan-linker-structure-tests.md`.
- Exported variables, tracked by `issues/open/5283-support-entry-export-var-declarations.md` and related variable-export slices.
- Full TS2301 class field initializer scoping diagnostics.
- AMD/CommonJS emit parity beyond the static dependency export-class boundary.

## Affected paths

Expected:

- `crates/compiler/src/lib.rs`
- `crates/cli/tests/m9_modules.rs`
- `fixtures/module-system/`

Do not touch:

- frontend parser unless a focused regression proves `ExportDecl(ClassDecl)` is no longer produced
- backend/runtime code unless existing module export metadata cannot represent dependency class declarations

## Acceptance criteria

- [ ] A dependency virtual file with `export class Test1 {}` builds far enough to expose `Test1` through the existing static module export path.
- [ ] `python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/classMemberInitializerWithLamdaScoping3.ts` no longer reports `issue-5005: dependency module declaration export uses a form outside the current static export slice`.
- [ ] Unsupported dependency export forms outside this slice still produce clear issue-5005 diagnostics.
- [ ] The next observed diagnostic, if any, is recorded in a narrower follow-up issue instead of hidden behind the export-class boundary.

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run -p ts2wasm-cli module
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/classMemberInitializerWithLamdaScoping3.ts
```

Impacted commands:

```sh
python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/classMemberInitializerWithLamdaScoping3.ts --detail --no-dashboard-data
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
`issues/done/1223-implement-classMemberInitializerWithLamdaScoping-import-export.md`.

Related but not duplicates:

- `issues/done/5232-w0-fixture-ize-runtimelinkplan-linker-structure-tests.md` covers entry-module `export class`, not dependency virtual files.
- `issues/open/5295-resolve-import-equals-require-to-virtual-node-modules-class-export.md` covers bare `require("myModule")` resolution plus a node_modules dependency class export shape.
- `issues/open/5283-support-entry-export-var-declarations.md` covers the sibling `classMemberInitializerWithLamdaScoping4.ts` first blocker, `export var field1: string`.

## Completion evidence

Fill when implemented.
