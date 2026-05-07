---
id: 1452
title: "Implement Constenumnoemitreexport"
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

Closed as superseded. Fresh triage shows
`reference/typescript/tests/cases/compiler/constEnumNoEmitReexport.ts`
currently stops at the same `export const enum` parser misclassification already
owned by `issues/done/5184-parse-const-enum-declarations.md`.

## Problem

Reference test results previously grouped this file under an import-export
re-export bucket. Current compiler behavior does not reach the later import,
default export, or re-export statements because parsing stops at the first
`export const enum MyConstEnum { ... }`.

Problem: `constEnumNoEmitReexport.ts` is blocked by const-enum declaration
parsing before no-emit re-export behavior can be triaged.

## Current failure

Representative reproduction:

```sh
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/constEnumNoEmitReexport.ts
```

Coverage window:

```sh
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/constEnumNoEmitReexport.ts --detail --no-dashboard-data
```

## Desired final state

This generated bucket is closed as superseded by
`issues/done/5184-parse-const-enum-declarations.md`. Do not implement directly
from this bucket.

## Scope

In scope:

- [x] Inspect fresh smart triage for the affected file.
- [x] Confirm the current first blocker is covered by issue 5184.
- [x] Preserve exact reproduction commands and representative diagnostic/AST
      evidence in this closed issue and the owner issue.

Out of scope:

- Direct implementation from this generated bucket.
- No-emit re-export behavior, default export diagnostics, module graph loading,
  or const-enum inlining.

## Affected paths

Expected:

- `crates/frontend/src/parser/`
- `crates/frontend/src/ast.rs`
- `fixtures/`
- focused parser tests

Do not touch:

- unrelated runtime/backend code unless focused triage proves a backend-only
  blocker after parser support lands

## Acceptance criteria

- [x] Existing issue 5184 is confirmed as the current first-blocker owner.
- [x] This closed issue includes failing path, diagnostic code, source context,
      visible token evidence, and TypeScript AST evidence.
- [x] Completion evidence names the exact reference path and current
      diagnostic/stdout change.

## Validation

Required commands:

```sh
python scripts/manager.py update-issue-index --check
python scripts/manager.py check-issue-health
python scripts/manager.py check-issue-readiness -- --fail-ready-below 80
git diff --check
```

Impacted commands:

```sh
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/constEnumNoEmitReexport.ts --detail --no-dashboard-data
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/constEnumNoEmitReexport.ts
```

Not run:

- `cargo fmt --all --check`; issue cleanup only, no Rust code changed
- `cargo nextest run`; issue cleanup only, no implementation changed

## Docs / current-state / issue sync

Final-state docs:

- [x] not affected

Current state:

- [x] not affected

Follow-up issues:

- [x] none; current first blocker is already tracked by issue 5184

## Notes

## Affected test files

- `reference/typescript/tests/cases/compiler/constEnumNoEmitReexport.ts`

## Duplicate detection

Current first blocker is covered by
`issues/done/5184-parse-const-enum-declarations.md`.

Resolution:

```text
Superseded by issue 5184. The active diagnostic is the const-enum parser
misclassification at the first exported const enum declaration.
```

## Smart triage

### Smart triage: Triage import export: constEnumNoEmitReexport

- Issue class: `triage-needed`
- Feature label: `import-export`
- Diagnostic: `UnsupportedSyntax` / `parser-or-frontend-unsupported`
- Path: `reference/typescript/tests/cases/compiler/constEnumNoEmitReexport.ts`

Reproduction:

```sh
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/constEnumNoEmitReexport.ts
```

Coverage reproduction:

```sh
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/constEnumNoEmitReexport.ts --detail --no-dashboard-data
```

Coverage result:

```text
executed=1
build_pass=0
unsupported=1
unsupported_diagcodes=UnsupportedSyntax:1
unsupported_features=import-export:1
```

Current diagnostic:

```text
UnsupportedSyntax: const declarations require an initializer at 13..17
ast/resolved: const declarations require an initializer at 82..86
```

Source context:

```ts
// @filename: ConstEnum.ts
export const enum MyConstEnum {
    Foo,
    Bar
};
```

Compiler evidence:

- Tokenization succeeds and includes `Export`, `Const`, `Ident("enum")`,
  `Ident("MyConstEnum")`, enum members, then later import/export list/default
  export/re-export tokens.
- AST construction fails before representing the enum declaration.
- No visible symbols are available before the first failure.

TypeScript oracle evidence:

- TypeScript parses the first declaration as
  `EnumDeclaration "export const enum MyConstEnum { ... }"`.
- The oracle then reports later semantic/module diagnostics, including
  duplicate identifier, merged declaration export consistency, and missing
  module diagnostics for `./ConstEnum`, `./ImportExportDefault`,
  `./ReExportDefault`, `./ImportExport`, and `./ReExport`.

## Completion evidence

Commits:

- `...`

Validation result:

```text
command: env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/constEnumNoEmitReexport.ts
result: pass; current first blocker is the same const-enum parser support tracked by issue 5184
date: 2026-05-07

command: env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/constEnumNoEmitReexport.ts --detail --no-dashboard-data
result: pass; executed=1 unsupported=1 unsupported_diagcodes=UnsupportedSyntax:1 unsupported_features=import-export:1
date: 2026-05-07
```

Remaining risks:

- After issue 5184 advances const-enum parsing, this reference file may expose
  no-emit re-export behavior, default export diagnostics, import/export module
  graph support, or const-enum inlining work as later blockers.
