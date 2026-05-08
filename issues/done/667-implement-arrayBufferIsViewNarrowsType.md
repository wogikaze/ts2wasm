---
id: 667
title: "Implement Arraybufferisviewnarrowstype"
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

Triage arrayBufferIsViewNarrowsType across 1 failing reference test cases and split this bucket into implementation-ready child issues.

## Problem

Reference test results show 1 cases fail in directory `arrayBufferIsViewNarrowsType` with diagnostics: name-resolution. The compiler cannot handle these syntax/semantics, preventing compilation of code in this category.

Problem: arrayBufferIsViewNarrowsType has 1 reference failures and needs smart-triage evidence before implementation starts.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/arrayBufferIsViewNarrowsType.ts
```

Coverage window:

```sh
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/arrayBufferIsViewNarrowsType.ts --detail
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
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/arrayBufferIsViewNarrowsType.ts --detail
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/arrayBufferIsViewNarrowsType.ts
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

- `reference/typescript/tests/cases/compiler/arrayBufferIsViewNarrowsType.ts`

## Duplicate detection

- none found by path/title/feature scan

## Smart triage

### Smart triage: Triage name resolution: arrayBufferIsViewNarrowsType

- Issue class: `triage-needed`
- Feature label: `name-resolution`
- Diagnostic: `UnresolvedName` / `resolver-symbol`
- Path: `reference/typescript/tests/cases/compiler/arrayBufferIsViewNarrowsType.ts`

Reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/arrayBufferIsViewNarrowsType.ts
```

Source overview:

```json
{
  "suite": "tsc",
  "bytes": 173,
  "lines": 6,
  "extension": ".ts",
  "first_code_line": "var obj: Object;"
}
```

Failure location:

```json
{
  "code": "UnresolvedName",
  "message": "unresolved name: `ArrayBuffer` at 40..51",
  "span_start": 40,
  "span_end": 51,
  "line": 3,
  "column": 5,
  "feature_label": "name-resolution",
  "error_type": "resolver-symbol"
}
```

Source context:

```text
1 | // @target: es2015
2 | var obj: Object;
3 | if (ArrayBuffer.isView(obj)) {
4 |     // isView should be a guard that narrows type to ArrayBufferView.
5 |     var ab: ArrayBufferView = obj;
6 | }
```

Visible symbols before failure:

```json
[
  {
    "kind": "binding",
    "name": "obj",
    "line": 2,
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
- truncated: `False`

```text
== tokens ==
[
    SpannedToken {
        kind: Var,
        span: Span {
            start: 19,
            end: 22,
        },
    },
    SpannedToken {
        kind: Ident(
            "obj",
        ),
        span: Span {
            start: 23,
            end: 26,
        },
    },
    SpannedToken {
        kind: Colon,
        span: Span {
            start: 26,
            end: 27,
        },
    },
    SpannedToken {
        kind: Ident(
            "Object",
        ),
        span: Span {
            start: 28,
            end: 34,
        },
    },
    SpannedToken {
        kind: Semicolon,
        span: Span {
            start: 34,
            end: 35,
        },
    },
    SpannedToken {
        kind: If,
        span: Span {
            start: 36,
            end: 38,
        },
    },
    SpannedToken {
        kind: LeftParen,
        span: Span {
            start: 39,
            end: 40,
        },
    },
    SpannedToken {
        kind: Ident(
            "ArrayBuffer",
        ),
        span: Span {
            start: 40,
            end: 51,
        },
    },
    SpannedToken {
        kind: Dot,
        span: Span {
            start: 51,
            end: 52,
        },
    },
    SpannedToken {
        kind: Ident(
            "isView",
        ),
        span: Span {
            start: 52,
            end: 58,
        },
    },
    SpannedToken {
        kind: LeftParen,
        span: Span {
            start: 58,
            end: 59,
        },
    },
    SpannedToken {
        kind: Ident(
            "obj",
        ),
        span: Span {
            start: 59,
            end: 62,
        },
    },
    SpannedToken {
        kind: RightParen,
        span: Span {
            start: 62,
            end: 63,
        },
    },
    SpannedToken {
        kind: RightParen,
        span: Span {
            start: 63,
            end: 64,
        },
    },
    SpannedToken {
        kind: LeftBrace,
        span: Span {
            start: 65,
            end: 66,
        },
    },
    SpannedToken {
        kind: Var,
        span: Span {
            start: 141,
            end: 144,
        },
    },
    SpannedToken {
        kind: Ident(
            "ab",
        ),
        span: Span {
            start: 145,
            end: 147,
        },
    },
    SpannedToken {
        kind: Colon,
        span: Span {
            start: 147,
            end: 148,
        },
    },
    SpannedToken {
        kind: Ident(
            "ArrayBufferView",
        ),
        span: Span {
            start: 149,
            end: 164,
        },
    },
    SpannedToken {
        kind: Equal,
        span: Span {
            start: 165,
            end: 166,
        },
    },
    SpannedToken {
        kind: Ident(
            "obj",
        ),
        span: Span {
            start: 167,
            end: 170,
        },
    },
    SpannedToken {
        kind: Semicolon,
        span: Span {
            start: 170,
            end: 171,
        },
    },
    SpannedToken {
        kind: RightBrace,
        span: Span {
            start: 172,
            end: 173,
        },
    },
]
```

#### ast

- ok: `True`
- truncated: `False`

```text
== ast ==
[
    Let {
        name: "obj",
        expr: Undefined {
            span: Span {
                start: 23,
                end: 26,
            },
        },
        span: Span {
            start: 19,
            end: 35,
        },
    },
    If {
        condition: Call {
            callee: Member {
                object: Ident {
                    name: "ArrayBuffer",
                    span: Span {
                        start: 40,
                        end: 51,
                    },
                },
                property: "isView",
                span: Span {
                    start: 40,
                    end: 58,
                },
            },
            args: [
                Ident {
                    name: "obj",
                    span: Span {
                        start: 59,
                        end: 62,
                    },
                },
            ],
            span: Span {
                start: 40,
                end: 63,
            },
        },
        then_body: [
            Let {
                name: "ab",
                expr: Ident {
                    name: "obj",
                    span: Span {
                        start: 167,
                        end: 170,
                    },
                },
                span: Span {
                    start: 141,
                    end: 171,
                },
            },
        ],
        else_body: [],
        span: Span {
            start: 36,
            end: 171,
        },
    },
]
```

#### resolved

- ok: `False`
- truncated: `False`

```text
error: [UnresolvedName] unresolved name: `ArrayBuffer` at 40..51
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
        "code": 2454,
        "category": "Error",
        "message": "Variable 'obj' is used before being assigned.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/arrayBufferIsViewNarrowsType.ts",
        "start": 59,
        "length": 3,
        "line": 3,
        "character": 24
      }
    ],
    "hints": [
      {
        "kind": "binding",
        "typeText": "Object",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/arrayBufferIsViewNarrowsType.ts",
        "start": 23,
        "length": 3,
        "line": 2,
        "character": 5,
        "name": "obj"
      },
      {
        "kind": "binding",
        "typeText": "ArrayBufferView<ArrayBufferLike>",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/arrayBufferIsViewNarrowsType.ts",
        "start": 145,
        "length": 2,
        "line": 5,
        "character": 9,
        "name": "ab"
      }
    ],
    "typescriptVersion": "6.0.3"
  },
  "ast": {
    "topLevel": [
      {
        "kind": "FirstStatement",
        "text": "var obj: Object;",
        "line": 2,
        "character": 1
      },
      {
        "kind": "IfStatement",
        "text": "if (ArrayBuffer.isView(obj)) {\n    // isView should be a guard that narrows type to ArrayBufferView.\n    var ab: ArrayBu",
        "line": 3,
        "character": 1
      }
    ],
    "pathToPosition": [
      {
        "kind": "SourceFile",
        "text": "var obj: Object;\nif (ArrayBuffer.isView(obj)) {\n    // isView should be a guard that narrows type to ArrayBufferView.\n  ",
        "line": 2,
        "character": 1
      },
      {
        "kind": "IfStatement",
        "text": "if (ArrayBuffer.isView(obj)) {\n    // isView should be a guard that narrows type to ArrayBufferView.\n    var ab: ArrayBu",
        "line": 3,
        "character": 1
      },
      {
        "kind": "CallExpression",
        "text": "ArrayBuffer.isView(obj)",
        "line": 3,
        "character": 5
      },
      {
        "kind": "PropertyAccessExpression",
        "text": "ArrayBuffer.isView",
        "line": 3,
        "character": 5
      },
      {
        "kind": "Identifier",
        "text": "ArrayBuffer",
        "line": 3,
        "character": 5
      }
    ]
  }
}
```

Stack trace:

```text
error: [UnresolvedName] unresolved name: `ArrayBuffer` at 40..51
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
