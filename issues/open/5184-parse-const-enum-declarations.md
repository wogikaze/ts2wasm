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
updated: 2026-05-06
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

Split from generated bucket `1070` on 2026-05-06. Sibling buckets such as `1446`, `633`, and `737` also show the same `const enum` parser boundary in different reference contexts; this child issue names the shared parser slice.

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
