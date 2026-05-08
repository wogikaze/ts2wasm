---
id: 774
title: "Implement Autolift"
type: spike
area: frontend/syntax
class: superseded
priority: P1
depends_on: [5159]
blocks: []
created: 2026-05-01
updated: 2026-05-06
status: done
completed: 2026-05-06
---

## Summary

Triage autoLift across 1 failing reference test cases and split this bucket into implementation-ready child issues.

## Problem

Reference test results show 1 cases fail in directory `autoLift` with diagnostics: parser-syntax. The compiler cannot handle these syntax/semantics, preventing compilation of code in this category.

Problem: autoLift has 1 reference failures and needs smart-triage evidence before implementation starts.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/autoLift2.ts
```

Coverage window:

```sh
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/autoLift2.ts --detail
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
mise run reference-coverage -- tsc --limit 2
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/autoLift2.ts --detail
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/autoLift2.ts
```

Not run:

- none

## Docs / current-state / issue sync

Final-state docs:

- [x] not affected

Current state:

- [x] not affected

Follow-up issues:

- [x] created/updated: `issues/open/5159-recover-colon-type-annotation-after-expression-statement.md`

## Notes

## Affected test files

- `reference/typescript/tests/cases/compiler/autoLift2.ts`

## Duplicate detection

- none found by path/title/feature scan

## Smart triage

### Smart triage: Triage parser syntax: autoLift2

- Issue class: `triage-needed`
- Feature label: `parser-syntax`
- Diagnostic: `UnsupportedSyntax` / `parser-or-frontend-unsupported`
- Path: `reference/typescript/tests/cases/compiler/autoLift2.ts`

Reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/autoLift2.ts
```

Source overview:

```json
{
  "suite": "tsc",
  "bytes": 280,
  "lines": 31,
  "extension": ".ts",
  "first_code_line": "class A"
}
```

Failure location:

```json
{
  "code": "UnsupportedSyntax",
  "message": "expected Semicolon, got Some(Colon) at 71..72",
  "span_start": 71,
  "span_end": 72,
  "line": 6,
  "column": 22,
  "feature_label": "parser-syntax",
  "error_type": "parser-or-frontend-unsupported"
}
```

Source context:

```text
3 | 
4 | {
5 |     constructor() {
6 |         this.foo: any;
7 |         this.bar: any;
8 |     }
9 |
```

Visible symbols before failure:

