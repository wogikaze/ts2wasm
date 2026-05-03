---
id: 713
title: "Implement Assertionfunctionwildcardimport"
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

Triage assertionFunctionWildcardImport across 2 failing reference test cases and split this bucket into implementation-ready child issues.

## Problem

Reference test results show 2 cases fail in directory `assertionFunctionWildcardImport` with diagnostics: import-export. The compiler cannot handle these syntax/semantics, preventing compilation of code in this category.

Problem: assertionFunctionWildcardImport has 2 reference failures and needs smart-triage evidence before implementation starts.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/assertionFunctionWildcardImport1.ts
```

Coverage window:

```sh
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/assertionFunctionWildcardImport1.ts --detail
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
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/assertionFunctionWildcardImport1.ts --detail
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/assertionFunctionWildcardImport1.ts
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

- `reference/typescript/tests/cases/compiler/assertionFunctionWildcardImport1.ts`
- `reference/typescript/tests/cases/compiler/assertionFunctionWildcardImport2.ts`

## Duplicate detection

- none found by path/title/feature scan

## Smart triage

### Smart triage: Triage import export: assertionFunctionWildcardImport1

- Issue class: `triage-needed`
- Feature label: `import-export`
- Diagnostic: `UnsupportedSyntax` / `parser-or-frontend-unsupported`
- Path: `reference/typescript/tests/cases/compiler/assertionFunctionWildcardImport1.ts`

Reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/assertionFunctionWildcardImport1.ts
```

Source overview:

```json
{
  "suite": "tsc",
  "bytes": 661,
  "lines": 30,
  "extension": ".ts",
  "first_code_line": "import * as Debug from \"../debug\";"
}
```

Failure location:

```json
{
  "code": "UnsupportedSyntax",
  "message": "expected Semicolon, got Some(Import) at 559..565",
  "span_start": 559,
  "span_end": 565,
  "line": 26,
  "column": 25,
  "feature_label": "import-export",
  "error_type": "parser-or-frontend-unsupported"
}
```

Source context:

```text
23 | 
24 | 
25 | // @filename: src/other/bar.ts
26 | import * as ts from "./_namespaces/ts";
27 | import { Debug } from "./_namespaces/ts";
28 | 
29 | ts.Debug.assert(true);
```

Visible symbols before failure:

```json
[
  {
    "kind": "import",
    "name": "../debug",
    "line": 6,
    "column": 1
  },
  {
    "kind": "function",
    "name": "assert",
    "line": 10,
    "column": 16,
    "params": "expression: unknown"
  },
  {
    "kind": "import",
    "name": "./_namespaces/ts",
    "line": 14,
    "column": 1
  },
  {
    "kind": "import",
    "name": "./_namespaces/ts",
    "line": 15,
    "column": 1
  }
]
```

Duplicate candidates:

```json
[
  {
    "state": "open",
    "path": "issues/open/432-implement-import-export.md",
    "title": "Implement import/export module syntax",
    "reason": "same feature label, title overlap"
  },
  {
    "state": "open",
    "path": "issues/open/457-implement-APISample-import-export.md",
    "title": "Implement Apisample Import Export",
    "reason": "same feature label, title overlap"
  },
  {
    "state": "open",
    "path": "issues/open/463-implement-FunctionDeclaration-import-export.md",
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
    "path": "issues/done/549-implement-FunctionDeclaration-import-export.md",
    "title": "Implement Functiondeclaration Import Export",
    "reason": "same feature label, title overlap"
  },
  {
    "state": "open",
    "path": "issues/open/662-implement-arrayAssignmentTest-import-export.md",
    "title": "Implement Arrayassignmenttest Import Export",
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

- Start at lexer/parser support and add a minimal fixture for the exact source construct at the failing span.
- Use `dump --tokens` and the TypeScript AST path to decide whether this is tokenization, precedence, or statement dispatch.
- Keep module graph behavior separate from parser syntax unless the diagnostic proves syntax is the blocker.

Compiler dumps:

#### tokens

- ok: `True`
- truncated: `True`

```text
== tokens ==
[
    SpannedToken {
        kind: Import,
        span: Span {
            start: 103,
            end: 109,
        },
    },
    SpannedToken {
        kind: Star,
        span: Span {
            start: 110,
            end: 111,
        },
    },
    SpannedToken {
        kind: Ident(
            "as",
        ),
        span: Span {
            start: 112,
            end: 114,
        },
    },
    SpannedToken {
        kind: Ident(
            "Debug",
        ),
        span: Span {
            start: 115,
            end: 120,
        },
    },
    SpannedToken {
        kind: Ident(
            "from",
        ),
        span: Span {
            start: 121,
            end: 125,
        },
    },
    SpannedToken {
        kind: String(
            "../debug",
        ),
        span: Span {
            start: 126,
            end: 136,
        },
    },
    SpannedToken {
        kind: Semicolon,
        span: Span {
            start: 136,
            end: 137,
        },
    },
    SpannedToken {
        kind: Export,
        span: Span {
            start: 139,
            end: 145,
        },
    },
    SpannedToken {
        kind: LeftBrace,
        span: Span {
            start: 146,
            end: 147,
        },
    },
    SpannedToken {
        kind: Ident(
            "Debug",
        ),
        span: Span {
            start: 148,
            end: 153,
        },
    },
    SpannedToken {
        kind: RightBrace,
        span: Span {
            start: 154,
            end: 155,
        },
    },
    SpannedToken {
        kind: Semicolon,
        span: Span {
            start: 155,
            end: 156,
        },
    },
    SpannedToken {
        kind: Export,
        span: Span {
            start: 193,
            end: 199,
        },
    },
    SpannedToken {
        kind: Ident(
            "declare",
        ),
        span: Span {
            start: 200,
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
            "assert",
        ),
        span: Span {
            start: 217,
            end: 223,
        },
    },
    SpannedToken {
        kind: LeftParen,
        span: Span {
            start: 223,
            end: 224,
        },
    },
    SpannedToken {
        kind: Ident(
            "expression",
        ),
        span: Span {
            start: 224,
            end: 234,
        },
    },
    SpannedToken {
        kind: Colon,
        span: Span {
            start: 234,
            end: 235,
        },
    },
    SpannedToken {
        kind: Ident(
            "unknown",
        ),
        span: Span {
            start: 236,
            end: 243,
        },
    },
    SpannedToken {
        kind: RightParen,
        span: Span {
            start: 243,
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
        kind: Ident(
            "asserts",
        ),
        span: Span {
            start: 246,
            end: 253,
        },
    },
    SpannedToken {
        kind: Ident(
            "expression",
        ),
        span: Span {
            start: 254,
            end: 264,
        },
    },
    SpannedToken {
        kind: Semicolon,
        span: Span {
            start: 2
```

#### ast

- ok: `False`
- truncated: `False`

```text
error: [UnsupportedSyntax] expected Semicolon, got Some(Import) at 559..565
```

#### resolved

- ok: `False`
- truncated: `False`

```text
error: [UnsupportedSyntax] expected Semicolon, got Some(Import) at 559..565
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
        "code": 2300,
        "category": "Error",
        "message": "Duplicate identifier 'Debug'.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/assertionFunctionWildcardImport1.ts",
        "start": 115,
        "length": 5,
        "line": 6,
        "character": 13
      },
      {
        "code": 2307,
        "category": "Error",
        "message": "Cannot find module '../debug' or its corresponding type declarations.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/assertionFunctionWildcardImport1.ts",
        "start": 126,
        "length": 10,
        "line": 6,
        "character": 24
      },
      {
        "code": 2300,
        "category": "Error",
        "message": "Duplicate identifier 'ts'.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/assertionFunctionWildcardImport1.ts",
        "start": 314,
        "length": 2,
        "line": 14,
        "character": 13
      },
      {
        "code": 2307,
        "category": "Error",
        "message": "Cannot find module './_namespaces/ts' or its corresponding type declarations.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/assertionFunctionWildcardImport1.ts",
        "start": 322,
        "length": 18,
        "line": 14,
        "character": 21
      },
      {
        "code": 2300,
        "category": "Error",
        "message": "Duplicate identifier 'Debug'.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/assertionFunctionWildcardImport1.ts",
        "start": 352,
        "length": 5,
        "line": 15,
        "character": 10
      },
      {
        "code": 2307,
        "category": "Error",
        "message": "Cannot find module './_namespaces/ts' or its corresponding type declarations.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/assertionFunctionWildcardImport1.ts",
        "start": 365,
        "length": 18,
        "line": 15,
        "character": 23
      },
      {
        "code": 2307,
        "category": "Error",
        "message": "Cannot find module '../../core/_namespaces/ts' or its corresponding type declarations.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/assertionFunctionWildcardImport1.ts",
        "start": 494,
        "length": 27,
        "line": 22,
        "character": 15
      },
      {
        "code": 2300,
        "category": "Error",
        "message": "Duplicate identifier 'ts'.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/assertionFunctionWildcardImport1.ts",
        "start": 571,
        "length": 2,
        "line": 26,
        "character": 13
      },
      {
        "code": 2307,
        "category": "Error",
        "message": "Cannot find module './_namespaces/ts' or its corresponding type declarations.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/assertionFunctionWildcardImport1.ts",
        "start": 579,
        "length": 18,
        "line": 26,
        "character": 21
      },
      {
        "code": 2300,
        "category": "Error",
        "message": "Duplicate identifier 'Debug'.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/assertionFunctionWildcardImport1.ts",
        "start": 609,
        "length": 5,
        "line": 27,
        "character": 10
      },
      {
        "code": 2307,
        "category": "Error",
        "message": "Cannot find module './_namespaces/ts' or its corresponding type declarations.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/assertionFunctionWildcardImport1.ts",
        "start": 622,
        "length": 18,
        "line": 27,
        "character": 23
      }
    ],
    "hints": [
      {
        "kind": "function",
        "typeText": "void",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/assertionFunctionWildcardImport1.ts",
        "start": 217,
        "length": 6,
        "line": 10,
        "character": 25,
        "name": "assert"
      },
      {
        "kind": "parameter",
        "typeText": "unknown",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/assertionFunctionWildcardImport1.ts",
        "start": 224,
        "length": 10,
        "line": 10,
        "character": 32,
        "name": "expression"
      }
    ],
    "typescriptVersion": "6.0.3"
  },
  "ast": {
    "topLevel": [
      {
        "kind": "ImportDeclaration",
        "text": "import * as Debug from \"../debug\";",
        "line": 6,
        "character": 1
      },
      {
        "kind": "ExportDeclaration",
        "text": "export { Debug };",
        "line": 7,
        "character": 1
      },
      {
        "kind": "FunctionDeclaration",
        "text": "export declare function assert(expression: unknown): asserts expression;",
        "line": 10,
        "character": 1
      },
      {
        "kind": "ImportDeclaration",
        "text": "import * as ts from \"./_namespaces/ts\";",
        "line": 14,
        "character": 1
      },
      {
        "kind": "ImportDeclaration",
        "text": "import { Debug } from \"./_namespaces/ts\";",
        "line": 15,
        "character": 1
      },
      {
        "kind": "ExpressionStatement",
        "text": "ts.Debug.assert(true);",
        "line": 17,
        "character": 1
      },
      {
        "kind": "ExpressionStatement",
        "text": "Debug.assert(true);",
        "line": 18,
        "character": 1
      },
      {
        "kind": "ExportDeclaration",
        "text": "export * from \"../../core/_namespaces/ts\"",
        "line": 22,
        "character": 1
      },
      {
        "kind": "ImportDeclaration",
        "text": "import * as ts from \"./_namespaces/ts\";",
        "line": 26,
        "character": 1
      },
      {
        "kind": "ImportDeclaration",
        "text": "import { Debug } from \"./_namespaces/ts\";",
        "line": 27,
        "character": 1
      },
      {
        "kind": "ExpressionStatement",
        "text": "ts.Debug.assert(true);",
        "line": 29,
        "character": 1
      },
      {
        "kind": "ExpressionStatement",
        "text": "Debug.assert(true);",
        "line": 30,
        "character": 1
      }
    ],
    "pathToPosition": [
      {
        "kind": "SourceFile",
        "text": "import * as Debug from \"../debug\";\r\nexport { Debug };\r\n\r\n// @filename: src/core/debug.ts\r\nexport declare function assert",
        "line": 6,
        "character": 1
      },
      {
        "kind": "ImportDeclaration",
        "text": "import * as ts from \"./_namespaces/ts\";",
        "line": 26,
        "character": 1
      }
    ]
  }
}
```

Stack trace:

```text
error: [UnsupportedSyntax] expected Semicolon, got Some(Import) at 559..565
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
