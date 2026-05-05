---
id: 541
title: "Implement Apilibcheck"
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

Triage APILibCheck across 1 failing reference test cases and split this bucket into implementation-ready child issues.

## Problem

Reference test results show 1 cases fail in directory `APILibCheck` with diagnostics: import-export. The compiler cannot handle these syntax/semantics, preventing compilation of code in this category.

Problem: APILibCheck has 1 reference failures and needs smart-triage evidence before implementation starts.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/APILibCheck.ts
```

Coverage window:

```sh
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/APILibCheck.ts --detail
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
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/APILibCheck.ts --detail
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/APILibCheck.ts
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

- `reference/typescript/tests/cases/compiler/APILibCheck.ts`

## Duplicate detection

- `issues/done/069-implement-APILibCheck.md` - Implement Apilibcheck (same reference path, same group key, title overlap)
- `issues/done/455-implement-APILibCheck.md` - Implement Apilibcheck (same reference path, same feature label, same group key, title overlap)

## Smart triage

### Smart triage: Triage import export: APILibCheck

- Issue class: `triage-needed`
- Feature label: `import-export`
- Diagnostic: `UnsupportedModule` / `unsupported-feature-boundary`
- Path: `reference/typescript/tests/cases/compiler/APILibCheck.ts`

Reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/APILibCheck.ts
```

Source overview:

```json
{
  "suite": "tsc",
  "bytes": 979,
  "lines": 38,
  "extension": ".ts",
  "first_code_line": "{"
}
```

Failure location:

```json
{
  "code": "UnsupportedModule",
  "message": "issue-055: unsupported default import; module resolution and loading are not implemented at 769..775",
  "span_start": 769,
  "span_end": 775,
  "line": 35,
  "column": 1,
  "feature_label": "import-export",
  "error_type": "unsupported-feature-boundary"
}
```

Source context:

```text
32 | }
33 | 
34 | // @filename: index.ts
35 | import ts = require("typescript");
36 | import tsInternal = require("typescript-internal");
37 | import tsserverlibrary = require("tsserverlibrary");
38 | import tsserverlibraryInternal = require("tsserverlibrary-internal");
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
    "path": "issues/done/069-implement-APILibCheck.md",
    "title": "Implement Apilibcheck",
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
    "path": "issues/done/455-implement-APILibCheck.md",
    "title": "Implement Apilibcheck",
    "reason": "same reference path, same feature label, title overlap"
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
- truncated: `True`

```text
== tokens ==
[
    SpannedToken {
        kind: LeftBrace,
        span: Span {
            start: 241,
            end: 242,
        },
    },
    SpannedToken {
        kind: String(
            "name",
        ),
        span: Span {
            start: 247,
            end: 253,
        },
    },
    SpannedToken {
        kind: Colon,
        span: Span {
            start: 253,
            end: 254,
        },
    },
    SpannedToken {
        kind: String(
            "typescript",
        ),
        span: Span {
            start: 255,
            end: 267,
        },
    },
    SpannedToken {
        kind: Comma,
        span: Span {
            start: 267,
            end: 268,
        },
    },
    SpannedToken {
        kind: String(
            "types",
        ),
        span: Span {
            start: 273,
            end: 280,
        },
    },
    SpannedToken {
        kind: Colon,
        span: Span {
            start: 280,
            end: 281,
        },
    },
    SpannedToken {
        kind: String(
            "/.ts/typescript.d.ts",
        ),
        span: Span {
            start: 282,
            end: 304,
        },
    },
    SpannedToken {
        kind: RightBrace,
        span: Span {
            start: 305,
            end: 306,
        },
    },
    SpannedToken {
        kind: LeftBrace,
        span: Span {
            start: 368,
            end: 369,
        },
    },
    SpannedToken {
        kind: String(
            "name",
        ),
        span: Span {
            start: 374,
            end: 380,
        },
    },
    SpannedToken {
        kind: Colon,
        span: Span {
            start: 380,
            end: 381,
        },
    },
    SpannedToken {
        kind: String(
            "typescript-internal",
        ),
        span: Span {
            start: 382,
            end: 403,
        },
    },
    SpannedToken {
        kind: Comma,
        span: Span {
            start: 403,
            end: 404,
        },
    },
    SpannedToken {
        kind: String(
            "types",
        ),
        span: Span {
            start: 409,
            end: 416,
        },
    },
    SpannedToken {
        kind: Colon,
        span: Span {
            start: 416,
            end: 417,
        },
    },
    SpannedToken {
        kind: String(
            "/.ts/typescript.internal.d.ts",
        ),
        span: Span {
            start: 418,
            end: 449,
        },
    },
    SpannedToken {
        kind: RightBrace,
        span: Span {
            start: 450,
            end: 451,
        },
    },
    SpannedToken {
        kind: LeftBrace,
        span: Span {
            start: 509,
            end: 510,
        },
    },
    SpannedToken {
        kind: String(
            "name",
        ),
        span: Span {
            start: 515,
            end: 521,
        },
    },
    SpannedToken {
        kind: Colon,
        span: Span {
            start: 521,
            end: 522,
        },
    },
    SpannedToken {
        kind: String(
            "tsserverlibrary",
        ),
        span: Span {
            start: 523,
            end: 540,
        },
    },
    SpannedToken {
        kind: Comma,
        span: Span {
            start: 540,
            end: 541,
        },
    },
    SpannedToken {
        kind: String(
            "types",
        ),
        span: Span {
            start: 546,
            end: 553,
        },
    },
    SpannedToken {
        kind: Co
```

#### ast

- ok: `False`
- truncated: `False`

```text
error: [UnsupportedModule] issue-055: unsupported default import; module resolution and loading are not implemented at 769..775
```

#### resolved

