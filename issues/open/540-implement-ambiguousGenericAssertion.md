---
id: 540
title: "Implement Ambiguousgenericassertion"
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

Triage ambiguousGenericAssertion across 1 failing reference test cases and split this bucket into implementation-ready child issues.

## Problem

Reference test results show 1 cases fail in directory `ambiguousGenericAssertion` with diagnostics: type-assertion. The compiler cannot handle these syntax/semantics, preventing compilation of code in this category.

Problem: ambiguousGenericAssertion has 1 reference failures and needs smart-triage evidence before implementation starts.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/ambiguousGenericAssertion1.ts
```

Coverage window:

```sh
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/ambiguousGenericAssertion1.ts --detail
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
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/ambiguousGenericAssertion1.ts --detail
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/ambiguousGenericAssertion1.ts
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

- `reference/typescript/tests/cases/compiler/ambiguousGenericAssertion1.ts`

## Duplicate detection

- `issues/open/167-implement-ambiguousGenericAssertion.md` - Implement Ambiguousgenericassertion (same reference path, same feature label, same group key, title overlap)

## Smart triage

### Smart triage: Triage type assertion: ambiguousGenericAssertion1

- Issue class: `triage-needed`
- Feature label: `type-assertion`
- Diagnostic: `UnsupportedSyntax` / `parser-or-frontend-unsupported`
- Path: `reference/typescript/tests/cases/compiler/ambiguousGenericAssertion1.ts`

Reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/ambiguousGenericAssertion1.ts
```

Source overview:

```json
{
  "suite": "tsc",
  "bytes": 221,
  "lines": 6,
  "extension": ".ts",
  "first_code_line": "function f<T>(x: T): T { return null; }"
}
```

Failure location:

```json
{
  "code": "UnsupportedSyntax",
  "message": "unsupported expression: Some(SpannedToken { kind: Less, span: Span { start: 88, end: 89 } }) at 89..90",
  "span_start": 89,
  "span_end": 90,
  "line": 4,
  "column": 13,
  "feature_label": "type-assertion",
  "error_type": "parser-or-frontend-unsupported"
}
```

Source context:

```text
1 | // @target: es2015
2 | // @strict: false
3 | function f<T>(x: T): T { return null; }
4 | var r = <T>(x: T) => x;
5 | var r2 = < <T>(x: T) => T>f; // valid
6 | var r3 = <<T>(x: T) => T>f; // ambiguous, appears to the parser as a << operation
```

Visible symbols before failure:

```json
[
  {
    "kind": "binding",
    "name": "r",
    "line": 4,
    "column": 1,
    "initializer": "<T>("
  }
]
```

Duplicate candidates:

```json
[
  {
    "state": "open",
    "path": "issues/open/167-implement-ambiguousGenericAssertion.md",
    "title": "Implement Ambiguousgenericassertion",
    "reason": "same reference path, same feature label"
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
        kind: Function,
        span: Span {
            start: 39,
            end: 47,
        },
    },
    SpannedToken {
        kind: Ident(
            "f",
        ),
        span: Span {
            start: 48,
            end: 49,
        },
    },
    SpannedToken {
        kind: Less,
        span: Span {
            start: 49,
            end: 50,
        },
    },
    SpannedToken {
        kind: Ident(
            "T",
        ),
        span: Span {
            start: 50,
            end: 51,
        },
    },
    SpannedToken {
        kind: Greater,
        span: Span {
            start: 51,
            end: 52,
        },
    },
    SpannedToken {
        kind: LeftParen,
        span: Span {
            start: 52,
            end: 53,
        },
    },
    SpannedToken {
        kind: Ident(
            "x",
        ),
        span: Span {
            start: 53,
            end: 54,
        },
    },
    SpannedToken {
        kind: Colon,
        span: Span {
            start: 54,
            end: 55,
        },
    },
    SpannedToken {
        kind: Ident(
            "T",
        ),
        span: Span {
            start: 56,
            end: 57,
        },
    },
    SpannedToken {
        kind: RightParen,
        span: Span {
            start: 57,
            end: 58,
        },
    },
    SpannedToken {
        kind: Colon,
        span: Span {
            start: 58,
            end: 59,
        },
    },
    SpannedToken {
        kind: Ident(
            "T",
        ),
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
        kind: Return,
        span: Span {
            start: 64,
            end: 70,
        },
    },
    SpannedToken {
        kind: Null,
        span: Span {
            start: 71,
            end: 75,
        },
    },
    SpannedToken {
        kind: Semicolon,
        span: Span {
            start: 75,
            end: 76,
        },
    },
    SpannedToken {
        kind: RightBrace,
        span: Span {
            start: 77,
            end: 78,
        },
    },
    SpannedToken {
        kind: Var,
        span: Span {
            start: 80,
            end: 83,
        },
    },
    SpannedToken {
        kind: Ident(
            "r",
        ),
        span: Span {
            start: 84,
            end: 85,
        },
    },
    SpannedToken {
        kind: Equal,
        span: Span {
            start: 86,
            end: 87,
        },
    },
    SpannedToken {
        kind: Less,
        span: Span {
            start: 88,
            end: 89,
        },
    },
    SpannedToken {
        kind: Ident(
            "T",
        ),
        span: Span {
            start: 89,
            end: 90,
        },
    },
    SpannedToken {
        kind: Greater,
        span: Span {
            start: 90,
            end: 91,
        },
    },
    SpannedToken {
        kind: LeftParen,
        span: Span {
            start: 91,
            end: 92,
        },
    },
    SpannedToken {
        kind: Ident(
            "x",
        ),
        span: Span {
            start: 92,
            end: 93,
        },
    },
    SpannedToken {
        kind: Colon,
        span: Span {
            start: 93,
            end: 94,
        },
    },
    SpannedToken {
        kind: I
```

#### ast

- ok: `False`
- truncated: `False`

```text
error: [UnsupportedSyntax] unsupported expression: Some(SpannedToken { kind: Less, span: Span { start: 88, end: 89 } }) at 89..90
```

#### resolved

- ok: `False`
- truncated: `False`

```text
error: [UnsupportedSyntax] unsupported expression: Some(SpannedToken { kind: Less, span: Span { start: 88, end: 89 } }) at 89..90
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
        "code": 2322,
        "category": "Error",
        "message": "Type 'null' is not assignable to type 'T'.\n  'T' could be instantiated with an arbitrary type which could be unrelated to 'null'.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/ambiguousGenericAssertion1.ts",
        "start": 64,
        "length": 6,
        "line": 3,
        "character": 26
      },
      {
        "code": 1109,
        "category": "Error",
        "message": "Expression expected.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/ambiguousGenericAssertion1.ts",
        "start": 153,
        "length": 2,
        "line": 6,
        "character": 10
      },
      {
        "code": 2304,
        "category": "Error",
        "message": "Cannot find name 'x'.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/ambiguousGenericAssertion1.ts",
        "start": 158,
        "length": 1,
        "line": 6,
        "character": 15
      },
      {
        "code": 1005,
        "category": "Error",
        "message": "')' expected.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/ambiguousGenericAssertion1.ts",
        "start": 159,
        "length": 1,
        "line": 6,
        "character": 16
      },
      {
        "code": 1005,
        "category": "Error",
        "message": "',' expected.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/ambiguousGenericAssertion1.ts",
        "start": 162,
        "length": 1,
        "line": 6,
        "character": 19
      },
      {
        "code": 1005,
        "category": "Error",
        "message": "';' expected.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/ambiguousGenericAssertion1.ts",
        "start": 164,
        "length": 2,
        "line": 6,
        "character": 21
      }
    ],
    "hints": [
      {
        "kind": "function",
        "typeText": "T",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/ambiguousGenericAssertion1.ts",
        "start": 48,
        "length": 1,
        "line": 3,
        "character": 10,
        "name": "f"
      },
      {
        "kind": "parameter",
        "typeText": "T",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/ambiguousGenericAssertion1.ts",
        "start": 53,
        "length": 1,
        "line": 3,
        "character": 15,
        "name": "x"
      },
      {
        "kind": "binding",
        "typeText": "<T>(x: T) => T",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/ambiguousGenericAssertion1.ts",
        "start": 84,
        "length": 1,
        "line": 4,
        "character": 5,
        "name": "r"
      },
      {
        "kind": "parameter",
        "typeText": "T",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/ambiguousGenericAssertion1.ts",
        "start": 92,
        "length": 1,
        "line": 4,
        "character": 13,
        "name": "x"
      },
      {
        "kind": "binding",
        "typeText": "<T>(x: T) => T",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/ambiguousGenericAssertion1.ts",
        "start": 109,
        "length": 2,
        "line": 5,
        "character": 5,
        "name": "r2"
      },
      {
        "kind": "parameter",
        "typeText": "T",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/ambiguousGenericAssertion1.ts",
        "start": 120,
        "length": 1,
        "line": 5,
        "character": 16,
        "name": "x"
      },
      {
        "kind": "binding",
        "typeText": "boolean",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/ambiguousGenericAssertion1.ts",
        "start": 148,
        "length": 2,
        "line": 6,
        "character": 5,
        "name": "r3"
      },
      {
        "kind": "binding",
        "typeText": "any",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/ambiguousGenericAssertion1.ts",
        "start": 161,
        "length": 1,
        "line": 6,
        "character": 18,
        "name": "T"
      }
    ],
    "typescriptVersion": "6.0.3"
  },
  "ast": {
    "topLevel": [
      {
        "kind": "FunctionDeclaration",
        "text": "function f<T>(x: T): T { return null; }",
        "line": 3,
        "character": 1
      },
      {
        "kind": "FirstStatement",
        "text": "var r = <T>(x: T) => x;",
        "line": 4,
        "character": 1
      },
      {
        "kind": "FirstStatement",
        "text": "var r2 = < <T>(x: T) => T>f;",
        "line": 5,
        "character": 1
      },
      {
        "kind": "FirstStatement",
        "text": "var r3 = <<T>(x: T)",
        "line": 6,
        "character": 1
      },
      {
        "kind": "ExpressionStatement",
        "text": "T>f;",
        "line": 6,
        "character": 24
      }
    ],
    "pathToPosition": [
      {
        "kind": "SourceFile",
        "text": "function f<T>(x: T): T { return null; }\r\nvar r = <T>(x: T) => x;\r\nvar r2 = < <T>(x: T) => T>f; // valid\r\nvar r3 = <<T>(x",
        "line": 3,
        "character": 1
      },
      {
        "kind": "FirstStatement",
        "text": "var r = <T>(x: T) => x;",
        "line": 4,
        "character": 1
      },
      {
        "kind": "VariableDeclarationList",
        "text": "var r = <T>(x: T) => x",
        "line": 4,
        "character": 1
      },
      {
        "kind": "VariableDeclaration",
        "text": "r = <T>(x: T) => x",
        "line": 4,
        "character": 5
      },
      {
        "kind": "ArrowFunction",
        "text": "<T>(x: T) => x",
        "line": 4,
        "character": 9
      },
      {
        "kind": "TypeParameter",
        "text": "T",
        "line": 4,
        "character": 10
      },
      {
        "kind": "Identifier",
        "text": "T",
        "line": 4,
        "character": 10
      }
    ]
  }
}
```

Stack trace:

```text
error: [UnsupportedSyntax] unsupported expression: Some(SpannedToken { kind: Less, span: Span { start: 88, end: 89 } }) at 89..90
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
