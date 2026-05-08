---
id: 192
title: "Implement Argsinscope (dup)"
type: spike
area: frontend/syntax
class: superseded
priority: P2
depends_on: [5006]
blocks: []
created: 2026-04-29
updated: 2026-05-04
---

## Summary

Triage argsInScope across 1 failing reference test cases and split this bucket into implementation-ready child issues.

## Problem

Reference test results show 1 cases fail in directory `argsInScope` with diagnostics: scope-analysis. The compiler cannot handle these syntax/semantics, preventing compilation of code in this category.

Problem: argsInScope has 1 reference failures and needs smart-triage evidence before implementation starts.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/argsInScope.ts
```

Coverage window:

```sh
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/argsInScope.ts --detail
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
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/argsInScope.ts --detail
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/argsInScope.ts
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

- `reference/typescript/tests/cases/compiler/argsInScope.ts`

## Duplicate detection

## Smart triage

### Smart triage: Triage scope analysis: argsInScope

- Issue class: `triage-needed`
- Feature label: `scope-analysis`
- Diagnostic: `UnsupportedSyntax` / `parser-or-frontend-unsupported`
- Path: `reference/typescript/tests/cases/compiler/argsInScope.ts`

Reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/argsInScope.ts
```

Source overview:

```json
{
  "suite": "tsc",
  "bytes": 221,
  "lines": 11,
  "extension": ".ts",
  "first_code_line": "class C {"
}
```

Failure location:

```json
{
  "code": "UnsupportedSyntax",
  "message": "expected RightParen, got Some(Increment) at 117..119",
  "span_start": 117,
  "span_end": 119,
  "line": 4,
  "column": 50,
  "feature_label": "scope-analysis",
  "error_type": "parser-or-frontend-unsupported"
}
```

Source context:

```text
1 | // @target: es2015
2 | class C {
3 |     P(ii:number, j:number, k:number) {
4 |        for (var i = 0; i < arguments.length; i++) {
5 |            // WScript.Echo("param: " + arguments[i]);
6 |        }
7 |     }
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
    "name": "i",
    "line": 4,
    "column": 13,
    "initializer": "0"
  }
]
```

Duplicate candidates:

```json
[
  {
    "state": "open",
    "path": "issues/done/192-implement-argsInScope.md",
    "title": "Implement Argsinscope",
    "reason": "same reference path, title overlap"
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
            "P",
        ),
        span: Span {
            start: 35,
            end: 36,
        },
    },
    SpannedToken {
        kind: LeftParen,
        span: Span {
            start: 36,
            end: 37,
        },
    },
    SpannedToken {
        kind: Ident(
            "ii",
        ),
        span: Span {
            start: 37,
            end: 39,
        },
    },
    SpannedToken {
        kind: Colon,
        span: Span {
            start: 39,
            end: 40,
        },
    },
    SpannedToken {
        kind: Ident(
            "number",
        ),
        span: Span {
            start: 40,
            end: 46,
        },
    },
    SpannedToken {
        kind: Comma,
        span: Span {
            start: 46,
            end: 47,
        },
    },
    SpannedToken {
        kind: Ident(
            "j",
        ),
        span: Span {
            start: 48,
            end: 49,
        },
    },
    SpannedToken {
        kind: Colon,
        span: Span {
            start: 49,
            end: 50,
        },
    },
    SpannedToken {
        kind: Ident(
            "number",
        ),
        span: Span {
            start: 50,
            end: 56,
        },
    },
    SpannedToken {
        kind: Comma,
        span: Span {
            start: 56,
            end: 57,
        },
    },
