---
id: 169
title: "Implement Ambiguousoverloadresolution (dup)"
type: spike
area: frontend/syntax
class: superseded
priority: P2
depends_on: [5005]
blocks: []
created: 2026-04-29
updated: 2026-05-04
---

## Summary

Triage ambiguousOverloadResolution across 1 failing reference test cases and split this bucket into implementation-ready child issues.

## Problem

Reference test results show 1 cases fail in directory `ambiguousOverloadResolution` with diagnostics: module-resolution. The compiler cannot handle these syntax/semantics, preventing compilation of code in this category.

Problem: ambiguousOverloadResolution has 1 reference failures and needs smart-triage evidence before implementation starts.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/ambiguousOverloadResolution.ts
```

Coverage window:

```sh
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/ambiguousOverloadResolution.ts --detail
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
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/ambiguousOverloadResolution.ts --detail
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/ambiguousOverloadResolution.ts
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

- `reference/typescript/tests/cases/compiler/ambiguousOverloadResolution.ts`

## Duplicate detection

## Smart triage

### Smart triage: Triage module resolution: ambiguousOverloadResolution

- Issue class: `triage-needed`
- Feature label: `module-resolution`
- Diagnostic: `UnsupportedSyntax` / `parser-or-frontend-unsupported`
- Path: `reference/typescript/tests/cases/compiler/ambiguousOverloadResolution.ts`

Reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/ambiguousOverloadResolution.ts
```

Source overview:

```json
{
  "suite": "tsc",
  "bytes": 214,
  "lines": 10,
  "extension": ".ts",
  "first_code_line": "class A { }"
}
```

Failure location:

```json
{
  "code": "UnsupportedSyntax",
  "message": "expected LeftParen, got Some(Colon) at 73..74",
  "span_start": 73,
  "span_end": 74,
  "line": 4,
  "column": 25,
  "feature_label": "module-resolution",
  "error_type": "parser-or-frontend-unsupported"
}
```

Source context:

```text
1 | // @target: es2015
2 | // @strict: false
3 | class A { }
4 | class B extends A { x: number; }
5 |
6 | declare function f(p: A, q: B): number;
7 | declare function f(p: B, q: A): string;
```

Visible symbols before failure:

```json
[
  {
    "kind": "class",
    "name": "A",
    "line": 3,
    "column": 1
  },
  {
    "kind": "class",
    "name": "B",
    "line": 4,
    "column": 1
  }
]
```

Duplicate candidates:

```json
[
  {
    "state": "open",
    "path": "issues/done/169-implement-ambiguousOverloadResolution.md",
    "title": "Implement Ambiguousoverloadresolution",
    "reason": "same reference path, title overlap"
  }
]
```

Error-specific suggestions:

- Start at lexer/parser support and add a minimal fixture for the exact source construct at the failing span.
- Use `dump --tokens` and the TypeScript AST path to decide whether this is tokenization, precedence, or statement dispatch.
- Keep module graph behavior separate from parser syntax unless the diagnostic proves syntax is the blocker.

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
            start: 39,
            end: 44,
        },
    },
    SpannedToken {
        kind: Ident(
            "A",
        ),
        span: Span {
            start: 45,
            end: 46,
        },
    },
    SpannedToken {
        kind: LeftBrace,
        span: Span {
            start: 47,
            end: 48,
        },
    },
    SpannedToken {
        kind: RightBrace,
        span: Span {
            start: 49,
            end: 50,
        },
    },
    SpannedToken {
        kind: Class,
        span: Span {
            start: 52,
            end: 57,
        },
    },
    SpannedToken {
        kind: Ident(
            "B",
        ),
        span: Span {
            start: 58,
            end: 59,
        },
    },
    SpannedToken {
        kind: Extends,
        span: Span {
            start: 60,
            end: 67,
        },
    },
    SpannedToken {
        kind: Ident(
            "A",
        ),
        span: Span {
            start: 68,
            end: 69,
        },
    },
    SpannedToken {
        kind: LeftBrace,
        span: Span {
            start: 70,
            end: 71,
        },
    },
    SpannedToken {
        kind: Ident(
            "x",
        ),
        span: Span {
            start: 72,
            end: 73,
        },
    },
    SpannedToken {
        kind: Colon,
        span: Span {
            start: 73,
            end: 74,
        },
    },
    SpannedToken {
        kind: Ident(
            "number",
        ),
        span: Span {
            start: 75,
            end: 81,
        },
    },
    SpannedToken {
        kind: Semicolon,
        span: Span {
            start: 81,
            end: 82,
        },
    },
    SpannedToken {
```

#### ast

- ok: `False`
- truncated: `False`

```text
error: [UnsupportedSyntax] expected LeftParen, got Some(Colon) at 73..74
```

