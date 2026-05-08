---
id: 5291
title: "Report malformed export type declarations"
type: feature
area: frontend/parser
class: implementation-ready
priority: P1
depends_on: []
blocks: []
created: 2026-05-07
updated: 2026-05-07
---

## Summary

Report source-spanned parser diagnostics for malformed `export type`
declarations instead of carrying an unterminated type alias parse into the next
`import` statement.

## Problem

`commonJsExportTypeDeclarationError.ts` contains intentionally malformed
`export type` declarations:

```ts
export type test
export type test =
```

The parser currently starts parsing `export type test` as a type alias and then
fails when it reaches the next file section's `import`, reporting
`expected Equal, got Some(Import)` or an unspanned EOF variant. TypeScript
reports syntax diagnostics at the malformed declarations.

Problem: malformed `export type` declarations are not diagnosed or recovered
at the local source line.

## Current failure

Reproduction:

```sh
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/commonJsExportTypeDeclarationError.ts
```

Current diagnostic:

```text
error: [UnsupportedSyntax] expected Equal, got Some(Import) at 266..272
```

Source context:

```ts
// @Filename: ./types1.ts
import test from "./test";
export type test

// @Filename: ./types2.ts
import test from "./test";
export type test =
```

Smart triage evidence:

```text
tokens: ok; Export, Ident("type"), Ident("test") are present
AST: fails with expected Equal at the following Import token
TypeScript oracle: TS1005 "=" expected at the following import; later diagnostics for the second malformed alias
```

## Desired final state

The parser reports a source-spanned syntax diagnostic for malformed
`export type` declarations and recovers enough to continue to the next
statement/file section.

## Scope

In scope:

- [ ] Detect `export type Name` without `=` before the next statement boundary.
- [ ] Detect `export type Name =` without a type expression before the next statement boundary.
- [ ] Report source-spanned diagnostics matching the malformed alias site, not an unspanned EOF or later import token.
- [ ] Preserve existing valid `export type Name = Type;` erasure.

Out of scope:

- CommonJS `module.exports` runtime/module support.
- Resolving `import test from "./test"`.
- Type alias semantic checking or declaration emit.
- `export { type X }` re-export forms.

## Affected paths

Expected:

- `crates/frontend/src/parser/statements_ts.rs`
- `crates/frontend/src/parser/statements_general.rs`
- `crates/frontend/src/parser/tests.rs`
- focused fixtures

Do not touch:

- `crates/backend-wasm/`
- module graph loading
- CommonJS runtime lowering

## Acceptance criteria

- [ ] `commonJsExportTypeDeclarationError.ts` no longer reports `expected Equal` at the following `import` token for `export type test`.
- [ ] A focused parser test covers `export type T` and reports a source-spanned missing-`=` diagnostic.
- [ ] A focused parser test covers `export type T =` and reports a source-spanned missing-type diagnostic.
- [ ] Existing valid `export type T = number;` parsing/erasure remains covered.

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run -p ts2wasm-frontend -E 'test(type) or test(export)'
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/commonJsExportTypeDeclarationError.ts
```

Impacted commands:

```sh
python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/commonJsExportTypeDeclarationError.ts --detail --no-dashboard-data
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
`issues/done/1383-implement-commonJsExportTypeDeclarationError.md`.

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
