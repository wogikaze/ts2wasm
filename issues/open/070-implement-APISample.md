---
id: 070
title: "Implement Apisample"
type: spike
area: runtime/builtins
class: superseded
priority: P1
depends_on: []
blocks: []
created: 2026-04-29
updated: 2026-04-29
completed: 2026-05-06
status: done
---

## Summary

Triage APISample across 9 failing reference test cases and split this bucket into implementation-ready child issues.

## Problem

Reference test results show 9 cases fail in directory `APISample` with diagnostics: jsdoc, parser-syntax.

### Triage breakdown

| File | True cause | Category |
|------|-----------|----------|
| APISample_Watch.ts | Module: UnsupportedModule (import/export) | module |
| APISample_WatchWithDefaults.ts | Module: UnsupportedModule (import/export) | module |
| APISample_WatchWithOwnWatchHost.ts | Module: UnsupportedModule (import/export) | module |
| APISample_compile.ts | Module: UnsupportedModule (import/export) | module |
| APISample_parseConfig.ts | Module: UnsupportedModule (import/export) | module |
| APISample_jsdoc.ts | jsdoc / JSDoc annotation | frontend |
| APISample_transform.ts | Parser / transform API | frontend |
| APISample_linter.ts | Parser: `<` token (multi-file test directive) | **parser** |
| APISample_watcher.ts | Runtime: issue-062k (arrow function body) → **split to 5023** | **runtime** |

**Result: 1 of 9 is a true parser issue.** 5 are module issues, 1 runtime (split), 1 jsdoc, 1 transform.

Problem: APISample has 9 reference failures — 1 parser, 5 module, 1 runtime, 2 other.

### Implementation feasibility

