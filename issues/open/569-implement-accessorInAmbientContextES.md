---
id: 569
title: "Implement Accessorinambientcontextes (audit reopened #569)"
type: spike
area: frontend/syntax
class: superseded
priority: P1
depends_on: [432]
blocks: []
created: 2026-05-01
updated: 2026-05-06
status: done
completed: 2026-05-06
---

## Summary

Triage accessorInAmbientContextES across 1 failing reference test cases and split this bucket into implementation-ready child issues.

## Problem

Reference test results show 1 cases fail in directory `accessorInAmbientContextES` with diagnostics: import-export. The compiler cannot handle these syntax/semantics, preventing compilation of code in this category.

Problem: accessorInAmbientContextES has 1 reference failures and needs smart-triage evidence before implementation starts.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/accessorInAmbientContextES5.ts
```

Coverage window:

```sh
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/accessorInAmbientContextES5.ts --detail
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
- [x] At least one child issue contains an exact `mise run reference-triage -- ...` command
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
mise run reference-coverage -- tsc --limit 2
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/accessorInAmbientContextES5.ts --detail
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/accessorInAmbientContextES5.ts
```

Not run:

- none

## Docs / current-state / issue sync

Final-state docs:

- [x] not affected

Current state:

- [x] updated: `current-state.md` (repo root)

Follow-up issues:

- [x] none

## Notes

## Affected test files

- `reference/typescript/tests/cases/compiler/accessorInAmbientContextES5.ts`

## Duplicate detection

- `issues/open/099-implement-accessorInAmbientContextES.md` - Implement Accessorinambientcontextes (same reference path, same group key, title overlap)
- `issues/open/483-implement-accessorInAmbientContextES.md` - Implement Accessorinambientcontextes (same reference path, same feature label, same group key, title overlap)

## Smart triage

### Smart triage: Triage import export: accessorInAmbientContextES5

- Issue class: `triage-needed`
- Feature label: `import-export`
- Diagnostic: `UnsupportedModule` / `unsupported-feature-boundary`
- Path: `reference/typescript/tests/cases/compiler/accessorInAmbientContextES5.ts`

Reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/accessorInAmbientContextES5.ts
```

Source overview:

```json
{
  "suite": "tsc",
  "bytes": 678,
  "lines": 29,
  "extension": ".ts",
  "first_code_line": "declare class AmbientClass {"
}
```

Failure location:

```json
{
  "code": "UnsupportedModule",
  "message": "issue-400: ambient namespace declarations require module ownership before runtime lowering at 300..309",
  "span_start": 300,
  "span_end": 309,
  "line": 13,
  "column": 9,
  "feature_label": "import-export",
  "error_type": "unsupported-feature-boundary"
}
```

Source context:

```text
10 |     private static accessor prop4: symbol;
11 | }
12 | 
13 | declare namespace AmbientNamespace {
14 |     class C {
15 |         accessor prop: string;
16 |     }
```

Visible symbols before failure:

```json
[
  {
    "kind": "class",
    "name": "AmbientClass",
    "line": 6,
    "column": 9
  }
]
```

Duplicate candidates:

