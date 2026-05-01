---
id: 697
title: "Implement Arrowfunctioninconstructorargument"
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

Triage arrowFunctionInConstructorArgument across 1 failing reference test cases and split this bucket into implementation-ready child issues.

## Problem

Reference test results show 1 cases fail in directory `arrowFunctionInConstructorArgument` with diagnostics: parser-syntax. The compiler cannot handle these syntax/semantics, preventing compilation of code in this category.

Problem: arrowFunctionInConstructorArgument has 1 reference failures and needs smart-triage evidence before implementation starts.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/arrowFunctionInConstructorArgument1.ts
```

Coverage window:

```sh
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/arrowFunctionInConstructorArgument1.ts --detail
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
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/arrowFunctionInConstructorArgument1.ts --detail
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/arrowFunctionInConstructorArgument1.ts
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

- `reference/typescript/tests/cases/compiler/arrowFunctionInConstructorArgument1.ts`

## Duplicate detection

- none found by path/title/feature scan

## Smart triage

### Smart triage: Triage parser syntax: arrowFunctionInConstructorArgument1

- Issue class: `triage-needed`
- Feature label: `parser-syntax`
- Diagnostic: `UnsupportedSyntax` / `parser-or-frontend-unsupported`
- Path: `reference/typescript/tests/cases/compiler/arrowFunctionInConstructorArgument1.ts`

Reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/arrowFunctionInConstructorArgument1.ts
```

Source overview:

```json
{
  "suite": "tsc",
  "bytes": 121,
  "lines": 5,
  "extension": ".ts",
  "first_code_line": "class C {"
}
```

Failure location:

```json
{
  "code": "UnsupportedSyntax",
  "message": "expected Semicolon, got None",
  "span_start": null,
  "span_end": null,
  "line": null,
  "column": null,
  "feature_label": "parser-syntax",
  "error_type": "parser-or-frontend-unsupported"
}
```

Source context:

```text
// @target: es2015
class C {
    constructor(x: () => void) { }
}
var c = new C(() => { return asdf; } ) // should error
```

Visible symbols before failure:

```json
[
  {
    "kind": "class",
    "name": "C",
    "line": 2,
    "column": 1
  },
  {
    "kind": "binding",
    "name": "c",
    "line": 5,
    "column": 1,
    "initializer": "new C(() => { return asdf"
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
    "path": "issues/open/464-implement-FunctionDeclaration-parser-syntax.md",
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
- truncated: `True`

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
            "C",
        ),
        span: Span {
            start: 26,
            end: 27,
        },
    },
    SpannedToken {
        kind: LeftBrace,
        span: Span {
            start: 28,
            end: 29,
        },
    },
    SpannedToken {
        kind: Ident(
            "constructor",
        ),
        span: Span {
            start: 35,
            end: 46,
        },
    },
    SpannedToken {
        kind: LeftParen,
        span: Span {
            start: 46,
            end: 47,
        },
    },
    SpannedToken {
        kind: Ident(
            "x",
        ),
        span: Span {
            start: 47,
            end: 48,
        },
    },
    SpannedToken {
        kind: Colon,
        span: Span {
            start: 48,
            end: 49,
        },
    },
    SpannedToken {
        kind: LeftParen,
        span: Span {
            start: 50,
            end: 51,
        },
    },
    SpannedToken {
        kind: RightParen,
        span: Span {
            start: 51,
            end: 52,
        },
    },
    SpannedToken {
        kind: Arrow,
        span: Span {
            start: 53,
            end: 55,
        },
    },
    SpannedToken {
        kind: Void,
        span: Span {
            start: 56,
            end: 60,
        },
    },
    SpannedToken {
        kind: RightParen,
        span: Span {
            start: 60,
            end: 61,
        },
    },
    SpannedToken {
        kind: LeftBrace,
        span: Span {
            start: 62,
            end: 63,
        },
    },
    SpannedToken {
        kind: RightBrace,
        span: Span {
            start: 64,
            end: 65,
        },
    },
    SpannedToken {
        kind: RightBrace,
        span: Span {
            start: 67,
            end: 68,
        },
    },
    SpannedToken {
        kind: Var,
        span: Span {
            start: 70,
            end: 73,
        },
    },
    SpannedToken {
        kind: Ident(
            "c",
        ),
        span: Span {
            start: 74,
            end: 75,
        },
    },
    SpannedToken {
        kind: Equal,
        span: Span {
            start: 76,
            end: 77,
        },
    },
    SpannedToken {
        kind: New,
        span: Span {
            start: 78,
            end: 81,
        },
    },
    SpannedToken {
        kind: Ident(
            "C",
        ),
        span: Span {
            start: 82,
            end: 83,
        },
    },
    SpannedToken {
        kind: LeftParen,
        span: Span {
            start: 83,
            end: 84,
        },
    },
    SpannedToken {
        kind: LeftParen,
        span: Span {
            start: 84,
            end: 85,
        },
    },
    SpannedToken {
        kind: RightParen,
        span: Span {
            start: 85,
            end: 86,
        },
    },
    SpannedToken {
        kind: Arrow,
        span: Span {
            start: 87,
            end: 89,
        },
    },
    SpannedToken {
        kind: LeftBrace,
        span: Span {
            start: 90,
            end: 91,
        },
    },
    SpannedToken {
        kind: Return,
        span: Span {
            start: 92,
            end: 98,
        },
    },
    SpannedToken {
        kind: Ident(
            "asdf",
        ),
        span: Spa
```

#### ast

- ok: `False`
- truncated: `False`

```text
error: [UnsupportedSyntax] expected Semicolon, got None
```

#### resolved

- ok: `False`
- truncated: `False`

```text
error: [UnsupportedSyntax] expected Semicolon, got None
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
        "code": 2304,
        "category": "Error",
        "message": "Cannot find name 'asdf'.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/arrowFunctionInConstructorArgument1.ts",
        "start": 99,
        "length": 4,
        "line": 5,
        "character": 30
      }
    ],
    "hints": [
      {
        "kind": "parameter",
        "typeText": "() => void",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/arrowFunctionInConstructorArgument1.ts",
        "start": 47,
        "length": 1,
        "line": 3,
        "character": 17,
        "name": "x"
      },
      {
        "kind": "binding",
        "typeText": "C",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/arrowFunctionInConstructorArgument1.ts",
        "start": 74,
        "length": 1,
        "line": 5,
        "character": 5,
        "name": "c"
      }
    ],
    "typescriptVersion": "6.0.3"
  }
}
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