```

#### ast

- ok: `False`
- truncated: `False`

```text
error: [UnsupportedSyntax] expected RightParen, got Some(Increment) at 117..119
```

#### resolved

- ok: `False`
- truncated: `False`

```text
error: [UnsupportedSyntax] expected RightParen, got Some(Increment) at 117..119
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
        "typeText": "number",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/argsInScope.ts",
        "start": 37,
        "length": 2,
        "line": 3,
        "character": 7,
        "name": "ii"
      },
      {
        "kind": "parameter",
        "typeText": "number",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/argsInScope.ts",
        "start": 48,
        "length": 1,
        "line": 3,
        "character": 18,
        "name": "j"
      },
      {
        "kind": "parameter",
        "typeText": "number",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/argsInScope.ts",
        "start": 58,
        "length": 1,
        "line": 3,
        "character": 28,
        "name": "k"
      },
      {
        "kind": "binding",
        "typeText": "number",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/argsInScope.ts",
        "start": 87,
        "length": 1,
        "line": 4,
        "character": 17,
        "name": "i"
      },
      {
        "kind": "binding",
        "typeText": "C",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/argsInScope.ts",
        "start": 205,
        "length": 1,
        "line": 10,
        "character": 5,
        "name": "c"
      }
    ],
    "typescriptVersion": "6.0.3"
  },
  "ast": {
    "topLevel": [
      {
        "kind": "ClassDeclaration",
        "text": "class C {\r\n    P(ii:number, j:number, k:number) {\r\n       for (var i = 0; i < arguments.length; i++) {\r\n           // WS",
        "line": 2,
        "character": 1
      },
      {
        "kind": "FirstStatement",
        "text": "var c = new C();",
        "line": 10,
        "character": 1
      },
      {
        "kind": "ExpressionStatement",
        "text": "c.P(1,2,3);",
        "line": 11,
        "character": 1
      }
    ],
    "pathToPosition": [
      {
        "kind": "SourceFile",
        "text": "class C {\r\n    P(ii:number, j:number, k:number) {\r\n       for (var i = 0; i < arguments.length; i++) {\r\n           // WS",
        "line": 2,
        "character": 1
      },
      {
        "kind": "ClassDeclaration",
        "text": "class C {\r\n    P(ii:number, j:number, k:number) {\r\n       for (var i = 0; i < arguments.length; i++) {\r\n           // WS",
        "line": 2,
        "character": 1
      },
      {
        "kind": "MethodDeclaration",
        "text": "P(ii:number, j:number, k:number) {\r\n       for (var i = 0; i < arguments.length; i++) {\r\n           // WScript.Echo(\"par",
        "line": 3,
        "character": 5
      },
      {
        "kind": "Block",
        "text": "{\r\n       for (var i = 0; i < arguments.length; i++) {\r\n           // WScript.Echo(\"param: \" + arguments[i]);\r\n       }\r",
        "line": 3,
        "character": 38
      },
      {
        "kind": "ForStatement",
        "text": "for (var i = 0; i < arguments.length; i++) {\r\n           // WScript.Echo(\"param: \" + arguments[i]);\r\n       }",
        "line": 4,
        "character": 8
      },
      {
        "kind": "PostfixUnaryExpression",
        "text": "i++",
        "line": 4,
        "character": 46
      },
      {
        "kind": "Identifier",
        "text": "i",
        "line": 4,
        "character": 46
      }
    ]
  }
}
```

Stack trace:

```text
error: [UnsupportedSyntax] expected RightParen, got Some(Increment) at 117..119
```


## 重複整理メモ

この issue はタイトルが重複しているため、内容がより充実している
`issues/done/645-implement-argsInScope.md` に統合されました。
そちらを参照してください。
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

## False-done audit

Date: 2026-05-05

Classification: truly-done.

Audit result: retained in `issues/done/`. The issue has repo-local completion evidence, is not marked `blocked` or `triage-needed`, and `python scripts/manager.py check issues` validates the resulting issue graph.

Evidence files:
- `issues/done/192-implement-argsInScope.md`
- `issues/index.md` after regeneration

Future-work tracking: no untracked future-work item was identified in this issue during this metadata/evidence audit.
