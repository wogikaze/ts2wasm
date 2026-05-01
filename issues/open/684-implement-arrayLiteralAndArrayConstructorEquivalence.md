---
id: 684
title: "Implement Arrayliteralandarrayconstructorequivalence"
type: spike
area: frontend/resolver
class: blocked
priority: P1
depends_on: [5005]
blocks: []
created: 2026-05-01
updated: 2026-05-01
---

## Summary

Triage arrayLiteralAndArrayConstructorEquivalence across 1 failing reference test cases and split this bucket into implementation-ready child issues.

## Problem

Reference test results show 1 cases fail in directory `arrayLiteralAndArrayConstructorEquivalence` with diagnostics: name-resolution. The compiler cannot handle these syntax/semantics, preventing compilation of code in this category.

Problem: arrayLiteralAndArrayConstructorEquivalence has 1 reference failures and needs smart-triage evidence before implementation starts.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/arrayLiteralAndArrayConstructorEquivalence1.ts
```

Coverage window:

```sh
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/arrayLiteralAndArrayConstructorEquivalence1.ts --detail
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
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/arrayLiteralAndArrayConstructorEquivalence1.ts --detail
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/arrayLiteralAndArrayConstructorEquivalence1.ts
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

- `reference/typescript/tests/cases/compiler/arrayLiteralAndArrayConstructorEquivalence1.ts`

## Duplicate detection

- none found by path/title/feature scan

## Smart triage

### Smart triage: Triage name resolution: arrayLiteralAndArrayConstructorEquivalence1

- Issue class: `triage-needed`
- Feature label: `name-resolution`
- Diagnostic: `UnresolvedName` / `resolver-symbol`
- Path: `reference/typescript/tests/cases/compiler/arrayLiteralAndArrayConstructorEquivalence1.ts`

Reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/arrayLiteralAndArrayConstructorEquivalence1.ts
```

Source overview:

```json
{
  "suite": "tsc",
  "bytes": 262,
  "lines": 13,
  "extension": ".ts",
  "first_code_line": "var myCars=new Array();"
}
```

Failure location:

```json
{
  "code": "UnresolvedName",
  "message": "unresolved name: `myCars4` at 181..188",
  "span_start": 181,
  "span_end": 188,
  "line": 8,
  "column": 17,
  "feature_label": "name-resolution",
  "error_type": "resolver-symbol"
}
```

Source context:

```text
 5 | declare var myCars5: Array<any>[];
 6 |  
 7 | myCars = myCars3;
 8 | myCars = myCars4;
 9 | myCars = myCars5;
