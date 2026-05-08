---
id: 737
title: "Implement Assignmentnonobjecttypeconstraints"
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

Triage assignmentNonObjectTypeConstraints across 1 failing reference test cases and split this bucket into implementation-ready child issues.

## Problem

Reference test results show 1 cases fail in directory `assignmentNonObjectTypeConstraints` with diagnostics: object-literal. The compiler cannot handle these syntax/semantics, preventing compilation of code in this category.

Problem: assignmentNonObjectTypeConstraints has 1 reference failures and needs smart-triage evidence before implementation starts.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/assignmentNonObjectTypeConstraints.ts
```

Coverage window:

```sh
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/assignmentNonObjectTypeConstraints.ts --detail
```

## Desired final state

This generated bucket is either split into implementation-ready child issues or superseded by an existing open/done issue with matching evidence. Do not implement directly from this bucket.

## Scope

In scope:

- [x] Inspect the smart triage report below
- [x] Confirm whether existing open/done issues already cover this bucket
- [x] Split one feature family, one observable behavior, or one fixed reference window into child issues
- [x] Preserve exact reproduction commands and representative AST/diagnostic evidence in each child issue

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
- [x] At least one child issue contains an exact `mise run reference-triage -- ...` command
- [x] Child issue includes failing path, diagnostic code, source context, visible symbols, and parser/TypeScript AST evidence
- [x] Child issue acceptance names the exact fixture/reference path and diagnostic/stdout change

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run
```

Impacted commands:

```sh
mise run reference-coverage -- tsc --limit 2
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/assignmentNonObjectTypeConstraints.ts --detail
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/assignmentNonObjectTypeConstraints.ts
```

Not run:

- none

## Docs / current-state / issue sync

Final-state docs:

- [x] not affected

Current state:

- [x] updated: `current-state.md` (repo root)

Follow-up issues:

- [x] none

## Notes

## Affected test files

- `reference/typescript/tests/cases/compiler/assignmentNonObjectTypeConstraints.ts`

## Duplicate detection

- none found by path/title/feature scan

## Smart triage

### Smart triage: Triage object literal: assignmentNonObjectTypeConstraints

- Issue class: `triage-needed`
- Feature label: `object-literal`
- Diagnostic: `UnsupportedSyntax` / `parser-or-frontend-unsupported`
- Path: `reference/typescript/tests/cases/compiler/assignmentNonObjectTypeConstraints.ts`

Reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/assignmentNonObjectTypeConstraints.ts
```

Source overview:

```json
{
  "suite": "tsc",
  "bytes": 277,
  "lines": 20,
  "extension": ".ts",
  "first_code_line": "const enum E { A, B, C }"
}
```

Failure location:

```json
{
  "code": "UnsupportedSyntax",
  "message": "const declarations require an initializer at 45..49",
  "span_start": 45,
  "span_end": 49,
  "line": 3,
  "column": 9,
  "feature_label": "object-literal",
  "error_type": "parser-or-frontend-unsupported"
}
```

Source context:

```text
1 | // @target: es2015
2 | // @strict: false
3 | const enum E { A, B, C }
4 | 
5 | function foo<T extends number>(x: T) {
6 |     var y: number = x;  // Ok
```

Visible symbols before failure:

```json
[
  {
    "kind": "binding",
    "name": "en",
    "line": 3,
    "column": 1
  }
]
```

Duplicate candidates:

```json
[
  {
    "state": "open",
    "path": "issues/done/374-design-broader-object-toprimitive-for-bigint-comparisons.md",
    "title": "Design broader object ToPrimitive for mixed BigInt comparisons",
    "reason": "same feature label, title overlap"
  },
  {
    "state": "open",
    "path": "issues/open/441-implement-object-literal.md",
    "title": "Implement object literal enhancements",
    "reason": "same feature label, title overlap"
  },
  {
    "state": "done",
    "path": "issues/done/355-dynamic-object-enumeration-spread.md",
    "title": "Implement dynamic object property enumeration spread",
    "reason": "same feature label, title overlap"
  },
  {
    "state": "done",
    "path": "issues/open/372-implement-bigint-object-toprimitive-non-bigint-primitive-returns.md",
    "title": "Implement BigInt object ToPrimitive non-BigInt primitive returns",
    "reason": "same feature label, title overlap"
  },
  {
    "state": "done",
    "path": "issues/open/373-handle-bigint-object-toprimitive-invalid-out-of-range-string-returns.md",
    "title": "Handle BigInt object ToPrimitive invalid and out-of-range string returns",
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
        kind: Const,
        span: Span {
            start: 39,
            end: 44,
        },
    },
    SpannedToken {
        kind: Ident(
            "enum",
        ),
        span: Span {
            start: 45,
            end: 49,
        },
    },
    SpannedToken {
        kind: Ident(
            "E",
        ),
        span: Span {
            start: 50,
            end: 51,
        },
    },
    SpannedToken {
        kind: LeftBrace,
        span: Span {
            start: 52,
            end: 53,
        },
    },
    SpannedToken {
        kind: Ident(
            "A",
        ),
        span: Span {
            start: 54,
            end: 55,
        },
    },
    SpannedToken {
        kind: Comma,
        span: Span {
            start: 55,
            end: 56,
        },
    },
    SpannedToken {
        kind: Ident(
            "B",
        ),
        span: Span {
            start: 57,
            end: 58,
        },
    },
    SpannedToken {
        kind: Comma,
        span: Span {
            start: 58,
            end: 59,
        },
    },
    SpannedToken {
        kind: Ident(
            "C",
        ),
        span: Span {
            start: 60,
            end: 61,
        },
    },
    SpannedToken {
        kind: RightBrace,
        span: Span {
            start: 62,
            end: 63,
        },
    },
    SpannedToken {
        kind: Function,
        span: Span {
            start: 67,
            end: 75,
        },
    },
    SpannedToken {
        kind: Ident(
            "foo",
        ),
        span: Span {
            start: 76,
            end: 79,
        },
    },
    SpannedToken {
        kind: Less,
        span: Span {
            start: 79,
            end: 80,
        },
    },
    SpannedToken {
        kind: Ident(
            "T",
        ),
        span: Span {
            start: 80,
            end: 81,
        },
    },
    SpannedToken {
        kind: Extends,
        span: Span {
            start: 82,
            end: 89,
        },
    },
    SpannedToken {
        kind: Ident(
            "number",
        ),
        span: Span {
            start: 90,
            end: 96,
        },
    },
    SpannedToken {
        kind: Greater,
        span: Span {
            start: 96,
            end: 97,
        },
    },
    SpannedToken {
        kind: LeftParen,
        span: Span {
            start: 97,
            end: 98,
        },
    },
    SpannedToken {
        kind: Ident(
            "x",
        ),
        span: Span {
            start: 98,
            end: 99,
        },
    },
    SpannedToken {
        kind: Colon,
        span: Span {
            start: 99,
            end: 100,
        },
    },
    SpannedToken {
        kind: Ident(
            "T",
        ),
        span: Span {
            start: 101,
            end: 102,
        },
    },
    SpannedToken {
        kind: RightParen,
        span: Span {
            start: 102,
            end: 103,
        },
    },
    SpannedToken {
        kind: LeftBrace,
        span: Span {
            start: 104,
            end: 105,
        },
    },
    SpannedToken {
        kind: Var,
        span: Span {
            start: 111,
            end: 114,
        },
    },
    SpannedToken {
        kind: Ident(
            "y",
        ),
        span: Span {
            start: 115,
            end: 116,
        },
    },
    SpannedToken {
        kind: Colon,
        span: S
```

#### ast

- ok: `False`
- truncated: `False`

```text
error: [UnsupportedSyntax] const declarations require an initializer at 45..49
```

#### resolved

- ok: `False`
- truncated: `False`

```text
error: [UnsupportedSyntax] const declarations require an initializer at 45..49
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
        "typeText": "void",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/assignmentNonObjectTypeConstraints.ts",
        "start": 76,
        "length": 3,
        "line": 5,
        "character": 10,
        "name": "foo"
      },
      {
        "kind": "parameter",
        "typeText": "T",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/assignmentNonObjectTypeConstraints.ts",
        "start": 98,
        "length": 1,
        "line": 5,
        "character": 32,
        "name": "x"
      },
      {
        "kind": "binding",
        "typeText": "number",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/assignmentNonObjectTypeConstraints.ts",
        "start": 115,
        "length": 1,
        "line": 6,
        "character": 9,
        "name": "y"
      },
      {
        "kind": "function",
        "typeText": "void",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/assignmentNonObjectTypeConstraints.ts",
        "start": 206,
        "length": 3,
        "line": 15,
        "character": 10,
        "name": "bar"
      },
      {
        "kind": "parameter",
        "typeText": "T",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/assignmentNonObjectTypeConstraints.ts",
        "start": 227,
        "length": 1,
        "line": 15,
        "character": 31,
        "name": "x"
      },
      {
        "kind": "binding",
        "typeText": "A | B",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/assignmentNonObjectTypeConstraints.ts",
        "start": 244,
        "length": 1,
        "line": 16,
        "character": 9,
        "name": "y"
      }
    ],
    "typescriptVersion": "6.0.3"
  },
  "ast": {
    "topLevel": [
      {
        "kind": "EnumDeclaration",
        "text": "const enum E { A, B, C }",
        "line": 3,
        "character": 1
      },
      {
        "kind": "FunctionDeclaration",
        "text": "function foo<T extends number>(x: T) {\r\n    var y: number = x;  // Ok\r\n}",
        "line": 5,
        "character": 1
      },
      {
        "kind": "ExpressionStatement",
        "text": "foo(5);",
        "line": 9,
        "character": 1
      },
      {
        "kind": "ExpressionStatement",
        "text": "foo(E.A);",
        "line": 10,
        "character": 1
      },
      {
        "kind": "ClassDeclaration",
        "text": "class A { a }",
        "line": 12,
        "character": 1
      },
      {
        "kind": "ClassDeclaration",
        "text": "class B { b }",
        "line": 13,
        "character": 1
      },
      {
        "kind": "FunctionDeclaration",
        "text": "function bar<T extends A | B>(x: T) {\r\n    var y: A | B = x;  // Ok\r\n}",
        "line": 15,
        "character": 1
      },
      {
        "kind": "ExpressionStatement",
        "text": "bar(new A);",
        "line": 19,
        "character": 1
      },
      {
        "kind": "ExpressionStatement",
        "text": "bar(new B);",
        "line": 20,
        "character": 1
      }
    ],
    "pathToPosition": [
      {
        "kind": "SourceFile",
        "text": "const enum E { A, B, C }\r\n\r\nfunction foo<T extends number>(x: T) {\r\n    var y: number = x;  // Ok\r\n}\r\n\r\nfoo(5);\r\nfoo(E.A",
        "line": 3,
        "character": 1
      },
      {
        "kind": "EnumDeclaration",
        "text": "const enum E { A, B, C }",
        "line": 3,
        "character": 1
      }
    ]
  }
}
```

Stack trace:

```text
error: [UnsupportedSyntax] const declarations require an initializer at 45..49
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
