---
id: 753
title: "Implement Asyncfunctionreturntype Parser Syntax"
type: spike
area: runtime/builtins
class: triage-needed
priority: P1
depends_on: []
blocks: []
created: 2026-05-01
updated: 2026-05-01
---

## Summary

Triage asyncFunctionReturnType-parser-syntax across 1 failing reference test cases and split this bucket into implementation-ready child issues.

## Problem

Reference test results show 1 cases fail in directory `asyncFunctionReturnType-parser-syntax` with diagnostics: parser-syntax. The compiler cannot handle these syntax/semantics, preventing compilation of code in this category.

Problem: asyncFunctionReturnType-parser-syntax has 1 reference failures and needs smart-triage evidence before implementation starts.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/asyncFunctionReturnType.2.ts
```

Coverage window:

```sh
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/asyncFunctionReturnType.2.ts --detail
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
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/asyncFunctionReturnType.2.ts --detail
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/asyncFunctionReturnType.2.ts
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

- `reference/typescript/tests/cases/compiler/asyncFunctionReturnType.2.ts`

## Duplicate detection

- `issues/open/442-implement-parser-syntax.md` - Implement parser syntax extensions (same feature label, title overlap)
- `issues/done/464-implement-FunctionDeclaration-parser-syntax.md` - Implement Functiondeclaration Parser Syntax (same feature label, title overlap)
- `issues/done/059-implement-parser-syntax-extensions.md` - Implement parser syntax extensions for TypeScript and advanced JS (same feature label, title overlap)
- `issues/done/065-implement-parser-syntax.md` - Implement parser syntax extensions (same feature label, title overlap)
- `issues/done/065a-merge-duplicate-parser-syntax-issue-into-059.md` - Merge duplicate parser syntax issue into 059 (same feature label, title overlap)
- `issues/done/200-implement-parser-syntax.md` - Implement parser syntax extensions (same feature label, title overlap)
- `issues/done/243-implement-numeric-literal-separator-parser.md` - Implement numeric literal separator parser support (same feature label, title overlap)
- `issues/done/244-implement-bigint-literal-parser-classification.md` - Implement BigInt literal parser classification (same feature label, title overlap)
- `issues/done/246-implement-optional-chaining-parser-support.md` - Implement optional chaining parser support (same feature label, title overlap)
- `issues/done/247-implement-destructuring-binding-pattern-parser.md` - Implement destructuring binding pattern parser support (same feature label, title overlap)

## Smart triage

### Smart triage: Triage parser syntax: asyncFunctionReturnType.2

- Issue class: `triage-needed`
- Feature label: `parser-syntax`
- Diagnostic: `UnsupportedSyntax` / `parser-or-frontend-unsupported`
- Path: `reference/typescript/tests/cases/compiler/asyncFunctionReturnType.2.ts`

Reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/asyncFunctionReturnType.2.ts
```

Source overview:

```json
{
  "suite": "tsc",
  "bytes": 128,
  "lines": 6,
  "extension": ".ts",
  "first_code_line": "class X {"
}
```

Failure location:

```json
{
  "code": "UnsupportedSyntax",
  "message": "expected LeftParen, got Some(Equal) at 96..97",
  "span_start": 96,
  "span_end": 97,
  "line": 5,
  "column": 11,
  "feature_label": "parser-syntax",
  "error_type": "parser-or-frontend-unsupported"
}
```

Source context:

```text
2 | 
3 | // https://github.com/microsoft/TypeScript/issues/47291
4 | class X {
5 |     f = async (): Promise<this> => this;
6 | }
```

Visible symbols before failure:

```json
[
  {
    "kind": "class",
    "name": "X",
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
            start: 79,
            end: 84,
        },
    },
    SpannedToken {
        kind: Ident(
            "X",
        ),
        span: Span {
            start: 85,
            end: 86,
        },
    },
    SpannedToken {
        kind: LeftBrace,
        span: Span {
            start: 87,
            end: 88,
        },
    },
    SpannedToken {
        kind: Ident(
            "f",
        ),
        span: Span {
            start: 94,
            end: 95,
        },
    },
    SpannedToken {
        kind: Equal,
        span: Span {
            start: 96,
            end: 97,
        },
    },
    SpannedToken {
        kind: Async,
        span: Span {
            start: 98,
            end: 103,
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
        kind: Colon,
        span: Span {
            start: 106,
            end: 107,
        },
    },
    SpannedToken {
        kind: Ident(
            "Promise",
        ),
        span: Span {
            start: 108,
            end: 115,
        },
    },
    SpannedToken {
        kind: Less,
        span: Span {
            start: 115,
            end: 116,
        },
    },
    SpannedToken {
        kind: This,
        span: Span {
            start: 116,
            end: 120,
        },
    },
    SpannedToken {
        kind: Greater,
        span: Span {
            start: 120,
            end: 121,
        },
    },
    SpannedToken {
        kind: Arrow,
        span: Span {
            start: 122,
            end: 124,
        },
    },
    SpannedToken {
        kind: This,
        span: Span {
            start: 125,
            end: 129,
        },
    },
    SpannedToken {
        kind: Semicolon,
        span: Span {
            start: 129,
            end: 130,
        },
    },
    SpannedToken {
        kind: RightBrace,
        span: Span {
            start: 132,
            end: 133,
        },
    },
]
```

#### ast

- ok: `False`
- truncated: `False`

```text
error: [UnsupportedSyntax] expected LeftParen, got Some(Equal) at 96..97
```

#### resolved

- ok: `False`
- truncated: `False`

```text
error: [UnsupportedSyntax] expected LeftParen, got Some(Equal) at 96..97
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
        "text": "class X {\r\n    f = async (): Promise<this> => this;\r\n}",
        "line": 4,
        "character": 1
      }
    ],
    "pathToPosition": [
      {
        "kind": "SourceFile",
        "text": "class X {\r\n    f = async (): Promise<this> => this;\r\n}",
        "line": 4,
        "character": 1
      },
      {
        "kind": "ClassDeclaration",
        "text": "class X {\r\n    f = async (): Promise<this> => this;\r\n}",
        "line": 4,
        "character": 1
      },
      {
        "kind": "PropertyDeclaration",
        "text": "f = async (): Promise<this> => this;",
        "line": 5,
        "character": 5
      }
    ]
  }
}
```

Stack trace:

```text
error: [UnsupportedSyntax] expected LeftParen, got Some(Equal) at 96..97
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
