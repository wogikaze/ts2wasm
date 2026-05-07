---
id: 582
title: "Implement Aliasinstantiationexpressiongenericintersectionnocrash"
type: spike
area: frontend/syntax
class: done
priority: P1
depends_on: [5161]
blocks: []
created: 2026-05-01
updated: 2026-05-08
completed: 2026-05-08
---

## Summary

Triage aliasInstantiationExpressionGenericIntersectionNoCrash across 2 failing reference test cases and split this bucket into implementation-ready child issues.

## Problem

Reference test results showed 2 cases in
`aliasInstantiationExpressionGenericIntersectionNoCrash` with diagnostics:
type-alias. Fresh coverage and triage show the parser/type-alias boundary has
advanced; both files now fail because declaration-only ambient `declare const`
values are not resolver-visible when referenced in the following `as`
expression.

Problem: aliasInstantiationExpressionGenericIntersectionNoCrash had 2 generated
bucket failures and needed smart-triage evidence. No new child is needed because
issue 5161 already owns resolver-visible ambient value declarations.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/aliasInstantiationExpressionGenericIntersectionNoCrash2.ts
```

Coverage window:

```sh
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/aliasInstantiationExpressionGenericIntersectionNoCrash2.ts --detail
```

## Desired final state

This generated bucket is either split into implementation-ready child issues or superseded by an existing open/done issue with matching evidence. Do not implement directly from this bucket.

## Scope

In scope:

- [x] Inspect the smart triage report below
- [x] Confirm whether existing open/done issues already cover this bucket
- [x] Supersede this generated bucket with issue 5161
- [x] Preserve exact reproduction commands and representative AST/diagnostic evidence

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
- [x] Existing issue 5161 contains the implementation-ready ambient value declaration owner
- [x] This issue includes failing paths, diagnostic code, source context, visible symbols, and parser/TypeScript AST evidence
- [x] Coverage names the exact reference window and diagnostic classification

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run
```

Impacted commands:

```sh
mise run reference-coverage -- tsc --limit 4
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/aliasInstantiationExpressionGenericIntersectionNoCrash2.ts --detail
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/aliasInstantiationExpressionGenericIntersectionNoCrash2.ts
```

Not run:

- none

## Docs / current-state / issue sync

Final-state docs:

- [x] not affected

Current state:

- [x] not affected

Follow-up issues:

- [x] none

## Notes

## Affected test files

- `reference/typescript/tests/cases/compiler/aliasInstantiationExpressionGenericIntersectionNoCrash2.ts`
- `reference/typescript/tests/cases/compiler/aliasInstantiationExpressionGenericIntersectionNoCrash1.ts`

## Duplicate detection

- `issues/done/116-implement-aliasInstantiationExpressionGenericIntersectionNoCrash.md` - Implement Aliasinstantiationexpressiongenericintersectionnocrash (same reference path, same feature label, same group key, title overlap)
- `issues/open/452-implement-type-alias.md` - Implement type-alias support (same feature label, same group key, title overlap)
- `issues/done/496-implement-aliasInstantiationExpressionGenericIntersectionNoCrash.md` - Implement Aliasinstantiationexpressiongenericintersectionnocrash (same reference path, same feature label, same group key, title overlap)

## Smart triage

### Smart triage: Triage type alias: aliasInstantiationExpressionGenericIntersectionNoCrash2

- Issue class: `triage-needed`
- Feature label: `type-alias`
- Diagnostic: `UnsupportedSyntax` / `parser-or-frontend-unsupported`
- Path: `reference/typescript/tests/cases/compiler/aliasInstantiationExpressionGenericIntersectionNoCrash2.ts`

Reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/aliasInstantiationExpressionGenericIntersectionNoCrash2.ts
```

Source overview:

```json
{
  "suite": "tsc",
  "bytes": 271,
  "lines": 18,
  "extension": ".ts",
  "first_code_line": "declare class Class<T> {"
}
```

Failure location:

```json
{
  "code": "UnsupportedSyntax",
  "message": "expected LeftBrace, got Some(Less) at 56..57",
  "span_start": 56,
  "span_end": 57,
  "line": 4,
  "column": 20,
  "feature_label": "type-alias",
  "error_type": "parser-or-frontend-unsupported"
}
```

Source context:

```text
1 | // @target: es2015
2 | // @strict: true
3 | 
4 | declare class Class<T> {
5 |   x: T;
6 | }
7 |
```

Visible symbols before failure:

```json
[
  {
    "kind": "class",
    "name": "Class",
    "line": 4,
    "column": 9
  }
]
```

Duplicate candidates:

```json
[
  {
    "state": "open",
    "path": "issues/done/116-implement-aliasInstantiationExpressionGenericIntersectionNoCrash.md",
    "title": "Implement Aliasinstantiationexpressiongenericintersectionnocrash",
    "reason": "same reference path, same feature label"
  },
  {
    "state": "open",
    "path": "issues/open/345-implement-tsc-type-alias-coverage.md",
    "title": "Implement TypeScript type alias coverage for tsc suite (23 cases)",
    "reason": "same feature label, title overlap"
  },
  {
    "state": "open",
    "path": "issues/done/496-implement-aliasInstantiationExpressionGenericIntersectionNoCrash.md",
    "title": "Implement Aliasinstantiationexpressiongenericintersectionnocrash",
    "reason": "same reference path, same feature label"
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
            start: 37,
            end: 44,
        },
    },
    SpannedToken {
        kind: Class,
        span: Span {
            start: 45,
            end: 50,
        },
    },
    SpannedToken {
        kind: Ident(
            "Class",
        ),
        span: Span {
            start: 51,
            end: 56,
        },
    },
    SpannedToken {
        kind: Less,
        span: Span {
            start: 56,
            end: 57,
        },
    },
    SpannedToken {
        kind: Ident(
            "T",
        ),
        span: Span {
            start: 57,
            end: 58,
        },
    },
    SpannedToken {
        kind: Greater,
        span: Span {
            start: 58,
            end: 59,
        },
    },
    SpannedToken {
        kind: LeftBrace,
        span: Span {
            start: 60,
            end: 61,
        },
    },
    SpannedToken {
        kind: Ident(
            "x",
        ),
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
            "T",
        ),
        span: Span {
            start: 67,
            end: 68,
        },
    },
    SpannedToken {
        kind: Semicolon,
        span: Span {
            start: 68,
            end: 69,
        },
    },
    SpannedToken {
        kind: RightBrace,
        span: Span {
            start: 70,
            end: 71,
        },
    },
    SpannedToken {
        kind: Ident(
            "declare",
        ),
        span: Span {
            start: 73,
            end: 80,
        },
    },
    SpannedToken {
        kind: Function,
        span: Span {
            start: 81,
            end: 89,
        },
    },
    SpannedToken {
        kind: Ident(
            "fn",
        ),
        span: Span {
            start: 90,
            end: 92,
        },
    },
    SpannedToken {
        kind: Less,
        span: Span {
            start: 92,
            end: 93,
        },
    },
    SpannedToken {
        kind: Ident(
            "T",
        ),
        span: Span {
            start: 93,
            end: 94,
        },
    },
    SpannedToken {
        kind: Greater,
        span: Span {
            start: 94,
            end: 95,
        },
    },
    SpannedToken {
        kind: LeftParen,
        span: Span {
            start: 95,
            end: 96,
        },
    },
    SpannedToken {
        kind: RightParen,
        span: Span {
            start: 96,
            end: 97,
        },
    },
    SpannedToken {
        kind: Colon,
        span: Span {
            start: 97,
            end: 98,
        },
    },
    SpannedToken {
        kind: Ident(
            "T",
        ),
        span: Span {
            start: 99,
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
        kind: Ident(
            "type",
        ),
        span: Span {
            start: 104,
            end: 108,
        },
    },
    SpannedToken {
        kind: Ident(
            "ClassAlias",
        ),
        span: Span {
            start: 109,
            end: 119,
        },
    },
    SpannedToken {
        kind: Le
```

#### ast

- ok: `False`
- truncated: `False`

```text
error: [UnsupportedSyntax] expected LeftBrace, got Some(Less) at 56..57
```

#### resolved

- ok: `False`
- truncated: `False`

```text
error: [UnsupportedSyntax] expected LeftBrace, got Some(Less) at 56..57
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
        "message": "Conversion of type 'Wat<number>' to type 'Wat<string>' may be a mistake because neither type sufficiently overlaps with the other. If this was intentional, convert the expression to 'unknown' first.\n  Type 'Wat<number>' is not comparable to type '{ new (): Class<string>; prototype: Class<any>; }'.\n    Type 'Class<number>' is not comparable to type 'Class<string>'.\n      Type 'number' is not comparable to type 'string'.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/aliasInstantiationExpressionGenericIntersectionNoCrash2.ts",
        "start": 251,
        "length": 18,
        "line": 18,
        "character": 1
      }
    ],
    "hints": [
      {
        "kind": "function",
        "typeText": "T",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/aliasInstantiationExpressionGenericIntersectionNoCrash2.ts",
        "start": 90,
        "length": 2,
        "line": 8,
        "character": 18,
        "name": "fn"
      },
      {
        "kind": "binding",
        "typeText": "Wat<number>",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/aliasInstantiationExpressionGenericIntersectionNoCrash2.ts",
        "start": 233,
        "length": 3,
        "line": 17,
        "character": 15,
        "name": "wat"
      }
    ],
    "typescriptVersion": "6.0.3"
  },
  "ast": {
    "topLevel": [
      {
        "kind": "ClassDeclaration",
        "text": "declare class Class<T> {\n  x: T;\n}",
        "line": 4,
        "character": 1
      },
      {
        "kind": "FunctionDeclaration",
        "text": "declare function fn<T>(): T;",
        "line": 8,
        "character": 1
      },
      {
        "kind": "TypeAliasDeclaration",
        "text": "type ClassAlias<T> = typeof Class<T>;",
        "line": 11,
        "character": 1
      },
      {
        "kind": "TypeAliasDeclaration",
        "text": "type FnAlias<T> = typeof fn<T>;",
        "line": 12,
        "character": 1
      },
      {
        "kind": "TypeAliasDeclaration",
        "text": "type Wat<T> = ClassAlias<T> & FnAlias<T>;",
        "line": 14,
        "character": 1
      },
      {
        "kind": "FirstStatement",
        "text": "declare const wat: Wat<number>;",
        "line": 17,
        "character": 1
      },
      {
        "kind": "ExpressionStatement",
        "text": "wat as Wat<string>;",
        "line": 18,
        "character": 1
      }
    ],
    "pathToPosition": [
      {
        "kind": "SourceFile",
        "text": "declare class Class<T> {\n  x: T;\n}\n\ndeclare function fn<T>(): T;\n\n\ntype ClassAlias<T> = typeof Class<T>;\ntype FnAlias<T>",
        "line": 4,
        "character": 1
      },
      {
        "kind": "ClassDeclaration",
        "text": "declare class Class<T> {\n  x: T;\n}",
        "line": 4,
        "character": 1
      },
      {
        "kind": "Identifier",
        "text": "Class",
        "line": 4,
        "character": 15
      }
    ]
  }
}
```

Stack trace:

```text
error: [UnsupportedSyntax] expected LeftBrace, got Some(Less) at 56..57
```

## Completion evidence

Commits:

- pending

Validation result:

```text
command: env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/aliasInstantiationExpressionGenericIntersectionNoCrash --detail --no-dashboard-data
result: pass; executed=2, build_pass=0, unsupported=2, unsupported_diagcodes=UnresolvedName:2, unsupported_features=name-resolution:2
date: 2026-05-08

command: env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/aliasInstantiationExpressionGenericIntersectionNoCrash1.ts
result: pass; tokens/AST succeed, current blocker is UnresolvedName for ambient `declare const e`, superseded by issue 5161
date: 2026-05-08

command: env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/aliasInstantiationExpressionGenericIntersectionNoCrash2.ts
result: pass; tokens/AST succeed, current blocker is UnresolvedName for ambient `declare const wat`, superseded by issue 5161
date: 2026-05-08
```

Remaining risks:

- Implementation remains open in `issues/open/5161-model-ambient-value-declarations-for-name-resolution.md`.
