---
id: 682
title: "Implement Arrayindexwitharrayfails"
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

Triage arrayIndexWithArrayFails across 1 failing reference test cases and split this bucket into implementation-ready child issues.

## Problem

Reference test results show 1 cases fail in directory `arrayIndexWithArrayFails` with diagnostics: name-resolution. The compiler cannot handle these syntax/semantics, preventing compilation of code in this category.

Problem: arrayIndexWithArrayFails has 1 reference failures and needs smart-triage evidence before implementation starts.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/arrayIndexWithArrayFails.ts
```

Coverage window:

```sh
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/arrayIndexWithArrayFails.ts --detail
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
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/arrayIndexWithArrayFails.ts --detail
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/arrayIndexWithArrayFails.ts
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

- `reference/typescript/tests/cases/compiler/arrayIndexWithArrayFails.ts`

## Duplicate detection

- none found by path/title/feature scan

## Smart triage

### Smart triage: Triage name resolution: arrayIndexWithArrayFails

- Issue class: `triage-needed`
- Feature label: `name-resolution`
- Diagnostic: `UnresolvedName` / `resolver-symbol`
- Path: `reference/typescript/tests/cases/compiler/arrayIndexWithArrayFails.ts`

Reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/arrayIndexWithArrayFails.ts
```

Source overview:

```json
{
  "suite": "tsc",
  "bytes": 150,
  "lines": 5,
  "extension": ".ts",
  "first_code_line": "declare const arr1: (string | string[])[];"
}
```

Failure location:

```json
{
  "code": "UnresolvedName",
  "message": "unresolved name: `arr2` at 124..128",
  "span_start": 124,
  "span_end": 128,
  "line": 5,
  "column": 15,
  "feature_label": "name-resolution",
  "error_type": "resolver-symbol"
}
```

Source context:

```text
2 | // @strict: false
3 | declare const arr1: (string | string[])[];
4 | declare const arr2: number[];
5 | const j = arr2[arr1[0]]; // should error
```

Visible symbols before failure:

```json
[
  {
    "kind": "binding",
    "name": "arr1",
    "line": 3,
    "column": 9
  },
  {
    "kind": "binding",
    "name": "arr2",
    "line": 4,
    "column": 9
  },
  {
    "kind": "binding",
    "name": "j",
    "line": 5,
    "column": 1,
    "initializer": "arr2"
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
        kind: Ident(
            "declare",
        ),
        span: Span {
            start: 39,
            end: 46,
        },
    },
    SpannedToken {
        kind: Const,
        span: Span {
            start: 47,
            end: 52,
        },
    },
    SpannedToken {
        kind: Ident(
            "arr1",
        ),
        span: Span {
            start: 53,
            end: 57,
        },
    },
    SpannedToken {
        kind: Colon,
        span: Span {
            start: 57,
            end: 58,
        },
    },
    SpannedToken {
        kind: LeftParen,
        span: Span {
            start: 59,
            end: 60,
        },
    },
    SpannedToken {
        kind: Ident(
            "string",
        ),
        span: Span {
            start: 60,
            end: 66,
        },
    },
    SpannedToken {
        kind: Pipe,
        span: Span {
            start: 67,
            end: 68,
        },
    },
    SpannedToken {
        kind: Ident(
            "string",
        ),
        span: Span {
            start: 69,
            end: 75,
        },
    },
    SpannedToken {
        kind: LeftBracket,
        span: Span {
            start: 75,
            end: 76,
        },
    },
    SpannedToken {
        kind: RightBracket,
        span: Span {
            start: 76,
            end: 77,
        },
    },
    SpannedToken {
        kind: RightParen,
        span: Span {
            start: 77,
            end: 78,
        },
    },
    SpannedToken {
        kind: LeftBracket,
        span: Span {
            start: 78,
            end: 79,
        },
    },
    SpannedToken {
        kind: RightBracket,
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
        kind: Const,
        span: Span {
            start: 91,
            end: 96,
        },
    },
    SpannedToken {
        kind: Ident(
            "arr2",
        ),
        span: Span {
            start: 97,
            end: 101,
        },
    },
    SpannedToken {
        kind: Colon,
        span: Span {
            start: 101,
            end: 102,
        },
    },
    SpannedToken {
        kind: Ident(
            "number",
        ),
        span: Span {
            start: 103,
            end: 109,
        },
    },
    SpannedToken {
        kind: LeftBracket,
        span: Span {
            start: 109,
            end: 110,
        },
    },
    SpannedToken {
        kind: RightBracket,
        span: Span {
            start: 110,
            end: 111,
        },
    },
    SpannedToken {
        kind: Semicolon,
        span: Span {
            start: 111,
            end: 112,
        },
    },
    SpannedToken {
        kind: Const,
        span: Span {
            start: 114,
            end: 119,
        },
    },
    SpannedToken {
        kind: Ident(
            "j",
        ),
        span: Span {
            start: 120,
            end: 121,
        },
    },
    SpannedToken {
        kind: Equal,
        span: Span {
            start: 122,
            end: 123,
        },
    },
    SpannedToken {
        kind: Ident(
            "arr2",
        ),
        sp
```

#### ast

- ok: `True`
- truncated: `False`

```text
== ast ==
[
    Let {
        name: "j",
        expr: Index {
            object: Ident {
                name: "arr2",
                span: Span {
                    start: 124,
                    end: 128,
                },
            },
            index: Index {
                object: Ident {
                    name: "arr1",
                    span: Span {
                        start: 129,
                        end: 133,
                    },
                },
                index: Number {
                    value: 0,
                    span: Span {
                        start: 134,
                        end: 135,
                    },
                },
                span: Span {
                    start: 129,
                    end: 136,
                },
            },
            span: Span {
                start: 124,
                end: 137,
            },
        },
        span: Span {
            start: 114,
            end: 138,
        },
    },
]
```

#### resolved

- ok: `False`
- truncated: `False`

```text
error: [UnresolvedName] unresolved name: `arr2` at 124..128
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
        "code": 2538,
        "category": "Error",
        "message": "Type 'string[]' cannot be used as an index type.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/arrayIndexWithArrayFails.ts",
        "start": 129,
        "length": 7,
        "line": 5,
        "character": 16
      }
    ],
    "hints": [
      {
        "kind": "binding",
        "typeText": "(string | string[])[]",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/arrayIndexWithArrayFails.ts",
        "start": 53,
        "length": 4,
        "line": 3,
        "character": 15,
        "name": "arr1"
      },
      {
        "kind": "binding",
        "typeText": "number[]",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/arrayIndexWithArrayFails.ts",
        "start": 97,
        "length": 4,
        "line": 4,
        "character": 15,
        "name": "arr2"
      },
      {
        "kind": "binding",
        "typeText": "any",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/arrayIndexWithArrayFails.ts",
        "start": 120,
        "length": 1,
        "line": 5,
        "character": 7,
        "name": "j"
      }
    ],
    "typescriptVersion": "6.0.3"
  },
  "ast": {
    "topLevel": [
      {
        "kind": "FirstStatement",
        "text": "declare const arr1: (string | string[])[];",
        "line": 3,
        "character": 1
      },
      {
        "kind": "FirstStatement",
        "text": "declare const arr2: number[];",
        "line": 4,
        "character": 1
      },
      {
        "kind": "FirstStatement",
        "text": "const j = arr2[arr1[0]];",
        "line": 5,
        "character": 1
      }
    ],
    "pathToPosition": [
      {
        "kind": "SourceFile",
        "text": "declare const arr1: (string | string[])[];\r\ndeclare const arr2: number[];\r\nconst j = arr2[arr1[0]]; // should error",
        "line": 3,
        "character": 1
      },
      {
        "kind": "FirstStatement",
        "text": "const j = arr2[arr1[0]];",
        "line": 5,
        "character": 1
      },
      {
        "kind": "VariableDeclarationList",
        "text": "const j = arr2[arr1[0]]",
        "line": 5,
        "character": 1
      },
      {
        "kind": "VariableDeclaration",
        "text": "j = arr2[arr1[0]]",
        "line": 5,
        "character": 7
      },
      {
        "kind": "ElementAccessExpression",
        "text": "arr2[arr1[0]]",
        "line": 5,
        "character": 11
      },
      {
        "kind": "Identifier",
        "text": "arr2",
        "line": 5,
        "character": 11
      }
    ]
  }
}
```

Stack trace:

```text
error: [UnresolvedName] unresolved name: `arr2` at 124..128
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