```json
[
  {
    "kind": "class",
    "name": "A",
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
    "state": "open",
    "path": "issues/open/734-implement-assignmentCompatability-parser-syntax.md",
    "title": "Implement Assignmentcompatability Parser Syntax",
    "reason": "same feature label, title overlap"
  },
  {
    "state": "open",
    "path": "issues/open/753-implement-asyncFunctionReturnType-parser-syntax.md",
    "title": "Implement Asyncfunctionreturntype Parser Syntax",
    "reason": "same feature label, title overlap"
  },
  {
    "state": "open",
    "path": "#767",
    "title": "Implement Augmentedtypesenum Parser Syntax",
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
            "A",
        ),
        span: Span {
            start: 26,
            end: 27,
        },
    },
    SpannedToken {
        kind: LeftBrace,
        span: Span {
            start: 31,
            end: 32,
        },
    },
    SpannedToken {
        kind: Ident(
            "constructor",
        ),
        span: Span {
            start: 38,
            end: 49,
        },
    },
    SpannedToken {
        kind: LeftParen,
        span: Span {
            start: 49,
            end: 50,
        },
    },
    SpannedToken {
        kind: RightParen,
        span: Span {
            start: 50,
            end: 51,
        },
    },
    SpannedToken {
        kind: LeftBrace,
        span: Span {
            start: 52,
            end: 53,
        },
    },
    SpannedToken {
        kind: This,
        span: Span {
            start: 63,
            end: 67,
        },
    },
    SpannedToken {
        kind: Dot,
        span: Span {
            start: 67,
            end: 68,
        },
    },
    SpannedToken {
        kind: Ident(
            "foo",
        ),
        span: Span {
            start: 68,
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
            "any",
        ),
        span: Span {
            start: 73,
            end: 76,
        },
    },
    SpannedToken {
        kind: Semicolon,
        span: Span {
            start: 76,
            end: 77,
        },
    },
    SpannedToken {
        kind: This,
        span: Span {
            start: 87,
            end: 91,
        },
    },
    SpannedToken {
        kind: Dot,
        span: Span {
            start: 91,
            end: 92,
        },
    },
    SpannedToken {
        kind: Ident(
            "bar",
        ),
        span: Span {
            start: 92,
            end: 95,
        },
    },
    SpannedToken {
        kind: Colon,
        span: Span {
            start: 95,
            end: 96,
        },
    },
    SpannedToken {
        kind: Ident(
            "any",
        ),
        span: Span {
            start: 97,
            end: 100,
        },
    },
    SpannedToken {
        kind: Semicolon,
        span: Span {
            start: 100,
            end: 101,
        },
    },
    SpannedToken {
        kind: RightBrace,
        span: Span {
            start: 107,
            end: 108,
        },
    },
    SpannedToken {
        kind: Ident(
            "baz",
        ),
        span: Span {
            start: 116,
            end: 119,
        },
    },
    SpannedToken {
        kind: LeftParen,
        span: Span {
            start: 119,
            end: 120,
        },
    },
    SpannedToken {
        kind: RightParen,
        span: Span {
            start: 120,
            end: 121,
        },
    },
    SpannedToken {
        kind: LeftBrace,
        span: Span {
            start: 122,
            end: 123,
        },
    },
    SpannedToken {
        kind: This,
        span: Span {
            start: 132,
            end: 136,
        },
    },
    SpannedToken {
        kind: Dot,
        span: Span {
            start: 136,
            end: 137,
        },
    },
    SpannedToken {
```

#### ast

- ok: `False`
- truncated: `False`

```text
error: [UnsupportedSyntax] expected Semicolon, got Some(Colon) at 71..72
```

#### resolved

- ok: `False`
- truncated: `False`

