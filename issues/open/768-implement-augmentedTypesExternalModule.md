---
id: 768
title: "Implement Augmentedtypesexternalmodule"
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

Triage augmentedTypesExternalModule across 1 failing reference test cases and split this bucket into implementation-ready child issues.

## Problem

Reference test results show 1 cases fail in directory `augmentedTypesExternalModule` with diagnostics: import-export. The compiler cannot handle these syntax/semantics, preventing compilation of code in this category.

Problem: augmentedTypesExternalModule has 1 reference failures and needs smart-triage evidence before implementation starts.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/augmentedTypesExternalModule1.ts
```

Coverage window:

```sh
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/augmentedTypesExternalModule1.ts --detail
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
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/augmentedTypesExternalModule1.ts --detail
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/augmentedTypesExternalModule1.ts
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

- `reference/typescript/tests/cases/compiler/augmentedTypesExternalModule1.ts`

## Duplicate detection

- none found by path/title/feature scan

## Smart triage

### Smart triage: Triage import export: augmentedTypesExternalModule1

- Issue class: `triage-needed`
- Feature label: `import-export`
- Diagnostic: `UnsupportedModule` / `unsupported-feature-boundary`
- Path: `reference/typescript/tests/cases/compiler/augmentedTypesExternalModule1.ts`

Reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/augmentedTypesExternalModule1.ts
```

Source overview:

```json
{
  "suite": "tsc",
  "bytes": 125,
  "lines": 5,
  "extension": ".ts",
  "first_code_line": "export var a = 1;"
}
```

Failure location:

```json
{
  "code": "UnsupportedModule",
  "message": "issue-055: unsupported variable export; module resolution and loading are not implemented at 36..42",
  "span_start": 36,
  "span_end": 42,
  "line": 3,
  "column": 3,
  "feature_label": "import-export",
  "error_type": "unsupported-feature-boundary"
}
```

Source context:

```text
1 | // @target: es2015
2 | //@module: amd
3 | export var a = 1;
4 | class c5 { public foo() { } }
5 | namespace c5 { } // should be ok everywhere
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
    "path": "issues/done/549-implement-FunctionDeclaration-import-export.md",
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
    "state": "open",
    "path": "issues/open/732-implement-assignmentCompatability-import-export.md",
    "title": "Implement Assignmentcompatability Import Export",
    "reason": "same feature label, title overlap"
  },
  {
    "state": "open",
    "path": "issues/open/766-implement-augmentedTypesEnum-import-export.md",
    "title": "Implement Augmentedtypesenum Import Export",
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
            start: 36,
            end: 42,
        },
    },
    SpannedToken {
        kind: Var,
        span: Span {
            start: 43,
            end: 46,
        },
    },
    SpannedToken {
        kind: Ident(
            "a",
        ),
        span: Span {
            start: 47,
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
        kind: Number(
            1,
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
        kind: Class,
        span: Span {
            start: 55,
            end: 60,
        },
    },
    SpannedToken {
        kind: Ident(
            "c5",
        ),
        span: Span {
            start: 61,
            end: 63,
        },
    },
    SpannedToken {
        kind: LeftBrace,
        span: Span {
            start: 64,
            end: 65,
        },
    },
    SpannedToken {
        kind: Ident(
            "public",
        ),
        span: Span {
            start: 66,
            end: 72,
        },
    },
    SpannedToken {
        kind: Ident(
            "foo",
        ),
        span: Span {
            start: 73,
            end: 76,
        },
    },
    SpannedToken {
        kind: LeftParen,
        span: Span {
            start: 76,
            end: 77,
        },
    },
    SpannedToken {
        kind: RightParen,
        span: Span {
            start: 77,
            end: 78,
        },
    },
    SpannedToken {
        kind: LeftBrace,
        span: Span {
            start: 79,
            end: 80,
        },
    },
    SpannedToken {
        kind: RightBrace,
        span: Span {
            start: 81,
            end: 82,
        },
    },
    SpannedToken {
        kind: RightBrace,
        span: Span {
            start: 83,
            end: 84,
        },
    },
    SpannedToken {
        kind: Ident(
            "namespace",
        ),
        span: Span {
            start: 86,
            end: 95,
        },
    },
    SpannedToken {
        kind: Ident(
            "c5",
        ),
        span: Span {
            start: 96,
            end: 98,
        },
    },
    SpannedToken {
        kind: LeftBrace,
        span: Span {
            start: 99,
            end: 100,
        },
    },
    SpannedToken {
        kind: RightBrace,
        span: Span {
            start: 101,
            end: 102,
        },
    },
]
```

#### ast

- ok: `False`
- truncated: `False`

```text
error: [UnsupportedModule] issue-055: unsupported variable export; module resolution and loading are not implemented at 36..42
```

#### resolved

- ok: `False`
- truncated: `False`

```text
error: [UnsupportedModule] issue-055: unsupported variable export; module resolution and loading are not implemented at 36..42
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
        "typeText": "number",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/augmentedTypesExternalModule1.ts",
        "start": 47,
        "length": 1,
        "line": 3,
        "character": 12,
        "name": "a"
      }
    ],
    "typescriptVersion": "6.0.3"
  },
  "ast": {
    "topLevel": [
      {
        "kind": "FirstStatement",
        "text": "export var a = 1;",
        "line": 3,
        "character": 1
      },
      {
        "kind": "ClassDeclaration",
        "text": "class c5 { public foo() { } }",
        "line": 4,
        "character": 1
      },
      {
        "kind": "ModuleDeclaration",
        "text": "namespace c5 { }",
        "line": 5,
        "character": 1
      }
    ],
    "pathToPosition": [
      {
        "kind": "SourceFile",
        "text": "export var a = 1;\r\nclass c5 { public foo() { } }\r\nnamespace c5 { } // should be ok everywhere",
        "line": 3,
        "character": 1
      },
      {
        "kind": "FirstStatement",
        "text": "export var a = 1;",
        "line": 3,
        "character": 1
      },
      {
        "kind": "ExportKeyword",
        "text": "export",
        "line": 3,
        "character": 1
      }
    ]
  }
}
```

Stack trace:

```text
error: [UnsupportedModule] issue-055: unsupported variable export; module resolution and loading are not implemented at 36..42
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
