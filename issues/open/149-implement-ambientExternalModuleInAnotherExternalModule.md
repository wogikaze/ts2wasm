---
id: 149
title: "Implement Ambientexternalmoduleinanotherexternalmodule"
type: spike
area: frontend/syntax
class: triage-needed
priority: P1
depends_on: []
blocks: []
created: 2026-04-29
updated: 2026-04-29
---

## Summary

Triage ambientExternalModuleInAnotherExternalModule across 1 failing reference test cases and split this bucket into implementation-ready child issues.

## Problem

Reference test results show 1 cases fail in directory `ambientExternalModuleInAnotherExternalModule` with diagnostics: ambient-declaration. The compiler cannot handle these syntax/semantics, preventing compilation of code in this category.

Problem: ambientExternalModuleInAnotherExternalModule has 1 reference failures and needs smart-triage evidence before implementation starts.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/ambientExternalModuleInAnotherExternalModule.ts
```

Coverage window:

```sh
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/ambientExternalModuleInAnotherExternalModule.ts --detail
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
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/ambientExternalModuleInAnotherExternalModule.ts --detail
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/ambientExternalModuleInAnotherExternalModule.ts
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

- `reference/typescript/tests/cases/compiler/ambientExternalModuleInAnotherExternalModule.ts`

## Duplicate detection

## Smart triage

### Smart triage: Triage ambient declaration: ambientExternalModuleInAnotherExternalModule

- Issue class: `triage-needed`
- Feature label: `ambient-declaration`
- Diagnostic: `UnsupportedSyntax` / `parser-or-frontend-unsupported`
- Path: `reference/typescript/tests/cases/compiler/ambientExternalModuleInAnotherExternalModule.ts`

Reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/ambientExternalModuleInAnotherExternalModule.ts
```

Source overview:

```json
{
  "suite": "tsc",
  "bytes": 199,
  "lines": 13,
  "extension": ".ts",
  "first_code_line": "class D { }"
}
```

Failure location:

```json
{
  "code": "UnsupportedSyntax",
  "message": "issue-055: unsupported static export; module resolution and loading are not implemented at 56..62",
  "span_start": 56,
  "span_end": 62,
  "line": 5,
  "column": 5,
  "feature_label": "ambient-declaration",
  "error_type": "parser-or-frontend-unsupported"
}
```

Source context:

```text
2 | //@module: commonjs
3 |
4 | class D { }
5 | export = D;
6 |
7 | declare module "ext" {
8 |     export class C { }
```

Visible symbols before failure:

```json
[
  {
    "kind": "class",
    "name": "D",
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
    "path": "issues/open/149-implement-ambientExternalModuleInAnotherExternalModule.md",
    "title": "Implement Ambientexternalmoduleinanotherexternalmodule",
    "reason": "same reference path, title overlap"
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
        kind: Class,
        span: Span {
            start: 43,
            end: 48,
        },
    },
    SpannedToken {
        kind: Ident(
            "D",
        ),
        span: Span {
            start: 49,
            end: 50,
        },
    },
    SpannedToken {
        kind: LeftBrace,
        span: Span {
            start: 51,
            end: 52,
        },
    },
    SpannedToken {
        kind: RightBrace,
        span: Span {
            start: 53,
            end: 54,
        },
    },
    SpannedToken {
        kind: Export,
        span: Span {
            start: 56,
            end: 62,
        },
    },
    SpannedToken {
        kind: Equal,
        span: Span {
            start: 63,
            end: 64,
        },
    },
    SpannedToken {
        kind: Ident(
            "D",
        ),
        span: Span {
            start: 65,
            end: 66,
        },
    },
    SpannedToken {
        kind: Semicolon,
        span: Span {
            start: 66,
            end: 67,
        },
    },
    SpannedToken {
        kind: Ident(
            "declare",
        ),
        span: Span {
            start: 71,
            end: 78,
        },
    },
    SpannedToken {
        kind: Ident(
            "module",
        ),
        span: Span {
            start: 79,
            end: 85,
        },
    },
    SpannedToken {
        kind: String(
            "ext",
        ),
        span: Span {
            start: 86,
            end: 91,
        },
    },
    SpannedToken {
        kind: LeftBrace,
        span: Span {
            start: 92,
            end: 93,
        },
    },
    SpannedToken {
        kind: Export,
        span: Span {
            start: 99,
            end: 105,
        },
    },
    SpannedT
```

#### ast

- ok: `False`
- truncated: `False`

```text
error: [UnsupportedSyntax] issue-055: unsupported static export; module resolution and loading are not implemented at 56..62
```

#### resolved

- ok: `False`
- truncated: `False`

```text
error: [UnsupportedSyntax] issue-055: unsupported static export; module resolution and loading are not implemented at 56..62
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
        "code": 2664,
        "category": "Error",
        "message": "Invalid module name in augmentation, module 'ext' cannot be found.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/ambientExternalModuleInAnotherExternalModule.ts",
        "start": 86,
        "length": 5,
        "line": 7,
        "character": 16
      },
      {
        "code": 2307,
        "category": "Error",
        "message": "Cannot find module 'ext' or its corresponding type declarations.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/ambientExternalModuleInAnotherExternalModule.ts",
        "start": 190,
        "length": 5,
        "line": 12,
        "character": 22
      }
    ],
    "hints": [
      {
        "kind": "binding",
        "typeText": "any",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/ambientExternalModuleInAnotherExternalModule.ts",
        "start": 203,
        "length": 1,
        "line": 13,
        "character": 5,
        "name": "x"
      }
    ],
    "typescriptVersion": "6.0.3"
  },
  "ast": {
    "topLevel": [
      {
        "kind": "ClassDeclaration",
        "text": "class D { }",
        "line": 4,
        "character": 1
      },
      {
        "kind": "ExportAssignment",
        "text": "export = D;",
        "line": 5,
        "character": 1
      },
      {
        "kind": "ModuleDeclaration",
        "text": "declare module \"ext\" {\r\n    export class C { }\r\n}",
        "line": 7,
        "character": 1
      },
      {
        "kind": "ImportEqualsDeclaration",
        "text": "import ext = require(\"ext\");",
        "line": 12,
        "character": 1
      },
      {
        "kind": "FirstStatement",
        "text": "var x = ext;",
        "line": 13,
        "character": 1
      }
    ],
    "pathToPosition": [
      {
        "kind": "SourceFile",
        "text": "class D { }\r\nexport = D;\r\n\r\ndeclare module \"ext\" {\r\n    export class C { }\r\n}\r\n\r\n// Cannot resolve this ext module refer",
        "line": 4,
        "character": 1
      },
      {
        "kind": "ExportAssignment",
        "text": "export = D;",
        "line": 5,
        "character": 1
      }
    ]
  }
}
```

Stack trace:

```text
error: [UnsupportedSyntax] issue-055: unsupported static export; module resolution and loading are not implemented at 56..62
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