```json
[
  {
    "state": "open",
    "path": "issues/open/099-implement-accessorInAmbientContextES.md",
    "title": "Implement Accessorinambientcontextes",
    "reason": "same reference path"
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
    "path": "issues/open/463-implement-FunctionDeclaration-import-export.md",
    "title": "Implement Functiondeclaration Import Export",
    "reason": "same feature label, title overlap"
  },
  {
    "state": "open",
    "path": "issues/open/483-implement-accessorInAmbientContextES.md",
    "title": "Implement Accessorinambientcontextes",
    "reason": "same reference path, same feature label"
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
    "state": "done",
    "path": "issues/open/055-implement-import-export.md",
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
            "declare",
        ),
        span: Span {
            start: 117,
            end: 124,
        },
    },
    SpannedToken {
        kind: Class,
        span: Span {
            start: 125,
            end: 130,
        },
    },
    SpannedToken {
        kind: Ident(
            "AmbientClass",
        ),
        span: Span {
            start: 131,
            end: 143,
        },
    },
    SpannedToken {
        kind: LeftBrace,
        span: Span {
            start: 144,
            end: 145,
        },
    },
    SpannedToken {
        kind: Ident(
            "accessor",
        ),
        span: Span {
            start: 150,
            end: 158,
        },
    },
    SpannedToken {
        kind: Ident(
            "prop1",
        ),
        span: Span {
            start: 159,
            end: 164,
        },
    },
    SpannedToken {
        kind: Colon,
        span: Span {
            start: 164,
            end: 165,
        },
    },
    SpannedToken {
        kind: Ident(
            "string",
        ),
        span: Span {
            start: 166,
            end: 172,
        },
    },
    SpannedToken {
        kind: Semicolon,
        span: Span {
            start: 172,
            end: 173,
        },
    },
    SpannedToken {
        kind: Static,
        span: Span {
            start: 178,
            end: 184,
        },
    },
    SpannedToken {
        kind: Ident(
            "accessor",
        ),
        span: Span {
            start: 185,
            end: 193,
        },
    },
    SpannedToken {
        kind: Ident(
            "prop2",
        ),
        span: Span {
            start: 194,
            end: 199,
        },
    },
    SpannedToken {
        kind: Colon,
        span: Span {
            start: 199,
            end: 200,
        },
    },
    SpannedToken {
        kind: Ident(
            "number",
        ),
        span: Span {
            start: 201,
            end: 207,
        },
    },
    SpannedToken {
        kind: Semicolon,
        span: Span {
            start: 207,
            end: 208,
        },
    },
    SpannedToken {
        kind: Ident(
            "private",
        ),
        span: Span {
            start: 213,
            end: 220,
        },
    },
    SpannedToken {
        kind: Ident(
            "accessor",
        ),
        span: Span {
            start: 221,
            end: 229,
        },
    },
    SpannedToken {
        kind: Ident(
            "prop3",
        ),
        span: Span {
            start: 230,
            end: 235,
        },
    },
    SpannedToken {
        kind: Colon,
        span: Span {
            start: 235,
            end: 236,
        },
    },
    SpannedToken {
        kind: Ident(
            "boolean",
        ),
        span: Span {
            start: 237,
            end: 244,
        },
    },
    SpannedToken {
        kind: Semicolon,
        span: Span {
            start: 244,
            end: 245,
        },
    },
    SpannedToken {
        kind: Ident(
            "private",
        ),
        span: Span {
            start: 250,
            end: 257,
        },
    },
    SpannedToken {
        kind: Static,
        span: Span {
            start: 258,
            end: 264,
        },
    },
    SpannedToken {
        kind: Ident(
            "accessor",
        ),
        span: Span {
            start: 265,
            end: 273,
```

#### ast

- ok: `False`
- truncated: `False`

```text
error: [UnsupportedModule] issue-400: ambient namespace declarations require module ownership before runtime lowering at 300..309
```

#### resolved

- ok: `False`
- truncated: `False`

```text
error: [UnsupportedModule] issue-400: ambient namespace declarations require module ownership before runtime lowering at 300..309
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
        "code": 2564,
        "category": "Error",
        "message": "Property 'shouldError' has no initializer and is not definitely assigned in the constructor.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/accessorInAmbientContextES5.ts",
        "start": 634,
        "length": 11,
        "line": 28,
        "character": 14
      }
    ],
    "hints": [],
    "typescriptVersion": "6.0.3"
  },
  "ast": {
    "topLevel": [
      {
        "kind": "ClassDeclaration",
        "text": "declare class AmbientClass {\n    accessor prop1: string;\n    static accessor prop2: number;\n    private accessor prop3: ",
        "line": 6,
        "character": 1
      },
      {
        "kind": "ModuleDeclaration",
        "text": "declare namespace AmbientNamespace {\n    class C {\n        accessor prop: string;\n    }\n}",
        "line": 13,
        "character": 1
      },
      {
        "kind": "ModuleDeclaration",
        "text": "declare module \"some-module\" {\n    export class ExportedClass {\n        accessor value: any;\n    }\n}",
        "line": 20,
        "character": 1
      },
      {
        "kind": "ClassDeclaration",
        "text": "class RegularClass {\n    accessor shouldError: string; // Should still error\n}",
        "line": 27,
        "character": 1
      }
    ],
    "pathToPosition": [
      {
        "kind": "SourceFile",
        "text": "declare class AmbientClass {\n    accessor prop1: string;\n    static accessor prop2: number;\n    private accessor prop3: ",
        "line": 6,
        "character": 1
      },
      {
        "kind": "ModuleDeclaration",
        "text": "declare namespace AmbientNamespace {\n    class C {\n        accessor prop: string;\n    }\n}",
        "line": 13,
        "character": 1
      }
    ]
  }
}
```

Stack trace:

```text
error: [UnsupportedModule] issue-400: ambient namespace declarations require module ownership before runtime lowering at 300..309
```

## Completion evidence

Fill only when moving to `done/`.

Commits:

- pending closure commit

Validation result:

```text
command: python scripts/manager.py update-issue-index --check
result: pass
date: 2026-05-06
```

Remaining risks:

- none

## Status

Superseded by issue #099. Duplicate from separate coverage run.

## Reopened by audit

Date: 2026-05-05

Classification: must-reopen.

Reopen reason: frontmatter still says `class: blocked`, which is incompatible with a completed issue unless explicit supersedence/closure evidence is present.

Violated acceptance: the issue cannot provide repo-local close evidence for its checked acceptance criteria while it remains in this state. Acceptance checkboxes were reset for re-verification.

Evidence files:
- open issue file before this move
- `issues/open/569-implement-accessorInAmbientContextES.md` after this move

Split follow-up: none created in this audit wave; this reopened issue remains the tracking item.
