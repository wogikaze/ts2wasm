---
id: 660
title: "Implement Argumentsusedinobjectliteralproperty"
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

Triage argumentsUsedInObjectLiteralProperty across 1 failing reference test cases and split this bucket into implementation-ready child issues.

## Problem

Reference test results show 1 cases fail in directory `argumentsUsedInObjectLiteralProperty` with diagnostics: arguments-object. The compiler cannot handle these syntax/semantics, preventing compilation of code in this category.

Problem: argumentsUsedInObjectLiteralProperty has 1 reference failures and needs smart-triage evidence before implementation starts.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/argumentsUsedInObjectLiteralProperty.ts
```

Coverage window:

```sh
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/argumentsUsedInObjectLiteralProperty.ts --detail
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
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/argumentsUsedInObjectLiteralProperty.ts --detail
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/argumentsUsedInObjectLiteralProperty.ts
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

- `reference/typescript/tests/cases/compiler/argumentsUsedInObjectLiteralProperty.ts`

## Duplicate detection

- none found by path/title/feature scan

## Smart triage

### Smart triage: Triage arguments object: argumentsUsedInObjectLiteralProperty

- Issue class: `triage-needed`
- Feature label: `arguments-object`
- Diagnostic: `UnsupportedSyntax` / `parser-or-frontend-unsupported`
- Path: `reference/typescript/tests/cases/compiler/argumentsUsedInObjectLiteralProperty.ts`

Reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/argumentsUsedInObjectLiteralProperty.ts
```

Source overview:

```json
{
  "suite": "tsc",
  "bytes": 195,
  "lines": 8,
  "extension": ".ts",
  "first_code_line": "class A {"
}
```

Failure location:

```json
{
  "code": "UnsupportedSyntax",
  "message": "expected LeftParen, got Some(Static) at 42..48",
  "span_start": 42,
  "span_end": 48,
  "line": 3,
  "column": 14,
  "feature_label": "arguments-object",
  "error_type": "parser-or-frontend-unsupported"
}
```

Source context:

```text
1 | // @target: es2015
2 | class A {
3 |     public static createSelectableViewModel(initialState?: any, selectedValue?: any) {
4 |         return {
5 |             selectedValue: arguments.length
6 |         };
```

Visible symbols before failure:

```json
[
  {
    "kind": "class",
    "name": "A",
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
    "path": "issues/done/311-fix-test262-arguments-object-index-assignment.md",
    "title": "Fix test262 arguments object index assignment semantics",
    "reason": "same feature label, title overlap"
  },
  {
    "state": "open",
    "path": "issues/open/646-implement-arguments.md",
    "title": "Implement Arguments",
    "reason": "same feature label, title overlap"
  },
  {
    "state": "open",
    "path": "issues/open/647-implement-argumentsAsPropertyName-arguments-object.md",
    "title": "Implement Argumentsaspropertyname Arguments Object",
    "reason": "same feature label, title overlap"
  },
  {
    "state": "open",
    "path": "issues/open/653-implement-argumentsReferenceInConstructor-arguments-object.md",
    "title": "Implement Argumentsreferenceinconstructor Arguments Object",
    "reason": "same feature label, title overlap"
  },
  {
    "state": "open",
    "path": "issues/open/656-implement-argumentsReferenceInMethod-arguments-object.md",
    "title": "Implement Argumentsreferenceinmethod Arguments Object",
    "reason": "same feature label, title overlap"
  },
  {
    "state": "done",
    "path": "issues/done/291-provide-object-global-binding-for-test262.md",
    "title": "Provide Object global binding for test262 cases",
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
            "A",
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
            "public",
        ),
        span: Span {
            start: 35,
            end: 41,
        },
    },
    SpannedToken {
        kind: Static,
        span: Span {
            start: 42,
            end: 48,
        },
    },
    SpannedToken {
        kind: Ident(
            "createSelectableViewModel",
        ),
        span: Span {
            start: 49,
            end: 74,
        },
    },
    SpannedToken {
        kind: LeftParen,
        span: Span {
            start: 74,
            end: 75,
        },
    },
    SpannedToken {
        kind: Ident(
            "initialState",
        ),
        span: Span {
            start: 75,
            end: 87,
        },
    },
    SpannedToken {
        kind: Question,
        span: Span {
            start: 87,
            end: 88,
        },
    },
    SpannedToken {
        kind: Colon,
        span: Span {
            start: 88,
            end: 89,
        },
    },
    SpannedToken {
        kind: Ident(
            "any",
        ),
        span: Span {
            start: 90,
            end: 93,
        },
    },
    SpannedToken {
        kind: Comma,
        span: Span {
            start: 93,
            end: 94,
        },
    },
    SpannedToken {
        kind: Ident(
            "selectedValue",
        ),
        span: Span {
            start: 95,
            end: 108,
        },
    },
    SpannedToken {
        kind: Question,
        span: Span {
            start: 108,
            end: 109,
        },
    },
    SpannedToken {
        kind: Colon,
        span: Span {
            start: 109,
            end: 110,
        },
    },
    SpannedToken {
        kind: Ident(
            "any",
        ),
        span: Span {
            start: 111,
            end: 114,
        },
    },
    SpannedToken {
        kind: RightParen,
        span: Span {
            start: 114,
            end: 115,
        },
    },
    SpannedToken {
        kind: LeftBrace,
        span: Span {
            start: 116,
            end: 117,
        },
    },
    SpannedToken {
        kind: Return,
        span: Span {
            start: 127,
            end: 133,
        },
    },
    SpannedToken {
        kind: LeftBrace,
        span: Span {
            start: 134,
            end: 135,
        },
    },
    SpannedToken {
        kind: Ident(
            "selectedValue",
        ),
        span: Span {
            start: 149,
            end: 162,
        },
    },
    SpannedToken {
        kind: Colon,
        span: Span {
            start: 162,
            end: 163,
        },
    },
    SpannedToken {
        kind: Ident(
            "arguments",
        ),
        span: Span {
            start: 164,
            end: 173,
        },
    },
    SpannedToken {
        kind: Dot,
        span: Span {
            start: 173,
            end: 174,
        },
    },
    SpannedToken {
        kind: Ident(
            "length",
        ),
        span: Span {
            start: 174,
            end: 180,
        },
```

#### ast

- ok: `False`
- truncated: `False`

```text
error: [UnsupportedSyntax] expected LeftParen, got Some(Static) at 42..48
```

#### resolved

- ok: `False`
- truncated: `False`

```text
error: [UnsupportedSyntax] expected LeftParen, got Some(Static) at 42..48
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
        "kind": "parameter",
        "typeText": "any",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/argumentsUsedInObjectLiteralProperty.ts",
        "start": 75,
        "length": 12,
        "line": 3,
        "character": 45,
        "name": "initialState"
      },
      {
        "kind": "parameter",
        "typeText": "any",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/argumentsUsedInObjectLiteralProperty.ts",
        "start": 95,
        "length": 13,
        "line": 3,
        "character": 65,
        "name": "selectedValue"
      }
    ],
    "typescriptVersion": "6.0.3"
  },
  "ast": {
    "topLevel": [
      {
        "kind": "ClassDeclaration",
        "text": "class A {\r\n    public static createSelectableViewModel(initialState?: any, selectedValue?: any) {\r\n        return {\r\n   ",
        "line": 2,
        "character": 1
      }
    ],
    "pathToPosition": [
      {
        "kind": "SourceFile",
        "text": "class A {\r\n    public static createSelectableViewModel(initialState?: any, selectedValue?: any) {\r\n        return {\r\n   ",
        "line": 2,
        "character": 1
      },
      {
        "kind": "ClassDeclaration",
        "text": "class A {\r\n    public static createSelectableViewModel(initialState?: any, selectedValue?: any) {\r\n        return {\r\n   ",
        "line": 2,
        "character": 1
      },
      {
        "kind": "MethodDeclaration",
        "text": "public static createSelectableViewModel(initialState?: any, selectedValue?: any) {\r\n        return {\r\n            select",
        "line": 3,
        "character": 5
      },
      {
        "kind": "StaticKeyword",
        "text": "static",
        "line": 3,
        "character": 12
      }
    ]
  }
}
```

Stack trace:

```text
error: [UnsupportedSyntax] expected LeftParen, got Some(Static) at 42..48
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
