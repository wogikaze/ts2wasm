---
id: 657
title: "Implement Argumentsreferenceinmethod Name Resolution"
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

Triage argumentsReferenceInMethod-name-resolution across 1 failing reference test cases and split this bucket into implementation-ready child issues.

## Problem

Reference test results show 1 cases fail in directory `argumentsReferenceInMethod-name-resolution` with diagnostics: name-resolution. The compiler cannot handle these syntax/semantics, preventing compilation of code in this category.

Problem: argumentsReferenceInMethod-name-resolution has 1 reference failures and needs smart-triage evidence before implementation starts.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/argumentsReferenceInMethod3_Js.ts
```

Coverage window:

```sh
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/argumentsReferenceInMethod3_Js.ts --detail
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
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/argumentsReferenceInMethod3_Js.ts --detail
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/argumentsReferenceInMethod3_Js.ts
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

- `reference/typescript/tests/cases/compiler/argumentsReferenceInMethod3_Js.ts`

## Duplicate detection

- `issues/open/064-implement-name-resolution.md` - Implement name resolution (triaged - superseded by test262 metadata issues) (same feature label, title overlap)
- `issues/done/437-implement-name-resolution.md` - Implement name resolution (same feature label, title overlap)

## Smart triage

### Smart triage: Triage name resolution: argumentsReferenceInMethod3 Js

- Issue class: `triage-needed`
- Feature label: `name-resolution`
- Diagnostic: `UnresolvedName` / `resolver-symbol`
- Path: `reference/typescript/tests/cases/compiler/argumentsReferenceInMethod3_Js.ts`

Reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/argumentsReferenceInMethod3_Js.ts
```

Source overview:

```json
{
  "suite": "tsc",
  "bytes": 355,
  "lines": 28,
  "extension": ".ts",
  "first_code_line": "class A {"
}
```

Failure location:

```json
{
  "code": "UnresolvedName",
  "message": "unresolved name: `super`",
  "span_start": null,
  "span_end": null,
  "line": null,
  "column": null,
  "feature_label": "name-resolution",
  "error_type": "resolver-symbol"
}
```

Source context:

```text
// @target: es2015
// @declaration: true
// @allowJs: true
// @emitDeclarationOnly: true

