---
id: 1229
title: "Implement Classnonuniquesymbolmethodhassymbolindexer"
type: spike
area: frontend/syntax
class: done
priority: P1
depends_on: []
blocks: []
created: 2026-05-01
updated: 2026-05-07
completed: 2026-05-07
status: done
---

## Summary

Triage classNonUniqueSymbolMethodHasSymbolIndexer across 1 failing reference
test case and close it as superseded by the existing computed class member
parser issue.

## Problem

Reference test results previously showed 1 case failing in directory
`classNonUniqueSymbolMethodHasSymbolIndexer` with diagnostics: import-export.
Fresh triage shows the current first blocker is parser support for computed
class member names.

Problem: `[a]()` in `export class A` stops with `expected property name, got
LeftParen`, which is the same feature family already owned by issue 5251.

## Current failure

Representative reproduction:

```sh
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/classNonUniqueSymbolMethodHasSymbolIndexer.ts
```

Coverage window:

```sh
python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/classNonUniqueSymbolMethodHasSymbolIndexer.ts --detail --no-dashboard-data
```

## Desired final state

This generated bucket is either split into implementation-ready child issues or superseded by an existing open/done issue with matching evidence. Do not implement directly from this bucket.

## Scope

In scope:

- [x] Inspect the smart triage report below
- [x] Confirm existing issue 5251 covers this parser feature family
- [x] Supersede this generated bucket without creating a duplicate child
- [x] Preserve exact reproduction commands and representative AST/diagnostic evidence

Out of scope:

- Direct implementation from this generated bucket
- Broad multi-feature fixes without child issue split

## Affected paths

Expected:

- `crates/frontend/src/`
- `crates/cli/src/`
- `fixtures/`
- `scripts/run/reference-triage.py`

Do not touch:

- unrelated runtime/backend code unless `reference-triage` proves the failure is not frontend-owned

## Acceptance criteria

- [x] Duplicate candidates below are confirmed as no-match or this issue is superseded
- [x] Superseding issue 5251 contains the implementation-ready parser scope
- [x] This issue includes failing path, diagnostic code, source context, visible symbols, and parser/TypeScript AST evidence
- [x] Completion evidence names the exact reference path and diagnostic/stdout change

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run
```

Impacted commands:

```sh
python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/classNonUniqueSymbolMethodHasSymbolIndexer.ts --detail --no-dashboard-data
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/classNonUniqueSymbolMethodHasSymbolIndexer.ts
```

Not run:

- `cargo fmt --all --check`; issue close only, no Rust code changed
- `cargo nextest run`; issue close only, no implementation changed

## Docs / current-state / issue sync

Final-state docs:

- [x] not affected

Current state:

- [x] not affected

Follow-up issues:

- [x] superseded by `issues/open/5251-parse-computed-class-member-names-in-class-declarations.md`

## Notes

## Affected test files

- `reference/typescript/tests/cases/compiler/classNonUniqueSymbolMethodHasSymbolIndexer.ts`

## Duplicate detection

- `issues/open/5251-parse-computed-class-member-names-in-class-declarations.md` - exact feature family for computed class member names in class declarations, including instance computed methods
- `issues/done/5214-computed-symbol-iterator-prerequisite-for-spread.md` - related class-expression shape, not exact for this top-level class declaration

## Smart triage

Fresh triage shows this generated import/export bucket is currently a parser
syntax blocker for computed class member names.

### Smart triage: classNonUniqueSymbolMethodHasSymbolIndexer

- Issue class: `triage-needed`
- Feature label: `parser-syntax`
- Diagnostic: `UnsupportedSyntax` / `parser-or-frontend-unsupported`
- Current compiler message: `expected property name, got LeftParen at 110..111`
- Path: `reference/typescript/tests/cases/compiler/classNonUniqueSymbolMethodHasSymbolIndexer.ts`

Reproduction:

```sh
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/classNonUniqueSymbolMethodHasSymbolIndexer.ts
```

Coverage:

```sh
python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/classNonUniqueSymbolMethodHasSymbolIndexer.ts --detail --no-dashboard-data
```

Coverage result:

```text
unsupported=1
unsupported_diagcodes=UnsupportedSyntax:1
unsupported_features=unknown-unsupported:1
```

Source context:

```ts
declare const a: symbol;
export class A {
    [a]() { return 1 };
}
declare const e1: A[typeof a];
```

Compiler evidence:

```text
tokens: ok; LeftBracket, Ident("a"), RightBracket, LeftParen, RightParen
ast/resolved: fail at `(` with expected property name, got LeftParen
```

TypeScript oracle evidence:

```text
typescript ok: true
diagnostics: []
AST path: ClassDeclaration -> MethodDeclaration "[a]() { return 1 }"
binding e1 type: () => number
```

Superseding owner:

- `issues/open/5251-parse-computed-class-member-names-in-class-declarations.md`

## Completion evidence

Commits:

- Superseded by `issues/open/5251-parse-computed-class-member-names-in-class-declarations.md`; see local commit for this issue cleanup.

Validation result:

```text
command: python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/classNonUniqueSymbolMethodHasSymbolIndexer.ts
result: pass on the main checkout with TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm; computed class method parser failure superseded by issue 5251
date: 2026-05-07

command: python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/classNonUniqueSymbolMethodHasSymbolIndexer.ts --detail --no-dashboard-data
result: pass on the main checkout with TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm; unsupported=1
date: 2026-05-07
```

Remaining risks:

- Issue 5251 currently also covers computed fields and static computed members;
  implementation should verify this reference path specifically once the parser
  support lands.
