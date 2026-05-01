---
id: 736
title: "Implement Assignmentnestedinliterals"
type: spike
area: reference/triage
class: triage-needed
priority: P1
depends_on: []
blocks: []
created: 2026-05-01
updated: 2026-05-01
---

## Summary

Triage assignmentNestedInLiterals across 1 failing reference test cases and split this bucket into implementation-ready child issues.

## Problem

Reference test results show 1 cases fail in directory `assignmentNestedInLiterals` with diagnostics: duplicate-local. The compiler cannot handle these syntax/semantics, preventing compilation of code in this category.

Problem: assignmentNestedInLiterals has 1 reference failures and needs smart-triage evidence before implementation starts.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/assignmentNestedInLiterals.ts
```

Coverage window:

```sh
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/assignmentNestedInLiterals.ts --detail
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

- `issues/open/`
- `scripts/run/reference-triage.py`
- `fixtures/`

Do not touch:

- implementation code until the triage report assigns a concrete frontend/runtime/backend owner

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
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/assignmentNestedInLiterals.ts --detail
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/assignmentNestedInLiterals.ts
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

- `reference/typescript/tests/cases/compiler/assignmentNestedInLiterals.ts`

## Duplicate detection

- none found by path/title/feature scan

## Smart triage

### Smart triage: Triage duplicate local: assignmentNestedInLiterals

- Issue class: `triage-needed`
- Feature label: `duplicate-local`
- Diagnostic: `DuplicateLocal` / `compiler-diagnostic`
- Path: `reference/typescript/tests/cases/compiler/assignmentNestedInLiterals.ts`

Reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/assignmentNestedInLiterals.ts
```

Source overview:

```json
{
  "suite": "tsc",
  "bytes": 194,
  "lines": 11,
  "extension": ".ts",
  "first_code_line": "var target, x, y;"
}
```

Failure location:

```json
{
  "code": "DuplicateLocal",
  "message": "duplicate local binding: `kowloona`",
  "span_start": null,
  "span_end": null,
  "line": null,
  "column": null,
  "feature_label": "duplicate-local",
  "error_type": "compiler-diagnostic"
}
```

Source context:

```text
// @target: es2015
// @noImplicitAny: true
var target, x, y;
target = [x = 1, y = x];

var aegis, a, b;
aegis = { x: a = 1, y: b = a };
```

Visible symbols before failure:

```json
[
  {
    "kind": "binding",
    "name": "target",
    "line": 3,
    "column": 1
  },
  {
    "kind": "binding",
    "name": "aegis",
    "line": 6,
    "column": 1
  },
  {
    "kind": "binding",
    "name": "kowloona",
    "line": 9,
    "column": 1
  }
]
```

Duplicate candidates:

```json
[
  {
    "state": "done",
    "path": "issues/done/298-allow-reused-for-loop-local-names.md",
    "title": "Allow reused for-loop local names in separate loop scopes",
    "reason": "same feature label, title overlap"
  }
]
```

Error-specific suggestions:

- Create a child issue around this exact path and diagnostic before broadening the reference window.

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
            start: 43,
            end: 46,
        },
    },
    SpannedToken {
        kind: Ident(
            "target",
        ),
        span: Span {
            start: 47,
            end: 53,
        },
    },
    SpannedToken {
        kind: Comma,
        span: Span {
            start: 53,
            end: 54,
        },
    },
    SpannedToken {
        kind: Ident(
            "x",
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
        kind: Ident(
            "y",
        ),
        span: Span {
            start: 58,
            end: 59,
        },
    },
    SpannedToken {
        kind: Semicolon,
        span: Span {
            start: 59,
            end: 60,
        },
    },
    SpannedToken {
        kind: Ident(
            "target",
        ),
        span: Span {
            start: 61,
            end: 67,
        },
    },
    SpannedToken {
        kind: Equal,
        span: Span {
            start: 68,
            end: 69,
        },
    },
    SpannedToken {
        kind: LeftBracket,
        span: Span {
            start: 70,
            end: 71,
        },
    },
    SpannedToken {
        kind: Ident(
            "x",
        ),
        span: Span {
            start: 71,
            end: 72,
        },
    },
    SpannedToken {
        kind: Equal,
        span: Span {
            start: 73,
            end: 74,
        },
    },
    SpannedToken {
        kind: Number(
            1,
        ),
        span: Span {
            start: 75,
            end: 76,
        },
    },
    SpannedToken {
        kind: Comma,
        span: Span {
            start: 76,
            end: 77,
        },
    },
    SpannedToken {
        kind: Ident(
            "y",
        ),
        span: Span {
            start: 78,
            end: 79,
        },
    },
    SpannedToken {
        kind: Equal,
        span: Span {
            start: 80,
            end: 81,
        },
    },
    SpannedToken {
        kind: Ident(
            "x",
        ),
        span: Span {
            start: 82,
            end: 83,
        },
    },
    SpannedToken {
        kind: RightBracket,
        span: Span {
            start: 83,
            end: 84,
        },
    },
    SpannedToken {
        kind: Semicolon,
        span: Span {
            start: 84,
            end: 85,
        },
    },
    SpannedToken {
        kind: Var,
        span: Span {
            start: 87,
            end: 90,
        },
    },
    SpannedToken {
        kind: Ident(
            "aegis",
        ),
        span: Span {
            start: 91,
            end: 96,
        },
    },
    SpannedToken {
        kind: Comma,
        span: Span {
            start: 96,
            end: 97,
        },
    },
    SpannedToken {
        kind: Ident(
            "a",
        ),
        span: Span {
            start: 98,
            end: 99,
        },
    },
    SpannedToken {
        kind: Comma,
        span: Span {
            start: 99,
            end: 100,
        },
    },
    SpannedToken {
        kind: Ident(
            "b",
        ),
        span: Span {
            start: 101,
            end: 102,
        },
    },
    SpannedToken {
        kind: Semicolon,
        span: Span {
```

