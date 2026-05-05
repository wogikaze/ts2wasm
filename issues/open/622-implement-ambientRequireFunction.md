---
id: 622
title: "Implement Ambientrequirefunction"
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

Triage ambientRequireFunction across 1 failing reference test cases and split this bucket into implementation-ready child issues.

## Problem

Reference test results show 1 cases fail in directory `ambientRequireFunction` with diagnostics: import-export. The compiler cannot handle these syntax/semantics, preventing compilation of code in this category.

Problem: ambientRequireFunction has 1 reference failures and needs smart-triage evidence before implementation starts.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/ambientRequireFunction.ts
```

Coverage window:

```sh
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/ambientRequireFunction.ts --detail
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
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/ambientRequireFunction.ts --detail
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/ambientRequireFunction.ts
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

- `reference/typescript/tests/cases/compiler/ambientRequireFunction.ts`

## Duplicate detection

- `issues/done/163-implement-ambientRequireFunction.md` - Implement Ambientrequirefunction (same reference path, same group key, title overlap)
- `issues/done/536-implement-ambientRequireFunction.md` - Implement Ambientrequirefunction (same reference path, same feature label, same group key, title overlap)

## Smart triage

### Smart triage: Triage import export: ambientRequireFunction

- Issue class: `triage-needed`
- Feature label: `import-export`
- Diagnostic: `UnsupportedModule` / `unsupported-feature-boundary`
- Path: `reference/typescript/tests/cases/compiler/ambientRequireFunction.ts`

Reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/ambientRequireFunction.ts
```

Source overview:

```json
{
  "suite": "tsc",
  "bytes": 364,
  "lines": 18,
  "extension": ".ts",
  "first_code_line": "declare function require(moduleName: string): any;"
}
```

Failure location:

```json
{
  "code": "UnsupportedModule",
  "message": "issue-400: ambient module declarations require module ownership before runtime lowering at 182..188",
  "span_start": 182,
  "span_end": 188,
  "line": 10,
  "column": 18,
  "feature_label": "import-export",
  "error_type": "unsupported-feature-boundary"
}
```

Source context:

```text
 7 | 
 8 | declare function require(moduleName: string): any;
 9 | 
