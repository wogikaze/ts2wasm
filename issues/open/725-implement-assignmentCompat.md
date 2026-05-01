---
id: 725
title: "Implement Assignmentcompat"
type: spike
area: frontend/resolver
class: triage-needed
priority: P1
depends_on: []
blocks: []
created: 2026-05-01
updated: 2026-05-01
---

## Summary

Triage assignmentCompat across 1 failing reference test cases and split this bucket into implementation-ready child issues.

## Problem

Reference test results show 1 cases fail in directory `assignmentCompat` with diagnostics: name-resolution. The compiler cannot handle these syntax/semantics, preventing compilation of code in this category.

Problem: assignmentCompat has 1 reference failures and needs smart-triage evidence before implementation starts.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/assignmentCompat1.ts
```

Coverage window:

```sh
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/assignmentCompat1.ts --detail
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
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/assignmentCompat1.ts --detail
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/assignmentCompat1.ts
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

- `reference/typescript/tests/cases/compiler/assignmentCompat1.ts`

## Duplicate detection

- none found by path/title/feature scan

## Smart triage

### Smart triage: Triage name resolution: assignmentCompat1

- Issue class: `triage-needed`
- Feature label: `name-resolution`
- Diagnostic: `UnresolvedName` / `resolver-symbol`
- Path: `reference/typescript/tests/cases/compiler/assignmentCompat1.ts`

Reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/assignmentCompat1.ts
```

Source overview:

```json
{
  "suite": "tsc",
  "bytes": 341,
  "lines": 12,
  "extension": ".ts",
  "first_code_line": "var x = { one: 1 };"
}
```

Failure location:

```json
{
  "code": "UnresolvedName",
  "message": "unresolved name: `y` at 129..130",
  "span_start": 129,
  "span_end": 130,
  "line": 5,
  "column": 9,
  "feature_label": "name-resolution",
  "error_type": "resolver-symbol"
}
```

Source context:

```text
2 | var x = { one: 1 };
3 | declare var y: { [index: string]: any };
4 | declare var z: { [index: number]: any };
5 | x = y;  // Error
6 | y = x;  // Ok because index signature type is any
7 | x = z;  // Error
8 | z = x;  // Ok because index signature type is any
```

Visible symbols before failure:

```json
[
  {
    "kind": "binding",
    "name": "x",
    "line": 2,
    "column": 1,
    "initializer": "{ one: 1 }"
  },
  {
    "kind": "binding",
    "name": "y",
    "line": 3,
    "column": 9
  },
  {
    "kind": "binding",
    "name": "z",
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
  },
  {
    "state": "open",
    "path": "issues/open/693-implement-arrayToLocaleStringES-name-resolution.md",
    "title": "Implement Arraytolocalestringes Name Resolution",
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
            "x",
        ),
        span: Span {
            start: 24,
            end: 25,
        },
    },
    SpannedToken {
        kind: Equal,
        span: Span {
            start: 26,
            end: 27,
        },
    },
    SpannedToken {
        kind: LeftBrace,
        span: Span {
            start: 28,
            end: 29,
        },
    },
    SpannedToken {
        kind: Ident(
            "one",
        ),
        span: Span {
            start: 30,
            end: 33,
        },
    },
    SpannedToken {
        kind: Colon,
        span: Span {
            start: 33,
            end: 34,
        },
    },
    SpannedToken {
        kind: Number(
            1,
        ),
        span: Span {
            start: 35,
            end: 36,
        },
    },
    SpannedToken {
        kind: RightBrace,
        span: Span {
            start: 37,
            end: 38,
        },
    },
    SpannedToken {
        kind: Semicolon,
        span: Span {
            start: 38,
            end: 39,
        },
    },
    SpannedToken {
        kind: Ident(
            "declare",
        ),
        span: Span {
            start: 41,
            end: 48,
        },
    },
    SpannedToken {
        kind: Var,
        span: Span {
            start: 49,
            end: 52,
        },
    },
    SpannedToken {
        kind: Ident(
            "y",
        ),
        span: Span {
            start: 53,
            end: 54,
        },
    },
    SpannedToken {
        kind: Colon,
        span: Span {
            start: 54,
            end: 55,
        },
    },
    SpannedToken {
        kind: LeftBrace,
        span: Span {
            start: 56,
            end: 57,
        },
    },
    SpannedToken {
        kind: LeftBracket,
        span: Span {
            start: 58,
            end: 59,
        },
    },
    SpannedToken {
        kind: Ident(
            "index",
        ),
        span: Span {
            start: 59,
            end: 64,
        },
    },
    SpannedToken {
        kind: Colon,
        span: Span {
            start: 64,
            end: 65,
        },
    },
    SpannedToken {
        kind: Ident(
            "string",
        ),
        span: Span {
            start: 66,
            end: 72,
        },
    },
    SpannedToken {
        kind: RightBracket,
        span: Span {
            start: 72,
            end: 73,
        },
    },
    SpannedToken {
        kind: Colon,
        span: Span {
            start: 73,
            end: 74,
        },
    },
    SpannedToken {
        kind: Ident(
            "any",
        ),
        span: Span {
            start: 75,
            end: 78,
        },
    },
    SpannedToken {
        kind: RightBrace,
        span: Span {
            start: 79,
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
            "declare",
        ),
        span: Span {
            start: 83,
            end: 90,
        },
    },
    SpannedToken {
        kind: Var,
        span: Span {
            start: 91,
            end: 94,
        },
    },
    SpannedToken {
        kind: Ident(
            "z",
        ),
        span: Span {
```

#### ast

- ok: `True`
- truncated: `False`

```text
== ast ==
[
    Let {
        name: "x",
        expr: Object {
            props: [
                (
                    "one",
                    Number {
                        value: 1,
                        span: Span {
                            start: 35,
                            end: 36,
                        },
                    },
                ),
            ],
            span: Span {
                start: 28,
                end: 38,
            },
        },
        span: Span {
            start: 20,
            end: 39,
        },
    },
    Assign {
        name: "x",
        expr: Ident {
            name: "y",
            span: Span {
                start: 129,
                end: 130,
            },
        },
        span: Span {
            start: 125,
            end: 131,
        },
    },
    Assign {
        name: "y",
        expr: Ident {
            name: "x",
            span: Span {
                start: 147,
                end: 148,
            },
        },
        span: Span {
            start: 143,
            end: 149,
        },
    },
    Assign {
        name: "x",
        expr: Ident {
            name: "z",
            span: Span {
                start: 198,
                end: 199,
            },
        },
        span: Span {
            start: 194,
            end: 200,
        },
    },
    Assign {
        name: "z",
        expr: Ident {
            name: "x",
            span: Span {
                start: 216,
                end: 217,
            },
        },
        span: Span {
            start: 212,
            end: 218,
        },
    },
    Assign {
        name: "y",
        expr: String {
            value: "foo",
            span: Span {
                start: 267,
                end: 272,
            },
        },
        span: Span {
            start: 263,
            end: 273,
        },
    },
    Assign {
        name: "z",
        expr: String {
            value: "foo",
            span: Span {
                start: 288,
                end: 293,
            },
        },
        span: Span {
            start: 284,
            end: 294,
        },
    },
    Assign {
        name: "z",
        expr: Bool {
            value: false,
            span: Span {
                start: 334,
                end: 339,
            },
        },
        span: Span {
            start: 330,
            end: 340,
        },
    },
]
```

#### resolved

- ok: `False`
- truncated: `False`

```text
error: [UnresolvedName] unresolved name: `y` at 129..130
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
        "code": 2741,
        "category": "Error",
        "message": "Property 'one' is missing in type '{ [index: string]: any; }' but required in type '{ one: number; }'.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/assignmentCompat1.ts",
        "start": 125,
        "length": 1,
        "line": 5,
        "character": 1
      },
      {
        "code": 2741,
        "category": "Error",
        "message": "Property 'one' is missing in type '{ [index: number]: any; }' but required in type '{ one: number; }'.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/assignmentCompat1.ts",
        "start": 194,
        "length": 1,
        "line": 7,
        "character": 1
      },
      {
        "code": 2322,
        "category": "Error",
        "message": "Type 'string' is not assignable to type '{ [index: string]: any; }'.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/assignmentCompat1.ts",
        "start": 263,
        "length": 1,
        "line": 9,
        "character": 1
      },
      {
        "code": 2322,
        "category": "Error",
        "message": "Type 'boolean' is not assignable to type '{ [index: number]: any; }'.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/assignmentCompat1.ts",
        "start": 330,
        "length": 1,
        "line": 11,
        "character": 1
      }
    ],
    "hints": [
      {
        "kind": "binding",
        "typeText": "{ one: number; }",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/assignmentCompat1.ts",
        "start": 24,
        "length": 1,
        "line": 2,
        "character": 5,
        "name": "x"
      },
      {
        "kind": "binding",
        "typeText": "{ [index: string]: any; }",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/assignmentCompat1.ts",
        "start": 53,
        "length": 1,
        "line": 3,
        "character": 13,
        "name": "y"
      },
      {
        "kind": "parameter",
        "typeText": "string",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/assignmentCompat1.ts",
        "start": 59,
        "length": 5,
        "line": 3,
        "character": 19,
        "name": "index"
      },
      {
        "kind": "binding",
        "typeText": "{ [index: number]: any; }",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/assignmentCompat1.ts",
        "start": 95,
        "length": 1,
        "line": 4,
        "character": 13,
        "name": "z"
      },
      {
        "kind": "parameter",
        "typeText": "number",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/assignmentCompat1.ts",
        "start": 101,
        "length": 5,
        "line": 4,
        "character": 19,
        "name": "index"
      }
    ],
    "typescriptVersion": "6.0.3"
  },
  "ast": {
    "topLevel": [
      {
        "kind": "FirstStatement",
        "text": "var x = { one: 1 };",
        "line": 2,
        "character": 1
      },
      {
        "kind": "FirstStatement",
        "text": "declare var y: { [index: string]: any };",
        "line": 3,
        "character": 1
      },
      {
        "kind": "FirstStatement",
        "text": "declare var z: { [index: number]: any };",
        "line": 4,
        "character": 1
      },
      {
        "kind": "ExpressionStatement",
        "text": "x = y;",
        "line": 5,
        "character": 1
      },
      {
        "kind": "ExpressionStatement",
        "text": "y = x;",
        "line": 6,
        "character": 1
      },
      {
        "kind": "ExpressionStatement",
        "text": "x = z;",
        "line": 7,
        "character": 1
      },
      {
        "kind": "ExpressionStatement",
        "text": "z = x;",
        "line": 8,
        "character": 1
      },
      {
        "kind": "ExpressionStatement",
        "text": "y = \"foo\";",
        "line": 9,
        "character": 1
      },
      {
        "kind": "ExpressionStatement",
        "text": "z = \"foo\";",
        "line": 10,
        "character": 1
      },
      {
        "kind": "ExpressionStatement",
        "text": "z = false;",
        "line": 11,
        "character": 1
      }
    ],
    "pathToPosition": [
      {
        "kind": "SourceFile",
        "text": "var x = { one: 1 };\r\ndeclare var y: { [index: string]: any };\r\ndeclare var z: { [index: number]: any };\r\nx = y;  // Erro",
        "line": 2,
        "character": 1
      },
      {
        "kind": "ExpressionStatement",
        "text": "x = y;",
        "line": 5,
        "character": 1
      },
      {
        "kind": "BinaryExpression",
        "text": "x = y",
        "line": 5,
        "character": 1
      },
      {
        "kind": "Identifier",
        "text": "y",
        "line": 5,
        "character": 5
      }
    ]
  }
}
```

Stack trace:

```text
error: [UnresolvedName] unresolved name: `y` at 129..130
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