```text
error: [UnsupportedSyntax] expected Semicolon, got Some(Colon) at 71..72
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
        "code": 2339,
        "category": "Error",
        "message": "Property 'foo' does not exist on type 'A'.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/autoLift2.ts",
        "start": 68,
        "length": 3,
        "line": 6,
        "character": 14
      },
      {
        "code": 1005,
        "category": "Error",
        "message": "';' expected.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/autoLift2.ts",
        "start": 71,
        "length": 1,
        "line": 6,
        "character": 17
      },
      {
        "code": 2693,
        "category": "Error",
        "message": "'any' only refers to a type, but is being used as a value here.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/autoLift2.ts",
        "start": 73,
        "length": 3,
        "line": 6,
        "character": 19
      },
      {
        "code": 2339,
        "category": "Error",
        "message": "Property 'bar' does not exist on type 'A'.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/autoLift2.ts",
        "start": 92,
        "length": 3,
        "line": 7,
        "character": 14
      },
      {
        "code": 1005,
        "category": "Error",
        "message": "';' expected.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/autoLift2.ts",
        "start": 95,
        "length": 1,
        "line": 7,
        "character": 17
      },
      {
        "code": 2693,
        "category": "Error",
        "message": "'any' only refers to a type, but is being used as a value here.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/autoLift2.ts",
        "start": 97,
        "length": 3,
        "line": 7,
        "character": 19
      },
      {
        "code": 2339,
        "category": "Error",
        "message": "Property 'foo' does not exist on type 'A'.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/autoLift2.ts",
        "start": 137,
        "length": 3,
        "line": 13,
        "character": 11
      },
      {
        "code": 2339,
        "category": "Error",
        "message": "Property 'bar' does not exist on type 'A'.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/autoLift2.ts",
        "start": 163,
        "length": 3,
        "line": 15,
        "character": 11
      },
      {
        "code": 2339,
        "category": "Error",
        "message": "Property 'foo' does not exist on type 'A'.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/autoLift2.ts",
        "start": 211,
        "length": 3,
        "line": 17,
        "character": 33
      },
      {
        "code": 2339,
        "category": "Error",
        "message": "Property 'bar' does not exist on type 'A'.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/autoLift2.ts",
        "start": 252,
        "length": 3,
        "line": 19,
        "character": 33
      }
    ],
    "hints": [
      {
        "kind": "parameter",
        "typeText": "number",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/autoLift2.ts",
        "start": 200,
        "length": 1,
        "line": 17,
        "character": 22,
        "name": "p"
      },
      {
        "kind": "parameter",
        "typeText": "number",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/autoLift2.ts",
        "start": 241,
        "length": 1,
        "line": 19,
        "character": 22,
        "name": "p"
      },
      {
        "kind": "binding",
        "typeText": "A",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/autoLift2.ts",
        "start": 281,
        "length": 1,
        "line": 27,
        "character": 5,
        "name": "a"
      }
    ],
    "typescriptVersion": "6.0.3"
  },
  "ast": {
    "topLevel": [
      {
        "kind": "ClassDeclaration",
        "text": "class A\r\n\r\n{\r\n    constructor() {\r\n        this.foo: any;\r\n        this.bar: any;\r\n    }\r\n\r\n\r\n  baz() {\r\n\r\n     this.foo",
        "line": 2,
        "character": 1
      },
      {
        "kind": "FirstStatement",
        "text": "var a = new A();",
        "line": 27,
        "character": 1
      },
      {
        "kind": "ExpressionStatement",
        "text": "a.baz();",
        "line": 29,
        "character": 1
      }
    ],
    "pathToPosition": [
      {
        "kind": "SourceFile",
        "text": "class A\r\n\r\n{\r\n    constructor() {\r\n        this.foo: any;\r\n        this.bar: any;\r\n    }\r\n\r\n\r\n  baz() {\r\n\r\n     this.foo",
        "line": 2,
        "character": 1
      },
      {
        "kind": "ClassDeclaration",
        "text": "class A\r\n\r\n{\r\n    constructor() {\r\n        this.foo: any;\r\n        this.bar: any;\r\n    }\r\n\r\n\r\n  baz() {\r\n\r\n     this.foo",
        "line": 2,
        "character": 1
      },
      {
        "kind": "Constructor",
        "text": "constructor() {\r\n        this.foo: any;\r\n        this.bar: any;\r\n    }",
        "line": 5,
        "character": 5
      },
      {
        "kind": "Block",
        "text": "{\r\n        this.foo: any;\r\n        this.bar: any;\r\n    }",
        "line": 5,
        "character": 19
      },
      {
        "kind": "ExpressionStatement",
        "text": "this.foo",
        "line": 6,
        "character": 9
      },
      {
        "kind": "PropertyAccessExpression",
        "text": "this.foo",
        "line": 6,
        "character": 9
      },
      {
        "kind": "Identifier",
        "text": "foo",
        "line": 6,
        "character": 14
      }
    ]
  }
}
```

Stack trace:

```text
error: [UnsupportedSyntax] expected Semicolon, got Some(Colon) at 71..72
```

Split child: `issues/open/5159-recover-colon-type-annotation-after-expression-statement.md`

## Completion evidence

Fill only when moving to `done/`.

Commits:

- `pending`

Validation result:

```text
command: python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/autoLift2.ts
result: pass; current blocker identified as parser recovery for `this.foo: any;` / `this.bar: any;`, split to issue 5159
date: 2026-05-06
```

Remaining risks:

- Broader auto-lift behavior needs follow-up triage after issue 5159 advances the parser beyond the current colon boundary.

## False-done audit

Date: 2026-05-06

Classification: truly-done.

Audit result: retained in `issues/done/`. This issue has repo-local close evidence
(completion evidence with validation commands) or proper superseded classification
with child issues in `issues/open/`. The acceptance criteria documented in the issue
are satisfied by the repo-local evidence cited in the completion evidence section.

Future-work tracking: no untracked future-work item was identified in this issue
during this metadata/evidence audit.

