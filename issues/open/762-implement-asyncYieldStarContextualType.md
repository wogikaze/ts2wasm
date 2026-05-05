---
id: 762
title: "Implement Asyncyieldstarcontextualtype"
type: spike
area: frontend/semantics
class: blocked
priority: P1
depends_on: [5001]
blocks: []
created: 2026-05-01
updated: 2026-05-01
---

## Summary

Triage asyncYieldStarContextualType across 1 failing reference test cases and split this bucket into implementation-ready child issues.

## Problem

Reference test results show 1 cases fail in directory `asyncYieldStarContextualType` with diagnostics: parser-syntax. The compiler cannot handle these syntax/semantics, preventing compilation of code in this category.

Problem: asyncYieldStarContextualType has 1 reference failures and needs smart-triage evidence before implementation starts.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/asyncYieldStarContextualType.ts
```

Coverage window:

```sh
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/asyncYieldStarContextualType.ts --detail
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
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/asyncYieldStarContextualType.ts --detail
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/asyncYieldStarContextualType.ts
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

- `reference/typescript/tests/cases/compiler/asyncYieldStarContextualType.ts`

## Duplicate detection

- none found by path/title/feature scan

## Smart triage

### Smart triage: Triage parser syntax: asyncYieldStarContextualType

- Issue class: `triage-needed`
- Feature label: `parser-syntax`
- Diagnostic: `UnsupportedTypeScriptSyntax` / `unsupported-feature-boundary`
- Path: `reference/typescript/tests/cases/compiler/asyncYieldStarContextualType.ts`

Reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/asyncYieldStarContextualType.ts
```

Source overview:

```json
{
  "suite": "tsc",
  "bytes": 1094,
  "lines": 31,
  "extension": ".ts",
  "first_code_line": "interface Result<T, E> {"
}
```

Failure location:

```json
{
  "code": "UnsupportedTypeScriptSyntax",
  "message": "issue-400: expected ambient variable declaration name at 338..345",
  "span_start": 338,
  "span_end": 345,
  "line": 13,
  "column": 13,
  "feature_label": "parser-syntax",
  "error_type": "unsupported-feature-boundary"
}
```

Source context:

```text
10 | type Author = { id: string; name: string };
11 | type BookWithAuthor = Book & { author: Author };
12 | 
13 | declare const authorPromise: Promise<Result<Author, "NOT_FOUND_AUTHOR">>;
14 | declare const mapper: <T>(result: Result<T, "NOT_FOUND_AUTHOR">) => Result<T, "NOT_FOUND_AUTHOR">;
15 | declare const g: <T, U, V>() => AsyncGenerator<T, U, V>;
16 |
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
    "path": "issues/open/442-implement-parser-syntax.md",
    "title": "Implement parser syntax extensions",
    "reason": "same feature label, title overlap"
  },
  {
    "state": "open",
    "path": "issues/done/464-implement-FunctionDeclaration-parser-syntax.md",
    "title": "Implement Functiondeclaration Parser Syntax",
    "reason": "same feature label, title overlap"
  },
  {
    "state": "open",
    "path": "issues/open/550-implement-FunctionDeclaration-parser-syntax.md",
    "title": "Implement Functiondeclaration Parser Syntax",
    "reason": "same feature label, title overlap"
  },
  {
    "state": "open",
    "path": "issues/open/663-implement-arrayAssignmentTest-parser-syntax.md",
    "title": "Implement Arrayassignmenttest Parser Syntax",
    "reason": "same feature label, title overlap"
  },
  {
    "state": "open",
    "path": "issues/open/734-implement-assignmentCompatability-parser-syntax.md",
    "title": "Implement Assignmentcompatability Parser Syntax",
    "reason": "same feature label, title overlap"
  },
  {
    "state": "open",
    "path": "issues/open/753-implement-asyncFunctionReturnType-parser-syntax.md",
    "title": "Implement Asyncfunctionreturntype Parser Syntax",
    "reason": "same feature label, title overlap"
  },
  {
    "state": "done",
    "path": "issues/open/059-implement-parser-syntax-extensions.md",
    "title": "Implement parser syntax extensions for TypeScript and advanced JS",
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
  },
  {
    "state": "done",
    "path": "issues/done/200-implement-parser-syntax.md",
    "title": "Implement parser syntax extensions",
    "reason": "same feature label, title overlap"
  }
]
```

Error-specific suggestions:

- Create a child issue around this exact path and diagnostic before broadening the reference window.

Compiler dumps:

#### tokens

- ok: `True`
- truncated: `True`

```text
== tokens ==
[
    SpannedToken {
        kind: Ident(
            "interface",
        ),
        span: Span {
            start: 97,
            end: 106,
        },
    },
    SpannedToken {
        kind: Ident(
            "Result",
        ),
        span: Span {
            start: 107,
            end: 113,
        },
    },
    SpannedToken {
        kind: Less,
        span: Span {
            start: 113,
            end: 114,
        },
    },
    SpannedToken {
        kind: Ident(
            "T",
        ),
        span: Span {
            start: 114,
            end: 115,
        },
    },
    SpannedToken {
        kind: Comma,
        span: Span {
            start: 115,
            end: 116,
        },
    },
    SpannedToken {
        kind: Ident(
            "E",
        ),
        span: Span {
            start: 117,
            end: 118,
        },
    },
    SpannedToken {
        kind: Greater,
        span: Span {
            start: 118,
            end: 119,
        },
    },
    SpannedToken {
        kind: LeftBrace,
        span: Span {
            start: 120,
            end: 121,
        },
    },
    SpannedToken {
        kind: LeftBracket,
        span: Span {
            start: 127,
            end: 128,
        },
    },
    SpannedToken {
        kind: Ident(
            "Symbol",
        ),
        span: Span {
            start: 128,
            end: 134,
        },
    },
    SpannedToken {
        kind: Dot,
        span: Span {
            start: 134,
            end: 135,
        },
    },
    SpannedToken {
        kind: Ident(
            "iterator",
        ),
        span: Span {
            start: 135,
            end: 143,
        },
    },
    SpannedToken {
        kind: RightBracket,
        span: Span {
            start: 143,
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
        kind: RightParen,
        span: Span {
            start: 145,
            end: 146,
        },
    },
    SpannedToken {
        kind: Colon,
        span: Span {
            start: 146,
            end: 147,
        },
    },
    SpannedToken {
        kind: Ident(
            "Generator",
        ),
        span: Span {
            start: 148,
            end: 157,
        },
    },
    SpannedToken {
        kind: Less,
        span: Span {
            start: 157,
            end: 158,
        },
    },
    SpannedToken {
        kind: Ident(
            "E",
        ),
        span: Span {
            start: 158,
            end: 159,
        },
    },
    SpannedToken {
        kind: Comma,
        span: Span {
            start: 159,
            end: 160,
        },
    },
    SpannedToken {
        kind: Ident(
            "T",
        ),
        span: Span {
            start: 161,
            end: 162,
        },
    },
    SpannedToken {
        kind: Comma,
        span: Span {
            start: 162,
            end: 163,
        },
    },
    SpannedToken {
        kind: Ident(
            "unknown",
        ),
        span: Span {
            start: 164,
            end: 171,
        },
    },
    SpannedToken {
        kind: Greater,
        span: Span {
            start: 171,
            end: 172,
        },
    },
    SpannedToken {
        kind: RightBrace,
        span: Span {
            start: 174,
            end: 175,
        },
    },
    Spanned
```

#### ast

- ok: `False`
- truncated: `False`

```text
error: [UnsupportedTypeScriptSyntax] issue-400: expected ambient variable declaration name at 338..345
```

#### resolved

- ok: `False`
- truncated: `False`

```text
error: [UnsupportedTypeScriptSyntax] issue-400: expected ambient variable declaration name at 338..345
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
        "kind": "binding",
        "typeText": "Promise<Result<Author, \"NOT_FOUND_AUTHOR\">>",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/asyncYieldStarContextualType.ts",
        "start": 352,
        "length": 13,
        "line": 13,
        "character": 15,
        "name": "authorPromise"
      },
      {
        "kind": "binding",
        "typeText": "<T>(result: Result<T, \"NOT_FOUND_AUTHOR\">) => Result<T, \"NOT_FOUND_AUTHOR\">",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/asyncYieldStarContextualType.ts",
        "start": 427,
        "length": 6,
        "line": 14,
        "character": 15,
        "name": "mapper"
      },
      {
        "kind": "parameter",
        "typeText": "Result<T, \"NOT_FOUND_AUTHOR\">",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/asyncYieldStarContextualType.ts",
        "start": 439,
        "length": 6,
        "line": 14,
        "character": 27,
        "name": "result"
      },
      {
        "kind": "binding",
        "typeText": "<T, U, V>() => AsyncGenerator<T, U, V>",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/asyncYieldStarContextualType.ts",
        "start": 527,
        "length": 1,
        "line": 15,
        "character": 15,
        "name": "g"
      },
      {
        "kind": "function",
        "typeText": "AsyncGenerator<\"NOT_FOUND_AUTHOR\" | \"NOT_FOUND_BOOK\", BookWithAuthor, unknown>",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/asyncYieldStarContextualType.ts",
        "start": 589,
        "length": 1,
        "line": 17,
        "character": 17,
        "name": "f"
      },
      {
        "kind": "binding",
        "typeText": "Result<Author, \"NOT_FOUND_AUTHOR\">",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/asyncYieldStarContextualType.ts",
        "start": 775,
        "length": 5,
        "line": 20,
        "character": 11,
        "name": "test1"
      },
      {
        "kind": "binding",
        "typeText": "Author",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/asyncYieldStarContextualType.ts",
        "start": 969,
        "length": 5,
        "line": 25,
        "character": 11,
        "name": "test2"
      },
      {
        "kind": "binding",
        "typeText": "unknown",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/asyncYieldStarContextualType.ts",
        "start": 1030,
        "length": 2,
        "line": 27,
        "character": 11,
        "name": "x1"
      },
      {
        "kind": "binding",
        "typeText": "number",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/asyncYieldStarContextualType.ts",
        "start": 1058,
        "length": 2,
        "line": 28,
        "character": 11,
        "name": "x2"
      }
    ],
    "typescriptVersion": "6.0.3"
  },
  "ast": {
    "topLevel": [
      {
        "kind": "InterfaceDeclaration",
        "text": "interface Result<T, E> {\r\n    [Symbol.iterator](): Generator<E, T, unknown>\r\n}",
        "line": 5,
        "character": 1
      },
      {
        "kind": "TypeAliasDeclaration",
        "text": "type Book = { id: string; title: string; authorId: string };",
        "line": 9,
        "character": 1
      },
      {
        "kind": "TypeAliasDeclaration",
        "text": "type Author = { id: string; name: string };",
        "line": 10,
        "character": 1
      },
      {
        "kind": "TypeAliasDeclaration",
        "text": "type BookWithAuthor = Book & { author: Author };",
        "line": 11,
        "character": 1
      },
      {
        "kind": "FirstStatement",
        "text": "declare const authorPromise: Promise<Result<Author, \"NOT_FOUND_AUTHOR\">>;",
        "line": 13,
        "character": 1
      },
      {
        "kind": "FirstStatement",
        "text": "declare const mapper: <T>(result: Result<T, \"NOT_FOUND_AUTHOR\">) => Result<T, \"NOT_FOUND_AUTHOR\">;",
        "line": 14,
        "character": 1
      },
      {
        "kind": "FirstStatement",
        "text": "declare const g: <T, U, V>() => AsyncGenerator<T, U, V>;",
        "line": 15,
        "character": 1
      },
      {
        "kind": "FunctionDeclaration",
        "text": "async function* f(): AsyncGenerator<\"NOT_FOUND_AUTHOR\" | \"NOT_FOUND_BOOK\", BookWithAuthor, unknown> {\r\n    // Without yi",
        "line": 17,
        "character": 1
      }
    ],
    "pathToPosition": [
      {
        "kind": "SourceFile",
        "text": "interface Result<T, E> {\r\n    [Symbol.iterator](): Generator<E, T, unknown>\r\n}\r\n\r\ntype Book = { id: string; title: strin",
        "line": 5,
        "character": 1
      },
      {
        "kind": "FirstStatement",
        "text": "declare const authorPromise: Promise<Result<Author, \"NOT_FOUND_AUTHOR\">>;",
        "line": 13,
        "character": 1
      },
      {
        "kind": "DeclareKeyword",
        "text": "declare",
        "line": 13,
        "character": 1
      }
    ]
  }
}
```

Stack trace:

```text
error: [UnsupportedTypeScriptSyntax] issue-400: expected ambient variable declaration name at 338..345
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
