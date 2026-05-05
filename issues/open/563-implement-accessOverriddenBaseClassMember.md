---
id: 563
title: "Implement Accessoverriddenbaseclassmember (audit reopened #563)"
type: spike
area: frontend/syntax
class: blocked
priority: P1
depends_on: [5000]
blocks: []
created: 2026-05-01
updated: 2026-05-05status: open
---

## Summary

Triage accessOverriddenBaseClassMember across 1 failing reference test cases and split this bucket into implementation-ready child issues.

## Problem

Reference test results show 1 cases fail in directory `accessOverriddenBaseClassMember` with diagnostics: parser-syntax. The compiler cannot handle these syntax/semantics, preventing compilation of code in this category.

Problem: accessOverriddenBaseClassMember has 1 reference failures and needs smart-triage evidence before implementation starts.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/accessOverriddenBaseClassMember1.ts
```

Coverage window:

```sh
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/accessOverriddenBaseClassMember1.ts --detail
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
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/accessOverriddenBaseClassMember1.ts --detail
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/accessOverriddenBaseClassMember1.ts
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

- `reference/typescript/tests/cases/compiler/accessOverriddenBaseClassMember1.ts`

## Duplicate detection

- `issues/done/092-implement-accessOverriddenBaseClassMember.md` - Implement Accessoverriddenbaseclassmember (same reference path, same feature label, same group key, title overlap)
- `issues/done/460-implement-ClassDeclaration.md` - Implement Classdeclaration (same feature label, same group key, title overlap)
- `issues/open/477-implement-accessOverriddenBaseClassMember.md` - Implement Accessoverriddenbaseclassmember (same reference path, same feature label, same group key, title overlap)

## Smart triage

### Smart triage: Triage parser syntax: accessOverriddenBaseClassMember1

- Issue class: `triage-needed`
- Feature label: `parser-syntax`
- Diagnostic: `UnsupportedSyntax` / `parser-or-frontend-unsupported`
- Path: `reference/typescript/tests/cases/compiler/accessOverriddenBaseClassMember1.ts`

Reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/accessOverriddenBaseClassMember1.ts
```

Source overview:

```json
{
  "suite": "tsc",
  "bytes": 382,
  "lines": 15,
  "extension": ".ts",
  "first_code_line": "class Point {"
}
```

Failure location:

```json
{
  "code": "UnsupportedSyntax",
  "message": "expected LeftParen, got Some(Ident(\"toString\")) at 103..111",
  "span_start": 103,
  "span_end": 111,
  "line": 4,
  "column": 15,
  "feature_label": "parser-syntax",
  "error_type": "parser-or-frontend-unsupported"
}
```

Source context:

```text
1 | // @target: es2015
2 | class Point {
3 |     constructor(public x: number, public y: number) { }
4 |     public toString() {
5 |         return "x=" + this.x + " y=" + this.y;
6 |     }
7 | }
```

Visible symbols before failure:

```json
[
  {
    "kind": "class",
    "name": "Point",
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
    "path": "issues/done/092-implement-accessOverriddenBaseClassMember.md",
    "title": "Implement Accessoverriddenbaseclassmember",
    "reason": "same reference path, same feature label"
  },
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
    "path": "issues/open/477-implement-accessOverriddenBaseClassMember.md",
    "title": "Implement Accessoverriddenbaseclassmember",
    "reason": "same reference path, same feature label"
  },
  {
    "state": "open",
    "path": "issues/open/550-implement-FunctionDeclaration-parser-syntax.md",
    "title": "Implement Functiondeclaration Parser Syntax",
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
            "Point",
        ),
        span: Span {
            start: 26,
            end: 31,
        },
    },
    SpannedToken {
        kind: LeftBrace,
        span: Span {
            start: 32,
            end: 33,
        },
    },
    SpannedToken {
        kind: Ident(
            "constructor",
        ),
        span: Span {
            start: 39,
            end: 50,
        },
    },
    SpannedToken {
        kind: LeftParen,
        span: Span {
            start: 50,
            end: 51,
        },
    },
    SpannedToken {
        kind: Ident(
            "public",
        ),
        span: Span {
            start: 51,
            end: 57,
        },
    },
    SpannedToken {
        kind: Ident(
            "x",
        ),
        span: Span {
            start: 58,
            end: 59,
        },
    },
    SpannedToken {
        kind: Colon,
        span: Span {
            start: 59,
            end: 60,
        },
    },
    SpannedToken {
        kind: Ident(
            "number",
        ),
        span: Span {
            start: 61,
            end: 67,
        },
    },
    SpannedToken {
        kind: Comma,
        span: Span {
            start: 67,
            end: 68,
        },
    },
    SpannedToken {
        kind: Ident(
            "public",
        ),
        span: Span {
            start: 69,
            end: 75,
        },
    },
    SpannedToken {
        kind: Ident(
            "y",
        ),
        span: Span {
            start: 76,
            end: 77,
        },
    },
    SpannedToken {
        kind: Colon,
        span: Span {
            start: 77,
            end: 78,
        },
    },
    SpannedToken {
        kind: Ident(
            "number",
        ),
        span: Span {
            start: 79,
            end: 85,
        },
    },
    SpannedToken {
        kind: RightParen,
        span: Span {
            start: 85,
            end: 86,
        },
    },
    SpannedToken {
        kind: LeftBrace,
        span: Span {
            start: 87,
            end: 88,
        },
    },
    SpannedToken {
        kind: RightBrace,
        span: Span {
            start: 89,
            end: 90,
        },
    },
    SpannedToken {
        kind: Ident(
            "public",
        ),
        span: Span {
            start: 96,
            end: 102,
        },
    },
    SpannedToken {
        kind: Ident(
            "toString",
        ),
        span: Span {
            start: 103,
            end: 111,
        },
    },
    SpannedToken {
        kind: LeftParen,
        span: Span {
            start: 111,
            end: 112,
        },
    },
    SpannedToken {
        kind: RightParen,
        span: Span {
            start: 112,
            end: 113,
        },
    },
    SpannedToken {
        kind: LeftBrace,
        span: Span {
            start: 114,
            end: 115,
        },
    },
    SpannedToken {
        kind: Return,
        span: Span {
            start: 125,
            end: 131,
        },
    },
    SpannedToken {
        kind: String(
            "x=",
        ),
        span: Span {
            start: 132,
            end: 136,
        },
    },
    SpannedToken {
        kind: Plus,
        span: Span {
            start: 137,
            end: 138,
        },
    },
