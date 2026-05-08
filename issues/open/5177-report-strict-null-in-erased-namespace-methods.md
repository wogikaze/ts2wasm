---
id: 5177
title: "Report strict-null diagnostics in erased namespace methods"
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

`binopAssignmentShouldHaveType.ts` hides a TypeScript `TS2322` diagnostic because the namespace/class body is erased before the typed local declaration is checked.

## Problem

The compiler tokenizes `namespace Test { export class Bug { ... } }`, including the method body `var name:string = null;`, but the AST/resolved dumps only contain `"use strict"`. TypeScript still reports a type diagnostic for the local declaration inside the namespace class method.

Problem: erased namespace class method bodies can hide typed local declaration diagnostics and produce a false build pass.

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
namespace Test {
 export class Bug {
  bug() {
   var name:string= null;
   if ((name= this.getName()).length > 0) {
    console.log(name);
   }
  }
 }
}
```

Compiler evidence:

- Token dump includes `namespace`, `export`, `class`, method `bug`, `Var`, `Ident("name")`, `Ident("string")`, and `Null`.
- AST/resolved dumps contain only the earlier `"use strict"` expression because the namespace body is erased.
- Visible symbols include class `Bug` and binding `name`.

TypeScript oracle evidence:

```text
TS2322: Type 'null' is not assignable to type 'string'.
```

The diagnostic is reported at `name` on line 12 in the representative reference case.

## Desired final state

The frontend preserves enough namespace class method body information to report the representative `string = null` diagnostic instead of allowing an empty-AST build pass.

## Scope

In scope:

- [x] Detect typed local declarations inside erased namespace class methods for the representative pattern.
- [x] Report a source-spanned diagnostic for `var name: string = null` in that namespace class method body.
- [x] Add focused coverage that omits `declare var console;` so this issue can be verified independently of issue `5176`.
- [x] Re-run the representative triage and confirm this diagnostic is no longer silently erased once earlier diagnostics are handled.

Out of scope:

- Full namespace runtime or emit support.
- Complete TypeScript type checking.
- General `strictNullChecks` coverage outside the `string = null` representative pattern.
- Ambient lib/global redeclaration diagnostics owned by issue `5176`.

## Affected paths

Expected:

- `crates/frontend/src/parser/statements_ts.rs`
- `crates/frontend/src/parser/statements_class.rs`
- `crates/frontend/src/parser/statements_core.rs`
- `crates/frontend/src/parser/statements_general.rs`
- `crates/frontend/src/parser/tests.rs`
- semantic/reference triage diagnostics if a new diagnostic mapping is needed

Do not touch:

- ES module import/export loading.
- Runtime/backend emission.

## Acceptance criteria

- [x] A focused frontend/compiler test covers `namespace Test { export class Bug { bug() { var name: string = null; } } }`.
- [x] The diagnostic is source-spanned at the local `name` binding and no longer disappears because the namespace body was erased.
- [x] A reduced reference-style fixture without `declare var console;` reports the `string = null` diagnostic rather than `BuildPass`.
- [x] `python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/binopAssignmentShouldHaveType.ts` no longer hides the namespace-method `TS2322` after issue `5176` handles the earlier ambient redeclaration diagnostic.

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run -p ts2wasm-frontend namespace
cargo nextest run -p ts2wasm-frontend var
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

Split from generated bucket `1064` on 2026-05-06 after current triage showed a false build pass rather than the stale copied `import-export` blocker.

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

Date: 2026-05-07

Classification: truly-done.

Audit result: retained in issues/open/. Implementation commits confirmed.
