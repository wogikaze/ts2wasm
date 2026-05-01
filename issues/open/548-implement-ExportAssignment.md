---
id: 548
title: "Implement Exportassignment"
type: spike
area: frontend/syntax
class: blocked
priority: P1
depends_on: [5001]
blocks: []
created: 2026-05-01
updated: 2026-05-01
---

## Summary

Triage ExportAssignment across 2 failing reference test cases and split this bucket into implementation-ready child issues.

## Problem

Reference test results show 2 cases fail in directory `ExportAssignment` with diagnostics: import-export. The compiler cannot handle these syntax/semantics, preventing compilation of code in this category.

Problem: ExportAssignment has 2 reference failures and needs smart-triage evidence before implementation starts.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/ExportAssignment8.ts
```

Coverage window:

```sh
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/ExportAssignment8.ts --detail
```

## Desired final state

This generated bucket is either split into implementation-ready child issues or superseded by an existing open/done issue with matching evidence. Do not implement directly from this bucket.

## Scope

In scope:

- [ ] Inspect the smart triage report below
- [ ] Confirm whether existing open/done issues already cover this bucket
- [ ] Split one feature family, one observable behavior, or one fixed reference window into child issues
- [ ] Preserve exact reproduction commands and representative AST/diagnostic evidence in each child issue

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

- [ ] Duplicate candidates below are confirmed as no-match or this issue is superseded
- [ ] At least one child issue contains an exact `mise run reference-triage -- ...` command
- [ ] Child issue includes failing path, diagnostic code, source context, visible symbols, and parser/TypeScript AST evidence
- [ ] Child issue acceptance names the exact fixture/reference path and diagnostic/stdout change

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run
```

Impacted commands:

```sh
mise run reference-coverage -- tsc --limit 4
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/ExportAssignment8.ts --detail
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/ExportAssignment8.ts
```

Not run:

- none

## Docs / current-state / issue sync

Final-state docs:

- [ ] not affected

Current state:

- [ ] updated: `current-state.md` (repo root)

Follow-up issues:

- [ ] none

## Notes

## Affected test files

- `reference/typescript/tests/cases/compiler/ExportAssignment8.ts`
- `reference/typescript/tests/cases/compiler/ExportAssignment7.ts`

## Duplicate detection

- `issues/open/075-implement-ExportAssignment.md` - Implement Exportassignment (same reference path, same feature label, same group key, title overlap)
- `issues/open/131-implement-allowImportClausesToMergeWithTypes.md` - Implement Allowimportclausestomergewithtypes (same feature label, same group key, title overlap)
- `issues/open/136-implement-allowSyntheticDefaultImportsCanPaintCrossModuleDeclaration.md` - Implement Allowsyntheticdefaultimportscanpaintcrossmoduledeclaration (same feature label, same group key, title overlap)
- `issues/open/432-implement-import-export.md` - Implement import/export module syntax (same feature label, same group key, title overlap)
- `issues/open/462-implement-ExportAssignment.md` - Implement Exportassignment (same reference path, same feature label, same group key, title overlap)
- `issues/open/508-implement-aliasWithInterfaceExportAssignmentUsedInVarInitializer.md` - Implement Aliaswithinterfaceexportassignmentusedinvarinitializer (same feature label, same group key, title overlap)
- `issues/open/510-implement-allowImportClausesToMergeWithTypes.md` - Implement Allowimportclausestomergewithtypes (same feature label, same group key, title overlap)
- `issues/open/515-implement-allowSyntheticDefaultImportsCanPaintCrossModuleDeclaration.md` - Implement Allowsyntheticdefaultimportscanpaintcrossmoduledeclaration (same feature label, same group key, title overlap)
- `issues/open/523-implement-ambientExportDefaultErrors.md` - Implement Ambientexportdefaulterrors (same feature label, same group key, title overlap)
- `issues/open/524-implement-ambientExternalModuleInAnotherExternalModule.md` - Implement Ambientexternalmoduleinanotherexternalmodule (same feature label, same group key, title overlap)

## Smart triage

### Smart triage: Triage import export: ExportAssignment8

- Issue class: `triage-needed`
- Feature label: `import-export`
- Diagnostic: `UnsupportedModule` / `unsupported-feature-boundary`
- Path: `reference/typescript/tests/cases/compiler/ExportAssignment8.ts`

Reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/ExportAssignment8.ts
```

Source overview:

```json
{
  "suite": "tsc",
  "bytes": 71,
  "lines": 6,
  "extension": ".ts",
  "first_code_line": "export = B;"
}
```

Failure location:

```json
{
  "code": "UnsupportedModule",
  "message": "issue-055: unsupported static export; module resolution and loading are not implemented at 42..48",
  "span_start": 42,
  "span_end": 48,
  "line": 3,
  "column": 3,
  "feature_label": "import-export",
  "error_type": "unsupported-feature-boundary"
}
```

Source context:

```text
1 | // @target: es2015
2 | // @module: commonjs
3 | export = B;
4 | 
5 | export class C {
6 | }
```

Visible symbols before failure:

```json
[]
```

Duplicate candidates:

```json
[
  {
    "state": "open",
    "path": "issues/open/075-implement-ExportAssignment.md",
    "title": "Implement Exportassignment",
    "reason": "same reference path, same feature label"
  },
  {
    "state": "open",
    "path": "issues/open/432-implement-import-export.md",
    "title": "Implement import/export module syntax",
    "reason": "same feature label, title overlap"
  },
  {
    "state": "open",
    "path": "issues/open/457-implement-APISample-import-export.md",
    "title": "Implement Apisample Import Export",
    "reason": "same feature label, title overlap"
  },
  {
    "state": "open",
    "path": "issues/open/462-implement-ExportAssignment.md",
    "title": "Implement Exportassignment",
    "reason": "same reference path, same feature label"
  },
  {
    "state": "open",
    "path": "issues/open/463-implement-FunctionDeclaration-import-export.md",
    "title": "Implement Functiondeclaration Import Export",
    "reason": "same feature label, title overlap"
  },
  {
    "state": "open",
    "path": "issues/open/543-implement-APISample-import-export.md",
    "title": "Implement Apisample Import Export",
    "reason": "same feature label, title overlap"
  },
  {
    "state": "done",
    "path": "issues/done/055-implement-import-export.md",
    "title": "Umbrella: implement import and export",
    "reason": "same feature label, title overlap"
  }
]
```

Error-specific suggestions:

- Keep module graph behavior separate from parser syntax unless the diagnostic proves syntax is the blocker.

Compiler dumps:

#### tokens

- ok: `True`
- truncated: `False`

```text
== tokens ==
[
    SpannedToken {
        kind: Export,
        span: Span {
            start: 42,
            end: 48,
        },
    },
    SpannedToken {
        kind: Equal,
        span: Span {
            start: 49,
            end: 50,
        },
    },
    SpannedToken {
        kind: Ident(
            "B",
        ),
        span: Span {
            start: 51,
            end: 52,
        },
    },
    SpannedToken {
        kind: Semicolon,
        span: Span {
            start: 52,
            end: 53,
        },
    },
    SpannedToken {
        kind: Export,
        span: Span {
            start: 57,
            end: 63,
        },
    },
    SpannedToken {
        kind: Class,
        span: Span {
            start: 64,
            end: 69,
        },
    },
    SpannedToken {
        kind: Ident(
            "C",
        ),
        span: Span {
            start: 70,
            end: 71,
        },
    },
    SpannedToken {
        kind: LeftBrace,
        span: Span {
            start: 72,
            end: 73,
        },
    },
    SpannedToken {
        kind: RightBrace,
        span: Span {
            start: 75,
            end: 76,
        },
    },
]
```

#### ast

- ok: `False`
- truncated: `False`

```text
error: [UnsupportedModule] issue-055: unsupported static export; module resolution and loading are not implemented at 42..48
```

#### resolved

- ok: `False`
- truncated: `False`

```text
error: [UnsupportedModule] issue-055: unsupported static export; module resolution and loading are not implemented at 42..48
```

TypeScript/JavaScript oracle:

```json
{
  "ok": true,
  "returncode": 0,
  "typescript": {
    "ok": false,
    "diagnostics": [
      {
        "code": 2309,
        "category": "Error",
        "message": "An export assignment cannot be used in a module with other exported elements.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/ExportAssignment8.ts",
        "start": 42,
        "length": 11,
        "line": 3,
        "character": 1
      },
      {
        "code": 2304,
        "category": "Error",
        "message": "Cannot find name 'B'.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/ExportAssignment8.ts",
        "start": 51,
        "length": 1,
        "line": 3,
        "character": 10
      }
    ],
    "hints": [],
    "typescriptVersion": "6.0.3"
  },
  "ast": {
    "topLevel": [
      {
        "kind": "ExportAssignment",
        "text": "export = B;",
        "line": 3,
        "character": 1
      },
      {
        "kind": "ClassDeclaration",
        "text": "export class C {\r\n}",
        "line": 5,
        "character": 1
      }
    ],
    "pathToPosition": [
      {
        "kind": "SourceFile",
        "text": "export = B;\r\n\r\nexport class C {\r\n}",
        "line": 3,
        "character": 1
      },
      {
        "kind": "ExportAssignment",
        "text": "export = B;",
        "line": 3,
        "character": 1
      }
    ]
  }
}
```

Stack trace:

```text
error: [UnsupportedModule] issue-055: unsupported static export; module resolution and loading are not implemented at 42..48
```

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
