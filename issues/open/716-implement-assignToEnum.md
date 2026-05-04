---
id: 716
title: "Implement Assigntoenum"
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

Triage assignToEnum across 1 failing reference test cases and split this bucket into implementation-ready child issues.

## Problem

Reference test results show 1 cases fail in directory `assignToEnum` with diagnostics: parser-syntax. The compiler cannot handle these syntax/semantics, preventing compilation of code in this category.

Problem: assignToEnum has 1 reference failures and needs smart-triage evidence before implementation starts.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/assignToEnum.ts
```

Coverage window:

```sh
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/assignToEnum.ts --detail
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
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/assignToEnum.ts --detail
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/assignToEnum.ts
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

- `reference/typescript/tests/cases/compiler/assignToEnum.ts`

## Duplicate detection

- none found by path/title/feature scan

## Smart triage

### Smart triage: Triage parser syntax: assignToEnum

- Issue class: `triage-needed`
- Feature label: `parser-syntax`
- Diagnostic: `UnsupportedTypeScriptSyntax` / `unsupported-feature-boundary`
- Path: `reference/typescript/tests/cases/compiler/assignToEnum.ts`

Reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/assignToEnum.ts
```

Source overview:

```json
{
  "suite": "tsc",
  "bytes": 164,
  "lines": 7,
  "extension": ".ts",
  "first_code_line": "enum A { foo, bar }"
}
```

Failure location:

```json
{
  "code": "UnsupportedTypeScriptSyntax",
  "message": "TypeScript enum declarations require an explicit frontend transform before runtime lowering at 20..24",
  "span_start": 20,
  "span_end": 24,
  "line": 2,
  "column": 2,
  "feature_label": "parser-syntax",
  "error_type": "unsupported-feature-boundary"
}
```

Source context:

```text
1 | // @target: es2015
2 | enum A { foo, bar }
3 | A = undefined;  // invalid LHS
4 | A = A.bar;      // invalid LHS
5 | A.foo = 1;      // invalid LHS
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
    "path": "issues/open/200-implement-parser-syntax.md",
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
            "enum",
        ),
        span: Span {
            start: 20,
            end: 24,
        },
    },
    SpannedToken {
        kind: Ident(
            "A",
        ),
        span: Span {
            start: 25,
            end: 26,
        },
    },
    SpannedToken {
        kind: LeftBrace,
        span: Span {
            start: 27,
            end: 28,
        },
    },
    SpannedToken {
        kind: Ident(
            "foo",
        ),
        span: Span {
            start: 29,
            end: 32,
        },
    },
    SpannedToken {
        kind: Comma,
        span: Span {
            start: 32,
            end: 33,
        },
    },
    SpannedToken {
        kind: Ident(
            "bar",
        ),
        span: Span {
            start: 34,
            end: 37,
        },
    },
    SpannedToken {
        kind: RightBrace,
        span: Span {
            start: 38,
            end: 39,
        },
    },
    SpannedToken {
        kind: Ident(
            "A",
        ),
        span: Span {
            start: 41,
            end: 42,
        },
    },
    SpannedToken {
        kind: Equal,
        span: Span {
            start: 43,
            end: 44,
        },
    },
    SpannedToken {
        kind: Undefined,
        span: Span {
            start: 45,
            end: 54,
        },
    },
    SpannedToken {
        kind: Semicolon,
        span: Span {
            start: 54,
            end: 55,
        },
    },
    SpannedToken {
        kind: Ident(
            "A",
        ),
        span: Span {
            start: 73,
            end: 74,
        },
    },
    SpannedToken {
        kind: Equal,
        span: Span {
            start: 75,
            end: 76,
        },
    },
    SpannedToken {
        kind: Ident(
            "A",
        ),
        span: Span {
            start: 77,
            end: 78,
        },
    },
    SpannedToken {
        kind: Dot,
        span: Span {
            start: 78,
            end: 79,
        },
    },
    SpannedToken {
        kind: Ident(
            "bar",
        ),
        span: Span {
            start: 79,
            end: 82,
        },
    },
    SpannedToken {
        kind: Semicolon,
        span: Span {
            start: 82,
            end: 83,
        },
    },
    SpannedToken {
        kind: Ident(
            "A",
        ),
        span: Span {
            start: 105,
            end: 106,
        },
    },
    SpannedToken {
        kind: Dot,
        span: Span {
            start: 106,
            end: 107,
        },
    },
    SpannedToken {
        kind: Ident(
            "foo",
        ),
        span: Span {
            start: 107,
            end: 110,
        },
    },
    SpannedToken {
        kind: Equal,
        span: Span {
            start: 111,
            end: 112,
        },
    },
    SpannedToken {
        kind: Number(
            1,
        ),
        span: Span {
            start: 113,
            end: 114,
        },
    },
    SpannedToken {
        kind: Semicolon,
        span: Span {
            start: 114,
            end: 115,
        },
    },
    SpannedToken {
        kind: Ident(
            "A",
        ),
        span: Span {
            start: 137,
            end: 138,
        },
    },
    SpannedToken {
        kind: Dot,
        span: Span {
            start: 138,
            end: 139,
        },
    },
    SpannedToken {
        ki
```

#### ast

- ok: `False`
- truncated: `False`

```text
error: [UnsupportedTypeScriptSyntax] TypeScript enum declarations require an explicit frontend transform before runtime lowering at 20..24
```

#### resolved

- ok: `False`
- truncated: `False`

```text
error: [UnsupportedTypeScriptSyntax] TypeScript enum declarations require an explicit frontend transform before runtime lowering at 20..24
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
        "code": 2628,
        "category": "Error",
        "message": "Cannot assign to 'A' because it is an enum.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/assignToEnum.ts",
        "start": 41,
        "length": 1,
        "line": 3,
        "character": 1
      },
      {
        "code": 2628,
        "category": "Error",
        "message": "Cannot assign to 'A' because it is an enum.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/assignToEnum.ts",
        "start": 73,
        "length": 1,
        "line": 4,
        "character": 1
      },
      {
        "code": 2540,
        "category": "Error",
        "message": "Cannot assign to 'foo' because it is a read-only property.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/assignToEnum.ts",
        "start": 107,
        "length": 3,
        "line": 5,
        "character": 3
      },
      {
        "code": 2540,
        "category": "Error",
        "message": "Cannot assign to 'foo' because it is a read-only property.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/assignToEnum.ts",
        "start": 139,
        "length": 3,
        "line": 6,
        "character": 3
      }
    ],
    "hints": [],
    "typescriptVersion": "6.0.3"
  },
  "ast": {
    "topLevel": [
      {
        "kind": "EnumDeclaration",
        "text": "enum A { foo, bar }",
        "line": 2,
        "character": 1
      },
      {
        "kind": "ExpressionStatement",
        "text": "A = undefined;",
        "line": 3,
        "character": 1
      },
      {
        "kind": "ExpressionStatement",
        "text": "A = A.bar;",
        "line": 4,
        "character": 1
      },
      {
        "kind": "ExpressionStatement",
        "text": "A.foo = 1;",
        "line": 5,
        "character": 1
      },
      {
        "kind": "ExpressionStatement",
        "text": "A.foo = A.bar;",
        "line": 6,
        "character": 1
      }
    ],
    "pathToPosition": [
      {
        "kind": "SourceFile",
        "text": "enum A { foo, bar }\r\nA = undefined;  // invalid LHS\r\nA = A.bar;      // invalid LHS\r\nA.foo = 1;      // invalid LHS\r\nA.f",
        "line": 2,
        "character": 1
      },
      {
        "kind": "EnumDeclaration",
        "text": "enum A { foo, bar }",
        "line": 2,
        "character": 1
      }
    ]
  }
}
```

Stack trace:

```text
error: [UnsupportedTypeScriptSyntax] TypeScript enum declarations require an explicit frontend transform before runtime lowering at 20..24
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
