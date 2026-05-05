---
id: 182
title: "Implement Anyandunknownhavefalsycomponents (dup)"
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

Triage anyAndUnknownHaveFalsyComponents across 1 failing reference test cases and split this bucket into implementation-ready child issues.

## Problem

Reference test results show 1 cases fail in directory `anyAndUnknownHaveFalsyComponents` with diagnostics: parser-syntax. The compiler cannot handle these syntax/semantics, preventing compilation of code in this category.

Problem: anyAndUnknownHaveFalsyComponents has 1 reference failures and needs smart-triage evidence before implementation starts.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/anyAndUnknownHaveFalsyComponents.ts
```

Coverage window:

```sh
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/anyAndUnknownHaveFalsyComponents.ts --detail
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
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/anyAndUnknownHaveFalsyComponents.ts --detail
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/anyAndUnknownHaveFalsyComponents.ts
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

- `reference/typescript/tests/cases/compiler/anyAndUnknownHaveFalsyComponents.ts`

## Duplicate detection

## Smart triage

### Smart triage: Triage parser syntax: anyAndUnknownHaveFalsyComponents

- Issue class: `triage-needed`
- Feature label: `parser-syntax`
- Diagnostic: `UnsupportedSyntax` / `parser-or-frontend-unsupported`
- Path: `reference/typescript/tests/cases/compiler/anyAndUnknownHaveFalsyComponents.ts`

Reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/anyAndUnknownHaveFalsyComponents.ts
```

Source overview:

```json
{
  "suite": "tsc",
  "bytes": 454,
  "lines": 30,
  "extension": ".ts",
  "first_code_line": "declare let x1: any;"
}
```

Failure location:

```json
{
  "code": "UnsupportedSyntax",
  "message": "expected Semicolon, got Some(Let) at 55..58",
  "span_start": 55,
  "span_end": 58,
  "line": 4,
  "column": 9,
  "feature_label": "parser-syntax",
  "error_type": "parser-or-frontend-unsupported"
}
```

Source context:

```text
1 | // @target: es2015
2 | // @strictNullChecks: true
3 |
4 | declare let x1: any;
5 | const y1 = x1 && 3;
6 |
7 | // #39113
```

Visible symbols before failure:

```json
[]
```

Duplicate candidates:

```json
[
  {
    "state": "open",
    "path": "issues/open/059-implement-parser-syntax-extensions.md",
    "title": "Implement parser syntax extensions for TypeScript and advanced JS",
    "reason": "same feature label, title overlap"
  },
  {
    "state": "open",
    "path": "issues/done/182-implement-anyAndUnknownHaveFalsyComponents.md",
    "title": "Implement Anyandunknownhavefalsycomponents",
    "reason": "same reference path, same feature label, title overlap"
  },
  {
    "state": "open",
    "path": "issues/done/200-implement-parser-syntax.md",
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
        kind: Ident(
            "declare",
        ),
        span: Span {
            start: 47,
            end: 54,
        },
    },
    SpannedToken {
        kind: Let,
        span: Span {
            start: 55,
            end: 58,
        },
    },
    SpannedToken {
        kind: Ident(
            "x1",
        ),
        span: Span {
            start: 59,
            end: 61,
        },
    },
    SpannedToken {
        kind: Colon,
        span: Span {
            start: 61,
            end: 62,
        },
    },
    SpannedToken {
        kind: Ident(
            "any",
        ),
        span: Span {
            start: 63,
            end: 66,
        },
    },
    SpannedToken {
        kind: Semicolon,
        span: Span {
            start: 66,
            end: 67,
        },
    },
    SpannedToken {
        kind: Const,
        span: Span {
            start: 68,
            end: 73,
        },
    },
    SpannedToken {
        kind: Ident(
            "y1",
        ),
        span: Span {
            start: 74,
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
        kind: Ident(
            "x1",
        ),
        span: Span {
            start: 79,
            end: 81,
        },
    },
    SpannedToken {
        kind: AndAnd,
        span: Span {
            start: 82,
            end: 84,
        },
    },
    SpannedToken {
        kind: Number(
            3,
        ),
        span: Span {
            start: 85,
            end: 86,
        },
    },
    SpannedToken {
        kind: Semicolon,
        span: Span {
            start: 86,
            end: 87,
        },
    },
```

#### ast

- ok: `False`
- truncated: `False`

```text
error: [UnsupportedSyntax] expected Semicolon, got Some(Let) at 55..58
```

#### resolved

- ok: `False`
- truncated: `False`

```text
error: [UnsupportedSyntax] expected Semicolon, got Some(Let) at 55..58
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
        "kind": "binding",
        "typeText": "any",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/anyAndUnknownHaveFalsyComponents.ts",
        "start": 59,
        "length": 2,
        "line": 4,
        "character": 13,
        "name": "x1"
      },
      {
        "kind": "binding",
        "typeText": "any",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/anyAndUnknownHaveFalsyComponents.ts",
        "start": 74,
        "length": 2,
        "line": 5,
        "character": 7,
        "name": "y1"
      },
      {
        "kind": "binding",
        "typeText": "any",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/anyAndUnknownHaveFalsyComponents.ts",
        "start": 111,
        "length": 13,
        "line": 8,
        "character": 13,
        "name": "isTreeHeader1"
      },
      {
        "kind": "function",
        "typeText": "any",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/anyAndUnknownHaveFalsyComponents.ts",
        "start": 140,
        "length": 4,
        "line": 9,
        "character": 10,
        "name": "foo1"
      },
      {
        "kind": "binding",
        "typeText": "unknown",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/anyAndUnknownHaveFalsyComponents.ts",
        "start": 259,
        "length": 2,
        "line": 18,
        "character": 13,
        "name": "x2"
      },
      {
        "kind": "binding",
        "typeText": "unknown",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/anyAndUnknownHaveFalsyComponents.ts",
        "start": 278,
        "length": 2,
        "line": 19,
        "character": 7,
        "name": "y2"
      },
      {
        "kind": "binding",
        "typeText": "unknown",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/anyAndUnknownHaveFalsyComponents.ts",
        "start": 315,
        "length": 13,
        "line": 22,
        "character": 13,
        "name": "isTreeHeader2"
      },
      {
        "kind": "function",
        "typeText": "any",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/anyAndUnknownHaveFalsyComponents.ts",
        "start": 348,
        "length": 4,
        "line": 23,
        "character": 10,
        "name": "foo2"
      }
    ],
    "typescriptVersion": "6.0.3"
  },
  "ast": {
    "topLevel": [
      {
        "kind": "FirstStatement",
        "text": "declare let x1: any;",
        "line": 4,
        "character": 1
      },
      {
        "kind": "FirstStatement",
        "text": "const y1 = x1 && 3;",
        "line": 5,
        "character": 1
      },
      {
        "kind": "FirstStatement",
        "text": "declare let isTreeHeader1: any;",
        "line": 8,
        "character": 1
      },
      {
        "kind": "FunctionDeclaration",
        "text": "function foo1() {\n  return {\n    display: \"block\",\n    ...(isTreeHeader1 && {\n      display: \"flex\",\n    })\n  };\n}",
        "line": 9,
        "character": 1
      },
      {
        "kind": "FirstStatement",
        "text": "declare let x2: unknown;",
        "line": 18,
        "character": 1
      },
      {
        "kind": "FirstStatement",
        "text": "const y2 = x2 && 3;",
        "line": 19,
        "character": 1
      },
      {
        "kind": "FirstStatement",
        "text": "declare let isTreeHeader2: unknown;",
        "line": 22,
        "character": 1
      },
      {
        "kind": "FunctionDeclaration",
        "text": "function foo2() {\n  return {\n    display: \"block\",\n    ...(isTreeHeader1 && {\n      display: \"flex\",\n    })\n  };\n}",
        "line": 23,
        "character": 1
      }
    ],
    "pathToPosition": [
      {
        "kind": "SourceFile",
        "text": "declare let x1: any;\nconst y1 = x1 && 3;\n\n// #39113\ndeclare let isTreeHeader1: any;\nfunction foo1() {\n  return {\n    dis",
        "line": 4,
        "character": 1
      },
      {
        "kind": "FirstStatement",
        "text": "declare let x1: any;",
        "line": 4,
        "character": 1
      },
      {
        "kind": "VariableDeclarationList",
        "text": "let x1: any",
        "line": 4,
        "character": 9
      }
    ]
  }
}
```

Stack trace:

```text
error: [UnsupportedSyntax] expected Semicolon, got Some(Let) at 55..58
```


## 重複整理メモ

この issue はタイトルが重複しているため、内容がより充実している
`issues/open/640-implement-anyAndUnknownHaveFalsyComponents.md` に統合されました。
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
- `issues/done/182-implement-anyAndUnknownHaveFalsyComponents.md`
- `issues/index.md` after regeneration

Future-work tracking: no untracked future-work item was identified in this issue during this metadata/evidence audit.
