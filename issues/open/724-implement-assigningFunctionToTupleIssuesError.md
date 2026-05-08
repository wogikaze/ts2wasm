---
id: 724
title: "Implement Assigningfunctiontotupleissueserror"
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

Triage assigningFunctionToTupleIssuesError across 1 failing reference test cases and split this bucket into implementation-ready child issues.

## Problem

Reference test results show 1 cases fail in directory `assigningFunctionToTupleIssuesError` with diagnostics: name-resolution. The compiler cannot handle these syntax/semantics, preventing compilation of code in this category.

Problem: assigningFunctionToTupleIssuesError has 1 reference failures and needs smart-triage evidence before implementation starts.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/assigningFunctionToTupleIssuesError.ts
```

Coverage window:

```sh
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/assigningFunctionToTupleIssuesError.ts --detail
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
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/assigningFunctionToTupleIssuesError.ts --detail
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/assigningFunctionToTupleIssuesError.ts
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

- `reference/typescript/tests/cases/compiler/assigningFunctionToTupleIssuesError.ts`

## Duplicate detection

- none found by path/title/feature scan

## Smart triage

### Smart triage: Triage name resolution: assigningFunctionToTupleIssuesError

- Issue class: `triage-needed`
- Feature label: `name-resolution`
- Diagnostic: `UnresolvedName` / `resolver-symbol`
- Path: `reference/typescript/tests/cases/compiler/assigningFunctionToTupleIssuesError.ts`

Reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/assigningFunctionToTupleIssuesError.ts
```

Source overview:

```json
{
  "suite": "tsc",
  "bytes": 66,
  "lines": 3,
  "extension": ".ts",
  "first_code_line": "declare let a: () => void;"
}
```

Failure location:

```json
{
  "code": "UnresolvedName",
  "message": "unresolved name: `a` at 66..67",
  "span_start": 66,
  "span_end": 67,
  "line": 3,
  "column": 21,
  "feature_label": "name-resolution",
  "error_type": "resolver-symbol"
}
```

Source context:

```text
1 | // @target: es2015
2 | declare let a: () => void;
3 | let b: [string] = a;
```

Visible symbols before failure:

```json
[
  {
    "kind": "binding",
    "name": "a",
    "line": 2,
    "column": 9
  },
  {
    "kind": "binding",
    "name": "b",
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
- truncated: `False`

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
        kind: Let,
        span: Span {
            start: 28,
            end: 31,
        },
    },
    SpannedToken {
        kind: Ident(
            "a",
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
        kind: LeftParen,
        span: Span {
            start: 35,
            end: 36,
        },
    },
    SpannedToken {
        kind: RightParen,
        span: Span {
            start: 36,
            end: 37,
        },
    },
    SpannedToken {
        kind: Arrow,
        span: Span {
            start: 38,
            end: 40,
        },
    },
    SpannedToken {
        kind: Void,
        span: Span {
            start: 41,
            end: 45,
        },
    },
    SpannedToken {
        kind: Semicolon,
        span: Span {
            start: 45,
            end: 46,
        },
    },
    SpannedToken {
        kind: Let,
        span: Span {
            start: 48,
            end: 51,
        },
    },
    SpannedToken {
        kind: Ident(
            "b",
        ),
        span: Span {
            start: 52,
            end: 53,
        },
    },
    SpannedToken {
        kind: Colon,
        span: Span {
            start: 53,
            end: 54,
        },
    },
    SpannedToken {
        kind: LeftBracket,
        span: Span {
            start: 55,
            end: 56,
        },
    },
    SpannedToken {
        kind: Ident(
            "string",
        ),
        span: Span {
            start: 56,
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
        kind: Equal,
        span: Span {
            start: 64,
            end: 65,
        },
    },
    SpannedToken {
        kind: Ident(
            "a",
        ),
        span: Span {
            start: 66,
            end: 67,
        },
    },
    SpannedToken {
        kind: Semicolon,
        span: Span {
            start: 67,
            end: 68,
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
        name: "b",
        expr: Ident {
            name: "a",
            span: Span {
                start: 66,
                end: 67,
            },
        },
        span: Span {
            start: 48,
            end: 68,
        },
    },
]
```

#### resolved

- ok: `False`
- truncated: `False`

```text
error: [UnresolvedName] unresolved name: `a` at 66..67
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
        "code": 2322,
        "category": "Error",
        "message": "Type '() => void' is not assignable to type '[string]'.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/assigningFunctionToTupleIssuesError.ts",
        "start": 52,
        "length": 1,
        "line": 3,
        "character": 5
      }
    ],
    "hints": [
      {
        "kind": "binding",
        "typeText": "() => void",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/assigningFunctionToTupleIssuesError.ts",
        "start": 32,
        "length": 1,
        "line": 2,
        "character": 13,
        "name": "a"
      },
      {
        "kind": "binding",
        "typeText": "[string]",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/assigningFunctionToTupleIssuesError.ts",
        "start": 52,
        "length": 1,
        "line": 3,
        "character": 5,
        "name": "b"
      }
    ],
    "typescriptVersion": "6.0.3"
  },
  "ast": {
    "topLevel": [
      {
        "kind": "FirstStatement",
        "text": "declare let a: () => void;",
        "line": 2,
        "character": 1
      },
      {
        "kind": "FirstStatement",
        "text": "let b: [string] = a;",
        "line": 3,
        "character": 1
      }
    ],
    "pathToPosition": [
      {
        "kind": "SourceFile",
        "text": "declare let a: () => void;\r\nlet b: [string] = a;",
        "line": 2,
        "character": 1
      },
      {
        "kind": "FirstStatement",
        "text": "let b: [string] = a;",
        "line": 3,
        "character": 1
      },
      {
        "kind": "VariableDeclarationList",
        "text": "let b: [string] = a",
        "line": 3,
        "character": 1
      },
      {
        "kind": "VariableDeclaration",
        "text": "b: [string] = a",
        "line": 3,
        "character": 5
      },
      {
        "kind": "Identifier",
        "text": "a",
        "line": 3,
        "character": 19
      }
    ]
  }
}
```

Stack trace:

```text
error: [UnresolvedName] unresolved name: `a` at 66..67
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