// @filename: /a.js
class A {
 get arguments() {
```

Visible symbols before failure:

```json
[
  {
    "kind": "class",
    "name": "A",
    "line": 7,
    "column": 1
  },
  {
    "kind": "class",
    "name": "B",
    "line": 13,
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
        kind: Class,
        span: Span {
            start: 110,
            end: 115,
        },
    },
    SpannedToken {
        kind: Ident(
            "A",
        ),
        span: Span {
            start: 116,
            end: 117,
        },
    },
    SpannedToken {
        kind: LeftBrace,
        span: Span {
            start: 118,
            end: 119,
        },
    },
    SpannedToken {
        kind: Ident(
            "get",
        ),
        span: Span {
            start: 121,
            end: 124,
        },
    },
    SpannedToken {
        kind: Ident(
            "arguments",
        ),
        span: Span {
            start: 125,
            end: 134,
        },
    },
    SpannedToken {
        kind: LeftParen,
        span: Span {
            start: 134,
            end: 135,
        },
    },
    SpannedToken {
        kind: RightParen,
        span: Span {
            start: 135,
            end: 136,
        },
    },
    SpannedToken {
        kind: LeftBrace,
        span: Span {
            start: 137,
            end: 138,
        },
    },
    SpannedToken {
        kind: Return,
        span: Span {
            start: 141,
            end: 147,
        },
    },
    SpannedToken {
        kind: LeftBrace,
        span: Span {
            start: 148,
            end: 149,
        },
    },
    SpannedToken {
        kind: Ident(
            "bar",
        ),
        span: Span {
            start: 150,
            end: 153,
        },
    },
    SpannedToken {
        kind: Colon,
        span: Span {
            start: 153,
            end: 154,
        },
    },
    SpannedToken {
        kind: LeftBrace,
        span: Span {
            start: 155,
            end: 156,
        },
    },
    SpannedToken {
        kind: RightBrace,
        span: Span {
            start: 156,
            end: 157,
        },
    },
    SpannedToken {
        kind: RightBrace,
        span: Span {
            start: 158,
            end: 159,
        },
    },
    SpannedToken {
        kind: Semicolon,
        span: Span {
            start: 159,
            end: 160,
        },
    },
    SpannedToken {
        kind: RightBrace,
        span: Span {
            start: 162,
            end: 163,
        },
    },
    SpannedToken {
        kind: RightBrace,
        span: Span {
            start: 164,
            end: 165,
        },
    },
    SpannedToken {
        kind: Class,
        span: Span {
            start: 167,
            end: 172,
        },
    },
    SpannedToken {
        kind: Ident(
            "B",
        ),
        span: Span {
            start: 173,
            end: 174,
        },
    },
    SpannedToken {
        kind: Extends,
        span: Span {
            start: 175,
            end: 182,
        },
    },
    SpannedToken {
        kind: Ident(
            "A",
        ),
        span: Span {
            start: 183,
            end: 184,
        },
    },
    SpannedToken {
        kind: LeftBrace,
        span: Span {
            start: 185,
            end: 186,
        },
    },
    SpannedToken {
        kind: Ident(
            "m",
        ),
        span: Span {
            start: 227,
            end: 228,
        },
    },
    SpannedToken {
        kind: LeftParen,
        span: Span {
            start: 228,
            end: 229,
        },
    },
    SpannedToken {
        kind: Ident(
            "foo",
        ),
        span: Span {
```

#### ast

- ok: `True`
- truncated: `True`

```text
== ast ==
[
    ClassDecl {
        name: "A",
        extends: None,
        body: [
            Function {
                name: "arguments",
                params: [],
                body: [
                    Return {
                        expr: Object {
                            props: [
                                (
                                    "bar",
                                    Object {
                                        props: [],
                                        span: Span {
                                            start: 155,
                                            end: 157,
                                        },
                                    },
                                ),
                            ],
                            span: Span {
                                start: 148,
                                end: 159,
                            },
                        },
                        span: Span {
                            start: 141,
                            end: 160,
                        },
                    },
                ],
                is_generator: false,
                span: Span {
                    start: 125,
                    end: 160,
                },
            },
        ],
        static_blocks: [],
        private_elements: [],
        span: Span {
            start: 110,
            end: 165,
        },
    },
    ClassDecl {
        name: "B",
        extends: Some(
            Ident {
                name: "A",
                span: Span {
                    start: 183,
                    end: 184,
                },
            },
        ),
        body: [
            Function {
                name: "m",
                params: [
                    (
                        "foo",
                        Some(
                            Object {
                                props: [],
                                span: Span {
                                    start: 235,
                                    end: 237,
                                },
                            },
                        ),
                        false,
                    ),
                ],
                body: [
                    Expr {
                        expr: PropertyAssign {
                            object: This {
                                span: Span {
                                    start: 273,
                                    end: 277,
                                },
                            },
                            property: "x",
                            value: Ident {
                                name: "foo",
                                span: Span {
                                    start: 282,
                                    end: 285,
                                },
                            },
                            span: Span {
                                start: 273,
                                end: 286,
                            },
                        },
                        span: Span {
                            start: 273,
                            end: 286,
                        },
                    },
                    Expr {
                        expr: PropertyAssign {
                            object: This {
                                span: Span {
```

#### resolved

- ok: `False`
- truncated: `False`

```text
error: [UnresolvedName] unresolved name: `super`
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
        "code": 2339,
        "category": "Error",
        "message": "Property 'x' does not exist on type 'B'.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/argumentsReferenceInMethod3_Js.ts",
        "start": 278,
        "length": 1,
        "line": 21,
        "character": 8
      },
      {
        "code": 2339,
        "category": "Error",
        "message": "Property 'y' does not exist on type 'B'.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/argumentsReferenceInMethod3_Js.ts",
        "start": 325,
        "length": 1,
        "line": 26,
        "character": 8
      }
    ],
    "hints": [
      {
        "kind": "parameter",
        "typeText": "{}",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/argumentsReferenceInMethod3_Js.ts",
        "start": 229,
        "length": 3,
        "line": 17,
        "character": 4,
        "name": "foo"
      }
    ],
    "typescriptVersion": "6.0.3"
  }
}
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
