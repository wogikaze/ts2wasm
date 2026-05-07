---
id: 5326
title: "Parse anonymous default class export"
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

Accept the anonymous `export default class extends Foo {}` form far enough to
advance past the current issue-055 default class export boundary.

## Problem

`classMergedWithInterfaceMultipleBasesNoError.ts` tokenizes interface/class
declarations, but parsing stops at the anonymous default class export:

```text
UnsupportedModule: issue-055: unsupported default class export; module resolution and loading are not implemented at 128..134
```

Problem: this single anonymous default class form is still treated as an
unsupported module form before the class declaration can be represented and
triaged.

## Current failure

Reproduction:

```sh
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/classMergedWithInterfaceMultipleBasesNoError.ts
```

Focused coverage:

```sh
python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/classMergedWithInterfaceMultipleBasesNoError.ts --detail --no-dashboard-data
```

Source context:

```ts
interface Bar { }
interface Baz { }
interface Q { }
interface Foo extends Bar, Baz { }
class Foo { }

export default class extends Foo {
    readonly observer = this.handleIntersection;
    readonly handleIntersection = () => { }
}
```

Compiler evidence:

```text
tokens: ok; Export, Default, Class, Extends, Ident("Foo")
ast: fails at issue-055 unsupported default class export
resolved: fails at issue-055 unsupported default class export
```

TypeScript oracle evidence:

```text
typescript ok: false
diagnostic TS2729: Property 'handleIntersection' is used before its initialization.
AST topLevel includes InterfaceDeclaration Bar/Baz/Q/Foo, ClassDeclaration Foo,
and default-exported anonymous ClassDeclaration extending Foo.
```

## Desired final state

The frontend/module syntax layer represents the anonymous default-exported class
and preserves its `extends Foo` clause, then advances to the next narrower
module or semantic blocker.

## Scope

In scope:

- [ ] Parse anonymous `export default class extends Foo {}` and preserve `extends Foo`.
- [ ] Add one focused parser/module regression for that exact form.
- [ ] Re-run the representative reference triage and confirm the issue-055 default class export boundary is gone.

Out of scope:

- Named `export default class Name {}` unless it falls out of the same minimal parser path.
- Default function exports.
- Default interface/type exports.
- Class field initialization order diagnostics such as TS2729.
- Full declaration emit behavior for default-exported classes.

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

- [ ] `export default class extends Foo {}` no longer reports `issue-055: unsupported default class export`.
- [ ] A focused regression proves the default export marker and `extends Foo` are preserved.
- [ ] The representative reference triage advances to the next narrower diagnostic or build pass.

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run -p ts2wasm-frontend export
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/classMergedWithInterfaceMultipleBasesNoError.ts
```

Impacted commands:

```sh
python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/classMergedWithInterfaceMultipleBasesNoError.ts --detail --no-dashboard-data
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
`issues/done/1226-implement-classMergedWithInterfaceMultipleBasesNoError.md`.

Related but not duplicates:

- `issues/done/231-parse-static-es-module-declarations.md` explicitly kept
  default function/class exports out of the parser-only `export default`
  expression slice.
- `issues/done/5008-static-es-module-export-default-namespace-reexport.md`
  completed expression default exports and static module infrastructure, but
  this current reference path still reports issue-055 for default class export.
- Generated default-class buckets such as
  `issues/open/2282-implement-exportDefaultClassAndValue.md` still need their
  own smart triage before they can be closed or superseded.

## Completion evidence

Fill when implemented.
