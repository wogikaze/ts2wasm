---
id: 473
title: "Implement Abstractpropertynegative (dup)"
type: spike
area: frontend/syntax
class: superseded
priority: P1
depends_on: [5000]
blocks: []
created: 2026-05-01
updated: 2026-05-04
---

## Summary

Triage abstractPropertyNegative across 1 failing reference test cases and split this bucket into implementation-ready child issues.

## Problem

Reference test results show 1 cases fail in directory `abstractPropertyNegative` with diagnostics: parser-syntax. The compiler cannot handle these syntax/semantics, preventing compilation of code in this category.

Problem: abstractPropertyNegative has 1 reference failures and needs smart-triage evidence before implementation starts.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/abstractPropertyNegative.ts
```

Coverage window:

```sh
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/abstractPropertyNegative.ts --detail
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
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/abstractPropertyNegative.ts --detail
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/abstractPropertyNegative.ts
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

- `reference/typescript/tests/cases/compiler/abstractPropertyNegative.ts`

## Duplicate detection

- `issues/done/088-implement-abstractPropertyNegative.md` - Implement Abstractpropertynegative (same reference path, same feature label, same group key, title overlap)

## Smart triage

### Smart triage: Triage parser syntax: abstractPropertyNegative

- Issue class: `triage-needed`
- Feature label: `parser-syntax`
- Diagnostic: `UnsupportedSyntax` / `parser-or-frontend-unsupported`
- Path: `reference/typescript/tests/cases/compiler/abstractPropertyNegative.ts`

Reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/abstractPropertyNegative.ts
```

Source overview:

```json
{
  "suite": "tsc",
  "bytes": 1104,
  "lines": 43,
  "extension": ".ts",
  "first_code_line": "interface A {"
}
```

Failure location:

```json
{
  "code": "UnsupportedSyntax",
  "message": "expected LeftBrace, got Some(Ident(\"implements\")) at 96..106",
  "span_start": 96,
  "span_end": 106,
  "line": 6,
  "column": 23,
  "feature_label": "parser-syntax",
  "error_type": "parser-or-frontend-unsupported"
}
```

Source context:

```text
3 |     prop: string;
4 |     m(): string;
5 | }
6 | abstract class B implements A {
7 |     abstract prop: string;
8 |     public abstract readonly ro: string;
9 |     abstract get readonlyProp(): string;
```

Visible symbols before failure:

```json
[
  {
    "kind": "class",
    "name": "B",
    "line": 6,
    "column": 10
  }
]
```

Duplicate candidates:

```json
[
  {
    "state": "open",
    "path": "issues/done/088-implement-abstractPropertyNegative.md",
    "title": "Implement Abstractpropertynegative",
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
    "path": "issues/done/464-implement-FunctionDeclaration-parser-syntax.md",
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
            "m",
        ),
        span: Span {
            start: 62,
            end: 63,
        },
    },
    SpannedToken {
        kind: LeftParen,
        span: Span {
            start: 63,
            end: 64,
        },
    },
    SpannedToken {
        kind: RightParen,
        span: Span {
            start: 64,
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
        kind: RightBrace,
        span: Span {
            start: 76,
            end: 77,
        },
    },
    SpannedToken {
        kind: Ident(
            "abstract",
        ),
        span: Span {
            start: 79,
            end: 87,
        },
    },
    SpannedToken {
        kind: Class,
        span: Span {
            start: 88,
            end: 93,
        },
    },
    SpannedToken {
        kind: Ident(
            "B",
        ),
        span: Span {
            start: 94,
            end: 95,
        },
    },
    SpannedToken {
        kind: Ident(
            "implements",
        ),
        span: Span {
            start: 96,
            end: 106,
        },
    },
    SpannedToken {
        kind: Ident(
            "A",
        ),
        span: Span {
            start: 107,
            end: 108,
        },
    },
    SpannedToken {
        kind: LeftBrace,
        span: Span {
            start: 109,
            end: 110,
        },
    },
    SpannedToken {
        kind: Ident(
            "abstract",
        ),
        span: Span {
            start: 116,
            end: 124,
        },
    },
    SpannedToken {
        kind: Ident(
            "prop",
        ),
        span: Span {
            start: 125,
            end: 129,
        },
    },
    SpannedToken {
        kind: Colon,
        span: Span {
            start: 129,
            end: 130,
        },
    },
    SpannedToken {
        kind: Ident(
            "string",
        ),
        span: Span {
            start: 131,
            end: 137,
        },
    },
    SpannedToken {
        kind: Semicolon,
        span: Span {
```

#### ast

- ok: `False`
- truncated: `False`

```text
error: [UnsupportedSyntax] expected LeftBrace, got Some(Ident("implements")) at 96..106
```

#### resolved

- ok: `False`
- truncated: `False`

```text
error: [UnsupportedSyntax] expected LeftBrace, got Some(Ident("implements")) at 96..106
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
        "code": 2654,
        "category": "Error",
        "message": "Non-abstract class 'C' is missing implementations for the following members of 'B': 'prop', 'readonlyProp', 'm', 'mismatch'.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/abstractPropertyNegative.ts",
        "start": 339,
        "length": 1,
        "line": 14,
        "character": 7
      },
      {
        "code": 1253,
        "category": "Error",
        "message": "Abstract properties can only appear within an abstract class.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/abstractPropertyNegative.ts",
        "start": 396,
        "length": 8,
        "line": 16,
        "character": 5
      },
      {
        "code": 1005,
        "category": "Error",
        "message": "'{' expected.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/abstractPropertyNegative.ts",
        "start": 462,
        "length": 1,
        "line": 17,
        "character": 37
      },
      {
        "code": 2540,
        "category": "Error",
        "message": "Cannot assign to 'ro' because it is a read-only property.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/abstractPropertyNegative.ts",
        "start": 488,
        "length": 2,
        "line": 20,
        "character": 3
      },
      {
        "code": 2416,
        "category": "Error",
        "message": "Property 'num' in type 'WrongTypePropertyImpl' is not assignable to the same property in base type 'WrongTypeProperty'.\n  Type 'string' is not assignable to type 'number'.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/abstractPropertyNegative.ts",
        "start": 669,
        "length": 3,
        "line": 26,
        "character": 5
      },
      {
        "code": 2416,
        "category": "Error",
        "message": "Property 'num' in type 'WrongTypeAccessorImpl' is not assignable to the same property in base type 'WrongTypeAccessor'.\n  Type 'string' is not assignable to type 'number'.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/abstractPropertyNegative.ts",
        "start": 831,
        "length": 3,
        "line": 32,
        "character": 9
      },
      {
        "code": 2416,
        "category": "Error",
        "message": "Property 'num' in type 'WrongTypeAccessorImpl2' is not assignable to the same property in base type 'WrongTypeAccessor'.\n  Type 'string' is not assignable to type 'number'.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/abstractPropertyNegative.ts",
        "start": 929,
        "length": 3,
        "line": 35,
        "character": 5
      },
      {
        "code": 2676,
        "category": "Error",
        "message": "Accessors must both be abstract or non-abstract.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/abstractPropertyNegative.ts",
        "start": 1016,
        "length": 2,
        "line": 39,
        "character": 18
      },
      {
        "code": 2676,
        "category": "Error",
        "message": "Accessors must both be abstract or non-abstract.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/abstractPropertyNegative.ts",
        "start": 1039,
        "length": 2,
        "line": 40,
        "character": 9
      },
      {
        "code": 2676,
        "category": "Error",
        "message": "Accessors must both be abstract or non-abstract.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/abstractPropertyNegative.ts",
        "start": 1069,
        "length": 2,
        "line": 41,
        "character": 9
      },
      {
        "code": 2676,
        "category": "Error",
        "message": "Accessors must both be abstract or non-abstract.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/abstractPropertyNegative.ts",
        "start": 1126,
        "length": 2,
        "line": 42,
        "character": 18
      }
    ],
    "hints": [
      {
        "kind": "parameter",
        "typeText": "number",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/abstractPropertyNegative.ts",
        "start": 315,
        "length": 3,
        "line": 12,
        "character": 27,
        "name": "val"
      },
      {
        "kind": "binding",
        "typeText": "C",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/abstractPropertyNegative.ts",
        "start": 472,
        "length": 1,
        "line": 19,
        "character": 5,
        "name": "c"
      },
      {
        "kind": "parameter",
        "typeText": "string",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/abstractPropertyNegative.ts",
        "start": 1042,
        "length": 3,
        "line": 40,
        "character": 12,
        "name": "val"
      },
      {
        "kind": "parameter",
        "typeText": "string",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/abstractPropertyNegative.ts",
        "start": 1129,
        "length": 3,
        "line": 42,
        "character": 21,
        "name": "val"
      }
    ],
    "typescriptVersion": "6.0.3"
  },
  "ast": {
    "topLevel": [
      {
        "kind": "InterfaceDeclaration",
        "text": "interface A {\r\n    prop: string;\r\n    m(): string;\r\n}",
        "line": 2,
        "character": 1
      },
      {
        "kind": "ClassDeclaration",
        "text": "abstract class B implements A {\r\n    abstract prop: string;\r\n    public abstract readonly ro: string;\r\n    abstract get ",
        "line": 6,
        "character": 1
      },
      {
        "kind": "ClassDeclaration",
        "text": "class C extends B {\r\n    readonly ro = \"readonly please\";\r\n    abstract notAllowed: string;\r\n    get concreteWithNoBody(",
        "line": 14,
        "character": 1
      },
      {
        "kind": "FirstStatement",
        "text": "let c = new C();",
        "line": 19,
        "character": 1
      },
      {
        "kind": "ExpressionStatement",
        "text": "c.ro = \"error: lhs of assignment can't be readonly\";",
        "line": 20,
        "character": 1
      },
      {
        "kind": "ClassDeclaration",
        "text": "abstract class WrongTypeProperty {\r\n    abstract num: number;\r\n}",
        "line": 22,
        "character": 1
      },
      {
        "kind": "ClassDeclaration",
        "text": "class WrongTypePropertyImpl extends WrongTypeProperty {\r\n    num = \"nope, wrong\";\r\n}",
        "line": 25,
        "character": 1
      },
      {
        "kind": "ClassDeclaration",
        "text": "abstract class WrongTypeAccessor {\r\n    abstract get num(): number;\r\n}",
        "line": 28,
        "character": 1
      },
      {
        "kind": "ClassDeclaration",
        "text": "class WrongTypeAccessorImpl extends WrongTypeAccessor {\r\n    get num() { return \"nope, wrong\"; }\r\n}",
        "line": 31,
        "character": 1
      },
      {
        "kind": "ClassDeclaration",
        "text": "class WrongTypeAccessorImpl2 extends WrongTypeAccessor {\r\n    num = \"nope, wrong\";\r\n}",
        "line": 34,
        "character": 1
      },
      {
        "kind": "ClassDeclaration",
        "text": "abstract class AbstractAccessorMismatch {\r\n    abstract get p1(): string;\r\n    set p1(val: string) { };\r\n    get p2(): s",
        "line": 38,
        "character": 1
      }
    ],
    "pathToPosition": [
      {
        "kind": "SourceFile",
        "text": "interface A {\r\n    prop: string;\r\n    m(): string;\r\n}\r\nabstract class B implements A {\r\n    abs
```

Stack trace:

```text
error: [UnsupportedSyntax] expected LeftBrace, got Some(Ident("implements")) at 96..106
```


## 重複整理メモ

この issue はタイトルが重複しているため、内容がより充実している
`issues/open/559-implement-abstractPropertyNegative.md` に統合されました。
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
- `issues/done/473-implement-abstractPropertyNegative.md`
- `issues/index.md` after regeneration

Future-work tracking: no untracked future-work item was identified in this issue during this metadata/evidence audit.
