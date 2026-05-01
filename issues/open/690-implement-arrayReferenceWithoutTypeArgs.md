---
id: 690
title: "Implement Arrayreferencewithouttypeargs"
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

Triage arrayReferenceWithoutTypeArgs across 1 failing reference test cases and split this bucket into implementation-ready child issues.

## Problem

Reference test results show 1 cases fail in directory `arrayReferenceWithoutTypeArgs` with diagnostics: arguments-object. The compiler cannot handle these syntax/semantics, preventing compilation of code in this category.

Problem: arrayReferenceWithoutTypeArgs has 1 reference failures and needs smart-triage evidence before implementation starts.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/arrayReferenceWithoutTypeArgs.ts
```

Coverage window:

```sh
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/arrayReferenceWithoutTypeArgs.ts --detail
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
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/arrayReferenceWithoutTypeArgs.ts --detail
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/arrayReferenceWithoutTypeArgs.ts
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

- `reference/typescript/tests/cases/compiler/arrayReferenceWithoutTypeArgs.ts`

## Duplicate detection

- none found by path/title/feature scan

## Smart triage

### Smart triage: Triage arguments object: arrayReferenceWithoutTypeArgs

- Issue class: `triage-needed`
- Feature label: `arguments-object`
- Diagnostic: `UnsupportedSyntax` / `parser-or-frontend-unsupported`
- Path: `reference/typescript/tests/cases/compiler/arrayReferenceWithoutTypeArgs.ts`

Reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/arrayReferenceWithoutTypeArgs.ts
```

Source overview:

```json
{
  "suite": "tsc",
  "bytes": 57,
  "lines": 4,
  "extension": ".ts",
  "first_code_line": "class X {"
}
```

Failure location:

```json
{
  "code": "UnsupportedSyntax",
  "message": "expected LeftParen, got Some(Ident(\"f\")) at 42..43",
  "span_start": 42,
  "span_end": 43,
  "line": 3,
  "column": 14,
  "feature_label": "arguments-object",
  "error_type": "parser-or-frontend-unsupported"
}
```

Source context:

```text
1 | // @target: es2015
2 | class X {
3 |     public f(a: Array) { }
4 | }
```

Visible symbols before failure:

```json
[
  {
    "kind": "class",
    "name": "X",
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
    "path": "issues/open/311-fix-test262-arguments-object-index-assignment.md",
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
            "X",
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
        kind: Ident(
            "f",
        ),
        span: Span {
            start: 42,
            end: 43,
        },
    },
    SpannedToken {
        kind: LeftParen,
        span: Span {
            start: 43,
            end: 44,
        },
    },
    SpannedToken {
        kind: Ident(
            "a",
        ),
        span: Span {
            start: 44,
            end: 45,
        },
    },
    SpannedToken {
        kind: Colon,
        span: Span {
            start: 45,
            end: 46,
        },
    },
    SpannedToken {
        kind: Ident(
            "Array",
        ),
        span: Span {
            start: 47,
            end: 52,
        },
    },
    SpannedToken {
        kind: RightParen,
        span: Span {
            start: 52,
            end: 53,
        },
    },
    SpannedToken {
        kind: LeftBrace,
        span: Span {
            start: 54,
            end: 55,
        },
    },
    SpannedToken {
        kind: RightBrace,
        span: Span {
            start: 56,
            end: 57,
        },
    },
    SpannedToken {
        kind: RightBrace,
        span: Span {
            start: 59,
            end: 60,
        },
    },
]
```

#### ast

- ok: `False`
- truncated: `False`

```text
error: [UnsupportedSyntax] expected LeftParen, got Some(Ident("f")) at 42..43
```

#### resolved

- ok: `False`
- truncated: `False`

```text
error: [UnsupportedSyntax] expected LeftParen, got Some(Ident("f")) at 42..43
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
        "code": 2314,
        "category": "Error",
        "message": "Generic type 'Array<T>' requires 1 type argument(s).",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/arrayReferenceWithoutTypeArgs.ts",
        "start": 47,
        "length": 5,
        "line": 3,
        "character": 17
      }
    ],
    "hints": [
      {
        "kind": "parameter",
        "typeText": "any",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/arrayReferenceWithoutTypeArgs.ts",
        "start": 44,
        "length": 1,
        "line": 3,
        "character": 14,
        "name": "a"
      }
    ],
    "typescriptVersion": "6.0.3"
  },
  "ast": {
    "topLevel": [
      {
        "kind": "ClassDeclaration",
        "text": "class X {\r\n    public f(a: Array) { }\r\n}",
        "line": 2,
        "character": 1
      }
    ],
    "pathToPosition": [
      {
        "kind": "SourceFile",
        "text": "class X {\r\n    public f(a: Array) { }\r\n}",
        "line": 2,
        "character": 1
      },
      {
        "kind": "ClassDeclaration",
        "text": "class X {\r\n    public f(a: Array) { }\r\n}",
        "line": 2,
        "character": 1
      },
      {
        "kind": "MethodDeclaration",
        "text": "public f(a: Array) { }",
        "line": 3,
        "character": 5
      },
      {
        "kind": "Identifier",
        "text": "f",
        "line": 3,
        "character": 12
      }
    ]
  }
}
```

Stack trace:

```text
error: [UnsupportedSyntax] expected LeftParen, got Some(Ident("f")) at 42..43
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
