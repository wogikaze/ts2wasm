---
id: 510
title: "Implement Allowimportclausestomergewithtypes"
type: spike
area: frontend/syntax
class: triage-needed
priority: P1
depends_on: []
blocks: []
created: 2026-05-01
updated: 2026-05-01
---

## Summary

Triage allowImportClausesToMergeWithTypes across 1 failing reference test cases and split this bucket into implementation-ready child issues.

## Problem

Reference test results show 1 cases fail in directory `allowImportClausesToMergeWithTypes` with diagnostics: import-export. The compiler cannot handle these syntax/semantics, preventing compilation of code in this category.

Problem: allowImportClausesToMergeWithTypes has 1 reference failures and needs smart-triage evidence before implementation starts.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/allowImportClausesToMergeWithTypes.ts
```

Coverage window:

```sh
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/allowImportClausesToMergeWithTypes.ts --detail
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
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/allowImportClausesToMergeWithTypes.ts --detail
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/allowImportClausesToMergeWithTypes.ts
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

- `reference/typescript/tests/cases/compiler/allowImportClausesToMergeWithTypes.ts`

## Duplicate detection

- `issues/open/131-implement-allowImportClausesToMergeWithTypes.md` - Implement Allowimportclausestomergewithtypes (same reference path, same feature label, same group key, title overlap)
- `issues/open/432-implement-import-export.md` - Implement import/export module syntax (same feature label, same group key, title overlap)

## Smart triage

### Smart triage: Triage import export: allowImportClausesToMergeWithTypes

- Issue class: `triage-needed`
- Feature label: `import-export`
- Diagnostic: `UnsupportedSyntax` / `parser-or-frontend-unsupported`
- Path: `reference/typescript/tests/cases/compiler/allowImportClausesToMergeWithTypes.ts`

Reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/allowImportClausesToMergeWithTypes.ts
```

Source overview:

```json
{
  "suite": "tsc",
  "bytes": 405,
  "lines": 28,
  "extension": ".ts",
  "first_code_line": "export const zzz = 123;"
}
```

Failure location:

```json
{
  "code": "UnsupportedSyntax",
  "message": "expected Semicolon, got Some(Ident(\"zzz\")) at 154..157",
  "span_start": 154,
  "span_end": 157,
  "line": 9,
  "column": 1,
  "feature_label": "import-export",
  "error_type": "parser-or-frontend-unsupported"
}
```

Source context:

```text
 6 | 
 7 | // @filename: a.ts
 8 | export default interface zzz {
 9 |     x: string;
10 | }
11 | 
12 | import zzz from "./b";
```

Visible symbols before failure:

```json
[
  {
    "kind": "binding",
    "name": "zzz",
    "line": 4,
    "column": 8,
    "initializer": "123"
  }
]
```

Duplicate candidates:

```json
[
  {
    "state": "open",
    "path": "issues/open/131-implement-allowImportClausesToMergeWithTypes.md",
    "title": "Implement Allowimportclausestomergewithtypes",
    "reason": "same reference path, same feature label, title overlap"
  },
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
        kind: Export,
        span: Span {
            start: 61,
            end: 67,
        },
    },
    SpannedToken {
        kind: Const,
        span: Span {
            start: 68,
            end: 73,
        },
    },
    SpannedToken {
        kind: Ident(
            "zzz",
        ),
        span: Span {
            start: 74,
            end: 77,
        },
    },
    SpannedToken {
        kind: Equal,
        span: Span {
            start: 78,
            end: 79,
        },
    },
    SpannedToken {
        kind: Number(
            123,
        ),
        span: Span {
            start: 80,
            end: 83,
        },
    },
    SpannedToken {
        kind: Semicolon,
        span: Span {
            start: 83,
            end: 84,
        },
    },
    SpannedToken {
        kind: Export,
        span: Span {
            start: 86,
            end: 92,
        },
    },
    SpannedToken {
        kind: Default,
        span: Span {
            start: 93,
            end: 100,
        },
    },
    SpannedToken {
        kind: Ident(
            "zzz",
        ),
        span: Span {
            start: 101,
            end: 104,
        },
    },
    SpannedToken {
        kind: Semicolon,
        span: Span {
            start: 104,
            end: 105,
        },
    },
    SpannedToken {
        kind: Export,
        span: Span {
            start: 129,
            end: 135,
        },
    },
    SpannedToken {
        kind: Default,
        span: Span {
            start: 136,
            end: 143,
        },
    },
    SpannedToken {
        kind: Ident(
            "interface",
        ),
        span: Span {
            start: 144,
            end: 153,
        },
    },
    SpannedToken {
        kind: Ident(
            "zzz",
        ),
        span: Span {
            start: 154,
            end: 157,
        },
    },
    SpannedToken {
        kind: LeftBrace,
        span: Span {
            start: 158,
            end: 159,
        },
    },
    SpannedToken {
        kind: Ident(
            "x",
        ),
        span: Span {
            start: 165,
            end: 166,
        },
    },
    SpannedToken {
        kind: Colon,
        span: Span {
            start: 166,
            end: 167,
        },
    },
    SpannedToken {
        kind: Ident(
            "string",
        ),
        span: Span {
            start: 168,
            end: 174,
        },
    },
    SpannedToken {
        kind: Semicolon,
        span: Span {
            start: 174,
            end: 175,
        },
    },
    SpannedToken {
        kind: RightBrace,
        span: Span {
            start: 177,
            end: 178,
        },
    },
    SpannedToken {
        kind: Import,
        span: Span {
            start: 182,
            end: 188,
        },
    },
    SpannedToken {
        kind: Ident(
            "zzz",
        ),
        span: Span {
            start: 189,
            end: 192,
        },
    },
    SpannedToken {
        kind: Ident(
            "from",
        ),
        span: Span {
            start: 193,
            end: 197,
        },
    },
    SpannedToken {
        kind: String(
            "./b",
        ),
        span: Span {
            start: 198,
            end: 203,
        },
    },
    SpannedToken {
        kind: Semicolon,
        span: Span {
            start: 203,
            end: 204,
        },
    },
    SpannedToken {
        kind: C
```

#### ast

- ok: `False`
- truncated: `False`

```text
error: [UnsupportedSyntax] expected Semicolon, got Some(Ident("zzz")) at 154..157
```

#### resolved

- ok: `False`
- truncated: `False`

```text
error: [UnsupportedSyntax] expected Semicolon, got Some(Ident("zzz")) at 154..157
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
        "message": "Duplicate identifier 'zzz'.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/allowImportClausesToMergeWithTypes.ts",
        "start": 74,
        "length": 3,
        "line": 4,
        "character": 14
      },
      {
        "code": 2528,
        "category": "Error",
        "message": "A module cannot have multiple default exports.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/allowImportClausesToMergeWithTypes.ts",
        "start": 101,
        "length": 3,
        "line": 5,
        "character": 16
      },
      {
        "code": 2300,
        "category": "Error",
        "message": "Duplicate identifier 'zzz'.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/allowImportClausesToMergeWithTypes.ts",
        "start": 154,
        "length": 3,
        "line": 8,
        "character": 26
      },
      {
        "code": 2528,
        "category": "Error",
        "message": "A module cannot have multiple default exports.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/allowImportClausesToMergeWithTypes.ts",
        "start": 154,
        "length": 3,
        "line": 8,
        "character": 26
      },
      {
        "code": 2300,
        "category": "Error",
        "message": "Duplicate identifier 'zzz'.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/allowImportClausesToMergeWithTypes.ts",
        "start": 189,
        "length": 3,
        "line": 12,
        "character": 8
      },
      {
        "code": 2307,
        "category": "Error",
        "message": "Cannot find module './b' or its corresponding type declarations.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/allowImportClausesToMergeWithTypes.ts",
        "start": 198,
        "length": 5,
        "line": 12,
        "character": 17
      },
      {
        "code": 2451,
        "category": "Error",
        "message": "Cannot redeclare block-scoped variable 'x'.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/allowImportClausesToMergeWithTypes.ts",
        "start": 214,
        "length": 1,
        "line": 14,
        "character": 7
      },
      {
        "code": 2528,
        "category": "Error",
        "message": "A module cannot have multiple default exports.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/allowImportClausesToMergeWithTypes.ts",
        "start": 259,
        "length": 7,
        "line": 17,
        "character": 17
      },
      {
        "code": 2300,
        "category": "Error",
        "message": "Duplicate identifier 'zzz'.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/allowImportClausesToMergeWithTypes.ts",
        "start": 304,
        "length": 3,
        "line": 20,
        "character": 8
      },
      {
        "code": 2307,
        "category": "Error",
        "message": "Cannot find module './a' or its corresponding type declarations.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/allowImportClausesToMergeWithTypes.ts",
        "start": 313,
        "length": 5,
        "line": 20,
        "character": 17
      },
      {
        "code": 2451,
        "category": "Error",
        "message": "Cannot redeclare block-scoped variable 'x'.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/allowImportClausesToMergeWithTypes.ts",
        "start": 329,
        "length": 1,
        "line": 22,
        "character": 7
      },
      {
        "code": 2307,
        "category": "Error",
        "message": "Cannot find module './b' or its corresponding type declarations.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/allowImportClausesToMergeWithTypes.ts",
        "start": 382,
        "length": 5,
        "line": 25,
        "character": 25
      }
    ],
    "hints": [
      {
        "kind": "binding",
        "typeText": "123",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/allowImportClausesToMergeWithTypes.ts",
        "start": 74,
        "length": 3,
        "line": 4,
        "character": 14,
        "name": "zzz"
      },
      {
        "kind": "binding",
        "typeText": "zzz",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/allowImportClausesToMergeWithTypes.ts",
        "start": 214,
        "length": 1,
        "line": 14,
        "character": 7,
        "name": "x"
      },
      {
        "kind": "binding",
        "typeText": "zzz",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/allowImportClausesToMergeWithTypes.ts",
        "start": 329,
        "length": 1,
        "line": 22,
        "character": 7,
        "name": "x"
      },
      {
        "kind": "binding",
        "typeText": "originalZZZ",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/allowImportClausesToMergeWithTypes.ts",
        "start": 412,
        "length": 1,
        "line": 28,
        "character": 7,
        "name": "y"
      }
    ],
    "typescriptVersion": "6.0.3"
  },
  "ast": {
    "topLevel": [
      {
        "kind": "FirstStatement",
        "text": "export const zzz = 123;",
        "line": 4,
        "character": 1
      },
      {
        "kind": "ExportAssignment",
        "text": "export default zzz;",
        "line": 5,
        "character": 1
      },
      {
        "kind": "InterfaceDeclaration",
        "text": "export default interface zzz {\r\n    x: string;\r\n}",
        "line": 8,
        "character": 1
      },
      {
        "kind": "ImportDeclaration",
        "text": "import zzz from \"./b\";",
        "line": 12,
        "character": 1
      },
      {
        "kind": "FirstStatement",
        "text": "const x: zzz = { x: \"\" };",
        "line": 14,
        "character": 1
      },
      {
        "kind": "ExpressionStatement",
        "text": "zzz;",
        "line": 15,
        "character": 1
      },
      {
        "kind": "ExportDeclaration",
        "text": "export { zzz as default };",
        "line": 17,
        "character": 1
      },
      {
        "kind": "ImportDeclaration",
        "text": "import zzz from \"./a\";",
        "line": 20,
        "character": 1
      },
      {
        "kind": "FirstStatement",
        "text": "const x: zzz = { x: \"\" };",
        "line": 22,
        "character": 1
      },
      {
        "kind": "ExpressionStatement",
        "text": "zzz;",
        "line": 23,
        "character": 1
      },
      {
        "kind": "ImportDeclaration",
        "text": "import originalZZZ from \"./b\";",
        "line": 25,
        "character": 1
      },
      {
        "kind": "ExpressionStatement",
        "text": "originalZZZ;",
        "line": 26,
        "character": 1
      },
      {
        "kind": "FirstStatement",
        "text": "const y: originalZZZ = x;",
        "line": 28,
        "character": 1
      }
    ],
    "pathToPosition": [
      {
        "kind": "SourceFile",
        "text": "export const zzz = 123;\r\nexport default zzz;\r\n\r\n// @filename: a.ts\r\nexport default interface zzz {\r\n    x: string;\r\n}\r\n\r",
        "line": 4,
        "character": 1
      },
      {
        "kind": "InterfaceDeclaration",
        "text": "export default interface zzz {\r\n    x: string;\r\n}",
        "line": 8,
        "character": 1
      },
      {
        "kind": "Identifier",
        "text": "zzz",
        "line": 8,
        "character": 26
      }
    ]
  }
}
```

Stack trace:

```text
error: [UnsupportedSyntax] expected Semicolon, got Some(Ident("zzz")) at 154..157
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
