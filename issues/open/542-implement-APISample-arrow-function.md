---
id: 542
title: "Implement Apisample Arrow Function"
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

Triage APISample-arrow-function across 1 failing reference test cases and split this bucket into implementation-ready child issues.

## Problem

Reference test results show 1 cases fail in directory `APISample-arrow-function` with diagnostics: arrow-function. The compiler cannot handle these syntax/semantics, preventing compilation of code in this category.

Problem: APISample-arrow-function has 1 reference failures and needs smart-triage evidence before implementation starts.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/APISample_watcher.ts
```

Coverage window:

```sh
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/APISample_watcher.ts --detail
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
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/APISample_watcher.ts --detail
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/APISample_watcher.ts
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

- `reference/typescript/tests/cases/compiler/APISample_watcher.ts`

## Duplicate detection

- `issues/open/070-implement-APISample.md` - Implement Apisample (same reference path, title overlap)
- `issues/done/415-implement-arrow-function.md` - Implement arrow functions (same feature label, title overlap)
- `issues/done/430-implement-function.md` - Implement function support (same feature label, title overlap)
- `issues/done/456-implement-APISample-arrow-function.md` - Implement Apisample Arrow Function (same reference path, same feature label, same group key, title overlap)
- `issues/done/036-implement-arrow-function.md` - Implement arrow function (same feature label, title overlap)
- `issues/done/062d-function-this-and-arguments.md` - Implement function this and arguments semantics (same feature label, title overlap)
- `issues/done/210-implement-arrow-function-closure-lexical-this.md` - Implement arrow function closure and lexical this semantics (same feature label, title overlap)

## Smart triage

### Smart triage: Triage arrow function: APISample watcher

- Issue class: `triage-needed`
- Feature label: `arrow-function`
- Diagnostic: `UnsupportedSyntax` / `parser-or-frontend-unsupported`
- Path: `reference/typescript/tests/cases/compiler/APISample_watcher.ts`

Reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/APISample_watcher.ts
```

Source overview:

```json
{
  "suite": "tsc",
  "bytes": 4542,
  "lines": 124,
  "extension": ".ts",
  "first_code_line": "﻿// @target: es2015"
}
```

Failure location:

```json
{
  "code": "UnsupportedSyntax",
  "message": "arrow function block bodies support a single return statement in this milestone at 1398..1406",
  "span_start": 1398,
  "span_end": 1406,
  "line": 39,
  "column": 26,
  "feature_label": "arrow-function",
  "error_type": "parser-or-frontend-unsupported"
}
```

Source context:

```text
36 | 
37 |     // initialize the list of files
38 |     rootFileNames.forEach(fileName => {
39 |         files[fileName] = { version: 0 };
40 |     });
41 | 
42 |     // Create the language service host to allow the LS to communicate with the host
```

Visible symbols before failure:

```json
[
  {
    "kind": "binding",
    "name": "process",
    "line": 21,
    "column": 9
  },
  {
    "kind": "binding",
    "name": "console",
    "line": 22,
    "column": 9
  },
  {
    "kind": "binding",
    "name": "fs",
    "line": 23,
    "column": 9
  },
  {
    "kind": "binding",
    "name": "path",
    "line": 30,
    "column": 9
  },
  {
    "kind": "import",
    "name": "typescript",
    "line": 32,
    "column": 1
  },
  {
    "kind": "function",
    "name": "watch",
    "line": 34,
    "column": 1,
    "params": "rootFileNames: string[], options: ts.CompilerOptions"
  },
  {
    "kind": "binding",
    "name": "files",
    "line": 35,
    "column": 5
  }
]
```

Duplicate candidates:

```json
[
  {
    "state": "open",
    "path": "issues/open/070-implement-APISample.md",
    "title": "Implement Apisample",
    "reason": "same reference path, title overlap"
  },
  {
    "state": "open",
    "path": "issues/done/415-implement-arrow-function.md",
    "title": "Implement arrow functions",
    "reason": "same feature label, title overlap"
  },
  {
    "state": "open",
    "path": "issues/done/430-implement-function.md",
    "title": "Implement function support",
    "reason": "same feature label, title overlap"
  },
  {
    "state": "open",
    "path": "issues/done/456-implement-APISample-arrow-function.md",
    "title": "Implement Apisample Arrow Function",
    "reason": "same reference path, same feature label, title overlap"
  },
  {
    "state": "done",
    "path": "issues/done/036-implement-arrow-function.md",
    "title": "Implement arrow function",
    "reason": "same feature label, title overlap"
  },
  {
    "state": "done",
    "path": "issues/done/062d-function-this-and-arguments.md",
    "title": "Implement function this and arguments semantics",
    "reason": "same feature label, title overlap"
  },
  {
    "state": "done",
    "path": "issues/done/210-implement-arrow-function-closure-lexical-this.md",
    "title": "Implement arrow function closure and lexical this semantics",
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
            start: 203,
            end: 204,
        },
    },
    SpannedToken {
        kind: String(
            "name",
        ),
        span: Span {
            start: 210,
            end: 216,
        },
    },
    SpannedToken {
        kind: Colon,
        span: Span {
            start: 216,
            end: 217,
        },
    },
    SpannedToken {
        kind: String(
            "typescript",
        ),
        span: Span {
            start: 218,
            end: 230,
        },
    },
    SpannedToken {
        kind: Comma,
        span: Span {
            start: 230,
            end: 231,
        },
    },
    SpannedToken {
        kind: String(
            "types",
        ),
        span: Span {
            start: 237,
            end: 244,
        },
    },
    SpannedToken {
        kind: Colon,
        span: Span {
            start: 244,
            end: 245,
        },
    },
    SpannedToken {
        kind: String(
            "/.ts/typescript.d.ts",
        ),
        span: Span {
            start: 246,
            end: 268,
        },
    },
    SpannedToken {
        kind: RightBrace,
        span: Span {
            start: 270,
            end: 271,
        },
    },
    SpannedToken {
        kind: Ident(
            "declare",
        ),
        span: Span {
            start: 630,
            end: 637,
        },
    },
    SpannedToken {
        kind: Var,
        span: Span {
            start: 638,
            end: 641,
        },
    },
    SpannedToken {
        kind: Ident(
            "process",
        ),
        span: Span {
            start: 642,
            end: 649,
        },
    },
    SpannedToken {
        kind: Colon,
        span: Span {
            start: 649,
            end: 650,
        },
    },
    SpannedToken {
        kind: Ident(
            "any",
        ),
        span: Span {
            start: 651,
            end: 654,
        },
    },
    SpannedToken {
        kind: Semicolon,
        span: Span {
            start: 654,
            end: 655,
        },
    },
    SpannedToken {
        kind: Ident(
            "declare",
        ),
        span: Span {
            start: 657,
            end: 664,
        },
    },
    SpannedToken {
        kind: Var,
        span: Span {
            start: 665,
            end: 668,
        },
    },
    SpannedToken {
        kind: Ident(
            "console",
        ),
        span: Span {
            start: 669,
            end: 676,
        },
    },
    SpannedToken {
        kind: Colon,
        span: Span {
            start: 676,
            end: 677,
        },
    },
    SpannedToken {
        kind: Ident(
            "any",
        ),
        span: Span {
            start: 678,
            end: 681,
        },
    },
    SpannedToken {
        kind: Semicolon,
        span: Span {
            start: 681,
            end: 682,
        },
    },
    SpannedToken {
        kind: Ident(
            "declare",
        ),
        span: Span {
            start: 684,
            end: 691,
        },
    },
    SpannedToken {
        kind: Var,
        span: Span {
            start: 692,
            end: 695,
        },
    },
    SpannedToken {
        kind: Ident(
            "fs",
        ),
        span: Span {
            start: 696,
            end: 698,
        },
    },
    SpannedToken {
        kind: Colon,
        span: Span {
```

#### ast

- ok: `False`
- truncated: `False`

```text
error: [UnsupportedSyntax] arrow function block bodies support a single return statement in this milestone at 1398..1406
```

#### resolved

- ok: `False`
- truncated: `False`

```text
error: [UnsupportedSyntax] arrow function block bodies support a single return statement in this milestone at 1398..1406
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
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/APISample_watcher.ts",
        "start": 213,
        "length": 1,
        "line": 10,
        "character": 11
      },
      {
        "code": 2695,
        "category": "Error",
        "message": "Left side of comma operator is unused and has no side effects.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/APISample_watcher.ts",
        "start": 215,
        "length": 12,
        "line": 10,
        "character": 13
      },
      {
        "code": 1005,
        "category": "Error",
        "message": "';' expected.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/APISample_watcher.ts",
        "start": 241,
        "length": 1,
        "line": 11,
        "character": 12
      }
    ],
    "hints": [
      {
        "kind": "binding",
        "typeText": "any",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/APISample_watcher.ts",
        "start": 639,
        "length": 7,
        "line": 21,
        "character": 13,
        "name": "process"
      },
      {
        "kind": "binding",
        "typeText": "any",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/APISample_watcher.ts",
        "start": 666,
        "length": 7,
        "line": 22,
        "character": 13,
        "name": "console"
      },
      {
        "kind": "binding",
        "typeText": "{ existsSync(path: string): boolean; readdirSync(path: string): string[]; readFileSync(filename: string, encoding?: string | undefined): string; writeFileSync(filename: string, data: any, options?: string | ... 1 more ... | undefined): void; watchFile(filename: string, options: { ...; }, listener: (curr: { ...; }, p...",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/APISample_watcher.ts",
        "start": 693,
        "length": 2,
        "line": 23,
        "character": 13,
        "name": "fs"
      },
      {
        "kind": "parameter",
        "typeText": "string",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/APISample_watcher.ts",
        "start": 715,
        "length": 4,
        "line": 24,
        "character": 16,
        "name": "path"
      },
      {
        "kind": "parameter",
        "typeText": "string",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/APISample_watcher.ts",
        "start": 756,
        "length": 4,
        "line": 25,
        "character": 17,
        "name": "path"
      },
      {
        "kind": "parameter",
        "typeText": "string",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/APISample_watcher.ts",
        "start": 799,
        "length": 8,
        "line": 26,
        "character": 18,
        "name": "filename"
      },
      {
        "kind": "parameter",
        "typeText": "string | undefined",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/APISample_watcher.ts",
        "start": 817,
        "length": 8,
        "line": 26,
        "character": 36,
        "name": "encoding"
      },
      {
        "kind": "parameter",
        "typeText": "string",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/APISample_watcher.ts",
        "start": 864,
        "length": 8,
        "line": 27,
        "character": 19,
        "name": "filename"
      },
      {
        "kind": "parameter",
        "typeText": "any",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/APISample_watcher.ts",
        "start": 882,
        "length": 4,
        "line": 27,
        "character": 37,
        "name": "data"
      },
      {
        "kind": "parameter",
        "typeText": "string | { encoding?: string | undefined; mode?: number | undefined; flag?: string | undefined; } | undefined",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/APISample_watcher.ts",
        "start": 893,
        "length": 7,
        "line": 27,
        "character": 48,
        "name": "options"
      },
      {
        "kind": "parameter",
        "typeText": "string",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/APISample_watcher.ts",
        "start": 988,
        "length": 8,
        "line": 28,
        "character": 15,
        "name": "filename"
      },
      {
        "kind": "parameter",
        "typeText": "{ persistent?: boolean | undefined; interval?: number | undefined; }",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/APISample_watcher.ts",
        "start": 1006,
        "length": 7,
        "line": 28,
        "character": 33,
        "name": "options"
      },
      {
        "kind": "parameter",
        "typeText": "(curr: { mtime: Date; }, prev: { mtime: Date; }) => void",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/APISample_watcher.ts",
        "start": 1061,
        "length": 8,
        "line": 28,
        "character": 88,
        "name": "listener"
      },
      {
        "kind": "parameter",
        "typeText": "{ mtime: Date; }",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/APISample_watcher.ts",
        "start": 1072,
        "length": 4,
        "line": 28,
        "character": 99,
        "name": "curr"
      },
      {
        "kind": "parameter",
        "typeText": "{ mtime: Date; }",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/APISample_watcher.ts",
        "start": 1095,
        "length": 4,
        "line": 28,
        "character": 122,
        "name": "prev"
      },
      {
        "kind": "binding",
        "typeText": "any",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/APISample_watcher.ts",
        "start": 1151,
        "length": 4,
        "line": 30,
        "character": 13,
        "name": "path"
      },
      {
        "kind": "function",
        "typeText": "void",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/APISample_watcher.ts",
        "start": 1211,
        "length": 5,
        "line": 34,
        "character": 10,
        "name": "watch"
      },
      {
        "kind": "parameter",
        "typeText": "string[]",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/APISample_watcher.ts",
        "start": 1217,
        "length": 13,
        "line": 34,
        "character": 16,
        "name": "rootFileNames"
      },
      {
        "kind": "parameter",
        "typeText": "CompilerOptions",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/APISample_watcher.ts",
        "start": 1242,
        "length": 7,
        "line": 34,
        "character": 41,
        "name": "options"
      },
      {
        "kind": "binding",
        "typeText": "MapLike<{ version: number; }>",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/APISample_watcher.ts",
        "start": 1284,
        "length": 5,
        "line": 35,
        "character": 11,
        "name": "files"
      },
      {
        "kind": "parameter",
        "typeText": "string",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/APISample_watcher.ts",
        "start": 1395,
        "length": 8,
        "line": 38,
        "character": 27,
        "name": "fileName"
      },
      {
        "kind": "binding",
        "typeText": "LanguageServ
```

Stack trace:

```text
error: [UnsupportedSyntax] arrow function block bodies support a single return statement in this milestone at 1398..1406
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
