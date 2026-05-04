---
id: 709
title: "Implement Asiines"
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

Triage asiInES across 1 failing reference test cases and split this bucket into implementation-ready child issues.

## Problem

Reference test results show 1 cases fail in directory `asiInES` with diagnostics: parser-syntax. The compiler cannot handle these syntax/semantics, preventing compilation of code in this category.

Problem: asiInES has 1 reference failures and needs smart-triage evidence before implementation starts.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/asiInES6Classes.ts
```

Coverage window:

```sh
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/asiInES6Classes.ts --detail
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
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/asiInES6Classes.ts --detail
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/asiInES6Classes.ts
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

- `reference/typescript/tests/cases/compiler/asiInES6Classes.ts`

## Duplicate detection

- none found by path/title/feature scan

## Smart triage

### Smart triage: Triage parser syntax: asiInES6Classes

- Issue class: `triage-needed`
- Feature label: `parser-syntax`
- Diagnostic: `UnsupportedSyntax` / `parser-or-frontend-unsupported`
- Path: `reference/typescript/tests/cases/compiler/asiInES6Classes.ts`

Reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/asiInES6Classes.ts
```

Source overview:

```json
{
  "suite": "tsc",
  "bytes": 128,
  "lines": 22,
  "extension": ".ts",
  "first_code_line": "class Foo {"
}
```

Failure location:

```json
{
  "code": "UnsupportedSyntax",
  "message": "expected LeftParen, got Some(Equal) at 53..54",
  "span_start": 53,
  "span_end": 54,
  "line": 8,
  "column": 1,
  "feature_label": "parser-syntax",
  "error_type": "parser-or-frontend-unsupported"
}
```

Source context:

```text
 5 | 
 6 |     defaults = {
 7 | 
 8 |         done: false
 9 | 
10 |     }
11 |
```

Visible symbols before failure:

```json
[
  {
    "kind": "class",
    "name": "Foo",
    "line": 2,
    "column": 1
  }
]
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
    "state": "done",
    "path": "issues/done/059-implement-parser-syntax-extensions.md",
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
  },
  {
    "state": "done",
    "path": "issues/done/243-implement-numeric-literal-separator-parser.md",
    "title": "Implement numeric literal separator parser support",
    "reason": "same feature label, title overlap"
  },
  {
    "state": "done",
    "path": "issues/done/244-implement-bigint-literal-parser-classification.md",
    "title": "Implement BigInt literal parser classification",
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
- truncated: `False`

```text
== tokens ==
[
    SpannedToken {
        kind: Class,
        span: Span {
            start: 20,
            end: 25,
        },
    },
    SpannedToken {
        kind: Ident(
            "Foo",
        ),
        span: Span {
            start: 26,
            end: 29,
        },
    },
    SpannedToken {
        kind: LeftBrace,
        span: Span {
            start: 30,
            end: 31,
        },
    },
    SpannedToken {
        kind: Ident(
            "defaults",
        ),
        span: Span {
            start: 44,
            end: 52,
        },
    },
    SpannedToken {
        kind: Equal,
        span: Span {
            start: 53,
            end: 54,
        },
    },
    SpannedToken {
        kind: LeftBrace,
        span: Span {
            start: 55,
            end: 56,
        },
    },
    SpannedToken {
        kind: Ident(
            "done",
        ),
        span: Span {
            start: 68,
            end: 72,
        },
    },
    SpannedToken {
        kind: Colon,
        span: Span {
            start: 72,
            end: 73,
        },
    },
    SpannedToken {
        kind: False,
        span: Span {
            start: 74,
            end: 79,
        },
    },
    SpannedToken {
        kind: RightBrace,
        span: Span {
            start: 87,
            end: 88,
        },
    },
    SpannedToken {
        kind: Ident(
            "bar",
        ),
        span: Span {
            start: 101,
            end: 104,
        },
    },
    SpannedToken {
        kind: LeftParen,
        span: Span {
            start: 104,
            end: 105,
        },
    },
    SpannedToken {
        kind: RightParen,
        span: Span {
            start: 105,
            end: 106,
        },
    },
    SpannedToken {
        kind: LeftBrace,
        span: Span {
            start: 107,
            end: 108,
        },
    },
    SpannedToken {
        kind: Return,
        span: Span {
            start: 120,
            end: 126,
        },
    },
    SpannedToken {
        kind: Number(
            3,
        ),
        span: Span {
            start: 127,
            end: 128,
        },
    },
    SpannedToken {
        kind: Semicolon,
        span: Span {
            start: 128,
            end: 129,
        },
    },
    SpannedToken {
        kind: RightBrace,
        span: Span {
            start: 137,
            end: 138,
        },
    },
    SpannedToken {
        kind: RightBrace,
        span: Span {
            start: 147,
            end: 148,
        },
    },
]
```

#### ast

- ok: `False`
- truncated: `False`

```text
error: [UnsupportedSyntax] expected LeftParen, got Some(Equal) at 53..54
```

#### resolved

- ok: `False`
- truncated: `False`

```text
error: [UnsupportedSyntax] expected LeftParen, got Some(Equal) at 53..54
```

TypeScript/JavaScript oracle:

```json
{
  "ok": true,
  "returncode": 0,
  "typescript": {
    "ok": true,
    "diagnostics": [],
    "hints": [],
    "typescriptVersion": "6.0.3"
  },
  "ast": {
    "topLevel": [
      {
        "kind": "ClassDeclaration",
        "text": "class Foo {\r\n\r\n \r\n\r\n    defaults = {\r\n\r\n        done: false\r\n\r\n    }\r\n\r\n \r\n\r\n    bar() {\r\n\r\n        return 3;\r\n\r\n    }\r\n",
        "line": 2,
        "character": 1
      }
    ],
    "pathToPosition": [
      {
        "kind": "SourceFile",
        "text": "class Foo {\r\n\r\n \r\n\r\n    defaults = {\r\n\r\n        done: false\r\n\r\n    }\r\n\r\n \r\n\r\n    bar() {\r\n\r\n        return 3;\r\n\r\n    }\r\n",
        "line": 2,
        "character": 1
      },
      {
        "kind": "ClassDeclaration",
        "text": "class Foo {\r\n\r\n \r\n\r\n    defaults = {\r\n\r\n        done: false\r\n\r\n    }\r\n\r\n \r\n\r\n    bar() {\r\n\r\n        return 3;\r\n\r\n    }\r\n",
        "line": 2,
        "character": 1
      },
      {
        "kind": "PropertyDeclaration",
        "text": "defaults = {\r\n\r\n        done: false\r\n\r\n    }",
        "line": 6,
        "character": 5
      }
    ]
  }
}
```

Stack trace:

```text
error: [UnsupportedSyntax] expected LeftParen, got Some(Equal) at 53..54
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