| File | Feasibility | Action |
|------|------------|--------|
| APISample_Watch.ts | **Blocked** – module resolution needed | Wait for 5005 |
| APISample_WatchWithDefaults.ts | **Blocked** – module resolution needed | Wait for 5005 |
| APISample_WatchWithOwnWatchHost.ts | **Blocked** – module resolution needed | Wait for 5005 |
| APISample_compile.ts | **Blocked** – module resolution needed | Wait for 5005 |
| APISample_parseConfig.ts | **Blocked** – module resolution needed | Wait for 5005 |
| APISample_jsdoc.ts | **Blocked** – JSDoc not supported | Separate issue |
| APISample_transform.ts | **Blocked** – transform API pattern | Separate issue |
| APISample_linter.ts | **Implementable** – `<` token in multi-file directive | Parser issue (#5000) |
| APISample_watcher.ts | → **split to 5023** (runtime: arrow function) | Issue 5023 |

Of 9 files, only **APISample_linter.ts** is independently implementable today.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/APISample_Watch.ts
```

Coverage window:

```sh
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/APISample_Watch.ts --detail
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
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/APISample_Watch.ts --detail
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/APISample_Watch.ts
```

Not run:

- none

## Docs / current-state / issue sync

Final-state docs:

- [x] not affected

Current state:

- [x] updated: `current-state.md` (repo root)

Follow-up issues:

- [x] existing: `issues/open/543-implement-APISample-import-export.md`
- [x] existing: `issues/open/544-implement-APISample-jsdoc.md`
- [x] existing: `issues/open/542-implement-APISample-arrow-function.md`
- [x] done: `issues/open/5023-implement-api-sample-watcher-arrow.md`
- [x] completed: `issues/done/5139-split-apisample-parser-leftovers.md`

## Notes

## Triage findings

2026-05-06:

- The APISample parent is a generated bucket with mixed causes and must not be selected directly.
- Existing issue 543 owns the import/export APISample bucket and already lists `APISample_Watch*.ts`, `APISample_compile.ts`, `APISample_parseConfig.ts`, `APISample_linter.ts`, and `APISample_transform.ts`.
- Existing issue 544 owns the JSDoc APISample bucket.
- Existing issue 542 and done issue 5023 cover the `APISample_watcher.ts` arrow-function split; issue 5023 records that the original arrow-body diagnostic is gone and a separate parser diagnostic remains.
- Issue 5139 completed the remaining APISample parser-window decision for `APISample_linter.ts` and `APISample_transform.ts`; the shared `node_modules/typescript/package.json` virtual-section blocker is split to issue 5224.

## Affected test files

- `reference/typescript/tests/cases/compiler/APISample_Watch.ts`
- `reference/typescript/tests/cases/compiler/APISample_WatchWithDefaults.ts`
- `reference/typescript/tests/cases/compiler/APISample_WatchWithOwnWatchHost.ts`
- `reference/typescript/tests/cases/compiler/APISample_compile.ts`
- `reference/typescript/tests/cases/compiler/APISample_jsdoc.ts`
- `reference/typescript/tests/cases/compiler/APISample_linter.ts`
- `reference/typescript/tests/cases/compiler/APISample_parseConfig.ts`
- `reference/typescript/tests/cases/compiler/APISample_transform.ts`
- `reference/typescript/tests/cases/compiler/APISample_watcher.ts`

## Duplicate detection

## Smart triage

### Smart triage: Triage parser syntax: APISample Watch

- Issue class: `triage-needed`
- Feature label: `parser-syntax`
- Diagnostic: `UnsupportedSyntax` / `parser-or-frontend-unsupported`
- Path: `reference/typescript/tests/cases/compiler/APISample_Watch.ts`

Reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/APISample_Watch.ts
```

Source overview:

```json
{
  "suite": "tsc",
  "bytes": 3640,
  "lines": 88,
  "extension": ".ts",
  "first_code_line": "﻿// @target: es2015"
}
```

Failure location:

```json
{
  "code": "UnsupportedSyntax",
  "message": "expected Semicolon, got Some(Ident(\"declare\")) at 628..635",
  "span_start": 628,
  "span_end": 635,
  "line": 21,
  "column": 23,
  "feature_label": "parser-syntax",
  "error_type": "parser-or-frontend-unsupported"
}
```

Source context:

```text
18 |  *       Please log a "breaking change" issue for any API breaking change affecting this issue
19 |  */
20 |
21 | declare var process: any;
22 | declare var console: any;
23 | declare var os: any;
24 |
```

Visible symbols before failure:

```json
[
  {
    "kind": "binding",
    "name": "process",
    "line": 21,
    "column": 9
  }
]
```

Duplicate candidates:

```json
[
  {
    "state": "open",
    "path": "issues/open/059-implement-parser-syntax-extensions.md",
    "title": "Implement parser syntax extensions for TypeScript and advanced JS",
    "reason": "same feature label, title overlap"
  },
  {
    "state": "open",
    "path": "issues/done/070-implement-APISample.md",
    "title": "Implement Apisample",
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
            start: 628,
            end: 635,
        },
    },
    SpannedToken {
        kind: Var,
        span: Span {
            start: 636,
            end: 639,
        },
    },
    SpannedToken {
        kind: Ident(
            "process",
        ),
        span: Span {
            start: 640,
            end: 647,
        },
    },
    SpannedToken {
        kind: Colon,
        span: Span
```

#### ast

- ok: `False`
- truncated: `False`

```text
error: [UnsupportedSyntax] expected Semicolon, got Some(Ident("declare")) at 628..635
```

#### resolved

- ok: `False`
- truncated: `False`

```text
error: [UnsupportedSyntax] expected Semicolon, got Some(Ident("declare")) at 628..635
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
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/APISample_Watch.ts",
        "start": 213,
        "length": 1,
        "line": 10,
        "character": 11
      },
      {
        "code": 2695,
        "category": "Error",
        "message": "Left side of comma operator is unused and has no side effects.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/APISample_Watch.ts",
        "start": 215,
        "length": 12,
        "line": 10,
        "character": 13
      },
      {
        "code": 1005,
        "category": "Error",
        "message": "';' expected.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/APISample_Watch.ts",
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
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/APISample_Watch.ts",
        "start": 637,
        "length": 7,
        "line": 21,
        "character": 13,
        "name": "process"
      },
      {
        "kind": "binding",
        "typeText": "any",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/APISample_Watch.ts",
        "start": 664,
        "length": 7,
        "line": 22,
        "character": 13,
        "name": "console"
      },
      {
        "kind": "binding",
        "typeText": "any",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/APISample_Watch.ts",
        "start": 691,
        "length": 2,
        "line": 23,
        "character": 13,
        "name": "os"
      },
      {
        "kind": "binding",
        "typeText": "FormatDiagnosticsHost",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/APISample_Watch.ts",
        "start": 747,
        "length": 10,
        "line": 27,
        "character": 7,
        "name": "formatHost"
      },
      {
        "kind": "parameter",
        "typeText": "string",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/APISample_Watch.ts",
        "start": 815,
        "length": 4,
        "line": 28,
        "character": 27,
        "name": "path"
      },
      {
        "kind": "function",
        "typeText": "void",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/APISample_Watch.ts",
        "start": 937,
        "length": 9,
        "line": 33,
        "character": 10,
        "name": "watchMain"
      },
      {
        "kind": "binding",
        "typeText": "string | undefined",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/APISample_Watch.ts",
        "start": 962,
        "length": 10,
        "line": 34,
        "character": 11,
        "name": "configPath"
      },
      {
        "kind": "binding",
        "typeText": "WatchCompilerHostOfConfigFile<SemanticDiagnosticsBuilderProgram>",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/APISample_Watch.ts",
        "start": 2149,
        "length": 4,
        "line": 50,
        "character": 11,
        "name": "host"
      },
      {
        "kind": "binding",
        "typeText": "CreateProgram<SemanticDiagnosticsBuilderProgram>",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/APISample_Watch.ts",
        "start": 2551,
        "length": 17,
        "line": 58,
        "character": 11,
        "name": "origCreateProgram"
      },
      {
        "kind": "parameter",
        "typeText": "readonly string[] | undefined",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/APISample_Watch.ts",
        "start": 2618,
        "length": 9,
        "line": 59,
        "character": 27,
        "name": "rootNames"
      },
      {
        "kind": "parameter",
        "typeText": "CompilerOptions | undefined",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/APISample_Watch.ts",
        "start": 2664,
        "length": 7,
        "line": 59,
        "character": 73,
        "name": "options"
      },
      {
        "kind": "parameter",
        "typeText": "CompilerHost | undefined",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/APISample_Watch.ts",
        "start": 2673,
        "length": 4,
        "line": 59,
        "character": 82,
        "name": "host"
      },
      {
        "kind": "parameter",
        "typeText": "SemanticDiagnosticsBuilderProgram | undefined",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/APISample_Watch.ts",
        "start": 2679,
        "length": 10,
        "line": 59,
        "character": 88,
        "name": "oldProgram"
      },
      {
        "kind": "binding",
        "typeText": "((program: SemanticDiagnosticsBuilderProgram) => void) | undefined",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/APISample_Watch.ts",
        "start": 2853,
        "length": 21,
        "line": 63,
        "character": 11,
        "name": "origPostProgramCreate"
      },
      {
        "kind": "parameter",
        "typeText": "SemanticDiagnosticsBuilderProgram",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/APISample_Watch.ts",
        "start": 2935,
        "length": 7,
        "line": 65,
        "character": 31,
        "name": "program"
      },
      {
        "kind": "function",
        "typeText": "void",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/APISample_Watch.ts",
        "start": 3219,
        "length": 16,
        "line": 74,
        "character": 10,
        "name": "reportDiagnostic"
      },
      {
        "kind": "parameter",
        "typeText": "Diagnostic",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/APISample_Watch.ts",
        "start": 3236,
        "length": 10,
        "line": 74,
        "character": 27,
        "name": "diagnostic"
      },
      {
        "kind": "function",
        "typeText": "void",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/APISample_Watch.ts",
        "start": 3587,
        "length": 24,
        "line": 84,
        "character": 10,
        "name": "reportWatchStatusChanged"
      },
      {
        "kind": "parameter",
        "typeText": "Diagnostic",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/APISample_Watch.ts",
        "start": 3612,
        "length": 10,
        "line": 84,
        "character": 35,
        "name": "diagnostic"
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
        "text": "declare var process: any;",
        "line": 21,
        "character": 1
      },
      {
        "kind": "FirstStatement",
        "text": "declare var console: any;",
        "line": 22,
        "character": 1
      },
      {
        "kind": "FirstStatement",
        "text": "declare var os: any;",
        "line": 23,
        "character": 1
      },
      {
        "kind": "ImportEqualsDeclaration",
        "text": "import ts = require(\"typescript\");",
        "line": 25,
        "character": 1
      },
      {
        "kind": "FirstStatement",
        "text": "const formatHost: ts.Forma
```

Stack trace:

```text
error: [UnsupportedSyntax] expected Semicolon, got Some(Ident("declare")) at 628..635
```

## Completion evidence

Fill only when moving to `done/`.

Commits:

- issue-state commit closing this generated bucket after child issue 5139 split

Validation result:

```text
command: python scripts/manager.py update-issue-index --check
result: pass
date: 2026-05-06

command: python scripts/manager.py check-issue-health
result: pass
date: 2026-05-06

command: python scripts/manager.py check-issue-readiness -- --fail-ready-below 80
result: pass
date: 2026-05-06
```

Remaining risks:

- APISample implementation remains open in child issues 542, 543, 544, and 5139.

## False-done audit

Date: 2026-05-06

Classification: truly-done.

Audit result: retained in `issues/done/`. This issue has repo-local close evidence
(completion evidence with validation commands) or proper superseded classification
with child issues in `issues/open/`. The acceptance criteria documented in the issue
are satisfied by the repo-local evidence cited in the completion evidence section.

Future-work tracking: no untracked future-work item was identified in this issue
during this metadata/evidence audit.

