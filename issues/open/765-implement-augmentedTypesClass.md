---
id: 765
title: "Implement Augmentedtypesclass"
type: spike
area: frontend/syntax
class: blocked
priority: P1
depends_on: []
blocks: []
created: 2026-05-01
updated: 2026-05-07
completed: 2026-05-07
status: done
---
> **Reopened by false-done audit** (2026-05-07)
> Classification: must-reopen
> Reason: Batch-migrated to issues/done/ without implementation commits.
> Evidence: Empty completion evidence. No feat/fix commit for #765.

## Summary

Closed this generated parser-syntax bucket by splitting the current concrete
non-parser blocker to
`issues/open/5347-align-class-var-redeclaration-diagnostics.md`.

## Problem

Fresh triage shows the parser now accepts the representative `public foo()`
class methods. The first remaining blocker is a resolver duplicate-local
diagnostic for `class c1` followed by `var c1`.

Problem: the generated bucket remained blocked as parser-syntax even though the
current failure belongs to duplicate identifier diagnostic parity.

## Current failure

Representative reproduction:

```sh
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/augmentedTypesClass.ts
```

Coverage window:

```sh
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/augmentedTypesClass.ts --detail --no-dashboard-data
```

## Desired final state

This generated bucket is closed. Implement from
`issues/open/5347-align-class-var-redeclaration-diagnostics.md`.

## Scope

In scope:

- [x] Inspect the smart triage report below
- [x] Confirm whether existing open/done issues already cover this bucket
- [x] Split one feature family, one observable behavior, or one fixed reference window into a child issue
- [x] Preserve exact reproduction commands and representative AST/diagnostic evidence in the child issue

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
- [x] Child issue contains an exact `reference-triage` command
- [x] Child issue includes failing path, diagnostic code, source context, visible symbols, and parser/TypeScript AST evidence
- [x] Child issue acceptance names the exact fixture/reference path and diagnostic/stdout change

## Validation

Required commands:

```sh
python scripts/manager.py update-issue-index --check
python scripts/manager.py check-issue-health
python scripts/manager.py check-issue-readiness -- --fail-ready-below 80
```

Impacted commands:

```sh
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/augmentedTypesClass.ts --detail --no-dashboard-data
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/augmentedTypesClass.ts
```

Not run:

- cargo fmt / nextest not run for this metadata-only issue lifecycle closure

## Docs / current-state / issue sync

Final-state docs:

- [x] not affected

Current state:

- [x] not affected

Follow-up issues:

- [x] `issues/open/5347-align-class-var-redeclaration-diagnostics.md`

## Notes

## Affected test files

- `reference/typescript/tests/cases/compiler/augmentedTypesClass.ts`
- `reference/typescript/tests/cases/compiler/augmentedTypesClass4.ts`
- `reference/typescript/tests/cases/compiler/augmentedTypesClass2.ts`
- `reference/typescript/tests/cases/compiler/augmentedTypesClass3.ts`
- `reference/typescript/tests/cases/compiler/augmentedTypesClass2a.ts`

## Duplicate detection

Split to `issues/open/5347-align-class-var-redeclaration-diagnostics.md`.

Related no-match issues:

- `issues/done/5162-allow-compatible-var-redeclarations.md` handles compatible
  duplicate `var` declarations.
- `issues/open/5249-scope-block-local-class-declarations.md` handles nested
  block-local classes colliding with outer classes.
- `issues/open/767-implement-augmentedTypesEnum-parser-syntax.md` is an
  enum-focused generated bucket, not the first current `class`/`var` blocker.

## Smart triage

### Smart triage: Triage parser syntax: augmentedTypesClass

- Issue class: `triage-needed`
- Feature label: `parser-syntax`
- Diagnostic: `UnsupportedSyntax` / `parser-or-frontend-unsupported`
- Path: `reference/typescript/tests/cases/compiler/augmentedTypesClass.ts`

Reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/augmentedTypesClass.ts
```

Source overview:

```json
{
  "suite": "tsc",
  "bytes": 166,
  "lines": 8,
  "extension": ".ts",
  "first_code_line": "class c1 { public foo() { } }"
}
```

Failure location:

```json
{
  "code": "UnsupportedSyntax",
  "message": "expected LeftParen, got Some(Ident(\"foo\")) at 59..62",
  "span_start": 59,
  "span_end": 62,
  "line": 3,
  "column": 21,
  "feature_label": "parser-syntax",
  "error_type": "parser-or-frontend-unsupported"
}
```

Source context:

```text
1 | // @target: es2015
2 | //// class then var
3 | class c1 { public foo() { } }
4 | var c1 = 1; // error
5 | 
6 | //// class then enum
```

Visible symbols before failure:

```json
[
  {
    "kind": "class",
    "name": "then",
    "line": 2,
    "column": 6
  },
  {
    "kind": "binding",
    "name": "class",
    "line": 2,
    "column": 17
  },
  {
    "kind": "class",
    "name": "c1",
    "line": 3,
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
    "path": "issues/open/550-implement-FunctionDeclaration-parser-syntax.md",
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
            start: 41,
            end: 46,
        },
    },
    SpannedToken {
        kind: Ident(
            "c1",
        ),
        span: Span {
            start: 47,
            end: 49,
        },
    },
    SpannedToken {
        kind: LeftBrace,
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
            start: 52,
            end: 58,
        },
    },
    SpannedToken {
        kind: Ident(
            "foo",
        ),
        span: Span {
            start: 59,
            end: 62,
        },
    },
    SpannedToken {
        kind: LeftParen,
        span: Span {
            start: 62,
            end: 63,
        },
    },
    SpannedToken {
        kind: RightParen,
        span: Span {
            start: 63,
            end: 64,
        },
    },
    SpannedToken {
        kind: LeftBrace,
        span: Span {
            start: 65,
            end: 66,
        },
    },
    SpannedToken {
        kind: RightBrace,
        span: Span {
            start: 67,
            end: 68,
        },
    },
    SpannedToken {
        kind: RightBrace,
        span: Span {
            start: 69,
            end: 70,
        },
    },
    SpannedToken {
        kind: Var,
        span: Span {
            start: 72,
            end: 75,
        },
    },
    SpannedToken {
        kind: Ident(
            "c1",
        ),
        span: Span {
            start: 76,
            end: 78,
        },
    },
    SpannedToken {
        kind: Equal,
        span: Span {
            start: 79,
            end: 80,
        },
    },
    SpannedToken {
        kind: Number(
            1,
        ),
        span: Span {
            start: 81,
            end: 82,
        },
    },
    SpannedToken {
        kind: Semicolon,
        span: Span {
            start: 82,
            end: 83,
        },
    },
    SpannedToken {
        kind: Class,
        span: Span {
            start: 118,
            end: 123,
        },
    },
    SpannedToken {
        kind: Ident(
            "c4",
        ),
        span: Span {
            start: 124,
            end: 126,
        },
    },
    SpannedToken {
        kind: LeftBrace,
        span: Span {
            start: 127,
            end: 128,
        },
    },
    SpannedToken {
        kind: Ident(
            "public",
        ),
        span: Span {
            start: 129,
            end: 135,
        },
    },
    SpannedToken {
        kind: Ident(
            "foo",
        ),
        span: Span {
            start: 136,
            end: 139,
        },
    },
    SpannedToken {
        kind: LeftParen,
        span: Span {
            start: 139,
            end: 140,
        },
    },
    SpannedToken {
        kind: RightParen,
        span: Span {
            start: 140,
            end: 141,
        },
    },
    SpannedToken {
        kind: LeftBrace,
        span: Span {
            start: 142,
            end: 143,
        },
    },
    SpannedToken {
        kind: RightBrace,
        span: Span {
            start: 144,
            end: 145,
        },
    },
    SpannedToken {
        kind: RightBrace,
        span: Span {
            start: 146,
            end: 147,
        },
    },
    SpannedToken {
        kind: Ident(
            "enum",
        ),
        span: Span {
```

#### ast

- ok: `False`
- truncated: `False`

```text
error: [UnsupportedSyntax] expected LeftParen, got Some(Ident("foo")) at 59..62
```

#### resolved

- ok: `False`
- truncated: `False`

```text
error: [UnsupportedSyntax] expected LeftParen, got Some(Ident("foo")) at 59..62
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
        "code": 2300,
        "category": "Error",
        "message": "Duplicate identifier 'c1'.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/augmentedTypesClass.ts",
        "start": 47,
        "length": 2,
        "line": 3,
        "character": 7
      },
      {
        "code": 2300,
        "category": "Error",
        "message": "Duplicate identifier 'c1'.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/augmentedTypesClass.ts",
        "start": 76,
        "length": 2,
        "line": 4,
        "character": 5
      },
      {
        "code": 2567,
        "category": "Error",
        "message": "Enum declarations can only merge with namespace or other enum declarations.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/augmentedTypesClass.ts",
        "start": 124,
        "length": 2,
        "line": 7,
        "character": 7
      },
      {
        "code": 2567,
        "category": "Error",
        "message": "Enum declarations can only merge with namespace or other enum declarations.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/augmentedTypesClass.ts",
        "start": 154,
        "length": 2,
        "line": 8,
        "character": 6
      }
    ],
    "hints": [
      {
        "kind": "binding",
        "typeText": "number",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/augmentedTypesClass.ts",
        "start": 76,
        "length": 2,
        "line": 4,
        "character": 5,
        "name": "c1"
      }
    ],
    "typescriptVersion": "6.0.3"
  },
  "ast": {
    "topLevel": [
      {
        "kind": "ClassDeclaration",
        "text": "class c1 { public foo() { } }",
        "line": 3,
        "character": 1
      },
      {
        "kind": "FirstStatement",
        "text": "var c1 = 1;",
        "line": 4,
        "character": 1
      },
      {
        "kind": "ClassDeclaration",
        "text": "class c4 { public foo() { } }",
        "line": 7,
        "character": 1
      },
      {
        "kind": "EnumDeclaration",
        "text": "enum c4 { One }",
        "line": 8,
        "character": 1
      }
    ],
    "pathToPosition": [
      {
        "kind": "SourceFile",
        "text": "class c1 { public foo() { } }\r\nvar c1 = 1; // error\r\n\r\n//// class then enum\r\nclass c4 { public foo() { } }\r\nenum c4 { On",
        "line": 3,
        "character": 1
      },
      {
        "kind": "ClassDeclaration",
        "text": "class c1 { public foo() { } }",
        "line": 3,
        "character": 1
      },
      {
        "kind": "MethodDeclaration",
        "text": "public foo() { }",
        "line": 3,
        "character": 12
      },
      {
        "kind": "Identifier",
        "text": "foo",
        "line": 3,
        "character": 19
      }
    ]
  }
}
```

Stack trace:

```text
error: [UnsupportedSyntax] expected LeftParen, got Some(Ident("foo")) at 59..62
```

## Completion evidence

Commits:

- this commit

Validation result:

```text
command: env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/augmentedTypesClass.ts
result: pass; current blocker is DuplicateLocal for class c1 / var c1, split to issue 5347
date: 2026-05-07

command: env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/augmentedTypesClass.ts --detail --no-dashboard-data
result: pass; executed=1, unsupported=1, DuplicateLocal=1
date: 2026-05-07
```

Remaining risks:

- Issue 5347 still needs implementation; later class/enum merge diagnostics remain after this first blocker advances.
