---
id: 1138
title: "Implement Checkjsdoctypetagonexportassignment"
type: spike
area: frontend/syntax
class: done
priority: P1
depends_on: [5229]
blocks: []
created: 2026-05-01
updated: 2026-05-06
completed: 2026-05-06
---
> **Reopened by false-done audit** (2026-05-07)
> Classification: must-reopen
> Reason: Batch-migrated to issues/done/ without implementation commits.
> Evidence: Empty completion evidence. No feat/fix commit for #1138.

## Summary

Triage checkJsdocTypeTagOnExportAssignment across 8 failing reference test cases and split this bucket into implementation-ready child issues.

## Problem

Reference test results show 8 cases fail in directory `checkJsdocTypeTagOnExportAssignment` with diagnostics: import-export. The compiler cannot handle these syntax/semantics, preventing compilation of code in this category.

Problem: checkJsdocTypeTagOnExportAssignment has 8 reference failures and needs smart-triage evidence before implementation starts.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/checkJsdocTypeTagOnExportAssignment2.ts
```

Coverage window:

```sh
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/checkJsdocTypeTagOnExportAssignment2.ts --detail
```

## Desired final state

This generated bucket is either split into implementation-ready child issues or superseded by an existing open/done issue with matching evidence. Do not implement directly from this bucket.

## Scope

In scope:

- [x] Inspect the smart triage report below
- [x] Confirm whether existing open/done issues already cover this bucket
- [x] Split one feature family, one observable behavior, or one fixed reference window into child issues
- [x] Preserve exact reproduction commands and representative AST/diagnostic evidence in each child issue

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
- [x] At least one child issue contains an exact `reference-triage` command
- [x] Child issue includes failing path, diagnostic code, source context, visible symbols, and parser/TypeScript AST evidence
- [x] Child issue acceptance names the exact fixture/reference path and diagnostic/stdout change

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run
```

Impacted commands:

```sh
mise run reference-coverage -- tsc --limit 16
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/checkJsdocTypeTagOnExportAssignment2.ts --detail
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/checkJsdocTypeTagOnExportAssignment2.ts
```

Not run:

- none

## Docs / current-state / issue sync

Final-state docs:

- [x] not affected

Current state:

- [x] not affected

Follow-up issues:

- [x] created: `issues/done/5229-w0-user-runtime-string-origin.md`

## Notes

## Affected test files

- `reference/typescript/tests/cases/compiler/checkJsdocTypeTagOnExportAssignment2.ts`
- `reference/typescript/tests/cases/compiler/checkJsdocTypeTagOnExportAssignment3.ts`
- `reference/typescript/tests/cases/compiler/checkJsdocTypeTagOnExportAssignment1.ts`
- `reference/typescript/tests/cases/compiler/checkJsdocTypeTagOnExportAssignment4.ts`
- `reference/typescript/tests/cases/compiler/checkJsdocTypeTagOnExportAssignment6.ts`
- `reference/typescript/tests/cases/compiler/checkJsdocTypeTagOnExportAssignment5.ts`
- `reference/typescript/tests/cases/compiler/checkJsdocTypeTagOnExportAssignment8.ts`
- `reference/typescript/tests/cases/compiler/checkJsdocTypeTagOnExportAssignment7.ts`

## Duplicate detection

Fresh duplicate scan found related import/export umbrella issues and export
assignment parser issue 5186, but the current blocker is narrower: resolving
local imports between `@Filename` virtual sections.

No-match rationale:

- `issues/done/5186-parse-export-assignment-for-diagnostics.md` covers
  `export = expr` syntax, while this representative already builds AST for
  `export default` and `import b from "./b"`.
- `issues/done/5127-implement-export-default-multifile-lowering.md` fixed
  duplicate default-export locals in multi-section files, but this
  representative fails earlier in module graph resolution for a sibling
  virtual section.
- Broad import/export buckets are not implementation-ready work orders.

## Smart triage

Fresh triage shows this generated import-export bucket has advanced past
frontend parsing. The first blocker is module graph resolution for virtual
`@Filename` sections.

### Smart triage: checkJsdocTypeTagOnExportAssignment2

- Issue class: `triage-needed`
- Feature label: `import-export`
- Diagnostic: `UnsupportedModule` / `module-resolution`
- Current compiler message: `issue-232: missing local module ./b`
- Path: `reference/typescript/tests/cases/compiler/checkJsdocTypeTagOnExportAssignment2.ts`

Reproduction:

```sh
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/checkJsdocTypeTagOnExportAssignment2.ts
```

Coverage:

```sh
python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/checkJsdocTypeTagOnExportAssignment2.ts --detail --no-dashboard-data
```

Coverage result:

```text
unsupported=1
unsupported_diagcodes=UnsupportedModule:1
unsupported_features=import-export:1
```

Source context:

```ts
// @Filename: b.js
/** @type {import("./a").Foo} */
export default { c: false };

// @Filename: c.js
import b from "./b";
b;
```

Compiler evidence:

```text
tokens: ok
ast: ok; ExportDefault, ImportDefault("./b"), Expr Ident("b")
module_graph: issue-232 missing local module ./b; tried on-disk b.ts/b.js/etc.
TypeScript oracle: TS2307 for ./b in this reference window
```

Split result:

- `issues/done/5229-w0-user-runtime-string-origin.md`

## Completion evidence

Fill only when moving to `done/`.

Commits:

- filled by closing commit

Validation result:

```text
command: python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/checkJsdocTypeTagOnExportAssignment2.ts --detail --no-dashboard-data
result: pass; reproduced UnsupportedModule/import-export blocker
date: 2026-05-06

command: python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/checkJsdocTypeTagOnExportAssignment2.ts
result: pass; reproduced issue-232 missing virtual ./b module and split to issue 5229
date: 2026-05-06
```

Remaining risks:

- Implementing issue 5229 may expose later JSDoc import type or export default semantic blockers across the remaining variants.