#### resolved

- ok: `False`
- truncated: `False`

```text
error: [UnsupportedSyntax] expected LeftParen, got Some(Colon) at 73..74
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
        "code": 2564,
        "category": "Error",
        "message": "Property 'x' has no initializer and is not definitely assigned in the constructor.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/ambiguousOverloadResolution.ts",
        "start": 72,
        "length": 1,
        "line": 4,
        "character": 21
      },
      {
        "code": 2454,
        "category": "Error",
        "message": "Variable 'x' is used before being assigned.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/ambiguousOverloadResolution.ts",
        "start": 201,
        "length": 1,
        "line": 10,
        "character": 19
      },
      {
        "code": 2454,
        "category": "Error",
        "message": "Variable 'x' is used before being assigned.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/ambiguousOverloadResolution.ts",
        "start": 204,
        "length": 1,
        "line": 10,
        "character": 22
      }
    ],
    "hints": [
      {
        "kind": "function",
        "typeText": "number",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/ambiguousOverloadResolution.ts",
        "start": 105,
        "length": 1,
        "line": 6,
        "character": 18,
        "name": "f"
      },
      {
        "kind": "parameter",
        "typeText": "A",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/ambiguousOverloadResolution.ts",
        "start": 107,
        "length": 1,
        "line": 6,
        "character": 20,
        "name": "p"
      },
      {
        "kind": "parameter",
        "typeText": "B",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/ambiguousOverloadResolution.ts",
        "start": 113,
        "length": 1,
        "line": 6,
        "character": 26,
        "name": "q"
      },
      {
        "kind": "function",
        "typeText": "string",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/ambiguousOverloadResolution.ts",
        "start": 146,
        "length": 1,
        "line": 7,
        "character": 18,
        "name": "f"
      },
      {
        "kind": "parameter",
        "typeText": "B",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/ambiguousOverloadResolution.ts",
        "start": 148,
        "length": 1,
        "line": 7,
        "character": 20,
        "name": "p"
      },
      {
        "kind": "parameter",
        "typeText": "A",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/ambiguousOverloadResolution.ts",
        "start": 154,
        "length": 1,
        "line": 7,
        "character": 26,
        "name": "q"
      },
      {
        "kind": "binding",
        "typeText": "B",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/ambiguousOverloadResolution.ts",
        "start": 176,
        "length": 1,
        "line": 9,
        "character": 5,
        "name": "x"
      },
      {
        "kind": "binding",
        "typeText": "number",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/ambiguousOverloadResolution.ts",
        "start": 187,
        "length": 1,
        "line": 10,
        "character": 5,
        "name": "t"
      }
    ],
    "typescriptVersion": "6.0.3"
  },
  "ast": {
    "topLevel": [
      {
        "kind": "ClassDeclaration",
        "text": "class A { }",
        "line": 3,
        "character": 1
      },
      {
        "kind": "ClassDeclaration",
        "text": "class B extends A { x: number; }",
        "line": 4,
        "character": 1
      },
      {
        "kind": "FunctionDeclaration",
        "text": "declare function f(p: A, q: B): number;",
        "line": 6,
        "character": 1
      },
      {
        "kind": "FunctionDeclaration",
        "text": "declare function f(p: B, q: A): string;",
        "line": 7,
        "character": 1
      },
      {
        "kind": "FirstStatement",
        "text": "var x: B;",
        "line": 9,
        "character": 1
      },
      {
        "kind": "FirstStatement",
        "text": "var t: number = f(x, x);",
        "line": 10,
        "character": 1
      }
    ],
    "pathToPosition": [
      {
        "kind": "SourceFile",
        "text": "class A { }\r\nclass B extends A { x: number; }\r\n\r\ndeclare function f(p: A, q: B): number;\r\ndeclare function f(p: B, q: A)",
        "line": 3,
        "character": 1
      },
      {
        "kind": "ClassDeclaration",
        "text": "class B extends A { x: number; }",
        "line": 4,
        "character": 1
      },
      {
        "kind": "PropertyDeclaration",
        "text": "x: number;",
        "line": 4,
        "character": 21
      },
      {
        "kind": "Identifier",
        "text": "x",
        "line": 4,
        "character": 21
      }
    ]
  }
}
```

Stack trace:

```text
error: [UnsupportedSyntax] expected LeftParen, got Some(Colon) at 73..74
```


## 重複整理メモ

この issue はタイトルが重複しているため、内容がより充実している
`issues/open/627-implement-ambiguousOverloadResolution.md` に統合されました。
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
- `issues/open/169-implement-ambiguousOverloadResolution.md`
- `issues/index.md` after regeneration

Future-work tracking: no untracked future-work item was identified in this issue during this metadata/evidence audit.
