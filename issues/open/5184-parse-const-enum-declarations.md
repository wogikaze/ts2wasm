---
id: 5184
title: "Parse const enum declarations"
type: feature
area: frontend/syntax
class: implementation-ready
priority: P1
depends_on: []
blocks: []
created: 2026-05-06
updated: 2026-05-07
---

## Summary

`blockScopedEnumVariablesUseBeforeDef.ts` stops at `const enum E { A }` because the parser treats `const enum` as an ordinary `const` declaration named `enum` with no initializer.

## Problem

The lexer emits `Const` followed by `Ident("enum")`. Statement parsing takes the `const` branch before recognizing the TypeScript `const enum` declaration form, producing `const declarations require an initializer`.

Problem: `const enum` declarations are parser-unsupported and are misclassified as malformed `const` variable declarations.

## Current failure

Reference triage:

```sh
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/blockScopedEnumVariablesUseBeforeDef.ts
```

Current compiler diagnostic:

```text
UnsupportedSyntax: const declarations require an initializer at 128..132
```

Representative source:

```ts
function foo2() {
    return E.A
    const enum E { A }
}
```

Compiler evidence:

- Token dump includes `Const`, `Ident("enum")`, `Ident("E")`, `{`, member `A`, and `}`.
- AST/resolved construction fails before representing the enum declaration.
- Visible symbol extraction reports a bogus binding named `enum`.

TypeScript oracle evidence:

```text
TS2450: Enum 'E' used before its declaration.
```

TypeScript AST evidence at the current blocker:

```text
FunctionDeclaration -> Block -> EnumDeclaration
```

## Desired final state

The frontend recognizes `const enum E { A }` as a TypeScript enum declaration before ordinary `const` variable parsing. The representative case should no longer fail with `const declarations require an initializer` at the `enum` token.

## Scope

In scope:

- [ ] Recognize `const enum` in statement positions before the ordinary `const` declaration parser.
- [ ] Preserve enough enum declaration metadata for later diagnostics or explicit unsupported-enum handling.
- [ ] Avoid creating a bogus binding named `enum`.
- [ ] Add focused parser/frontend coverage for `const enum E { A }` inside a function block.
- [ ] Re-run representative triage and confirm the current const-initializer diagnostic is gone.

Out of scope:

- Full enum runtime emit.
- Full const-enum inlining.
- `export const enum` module handling.
- The later `TS2450` enum used-before-declaration diagnostic after parsing succeeds.

## Affected paths

Expected:

- `crates/frontend/src/parser/statements_general.rs`
- `crates/frontend/src/parser/statements_ts.rs`
- `crates/frontend/src/parser/tests.rs`
- `crates/frontend/src/ast.rs`
- `crates/compiler/src/dump.rs`
- enum/reference triage diagnostic mapping only if classification needs refining

Do not touch:

- ES module import/export loading.
- Backend enum runtime emit.
- General `const` declaration initializer rules.

## Acceptance criteria

- [ ] `const enum E { A }` parses as an enum declaration or reports an enum-specific diagnostic, not `const declarations require an initializer`.
- [ ] `constEnumBadPropertyNames.ts` no longer reports `const declarations require an initializer` at `const enum E { A }`.
- [ ] A function-block `const enum E { A }` no longer creates a bogus binding named `enum`.
- [ ] Ordinary `const x;` still reports the missing-initializer diagnostic.
- [ ] `python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/blockScopedEnumVariablesUseBeforeDef.ts` no longer reports the current const-initializer diagnostic at `const enum E`.

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run -p ts2wasm-frontend enum
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/blockScopedEnumVariablesUseBeforeDef.ts
```

Impacted commands:

```sh
python scripts/manager.py check-issue-readiness -- --fail-ready-below 80
python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/commentsdoNotEmitComments.ts --detail --no-dashboard-data
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/constEnumBadPropertyNames.ts
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

Split from generated bucket `1070` on 2026-05-06. Bucket `1071` was later superseded by the same child after fresh triage of `blockScopedEnumVariablesUseBeforeDef_isolatedModules.ts` showed the identical `const enum` parser boundary. Sibling buckets such as `1446`, `633`, and `737` also show the same boundary in different reference contexts; this child issue names the shared parser slice.

Additional superseded bucket:

- `issues/done/1381-implement-commentsdoNotEmitComments.md` reaches the same
  `const enum` misclassification family at
  `const enum color { red, green, blue }`. Fresh triage on 2026-05-07 shows a
  bogus visible binding named `enum`, then `var shade: color = color.green`
  fails with `UnresolvedName` for `color`. Comment emit behavior is not reached
  before this const-enum parser/binding boundary.
- `issues/done/1445-implement-constEnumBadPropertyNames.md` reaches the same
  `const enum` parser boundary at top level. Fresh triage on 2026-05-07 reports
  `const declarations require an initializer at 26..30` for `const enum E { A }`;
  TypeScript parses the enum and reports TS2339 for the later `E["B"]` access.
- `issues/done/1446-implement-constEnumDeclarations.md` reaches the same
  parser boundary for ordinary top-level const enum declarations. Fresh triage
  on 2026-05-07 reports `const declarations require an initializer at 51..55`
  for `const enum E { ... }`; TypeScript accepts both `E` and `E2` const enum
  declarations with no diagnostics.
- `issues/done/1448-implement-constEnumExternalModule.md` reaches the same
  parser boundary before external module handling. Fresh triage on 2026-05-07
  reports `const declarations require an initializer at 62..66` for
  `const enum E { V = 100 }`; TypeScript parses the enum, `export = E`, and
  `import A = require('m1')` before reporting TS2307 for module resolution.
- `issues/done/1451-implement-constEnumNamespaceReferenceCausesNoImport.md`
  includes `constEnumNamespaceReferenceCausesNoImport.ts`, which reaches the
  same parser boundary through `export const enum ConstFooEnum { ... }`. Fresh
  triage on 2026-05-07 reports `const declarations require an initializer at
  112..116` at `Ident("enum")`; TypeScript parses the declaration as an
  exported `EnumDeclaration` and only reports later TS2307 for `./foo`.
- `issues/done/1452-implement-constEnumNoEmitReexport.md` reaches the same
  parser boundary before no-emit re-export behavior. Fresh triage on
  2026-05-07 reports `const declarations require an initializer at 82..86` for
  `export const enum MyConstEnum { ... }`; TypeScript parses the declaration and
  then reports later duplicate identifier, export consistency, default export,
  and missing module diagnostics.

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

- Later triage may expose ordinary enum parsing, enum used-before-declaration diagnostics, const-enum inlining, or export/module handling after this parser boundary is removed.
