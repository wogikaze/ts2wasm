---
id: 496
title: "Implement Aliasinstantiationexpressiongenericintersectionnocrash"
type: spike
area: frontend/syntax
class: blocked
priority: P1
depends_on: [5001]
blocks: []
created: 2026-05-01
updated: 2026-05-01
---

## Summary

Triage aliasInstantiationExpressionGenericIntersectionNoCrash across 2 failing reference test cases and split this bucket into implementation-ready child issues.

## Problem

Reference test results show 2 cases fail in directory `aliasInstantiationExpressionGenericIntersectionNoCrash` with diagnostics: type-alias. The compiler cannot handle these syntax/semantics, preventing compilation of code in this category.

Problem: aliasInstantiationExpressionGenericIntersectionNoCrash has 2 reference failures and needs smart-triage evidence before implementation starts.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/aliasInstantiationExpressionGenericIntersectionNoCrash1.ts
```

Coverage window:

```sh
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/aliasInstantiationExpressionGenericIntersectionNoCrash1.ts --detail
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
mise run reference-coverage -- tsc --limit 4
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/aliasInstantiationExpressionGenericIntersectionNoCrash1.ts --detail
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/aliasInstantiationExpressionGenericIntersectionNoCrash1.ts
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

- `reference/typescript/tests/cases/compiler/aliasInstantiationExpressionGenericIntersectionNoCrash1.ts`
- `reference/typescript/tests/cases/compiler/aliasInstantiationExpressionGenericIntersectionNoCrash2.ts`

## Duplicate detection

- `issues/open/116-implement-aliasInstantiationExpressionGenericIntersectionNoCrash.md` - Implement Aliasinstantiationexpressiongenericintersectionnocrash (same reference path, same feature label, same group key, title overlap)
- `issues/open/452-implement-type-alias.md` - Implement type-alias support (same feature label, same group key, title overlap)

## Smart triage

### Smart triage: Triage type alias: aliasInstantiationExpressionGenericIntersectionNoCrash1

- Issue class: `triage-needed`
- Feature label: `type-alias`
- Diagnostic: `UnsupportedSyntax` / `parser-or-frontend-unsupported`
- Path: `reference/typescript/tests/cases/compiler/aliasInstantiationExpressionGenericIntersectionNoCrash1.ts`

Reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/aliasInstantiationExpressionGenericIntersectionNoCrash1.ts
```

Source overview:

```json
{
  "suite": "tsc",
  "bytes": 212,
  "lines": 13,
  "extension": ".ts",
  "first_code_line": "class ErrImpl<E> {"
}
```

Failure location:

```json
{
  "code": "UnsupportedSyntax",
  "message": "expected LeftBrace, got Some(Less) at 50..51",
  "span_start": 50,
  "span_end": 51,
  "line": 4,
  "column": 14,
  "feature_label": "type-alias",
  "error_type": "parser-or-frontend-unsupported"
}
```

Source context:

```text
1 | // @target: es2015
2 | // @strict: true
3 | 
4 | class ErrImpl<E> {
5 |   e!: E;
6 | }
7 |
```

Visible symbols before failure:

```json
[
  {
    "kind": "class",
    "name": "ErrImpl",
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
    "path": "issues/open/116-implement-aliasInstantiationExpressionGenericIntersectionNoCrash.md",
    "title": "Implement Aliasinstantiationexpressiongenericintersectionnocrash",
    "reason": "same reference path, same feature label"
  },
  {
    "state": "open",
    "path": "issues/open/345-implement-tsc-type-alias-coverage.md",
    "title": "Implement TypeScript type alias coverage for tsc suite (23 cases)",
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
            start: 37,
            end: 42,
        },
    },
    SpannedToken {
        kind: Ident(
            "ErrImpl",
        ),
        span: Span {
            start: 43,
            end: 50,
        },
    },
    SpannedToken {
        kind: Less,
        span: Span {
            start: 50,
            end: 51,
        },
    },
    SpannedToken {
        kind: Ident(
            "E",
        ),
        span: Span {
            start: 51,
            end: 52,
        },
    },
    SpannedToken {
        kind: Greater,
        span: Span {
            start: 52,
            end: 53,
        },
    },
    SpannedToken {
        kind: LeftBrace,
        span: Span {
            start: 54,
            end: 55,
        },
    },
    SpannedToken {
        kind: Ident(
            "e",
        ),
        span: Span {
            start: 58,
            end: 59,
        },
    },
    SpannedToken {
        kind: Bang,
        span: Span {
            start: 59,
            end: 60,
        },
    },
    SpannedToken {
        kind: Colon,
        span: Span {
            start: 60,
            end: 61,
        },
    },
    SpannedToken {
        kind: Ident(
            "E",
        ),
        span: Span {
            start: 62,
            end: 63,
        },
    },
    SpannedToken {
        kind: Semicolon,
        span: Span {
            start: 63,
            end: 64,
        },
    },
    SpannedToken {
        kind: RightBrace,
        span: Span {
            start: 65,
            end: 66,
        },
    },
    SpannedToken {
        kind: Ident(
            "declare",
        ),
        span: Span {
            start: 68,
            end: 75,
        },
    },
    SpannedToken {
        kind: Const,
        span: Span {
            start: 76,
            end: 81,
        },
    },
    SpannedToken {
        kind: Ident(
            "Err",
        ),
        span: Span {
            start: 82,
            end: 85,
        },
    },
    SpannedToken {
        kind: Colon,
        span: Span {
            start: 85,
            end: 86,
        },
    },
    SpannedToken {
        kind: TypeOf,
        span: Span {
            start: 87,
            end: 93,
        },
    },
    SpannedToken {
        kind: Ident(
            "ErrImpl",
        ),
        span: Span {
            start: 94,
            end: 101,
        },
    },
    SpannedToken {
        kind: Ampersand,
        span: Span {
            start: 102,
            end: 103,
        },
    },
    SpannedToken {
        kind: LeftParen,
        span: Span {
            start: 104,
            end: 105,
        },
    },
    SpannedToken {
        kind: Less,
        span: Span {
            start: 105,
            end: 106,
        },
    },
    SpannedToken {
        kind: Ident(
            "T",
        ),
        span: Span {
            start: 106,
            end: 107,
        },
    },
    SpannedToken {
        kind: Greater,
        span: Span {
            start: 107,
            end: 108,
        },
    },
    SpannedToken {
        kind: LeftParen,
        span: Span {
            start: 108,
            end: 109,
        },
    },
    SpannedToken {
        kind: RightParen,
        span: Span {
            start: 109,
            end: 110,
        },
    },
    SpannedToken {
        kind: Arrow,
        span: Span {
            start: 111,
            end: 113,
        },
```

#### ast

- ok: `False`
- truncated: `False`

```text
error: [UnsupportedSyntax] expected LeftBrace, got Some(Less) at 50..51
```

#### resolved

- ok: `False`
- truncated: `False`

```text
error: [UnsupportedSyntax] expected LeftBrace, got Some(Less) at 50..51
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
        "code": 2352,
        "category": "Error",
        "message": "Conversion of type '{ new (): ErrImpl<number>; prototype: ErrImpl<any>; } & (() => number)' to type '{ new (): ErrImpl<string>; prototype: ErrImpl<any>; } & (() => string)' may be a mistake because neither type sufficiently overlaps with the other. If this was intentional, convert the expression to 'unknown' first.\n  Type '{ new (): ErrImpl<number>; prototype: ErrImpl<any>; } & (() => number)' is not comparable to type '{ new (): ErrImpl<string>; prototype: ErrImpl<any>; }'.\n    Type 'ErrImpl<number>' is not comparable to type 'ErrImpl<string>'.\n      Type 'number' is not comparable to type 'string'.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/aliasInstantiationExpressionGenericIntersectionNoCrash1.ts",
        "start": 189,
        "length": 21,
        "line": 13,
        "character": 1
      }
    ],
    "hints": [
      {
        "kind": "binding",
        "typeText": "typeof ErrImpl & (<T>() => T)",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/aliasInstantiationExpressionGenericIntersectionNoCrash1.ts",
        "start": 82,
        "length": 3,
        "line": 8,
        "character": 15,
        "name": "Err"
      },
      {
        "kind": "binding",
        "typeText": "{ new (): ErrImpl<number>; prototype: ErrImpl<any>; } & (() => number)",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/aliasInstantiationExpressionGenericIntersectionNoCrash1.ts",
        "start": 168,
        "length": 1,
        "line": 12,
        "character": 15,
        "name": "e"
      }
    ],
    "typescriptVersion": "6.0.3"
  },
  "ast": {
    "topLevel": [
      {
        "kind": "ClassDeclaration",
        "text": "class ErrImpl<E> {\n  e!: E;\n}",
        "line": 4,
        "character": 1
      },
      {
        "kind": "FirstStatement",
        "text": "declare const Err: typeof ErrImpl & (<T>() => T);",
        "line": 8,
        "character": 1
      },
      {
        "kind": "TypeAliasDeclaration",
        "text": "type ErrAlias<U> = typeof Err<U>;",
        "line": 10,
        "character": 1
      },
      {
        "kind": "FirstStatement",
        "text": "declare const e: ErrAlias<number>;",
        "line": 12,
        "character": 1
      },
      {
        "kind": "ExpressionStatement",
        "text": "e as ErrAlias<string>;",
        "line": 13,
        "character": 1
      }
    ],
    "pathToPosition": [
      {
        "kind": "SourceFile",
        "text": "class ErrImpl<E> {\n  e!: E;\n}\n\ndeclare const Err: typeof ErrImpl & (<T>() => T);\n\ntype ErrAlias<U> = typeof Err<U>;\n\ndec",
        "line": 4,
        "character": 1
      },
      {
        "kind": "ClassDeclaration",
        "text": "class ErrImpl<E> {\n  e!: E;\n}",
        "line": 4,
        "character": 1
      },
      {
        "kind": "Identifier",
        "text": "ErrImpl",
        "line": 4,
        "character": 7
      }
    ]
  }
}
```

Stack trace:

```text
error: [UnsupportedSyntax] expected LeftBrace, got Some(Less) at 50..51
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
