---
id: 093
title: "Implement Accessstaticmemberfrominstancemethod (dup)"
type: spike
area: frontend/syntax
class: superseded
priority: P1
depends_on: [5000]
blocks: []
created: 2026-04-29
updated: 2026-05-04
---

## Summary

Triage accessStaticMemberFromInstanceMethod across 1 failing reference test cases and split this bucket into implementation-ready child issues.

## Problem

Reference test results show 1 cases fail in directory `accessStaticMemberFromInstanceMethod` with diagnostics: parser-syntax. The compiler cannot handle these syntax/semantics, preventing compilation of code in this category.

Problem: accessStaticMemberFromInstanceMethod has 1 reference failures and needs smart-triage evidence before implementation starts.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/accessStaticMemberFromInstanceMethod01.ts
```

Coverage window:

```sh
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/accessStaticMemberFromInstanceMethod01.ts --detail
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
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/accessStaticMemberFromInstanceMethod01.ts --detail
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/accessStaticMemberFromInstanceMethod01.ts
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

- `reference/typescript/tests/cases/compiler/accessStaticMemberFromInstanceMethod01.ts`

## Duplicate detection

## Smart triage

### Smart triage: Triage parser syntax: accessStaticMemberFromInstanceMethod01

- Issue class: `triage-needed`
- Feature label: `parser-syntax`
- Diagnostic: `UnsupportedSyntax` / `parser-or-frontend-unsupported`
- Path: `reference/typescript/tests/cases/compiler/accessStaticMemberFromInstanceMethod01.ts`

Reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/accessStaticMemberFromInstanceMethod01.ts
```

Source overview:

```json
{
  "suite": "tsc",
  "bytes": 94,
  "lines": 8,
  "extension": ".ts",
  "first_code_line": "class C {"
}
```

Failure location:

```json
{
  "code": "UnsupportedSyntax",
  "message": "expected LeftParen, got Some(Colon) at 38..39",
  "span_start": 38,
  "span_end": 39,
  "line": 3,
  "column": 10,
  "feature_label": "parser-syntax",
  "error_type": "parser-or-frontend-unsupported"
}
```

Source context:

```text
1 | // @target: es2015
2 | class C {
3 |     foo: string;
4 |
5 |     static bar() {
6 |         let k = foo;
```

Visible symbols before failure:

```json
[
  {
    "kind": "class",
    "name": "C",
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
    "path": "issues/done/059-implement-parser-syntax-extensions.md",
    "title": "Implement parser syntax extensions for TypeScript and advanced JS",
    "reason": "same feature label, title overlap"
  },
  {
    "state": "open",
    "path": "issues/open/093-implement-accessStaticMemberFromInstanceMethod.md",
    "title": "Implement Accessstaticmemberfrominstancemethod",
    "reason": "same reference path, same feature label"
  },
  {
    "state": "open",
    "path": "issues/open/200-implement-parser-syntax.md",
    "title": "Implement parser syntax extensions",
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
            "foo",
        ),
        span: Span {
            start: 35,
            end: 38,
        },
    },
    SpannedToken {
        kind: Colon,
        span: Span {
            start: 38,
            end: 39,
        },
    },
    SpannedToken {
        kind: Ident(
            "string",
        ),
        span: Span {
            start: 40,
            end: 46,
        },
    },
    SpannedToken {
        kind: Semicolon,
        span: Span {
            start: 46,
            end: 47,
        },
    },
    SpannedToken {
        kind: Static,
        span: Span {
            start: 55,
            end: 61,
        },
    },
    SpannedToken {
        kind: Ident(
            "bar",
        ),
        span: Span {
            start: 62,
            end: 65,
        },
    },
    SpannedToken {
        kind: LeftParen,
        span: Span {
            start: 65,
            end: 66,
        },
    },
    SpannedToken {
        kind: RightParen,
        span: Span {
            start: 66,
            end: 67,
        },
    },
    SpannedToken {
        kind: LeftBrace,
        span: Span {
            start: 68,
            end: 69,
        },
    },
    SpannedToken {
        kind: Let,
        span: Span {
            start: 79,
            end: 82,
        },
    },
    SpannedToken {
        kind: Ident(
```

#### ast

- ok: `False`
- truncated: `False`

```text
error: [UnsupportedSyntax] expected LeftParen, got Some(Colon) at 38..39
```

#### resolved

- ok: `False`
- truncated: `False`

```text
error: [UnsupportedSyntax] expected LeftParen, got Some(Colon) at 38..39
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
        "message": "Property 'foo' has no initializer and is not definitely assigned in the constructor.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/accessStaticMemberFromInstanceMethod01.ts",
        "start": 35,
        "length": 3,
        "line": 3,
        "character": 5
      },
      {
        "code": 2304,
        "category": "Error",
        "message": "Cannot find name 'foo'.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/accessStaticMemberFromInstanceMethod01.ts",
        "start": 87,
        "length": 3,
        "line": 6,
        "character": 17
      }
    ],
    "hints": [
      {
        "kind": "binding",
        "typeText": "any",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/accessStaticMemberFromInstanceMethod01.ts",
        "start": 83,
        "length": 1,
        "line": 6,
        "character": 13,
        "name": "k"
      }
    ],
    "typescriptVersion": "6.0.3"
  },
  "ast": {
    "topLevel": [
      {
        "kind": "ClassDeclaration",
        "text": "class C {\r\n    foo: string;\r\n\r\n    static bar() {\r\n        let k = foo;\r\n    }\r\n}",
        "line": 2,
        "character": 1
      }
    ],
    "pathToPosition": [
      {
        "kind": "SourceFile",
        "text": "class C {\r\n    foo: string;\r\n\r\n    static bar() {\r\n        let k = foo;\r\n    }\r\n}",
        "line": 2,
        "character": 1
      },
      {
        "kind": "ClassDeclaration",
        "text": "class C {\r\n    foo: string;\r\n\r\n    static bar() {\r\n        let k = foo;\r\n    }\r\n}",
        "line": 2,
        "character": 1
      },
      {
        "kind": "PropertyDeclaration",
        "text": "foo: string;",
        "line": 3,
        "character": 5
      },
      {
        "kind": "Identifier",
        "text": "foo",
        "line": 3,
        "character": 5
      }
    ]
  }
}
```

Stack trace:

```text
error: [UnsupportedSyntax] expected LeftParen, got Some(Colon) at 38..39
```


## 重複整理メモ

この issue はタイトルが重複しているため、内容がより充実している
`issues/open/564-implement-accessStaticMemberFromInstanceMethod.md` に統合されました。
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