#### ast

- ok: `True`
- truncated: `True`

```text
== ast ==
[
    Let {
        name: "target",
        expr: Undefined {
            span: Span {
                start: 47,
                end: 53,
            },
        },
        span: Span {
            start: 43,
            end: 60,
        },
    },
    Let {
        name: "x",
        expr: Undefined {
            span: Span {
                start: 55,
                end: 56,
            },
        },
        span: Span {
            start: 43,
            end: 56,
        },
    },
    Let {
        name: "y",
        expr: Undefined {
            span: Span {
                start: 58,
                end: 59,
            },
        },
        span: Span {
            start: 43,
            end: 59,
        },
    },
    Assign {
        name: "target",
        expr: Array {
            elements: [
                Present(
                    Assign {
                        name: "x",
                        expr: Number {
                            value: 1,
                            span: Span {
                                start: 75,
                                end: 76,
                            },
                        },
                        span: Span {
                            start: 71,
                            end: 76,
                        },
                    },
                ),
                Present(
                    Assign {
                        name: "y",
                        expr: Ident {
                            name: "x",
                            span: Span {
                                start: 82,
                                end: 83,
                            },
                        },
                        span: Span {
                            start: 78,
                            end: 83,
                        },
                    },
                ),
            ],
            span: Span {
                start: 70,
                end: 84,
            },
        },
        span: Span {
            start: 61,
            end: 85,
        },
    },
    Let {
        name: "aegis",
        expr: Undefined {
            span: Span {
                start: 91,
                end: 96,
            },
        },
        span: Span {
            start: 87,
            end: 103,
        },
    },
    Let {
        name: "a",
        expr: Undefined {
            span: Span {
                start: 98,
                end: 99,
            },
        },
        span: Span {
            start: 87,
            end: 99,
        },
    },
    Let {
        name: "b",
        expr: Undefined {
            span: Span {
                start: 101,
                end: 102,
            },
        },
        span: Span {
            start: 87,
            end: 102,
        },
    },
    Assign {
        name: "aegis",
        expr: Object {
            props: [
                (
                    "x",
                    Assign {
                        name: "a",
                        expr: Number {
                            value: 1,
                            span: Span {
                                start: 121,
                                end: 122,
                            },
                        },
                        span: Span {
                            start: 117,
                            end: 122,
                        },
                    },
                ),
                (
                    "y",
```

#### resolved

- ok: `False`
- truncated: `False`

```text
error: [DuplicateLocal] duplicate local binding: `kowloona`
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
        "typeText": "any",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/assignmentNestedInLiterals.ts",
        "start": 47,
        "length": 6,
        "line": 3,
        "character": 5,
        "name": "target"
      },
      {
        "kind": "binding",
        "typeText": "any",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/assignmentNestedInLiterals.ts",
        "start": 55,
        "length": 1,
        "line": 3,
        "character": 13,
        "name": "x"
      },
      {
        "kind": "binding",
        "typeText": "any",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/assignmentNestedInLiterals.ts",
        "start": 58,
        "length": 1,
        "line": 3,
        "character": 16,
        "name": "y"
      },
      {
        "kind": "binding",
        "typeText": "any",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/assignmentNestedInLiterals.ts",
        "start": 91,
        "length": 5,
        "line": 6,
        "character": 5,
        "name": "aegis"
      },
      {
        "kind": "binding",
        "typeText": "any",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/assignmentNestedInLiterals.ts",
        "start": 98,
        "length": 1,
        "line": 6,
        "character": 12,
        "name": "a"
      },
      {
        "kind": "binding",
        "typeText": "any",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/assignmentNestedInLiterals.ts",
        "start": 101,
        "length": 1,
        "line": 6,
        "character": 15,
        "name": "b"
      },
      {
        "kind": "binding",
        "typeText": "any",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/assignmentNestedInLiterals.ts",
        "start": 141,
        "length": 8,
        "line": 9,
        "character": 5,
        "name": "kowloona"
      },
      {
        "kind": "binding",
        "typeText": "any",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/assignmentNestedInLiterals.ts",
        "start": 151,
        "length": 1,
        "line": 9,
        "character": 15,
        "name": "c"
      },
      {
        "kind": "binding",
        "typeText": "any",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/assignmentNestedInLiterals.ts",
        "start": 154,
        "length": 1,
        "line": 9,
        "character": 18,
        "name": "d"
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
