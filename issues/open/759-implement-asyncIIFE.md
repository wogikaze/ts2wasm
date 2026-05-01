---
id: 759
title: "Implement Asynciife"
type: spike
area: frontend/syntax
class: triage-needed
priority: P2
depends_on: []
blocks: []
created: 2026-05-01
updated: 2026-05-01
---

## Summary

Triage asyncIIFE across 1 failing reference test cases and split this bucket into implementation-ready child issues.

## Problem

Reference test results show 1 cases fail in directory `asyncIIFE` with diagnostics: unknown-unsupported. The compiler cannot handle these syntax/semantics, preventing compilation of code in this category.

Problem: asyncIIFE has 1 reference failures and needs smart-triage evidence before implementation starts.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/asyncIIFE.ts
```

Coverage window:

```sh
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/asyncIIFE.ts --detail
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
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/asyncIIFE.ts --detail
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/asyncIIFE.ts
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

- `reference/typescript/tests/cases/compiler/asyncIIFE.ts`

## Duplicate detection

- none found by path/title/feature scan

## Smart triage

### Smart triage: Triage unknown unsupported: asyncIIFE

- Issue class: `triage-needed`
- Feature label: `unknown-unsupported`
- Diagnostic: `UnsupportedSyntax` / `parser-or-frontend-unsupported`
- Path: `reference/typescript/tests/cases/compiler/asyncIIFE.ts`

Reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/asyncIIFE.ts
```

Source overview:

```json
{
  "suite": "tsc",
  "bytes": 124,
  "lines": 10,
  "extension": ".ts",
  "first_code_line": "function f1() {"
}
```

Failure location:

```json
{
  "code": "UnsupportedSyntax",
  "message": "unsupported expression: Some(SpannedToken { kind: Async, span: Span { start: 41, end: 46 } }) at 47..48",
  "span_start": 47,
  "span_end": 48,
  "line": 4,
  "column": 15,
  "feature_label": "unknown-unsupported",
  "error_type": "parser-or-frontend-unsupported"
}
```

Source context:

```text
1 | // @target: ES6
2 | 
3 | function f1() {
4 |     (async () => {
5 |         await 10
6 |         throw new Error();
7 |     })();
```

Visible symbols before failure:

```json
[
  {
    "kind": "function",
    "name": "f1",
    "line": 3,
    "column": 1,
    "params": ""
  }
]
```

Duplicate candidates:

```json
[
  {
    "state": "open",
    "path": "issues/open/694-implement-arrayToLocaleStringES-unknown-unsupported.md",
    "title": "Implement Arraytolocalestringes Unknown Unsupported",
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
        kind: Function,
        span: Span {
            start: 19,
            end: 27,
        },
    },
    SpannedToken {
        kind: Ident(
            "f1",
        ),
        span: Span {
            start: 28,
            end: 30,
        },
    },
    SpannedToken {
        kind: LeftParen,
        span: Span {
            start: 30,
            end: 31,
        },
    },
    SpannedToken {
        kind: RightParen,
        span: Span {
            start: 31,
            end: 32,
        },
    },
    SpannedToken {
        kind: LeftBrace,
        span: Span {
            start: 33,
            end: 34,
        },
    },
    SpannedToken {
        kind: LeftParen,
        span: Span {
            start: 40,
            end: 41,
        },
    },
    SpannedToken {
        kind: Async,
        span: Span {
            start: 41,
            end: 46,
        },
    },
    SpannedToken {
        kind: LeftParen,
        span: Span {
            start: 47,
            end: 48,
        },
    },
    SpannedToken {
        kind: RightParen,
        span: Span {
            start: 48,
            end: 49,
        },
    },
    SpannedToken {
        kind: Arrow,
        span: Span {
            start: 50,
            end: 52,
        },
    },
    SpannedToken {
        kind: LeftBrace,
        span: Span {
            start: 53,
            end: 54,
        },
    },
    SpannedToken {
        kind: Await,
        span: Span {
            start: 64,
            end: 69,
        },
    },
    SpannedToken {
        kind: Number(
            10,
        ),
        span: Span {
            start: 70,
            end: 72,
        },
    },
    SpannedToken {
        kind: Throw,
        span: Span {
            start: 82,
            end: 87,
        },
    },
    SpannedToken {
        kind: New,
        span: Span {
            start: 88,
            end: 91,
        },
    },
    SpannedToken {
        kind: Ident(
            "Error",
        ),
        span: Span {
            start: 92,
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
        kind: RightParen,
        span: Span {
            start: 98,
            end: 99,
        },
    },
    SpannedToken {
        kind: Semicolon,
        span: Span {
            start: 99,
            end: 100,
        },
    },
    SpannedToken {
        kind: RightBrace,
        span: Span {
            start: 106,
            end: 107,
        },
    },
    SpannedToken {
        kind: RightParen,
        span: Span {
            start: 107,
            end: 108,
        },
    },
    SpannedToken {
        kind: LeftParen,
        span: Span {
            start: 108,
            end: 109,
        },
    },
    SpannedToken {
        kind: RightParen,
        span: Span {
            start: 109,
            end: 110,
        },
    },
    SpannedToken {
        kind: Semicolon,
        span: Span {
            start: 110,
            end: 111,
        },
    },
    SpannedToken {
        kind: Var,
        span: Span {
            start: 119,
            end: 122,
        },
    },
    SpannedToken {
        kind: Ident(
            "x",
        ),
        span: Span {
            start: 123,
            end: 124,
        },
    },
    SpannedToken {
        kind: Equal,
        span: Span {
            start: 125,
```

#### ast

- ok: `False`
- truncated: `False`

```text
error: [UnsupportedSyntax] unsupported expression: Some(SpannedToken { kind: Async, span: Span { start: 41, end: 46 } }) at 47..48
```

#### resolved

- ok: `False`
- truncated: `False`

```text
error: [UnsupportedSyntax] unsupported expression: Some(SpannedToken { kind: Async, span: Span { start: 41, end: 46 } }) at 47..48
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
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/asyncIIFE.ts",
        "start": 28,
        "length": 2,
        "line": 3,
        "character": 10,
        "name": "f1"
      },
      {
        "kind": "binding",
        "typeText": "number",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/asyncIIFE.ts",
        "start": 123,
        "length": 1,
        "line": 9,
        "character": 9,
        "name": "x"
      }
    ],
    "typescriptVersion": "6.0.3"
  },
  "ast": {
    "topLevel": [
      {
        "kind": "FunctionDeclaration",
        "text": "function f1() {\r\n    (async () => {\r\n        await 10\r\n        throw new Error();\r\n    })();\r\n\r\n    var x = 1;\r\n}",
        "line": 3,
        "character": 1
      }
    ],
    "pathToPosition": [
      {
        "kind": "SourceFile",
        "text": "function f1() {\r\n    (async () => {\r\n        await 10\r\n        throw new Error();\r\n    })();\r\n\r\n    var x = 1;\r\n}\r\n",
        "line": 3,
        "character": 1
      },
      {
        "kind": "FunctionDeclaration",
        "text": "function f1() {\r\n    (async () => {\r\n        await 10\r\n        throw new Error();\r\n    })();\r\n\r\n    var x = 1;\r\n}",
        "line": 3,
        "character": 1
      },
      {
        "kind": "Block",
        "text": "{\r\n    (async () => {\r\n        await 10\r\n        throw new Error();\r\n    })();\r\n\r\n    var x = 1;\r\n}",
        "line": 3,
        "character": 15
      },
      {
        "kind": "ExpressionStatement",
        "text": "(async () => {\r\n        await 10\r\n        throw new Error();\r\n    })();",
        "line": 4,
        "character": 5
      },
      {
        "kind": "CallExpression",
        "text": "(async () => {\r\n        await 10\r\n        throw new Error();\r\n    })()",
        "line": 4,
        "character": 5
      },
      {
        "kind": "ParenthesizedExpression",
        "text": "(async () => {\r\n        await 10\r\n        throw new Error();\r\n    })",
        "line": 4,
        "character": 5
      },
      {
        "kind": "ArrowFunction",
        "text": "async () => {\r\n        await 10\r\n        throw new Error();\r\n    }",
        "line": 4,
        "character": 6
      }
    ]
  }
}
```

Stack trace:

```text
error: [UnsupportedSyntax] unsupported expression: Some(SpannedToken { kind: Async, span: Span { start: 41, end: 46 } }) at 47..48
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
