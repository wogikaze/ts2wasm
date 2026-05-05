---
id: 652
title: "Implement Argumentspropertynameinjsmode"
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

Triage argumentsPropertyNameInJsMode across 1 failing reference test cases and split this bucket into implementation-ready child issues.

## Problem

Reference test results show 1 cases fail in directory `argumentsPropertyNameInJsMode` with diagnostics: arguments-object. The compiler cannot handle these syntax/semantics, preventing compilation of code in this category.

Problem: argumentsPropertyNameInJsMode has 1 reference failures and needs smart-triage evidence before implementation starts.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/argumentsPropertyNameInJsMode1.ts
```

Coverage window:

```sh
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/argumentsPropertyNameInJsMode1.ts --detail
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
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/argumentsPropertyNameInJsMode1.ts --detail
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/argumentsPropertyNameInJsMode1.ts
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

- `reference/typescript/tests/cases/compiler/argumentsPropertyNameInJsMode1.ts`

## Duplicate detection

- `issues/done/198-implement-argumentsPropertyNameInJsMode.md` - Implement Argumentspropertynameinjsmode (same reference path, same feature label, same group key, title overlap)
- `issues/done/412-implement-arguments-object.md` - Implement arguments-object support (same feature label, same group key, title overlap)

## Smart triage

### Smart triage: Triage arguments object: argumentsPropertyNameInJsMode1

- Issue class: `triage-needed`
- Feature label: `arguments-object`
- Diagnostic: `UnsupportedSyntax` / `parser-or-frontend-unsupported`
- Path: `reference/typescript/tests/cases/compiler/argumentsPropertyNameInJsMode1.ts`

Reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/argumentsPropertyNameInJsMode1.ts
```

Source overview:

```json
{
  "suite": "tsc",
  "bytes": 238,
  "lines": 17,
  "extension": ".ts",
  "first_code_line": "const foo = {"
}
```

Failure location:

```json
{
  "code": "UnsupportedSyntax",
  "message": "expected Semicolon, got Some(Function) at 173..181",
  "span_start": 173,
  "span_end": 181,
  "line": 13,
  "column": 1,
  "feature_label": "arguments-object",
  "error_type": "parser-or-frontend-unsupported"
}
```

Source context:

```text
10 |    f1: (params) => { }
11 | }
12 | 
13 | function f2(x) {
14 |   foo.f1({ x, arguments: [] });
15 | }
16 |
```

Visible symbols before failure:

```json
[
  {
    "kind": "binding",
    "name": "foo",
    "line": 9,
    "column": 1,
    "initializer": "{"
  }
]
```

Duplicate candidates:

```json
[
  {
    "state": "open",
    "path": "issues/done/198-implement-argumentsPropertyNameInJsMode.md",
    "title": "Implement Argumentspropertynameinjsmode",
    "reason": "same reference path, same feature label"
  },
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
            start: 133,
            end: 138,
        },
    },
    SpannedToken {
        kind: Ident(
            "foo",
        ),
        span: Span {
            start: 139,
            end: 142,
        },
    },
    SpannedToken {
        kind: Equal,
        span: Span {
            start: 143,
            end: 144,
        },
    },
    SpannedToken {
        kind: LeftBrace,
        span: Span {
            start: 145,
            end: 146,
        },
    },
    SpannedToken {
        kind: Ident(
            "f1",
        ),
        span: Span {
            start: 150,
            end: 152,
        },
    },
    SpannedToken {
        kind: Colon,
        span: Span {
            start: 152,
            end: 153,
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
        kind: Ident(
            "params",
        ),
        span: Span {
            start: 155,
            end: 161,
        },
    },
    SpannedToken {
        kind: RightParen,
        span: Span {
            start: 161,
            end: 162,
        },
    },
    SpannedToken {
        kind: Arrow,
        span: Span {
            start: 163,
            end: 165,
        },
    },
    SpannedToken {
        kind: LeftBrace,
        span: Span {
            start: 166,
            end: 167,
        },
    },
    SpannedToken {
        kind: RightBrace,
        span: Span {
            start: 168,
            end: 169,
        },
    },
    SpannedToken {
        kind: RightBrace,
        span: Span {
            start: 170,
            end: 171,
        },
    },
    SpannedToken {
        kind: Function,
        span: Span {
            start: 173,
            end: 181,
        },
    },
    SpannedToken {
        kind: Ident(
            "f2",
        ),
        span: Span {
            start: 182,
            end: 184,
        },
    },
    SpannedToken {
        kind: LeftParen,
        span: Span {
            start: 184,
            end: 185,
        },
    },
    SpannedToken {
        kind: Ident(
            "x",
        ),
        span: Span {
            start: 185,
            end: 186,
        },
    },
    SpannedToken {
        kind: RightParen,
        span: Span {
            start: 186,
            end: 187,
        },
    },
    SpannedToken {
        kind: LeftBrace,
        span: Span {
            start: 188,
            end: 189,
        },
    },
    SpannedToken {
        kind: Ident(
            "foo",
        ),
        span: Span {
            start: 192,
            end: 195,
        },
    },
    SpannedToken {
        kind: Dot,
        span: Span {
            start: 195,
            end: 196,
        },
    },
    SpannedToken {
        kind: Ident(
            "f1",
        ),
        span: Span {
            start: 196,
            end: 198,
        },
    },
    SpannedToken {
        kind: LeftParen,
        span: Span {
            start: 198,
            end: 199,
        },
    },
    SpannedToken {
        kind: LeftBrace,
        span: Span {
            start: 199,
            end: 200,
        },
    },
    SpannedToken {
        kind: Ident(
            "x",
        ),
        span: Span {
            start: 201,
            end: 202,
        },
    },
    SpannedToken {
        kind: Comma,
        span: Span {
            st
```

#### ast

- ok: `False`
- truncated: `False`

```text
error: [UnsupportedSyntax] expected Semicolon, got Some(Function) at 173..181
```

#### resolved

- ok: `False`
- truncated: `False`

```text
error: [UnsupportedSyntax] expected Semicolon, got Some(Function) at 173..181
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
        "code": 2554,
        "category": "Error",
        "message": "Expected 1 arguments, but got 3.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/argumentsPropertyNameInJsMode1.ts",
        "start": 231,
        "length": 4,
        "line": 17,
        "character": 7
      }
    ],
    "hints": [
      {
        "kind": "binding",
        "typeText": "{ f1: (params: any) => void; }",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/argumentsPropertyNameInJsMode1.ts",
        "start": 139,
        "length": 3,
        "line": 9,
        "character": 7,
        "name": "foo"
      },
      {
        "kind": "parameter",
        "typeText": "any",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/argumentsPropertyNameInJsMode1.ts",
        "start": 155,
        "length": 6,
        "line": 10,
        "character": 9,
        "name": "params"
      },
      {
        "kind": "function",
        "typeText": "void",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/argumentsPropertyNameInJsMode1.ts",
        "start": 182,
        "length": 2,
        "line": 13,
        "character": 10,
        "name": "f2"
      },
      {
        "kind": "parameter",
        "typeText": "any",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/argumentsPropertyNameInJsMode1.ts",
        "start": 185,
        "length": 1,
        "line": 13,
        "character": 13,
        "name": "x"
      }
    ],
    "typescriptVersion": "6.0.3"
  },
  "ast": {
    "topLevel": [
      {
        "kind": "FirstStatement",
        "text": "const foo = {\n   f1: (params) => { }\n}",
        "line": 9,
        "character": 1
      },
      {
        "kind": "FunctionDeclaration",
        "text": "function f2(x) {\n  foo.f1({ x, arguments: [] });\n}",
        "line": 13,
        "character": 1
      },
      {
        "kind": "ExpressionStatement",
        "text": "f2(1, 2, 3);",
        "line": 17,
        "character": 1
      }
    ],
    "pathToPosition": [
      {
        "kind": "SourceFile",
        "text": "const foo = {\n   f1: (params) => { }\n}\n\nfunction f2(x) {\n  foo.f1({ x, arguments: [] });\n}\n\nf2(1, 2, 3);\n",
        "line": 9,
        "character": 1
      },
      {
        "kind": "FunctionDeclaration",
        "text": "function f2(x) {\n  foo.f1({ x, arguments: [] });\n}",
        "line": 13,
        "character": 1
      }
    ]
  }
}
```

Stack trace:

```text
error: [UnsupportedSyntax] expected Semicolon, got Some(Function) at 173..181
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