10 |  
11 | myCars3 = myCars;
```

Visible symbols before failure:

```json
[
  {
    "kind": "binding",
    "name": "myCars",
    "line": 2,
    "column": 1,
    "initializer": "new Array()"
  },
  {
    "kind": "binding",
    "name": "myCars3",
    "line": 3,
    "column": 1,
    "initializer": "new Array({})"
  },
  {
    "kind": "binding",
    "name": "myCars4",
    "line": 4,
    "column": 9
  },
  {
    "kind": "binding",
    "name": "myCars5",
    "line": 5,
    "column": 9
  }
]
```

Duplicate candidates:

```json
[
  {
    "state": "open",
    "path": "issues/open/064-implement-name-resolution.md",
    "title": "Implement name resolution (triaged - superseded by test262 metadata issues)",
    "reason": "same feature label, title overlap"
  },
  {
    "state": "open",
    "path": "issues/open/437-implement-name-resolution.md",
    "title": "Implement name resolution",
    "reason": "same feature label, title overlap"
  },
  {
    "state": "open",
    "path": "issues/open/648-implement-argumentsAsPropertyName-name-resolution.md",
    "title": "Implement Argumentsaspropertyname Name Resolution",
    "reason": "same feature label, title overlap"
  },
  {
    "state": "open",
    "path": "issues/open/654-implement-argumentsReferenceInConstructor-name-resolution.md",
    "title": "Implement Argumentsreferenceinconstructor Name Resolution",
    "reason": "same feature label, title overlap"
  },
  {
    "state": "open",
    "path": "issues/open/657-implement-argumentsReferenceInMethod-name-resolution.md",
    "title": "Implement Argumentsreferenceinmethod Name Resolution",
    "reason": "same feature label, title overlap"
  }
]
```

Error-specific suggestions:

- Check whether the missing name should be a local binding, function binding, builtin, import binding, or runtime global.
- Acceptance should assert both the formerly missing symbol and an adjacent negative case.

Automatic repair sketch:

```rust
// Rough sketch only: make unresolved names inspectable at resolver failure.
if let Some(binding) = self.lookup_name(name) {
    return Ok(binding);
}
return Err(Diagnostic {
    code: DiagCode::UnresolvedName,
    message: format!("unresolved name `{name}`; visible bindings: {:?}", self.visible_names()),
    span,
});
```

Compiler dumps:

#### tokens

- ok: `True`
- truncated: `True`

```text
== tokens ==
[
    SpannedToken {
        kind: Var,
        span: Span {
            start: 20,
            end: 23,
        },
    },
    SpannedToken {
        kind: Ident(
            "myCars",
        ),
        span: Span {
            start: 24,
            end: 30,
        },
    },
    SpannedToken {
        kind: Equal,
        span: Span {
            start: 30,
            end: 31,
        },
    },
    SpannedToken {
        kind: New,
        span: Span {
            start: 31,
            end: 34,
        },
    },
    SpannedToken {
        kind: Ident(
            "Array",
        ),
        span: Span {
            start: 35,
            end: 40,
        },
    },
    SpannedToken {
        kind: LeftParen,
        span: Span {
            start: 40,
            end: 41,
        },
    },
    SpannedToken {
        kind: RightParen,
        span: Span {
            start: 41,
            end: 42,
        },
    },
    SpannedToken {
        kind: Semicolon,
        span: Span {
            start: 42,
            end: 43,
        },
    },
    SpannedToken {
        kind: Var,
        span: Span {
            start: 46,
            end: 49,
        },
    },
    SpannedToken {
        kind: Ident(
            "myCars3",
        ),
        span: Span {
            start: 50,
            end: 57,
        },
    },
    SpannedToken {
        kind: Equal,
        span: Span {
            start: 58,
            end: 59,
        },
    },
    SpannedToken {
        kind: New,
        span: Span {
            start: 60,
            end: 63,
        },
    },
    SpannedToken {
        kind: Ident(
            "Array",
        ),
        span: Span {
            start: 64,
            end: 69,
        },
    },
    SpannedToken {
        kind: LeftParen,
        span: Span {
            start: 69,
            end: 70,
        },
    },
    SpannedToken {
        kind: LeftBrace,
        span: Span {
            start: 70,
            end: 71,
        },
    },
    SpannedToken {
        kind: RightBrace,
        span: Span {
            start: 71,
            end: 72,
        },
    },
    SpannedToken {
        kind: RightParen,
        span: Span {
            start: 72,
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
            "declare",
        ),
        span: Span {
            start: 76,
            end: 83,
        },
    },
    SpannedToken {
        kind: Var,
        span: Span {
            start: 84,
            end: 87,
        },
    },
    SpannedToken {
        kind: Ident(
            "myCars4",
        ),
        span: Span {
            start: 88,
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
            "Array",
        ),
        span: Span {
            start: 97,
            end: 102,
        },
    },
    SpannedToken {
        kind: Semicolon,
        span: Span {
            start: 102,
            end: 103,
        },
    },
    SpannedToken {
        kind: Ident(
            "declare",
        ),
        span: Span {
            start: 114,
            end: 121,
        },
    },
    SpannedToken {
        kind: Var,
        span: Span {
            start: 122,
            end: 125,
```

#### ast

- ok: `True`
- truncated: `False`

```text
== ast ==
[
    Let {
        name: "myCars",
        expr: New {
            expr: Ident {
                name: "Array",
                span: Span {
                    start: 35,
                    end: 40,
                },
            },
            args: [],
            span: Span {
                start: 31,
                end: 42,
            },
        },
        span: Span {
            start: 20,
            end: 43,
        },
    },
    Let {
        name: "myCars3",
        expr: New {
            expr: Ident {
                name: "Array",
                span: Span {
                    start: 64,
                    end: 69,
                },
            },
            args: [
                Object {
                    props: [],
                    span: Span {
                        start: 70,
                        end: 72,
                    },
                },
            ],
            span: Span {
                start: 60,
                end: 73,
            },
        },
        span: Span {
            start: 46,
            end: 74,
        },
    },
    Assign {
        name: "myCars",
        expr: Ident {
            name: "myCars3",
            span: Span {
                start: 162,
                end: 169,
            },
        },
        span: Span {
            start: 153,
            end: 170,
        },
    },
    Assign {
        name: "myCars",
        expr: Ident {
            name: "myCars4",
            span: Span {
                start: 181,
                end: 188,
            },
        },
        span: Span {
            start: 172,
            end: 189,
        },
    },
    Assign {
        name: "myCars",
        expr: Ident {
            name: "myCars5",
            span: Span {
                start: 200,
                end: 207,
            },
        },
        span: Span {
            start: 191,
            end: 208,
        },
    },
    Assign {
        name: "myCars3",
        expr: Ident {
            name: "myCars",
            span: Span {
                start: 223,
                end: 229,
            },
        },
        span: Span {
            start: 213,
            end: 230,
        },
    },
    Assign {
        name: "myCars3",
        expr: Ident {
            name: "myCars4",
            span: Span {
                start: 242,
                end: 249,
            },
        },
        span: Span {
            start: 232,
            end: 250,
        },
    },
    Assign {
        name: "myCars3",
        expr: Ident {
            name: "myCars5",
            span: Span {
                start: 262,
                end: 269,
            },
        },
        span: Span {
            start: 252,
            end: 270,
        },
    },
]
```

#### resolved

- ok: `False`
- truncated: `False`

```text
error: [UnresolvedName] unresolved name: `myCars4` at 181..188
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
        "code": 2314,
        "category": "Error",
        "message": "Generic type 'Array<T>' requires 1 type argument(s).",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/arrayLiteralAndArrayConstructorEquivalence1.ts",
        "start": 97,
        "length": 5,
        "line": 4,
        "character": 22
      }
    ],
    "hints": [
      {
        "kind": "binding",
        "typeText": "any[]",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/arrayLiteralAndArrayConstructorEquivalence1.ts",
        "start": 24,
        "length": 6,
        "line": 2,
        "character": 5,
        "name": "myCars"
      },
      {
        "kind": "binding",
        "typeText": "{}[]",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/arrayLiteralAndArrayConstructorEquivalence1.ts",
        "start": 50,
        "length": 7,
        "line": 3,
        "character": 5,
        "name": "myCars3"
      },
      {
        "kind": "binding",
        "typeText": "any",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/arrayLiteralAndArrayConstructorEquivalence1.ts",
        "start": 88,
        "length": 7,
        "line": 4,
        "character": 13,
        "name": "myCars4"
      },
      {
        "kind": "binding",
        "typeText": "any[][]",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/arrayLiteralAndArrayConstructorEquivalence1.ts",
        "start": 126,
        "length": 7,
        "line": 5,
        "character": 13,
        "name": "myCars5"
      }
    ],
    "typescriptVersion": "6.0.3"
  },
  "ast": {
    "topLevel": [
      {
        "kind": "FirstStatement",
        "text": "var myCars=new Array();",
        "line": 2,
        "character": 1
      },
      {
        "kind": "FirstStatement",
        "text": "var myCars3 = new Array({});",
        "line": 3,
        "character": 1
      },
      {
        "kind": "FirstStatement",
        "text": "declare var myCars4: Array;",
        "line": 4,
        "character": 1
      },
      {
        "kind": "FirstStatement",
        "text": "declare var myCars5: Array<any>[];",
        "line": 5,
        "character": 1
      },
      {
        "kind": "ExpressionStatement",
        "text": "myCars = myCars3;",
        "line": 7,
        "character": 1
      },
      {
        "kind": "ExpressionStatement",
        "text": "myCars = myCars4;",
        "line": 8,
        "character": 1
      },
      {
        "kind": "ExpressionStatement",
        "text": "myCars = myCars5;",
        "line": 9,
        "character": 1
      },
      {
        "kind": "ExpressionStatement",
        "text": "myCars3 = myCars;",
        "line": 11,
        "character": 1
      },
      {
        "kind": "ExpressionStatement",
        "text": "myCars3 = myCars4;",
        "line": 12,
        "character": 1
      },
      {
        "kind": "ExpressionStatement",
        "text": "myCars3 = myCars5;",
        "line": 13,
        "character": 1
      }
    ],
    "pathToPosition": [
      {
        "kind": "SourceFile",
        "text": "var myCars=new Array(); \r\nvar myCars3 = new Array({});\r\ndeclare var myCars4: Array; // error\r\ndeclare var myCars5: Array",
        "line": 2,
        "character": 1
      },
      {
        "kind": "ExpressionStatement",
        "text": "myCars = myCars4;",
        "line": 8,
        "character": 1
      },
      {
        "kind": "BinaryExpression",
        "text": "myCars = myCars4",
        "line": 8,
        "character": 1
      },
      {
        "kind": "Identifier",
        "text": "myCars4",
        "line": 8,
        "character": 10
      }
    ]
  }
}
```

Stack trace:

```text
error: [UnresolvedName] unresolved name: `myCars4` at 181..188
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
