---
id: 5465
title: "Parse abstract anonymous default class exports"
type: bug
area: frontend/module-syntax
class: implementation-ready
priority: P1
depends_on: []
blocks: []
created: 2026-05-08
updated: 2026-05-08
---

## Summary

Parse `export default abstract class {}` far enough to represent the
default-exported abstract anonymous class or report the existing default-class
export boundary, instead of treating `abstract` as an unsupported expression.

Split from generated bucket
`issues/open/3495-implement-newAbstractInstance-parser-syntax.md`.

## Problem

Problem: `newAbstractInstance2.ts` stops before AST construction on the
`abstract` token inside an export-default class declaration:

```text
UnsupportedSyntax: unsupported expression: Some(SpannedToken { kind: Abstract, ... }) at 86..91
```

The lexer tokenizes `Export`, `Default`, `Abstract`, `Class`, `{`, `}`. The
TypeScript oracle accepts that top-level as a default-exported abstract
`ClassDeclaration`; the only oracle diagnostic is a later unresolved import in
the multi-file fixture.

## Current failure

```sh
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/newAbstractInstance2.ts
```

Representative source:

```ts
// @Filename: /a.ts
export default abstract class {}

// @Filename: /b.ts
import A from "./a";
new A();
```

Compiler evidence:

```text
tokens: ok; Export, Default, Abstract, Class, LeftBrace, RightBrace, Import, New A()
ast: fails before AST construction
resolved: fails with the same parser diagnostic
diagnostic: UnsupportedSyntax / parser-or-frontend-unsupported
message: unsupported expression at `abstract class`
```

TypeScript oracle evidence:

```text
TypeScript AST topLevel:
- ClassDeclaration "export default abstract class {}"
- ImportDeclaration "import A from \"./a\";"
- ExpressionStatement "new A();"
TypeScript diagnostics: TS2307 for later import resolution of "./a"
```

Coverage evidence:

```sh
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/newAbstractInstance2.ts --detail --no-dashboard-data
```

```text
executed=1
build_pass=0
unsupported=1
unsupported_diagcodes=UnsupportedSyntax:1
unsupported_features=unknown-unsupported:1
```

## Desired final state

The parser/module-syntax layer recognizes `export default abstract class {}` as
a class declaration form, preserving both the default export marker and the
abstract modifier, then advances to the next narrower module, import-resolution,
or constructability diagnostic.

## Scope

In scope:

- [ ] Parse `export default abstract class {}`.
- [ ] Preserve the default export marker and abstract class modifier in the AST
  or a source-spanned unsupported default-class export diagnostic.
- [ ] Add one focused parser/module regression for the exact form.
- [ ] Re-run `newAbstractInstance2.ts` triage and record the next blocker.

Out of scope:

- Named default class exports, tracked by
  `issues/open/5367-support-named-default-class-export-declarations.md`.
- Non-abstract anonymous default class exports with `extends`, tracked by
  `issues/open/5326-support-default-class-export-declarations.md`.
- Default function/interface/type exports.
- Full constructability semantics for importing and instantiating abstract
  classes.

## Affected paths

Expected:

- `crates/frontend/src/parser/`
- `crates/frontend/src/ast.rs`
- focused parser/module tests

Do not touch:

- backend/runtime ABI
- unrelated import/export forms

## Acceptance criteria

- [ ] `newAbstractInstance2.ts` no longer reports
  `unsupported expression` at `abstract class`.
- [ ] A focused regression proves `export default abstract class {}` is parsed
  or reaches a source-spanned default-class export diagnostic.
- [ ] Existing default class export issue owners 5326 and 5367 remain valid for
  their anonymous/named non-abstract representatives.
- [ ] If the representative advances to a new blocker, this issue records that
  blocker before closure.

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run -p ts2wasm-frontend export
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/newAbstractInstance2.ts
```

Impacted commands:

```sh
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/newAbstractInstance2.ts --detail --no-dashboard-data
python scripts/manager.py update-issue-index --check
python scripts/manager.py check-issue-health
python scripts/manager.py check-issue-readiness -- --fail-ready-below 80
git diff --check
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

This issue is separate from the broader `newAbstractInstance` semantic question.
The first current blocker is parser/module syntax for the exported abstract
class declaration.

## Completion evidence

Fill only when implemented.

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
