---
id: 670
title: "Implement Arrayconcatmap (audit reopened #670)"
type: spike
area: frontend/syntax
class: blocked
priority: P1
depends_on: [5001]
blocks: []
created: 2026-05-01
updated: 2026-05-05status: open
---

## Summary

Triage arrayConcatMap across 1 failing reference test cases and split this bucket into implementation-ready child issues.

## Problem

Reference test results show 1 cases fail in directory `arrayConcatMap` with diagnostics: method-call. The compiler cannot handle these syntax/semantics, preventing compilation of code in this category.

Problem: arrayConcatMap has 1 reference failures and needs smart-triage evidence before implementation starts.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/arrayConcatMap.ts
```

Coverage window:

```sh
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/arrayConcatMap.ts --detail
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
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/arrayConcatMap.ts --detail
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/arrayConcatMap.ts
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

- `reference/typescript/tests/cases/compiler/arrayConcatMap.ts`

## Duplicate detection

- none found by path/title/feature scan

## Smart triage

### Smart triage: Triage method call: arrayConcatMap

- Issue class: `triage-needed`
- Feature label: `method-call`
- Diagnostic: `UnsupportedSyntax` / `parser-or-frontend-unsupported`
- Path: `reference/typescript/tests/cases/compiler/arrayConcatMap.ts`

Reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/arrayConcatMap.ts
```

Source overview:

```json
{
  "suite": "tsc",
  "bytes": 86,
  "lines": 3,
  "extension": ".ts",
  "first_code_line": "var x = [].concat([{ a: 1 }], [{ a: 2 }])"
}
```

Failure location:

```json
{
  "code": "UnsupportedSyntax",
  "message": "issue-211: method `map` requires an identifier receiver at 28..87",
  "span_start": 28,
  "span_end": 87,
  "line": 2,
  "column": 10,
  "feature_label": "method-call",
  "error_type": "parser-or-frontend-unsupported"
}
```

Source context:

```text
1 | // @target: es2015
2 | var x = [].concat([{ a: 1 }], [{ a: 2 }])
3 |           .map(b => b.a);
```

Visible symbols before failure:

```json
[
  {
    "kind": "binding",
    "name": "x",
    "line": 2,
    "column": 1,
    "initializer": "["
  }
]
```

Duplicate candidates:

```json
[
  {
    "state": "open",
    "path": "issues/open/435-implement-method-call.md",
    "title": "Implement method call support",
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
        kind: LeftBracket,
        span: Span {
            start: 28,
            end: 29,
        },
    },
    SpannedToken {
        kind: RightBracket,
        span: Span {
            start: 29,
            end: 30,
        },
    },
    SpannedToken {
        kind: Dot,
        span: Span {
            start: 30,
            end: 31,
        },
    },
    SpannedToken {
        kind: Ident(
            "concat",
        ),
        span: Span {
            start: 31,
            end: 37,
        },
    },
    SpannedToken {
        kind: LeftParen,
        span: Span {
            start: 37,
            end: 38,
        },
    },
    SpannedToken {
        kind: LeftBracket,
        span: Span {
            start: 38,
            end: 39,
        },
    },
    SpannedToken {
        kind: LeftBrace,
        span: Span {
            start: 39,
            end: 40,
        },
    },
    SpannedToken {
        kind: Ident(
            "a",
        ),
        span: Span {
            start: 41,
            end: 42,
        },
    },
    SpannedToken {
        kind: Colon,
        span: Span {
            start: 42,
            end: 43,
        },
    },
    SpannedToken {
        kind: Number(
            1,
        ),
        span: Span {
            start: 44,
            end: 45,
        },
    },
    SpannedToken {
        kind: RightBrace,
        span: Span {
            start: 46,
            end: 47,
        },
    },
    SpannedToken {
        kind: RightBracket,
        span: Span {
            start: 47,
            end: 48,
        },
    },
    SpannedToken {
        kind: Comma,
        span: Span {
            start: 48,
            end: 49,
        },
    },
    SpannedToken {
        kind: LeftBracket,
        span: Span {
            start: 50,
            end: 51,
        },
    },
    SpannedToken {
        kind: LeftBrace,
        span: Span {
            start: 51,
            end: 52,
        },
    },
    SpannedToken {
        kind: Ident(
            "a",
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
        kind: Number(
            2,
        ),
        span: Span {
            start: 56,
            end: 57,
        },
    },
    SpannedToken {
        kind: RightBrace,
        span: Span {
            start: 58,
            end: 59,
        },
    },
    SpannedToken {
        kind: RightBracket,
        span: Span {
            start: 59,
            end: 60,
        },
    },
    SpannedToken {
        kind: RightParen,
        span: Span {
            start: 60,
            end: 61,
        },
    },
    SpannedToken {
        kind: Dot,
        span: Span {
            start: 73,
            end: 74,
        },
    },
    SpannedToken {
        kind: Ident(
            "map",
        ),
        span: Span {
            start: 74,
            end: 77,
        },
    },
    SpannedToken {
```

#### ast

- ok: `True`
- truncated: `True`

```text
== ast ==
[
    Let {
        name: "x",
        expr: Call {
            callee: Member {
                object: Call {
                    callee: Member {
                        object: Array {
                            elements: [],
                            span: Span {
                                start: 28,
                                end: 30,
                            },
                        },
                        property: "concat",
                        span: Span {
                            start: 28,
                            end: 37,
                        },
                    },
                    args: [
                        Array {
                            elements: [
                                Present(
                                    Object {
                                        props: [
                                            (
                                                "a",
                                                Number {
                                                    value: 1,
                                                    span: Span {
                                                        start: 44,
                                                        end: 45,
                                                    },
                                                },
                                            ),
                                        ],
                                        span: Span {
                                            start: 39,
                                            end: 47,
                                        },
                                    },
                                ),
                            ],
                            span: Span {
                                start: 38,
                                end: 48,
                            },
                        },
                        Array {
                            elements: [
                                Present(
                                    Object {
                                        props: [
                                            (
                                                "a",
                                                Number {
                                                    value: 2,
                                                    span: Span {
                                                        start: 56,
                                                        end: 57,
                                                    },
                                                },
                                            ),
                                        ],
                                        span: Span {
                                            start: 51,
                                            end: 59,
                                        },
                                    },
                                ),
                            ],
                            span: Span {
                                start: 50,
                                end: 60,
                            },
                        },
                    ],
                    span: Span {
                        start: 28,
                        end: 61,
                    },
                },
                property: "map",
                span:
```

#### resolved

- ok: `False`
- truncated: `False`

```text
error: [UnsupportedSyntax] issue-211: method `map` requires an identifier receiver at 28..87
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
        "code": 2769,
        "category": "Error",
        "message": "No overload matches this call.\n  Overload 1 of 2, '(...items: ConcatArray<never>[]): never[]', gave the following error.\n    Type '{ a: number; }' is not assignable to type 'never'.\n  Overload 2 of 2, '(...items: ConcatArray<never>[]): never[]', gave the following error.\n    Type '{ a: number; }' is not assignable to type 'never'.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/arrayConcatMap.ts",
        "start": 39,
        "length": 8,
        "line": 2,
        "character": 20
      },
      {
        "code": 2339,
        "category": "Error",
        "message": "Property 'a' does not exist on type 'never'.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/arrayConcatMap.ts",
        "start": 85,
        "length": 1,
        "line": 3,
        "character": 23
      }
    ],
    "hints": [
      {
        "kind": "binding",
        "typeText": "any[]",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/arrayConcatMap.ts",
        "start": 24,
        "length": 1,
        "line": 2,
        "character": 5,
        "name": "x"
      },
      {
        "kind": "parameter",
        "typeText": "never",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/arrayConcatMap.ts",
        "start": 78,
        "length": 1,
        "line": 3,
        "character": 16,
        "name": "b"
      }
    ],
    "typescriptVersion": "6.0.3"
  },
  "ast": {
    "topLevel": [
      {
        "kind": "FirstStatement",
        "text": "var x = [].concat([{ a: 1 }], [{ a: 2 }])\r\n          .map(b => b.a);",
        "line": 2,
        "character": 1
      }
    ],
    "pathToPosition": [
      {
        "kind": "SourceFile",
        "text": "var x = [].concat([{ a: 1 }], [{ a: 2 }])\r\n          .map(b => b.a);",
        "line": 2,
        "character": 1
      },
      {
        "kind": "FirstStatement",
        "text": "var x = [].concat([{ a: 1 }], [{ a: 2 }])\r\n          .map(b => b.a);",
        "line": 2,
        "character": 1
      },
      {
        "kind": "VariableDeclarationList",
        "text": "var x = [].concat([{ a: 1 }], [{ a: 2 }])\r\n          .map(b => b.a)",
        "line": 2,
        "character": 1
      },
      {
        "kind": "VariableDeclaration",
        "text": "x = [].concat([{ a: 1 }], [{ a: 2 }])\r\n          .map(b => b.a)",
        "line": 2,
        "character": 5
      },
      {
        "kind": "CallExpression",
        "text": "[].concat([{ a: 1 }], [{ a: 2 }])\r\n          .map(b => b.a)",
        "line": 2,
        "character": 9
      },
      {
        "kind": "PropertyAccessExpression",
        "text": "[].concat([{ a: 1 }], [{ a: 2 }])\r\n          .map",
        "line": 2,
        "character": 9
      },
      {
        "kind": "CallExpression",
        "text": "[].concat([{ a: 1 }], [{ a: 2 }])",
        "line": 2,
        "character": 9
      },
      {
        "kind": "PropertyAccessExpression",
        "text": "[].concat",
        "line": 2,
        "character": 9
      },
      {
        "kind": "ArrayLiteralExpression",
        "text": "[]",
        "line": 2,
        "character": 9
      }
    ]
  }
}
```

Stack trace:

```text
error: [UnsupportedSyntax] issue-211: method `map` requires an identifier receiver at 28..87
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

## Reopened by audit

Date: 2026-05-05

Classification: must-reopen.

Reopen reason: frontmatter still says `class: blocked`, which is incompatible with a completed issue unless explicit supersedence/closure evidence is present.

Violated acceptance: the issue cannot provide repo-local close evidence for its checked acceptance criteria while it remains in this state. Acceptance checkboxes were reset for re-verification.

Evidence files:
- `issues/open/670-implement-arrayConcatMap.md` before this move
- `issues/open/670-implement-arrayConcatMap.md` after this move

Split follow-up: none created in this audit wave; this reopened issue remains the tracking item.
