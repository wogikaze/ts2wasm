---
id: 495
title: "Implement Aliasinaccessiblemodule"
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

Triage aliasInaccessibleModule across 2 failing reference test cases and split this bucket into implementation-ready child issues.

## Problem

Reference test results show 2 cases fail in directory `aliasInaccessibleModule` with diagnostics: import-export. The compiler cannot handle these syntax/semantics, preventing compilation of code in this category.

Problem: aliasInaccessibleModule has 2 reference failures and needs smart-triage evidence before implementation starts.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/aliasInaccessibleModule.ts
```

Coverage window:

```sh
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/aliasInaccessibleModule.ts --detail
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
mise run reference-coverage -- tsc --limit 4
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/aliasInaccessibleModule.ts --detail
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/aliasInaccessibleModule.ts
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

- `reference/typescript/tests/cases/compiler/aliasInaccessibleModule.ts`
- `reference/typescript/tests/cases/compiler/aliasInaccessibleModule2.ts`

## Duplicate detection

- `issues/done/115-implement-aliasInaccessibleModule.md` - Implement Aliasinaccessiblemodule (same reference path, same group key, title overlap)

## Smart triage

### Smart triage: Triage import export: aliasInaccessibleModule

- Issue class: `triage-needed`
- Feature label: `import-export`
- Diagnostic: `UnsupportedModule` / `unsupported-feature-boundary`
- Path: `reference/typescript/tests/cases/compiler/aliasInaccessibleModule.ts`

Reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/aliasInaccessibleModule.ts
```

Source overview:

```json
{
  "suite": "tsc",
  "bytes": 105,
  "lines": 7,
  "extension": ".ts",
  "first_code_line": "namespace M {"
}
```

Failure location:

```json
{
  "code": "UnsupportedModule",
  "message": "issue-399: TypeScript namespace/internal module declarations require module ownership before runtime lowering at 43..52",
  "span_start": 43,
  "span_end": 52,
  "line": 3,
  "column": 3,
  "feature_label": "import-export",
  "error_type": "unsupported-feature-boundary"
}
```

Source context:

```text
1 | // @target: es2015
2 | // @declaration: true
3 | namespace M {
4 |     namespace N {
5 |     }
6 |     export import X = N;
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
    "path": "issues/done/115-implement-aliasInaccessibleModule.md",
    "title": "Implement Aliasinaccessiblemodule",
    "reason": "same reference path, title overlap"
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
        kind: Ident(
            "namespace",
        ),
        span: Span {
            start: 43,
            end: 52,
        },
    },
    SpannedToken {
        kind: Ident(
            "M",
        ),
        span: Span {
            start: 53,
            end: 54,
        },
    },
    SpannedToken {
        kind: LeftBrace,
        span: Span {
            start: 55,
            end: 56,
        },
    },
    SpannedToken {
        kind: Ident(
            "namespace",
        ),
        span: Span {
            start: 62,
            end: 71,
        },
    },
    SpannedToken {
        kind: Ident(
            "N",
        ),
        span: Span {
            start: 72,
            end: 73,
        },
    },
    SpannedToken {
        kind: LeftBrace,
        span: Span {
            start: 74,
            end: 75,
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
        kind: Export,
        span: Span {
            start: 88,
            end: 94,
        },
    },
    SpannedToken {
        kind: Import,
        span: Span {
            start: 95,
            end: 101,
        },
    },
    SpannedToken {
        kind: Ident(
            "X",
        ),
        span: Span {
            start: 102,
            end: 103,
        },
    },
    SpannedToken {
        kind: Equal,
        span: Span {
            start: 104,
            end: 105,
        },
    },
    SpannedToken {
        kind: Ident(
            "N",
        ),
        span: Span {
            start: 106,
            end: 107,
        },
    },
    SpannedToken {
        kind: Semicolon,
        span: Span {
            start: 107,
            end: 108,
        },
    },
    SpannedToken {
        kind: RightBrace,
        span: Span {
            start: 110,
            end: 111,
        },
    },
]
```

#### ast

- ok: `False`
- truncated: `False`

```text
error: [UnsupportedModule] issue-399: TypeScript namespace/internal module declarations require module ownership before runtime lowering at 43..52
```

#### resolved

- ok: `False`
- truncated: `False`

```text
error: [UnsupportedModule] issue-399: TypeScript namespace/internal module declarations require module ownership before runtime lowering at 43..52
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
        "kind": "ModuleDeclaration",
        "text": "namespace M {\r\n    namespace N {\r\n    }\r\n    export import X = N;\r\n}",
        "line": 3,
        "character": 1
      }
    ],
    "pathToPosition": [
      {
        "kind": "SourceFile",
        "text": "namespace M {\r\n    namespace N {\r\n    }\r\n    export import X = N;\r\n}",
        "line": 3,
        "character": 1
      },
      {
        "kind": "ModuleDeclaration",
        "text": "namespace M {\r\n    namespace N {\r\n    }\r\n    export import X = N;\r\n}",
        "line": 3,
        "character": 1
      }
    ]
  }
}
```

Stack trace:

```text
error: [UnsupportedModule] issue-399: TypeScript namespace/internal module declarations require module ownership before runtime lowering at 43..52
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

## Status

Superseded by issue #115. Duplicate from separate coverage run.