```

#### ast

- ok: `False`
- truncated: `False`

```text
error: [UnsupportedSyntax] expected LeftParen, got Some(Ident("toString")) at 103..111
```

#### resolved

- ok: `False`
- truncated: `False`

```text
error: [UnsupportedSyntax] expected LeftParen, got Some(Ident("toString")) at 103..111
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
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/accessOverriddenBaseClassMember1.ts",
        "start": 58,
        "length": 1,
        "line": 3,
        "character": 24,
        "name": "x"
      },
      {
        "kind": "parameter",
        "typeText": "number",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/accessOverriddenBaseClassMember1.ts",
        "start": 76,
        "length": 1,
        "line": 3,
        "character": 42,
        "name": "y"
      },
      {
        "kind": "binary-expression",
        "typeText": "string",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/accessOverriddenBaseClassMember1.ts",
        "start": 132,
        "length": 30,
        "line": 5,
        "character": 16,
        "operator": "+",
        "leftType": "string",
        "rightType": "number",
        "candidate": "string-concat-fast-path"
      },
      {
        "kind": "binary-expression",
        "typeText": "string",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/accessOverriddenBaseClassMember1.ts",
        "start": 132,
        "length": 21,
        "line": 5,
        "character": 16,
        "operator": "+",
        "leftType": "string",
        "rightType": "\" y=\"",
        "candidate": "string-concat-fast-path"
      },
      {
        "kind": "binary-expression",
        "typeText": "string",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/accessOverriddenBaseClassMember1.ts",
        "start": 132,
        "length": 13,
        "line": 5,
        "character": 16,
        "operator": "+",
        "leftType": "\"x=\"",
        "rightType": "number"
      },
      {
        "kind": "parameter",
        "typeText": "number",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/accessOverriddenBaseClassMember1.ts",
        "start": 227,
        "length": 1,
        "line": 9,
        "character": 17,
        "name": "x"
      },
      {
        "kind": "parameter",
        "typeText": "number",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/accessOverriddenBaseClassMember1.ts",
        "start": 238,
        "length": 1,
        "line": 9,
        "character": 28,
        "name": "y"
      },
      {
        "kind": "parameter",
        "typeText": "string",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/accessOverriddenBaseClassMember1.ts",
        "start": 256,
        "length": 5,
        "line": 9,
        "character": 46,
        "name": "color"
      },
      {
        "kind": "binary-expression",
        "typeText": "string",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/accessOverriddenBaseClassMember1.ts",
        "start": 343,
        "length": 41,
        "line": 13,
        "character": 16,
        "operator": "+",
        "leftType": "string",
        "rightType": "string",
        "candidate": "string-concat-fast-path"
      },
      {
        "kind": "binary-expression",
        "typeText": "string",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/accessOverriddenBaseClassMember1.ts",
        "start": 343,
        "length": 28,
        "line": 13,
        "character": 16,
        "operator": "+",
        "leftType": "string",
        "rightType": "\" color=\"",
        "candidate": "string-concat-fast-path"
      }
    ],
    "typescriptVersion": "6.0.3"
  },
  "ast": {
    "topLevel": [
      {
        "kind": "ClassDeclaration",
        "text": "class Point {\r\n    constructor(public x: number, public y: number) { }\r\n    public toString() {\r\n        return \"x=\" + t",
        "line": 2,
        "character": 1
      },
      {
        "kind": "ClassDeclaration",
        "text": "class ColoredPoint extends Point {\r\n    constructor(x: number, y: number, public color: string) {\r\n        super(x, y);\r",
        "line": 8,
        "character": 1
      }
    ],
    "pathToPosition": [
      {
        "kind": "SourceFile",
        "text": "class Point {\r\n    constructor(public x: number, public y: number) { }\r\n    public toString() {\r\n        return \"x=\" + t",
        "line": 2,
        "character": 1
      },
      {
        "kind": "ClassDeclaration",
        "text": "class Point {\r\n    constructor(public x: number, public y: number) { }\r\n    public toString() {\r\n        return \"x=\" + t",
        "line": 2,
        "character": 1
      },
      {
        "kind": "MethodDeclaration",
        "text": "public toString() {\r\n        return \"x=\" + this.x + \" y=\" + this.y;\r\n    }",
        "line": 4,
        "character": 5
      },
      {
        "kind": "Identifier",
        "text": "toString",
        "line": 4,
        "character": 12
      }
    ]
  }
}
```

Stack trace:

```text
error: [UnsupportedSyntax] expected LeftParen, got Some(Ident("toString")) at 103..111
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

## Status

Superseded by issue #092. Duplicate from separate coverage run.

## Reopened by audit

Date: 2026-05-05

Classification: must-reopen.

Reopen reason: frontmatter still says `class: blocked`, which is incompatible with a completed issue unless explicit supersedence/closure evidence is present.

Violated acceptance: the issue cannot provide repo-local close evidence for its checked acceptance criteria while it remains in this state. Acceptance checkboxes were reset for re-verification.

Evidence files:
- `issues/open/563-implement-accessOverriddenBaseClassMember.md` before this move
- `issues/open/563-implement-accessOverriddenBaseClassMember.md` after this move

Split follow-up: none created in this audit wave; this reopened issue remains the tracking item.
