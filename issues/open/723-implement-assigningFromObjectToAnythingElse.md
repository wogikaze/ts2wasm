---
id: 723
title: "Implement Assigningfromobjecttoanythingelse"
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

Triage assigningFromObjectToAnythingElse across 1 failing reference test cases and split this bucket into implementation-ready child issues.

## Problem

Reference test results show 1 cases fail in directory `assigningFromObjectToAnythingElse` with diagnostics: name-resolution. The compiler cannot handle these syntax/semantics, preventing compilation of code in this category.

Problem: assigningFromObjectToAnythingElse has 1 reference failures and needs smart-triage evidence before implementation starts.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/assigningFromObjectToAnythingElse.ts
```

Coverage window:

```sh
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/assigningFromObjectToAnythingElse.ts --detail
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
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/assigningFromObjectToAnythingElse.ts --detail
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/assigningFromObjectToAnythingElse.ts
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

- `reference/typescript/tests/cases/compiler/assigningFromObjectToAnythingElse.ts`

## Duplicate detection

- none found by path/title/feature scan

## Smart triage

### Smart triage: Triage name resolution: assigningFromObjectToAnythingElse

- Issue class: `triage-needed`
- Feature label: `name-resolution`
- Diagnostic: `UnresolvedName` / `resolver-symbol`
- Path: `reference/typescript/tests/cases/compiler/assigningFromObjectToAnythingElse.ts`

Reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/assigningFromObjectToAnythingElse.ts
```

Source overview:

```json
{
  "suite": "tsc",
  "bytes": 180,
  "lines": 9,
  "extension": ".ts",
  "first_code_line": "declare var x: Object;"
}
```

Failure location:

```json
{
  "code": "UnresolvedName",
  "message": "unresolved name: `x` at 64..65",
  "span_start": 64,
  "span_end": 65,
  "line": 5,
  "column": 1,
  "feature_label": "name-resolution",
  "error_type": "resolver-symbol"
}
```

Source context:

```text
2 | declare var x: Object;
3 | var y: RegExp;
4 | y = x;
5 | 
6 | var a: String = Object.create<Object>("");
7 | var c: String = Object.create<Number>(1);
8 |
```

Visible symbols before failure:

```json
[
  {
    "kind": "binding",
    "name": "x",
    "line": 2,
    "column": 9
  },
  {
    "kind": "binding",
    "name": "y",
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
        kind: Ident(
            "declare",
        ),
        span: Span {
            start: 20,
            end: 27,
        },
    },
    SpannedToken {
        kind: Var,
        span: Span {
            start: 28,
            end: 31,
        },
    },
    SpannedToken {
        kind: Ident(
            "x",
        ),
        span: Span {
            start: 32,
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
        kind: Ident(
            "Object",
        ),
        span: Span {
            start: 35,
            end: 41,
        },
    },
    SpannedToken {
        kind: Semicolon,
        span: Span {
            start: 41,
            end: 42,
        },
    },
    SpannedToken {
        kind: Var,
        span: Span {
            start: 44,
            end: 47,
        },
    },
    SpannedToken {
        kind: Ident(
            "y",
        ),
        span: Span {
            start: 48,
            end: 49,
        },
    },
    SpannedToken {
        kind: Colon,
        span: Span {
            start: 49,
            end: 50,
        },
    },
    SpannedToken {
        kind: Ident(
            "RegExp",
        ),
        span: Span {
            start: 51,
            end: 57,
        },
    },
    SpannedToken {
        kind: Semicolon,
        span: Span {
            start: 57,
            end: 58,
        },
    },
    SpannedToken {
        kind: Ident(
            "y",
        ),
        span: Span {
            start: 60,
            end: 61,
        },
    },
    SpannedToken {
        kind: Equal,
        span: Span {
            start: 62,
            end: 63,
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
        kind: Semicolon,
        span: Span {
            start: 65,
            end: 66,
        },
    },
    SpannedToken {
        kind: Var,
        span: Span {
            start: 70,
            end: 73,
        },
    },
    SpannedToken {
        kind: Ident(
            "a",
        ),
        span: Span {
            start: 74,
            end: 75,
        },
    },
    SpannedToken {
        kind: Colon,
        span: Span {
            start: 75,
            end: 76,
        },
    },
    SpannedToken {
        kind: Ident(
            "String",
        ),
        span: Span {
            start: 77,
            end: 83,
        },
    },
    SpannedToken {
        kind: Equal,
        span: Span {
            start: 84,
            end: 85,
        },
    },
    SpannedToken {
        kind: Ident(
            "Object",
        ),
        span: Span {
            start: 86,
            end: 92,
        },
    },
    SpannedToken {
        kind: Dot,
        span: Span {
            start: 92,
            end: 93,
        },
    },
    SpannedToken {
        kind: Ident(
            "create",
        ),
        span: Span {
            start: 93,
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
            "Object",
        ),
        span: Span {
            start: 100,
            end: 106,
        },
    },
    SpannedToken {
        kind:
```

#### ast

- ok: `True`
- truncated: `True`

```text
== ast ==
[
    Let {
        name: "y",
        expr: Undefined {
            span: Span {
                start: 48,
                end: 49,
            },
        },
        span: Span {
            start: 44,
            end: 58,
        },
    },
    Assign {
        name: "y",
        expr: Ident {
            name: "x",
            span: Span {
                start: 64,
                end: 65,
            },
        },
        span: Span {
            start: 60,
            end: 66,
        },
    },
    Let {
        name: "a",
        expr: Binary {
            left: Binary {
                left: Member {
                    object: Ident {
                        name: "Object",
                        span: Span {
                            start: 86,
                            end: 92,
                        },
                    },
                    property: "create",
                    span: Span {
                        start: 86,
                        end: 99,
                    },
                },
                op: Less,
                right: Ident {
                    name: "Object",
                    span: Span {
                        start: 100,
                        end: 106,
                    },
                },
                span: Span {
                    start: 86,
                    end: 106,
                },
            },
            op: Greater,
            right: String {
                value: "",
                span: Span {
                    start: 108,
                    end: 110,
                },
            },
            span: Span {
                start: 86,
                end: 110,
            },
        },
        span: Span {
            start: 70,
            end: 112,
        },
    },
    Let {
        name: "c",
        expr: Binary {
            left: Binary {
                left: Member {
                    object: Ident {
                        name: "Object",
                        span: Span {
                            start: 130,
                            end: 136,
                        },
                    },
                    property: "create",
                    span: Span {
                        start: 130,
                        end: 143,
                    },
                },
                op: Less,
                right: Ident {
                    name: "Number",
                    span: Span {
                        start: 144,
                        end: 150,
                    },
                },
                span: Span {
                    start: 130,
                    end: 150,
                },
            },
            op: Greater,
            right: Number {
                value: 1,
                span: Span {
                    start: 152,
                    end: 153,
                },
            },
            span: Span {
                start: 130,
                end: 153,
            },
        },
        span: Span {
            start: 114,
            end: 155,
        },
    },
    Let {
        name: "w",
        expr: New {
            expr: Ident {
                name: "Object",
                span: Span {
                    start: 178,
                    end: 184,
                },
            },
            args: [],
            span: Span {
                start: 174,
                end: 186,
            },
        },
        span: Span {
            start: 159,
```

#### resolved

- ok: `False`
- truncated: `False`

```text
error: [UnresolvedName] unresolved name: `x` at 64..65
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
        "code": 2696,
        "category": "Error",
        "message": "The 'Object' type is assignable to very few other types. Did you mean to use the 'any' type instead?\n  Type 'Object' is missing the following properties from type 'RegExp': exec, test, source, global, and 13 more.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/assigningFromObjectToAnythingElse.ts",
        "start": 60,
        "length": 1,
        "line": 4,
        "character": 1
      },
      {
        "code": 2558,
        "category": "Error",
        "message": "Expected 0 type arguments, but got 1.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/assigningFromObjectToAnythingElse.ts",
        "start": 100,
        "length": 6,
        "line": 6,
        "character": 31
      },
      {
        "code": 2558,
        "category": "Error",
        "message": "Expected 0 type arguments, but got 1.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/assigningFromObjectToAnythingElse.ts",
        "start": 144,
        "length": 6,
        "line": 7,
        "character": 31
      },
      {
        "code": 2696,
        "category": "Error",
        "message": "The 'Object' type is assignable to very few other types. Did you mean to use the 'any' type instead?\n  Type 'Object' is missing the following properties from type 'Error': name, message",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/assigningFromObjectToAnythingElse.ts",
        "start": 163,
        "length": 1,
        "line": 9,
        "character": 5
      }
    ],
    "hints": [
      {
        "kind": "binding",
        "typeText": "Object",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/assigningFromObjectToAnythingElse.ts",
        "start": 32,
        "length": 1,
        "line": 2,
        "character": 13,
        "name": "x"
      },
      {
        "kind": "binding",
        "typeText": "RegExp",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/assigningFromObjectToAnythingElse.ts",
        "start": 48,
        "length": 1,
        "line": 3,
        "character": 5,
        "name": "y"
      },
      {
        "kind": "binding",
        "typeText": "String",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/assigningFromObjectToAnythingElse.ts",
        "start": 74,
        "length": 1,
        "line": 6,
        "character": 5,
        "name": "a"
      },
      {
        "kind": "binding",
        "typeText": "String",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/assigningFromObjectToAnythingElse.ts",
        "start": 118,
        "length": 1,
        "line": 7,
        "character": 5,
        "name": "c"
      },
      {
        "kind": "binding",
        "typeText": "Error",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/assigningFromObjectToAnythingElse.ts",
        "start": 163,
        "length": 1,
        "line": 9,
        "character": 5,
        "name": "w"
      }
    ],
    "typescriptVersion": "6.0.3"
  },
  "ast": {
    "topLevel": [
      {
        "kind": "FirstStatement",
        "text": "declare var x: Object;",
        "line": 2,
        "character": 1
      },
      {
        "kind": "FirstStatement",
        "text": "var y: RegExp;",
        "line": 3,
        "character": 1
      },
      {
        "kind": "ExpressionStatement",
        "text": "y = x;",
        "line": 4,
        "character": 1
      },
      {
        "kind": "FirstStatement",
        "text": "var a: String = Object.create<Object>(\"\");",
        "line": 6,
        "character": 1
      },
      {
        "kind": "FirstStatement",
        "text": "var c: String = Object.create<Number>(1);",
        "line": 7,
        "character": 1
      },
      {
        "kind": "FirstStatement",
        "text": "var w: Error = new Object();",
        "line": 9,
        "character": 1
      }
    ],
    "pathToPosition": [
      {
        "kind": "SourceFile",
        "text": "declare var x: Object;\r\nvar y: RegExp;\r\ny = x;\r\n\r\nvar a: String = Object.create<Object>(\"\");\r\nvar c: String = Object.cre",
        "line": 2,
        "character": 1
      },
      {
        "kind": "ExpressionStatement",
        "text": "y = x;",
        "line": 4,
        "character": 1
      },
      {
        "kind": "BinaryExpression",
        "text": "y = x",
        "line": 4,
        "character": 1
      },
      {
        "kind": "Identifier",
        "text": "x",
        "line": 4,
        "character": 5
      }
    ]
  }
}
```

Stack trace:

```text
error: [UnresolvedName] unresolved name: `x` at 64..65
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
