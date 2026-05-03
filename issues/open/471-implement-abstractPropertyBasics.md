---
id: 471
title: "Implement Abstractpropertybasics"
type: spike
area: frontend/syntax
class: blocked
priority: P1
depends_on: [5000]
blocks: []
created: 2026-05-01
updated: 2026-05-01
---

## Summary

Triage abstractPropertyBasics across 1 failing reference test cases and split this bucket into implementation-ready child issues.

## Problem

Reference test results show 1 cases fail in directory `abstractPropertyBasics` with diagnostics: parser-syntax. The compiler cannot handle these syntax/semantics, preventing compilation of code in this category.

Problem: abstractPropertyBasics has 1 reference failures and needs smart-triage evidence before implementation starts.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/abstractPropertyBasics.ts
```

Coverage window:

```sh
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/abstractPropertyBasics.ts --detail
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
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/abstractPropertyBasics.ts --detail
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/abstractPropertyBasics.ts
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

- `reference/typescript/tests/cases/compiler/abstractPropertyBasics.ts`

## Duplicate detection

- `issues/open/086-implement-abstractPropertyBasics.md` - Implement Abstractpropertybasics (same reference path, same feature label, same group key, title overlap)
- `issues/open/442-implement-parser-syntax.md` - Implement parser syntax extensions (same feature label, same group key, title overlap)

## Smart triage

### Smart triage: Triage parser syntax: abstractPropertyBasics

- Issue class: `triage-needed`
- Feature label: `parser-syntax`
- Diagnostic: `UnsupportedSyntax` / `parser-or-frontend-unsupported`
- Path: `reference/typescript/tests/cases/compiler/abstractPropertyBasics.ts`

Reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/abstractPropertyBasics.ts
```

Source overview:

```json
{
  "suite": "tsc",
  "bytes": 487,
  "lines": 22,
  "extension": ".ts",
  "first_code_line": "interface A {"
}
```

Failure location:

```json
{
  "code": "UnsupportedSyntax",
  "message": "expected LeftBrace, got Some(Ident(\"implements\")) at 112..122",
  "span_start": 112,
  "span_end": 122,
  "line": 7,
  "column": 24,
  "feature_label": "parser-syntax",
  "error_type": "parser-or-frontend-unsupported"
}
```

Source context:

```text
 4 |     raw: string;
 5 |     m(): void;
 6 | }
 7 | abstract class B implements A {
 8 |     abstract prop: string;
 9 |     abstract raw: string;
10 |     abstract readonly ro: string;
```

Visible symbols before failure:

```json
[
  {
    "kind": "class",
    "name": "B",
    "line": 7,
    "column": 10
  }
]
```

Duplicate candidates:

```json
[
  {
    "state": "open",
    "path": "issues/open/086-implement-abstractPropertyBasics.md",
    "title": "Implement Abstractpropertybasics",
    "reason": "same reference path, same feature label, title overlap"
  },
  {
    "state": "open",
    "path": "issues/open/442-implement-parser-syntax.md",
    "title": "Implement parser syntax extensions",
    "reason": "same feature label, title overlap"
  },
  {
    "state": "open",
    "path": "issues/open/464-implement-FunctionDeclaration-parser-syntax.md",
    "title": "Implement Functiondeclaration Parser Syntax",
    "reason": "same feature label, title overlap"
  },
  {
    "state": "done",
    "path": "issues/done/059-implement-parser-syntax-extensions.md",
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
    "path": "issues/open/200-implement-parser-syntax.md",
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
  },
  {
    "state": "done",
    "path": "issues/done/246-implement-optional-chaining-parser-support.md",
    "title": "Implement optional chaining parser support",
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
            start: 24,
            end: 33,
        },
    },
    SpannedToken {
        kind: Ident(
            "A",
        ),
        span: Span {
            start: 34,
            end: 35,
        },
    },
    SpannedToken {
        kind: LeftBrace,
        span: Span {
            start: 36,
            end: 37,
        },
    },
    SpannedToken {
        kind: Ident(
            "prop",
        ),
        span: Span {
            start: 43,
            end: 47,
        },
    },
    SpannedToken {
        kind: Colon,
        span: Span {
            start: 47,
            end: 48,
        },
    },
    SpannedToken {
        kind: Ident(
            "string",
        ),
        span: Span {
            start: 49,
            end: 55,
        },
    },
    SpannedToken {
        kind: Semicolon,
        span: Span {
            start: 55,
            end: 56,
        },
    },
    SpannedToken {
        kind: Ident(
            "raw",
        ),
        span: Span {
            start: 62,
            end: 65,
        },
    },
    SpannedToken {
        kind: Colon,
        span: Span {
            start: 65,
            end: 66,
        },
    },
    SpannedToken {
        kind: Ident(
            "string",
        ),
        span: Span {
            start: 67,
            end: 73,
        },
    },
    SpannedToken {
        kind: Semicolon,
        span: Span {
            start: 73,
            end: 74,
        },
    },
    SpannedToken {
        kind: Ident(
            "m",
        ),
        span: Span {
            start: 80,
            end: 81,
        },
    },
    SpannedToken {
        kind: LeftParen,
        span: Span {
            start: 81,
            end: 82,
        },
    },
    SpannedToken {
        kind: RightParen,
        span: Span {
            start: 82,
            end: 83,
        },
    },
    SpannedToken {
        kind: Colon,
        span: Span {
            start: 83,
            end: 84,
        },
    },
    SpannedToken {
        kind: Void,
        span: Span {
            start: 85,
            end: 89,
        },
    },
    SpannedToken {
        kind: Semicolon,
        span: Span {
            start: 89,
            end: 90,
        },
    },
    SpannedToken {
        kind: RightBrace,
        span: Span {
            start: 92,
            end: 93,
        },
    },
    SpannedToken {
        kind: Ident(
            "abstract",
        ),
        span: Span {
            start: 95,
            end: 103,
        },
    },
    SpannedToken {
        kind: Class,
        span: Span {
            start: 104,
            end: 109,
        },
    },
    SpannedToken {
        kind: Ident(
            "B",
        ),
        span: Span {
            start: 110,
            end: 111,
        },
    },
    SpannedToken {
        kind: Ident(
            "implements",
        ),
        span: Span {
            start: 112,
            end: 122,
        },
    },
    SpannedToken {
        kind: Ident(
            "A",
        ),
        span: Span {
            start: 123,
            end: 124,
        },
    },
    SpannedToken {
        kind: LeftBrace,
        span: Span {
            start: 125,
            end: 126,
        },
    },
    SpannedToken {
        kind: Ident(
            "abstract",
        ),
        span: Span {
            start: 132,
            end: 140,
```

#### ast

- ok: `False`
- truncated: `False`

```text
error: [UnsupportedSyntax] expected LeftBrace, got Some(Ident("implements")) at 112..122
```

#### resolved

- ok: `False`
- truncated: `False`

```text
error: [UnsupportedSyntax] expected LeftBrace, got Some(Ident("implements")) at 112..122
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
        "typeText": "string",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/abstractPropertyBasics.ts",
        "start": 290,
        "length": 3,
        "line": 12,
        "character": 31,
        "name": "val"
      },
      {
        "kind": "parameter",
        "typeText": "string",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/abstractPropertyBasics.ts",
        "start": 401,
        "length": 1,
        "line": 17,
        "character": 14,
        "name": "v"
      }
    ],
    "typescriptVersion": "6.0.3"
  },
  "ast": {
    "topLevel": [
      {
        "kind": "InterfaceDeclaration",
        "text": "interface A {\r\n    prop: string;\r\n    raw: string;\r\n    m(): void;\r\n}",
        "line": 2,
        "character": 1
      },
      {
        "kind": "ClassDeclaration",
        "text": "abstract class B implements A {\r\n    abstract prop: string;\r\n    abstract raw: string;\r\n    abstract readonly ro: string",
        "line": 7,
        "character": 1
      },
      {
        "kind": "ClassDeclaration",
        "text": "class C extends B {\r\n    get prop() { return \"foo\"; }\r\n    set prop(v) { }\r\n    raw = \"edge\";\r\n    readonly ro = \"readon",
        "line": 15,
        "character": 1
      }
    ],
    "pathToPosition": [
      {
        "kind": "SourceFile",
        "text": "interface A {\r\n    prop: string;\r\n    raw: string;\r\n    m(): void;\r\n}\r\nabstract class B implements A {\r\n    abstract pro",
        "line": 2,
        "character": 1
      },
      {
        "kind": "ClassDeclaration",
        "text": "abstract class B implements A {\r\n    abstract prop: string;\r\n    abstract raw: string;\r\n    abstract readonly ro: string",
        "line": 7,
        "character": 1
      },
      {
        "kind": "HeritageClause",
        "text": "implements A",
        "line": 7,
        "character": 18
      }
    ]
  }
}
```

Stack trace:

```text
error: [UnsupportedSyntax] expected LeftBrace, got Some(Ident("implements")) at 112..122
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
