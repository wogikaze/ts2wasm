---
id: 689
title: "Implement Arrayofsubtypeisassignabletoreadonlyarray"
type: spike
area: frontend/syntax
class: triage-needed
priority: P1
depends_on: []
blocks: []
created: 2026-05-01
updated: 2026-05-01
---

## Summary

Triage arrayOfSubtypeIsAssignableToReadonlyArray across 1 failing reference test cases and split this bucket into implementation-ready child issues.

## Problem

Reference test results show 1 cases fail in directory `arrayOfSubtypeIsAssignableToReadonlyArray` with diagnostics: parser-syntax. The compiler cannot handle these syntax/semantics, preventing compilation of code in this category.

Problem: arrayOfSubtypeIsAssignableToReadonlyArray has 1 reference failures and needs smart-triage evidence before implementation starts.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/arrayOfSubtypeIsAssignableToReadonlyArray.ts
```

Coverage window:

```sh
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/arrayOfSubtypeIsAssignableToReadonlyArray.ts --detail
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
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/arrayOfSubtypeIsAssignableToReadonlyArray.ts --detail
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/arrayOfSubtypeIsAssignableToReadonlyArray.ts
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

- `reference/typescript/tests/cases/compiler/arrayOfSubtypeIsAssignableToReadonlyArray.ts`

## Duplicate detection

- none found by path/title/feature scan

## Smart triage

### Smart triage: Triage parser syntax: arrayOfSubtypeIsAssignableToReadonlyArray

- Issue class: `triage-needed`
- Feature label: `parser-syntax`
- Diagnostic: `UnsupportedSyntax` / `parser-or-frontend-unsupported`
- Path: `reference/typescript/tests/cases/compiler/arrayOfSubtypeIsAssignableToReadonlyArray.ts`

Reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/arrayOfSubtypeIsAssignableToReadonlyArray.ts
```

Source overview:

```json
{
  "suite": "tsc",
  "bytes": 532,
  "lines": 20,
  "extension": ".ts",
  "first_code_line": "class A { a }"
}
```

Failure location:

```json
{
  "code": "UnsupportedSyntax",
  "message": "expected LeftParen, got Some(RightBrace) at 49..50",
  "span_start": 49,
  "span_end": 50,
  "line": 3,
  "column": 13,
  "feature_label": "parser-syntax",
  "error_type": "parser-or-frontend-unsupported"
}
```

Source context:

```text
1 | // @target: es2015
2 | // @strict: false
3 | class A { a }
4 | class B extends A { b }
5 | class C<T> extends Array<T> { c }
6 | declare var ara: A[];
```

Visible symbols before failure:

```json
[
  {
    "kind": "class",
    "name": "A",
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
    "path": "issues/open/464-implement-FunctionDeclaration-parser-syntax.md",
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
        kind: Class,
        span: Span {
            start: 37,
            end: 42,
        },
    },
    SpannedToken {
        kind: Ident(
            "A",
        ),
        span: Span {
            start: 43,
            end: 44,
        },
    },
    SpannedToken {
        kind: LeftBrace,
        span: Span {
            start: 45,
            end: 46,
        },
    },
    SpannedToken {
        kind: Ident(
            "a",
        ),
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
            start: 51,
            end: 56,
        },
    },
    SpannedToken {
        kind: Ident(
            "B",
        ),
        span: Span {
            start: 57,
            end: 58,
        },
    },
    SpannedToken {
        kind: Extends,
        span: Span {
            start: 59,
            end: 66,
        },
    },
    SpannedToken {
        kind: Ident(
            "A",
        ),
        span: Span {
            start: 67,
            end: 68,
        },
    },
    SpannedToken {
        kind: LeftBrace,
        span: Span {
            start: 69,
            end: 70,
        },
    },
    SpannedToken {
        kind: Ident(
            "b",
        ),
        span: Span {
            start: 71,
            end: 72,
        },
    },
    SpannedToken {
        kind: RightBrace,
        span: Span {
            start: 73,
            end: 74,
        },
    },
    SpannedToken {
        kind: Class,
        span: Span {
            start: 75,
            end: 80,
        },
    },
    SpannedToken {
        kind: Ident(
            "C",
        ),
        span: Span {
            start: 81,
            end: 82,
        },
    },
    SpannedToken {
        kind: Less,
        span: Span {
            start: 82,
            end: 83,
        },
    },
    SpannedToken {
        kind: Ident(
            "T",
        ),
        span: Span {
            start: 83,
            end: 84,
        },
    },
    SpannedToken {
        kind: Greater,
        span: Span {
            start: 84,
            end: 85,
        },
    },
    SpannedToken {
        kind: Extends,
        span: Span {
            start: 86,
            end: 93,
        },
    },
    SpannedToken {
        kind: Ident(
            "Array",
        ),
        span: Span {
            start: 94,
            end: 99,
        },
    },
    SpannedToken {
        kind: Less,
        span: Span {
            start: 99,
            end: 100,
        },
    },
    SpannedToken {
        kind: Ident(
            "T",
        ),
        span: Span {
            start: 100,
            end: 101,
        },
    },
    SpannedToken {
        kind: Greater,
        span: Span {
            start: 101,
            end: 102,
        },
    },
    SpannedToken {
        kind: LeftBrace,
        span: Span {
            start: 103,
            end: 104,
        },
    },
    SpannedToken {
        kind: Ident(
            "c",
        ),
        span: Span {
            start: 105,
            end: 106,
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
            "declare",
        ),
```

#### ast

- ok: `False`
- truncated: `False`

```text
error: [UnsupportedSyntax] expected LeftParen, got Some(RightBrace) at 49..50
```

#### resolved

- ok: `False`
- truncated: `False`

```text
error: [UnsupportedSyntax] expected LeftParen, got Some(RightBrace) at 49..50
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
        "code": 2322,
        "category": "Error",
        "message": "Type 'A[]' is not assignable to type 'readonly B[]'.\n  Property 'b' is missing in type 'A' but required in type 'B'.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/arrayOfSubtypeIsAssignableToReadonlyArray.ts",
        "start": 352,
        "length": 3,
        "line": 15,
        "character": 1
      },
      {
        "code": 2322,
        "category": "Error",
        "message": "Type 'C<A>' is not assignable to type 'readonly B[]'.\n  The types returned by 'concat(...)' are incompatible between these types.\n    Type 'A[]' is not assignable to type 'B[]'.\n      Property 'b' is missing in type 'A' but required in type 'B'.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/arrayOfSubtypeIsAssignableToReadonlyArray.ts",
        "start": 482,
        "length": 3,
        "line": 20,
        "character": 1
      }
    ],
    "hints": [
      {
        "kind": "binding",
        "typeText": "A[]",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/arrayOfSubtypeIsAssignableToReadonlyArray.ts",
        "start": 121,
        "length": 3,
        "line": 6,
        "character": 13,
        "name": "ara"
      },
      {
        "kind": "binding",
        "typeText": "B[]",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/arrayOfSubtypeIsAssignableToReadonlyArray.ts",
        "start": 143,
        "length": 3,
        "line": 7,
        "character": 13,
        "name": "arb"
      },
      {
        "kind": "binding",
        "typeText": "C<A>",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/arrayOfSubtypeIsAssignableToReadonlyArray.ts",
        "start": 165,
        "length": 3,
        "line": 8,
        "character": 13,
        "name": "cra"
      },
      {
        "kind": "binding",
        "typeText": "C<B>",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/arrayOfSubtypeIsAssignableToReadonlyArray.ts",
        "start": 188,
        "length": 3,
        "line": 9,
        "character": 13,
        "name": "crb"
      },
      {
        "kind": "binding",
        "typeText": "readonly A[]",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/arrayOfSubtypeIsAssignableToReadonlyArray.ts",
        "start": 211,
        "length": 3,
        "line": 10,
        "character": 13,
        "name": "rra"
      },
      {
        "kind": "binding",
        "typeText": "readonly B[]",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/arrayOfSubtypeIsAssignableToReadonlyArray.ts",
        "start": 246,
        "length": 3,
        "line": 11,
        "character": 13,
        "name": "rrb"
      }
    ],
    "typescriptVersion": "6.0.3"
  },
  "ast": {
    "topLevel": [
      {
        "kind": "ClassDeclaration",
        "text": "class A { a }",
        "line": 3,
        "character": 1
      },
      {
        "kind": "ClassDeclaration",
        "text": "class B extends A { b }",
        "line": 4,
        "character": 1
      },
      {
        "kind": "ClassDeclaration",
        "text": "class C<T> extends Array<T> { c }",
        "line": 5,
        "character": 1
      },
      {
        "kind": "FirstStatement",
        "text": "declare var ara: A[];",
        "line": 6,
        "character": 1
      },
      {
        "kind": "FirstStatement",
        "text": "declare var arb: B[];",
        "line": 7,
        "character": 1
      },
      {
        "kind": "FirstStatement",
        "text": "declare var cra: C<A>;",
        "line": 8,
        "character": 1
      },
      {
        "kind": "FirstStatement",
        "text": "declare var crb: C<B>;",
        "line": 9,
        "character": 1
      },
      {
        "kind": "FirstStatement",
        "text": "declare var rra: ReadonlyArray<A>;",
        "line": 10,
        "character": 1
      },
      {
        "kind": "FirstStatement",
        "text": "declare var rrb: ReadonlyArray<B>;",
        "line": 11,
        "character": 1
      },
      {
        "kind": "ExpressionStatement",
        "text": "rra = ara;",
        "line": 12,
        "character": 1
      },
      {
        "kind": "ExpressionStatement",
        "text": "rrb = arb;",
        "line": 13,
        "character": 1
      },
      {
        "kind": "ExpressionStatement",
        "text": "rra = arb;",
        "line": 14,
        "character": 1
      },
      {
        "kind": "ExpressionStatement",
        "text": "rrb = ara;",
        "line": 15,
        "character": 1
      },
      {
        "kind": "ExpressionStatement",
        "text": "rra = cra;",
        "line": 17,
        "character": 1
      },
      {
        "kind": "ExpressionStatement",
        "text": "rra = crb;",
        "line": 18,
        "character": 1
      },
      {
        "kind": "ExpressionStatement",
        "text": "rrb = crb;",
        "line": 19,
        "character": 1
      },
      {
        "kind": "ExpressionStatement",
        "text": "rrb = cra;",
        "line": 20,
        "character": 1
      }
    ],
    "pathToPosition": [
      {
        "kind": "SourceFile",
        "text": "class A { a }\nclass B extends A { b }\nclass C<T> extends Array<T> { c }\ndeclare var ara: A[];\ndeclare var arb: B[];\ndecl",
        "line": 3,
        "character": 1
      },
      {
        "kind": "ClassDeclaration",
        "text": "class A { a }",
        "line": 3,
        "character": 1
      }
    ]
  }
}
```

Stack trace:

```text
error: [UnsupportedSyntax] expected LeftParen, got Some(RightBrace) at 49..50
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
