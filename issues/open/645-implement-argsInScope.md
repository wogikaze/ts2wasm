---
id: 645
title: "Implement Argsinscope"
type: spike
area: frontend/resolver
class: blocked
priority: P2
depends_on: [5006]
blocks: []
created: 2026-05-01
updated: 2026-05-01
---

## Summary

Triage argsInScope across 1 failing reference test cases and split this bucket into implementation-ready child issues.

## Problem

Reference test results show 1 cases fail in directory `argsInScope` with diagnostics: scope-analysis. The compiler cannot handle these syntax/semantics, preventing compilation of code in this category.

Problem: argsInScope has 1 reference failures and needs smart-triage evidence before implementation starts.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/argsInScope.ts
```

Coverage window:

```sh
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/argsInScope.ts --detail
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
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/argsInScope.ts --detail
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/argsInScope.ts
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

- `reference/typescript/tests/cases/compiler/argsInScope.ts`

## Duplicate detection

- `issues/open/192-implement-argsInScope.md` - Implement Argsinscope (same reference path, same feature label, same group key, title overlap)
- `issues/open/446-implement-scope-analysis.md` - Implement scope-analysis support (same feature label, same group key, title overlap)

## Smart triage

### Smart triage: Triage scope analysis: argsInScope

- Issue class: `triage-needed`
- Feature label: `scope-analysis`
- Diagnostic: `UnsupportedSyntax` / `parser-or-frontend-unsupported`
- Path: `reference/typescript/tests/cases/compiler/argsInScope.ts`

Reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/argsInScope.ts
```

Source overview:

```json
{
  "suite": "tsc",
  "bytes": 221,
  "lines": 11,
  "extension": ".ts",
  "first_code_line": "class C {"
}
```

Failure location:

```json
{
  "code": "UnsupportedSyntax",
  "message": "issue-062d: `arguments` is only supported inside non-arrow functions in this milestone",
  "span_start": null,
  "span_end": null,
  "line": null,
  "column": null,
  "feature_label": "scope-analysis",
  "error_type": "parser-or-frontend-unsupported"
}
```

Source context:

```text
// @target: es2015
class C {
    P(ii:number, j:number, k:number) {
       for (var i = 0; i < arguments.length; i++) {
           // WScript.Echo("param: " + arguments[i]);
       }
    }
}
```

Visible symbols before failure:

```json
[
  {
    "kind": "class",
    "name": "C",
    "line": 2,
    "column": 1
  },
  {
    "kind": "binding",
    "name": "i",
    "line": 4,
    "column": 13,
    "initializer": "0"
  },
  {
    "kind": "binding",
    "name": "c",
    "line": 10,
    "column": 1,
    "initializer": "new C()"
  }
]
```

Duplicate candidates:

```json
[
  {
    "state": "open",
    "path": "issues/open/192-implement-argsInScope.md",
    "title": "Implement Argsinscope",
    "reason": "same reference path, same feature label, title overlap"
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
            start: 20,
            end: 25,
        },
    },
    SpannedToken {
        kind: Ident(
            "C",
        ),
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
            "P",
        ),
        span: Span {
            start: 35,
            end: 36,
        },
    },
    SpannedToken {
        kind: LeftParen,
        span: Span {
            start: 36,
            end: 37,
        },
    },
    SpannedToken {
        kind: Ident(
            "ii",
        ),
        span: Span {
            start: 37,
            end: 39,
        },
    },
    SpannedToken {
        kind: Colon,
        span: Span {
            start: 39,
            end: 40,
        },
    },
    SpannedToken {
        kind: Ident(
            "number",
        ),
        span: Span {
            start: 40,
            end: 46,
        },
    },
    SpannedToken {
        kind: Comma,
        span: Span {
            start: 46,
            end: 47,
        },
    },
    SpannedToken {
        kind: Ident(
            "j",
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
            "number",
        ),
        span: Span {
            start: 50,
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
        kind: Ident(
            "k",
        ),
        span: Span {
            start: 58,
            end: 59,
        },
    },
    SpannedToken {
        kind: Colon,
        span: Span {
            start: 59,
            end: 60,
        },
    },
    SpannedToken {
        kind: Ident(
            "number",
        ),
        span: Span {
            start: 60,
            end: 66,
        },
    },
    SpannedToken {
        kind: RightParen,
        span: Span {
            start: 66,
            end: 67,
        },
    },
    SpannedToken {
        kind: LeftBrace,
        span: Span {
            start: 68,
            end: 69,
        },
    },
    SpannedToken {
        kind: For,
        span: Span {
            start: 78,
            end: 81,
        },
    },
    SpannedToken {
        kind: LeftParen,
        span: Span {
            start: 82,
            end: 83,
        },
    },
    SpannedToken {
        kind: Var,
        span: Span {
            start: 83,
            end: 86,
        },
    },
    SpannedToken {
        kind: Ident(
            "i",
        ),
        span: Span {
            start: 87,
            end: 88,
        },
    },
    SpannedToken {
        kind: Equal,
        span: Span {
            start: 89,
            end: 90,
        },
    },
    SpannedToken {
        kind: Number(
            0,
        ),
        span: Span {
            start: 91,
            end: 92,
        },
    },
    SpannedToken {
        kind: Semicolon,
        span: Span {
            start: 92,
            end: 93,
        },
    },
    SpannedToken {
        kind: Ident(
            "i",
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
        name: "C",
        extends: None,
        body: [
            Function {
                name: "P",
                params: [
                    (
                        "ii",
                        None,
                        false,
                    ),
                    (
                        "j",
                        None,
                        false,
                    ),
                    (
                        "k",
                        None,
                        false,
                    ),
                ],
                body: [
                    For {
                        init: Some(
                            Let {
                                name: "i",
                                expr: Number {
                                    value: 0,
                                    span: Span {
                                        start: 91,
                                        end: 92,
                                    },
                                },
                                span: Span {
                                    start: 83,
                                    end: 93,
                                },
                            },
                        ),
                        condition: Some(
                            Binary {
                                left: Ident {
                                    name: "i",
                                    span: Span {
                                        start: 94,
                                        end: 95,
                                    },
                                },
                                op: Less,
                                right: Member {
                                    object: Ident {
                                        name: "arguments",
                                        span: Span {
                                            start: 98,
                                            end: 107,
                                        },
                                    },
                                    property: "length",
                                    span: Span {
                                        start: 98,
                                        end: 114,
                                    },
                                },
                                span: Span {
                                    start: 94,
                                    end: 114,
                                },
                            },
                        ),
                        update: Some(
                            Unary {
                                op: Increment,
                                expr: Ident {
                                    name: "i",
                                    span: Span {
                                        start: 116,
                                        end: 117,
                                    },
                                },
                                span: Span {
                                    start: 116,
                                    end: 119,
                                },
                            },
                        ),
                        body: [],
                        span: Span {
                            start: 78,
                            end: 81,
                        },
```

#### resolved

- ok: `False`
- truncated: `False`

```text
error: [UnsupportedSyntax] issue-062d: `arguments` is only supported inside non-arrow functions in this milestone
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
        "kind": "parameter",
        "typeText": "number",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/argsInScope.ts",
        "start": 37,
        "length": 2,
        "line": 3,
        "character": 7,
        "name": "ii"
      },
      {
        "kind": "parameter",
        "typeText": "number",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/argsInScope.ts",
        "start": 48,
        "length": 1,
        "line": 3,
        "character": 18,
        "name": "j"
      },
      {
        "kind": "parameter",
        "typeText": "number",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/argsInScope.ts",
        "start": 58,
        "length": 1,
        "line": 3,
        "character": 28,
        "name": "k"
      },
      {
        "kind": "binding",
        "typeText": "number",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/argsInScope.ts",
        "start": 87,
        "length": 1,
        "line": 4,
        "character": 17,
        "name": "i"
      },
      {
        "kind": "binding",
        "typeText": "C",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/argsInScope.ts",
        "start": 205,
        "length": 1,
        "line": 10,
        "character": 5,
        "name": "c"
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
