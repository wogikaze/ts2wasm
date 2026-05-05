---
id: 767
title: "Implement Augmentedtypesenum Parser Syntax"
type: spike
area: frontend/syntax
class: blocked
priority: P1
depends_on: [5000]
blocks: []
created: 2026-05-01
updated: 2026-05-01
---

## Summary

Triage augmentedTypesEnum-parser-syntax across 2 failing reference test cases and split this bucket into implementation-ready child issues.

## Problem

Reference test results show 2 cases fail in directory `augmentedTypesEnum-parser-syntax` with diagnostics: parser-syntax. The compiler cannot handle these syntax/semantics, preventing compilation of code in this category.

Problem: augmentedTypesEnum-parser-syntax has 2 reference failures and needs smart-triage evidence before implementation starts.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/augmentedTypesEnum.ts
```

Coverage window:

```sh
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/augmentedTypesEnum.ts --detail
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
mise run reference-coverage -- tsc --limit 4
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/augmentedTypesEnum.ts --detail
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/augmentedTypesEnum.ts
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

- `reference/typescript/tests/cases/compiler/augmentedTypesEnum.ts`
- `reference/typescript/tests/cases/compiler/augmentedTypesEnum2.ts`

## Duplicate detection

- `issues/open/442-implement-parser-syntax.md` - Implement parser syntax extensions (same feature label, title overlap)
- `issues/done/464-implement-FunctionDeclaration-parser-syntax.md` - Implement Functiondeclaration Parser Syntax (same feature label, title overlap)
- `issues/open/059-implement-parser-syntax-extensions.md` - Implement parser syntax extensions for TypeScript and advanced JS (same feature label, title overlap)
- `issues/done/065-implement-parser-syntax.md` - Implement parser syntax extensions (same feature label, title overlap)
- `issues/done/065a-merge-duplicate-parser-syntax-issue-into-059.md` - Merge duplicate parser syntax issue into 059 (same feature label, title overlap)
- `issues/done/200-implement-parser-syntax.md` - Implement parser syntax extensions (same feature label, title overlap)
- `issues/done/243-implement-numeric-literal-separator-parser.md` - Implement numeric literal separator parser support (same feature label, title overlap)
- `issues/done/244-implement-bigint-literal-parser-classification.md` - Implement BigInt literal parser classification (same feature label, title overlap)
- `issues/done/246-implement-optional-chaining-parser-support.md` - Implement optional chaining parser support (same feature label, title overlap)
- `issues/done/247-implement-destructuring-binding-pattern-parser.md` - Implement destructuring binding pattern parser support (same feature label, title overlap)

## Smart triage

### Smart triage: Triage parser syntax: augmentedTypesEnum

- Issue class: `triage-needed`
- Feature label: `parser-syntax`
- Diagnostic: `UnsupportedTypeScriptSyntax` / `unsupported-feature-boundary`
- Path: `reference/typescript/tests/cases/compiler/augmentedTypesEnum.ts`

Reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/augmentedTypesEnum.ts
```

Source overview:

```json
{
  "suite": "tsc",
  "bytes": 736,
  "lines": 36,
  "extension": ".ts",
  "first_code_line": "enum e1111 { One } // error"
}
```

Failure location:

```json
{
  "code": "UnsupportedTypeScriptSyntax",
  "message": "TypeScript enum declarations require an explicit frontend transform before runtime lowering at 38..42",
  "span_start": 38,
  "span_end": 42,
  "line": 3,
  "column": 3,
  "feature_label": "parser-syntax",
  "error_type": "unsupported-feature-boundary"
}
```

Source context:

```text
1 | // @target: es2015
2 | // enum then var
3 | enum e1111 { One } // error
4 | var e1111 = 1; // error
5 | 
6 | // enum then function
```

Visible symbols before failure:

```json
[
  {
    "kind": "binding",
    "name": "en",
    "line": 2,
    "column": 14
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
            "enum",
        ),
        span: Span {
            start: 38,
            end: 42,
        },
    },
    SpannedToken {
        kind: Ident(
            "e1111",
        ),
        span: Span {
            start: 43,
            end: 48,
        },
    },
    SpannedToken {
        kind: LeftBrace,
        span: Span {
            start: 49,
            end: 50,
        },
    },
    SpannedToken {
        kind: Ident(
            "One",
        ),
        span: Span {
            start: 51,
            end: 54,
        },
    },
    SpannedToken {
        kind: RightBrace,
        span: Span {
            start: 55,
            end: 56,
        },
    },
    SpannedToken {
        kind: Var,
        span: Span {
            start: 67,
            end: 70,
        },
    },
    SpannedToken {
        kind: Ident(
            "e1111",
        ),
        span: Span {
            start: 71,
            end: 76,
        },
    },
    SpannedToken {
        kind: Equal,
        span: Span {
            start: 77,
            end: 78,
        },
    },
    SpannedToken {
        kind: Number(
            1,
        ),
        span: Span {
            start: 79,
            end: 80,
        },
    },
    SpannedToken {
        kind: Semicolon,
        span: Span {
            start: 80,
            end: 81,
        },
    },
    SpannedToken {
        kind: Ident(
            "enum",
        ),
        span: Span {
            start: 117,
            end: 121,
        },
    },
    SpannedToken {
        kind: Ident(
            "e2",
        ),
        span: Span {
            start: 122,
            end: 124,
        },
    },
    SpannedToken {
        kind: LeftBrace,
        span: Span {
            start: 125,
            end: 126,
        },
    },
    SpannedToken {
        kind: Ident(
            "One",
        ),
        span: Span {
            start: 127,
            end: 130,
        },
    },
    SpannedToken {
        kind: RightBrace,
        span: Span {
            start: 131,
            end: 132,
        },
    },
    SpannedToken {
        kind: Function,
        span: Span {
            start: 143,
            end: 151,
        },
    },
    SpannedToken {
        kind: Ident(
            "e2",
        ),
        span: Span {
            start: 152,
            end: 154,
        },
    },
    SpannedToken {
        kind: LeftParen,
        span: Span {
            start: 154,
            end: 155,
        },
    },
    SpannedToken {
        kind: RightParen,
        span: Span {
            start: 155,
            end: 156,
        },
    },
    SpannedToken {
        kind: LeftBrace,
        span: Span {
            start: 157,
            end: 158,
        },
    },
    SpannedToken {
        kind: RightBrace,
        span: Span {
            start: 159,
            end: 160,
        },
    },
    SpannedToken {
        kind: Ident(
            "enum",
        ),
        span: Span {
            start: 173,
            end: 177,
        },
    },
    SpannedToken {
        kind: Ident(
            "e3",
        ),
        span: Span {
            start: 178,
            end: 180,
        },
    },
    SpannedToken {
        kind: LeftBrace,
        span: Span {
            start: 181,
            end: 182,
        },
    },
    SpannedToken {
        kind: Ident(
            "One",
        ),
        span: Span {
            start: 183,
            en
```

#### ast

- ok: `False`
- truncated: `False`

```text
error: [UnsupportedTypeScriptSyntax] TypeScript enum declarations require an explicit frontend transform before runtime lowering at 38..42
```

#### resolved

- ok: `False`
- truncated: `False`

```text
error: [UnsupportedTypeScriptSyntax] TypeScript enum declarations require an explicit frontend transform before runtime lowering at 38..42
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
        "code": 2567,
        "category": "Error",
        "message": "Enum declarations can only merge with namespace or other enum declarations.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/augmentedTypesEnum.ts",
        "start": 43,
        "length": 5,
        "line": 3,
        "character": 6
      },
      {
        "code": 2567,
        "category": "Error",
        "message": "Enum declarations can only merge with namespace or other enum declarations.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/augmentedTypesEnum.ts",
        "start": 71,
        "length": 5,
        "line": 4,
        "character": 5
      },
      {
        "code": 2567,
        "category": "Error",
        "message": "Enum declarations can only merge with namespace or other enum declarations.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/augmentedTypesEnum.ts",
        "start": 122,
        "length": 2,
        "line": 7,
        "character": 6
      },
      {
        "code": 2567,
        "category": "Error",
        "message": "Enum declarations can only merge with namespace or other enum declarations.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/augmentedTypesEnum.ts",
        "start": 152,
        "length": 2,
        "line": 8,
        "character": 10
      },
      {
        "code": 2567,
        "category": "Error",
        "message": "Enum declarations can only merge with namespace or other enum declarations.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/augmentedTypesEnum.ts",
        "start": 178,
        "length": 2,
        "line": 10,
        "character": 6
      },
      {
        "code": 2567,
        "category": "Error",
        "message": "Enum declarations can only merge with namespace or other enum declarations.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/augmentedTypesEnum.ts",
        "start": 203,
        "length": 2,
        "line": 11,
        "character": 5
      },
      {
        "code": 2567,
        "category": "Error",
        "message": "Enum declarations can only merge with namespace or other enum declarations.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/augmentedTypesEnum.ts",
        "start": 255,
        "length": 2,
        "line": 14,
        "character": 6
      },
      {
        "code": 2567,
        "category": "Error",
        "message": "Enum declarations can only merge with namespace or other enum declarations.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/augmentedTypesEnum.ts",
        "start": 282,
        "length": 2,
        "line": 15,
        "character": 7
      },
      {
        "code": 2432,
        "category": "Error",
        "message": "In an enum with multiple declarations, only one declaration can omit an initializer for its first enum element.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/augmentedTypesEnum.ts",
        "start": 364,
        "length": 3,
        "line": 19,
        "character": 11
      },
      {
        "code": 2300,
        "category": "Error",
        "message": "Duplicate identifier 'One'.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/augmentedTypesEnum.ts",
        "start": 393,
        "length": 3,
        "line": 21,
        "character": 12
      },
      {
        "code": 2300,
        "category": "Error",
        "message": "Duplicate identifier 'One'.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/augmentedTypesEnum.ts",
        "start": 420,
        "length": 3,
        "line": 22,
        "character": 12
      },
      {
        "code": 2432,
        "category": "Error",
        "message": "In an enum with multiple declarations, only one declaration can omit an initializer for its first enum element.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/augmentedTypesEnum.ts",
        "start": 420,
        "length": 3,
        "line": 22,
        "character": 12
      }
    ],
    "hints": [
      {
        "kind": "binding",
        "typeText": "number",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/augmentedTypesEnum.ts",
        "start": 71,
        "length": 5,
        "line": 4,
        "character": 5,
        "name": "e1111"
      },
      {
        "kind": "function",
        "typeText": "void",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/augmentedTypesEnum.ts",
        "start": 152,
        "length": 2,
        "line": 8,
        "character": 10,
        "name": "e2"
      },
      {
        "kind": "binding",
        "typeText": "() => void",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/augmentedTypesEnum.ts",
        "start": 203,
        "length": 2,
        "line": 11,
        "character": 5,
        "name": "e3"
      },
      {
        "kind": "binding",
        "typeText": "number",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/augmentedTypesEnum.ts",
        "start": 550,
        "length": 1,
        "line": 29,
        "character": 21,
        "name": "y"
      },
      {
        "kind": "binding",
        "typeText": "number",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/augmentedTypesEnum.ts",
        "start": 626,
        "length": 1,
        "line": 32,
        "character": 28,
        "name": "y"
      }
    ],
    "typescriptVersion": "6.0.3"
  },
  "ast": {
    "topLevel": [
      {
        "kind": "EnumDeclaration",
        "text": "enum e1111 { One }",
        "line": 3,
        "character": 1
      },
      {
        "kind": "FirstStatement",
        "text": "var e1111 = 1;",
        "line": 4,
        "character": 1
      },
      {
        "kind": "EnumDeclaration",
        "text": "enum e2 { One }",
        "line": 7,
        "character": 1
      },
      {
        "kind": "FunctionDeclaration",
        "text": "function e2() { }",
        "line": 8,
        "character": 1
      },
      {
        "kind": "EnumDeclaration",
        "text": "enum e3 { One }",
        "line": 10,
        "character": 1
      },
      {
        "kind": "FirstStatement",
        "text": "var e3 = () => { }",
        "line": 11,
        "character": 1
      },
      {
        "kind": "EnumDeclaration",
        "text": "enum e4 { One }",
        "line": 14,
        "character": 1
      },
      {
        "kind": "ClassDeclaration",
        "text": "class e4 { public foo() { } }",
        "line": 15,
        "character": 1
      },
      {
        "kind": "EnumDeclaration",
        "text": "enum e5 { One }",
        "line": 18,
        "character": 1
      },
      {
        "kind": "EnumDeclaration",
        "text": "enum e5 { Two }",
        "line": 19,
        "character": 1
      },
      {
        "kind": "EnumDeclaration",
        "text": "enum e5a { One }",
        "line": 21,
        "character": 1
      },
      {
        "kind": "EnumDeclaration",
        "text": "enum e5a { One }",
        "line": 22,
        "character": 1
      },
      {
        "kind": "EnumDeclaration",
        "text": "enum e6 { One }",
        "line": 25,
        "character": 1
      },
      {
        "kind": "ModuleDeclaration",
        "text": "namespace e6 { }",
        "line": 26,
        "character": 1
      },
      {
        "kind": "EnumDeclaration",
        "text": "enum e6a { One }",
        "line": 28,
        "character": 1
      },
      {
        "kind": "ModuleDeclarat
```

Stack trace:

```text
error: [UnsupportedTypeScriptSyntax] TypeScript enum declarations require an explicit frontend transform before runtime lowering at 38..42
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
