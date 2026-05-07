---
id: 1451
title: "Implement Constenumnamespacereferencecausesnoimport"
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
> **Reopened by false-done audit** (2026-05-07)
> Classification: must-reopen
> Reason: Batch-migrated to issues/done/ without implementation commits.
> Evidence: Empty completion evidence. No feat/fix commit for #1451.

## Summary

Closed as split. Fresh triage of the two
`constEnumNamespaceReferenceCausesNoImport` reference files shows two current
frontend blockers:

- `constEnumNamespaceReferenceCausesNoImport.ts` stops at the existing
  `const enum` parser misclassification owned by
  `issues/done/5184-parse-const-enum-declarations.md`.
- `constEnumNamespaceReferenceCausesNoImport2.ts` stops at a top-level
  `export namespace ConstEnumOnlyModule { ... }` static export boundary, split
  to `issues/open/5352-parse-export-namespace-declarations.md`.

## Problem

Reference test results previously grouped both files under one generated
import-export bucket. Fresh evidence shows this bucket mixes at least two
implementation slices and should not be implemented directly.

Problem: `constEnumNamespaceReferenceCausesNoImport` needs to be tracked through
focused const-enum and exported-namespace parser issues instead of one broad
generated bucket.

## Current failure

Representative reproductions:

```sh
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/constEnumNamespaceReferenceCausesNoImport.ts
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/constEnumNamespaceReferenceCausesNoImport2.ts
```

Coverage windows:

```sh
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/constEnumNamespaceReferenceCausesNoImport.ts --detail --no-dashboard-data
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/constEnumNamespaceReferenceCausesNoImport2.ts --detail --no-dashboard-data
```

## Desired final state

This generated bucket is closed. Implementation proceeds through the focused
children and existing owner issues named below.

## Scope

In scope:

- [x] Inspect fresh smart triage for both affected files.
- [x] Confirm the first file is covered by issue 5184.
- [x] Split the exported namespace blocker to issue 5352.
- [x] Preserve exact reproduction commands and representative diagnostic/AST
      evidence in the owner issues.

Out of scope:

- Direct implementation from this generated bucket.
- Full const-enum inlining, CommonJS module loading, import-equals resolution,
  or export-assignment emit.

## Affected paths

Expected:

- `crates/frontend/src/parser/`
- `crates/frontend/src/ast.rs`
- `fixtures/`
- focused CLI/frontend parser tests

Do not touch:

- unrelated runtime/backend code unless focused triage proves a backend-only
  blocker after parser support lands

## Acceptance criteria

- [x] Duplicate/owner candidates are confirmed and this issue is split.
- [x] `issues/done/5184-parse-const-enum-declarations.md` records the
      `constEnumNamespaceReferenceCausesNoImport.ts` first blocker.
- [x] `issues/open/5352-parse-export-namespace-declarations.md` records the
      `constEnumNamespaceReferenceCausesNoImport2.ts` first blocker.
- [x] Completion evidence names the exact reference paths and current
      diagnostics.

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
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/constEnumNamespaceReferenceCausesNoImport.ts --detail --no-dashboard-data
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/constEnumNamespaceReferenceCausesNoImport2.ts --detail --no-dashboard-data
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/constEnumNamespaceReferenceCausesNoImport.ts
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/constEnumNamespaceReferenceCausesNoImport2.ts
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

- [x] created: `issues/open/5352-parse-export-namespace-declarations.md`
- [x] updated: `issues/done/5184-parse-const-enum-declarations.md`

## Notes

## Affected test files

- `reference/typescript/tests/cases/compiler/constEnumNamespaceReferenceCausesNoImport.ts`
- `reference/typescript/tests/cases/compiler/constEnumNamespaceReferenceCausesNoImport2.ts`

## Duplicate detection

- `issues/done/5184-parse-const-enum-declarations.md` covers the first file's
  current `const enum` parser misclassification.
- No existing issue matched the second file's top-level
  `export namespace ConstEnumOnlyModule { ... }` static export boundary; issue
  5352 was created for that slice.

## Smart triage

### `constEnumNamespaceReferenceCausesNoImport.ts`

- Diagnostic: `UnsupportedSyntax` / `parser-or-frontend-unsupported`
- Current message: `const declarations require an initializer at 112..116`
- First failing source: `export const enum ConstFooEnum {`
- Token evidence includes `Export`, `Const`, `Ident("enum")`,
  `Ident("ConstFooEnum")`, enum members, `export function fooFunc`, and
  `import * as Foo from "./foo"`.
- TypeScript AST parses the first declaration as `EnumDeclaration` with
  `ExportKeyword`; TypeScript's later diagnostic is TS2307 for missing
  `./foo`.

### `constEnumNamespaceReferenceCausesNoImport2.ts`

- Diagnostic: `UnsupportedModule` / `unsupported-feature-boundary`
- Current message: `issue-055: unsupported static export; module resolution and loading are not implemented at 30..36`
- First failing source: `export namespace ConstEnumOnlyModule {`
- Token evidence includes `Export`, `Ident("namespace")`,
  `Ident("ConstEnumOnlyModule")`, nested `export const enum ConstFooEnum`,
  `import * as Foo from "./foo"`, `export = Foo.ConstEnumOnlyModule`, and
  `import Foo = require("./reexport")`.
- TypeScript AST parses the first declaration as `ModuleDeclaration`, then also
  sees `ImportDeclaration`, `ExportAssignment`, `ImportEqualsDeclaration`, and
  `FunctionDeclaration`.

## Completion evidence

Commits:

- `...`

Validation result:

```text
command: env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/constEnumNamespaceReferenceCausesNoImport.ts
result: pass; current first blocker is the existing const-enum parser issue 5184
date: 2026-05-07

command: env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/constEnumNamespaceReferenceCausesNoImport2.ts
result: pass; current first blocker split to exported-namespace parser issue 5352
date: 2026-05-07

command: env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/constEnumNamespaceReferenceCausesNoImport.ts --detail --no-dashboard-data
result: pass; executed=1 unsupported=1 unsupported_diagcodes=UnsupportedSyntax:1 unsupported_features=import-export:1
date: 2026-05-07

command: env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/constEnumNamespaceReferenceCausesNoImport2.ts --detail --no-dashboard-data
result: pass; executed=1 unsupported=1 unsupported_diagcodes=UnsupportedSyntax:1 unsupported_features=import-export:1
date: 2026-05-07
```

Remaining risks:

- After issue 5184 and issue 5352 advance parsing, these files may expose
  nested const-enum semantics, namespace export binding, `export =` diagnostics,
  import-equals resolution, or missing local module handling.