10 | declare module "fs" {
11 |     export function readFileSync(s: string): string;
12 | }
13 |
```

Visible symbols before failure:

```json
[
  {
    "kind": "function",
    "name": "require",
    "line": 8,
    "column": 9,
    "params": "moduleName: string"
  }
]
```

Duplicate candidates:

```json
[
  {
    "state": "open",
    "path": "issues/done/163-implement-ambientRequireFunction.md",
    "title": "Implement Ambientrequirefunction",
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
    "state": "open",
    "path": "issues/done/536-implement-ambientRequireFunction.md",
    "title": "Implement Ambientrequirefunction",
    "reason": "same reference path, same feature label, title overlap"
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
- truncated: `True`

```text
== tokens ==
[
    SpannedToken {
        kind: Ident(
            "declare",
        ),
        span: Span {
            start: 120,
            end: 127,
        },
    },
    SpannedToken {
        kind: Function,
        span: Span {
            start: 128,
            end: 136,
        },
    },
    SpannedToken {
        kind: Ident(
            "require",
        ),
        span: Span {
            start: 137,
            end: 144,
        },
    },
    SpannedToken {
        kind: LeftParen,
        span: Span {
            start: 144,
            end: 145,
        },
    },
    SpannedToken {
        kind: Ident(
            "moduleName",
        ),
        span: Span {
            start: 145,
            end: 155,
        },
    },
    SpannedToken {
        kind: Colon,
        span: Span {
            start: 155,
            end: 156,
        },
    },
    SpannedToken {
        kind: Ident(
            "string",
        ),
        span: Span {
            start: 157,
            end: 163,
        },
    },
    SpannedToken {
        kind: RightParen,
        span: Span {
            start: 163,
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
            "any",
        ),
        span: Span {
            start: 166,
            end: 169,
        },
    },
    SpannedToken {
        kind: Semicolon,
        span: Span {
            start: 169,
            end: 170,
        },
    },
    SpannedToken {
        kind: Ident(
            "declare",
        ),
        span: Span {
            start: 174,
            end: 181,
        },
    },
    SpannedToken {
        kind: Ident(
            "module",
        ),
        span: Span {
            start: 182,
            end: 188,
        },
    },
    SpannedToken {
        kind: String(
            "fs",
        ),
        span: Span {
            start: 189,
            end: 193,
        },
    },
    SpannedToken {
        kind: LeftBrace,
        span: Span {
            start: 194,
            end: 195,
        },
    },
    SpannedToken {
        kind: Export,
        span: Span {
            start: 201,
            end: 207,
        },
    },
    SpannedToken {
        kind: Function,
        span: Span {
            start: 208,
            end: 216,
        },
    },
    SpannedToken {
        kind: Ident(
            "readFileSync",
        ),
        span: Span {
            start: 217,
            end: 229,
        },
    },
    SpannedToken {
        kind: LeftParen,
        span: Span {
            start: 229,
            end: 230,
        },
    },
    SpannedToken {
        kind: Ident(
            "s",
        ),
        span: Span {
            start: 230,
            end: 231,
        },
    },
    SpannedToken {
        kind: Colon,
        span: Span {
            start: 231,
            end: 232,
        },
    },
    SpannedToken {
        kind: Ident(
            "string",
        ),
        span: Span {
            start: 233,
            end: 239,
        },
    },
    SpannedToken {
        kind: RightParen,
        span: Span {
            start: 239,
            end: 240,
        },
    },
    SpannedToken {
        kind: Colon,
        span: Span {
            start: 240,
            end: 241,
        },
    },
    SpannedToken {
        kind: Ident(
            "string",
        ),
        span:
```

#### ast

- ok: `False`
- truncated: `False`

```text
error: [UnsupportedModule] issue-400: ambient module declarations require module ownership before runtime lowering at 182..188
```

#### resolved

- ok: `False`
- truncated: `False`

```text
error: [UnsupportedModule] issue-400: ambient module declarations require module ownership before runtime lowering at 182..188
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
        "kind": "function",
        "typeText": "any",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/ambientRequireFunction.ts",
        "start": 137,
        "length": 7,
        "line": 8,
        "character": 18,
        "name": "require"
      },
      {
        "kind": "parameter",
        "typeText": "string",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/ambientRequireFunction.ts",
        "start": 145,
        "length": 10,
        "line": 8,
        "character": 26,
        "name": "moduleName"
      },
      {
        "kind": "function",
        "typeText": "string",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/ambientRequireFunction.ts",
        "start": 217,
        "length": 12,
        "line": 11,
        "character": 21,
        "name": "readFileSync"
      },
      {
        "kind": "parameter",
        "typeText": "string",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/ambientRequireFunction.ts",
        "start": 230,
        "length": 1,
        "line": 11,
        "character": 34,
        "name": "s"
      },
      {
        "kind": "binding",
        "typeText": "any",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/ambientRequireFunction.ts",
        "start": 321,
        "length": 2,
        "line": 17,
        "character": 7,
        "name": "fs"
      },
      {
        "kind": "binding",
        "typeText": "any",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/ambientRequireFunction.ts",
        "start": 348,
        "length": 4,
        "line": 18,
        "character": 7,
        "name": "text"
      }
    ],
    "typescriptVersion": "6.0.3"
  },
  "ast": {
    "topLevel": [
      {
        "kind": "FunctionDeclaration",
        "text": "declare function require(moduleName: string): any;",
        "line": 8,
        "character": 1
      },
      {
        "kind": "ModuleDeclaration",
        "text": "declare module \"fs\" {\r\n    export function readFileSync(s: string): string;\r\n}",
        "line": 10,
        "character": 1
      },
      {
        "kind": "FirstStatement",
        "text": "const fs = require(\"fs\");",
        "line": 17,
        "character": 1
      },
      {
        "kind": "FirstStatement",
        "text": "const text = fs.readFileSync(\"/a/b/c\");",
        "line": 18,
        "character": 1
      }
    ],
    "pathToPosition": [
      {
        "kind": "SourceFile",
        "text": "declare function require(moduleName: string): any;\r\n\r\ndeclare module \"fs\" {\r\n    export function readFileSync(s: string)",
        "line": 8,
        "character": 1
      },
      {
        "kind": "ModuleDeclaration",
        "text": "declare module \"fs\" {\r\n    export function readFileSync(s: string): string;\r\n}",
        "line": 10,
        "character": 1
      }
    ]
  }
}
```

Stack trace:

```text
error: [UnsupportedModule] issue-400: ambient module declarations require module ownership before runtime lowering at 182..188
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
