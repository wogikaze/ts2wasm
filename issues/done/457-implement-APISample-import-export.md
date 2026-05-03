---
id: 457
title: "Implement Apisample Import Export (dup)"
type: spike
area: frontend/syntax
class: superseded
priority: P1
depends_on: [5001]
blocks: []
created: 2026-05-01
updated: 2026-05-04
---

## Summary

Triage APISample-import-export across 7 failing reference test cases and split this bucket into implementation-ready child issues.

## Problem

Reference test results show 7 cases fail in directory `APISample-import-export` with diagnostics: import-export. The compiler cannot handle these syntax/semantics, preventing compilation of code in this category.

Problem: APISample-import-export has 7 reference failures and needs smart-triage evidence before implementation starts.

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
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/APISample_Watch.ts --detail
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/APISample_Watch.ts
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

- `reference/typescript/tests/cases/compiler/APISample_Watch.ts`
- `reference/typescript/tests/cases/compiler/APISample_WatchWithDefaults.ts`
- `reference/typescript/tests/cases/compiler/APISample_WatchWithOwnWatchHost.ts`
- `reference/typescript/tests/cases/compiler/APISample_compile.ts`
- `reference/typescript/tests/cases/compiler/APISample_linter.ts`
- `reference/typescript/tests/cases/compiler/APISample_parseConfig.ts`
- `reference/typescript/tests/cases/compiler/APISample_transform.ts`

## Duplicate detection

- `issues/open/070-implement-APISample.md` - Implement Apisample (same reference path, title overlap)
- `issues/open/432-implement-import-export.md` - Implement import/export module syntax (same feature label, title overlap)
- `issues/done/055-implement-import-export.md` - Umbrella: implement import and export (same feature label, title overlap)

## Smart triage

### Smart triage: Triage import export: APISample Watch

- Issue class: `triage-needed`
- Feature label: `import-export`
- Diagnostic: `UnsupportedModule` / `unsupported-feature-boundary`
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
  "code": "UnsupportedModule",
  "message": "issue-055: unsupported default import; module resolution and loading are not implemented at 706..712",
  "span_start": 706,
  "span_end": 712,
  "line": 25,
  "column": 27,
  "feature_label": "import-export",
  "error_type": "unsupported-feature-boundary"
}
```

Source context:

```text
22 | declare var console: any;
23 | declare var os: any;
24 | 
25 | import ts = require("typescript");
26 | 
27 | const formatHost: ts.FormatDiagnosticsHost = {
28 |     getCanonicalFileName: path => path,
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
    "name": "os",
    "line": 23,
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
        span: Span {
            start: 647,
            end: 648,
        },
    },
    SpannedToken {
        kind: Ident(
            "any",
        ),
        span: Span {
            start: 649,
            end: 652,
        },
    },
    SpannedToken {
        kind: Semicolon,
        span: Span {
            start: 652,
            end: 653,
        },
    },
    SpannedToken {
        kind: Ident(
            "declare",
        ),
        span: Span {
            start: 655,
            end: 662,
        },
    },
    SpannedToken {
        kind: Var,
        span: Span {
            start: 663,
            end: 666,
        },
    },
    SpannedToken {
        kind: Ident(
            "console",
        ),
        span: Span {
            start: 667,
            end: 674,
        },
    },
    SpannedToken {
        kind: Colon,
        span: Span {
            start: 674,
            end: 675,
        },
    },
    SpannedToken {
        kind: Ident(
            "any",
        ),
        span: Span {
            start: 676,
            end: 679,
        },
    },
    SpannedToken {
        kind: Semicolon,
        span: Span {
            start: 679,
            end: 680,
        },
    },
    SpannedToken {
        kind: Ident(
            "declare",
        ),
        span: Span {
            start: 682,
            end: 689,
        },
    },
    SpannedToken {
        kind: Var,
        span: Span {
            start: 690,
            end: 693,
        },
    },
    SpannedToken {
        kind: Ident(
            "os",
        ),
        span: Span {
            start: 694,
            end: 696,
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
error: [UnsupportedModule] issue-055: unsupported default import; module resolution and loading are not implemented at 706..712
```

#### resolved

- ok: `False`
- truncated: `False`

```text
error: [UnsupportedModule] issue-055: unsupported default import; module resolution and loading are not implemented at 706..712
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
error: [UnsupportedModule] issue-055: unsupported default import; module resolution and loading are not implemented at 706..712
```


## 重複整理メモ

この issue はタイトルが重複しているため、内容がより充実している
`issues/open/543-implement-APISample-import-export.md` に統合されました。
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
