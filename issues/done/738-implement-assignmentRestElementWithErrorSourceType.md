---
id: 738
title: "Implement Assignmentrestelementwitherrorsourcetype"
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

Triage assignmentRestElementWithErrorSourceType across 1 failing reference test cases and split this bucket into implementation-ready child issues.

## Problem

Reference test results show 1 cases fail in directory `assignmentRestElementWithErrorSourceType` with diagnostics: name-resolution. The compiler cannot handle these syntax/semantics, preventing compilation of code in this category.

Problem: assignmentRestElementWithErrorSourceType has 1 reference failures and needs smart-triage evidence before implementation starts.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/assignmentRestElementWithErrorSourceType.ts
```

Coverage window:

```sh
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/assignmentRestElementWithErrorSourceType.ts --detail
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
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/assignmentRestElementWithErrorSourceType.ts --detail
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/assignmentRestElementWithErrorSourceType.ts
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

- `reference/typescript/tests/cases/compiler/assignmentRestElementWithErrorSourceType.ts`

## Duplicate detection

- none found by path/title/feature scan

## Smart triage

### Smart triage: Triage name resolution: assignmentRestElementWithErrorSourceType

- Issue class: `triage-needed`
- Feature label: `name-resolution`
- Diagnostic: `UnresolvedName` / `resolver-symbol`
- Path: `reference/typescript/tests/cases/compiler/assignmentRestElementWithErrorSourceType.ts`

Reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/assignmentRestElementWithErrorSourceType.ts
```

Source overview:

```json
{
  "suite": "tsc",
  "bytes": 91,
  "lines": 3,
  "extension": ".ts",
  "first_code_line": "var tuple: [string, number];"
}
```

Failure location:

```json
{
  "code": "UnresolvedName",
  "message": "unresolved name: `[...c]` at 50..64",
  "span_start": 50,
  "span_end": 64,
  "line": 3,
  "column": 3,
  "feature_label": "name-resolution",
  "error_type": "resolver-symbol"
}
```

Source context:

```text
1 | // @target: es2015
2 | var tuple: [string, number];
3 | [...c] = tupel; // intentionally misspelled
```

Visible symbols before failure:

```json
[
  {
    "kind": "binding",
    "name": "tuple",
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
  },
  {
    "state": "open",
    "path": "issues/open/693-implement-arrayToLocaleStringES-name-resolution.md",
    "title": "Implement Arraytolocalestringes Name Resolution",
    "reason": "same feature label, title overlap"
  },
  {
    "state": "open",
    "path": "issues/open/733-implement-assignmentCompatability-name-resolution.md",
    "title": "Implement Assignmentcompatability Name Resolution",
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
            start: 20,
            end: 23,
        },
    },
    SpannedToken {
        kind: Ident(
            "tuple",
        ),
        span: Span {
            start: 24,
            end: 29,
        },
    },
    SpannedToken {
        kind: Colon,
        span: Span {
            start: 29,
            end: 30,
        },
    },
    SpannedToken {
        kind: LeftBracket,
        span: Span {
            start: 31,
            end: 32,
        },
    },
    SpannedToken {
        kind: Ident(
            "string",
        ),
        span: Span {
            start: 32,
            end: 38,
        },
    },
    SpannedToken {
        kind: Comma,
        span: Span {
            start: 38,
            end: 39,
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
        kind: RightBracket,
        span: Span {
            start: 46,
            end: 47,
        },
    },
    SpannedToken {
        kind: Semicolon,
        span: Span {
            start: 47,
            end: 48,
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
        kind: DotDotDot,
        span: Span {
            start: 51,
            end: 54,
        },
    },
    SpannedToken {
        kind: Ident(
            "c",
        ),
        span: Span {
            start: 54,
            end: 55,
        },
    },
    SpannedToken {
        kind: RightBracket,
        span: Span {
            start: 55,
            end: 56,
        },
    },
    SpannedToken {
        kind: Equal,
        span: Span {
            start: 57,
            end: 58,
        },
    },
    SpannedToken {
        kind: Ident(
            "tupel",
        ),
        span: Span {
            start: 59,
            end: 64,
        },
    },
    SpannedToken {
        kind: Semicolon,
        span: Span {
            start: 64,
            end: 65,
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
        name: "tuple",
        expr: Undefined {
            span: Span {
                start: 24,
                end: 29,
            },
        },
        span: Span {
            start: 20,
            end: 48,
        },
    },
    Expr {
        expr: Assign {
            name: "[...c]",
            expr: Ident {
                name: "tupel",
                span: Span {
                    start: 59,
                    end: 64,
                },
            },
            span: Span {
                start: 50,
                end: 64,
            },
        },
        span: Span {
            start: 50,
            end: 65,
        },
    },
]
```

#### resolved

- ok: `False`
- truncated: `False`

```text
error: [UnresolvedName] unresolved name: `[...c]` at 50..64
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
        "code": 2304,
        "category": "Error",
        "message": "Cannot find name 'c'.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/assignmentRestElementWithErrorSourceType.ts",
        "start": 54,
        "length": 1,
        "line": 3,
        "character": 5
      },
      {
        "code": 2552,
        "category": "Error",
        "message": "Cannot find name 'tupel'. Did you mean 'tuple'?",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/assignmentRestElementWithErrorSourceType.ts",
        "start": 59,
        "length": 5,
        "line": 3,
        "character": 10
      }
    ],
    "hints": [
      {
        "kind": "binding",
        "typeText": "[string, number]",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/assignmentRestElementWithErrorSourceType.ts",
        "start": 24,
        "length": 5,
        "line": 2,
        "character": 5,
        "name": "tuple"
      }
    ],
    "typescriptVersion": "6.0.3"
  },
  "ast": {
    "topLevel": [
      {
        "kind": "FirstStatement",
        "text": "var tuple: [string, number];",
        "line": 2,
        "character": 1
      },
      {
        "kind": "ExpressionStatement",
        "text": "[...c] = tupel;",
        "line": 3,
        "character": 1
      }
    ],
    "pathToPosition": [
      {
        "kind": "SourceFile",
        "text": "var tuple: [string, number];\r\n[...c] = tupel; // intentionally misspelled",
        "line": 2,
        "character": 1
      },
      {
        "kind": "ExpressionStatement",
        "text": "[...c] = tupel;",
        "line": 3,
        "character": 1
      },
      {
        "kind": "BinaryExpression",
        "text": "[...c] = tupel",
        "line": 3,
        "character": 1
      },
      {
        "kind": "ArrayLiteralExpression",
        "text": "[...c]",
        "line": 3,
        "character": 1
      }
    ]
  }
}
```

Stack trace:

```text
error: [UnresolvedName] unresolved name: `[...c]` at 50..64
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
