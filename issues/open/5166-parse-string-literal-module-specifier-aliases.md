---
id: 5166
title: "Parse string-literal module specifier aliases"
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

TypeScript allows string-literal names in static import/export specifier positions such as `export { foo as "0n" }` and `import { "0n" as foo } from "./foo"`. The parser currently expects identifiers only, so it stops on the first string-literal exported name.

## Problem

Problem: `reference/typescript/tests/cases/compiler/bigintArbirtraryIdentifier.ts` currently reports `UnsupportedSyntax: expected identifier, got Some(SpannedToken { kind: String("0n"), ... })` for `export { foo as "0n" };`.

## Current failure

Reference triage:

```sh
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/bigintArbirtraryIdentifier.ts
```

Current compiler diagnostic:

```text
UnsupportedSyntax: expected identifier, got Some(SpannedToken { kind: String("0n"), span: Span { start: 97, end: 101 } }) at 102..103
```

Representative source:

```ts
const foo = 0n;
export { foo as "0n" };

import { "0n" as foo } from "./foo";
export { foo as "0n" };
```

Compiler evidence:

- Lexer emits `String("0n")` for the valid arbitrary identifier specifier.
- AST construction stops in module specifier parsing before representing the valid export/import declarations.
- Later invalid cases use `BigIntLiteral("0n")` in specifier positions and should remain rejected with source-spanned syntax diagnostics.

TypeScript oracle evidence:

```text
foo: 0n
TS accepts string-literal import/export specifier names.
TS reports diagnostics for BigInt literal specifiers such as `import { 0n as foo }`.
```

TypeScript AST evidence:

```text
ExportDeclaration: export { foo as "0n" };
ImportDeclaration: import { "0n" as foo } from "./foo";
```

## Desired final state

The parser accepts string-literal module specifier names in valid static import/export specifier positions while preserving rejection of BigInt literal specifiers and other non-string/non-identifier tokens.

## Scope

In scope:

- [x] Parse `export { foo as "0n" };`.
- [x] Parse `import { "0n" as foo } from "./foo";`.
- [x] Parse `export { foo as "0n" } from "./foo";` if the existing re-export parser shares the same specifier path.
- [x] Preserve spans and string values in the existing `ImportNamedSpecifier`, `ExportNamedSpecifier`, and `ReExportNamedSpecifier` structures.
- [x] Keep `import { 0n as foo }`, `import { foo as 0n }`, `export { foo as 0n }`, and `export { 0n as foo }` rejected.

Out of scope:

- Full module graph/package resolution for `./foo`.
- Runtime support for arbitrary exported names beyond preserving the parsed specifier names for downstream module lowering.
- General parser recovery after intentionally invalid TypeScript diagnostics later in the same reference file.

## Affected paths

Expected:

- `crates/frontend/src/parser/statements_general.rs`
- `crates/frontend/src/parser/tokens.rs`
- `crates/frontend/src/parser/tests.rs`
- `crates/frontend/src/ast.rs`
- `crates/compiler/src/dump.rs`

Do not touch:

- module graph resolution unless parsing advances and exposes a downstream module-specific diagnostic.
- BigInt literal expression parsing.

## Acceptance criteria

- [x] `parse_program("export { foo as \"0n\" };")` succeeds and preserves exported name `"0n"`.
- [x] `parse_program("import { \"0n\" as foo } from \"./foo\";")` succeeds and preserves imported name `"0n"` with local name `foo`.
- [x] Invalid BigInt literal specifiers still fail with a source-spanned syntax diagnostic.
- [x] `python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/bigintArbirtraryIdentifier.ts` no longer reports the first `String("0n")` expected-identifier parser diagnostic.

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run -p ts2wasm-frontend
cargo nextest run -p ts2wasm-compiler
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/bigintArbirtraryIdentifier.ts
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

Split from generated bucket `1050` on 2026-05-06. The existing static module parser issue `231` covered ordinary identifier specifiers; this issue covers TypeScript's arbitrary string-literal module specifier names.

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

Date: 2026-05-06

Classification: truly-done.

Audit result: retained in issues/open/. Commit 2555793f
Future-work tracking: none identified.
