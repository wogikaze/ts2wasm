---
id: 696
title: "Implement Arrayconcat (audit reopened #696)"
type: spike
area: runtime/builtins
class: triage-needed
priority: P1
depends_on: []
blocks: []
created: 2026-05-01
updated: 2026-05-05status: open
---

## Summary

Triage arrayconcat across 1 failing reference test cases and split this bucket into implementation-ready child issues.

## Problem

Reference test results show 1 cases fail in directory `arrayconcat` with diagnostics: parser-syntax. The compiler cannot handle these syntax/semantics, preventing compilation of code in this category.

Problem: arrayconcat has 1 reference failures and needs smart-triage evidence before implementation starts.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/arrayconcat.ts
```

Coverage window:

```sh
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/arrayconcat.ts --detail
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
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/arrayconcat.ts --detail
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/arrayconcat.ts
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

- `reference/typescript/tests/cases/compiler/arrayconcat.ts`

## Duplicate detection

- none found by path/title/feature scan

## Smart triage

### Smart triage: Triage parser syntax: arrayconcat

- Issue class: `triage-needed`
- Feature label: `parser-syntax`
- Diagnostic: `UnsupportedSyntax` / `parser-or-frontend-unsupported`
- Path: `reference/typescript/tests/cases/compiler/arrayconcat.ts`

Reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/arrayconcat.ts
```

Source overview:

```json
{
  "suite": "tsc",
  "bytes": 607,
  "lines": 29,
  "extension": ".ts",
  "first_code_line": "interface IOptions {"
}
```

Failure location:

```json
{
  "code": "UnsupportedSyntax",
  "message": "expected LeftParen, got Some(Ident(\"options\")) at 235..242",
  "span_start": 235,
  "span_end": 242,
  "line": 13,
  "column": 21,
  "feature_label": "parser-syntax",
  "error_type": "parser-or-frontend-unsupported"
}
```

Source context:

```text
10 | }
11 | 
12 | class parser {
13 |  public options: IOptions[];
14 | 
15 |  public m() {
16 |   this.options = this.options.sort(function(a, b) {
```

Visible symbols before failure:

```json
[
  {
    "kind": "class",
    "name": "parser",
    "line": 12,
    "column": 1
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
    "path": "issues/done/550-implement-FunctionDeclaration-parser-syntax.md",
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
  },
  {
    "state": "done",
    "path": "issues/done/243-implement-numeric-literal-separator-parser.md",
    "title": "Implement numeric literal separator parser support",
    "reason": "same feature label, title overlap"
  },
  {
    "state": "done",
    "path": "issues/done/244-implement-bigint-literal-parser-classification.md",
    "title": "Implement BigInt literal parser classification",
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
            "interface",
        ),
        span: Span {
            start: 20,
            end: 29,
        },
    },
    SpannedToken {
        kind: Ident(
            "IOptions",
        ),
        span: Span {
            start: 30,
            end: 38,
        },
    },
    SpannedToken {
        kind: LeftBrace,
        span: Span {
            start: 39,
            end: 40,
        },
    },
    SpannedToken {
        kind: Ident(
            "name",
        ),
        span: Span {
            start: 46,
            end: 50,
        },
    },
    SpannedToken {
        kind: Question,
        span: Span {
            start: 50,
            end: 51,
        },
    },
    SpannedToken {
        kind: Colon,
        span: Span {
            start: 51,
            end: 52,
        },
    },
    SpannedToken {
        kind: Ident(
            "string",
        ),
        span: Span {
            start: 53,
            end: 59,
        },
    },
    SpannedToken {
        kind: Semicolon,
        span: Span {
            start: 59,
            end: 60,
        },
    },
    SpannedToken {
        kind: Ident(
            "flag",
        ),
        span: Span {
            start: 66,
            end: 70,
        },
    },
    SpannedToken {
        kind: Question,
        span: Span {
            start: 70,
            end: 71,
        },
    },
    SpannedToken {
        kind: Colon,
        span: Span {
            start: 71,
            end: 72,
        },
    },
    SpannedToken {
        kind: Ident(
            "boolean",
        ),
        span: Span {
            start: 73,
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
            "short",
        ),
        span: Span {
            start: 87,
            end: 92,
        },
    },
    SpannedToken {
        kind: Question,
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
        kind: Ident(
            "string",
        ),
        span: Span {
            start: 95,
            end: 101,
        },
    },
    SpannedToken {
        kind: Semicolon,
        span: Span {
            start: 101,
            end: 102,
        },
    },
    SpannedToken {
        kind: Ident(
            "usage",
        ),
        span: Span {
            start: 108,
            end: 113,
        },
    },
    SpannedToken {
        kind: Question,
        span: Span {
            start: 113,
            end: 114,
        },
    },
    SpannedToken {
        kind: Colon,
        span: Span {
            start: 114,
            end: 115,
        },
    },
    SpannedToken {
        kind: Ident(
            "string",
        ),
        span: Span {
            start: 116,
            end: 122,
        },
    },
    SpannedToken {
        kind: Semicolon,
        span: Span {
            start: 122,
            end: 123,
        },
    },
    SpannedToken {
        kind: Ident(
            "set",
        ),
        span: Span {
            start: 129,
            end: 132,
        },
    },
    SpannedToken {
        kind: Question,
        span: Span {
            start: 132,
            end: 133,
        },
    }
```

#### ast

- ok: `False`
- truncated: `False`

```text
error: [UnsupportedSyntax] expected LeftParen, got Some(Ident("options")) at 235..242
```

#### resolved

- ok: `False`
- truncated: `False`

```text
error: [UnsupportedSyntax] expected LeftParen, got Some(Ident("options")) at 235..242
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
        "message": "Property 'options' has no initializer and is not definitely assigned in the constructor.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/arrayconcat.ts",
        "start": 235,
        "length": 7,
        "line": 13,
        "character": 9
      },
      {
        "code": 18048,
        "category": "Error",
        "message": "'a.name' is possibly 'undefined'.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/arrayconcat.ts",
        "start": 351,
        "length": 6,
        "line": 17,
        "character": 25
      },
      {
        "code": 18048,
        "category": "Error",
        "message": "'b.name' is possibly 'undefined'.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/arrayconcat.ts",
        "start": 398,
        "length": 6,
        "line": 18,
        "character": 25
      }
    ],
    "hints": [
      {
        "kind": "parameter",
        "typeText": "string",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/arrayconcat.ts",
        "start": 136,
        "length": 1,
        "line": 7,
        "character": 12,
        "name": "s"
      },
      {
        "kind": "parameter",
        "typeText": "IOptions",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/arrayconcat.ts",
        "start": 318,
        "length": 1,
        "line": 16,
        "character": 45,
        "name": "a"
      },
      {
        "kind": "parameter",
        "typeText": "IOptions",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/arrayconcat.ts",
        "start": 321,
        "length": 1,
        "line": 16,
        "character": 48,
        "name": "b"
      },
      {
        "kind": "binding",
        "typeText": "string",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/arrayconcat.ts",
        "start": 343,
        "length": 5,
        "line": 17,
        "character": 17,
        "name": "aName"
      },
      {
        "kind": "binding",
        "typeText": "string",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/arrayconcat.ts",
        "start": 390,
        "length": 5,
        "line": 18,
        "character": 17,
        "name": "bName"
      }
    ],
    "typescriptVersion": "6.0.3"
  },
  "ast": {
    "topLevel": [
      {
        "kind": "InterfaceDeclaration",
        "text": "interface IOptions {\r\n    name?: string;\r\n    flag?: boolean;\r\n    short?: string;\r\n    usage?: string;\r\n    set?: (s: s",
        "line": 2,
        "character": 1
      },
      {
        "kind": "ClassDeclaration",
        "text": "class parser {\r\n\tpublic options: IOptions[];\r\n\r\n\tpublic m() {\r\n\t\tthis.options = this.options.sort(function(a, b) {\r\n    ",
        "line": 12,
        "character": 1
      }
    ],
    "pathToPosition": [
      {
        "kind": "SourceFile",
        "text": "interface IOptions {\r\n    name?: string;\r\n    flag?: boolean;\r\n    short?: string;\r\n    usage?: string;\r\n    set?: (s: s",
        "line": 2,
        "character": 1
      },
      {
        "kind": "ClassDeclaration",
        "text": "class parser {\r\n\tpublic options: IOptions[];\r\n\r\n\tpublic m() {\r\n\t\tthis.options = this.options.sort(function(a, b) {\r\n    ",
        "line": 12,
        "character": 1
      },
      {
        "kind": "PropertyDeclaration",
        "text": "public options: IOptions[];",
        "line": 13,
        "character": 2
      },
      {
        "kind": "Identifier",
        "text": "options",
        "line": 13,
        "character": 9
      }
    ]
  }
}
```

Stack trace:

```text
error: [UnsupportedSyntax] expected LeftParen, got Some(Ident("options")) at 235..242
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

## Reopened by audit

Date: 2026-05-05

Classification: must-reopen.

Reopen reason: frontmatter still says `class: triage-needed`; generated triage buckets are not done until split or superseded with evidence.

Violated acceptance: the issue cannot provide repo-local close evidence for its checked acceptance criteria while it remains in this state. Acceptance checkboxes were reset for re-verification.

Evidence files:
- `issues/open/696-implement-arrayconcat.md` before this move
- `issues/open/696-implement-arrayconcat.md` after this move

Split follow-up: none created in this audit wave; this reopened issue remains the tracking item.
