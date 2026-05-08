---
id: 658
title: "Implement Argumentsreferenceinobjectliteral"
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

Triage argumentsReferenceInObjectLiteral across 1 failing reference test cases and split this bucket into implementation-ready child issues.

## Problem

Reference test results show 1 cases fail in directory `argumentsReferenceInObjectLiteral` with diagnostics: arguments-object. The compiler cannot handle these syntax/semantics, preventing compilation of code in this category.

Problem: argumentsReferenceInObjectLiteral has 1 reference failures and needs smart-triage evidence before implementation starts.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/argumentsReferenceInObjectLiteral_Js.ts
```

Coverage window:

```sh
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/argumentsReferenceInObjectLiteral_Js.ts --detail
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
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/argumentsReferenceInObjectLiteral_Js.ts --detail
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/argumentsReferenceInObjectLiteral_Js.ts
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

- `reference/typescript/tests/cases/compiler/argumentsReferenceInObjectLiteral_Js.ts`

## Duplicate detection

- none found by path/title/feature scan

## Smart triage

### Smart triage: Triage arguments object: argumentsReferenceInObjectLiteral Js

- Issue class: `triage-needed`
- Feature label: `arguments-object`
- Diagnostic: `UnsupportedSyntax` / `parser-or-frontend-unsupported`
- Path: `reference/typescript/tests/cases/compiler/argumentsReferenceInObjectLiteral_Js.ts`

Reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/argumentsReferenceInObjectLiteral_Js.ts
```

Source overview:

```json
{
  "suite": "tsc",
  "bytes": 465,
  "lines": 34,
  "extension": ".ts",
  "first_code_line": "const a = () => {"
}
```

Failure location:

```json
{
  "code": "UnsupportedSyntax",
  "message": "expected Semicolon, got Some(Return) at 318..324",
  "span_start": 318,
  "span_end": 324,
  "line": 23,
  "column": 6,
  "feature_label": "arguments-object",
  "error_type": "parser-or-frontend-unsupported"
}
```

Source context:

```text
20 |     return c;
21 | };
22 | 
23 | const c = () => {
24 |     return {
25 |         arguments,
26 |     };
```

Visible symbols before failure:

```json
[
  {
    "kind": "binding",
    "name": "a",
    "line": 10,
    "column": 1,
    "initializer": "() => {"
  },
  {
    "kind": "binding",
    "name": "b",
    "line": 16,
    "column": 1,
    "initializer": "() => {"
  },
  {
    "kind": "binding",
    "name": "c",
    "line": 17,
    "column": 5,
    "initializer": "{"
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
        kind: Const,
        span: Span {
            start: 176,
            end: 181,
        },
    },
    SpannedToken {
        kind: Ident(
            "a",
        ),
        span: Span {
            start: 182,
            end: 183,
        },
    },
    SpannedToken {
        kind: Equal,
        span: Span {
            start: 184,
            end: 185,
        },
    },
    SpannedToken {
        kind: LeftParen,
        span: Span {
            start: 186,
            end: 187,
        },
    },
    SpannedToken {
        kind: RightParen,
        span: Span {
            start: 187,
            end: 188,
        },
    },
    SpannedToken {
        kind: Arrow,
        span: Span {
            start: 189,
            end: 191,
        },
    },
    SpannedToken {
        kind: LeftBrace,
        span: Span {
            start: 192,
            end: 193,
        },
    },
    SpannedToken {
        kind: Return,
        span: Span {
            start: 199,
            end: 205,
        },
    },
    SpannedToken {
        kind: LeftBrace,
        span: Span {
            start: 206,
            end: 207,
        },
    },
    SpannedToken {
        kind: Ident(
            "arguments",
        ),
        span: Span {
            start: 217,
            end: 226,
        },
    },
    SpannedToken {
        kind: Colon,
        span: Span {
            start: 226,
            end: 227,
        },
    },
    SpannedToken {
        kind: LeftBracket,
        span: Span {
            start: 228,
            end: 229,
        },
    },
    SpannedToken {
        kind: RightBracket,
        span: Span {
            start: 229,
            end: 230,
        },
    },
    SpannedToken {
        kind: Comma,
        span: Span {
            start: 230,
            end: 231,
        },
    },
    SpannedToken {
        kind: RightBrace,
        span: Span {
            start: 237,
            end: 238,
        },
    },
    SpannedToken {
        kind: Semicolon,
        span: Span {
            start: 238,
            end: 239,
        },
    },
    SpannedToken {
        kind: RightBrace,
        span: Span {
            start: 241,
            end: 242,
        },
    },
    SpannedToken {
        kind: Semicolon,
        span: Span {
            start: 242,
            end: 243,
        },
    },
    SpannedToken {
        kind: Const,
        span: Span {
            start: 247,
            end: 252,
        },
    },
    SpannedToken {
        kind: Ident(
            "b",
        ),
        span: Span {
            start: 253,
            end: 254,
        },
    },
    SpannedToken {
        kind: Equal,
        span: Span {
            start: 255,
            end: 256,
        },
    },
    SpannedToken {
        kind: LeftParen,
        span: Span {
            start: 257,
            end: 258,
        },
    },
    SpannedToken {
        kind: RightParen,
        span: Span {
            start: 258,
            end: 259,
        },
    },
    SpannedToken {
        kind: Arrow,
        span: Span {
            start: 260,
            end: 262,
        },
    },
    SpannedToken {
        kind: LeftBrace,
        span: Span {
            start: 263,
            end: 264,
        },
    },
    SpannedToken {
        kind: Const,
        span: Span {
            start: 270,
            end: 275,
        },
    },
    SpannedToken {
        kind: Ident(
            "c",
        ),
        span:
```

#### ast

- ok: `False`
- truncated: `False`

```text
error: [UnsupportedSyntax] expected Semicolon, got Some(Return) at 318..324
```

#### resolved

- ok: `False`
- truncated: `False`

```text
error: [UnsupportedSyntax] expected Semicolon, got Some(Return) at 318..324
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
        "code": 18004,
        "category": "Error",
        "message": "No value exists in scope for the shorthand property 'arguments'. Either declare one or provide an initializer.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/argumentsReferenceInObjectLiteral_Js.ts",
        "start": 376,
        "length": 9,
        "line": 25,
        "character": 9
      },
      {
        "code": 1100,
        "category": "Error",
        "message": "Invalid use of 'arguments' in strict mode.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/argumentsReferenceInObjectLiteral_Js.ts",
        "start": 430,
        "length": 9,
        "line": 30,
        "character": 11
      }
    ],
    "hints": [
      {
        "kind": "binding",
        "typeText": "() => { arguments: never[]; }",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/argumentsReferenceInObjectLiteral_Js.ts",
        "start": 182,
        "length": 1,
        "line": 10,
        "character": 7,
        "name": "a"
      },
      {
        "kind": "binding",
        "typeText": "() => { arguments: never[]; }",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/argumentsReferenceInObjectLiteral_Js.ts",
        "start": 253,
        "length": 1,
        "line": 16,
        "character": 7,
        "name": "b"
      },
      {
        "kind": "binding",
        "typeText": "{ arguments: never[]; }",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/argumentsReferenceInObjectLiteral_Js.ts",
        "start": 276,
        "length": 1,
        "line": 17,
        "character": 11,
        "name": "c"
      },
      {
        "kind": "binding",
        "typeText": "() => { arguments: any; }",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/argumentsReferenceInObjectLiteral_Js.ts",
        "start": 341,
        "length": 1,
        "line": 23,
        "character": 7,
        "name": "c"
      },
      {
        "kind": "binding",
        "typeText": "() => { arguments: undefined; }",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/argumentsReferenceInObjectLiteral_Js.ts",
        "start": 407,
        "length": 1,
        "line": 29,
        "character": 7,
        "name": "d"
      },
      {
        "kind": "binding",
        "typeText": "undefined",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/argumentsReferenceInObjectLiteral_Js.ts",
        "start": 430,
        "length": 9,
        "line": 30,
        "character": 11,
        "name": "arguments"
      }
    ],
    "typescriptVersion": "6.0.3"
  },
  "ast": {
    "topLevel": [
      {
        "kind": "FirstStatement",
        "text": "const a = () => {\r\n    return {\r\n        arguments: [],\r\n    };\r\n};",
        "line": 10,
        "character": 1
      },
      {
        "kind": "FirstStatement",
        "text": "const b = () => {\r\n    const c = {\r\n        arguments: [],\r\n    }\r\n    return c;\r\n};",
        "line": 16,
        "character": 1
      },
      {
        "kind": "FirstStatement",
        "text": "const c = () => {\r\n    return {\r\n        arguments,\r\n    };\r\n}",
        "line": 23,
        "character": 1
      },
      {
        "kind": "FirstStatement",
        "text": "const d = () => {\r\n    const arguments = undefined;\r\n    return {\r\n        arguments,\r\n    };\r\n}",
        "line": 29,
        "character": 1
      }
    ],
    "pathToPosition": [
      {
        "kind": "SourceFile",
        "text": "const a = () => {\r\n    return {\r\n        arguments: [],\r\n    };\r\n};\r\n\r\nconst b = () => {\r\n    const c = {\r\n        argum",
        "line": 10,
        "character": 1
      },
      {
        "kind": "FirstStatement",
        "text": "const b = () => {\r\n    const c = {\r\n        arguments: [],\r\n    }\r\n    return c;\r\n};",
        "line": 16,
        "character": 1
      },
      {
        "kind": "VariableDeclarationList",
        "text": "const b = () => {\r\n    const c = {\r\n        arguments: [],\r\n    }\r\n    return c;\r\n}",
        "line": 16,
        "character": 1
      },
      {
        "kind": "VariableDeclaration",
        "text": "b = () => {\r\n    const c = {\r\n        arguments: [],\r\n    }\r\n    return c;\r\n}",
        "line": 16,
        "character": 7
      },
      {
        "kind": "ArrowFunction",
        "text": "() => {\r\n    const c = {\r\n        arguments: [],\r\n    }\r\n    return c;\r\n}",
        "line": 16,
        "character": 11
      },
      {
        "kind": "Block",
        "text": "{\r\n    const c = {\r\n        arguments: [],\r\n    }\r\n    return c;\r\n}",
        "line": 16,
        "character": 17
      },
      {
        "kind": "ReturnStatement",
        "text": "return c;",
        "line": 20,
        "character": 5
      }
    ]
  }
}
```

Stack trace:

```text
error: [UnsupportedSyntax] expected Semicolon, got Some(Return) at 318..324
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
