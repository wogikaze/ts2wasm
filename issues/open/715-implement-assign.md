---
id: 715
title: "Implement Assign"
type: spike
area: frontend/syntax
class: triage-needed
priority: P1
depends_on: []
blocks: []
created: 2026-05-01
updated: 2026-05-01
---

## Summary

Triage assign across 1 failing reference test cases and split this bucket into implementation-ready child issues.

## Problem

Reference test results show 1 cases fail in directory `assign` with diagnostics: import-export. The compiler cannot handle these syntax/semantics, preventing compilation of code in this category.

Problem: assign has 1 reference failures and needs smart-triage evidence before implementation starts.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/assign1.ts
```

Coverage window:

```sh
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/assign1.ts --detail
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
mise run reference-coverage -- tsc --limit 2
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/assign1.ts --detail
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/assign1.ts
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

- `reference/typescript/tests/cases/compiler/assign1.ts`

## Duplicate detection

- `issues/open/075-implement-ExportAssignment.md` - Implement Exportassignment (same feature label, same group key, title overlap)
- `issues/open/135-implement-allowSyntheticDefaultImports.md` - Implement Allowsyntheticdefaultimports (same feature label, same group key, title overlap)
- `issues/open/432-implement-import-export.md` - Implement import/export module syntax (same feature label, same group key, title overlap)
- `issues/open/462-implement-ExportAssignment.md` - Implement Exportassignment (same feature label, same group key, title overlap)
- `issues/open/483-implement-accessorInAmbientContextES.md` - Implement Accessorinambientcontextes (same feature label, same group key, title overlap)
- `issues/open/491-implement-aliasAssignments.md` - Implement Aliasassignments (same feature label, same group key, title overlap)
- `issues/open/493-implement-aliasDoesNotDuplicateSignatures.md` - Implement Aliasdoesnotduplicatesignatures (same feature label, same group key, title overlap)
- `issues/open/498-implement-aliasUsageInAccessorsOfClass.md` - Implement Aliasusageinaccessorsofclass (same feature label, same group key, title overlap)
- `issues/open/499-implement-aliasUsageInArray.md` - Implement Aliasusageinarray (same feature label, same group key, title overlap)
- `issues/open/500-implement-aliasUsageInFunctionExpression.md` - Implement Aliasusageinfunctionexpression (same feature label, same group key, title overlap)

## Smart triage

### Smart triage: Triage import export: assign1

- Issue class: `triage-needed`
- Feature label: `import-export`
- Diagnostic: `UnsupportedModule` / `unsupported-feature-boundary`
- Path: `reference/typescript/tests/cases/compiler/assign1.ts`

Reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/assign1.ts
```

Source overview:

```json
{
  "suite": "tsc",
  "bytes": 135,
  "lines": 9,
  "extension": ".ts",
  "first_code_line": "namespace M {"
}
```

Failure location:

```json
{
  "code": "UnsupportedModule",
  "message": "issue-399: TypeScript namespace/internal module declarations require module ownership before runtime lowering at 20..29",
  "span_start": 20,
  "span_end": 29,
  "line": 2,
  "column": 2,
  "feature_label": "import-export",
  "error_type": "unsupported-feature-boundary"
}
```

Source context:

```text
1 | // @target: es2015
2 | namespace M {
3 |     interface I {
4 |         salt:number;
5 |         pepper:number;
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
    "state": "open",
    "path": "issues/open/549-implement-FunctionDeclaration-import-export.md",
    "title": "Implement Functiondeclaration Import Export",
    "reason": "same feature label, title overlap"
  },
  {
    "state": "open",
    "path": "issues/open/662-implement-arrayAssignmentTest-import-export.md",
    "title": "Implement Arrayassignmenttest Import Export",
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
- truncated: `True`

```text
== tokens ==
[
    SpannedToken {
        kind: Ident(
            "namespace",
        ),
        span: Span {
            start: 20,
            end: 29,
        },
    },
    SpannedToken {
        kind: Ident(
            "M",
        ),
        span: Span {
            start: 30,
            end: 31,
        },
    },
    SpannedToken {
        kind: LeftBrace,
        span: Span {
            start: 32,
            end: 33,
        },
    },
    SpannedToken {
        kind: Ident(
            "interface",
        ),
        span: Span {
            start: 39,
            end: 48,
        },
    },
    SpannedToken {
        kind: Ident(
            "I",
        ),
        span: Span {
            start: 49,
            end: 50,
        },
    },
    SpannedToken {
        kind: LeftBrace,
        span: Span {
            start: 51,
            end: 52,
        },
    },
    SpannedToken {
        kind: Ident(
            "salt",
        ),
        span: Span {
            start: 62,
            end: 66,
        },
    },
    SpannedToken {
        kind: Colon,
        span: Span {
            start: 66,
            end: 67,
        },
    },
    SpannedToken {
        kind: Ident(
            "number",
        ),
        span: Span {
            start: 67,
            end: 73,
        },
    },
    SpannedToken {
        kind: Semicolon,
        span: Span {
            start: 73,
            end: 74,
        },
    },
    SpannedToken {
        kind: Ident(
            "pepper",
        ),
        span: Span {
            start: 84,
            end: 90,
        },
    },
    SpannedToken {
        kind: Colon,
        span: Span {
            start: 90,
            end: 91,
        },
    },
    SpannedToken {
        kind: Ident(
            "number",
        ),
        span: Span {
            start: 91,
            end: 97,
        },
    },
    SpannedToken {
        kind: Semicolon,
        span: Span {
            start: 97,
            end: 98,
        },
    },
    SpannedToken {
        kind: RightBrace,
        span: Span {
            start: 104,
            end: 105,
        },
    },
    SpannedToken {
        kind: Var,
        span: Span {
            start: 113,
            end: 116,
        },
    },
    SpannedToken {
        kind: Ident(
            "x",
        ),
        span: Span {
            start: 117,
            end: 118,
        },
    },
    SpannedToken {
        kind: Colon,
        span: Span {
            start: 118,
            end: 119,
        },
    },
    SpannedToken {
        kind: Ident(
            "I",
        ),
        span: Span {
            start: 119,
            end: 120,
        },
    },
    SpannedToken {
        kind: Equal,
        span: Span {
            start: 120,
            end: 121,
        },
    },
    SpannedToken {
        kind: LeftBrace,
        span: Span {
            start: 121,
            end: 122,
        },
    },
    SpannedToken {
        kind: Ident(
            "salt",
        ),
        span: Span {
            start: 122,
            end: 126,
        },
    },
    SpannedToken {
        kind: Colon,
        span: Span {
            start: 126,
            end: 127,
        },
    },
    SpannedToken {
        kind: Number(
            2,
        ),
        span: Span {
            start: 127,
            end: 128,
        },
    },
    SpannedToken {
        kind: Comma,
        span: Span {
            start: 128,
            end: 129,
        },
```

#### ast

- ok: `False`
- truncated: `False`

```text
error: [UnsupportedModule] issue-399: TypeScript namespace/internal module declarations require module ownership before runtime lowering at 20..29
```

#### resolved

- ok: `False`
- truncated: `False`

```text
error: [UnsupportedModule] issue-399: TypeScript namespace/internal module declarations require module ownership before runtime lowering at 20..29
```

TypeScript/JavaScript oracle:

```json
{
  "ok": true,
  "returncode": 0,
  "typescript": {
    "ok": true,
    "diagnostics": [],
    "hints": [
      {
        "kind": "binding",
        "typeText": "I",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/assign1.ts",
        "start": 117,
        "length": 1,
        "line": 8,
        "character": 9,
        "name": "x"
      }
    ],
    "typescriptVersion": "6.0.3"
  },
  "ast": {
    "topLevel": [
      {
        "kind": "ModuleDeclaration",
        "text": "namespace M {\r\n    interface I {\r\n        salt:number;\r\n        pepper:number;\r\n    }\r\n\r\n    var x:I={salt:2,pepper:0};\r",
        "line": 2,
        "character": 1
      }
    ],
    "pathToPosition": [
      {
        "kind": "SourceFile",
        "text": "namespace M {\r\n    interface I {\r\n        salt:number;\r\n        pepper:number;\r\n    }\r\n\r\n    var x:I={salt:2,pepper:0};\r",
        "line": 2,
        "character": 1
      },
      {
        "kind": "ModuleDeclaration",
        "text": "namespace M {\r\n    interface I {\r\n        salt:number;\r\n        pepper:number;\r\n    }\r\n\r\n    var x:I={salt:2,pepper:0};\r",
        "line": 2,
        "character": 1
      }
    ]
  }
}
```

Stack trace:

```text
error: [UnsupportedModule] issue-399: TypeScript namespace/internal module declarations require module ownership before runtime lowering at 20..29
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
