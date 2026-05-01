---
id: 693
title: "Implement Arraytolocalestringes Name Resolution"
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

Triage arrayToLocaleStringES-name-resolution across 1 failing reference test cases and split this bucket into implementation-ready child issues.

## Problem

Reference test results show 1 cases fail in directory `arrayToLocaleStringES-name-resolution` with diagnostics: name-resolution. The compiler cannot handle these syntax/semantics, preventing compilation of code in this category.

Problem: arrayToLocaleStringES-name-resolution has 1 reference failures and needs smart-triage evidence before implementation starts.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/arrayToLocaleStringES5.ts
```

Coverage window:

```sh
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/arrayToLocaleStringES5.ts --detail
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
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/arrayToLocaleStringES5.ts --detail
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/arrayToLocaleStringES5.ts
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

- `reference/typescript/tests/cases/compiler/arrayToLocaleStringES5.ts`

## Duplicate detection

- `issues/open/064-implement-name-resolution.md` - Implement name resolution (triaged - superseded by test262 metadata issues) (same feature label, title overlap)
- `issues/open/437-implement-name-resolution.md` - Implement name resolution (same feature label, title overlap)

## Smart triage

### Smart triage: Triage name resolution: arrayToLocaleStringES5

- Issue class: `triage-needed`
- Feature label: `name-resolution`
- Diagnostic: `UnresolvedName` / `resolver-symbol`
- Path: `reference/typescript/tests/cases/compiler/arrayToLocaleStringES5.ts`

Reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/arrayToLocaleStringES5.ts
```

Source overview:

```json
{
  "suite": "tsc",
  "bytes": 2706,
  "lines": 57,
  "extension": ".ts",
  "first_code_line": "let str: string;"
}
```

Failure location:

```json
{
  "code": "UnresolvedName",
  "message": "unresolved name: `Int8Array` at 491..500",
  "span_start": 491,
  "span_end": 500,
  "line": 14,
  "column": 23,
  "feature_label": "name-resolution",
  "error_type": "resolver-symbol"
}
```

Source context:

```text
11 | str = dates.toLocaleString('fr'); // should be error
12 | str = dates.toLocaleString('fr', { timeZone: 'UTC' }); // should be error
13 | 
14 | const int8Array = new Int8Array(3);
15 | str = int8Array.toLocaleString(); // OK
16 | str = int8Array.toLocaleString('en-US'); // should be error
17 | str = int8Array.toLocaleString('en-US', { style: 'currency', currency: 'EUR' }); // should be error
```

Visible symbols before failure:

```json
[
  {
    "kind": "binding",
    "name": "str",
    "line": 3,
    "column": 1
  },
  {
    "kind": "binding",
    "name": "arr",
    "line": 4,
    "column": 1,
    "initializer": "[1, 2, 3]"
  },
  {
    "kind": "binding",
    "name": "dates",
    "line": 9,
    "column": 1
  },
  {
    "kind": "binding",
    "name": "int8Array",
    "line": 14,
    "column": 1,
    "initializer": "new"
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
        kind: Let,
        span: Span {
            start: 25,
            end: 28,
        },
    },
    SpannedToken {
        kind: Ident(
            "str",
        ),
        span: Span {
            start: 29,
            end: 32,
        },
    },
    SpannedToken {
        kind: Colon,
        span: Span {
            start: 32,
            end: 33,
        },
    },
    SpannedToken {
        kind: Ident(
            "string",
        ),
        span: Span {
            start: 34,
            end: 40,
        },
    },
    SpannedToken {
        kind: Semicolon,
        span: Span {
            start: 40,
            end: 41,
        },
    },
    SpannedToken {
        kind: Const,
        span: Span {
            start: 42,
            end: 47,
        },
    },
    SpannedToken {
        kind: Ident(
            "arr",
        ),
        span: Span {
            start: 48,
            end: 51,
        },
    },
    SpannedToken {
        kind: Equal,
        span: Span {
            start: 52,
            end: 53,
        },
    },
    SpannedToken {
        kind: LeftBracket,
        span: Span {
            start: 54,
            end: 55,
        },
    },
    SpannedToken {
        kind: Number(
            1,
        ),
        span: Span {
            start: 55,
            end: 56,
        },
    },
    SpannedToken {
        kind: Comma,
        span: Span {
            start: 56,
            end: 57,
        },
    },
    SpannedToken {
        kind: Number(
            2,
        ),
        span: Span {
            start: 58,
            end: 59,
        },
    },
    SpannedToken {
        kind: Comma,
        span: Span {
            start: 59,
            end: 60,
        },
    },
    SpannedToken {
        kind: Number(
            3,
        ),
        span: Span {
            start: 61,
            end: 62,
        },
    },
    SpannedToken {
        kind: RightBracket,
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
        kind: Ident(
            "str",
        ),
        span: Span {
            start: 65,
            end: 68,
        },
    },
    SpannedToken {
        kind: Equal,
        span: Span {
            start: 69,
            end: 70,
        },
    },
    SpannedToken {
        kind: Ident(
            "arr",
        ),
        span: Span {
            start: 71,
            end: 74,
        },
    },
    SpannedToken {
        kind: Dot,
        span: Span {
            start: 74,
            end: 75,
        },
    },
    SpannedToken {
        kind: Ident(
            "toLocaleString",
        ),
        span: Span {
            start: 75,
            end: 89,
        },
    },
    SpannedToken {
        kind: LeftParen,
        span: Span {
            start: 89,
            end: 90,
        },
    },
    SpannedToken {
        kind: RightParen,
        span: Span {
            start: 90,
            end: 91,
        },
    },
    SpannedToken {
        kind: Semicolon,
        span: Span {
            start: 91,
            end: 92,
        },
    },
    SpannedToken {
        kind: Ident(
            "str",
        ),
        span: Span {
            start: 99,
            end: 102,
        },
    },
    SpannedToken {
        kind: Equal,
        span: Span {
            s
```

#### ast

- ok: `True`
- truncated: `True`

```text
== ast ==
[
    Let {
        name: "str",
        expr: Undefined {
            span: Span {
                start: 29,
                end: 32,
            },
        },
        span: Span {
            start: 25,
            end: 41,
        },
    },
    Let {
        name: "arr",
        expr: Array {
            elements: [
                Present(
                    Number {
                        value: 1,
                        span: Span {
                            start: 55,
                            end: 56,
                        },
                    },
                ),
                Present(
                    Number {
                        value: 2,
                        span: Span {
                            start: 58,
                            end: 59,
                        },
                    },
                ),
                Present(
                    Number {
                        value: 3,
                        span: Span {
                            start: 61,
                            end: 62,
                        },
                    },
                ),
            ],
            span: Span {
                start: 54,
                end: 63,
            },
        },
        span: Span {
            start: 42,
            end: 64,
        },
    },
    Assign {
        name: "str",
        expr: Call {
            callee: Member {
                object: Ident {
                    name: "arr",
                    span: Span {
                        start: 71,
                        end: 74,
                    },
                },
                property: "toLocaleString",
                span: Span {
                    start: 71,
                    end: 89,
                },
            },
            args: [],
            span: Span {
                start: 71,
                end: 91,
            },
        },
        span: Span {
            start: 65,
            end: 92,
        },
    },
    Assign {
        name: "str",
        expr: Call {
            callee: Member {
                object: Ident {
                    name: "arr",
                    span: Span {
                        start: 105,
                        end: 108,
                    },
                },
                property: "toLocaleString",
                span: Span {
                    start: 105,
                    end: 123,
                },
            },
            args: [
                String {
                    value: "en-US",
                    span: Span {
                        start: 124,
                        end: 131,
                    },
                },
            ],
            span: Span {
                start: 105,
                end: 132,
            },
        },
        span: Span {
            start: 99,
            end: 133,
        },
    },
    Assign {
        name: "str",
        expr: Call {
            callee: Member {
                object: Ident {
                    name: "arr",
                    span: Span {
                        start: 159,
                        end: 162,
                    },
                },
                property: "toLocaleString",
                span: Span {
                    start: 159,
                    end: 177,
                },
            },
            args: [
                String {
                    value: "en-US",
                    span: Span {
```

#### resolved

- ok: `False`
- truncated: `False`

```text
error: [UnresolvedName] unresolved name: `Int8Array` at 491..500
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
        "kind": "binding",
        "typeText": "string",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/arrayToLocaleStringES5.ts",
        "start": 29,
        "length": 3,
        "line": 3,
        "character": 5,
        "name": "str"
      },
      {
        "kind": "binding",
        "typeText": "number[]",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/arrayToLocaleStringES5.ts",
        "start": 48,
        "length": 3,
        "line": 4,
        "character": 7,
        "name": "arr"
      },
      {
        "kind": "binding",
        "typeText": "readonly Date[]",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/arrayToLocaleStringES5.ts",
        "start": 254,
        "length": 5,
        "line": 9,
        "character": 7,
        "name": "dates"
      },
      {
        "kind": "binding",
        "typeText": "Int8Array<ArrayBuffer>",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/arrayToLocaleStringES5.ts",
        "start": 475,
        "length": 9,
        "line": 14,
        "character": 7,
        "name": "int8Array"
      },
      {
        "kind": "binding",
        "typeText": "Uint8Array<ArrayBuffer>",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/arrayToLocaleStringES5.ts",
        "start": 712,
        "length": 10,
        "line": 19,
        "character": 7,
        "name": "uint8Array"
      },
      {
        "kind": "binding",
        "typeText": "Uint8ClampedArray<ArrayBuffer>",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/arrayToLocaleStringES5.ts",
        "start": 954,
        "length": 17,
        "line": 24,
        "character": 7,
        "name": "uint8ClampedArray"
      },
      {
        "kind": "binding",
        "typeText": "Int16Array<ArrayBuffer>",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/arrayToLocaleStringES5.ts",
        "start": 1231,
        "length": 10,
        "line": 29,
        "character": 7,
        "name": "int16Array"
      },
      {
        "kind": "binding",
        "typeText": "Uint16Array<ArrayBuffer>",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/arrayToLocaleStringES5.ts",
        "start": 1473,
        "length": 11,
        "line": 34,
        "character": 7,
        "name": "uint16Array"
      },
      {
        "kind": "binding",
        "typeText": "Int32Array<ArrayBuffer>",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/arrayToLocaleStringES5.ts",
        "start": 1720,
        "length": 10,
        "line": 39,
        "character": 7,
        "name": "int32Array"
      },
      {
        "kind": "binding",
        "typeText": "Uint32Array<ArrayBuffer>",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/arrayToLocaleStringES5.ts",
        "start": 1962,
        "length": 11,
        "line": 44,
        "character": 7,
        "name": "uint32Array"
      },
      {
        "kind": "binding",
        "typeText": "Float32Array<ArrayBuffer>",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/arrayToLocaleStringES5.ts",
        "start": 2209,
        "length": 12,
        "line": 49,
        "character": 7,
        "name": "float32Array"
      },
      {
        "kind": "binding",
        "typeText": "Float64Array<ArrayBuffer>",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/arrayToLocaleStringES5.ts",
        "start": 2461,
        "length": 12,
        "line": 54,
        "character": 7,
        "name": "float64Array"
      }
    ],
    "typescriptVersion": "6.0.3"
  },
  "ast": {
    "topLevel": [
      {
        "kind": "FirstStatement",
        "text": "let str: string;",
        "line": 3,
        "character": 1
      },
      {
        "kind": "FirstStatement",
        "text": "const arr = [1, 2, 3];",
        "line": 4,
        "character": 1
      },
      {
        "kind": "ExpressionStatement",
        "text": "str = arr.toLocaleString();",
        "line": 5,
        "character": 1
      },
      {
        "kind": "ExpressionStatement",
        "text": "str = arr.toLocaleString('en-US');",
        "line": 6,
        "character": 1
      },
      {
        "kind": "ExpressionStatement",
        "text": "str = arr.toLocaleString('en-US', { style: 'currency', currency: 'EUR' });",
        "line": 7,
        "character": 1
      },
      {
        "kind": "FirstStatement",
        "text": "const dates: readonly Date[] = [new Date(), new Date()];",
        "line": 9,
        "character": 1
      },
      {
        "kind": "ExpressionStatement",
        "text": "str = dates.toLocaleString();",
        "line": 10,
        "character": 1
      },
      {
        "kind": "ExpressionStatement",
        "text": "str = dates.toLocaleString('fr');",
        "line": 11,
        "character": 1
      },
      {
        "kind": "ExpressionStatement",
        "text": "str = dates.toLocaleString('fr', { timeZone: 'UTC' });",
        "line": 12,
        "character": 1
      },
      {
        "kind": "FirstStatement",
        "text": "const int8Array = new Int8Array(3);",
        "line": 14,
        "character": 1
      },
      {
        "kind": "ExpressionStatement",
        "text": "str = int8Array.toLocaleString();",
        "line": 15,
        "character": 1
      },
      {
        "kind": "ExpressionStatement",
        "text": "str = int8Array.toLocaleString('en-US');",
        "line": 16,
        "character": 1
      },
      {
        "kind": "ExpressionStatement",
        "text": "str = int8Array.toLocaleString('en-US', { style: 'currency', currency: 'EUR' });",
        "line": 17,
        "character": 1
      },
      {
        "kind": "FirstStatement",
        "text": "const uint8Array = new Uint8Array(3);",
        "line": 19,
        "character": 1
      },
      {
        "kind": "ExpressionStatement",
        "text": "str = uint8Array.toLocaleString();",
        "line": 20,
        "character": 1
      },
      {
        "kind": "ExpressionStatement",
        "text": "str = uint8Array.toLocaleString('en-US');",
        "line": 21,
        "character": 1
      },
      {
        "kind": "ExpressionStatement",
        "text": "str = uint8Array.toLocaleString('en-US', { style: 'currency', currency: 'EUR' });",
        "line": 22,
        "character": 1
      },
      {
        "kind": "FirstStatement",
        "text": "const uint8ClampedArray = new Uint8ClampedArray(3);",
        "line": 24,
        "character": 1
      },
      {
        "kind": "ExpressionStatement",
        "text": "str = uint8ClampedArray.toLocaleString();",
        "line": 25,
        "character": 1
      },
      {
        "kind": "ExpressionStatement",
        "text": "str = uint8ClampedArray.toLocaleString('en-US');",
        "line": 26,
        "character": 1
      }
    ],
    "pathToPosition": [
      {
        "kind": "SourceFile",
        "text": "let str: string;\nconst arr = [1, 2, 3];\nstr = arr.toLocaleString(); // OK\nstr = arr.toLocaleString('en-US'); // should b",
        "line": 3,
        "character": 1
      },
      {
        "kind": "FirstStatement",
        "text": "const int8Array = new Int8Array(3);",
        "line": 14,
        "character": 1
      },
      {
        "kind": "VariableDeclarationList",
        "text": "const int8Array = new Int8Array(3)",
        "line": 14,
        "character": 1
      },
      {
        "kind": "VariableDeclaration",
        "text": "int8Array = new Int8Array(3)",
        "line": 14,
        "character": 7
      },
      {
        "kind": "NewExpression",
        "text":
```

Stack trace:

```text
error: [UnresolvedName] unresolved name: `Int8Array` at 491..500
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
