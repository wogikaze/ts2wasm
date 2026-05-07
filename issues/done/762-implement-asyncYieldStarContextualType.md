---
id: 762
title: "Implement Asyncyieldstarcontextualtype"
type: spike
area: frontend/syntax
class: blocked
priority: P1
depends_on: []
blocks: []
created: 2026-05-01
updated: 2026-05-07
completed: 2026-05-07
status: done
---

## Summary

Closed this generated bucket by splitting the current concrete blocker to
`issues/open/5345-parse-generic-ambient-const-type-annotations.md`.

## Problem

Fresh triage shows the current first blocker is not `yield*` contextual typing
yet. The compiler stops while parsing the ambient declaration
`declare const authorPromise: Promise<Result<Author, "NOT_FOUND_AUTHOR">>;`,
before it reaches the async generator body.

Problem: nested generic ambient const annotations are not erased as complete
TypeScript type annotations before ambient declaration parsing resumes.

## Current failure

Representative reproduction:

```sh
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/asyncYieldStarContextualType.ts
```

Coverage window:

```sh
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/asyncYieldStarContextualType.ts --detail --no-dashboard-data
```

## Desired final state

This generated bucket is closed. Implement from
`issues/open/5345-parse-generic-ambient-const-type-annotations.md`.

## Scope

In scope:

- [x] Inspect the smart triage report below
- [x] Confirm whether existing open/done issues already cover this bucket
- [x] Split one feature family, one observable behavior, or one fixed reference window into child issues
- [x] Preserve exact reproduction commands and representative AST/diagnostic evidence in the child issue

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
- [x] Child issue contains an exact `reference-triage` command
- [x] Child issue includes failing path, diagnostic code, source context, visible symbols, and parser/TypeScript AST evidence
- [x] Child issue acceptance names the exact fixture/reference path and diagnostic/stdout change

## Validation

Required commands:

```sh
python scripts/manager.py update-issue-index --check
python scripts/manager.py check-issue-health
python scripts/manager.py check-issue-readiness -- --fail-ready-below 80
```

Impacted commands:

```sh
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/asyncYieldStarContextualType.ts --detail --no-dashboard-data
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/asyncYieldStarContextualType.ts
```

Not run:

- cargo fmt / nextest not run for this metadata-only issue lifecycle closure

## Docs / current-state / issue sync

Final-state docs:

- [x] not affected

Current state:

- [x] not affected

Follow-up issues:

- [x] `issues/open/5345-parse-generic-ambient-const-type-annotations.md`

## Notes

## Affected test files

- `reference/typescript/tests/cases/compiler/asyncYieldStarContextualType.ts`

## Duplicate detection

Split to `issues/open/5345-parse-generic-ambient-const-type-annotations.md`.

No exact existing owner was found. Related no-match issues:

- `issues/open/5193-parse-asi-after-ambient-variable-declarations.md` covers
  ASI after ambient declarations, not nested generic annotation erasure.
- `issues/open/5242-parse-direct-generic-call-type-arguments-for-callable-consts.md`
  covers later generic call expression syntax, not declaration parsing.
- `issues/done/5148-parse-generic-async-generator-declarations.md` covers the
  async generator declaration after this ambient declaration blocker advances.

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
    "path": "issues/done/550-implement-FunctionDeclaration-parser-syntax.md",
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

Commits:

- this commit

Validation result:

```text
command: env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/asyncYieldStarContextualType.ts
result: pass; reproduced issue-400 expected ambient variable declaration name and split to issue 5345
date: 2026-05-07

command: env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/asyncYieldStarContextualType.ts --detail --no-dashboard-data
result: pass; executed=1, unsupported=1, UnsupportedTypeScriptSyntax=1
date: 2026-05-07
```

Remaining risks:

- Issue 5345 still needs implementation; this closure only removes the generated bucket from the blocked queue.
