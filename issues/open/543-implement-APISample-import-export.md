---
id: 543
title: "Implement Apisample Import Export"
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

Triage APISample-import-export across 7 failing reference test cases and split this bucket into implementation-ready child issues.

## Problem

Reference test results show 7 cases fail in directory `APISample-import-export` with diagnostics: import-export. The compiler cannot handle these syntax/semantics, preventing compilation of code in this category.

Problem: APISample-import-export has 7 reference failures and needs smart-triage evidence before implementation starts.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/APISample_WatchWithOwnWatchHost.ts
```

Coverage window:

```sh
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/APISample_WatchWithOwnWatchHost.ts --detail
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
mise run reference-coverage -- tsc --limit 14
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/APISample_WatchWithOwnWatchHost.ts --detail
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/APISample_WatchWithOwnWatchHost.ts
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

- `reference/typescript/tests/cases/compiler/APISample_WatchWithOwnWatchHost.ts`
- `reference/typescript/tests/cases/compiler/APISample_compile.ts`
- `reference/typescript/tests/cases/compiler/APISample_WatchWithDefaults.ts`
- `reference/typescript/tests/cases/compiler/APISample_linter.ts`
- `reference/typescript/tests/cases/compiler/APISample_transform.ts`
- `reference/typescript/tests/cases/compiler/APISample_Watch.ts`
- `reference/typescript/tests/cases/compiler/APISample_parseConfig.ts`

## Duplicate detection

- `issues/open/070-implement-APISample.md` - Implement Apisample (same reference path, title overlap)
- `issues/open/432-implement-import-export.md` - Implement import/export module syntax (same feature label, title overlap)
- `issues/done/457-implement-APISample-import-export.md` - Implement Apisample Import Export (same reference path, same feature label, same group key, title overlap)
- `issues/done/462-implement-ExportAssignment.md` - Implement Exportassignment (same feature label, same group key, title overlap)
- `issues/done/463-implement-FunctionDeclaration-import-export.md` - Implement Functiondeclaration Import Export (same feature label, same group key, title overlap)
- `issues/done/475-implement-acceptableAlias.md` - Implement Acceptablealias (same feature label, same group key, title overlap)
- `issues/done/481-implement-accessorDeclarationEmitVisibilityErrors.md` - Implement Accessordeclarationemitvisibilityerrors (same feature label, same group key, title overlap)
- `issues/done/483-implement-accessorInAmbientContextES.md` - Implement Accessorinambientcontextes (same feature label, same group key, title overlap)
- `issues/done/484-implement-accessorInferredReturnTypeErrorInReturnStatement.md` - Implement Accessorinferredreturntypeerrorinreturnstatement (same feature label, same group key, title overlap)
- `issues/open/489-implement-accessorsInAmbientContext.md` - Implement Accessorsinambientcontext (same feature label, same group key, title overlap)

## Smart triage

### Smart triage: Triage import export: APISample WatchWithOwnWatchHost

- Issue class: `triage-needed`
- Feature label: `import-export`
- Diagnostic: `UnsupportedModule` / `unsupported-feature-boundary`
- Path: `reference/typescript/tests/cases/compiler/APISample_WatchWithOwnWatchHost.ts`

Reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/APISample_WatchWithOwnWatchHost.ts
```

Source overview:

```json
{
  "suite": "tsc",
  "bytes": 2360,
  "lines": 68,
  "extension": ".ts",
  "first_code_line": "﻿// @target: es2015"
}
```

Failure location:

```json
{
  "code": "UnsupportedModule",
  "message": "issue-055: unsupported default import; module resolution and loading are not implemented at 580..586",
  "span_start": 580,
  "span_end": 586,
  "line": 22,
  "column": 24,
  "feature_label": "import-export",
  "error_type": "unsupported-feature-boundary"
}
```

Source context:

```text
19 | 
20 | declare var console: any;
21 | 
22 | import ts = require("typescript");
23 | 
24 | function watchMain() {
25 |     // get list of files and compiler options somehow
```

Visible symbols before failure:

```json
[
  {
    "kind": "binding",
    "name": "console",
    "line": 20,
    "column": 9
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
    "path": "issues/open/432-implement-import-export.md",
    "title": "Implement import/export module syntax",
    "reason": "same feature label, title overlap"
  },
  {
    "state": "open",
    "path": "issues/done/457-implement-APISample-import-export.md",
    "title": "Implement Apisample Import Export",
    "reason": "same reference path, same feature label, title overlap"
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
            start: 551,
            end: 558,
        },
    },
    SpannedToken {
        kind: Var,
        span: Span {
            start: 559,
            end: 562,
        },
    },
    SpannedToken {
        kind: Ident(
            "console",
        ),
        span: Span {
            start: 563,
            end: 570,
        },
    },
    SpannedToken {
        kind: Colon,
        span: Span {
            start: 570,
            end: 571,
        },
    },
    SpannedToken {
        kind: Ident(
            "any",
        ),
        span: Span {
            start: 572,
            end: 575,
        },
    },
    SpannedToken {
        kind: Semicolon,
        span: Span {
            start: 575,
            end: 576,
        },
    },
    SpannedToken {
        kind: Import,
        span: Span {
            start: 580,
            end: 586,
        },
    },
    SpannedToken {
        kind: Ident(
            "ts",
        ),
        span: Span {
            start: 587,
            end: 589,
        },
    },
    SpannedToken {
        kind: Equal,
        span: Span {
            start: 590,
            end: 591,
        },
    },
    SpannedToken {
        kind: Ident(
            "require",
        ),
        span: Span {
            start: 592,
            end: 599,
        },
    },
    SpannedToken {
        kind: LeftParen,
        span: Span {
            start: 599,
            end: 600,
        },
    },
    SpannedToken {
        kind: String(
            "typescript",
        ),
        span: Span {
            start: 600,
            end: 612,
        },
    },
    SpannedToken {
        kind: RightParen,
        span: Span {
            start: 612,
            end: 613,
        },
    },
    SpannedToken {
        kind: Semicolon,
        span: Span {
            start: 613,
            end: 614,
        },
    },
    SpannedToken {
        kind: Function,
        span: Span {
            start: 618,
            end: 626,
        },
    },
    SpannedToken {
        kind: Ident(
            "watchMain",
        ),
        span: Span {
```

#### ast

- ok: `False`
- truncated: `False`

```text
error: [UnsupportedModule] issue-055: unsupported default import; module resolution and loading are not implemented at 580..586
```

#### resolved

- ok: `False`
- truncated: `False`

```text
error: [UnsupportedModule] issue-055: unsupported default import; module resolution and loading are not implemented at 580..586
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
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/APISample_WatchWithOwnWatchHost.ts",
        "start": 213,
        "length": 1,
        "line": 10,
        "character": 11
      },
      {
        "code": 2695,
        "category": "Error",
        "message": "Left side of comma operator is unused and has no side effects.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/APISample_WatchWithOwnWatchHost.ts",
        "start": 215,
        "length": 12,
        "line": 10,
        "character": 13
      },
      {
        "code": 1005,
        "category": "Error",
        "message": "';' expected.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/APISample_WatchWithOwnWatchHost.ts",
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
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/APISample_WatchWithOwnWatchHost.ts",
        "start": 560,
        "length": 7,
        "line": 20,
        "character": 13,
        "name": "console"
      },
      {
        "kind": "function",
        "typeText": "void",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/APISample_WatchWithOwnWatchHost.ts",
        "start": 624,
        "length": 9,
        "line": 24,
        "character": 10,
        "name": "watchMain"
      },
      {
        "kind": "binding",
        "typeText": "string[]",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/APISample_WatchWithOwnWatchHost.ts",
        "start": 704,
        "length": 5,
        "line": 26,
        "character": 11,
        "name": "files"
      },
      {
        "kind": "binding",
        "typeText": "CompilerOptions",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/APISample_WatchWithOwnWatchHost.ts",
        "start": 737,
        "length": 7,
        "line": 27,
        "character": 11,
        "name": "options"
      },
      {
        "kind": "binding",
        "typeText": "WatchCompilerHostOfFilesAndCompilerOptions<BuilderProgram>",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/APISample_WatchWithOwnWatchHost.ts",
        "start": 784,
        "length": 4,
        "line": 29,
        "character": 11,
        "name": "host"
      },
      {
        "kind": "parameter",
        "typeText": "CompilerOptions",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/APISample_WatchWithOwnWatchHost.ts",
        "start": 1115,
        "length": 7,
        "line": 36,
        "character": 32,
        "name": "options"
      },
      {
        "kind": "binding",
        "typeText": "CreateProgram<BuilderProgram>",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/APISample_WatchWithOwnWatchHost.ts",
        "start": 1787,
        "length": 17,
        "line": 52,
        "character": 11,
        "name": "origCreateProgram"
      },
      {
        "kind": "parameter",
        "typeText": "readonly string[] | undefined",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/APISample_WatchWithOwnWatchHost.ts",
        "start": 1854,
        "length": 9,
        "line": 53,
        "character": 27,
        "name": "rootNames"
      },
      {
        "kind": "parameter",
        "typeText": "CompilerOptions | undefined",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/APISample_WatchWithOwnWatchHost.ts",
        "start": 1865,
        "length": 7,
        "line": 53,
        "character": 38,
        "name": "options"
      },
      {
        "kind": "parameter",
        "typeText": "CompilerHost | undefined",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/APISample_WatchWithOwnWatchHost.ts",
        "start": 1874,
        "length": 4,
        "line": 53,
        "character": 47,
        "name": "host"
      },
      {
        "kind": "parameter",
        "typeText": "BuilderProgram | undefined",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/APISample_WatchWithOwnWatchHost.ts",
        "start": 1880,
        "length": 10,
        "line": 53,
        "character": 53,
        "name": "oldProgram"
      },
      {
        "kind": "binding",
        "typeText": "((program: BuilderProgram) => void) | undefined",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/APISample_WatchWithOwnWatchHost.ts",
        "start": 2054,
        "length": 21,
        "line": 57,
        "character": 11,
        "name": "origPostProgramCreate"
      },
      {
        "kind": "parameter",
        "typeText": "BuilderProgram",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/APISample_WatchWithOwnWatchHost.ts",
        "start": 2136,
        "length": 7,
        "line": 59,
        "character": 31,
        "name": "program"
      }
    ],
    "typescriptVersion": "6.0.3"
  },
  "ast": {
    "topLevel": [
      {
        "kind": "Block",
        "text": "{\r\n    \"name\": \"typescript\",\r\n    \"types\": \"/.ts/typescript.d.ts\"\r\n}",
        "line": 9,
        "character": 1
      },
      {
        "kind": "FirstStatement",
        "text": "declare var console: any;",
        "line": 20,
        "character": 1
      },
      {
        "kind": "ImportEqualsDeclaration",
        "text": "import ts = require(\"typescript\");",
        "line": 22,
        "character": 1
      },
      {
        "kind": "FunctionDeclaration",
        "text": "function watchMain() {\r\n    // get list of files and compiler options somehow\r\n    const files: string[] = [];\r\n    cons",
        "line": 24,
        "character": 1
      },
      {
        "kind": "ExpressionStatement",
        "text": "watchMain();",
        "line": 68,
        "character": 1
      }
    ],
    "pathToPosition": [
      {
        "kind": "SourceFile",
        "text": "{\r\n    \"name\": \"typescript\",\r\n    \"types\": \"/.ts/typescript.d.ts\"\r\n}\r\n\r\n// @filename: APISample_WatchWithOwnWatchHost.ts",
        "line": 9,
        "character": 1
      },
      {
        "kind": "ImportEqualsDeclaration",
        "text": "import ts = require(\"typescript\");",
        "line": 22,
        "character": 1
      }
    ]
  }
}
```

Stack trace:

```text
error: [UnsupportedModule] issue-055: unsupported default import; module resolution and loading are not implemented at 580..586
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
