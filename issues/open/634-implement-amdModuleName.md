---
id: 634
title: "Implement Amdmodulename"
type: spike
area: frontend/syntax
class: blocked
priority: P1
depends_on: [432]
blocks: []
created: 2026-05-01
updated: 2026-05-01
---

## Summary

Triage amdModuleName across 2 failing reference test cases and split this bucket into implementation-ready child issues.

## Problem

Reference test results show 2 cases fail in directory `amdModuleName` with diagnostics: import-export. The compiler cannot handle these syntax/semantics, preventing compilation of code in this category.

Problem: amdModuleName has 2 reference failures and needs smart-triage evidence before implementation starts.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/amdModuleName1.ts
```

Coverage window:

```sh
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/amdModuleName1.ts --detail
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
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/amdModuleName1.ts --detail
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/amdModuleName1.ts
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

- `reference/typescript/tests/cases/compiler/amdModuleName1.ts`
- `reference/typescript/tests/cases/compiler/amdModuleName2.ts`

## Duplicate detection

- `issues/done/176-implement-amdModuleName.md` - Implement Amdmodulename (same reference path, same group key, title overlap)

## Smart triage

### Smart triage: Triage import export: amdModuleName1

- Issue class: `triage-needed`
- Feature label: `import-export`
- Diagnostic: `UnsupportedModule` / `unsupported-feature-boundary`
- Path: `reference/typescript/tests/cases/compiler/amdModuleName1.ts`

Reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/amdModuleName1.ts
```

Source overview:

```json
{
  "suite": "tsc",
  "bytes": 159,
  "lines": 10,
  "extension": ".ts",
  "first_code_line": "class Foo {"
}
```

Failure location:

```json
{
  "code": "UnsupportedModule",
  "message": "issue-055: unsupported static export; module resolution and loading are not implemented at 145..151",
  "span_start": 145,
  "span_end": 151,
  "line": 10,
  "column": 1,
  "feature_label": "import-export",
  "error_type": "unsupported-feature-boundary"
}
```

Source context:

```text
 7 |         this.x = 5;
 8 |     }
 9 | }
10 | export = Foo;
```

Visible symbols before failure:

```json
[
  {
    "kind": "class",
    "name": "Foo",
    "line": 4,
    "column": 1
  }
]
```

Duplicate candidates:

```json
[
  {
    "state": "open",
    "path": "issues/done/176-implement-amdModuleName.md",
    "title": "Implement Amdmodulename",
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
    "path": "issues/done/457-implement-APISample-import-export.md",
    "title": "Implement Apisample Import Export",
    "reason": "same feature label, title overlap"
  },
  {
    "state": "open",
    "path": "issues/done/463-implement-FunctionDeclaration-import-export.md",
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
        kind: Class,
        span: Span {
            start: 70,
            end: 75,
        },
    },
    SpannedToken {
        kind: Ident(
            "Foo",
        ),
        span: Span {
            start: 76,
            end: 79,
        },
    },
    SpannedToken {
        kind: LeftBrace,
        span: Span {
            start: 80,
            end: 81,
        },
    },
    SpannedToken {
        kind: Ident(
            "x",
        ),
        span: Span {
            start: 86,
            end: 87,
        },
    },
    SpannedToken {
        kind: Colon,
        span: Span {
            start: 87,
            end: 88,
        },
    },
    SpannedToken {
        kind: Ident(
            "number",
        ),
        span: Span {
            start: 89,
            end: 95,
        },
    },
    SpannedToken {
        kind: Semicolon,
        span: Span {
            start: 95,
            end: 96,
        },
    },
    SpannedToken {
        kind: Ident(
            "constructor",
        ),
        span: Span {
            start: 101,
            end: 112,
        },
    },
    SpannedToken {
        kind: LeftParen,
        span: Span {
            start: 112,
            end: 113,
        },
    },
    SpannedToken {
        kind: RightParen,
        span: Span {
            start: 113,
            end: 114,
        },
    },
    SpannedToken {
        kind: LeftBrace,
        span: Span {
            start: 115,
            end: 116,
        },
    },
    SpannedToken {
        kind: This,
        span: Span {
            start: 125,
            end: 129,
        },
    },
    SpannedToken {
        kind: Dot,
        span: Span {
            start: 129,
            end: 130,
        },
    },
    SpannedToken {
        kind: Ident(
            "x",
        ),
        span: Span {
            start: 130,
            end: 131,
        },
    },
    SpannedToken {
        kind: Equal,
        span: Span {
            start: 132,
            end: 133,
        },
    },
    SpannedToken {
        kind: Number(
            5,
        ),
        span: Span {
            start: 134,
            end: 135,
        },
    },
    SpannedToken {
        kind: Semicolon,
        span: Span {
            start: 135,
            end: 136,
        },
    },
    SpannedToken {
        kind: RightBrace,
        span: Span {
            start: 141,
            end: 142,
        },
    },
    SpannedToken {
        kind: RightBrace,
        span: Span {
            start: 143,
            end: 144,
        },
    },
    SpannedToken {
        kind: Export,
        span: Span {
            start: 145,
            end: 151,
        },
    },
    SpannedToken {
        kind: Equal,
        span: Span {
            start: 152,
            end: 153,
        },
    },
    SpannedToken {
        kind: Ident(
            "Foo",
        ),
        span: Span {
            start: 154,
            end: 157,
        },
    },
    SpannedToken {
        kind: Semicolon,
        span: Span {
            start: 157,
            end: 158,
        },
    },
]
```

#### ast

- ok: `False`
- truncated: `False`

```text
error: [UnsupportedModule] issue-055: unsupported static export; module resolution and loading are not implemented at 145..151
```

#### resolved

- ok: `False`
- truncated: `False`

```text
error: [UnsupportedModule] issue-055: unsupported static export; module resolution and loading are not implemented at 145..151
```

TypeScript/JavaScript oracle:

```json
{
  "ok": true,
  "returncode": 0,
  "typescript": {
    "ok": true,
    "diagnostics": [],
    "hints": [],
    "typescriptVersion": "6.0.3"
  },
  "ast": {
    "topLevel": [
      {
        "kind": "ClassDeclaration",
        "text": "class Foo {\n    x: number;\n    constructor() {\n        this.x = 5;\n    }\n}",
        "line": 4,
        "character": 1
      },
      {
        "kind": "ExportAssignment",
        "text": "export = Foo;",
        "line": 10,
        "character": 1
      }
    ],
    "pathToPosition": [
      {
        "kind": "SourceFile",
        "text": "class Foo {\n    x: number;\n    constructor() {\n        this.x = 5;\n    }\n}\nexport = Foo;\n",
        "line": 4,
        "character": 1
      },
      {
        "kind": "ExportAssignment",
        "text": "export = Foo;",
        "line": 10,
        "character": 1
      }
    ]
  }
}
```

Stack trace:

```text
error: [UnsupportedModule] issue-055: unsupported static export; module resolution and loading are not implemented at 145..151
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
