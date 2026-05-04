---
id: 069
title: "Implement Apilibcheck (dup)"
type: spike
area: runtime/builtins
class: superseded
priority: P1
depends_on: [5004]
blocks: []
created: 2026-04-29
updated: 2026-05-04
---

## Summary

Triage APILibCheck across 1 failing reference test cases and split this bucket into implementation-ready child issues.

## Problem

Reference test results show 1 cases fail in directory `APILibCheck` with diagnostics: parser-syntax. Root cause is a module/import issue (UnsupportedModule): the test uses TypeScript API imports that the module system cannot resolve. This is not a parser issue.

Problem: APILibCheck has 1 reference failure due to UnsupportedModule (import/export).

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
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/APILibCheck.ts --detail
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/APILibCheck.ts
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

### Implementation feasibility: NOT IMPLEMENTABLE (as-is)

The raw test file `APILibCheck.ts` uses `// @filename:` multi-file test harness directives. Our compiler does not support this TypeScript test harness pattern. The first error is `expected Semicolon, got Some(LeftBrace)` on a JSON block following `@filename` — the raw file is not valid standalone TypeScript.

**Options:**
- Close as won't-fix: multi-file harness is a reference testing concern, not a compiler feature gap
- Block on module resolution (5005): needs multi-file harness or manual decomposition

- `reference/typescript/tests/cases/compiler/APILibCheck.ts`

## Duplicate detection

## Smart triage

### Smart triage: Triage parser syntax: APILibCheck

- Issue class: `triage-needed`
- Feature label: `parser-syntax`
- Diagnostic: `UnsupportedSyntax` / `parser-or-frontend-unsupported`
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
  "code": "UnsupportedSyntax",
  "message": "expected Semicolon, got Some(LeftBrace) at 368..369",
  "span_start": 368,
  "span_end": 369,
  "line": 17,
  "column": 1,
  "feature_label": "parser-syntax",
  "error_type": "parser-or-frontend-unsupported"
}
```

Source context:

```text
14 | }
15 |
16 | // @filename: node_modules/typescript-internal/package.json
17 | {
18 |     "name": "typescript-internal",
19 |     "types": "/.ts/typescript.internal.d.ts"
20 | }
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
    "path": "issues/done/059-implement-parser-syntax-extensions.md",
    "title": "Implement parser syntax extensions for TypeScript and advanced JS",
    "reason": "same feature label, title overlap"
  },
  {
    "state": "open",
    "path": "issues/done/069-implement-APILibCheck.md",
    "title": "Implement Apilibcheck",
    "reason": "same reference path, same feature label, title overlap"
  },
  {
    "state": "open",
    "path": "issues/done/200-implement-parser-syntax.md",
    "title": "Implement parser syntax extensions",
    "reason": "same feature label, title overlap"
  },
  {
    "state": "done",
    "path": "issues/done/065-implement-parser-syntax.md",
    "title": "Implement parser syntax extensions",
    "reason": "same feature label, title overlap"
  },
  {
    "state": "done",
    "path": "issues/done/065a-merge-duplicate-parser-syntax-issue-into-059.md",
    "title": "Merge duplicate parser syntax issue into 059",
    "reason": "same feature label, title overlap"
  }
]
```

Error-specific suggestions:

- Start at lexer/parser support and add a minimal fixture for the exact source construct at the failing span.
- Use `dump --tokens` and the TypeScript AST path to decide whether this is tokenization, precedence, or statement dispatch.

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
```

#### ast

- ok: `False`
- truncated: `False`

```text
error: [UnsupportedSyntax] expected Semicolon, got Some(LeftBrace) at 368..369
```

#### resolved

- ok: `False`
- truncated: `False`

```text
error: [UnsupportedSyntax] expected Semicolon, got Some(LeftBrace) at 368..369
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
        "kind": "Block",
        "text": "{\n    \"name\": \"typescript-internal\",\n    \"types\": \"/.ts/typescript.internal.d.ts\"\n}",
        "line": 17,
        "character": 1
      }
    ]
  }
}
```

Stack trace:

```text
error: [UnsupportedSyntax] expected Semicolon, got Some(LeftBrace) at 368..369
```


## 重複整理メモ

この issue はタイトルが重複しているため、内容がより充実している
`issues/open/541-implement-APILibCheck.md` に統合されました。
そちらを参照してください。
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