- ok: `False`
- truncated: `False`

```text
error: [UnsupportedModule] issue-055: unsupported default import; module resolution and loading are not implemented at 769..775
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
        "code": 1005,
        "category": "Error",
        "message": "';' expected.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/APILibCheck.ts",
        "start": 253,
        "length": 1,
        "line": 12,
        "character": 11
      },
      {
        "code": 2695,
        "category": "Error",
        "message": "Left side of comma operator is unused and has no side effects.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/APILibCheck.ts",
        "start": 255,
        "length": 12,
        "line": 12,
        "character": 13
      },
      {
        "code": 1005,
        "category": "Error",
        "message": "';' expected.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/APILibCheck.ts",
        "start": 280,
        "length": 1,
        "line": 13,
        "character": 12
      },
      {
        "code": 1005,
        "category": "Error",
        "message": "';' expected.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/APILibCheck.ts",
        "start": 380,
        "length": 1,
        "line": 18,
        "character": 11
      },
      {
        "code": 2695,
        "category": "Error",
        "message": "Left side of comma operator is unused and has no side effects.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/APILibCheck.ts",
        "start": 382,
        "length": 21,
        "line": 18,
        "character": 13
      },
      {
        "code": 1005,
        "category": "Error",
        "message": "';' expected.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/APILibCheck.ts",
        "start": 416,
        "length": 1,
        "line": 19,
        "character": 12
      },
      {
        "code": 1005,
        "category": "Error",
        "message": "';' expected.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/APILibCheck.ts",
        "start": 521,
        "length": 1,
        "line": 24,
        "character": 11
      },
      {
        "code": 2695,
        "category": "Error",
        "message": "Left side of comma operator is unused and has no side effects.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/APILibCheck.ts",
        "start": 523,
        "length": 17,
        "line": 24,
        "character": 13
      },
      {
        "code": 1005,
        "category": "Error",
        "message": "';' expected.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/APILibCheck.ts",
        "start": 553,
        "length": 1,
        "line": 25,
        "character": 12
      },
      {
        "code": 1005,
        "category": "Error",
        "message": "';' expected.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/APILibCheck.ts",
        "start": 663,
        "length": 1,
        "line": 30,
        "character": 11
      },
      {
        "code": 2695,
        "category": "Error",
        "message": "Left side of comma operator is unused and has no side effects.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/APILibCheck.ts",
        "start": 665,
        "length": 26,
        "line": 30,
        "character": 13
      },
      {
        "code": 1005,
        "category": "Error",
        "message": "';' expected.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/APILibCheck.ts",
        "start": 704,
        "length": 1,
        "line": 31,
        "character": 12
      },
      {
        "code": 2307,
        "category": "Error",
        "message": "Cannot find module 'typescript-internal' or its corresponding type declarations.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/APILibCheck.ts",
        "start": 832,
        "length": 21,
        "line": 36,
        "character": 29
      },
      {
        "code": 2307,
        "category": "Error",
        "message": "Cannot find module 'tsserverlibrary' or its corresponding type declarations.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/APILibCheck.ts",
        "start": 889,
        "length": 17,
        "line": 37,
        "character": 34
      },
      {
        "code": 2307,
        "category": "Error",
        "message": "Cannot find module 'tsserverlibrary-internal' or its corresponding type declarations.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/APILibCheck.ts",
        "start": 950,
        "length": 26,
        "line": 38,
        "character": 42
      }
    ],
    "hints": [],
    "typescriptVersion": "6.0.3"
  },
  "ast": {
    "topLevel": [
      {
        "kind": "Block",
        "text": "{\n    \"name\": \"typescript\",\n    \"types\": \"/.ts/typescript.d.ts\"\n}",
        "line": 11,
        "character": 1
      },
      {
        "kind": "Block",
        "text": "{\n    \"name\": \"typescript-internal\",\n    \"types\": \"/.ts/typescript.internal.d.ts\"\n}",
        "line": 17,
        "character": 1
      },
      {
        "kind": "Block",
        "text": "{\n    \"name\": \"tsserverlibrary\",\n    \"types\": \"/.ts/tsserverlibrary.d.ts\"\n}",
        "line": 23,
        "character": 1
      },
      {
        "kind": "Block",
        "text": "{\n    \"name\": \"tsserverlibrary-internal\",\n    \"types\": \"/.ts/tsserverlibrary.internal.d.ts\"\n}",
        "line": 29,
        "character": 1
      },
      {
        "kind": "ImportEqualsDeclaration",
        "text": "import ts = require(\"typescript\");",
        "line": 35,
        "character": 1
      },
      {
        "kind": "ImportEqualsDeclaration",
        "text": "import tsInternal = require(\"typescript-internal\");",
        "line": 36,
        "character": 1
      },
      {
        "kind": "ImportEqualsDeclaration",
        "text": "import tsserverlibrary = require(\"tsserverlibrary\");",
        "line": 37,
        "character": 1
      },
      {
        "kind": "ImportEqualsDeclaration",
        "text": "import tsserverlibraryInternal = require(\"tsserverlibrary-internal\");",
        "line": 38,
        "character": 1
      }
    ],
    "pathToPosition": [
      {
        "kind": "SourceFile",
        "text": "{\n    \"name\": \"typescript\",\n    \"types\": \"/.ts/typescript.d.ts\"\n}\n\n// @filename: node_modules/typescript-internal/packag",
        "line": 11,
        "character": 1
      },
      {
        "kind": "ImportEqualsDeclaration",
        "text": "import ts = require(\"typescript\");",
        "line": 35,
        "character": 1
      }
    ]
  }
}
```

Stack trace:

```text
error: [UnsupportedModule] issue-055: unsupported default import; module resolution and loading are not implemented at 769..775
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
