---
id: 662
title: "Implement Arrayassignmenttest Import Export"
type: spike
area: frontend/syntax
class: blocked
priority: P1
depends_on: [5001]
blocks: []
created: 2026-05-01
updated: 2026-05-01
---

## Summary

Triage arrayAssignmentTest-import-export across 2 failing reference test cases and split this bucket into implementation-ready child issues.

## Problem

Reference test results show 2 cases fail in directory `arrayAssignmentTest-import-export` with diagnostics: import-export. The compiler cannot handle these syntax/semantics, preventing compilation of code in this category.

Problem: arrayAssignmentTest-import-export has 2 reference failures and needs smart-triage evidence before implementation starts.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/arrayAssignmentTest6.ts
```

Coverage window:

```sh
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/arrayAssignmentTest6.ts --detail
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
mise run reference-coverage -- tsc --limit 4
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/arrayAssignmentTest6.ts --detail
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/arrayAssignmentTest6.ts
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

- `reference/typescript/tests/cases/compiler/arrayAssignmentTest6.ts`
- `reference/typescript/tests/cases/compiler/arrayAssignmentTest5.ts`

## Duplicate detection

- `issues/open/432-implement-import-export.md` - Implement import/export module syntax (same feature label, title overlap)
- `issues/open/457-implement-APISample-import-export.md` - Implement Apisample Import Export (same feature label, title overlap)
- `issues/open/463-implement-FunctionDeclaration-import-export.md` - Implement Functiondeclaration Import Export (same feature label, title overlap)
- `issues/done/055-implement-import-export.md` - Umbrella: implement import and export (same feature label, title overlap)

## Smart triage

### Smart triage: Triage import export: arrayAssignmentTest6

- Issue class: `triage-needed`
- Feature label: `import-export`
- Diagnostic: `UnsupportedModule` / `unsupported-feature-boundary`
- Path: `reference/typescript/tests/cases/compiler/arrayAssignmentTest6.ts`

Reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/arrayAssignmentTest6.ts
```

Source overview:

```json
{
  "suite": "tsc",
  "bytes": 494,
  "lines": 20,
  "extension": ".ts",
  "first_code_line": "namespace Test {"
}
```

Failure location:

```json
{
  "code": "UnsupportedModule",
  "message": "issue-399: TypeScript namespace/internal module declarations require module ownership before runtime lowering at 20..29",
  "span_start": 20,
  "span_end": 29,
  "line": 2,
  "column": 2,
  "feature_label": "import-export",
  "error_type": "unsupported-feature-boundary"
}
```

Source context:

```text
1 | // @target: es2015
2 | namespace Test {
3 |     interface IState {
4 |     }
5 |     interface IToken {
```

Visible symbols before failure:

```json
[]
```

Duplicate candidates:

```json
[
  {
    "state": "open",
    "path": "issues/open/432-implement-import-export.md",
    "title": "Implement import/export module syntax",
    "reason": "same feature label, title overlap"
  },
  {
    "state": "open",
    "path": "issues/open/457-implement-APISample-import-export.md",
    "title": "Implement Apisample Import Export",
    "reason": "same feature label, title overlap"
  },
  {
    "state": "open",
    "path": "issues/open/463-implement-FunctionDeclaration-import-export.md",
    "title": "Implement Functiondeclaration Import Export",
    "reason": "same feature label, title overlap"
  },
  {
    "state": "open",
    "path": "issues/open/543-implement-APISample-import-export.md",
    "title": "Implement Apisample Import Export",
    "reason": "same feature label, title overlap"
  },
  {
    "state": "open",
    "path": "issues/done/549-implement-FunctionDeclaration-import-export.md",
    "title": "Implement Functiondeclaration Import Export",
    "reason": "same feature label, title overlap"
  },
  {
    "state": "done",
    "path": "issues/done/055-implement-import-export.md",
    "title": "Umbrella: implement import and export",
    "reason": "same feature label, title overlap"
  }
]
```

Error-specific suggestions:

- Keep module graph behavior separate from parser syntax unless the diagnostic proves syntax is the blocker.

Compiler dumps:

#### tokens

- ok: `True`
- truncated: `True`

```text
== tokens ==
[
    SpannedToken {
        kind: Ident(
            "namespace",
        ),
        span: Span {
            start: 20,
            end: 29,
        },
    },
    SpannedToken {
        kind: Ident(
            "Test",
        ),
        span: Span {
            start: 30,
            end: 34,
        },
    },
    SpannedToken {
        kind: LeftBrace,
        span: Span {
            start: 35,
            end: 36,
        },
    },
    SpannedToken {
        kind: Ident(
            "interface",
        ),
        span: Span {
            start: 42,
            end: 51,
        },
    },
    SpannedToken {
        kind: Ident(
            "IState",
        ),
        span: Span {
            start: 52,
            end: 58,
        },
    },
    SpannedToken {
        kind: LeftBrace,
        span: Span {
            start: 59,
            end: 60,
        },
    },
    SpannedToken {
        kind: RightBrace,
        span: Span {
            start: 66,
            end: 67,
        },
    },
    SpannedToken {
        kind: Ident(
            "interface",
        ),
        span: Span {
            start: 73,
            end: 82,
        },
    },
    SpannedToken {
        kind: Ident(
            "IToken",
        ),
        span: Span {
            start: 83,
            end: 89,
        },
    },
    SpannedToken {
        kind: LeftBrace,
        span: Span {
            start: 90,
            end: 91,
        },
    },
    SpannedToken {
        kind: Ident(
            "startIndex",
        ),
        span: Span {
            start: 101,
            end: 111,
        },
    },
    SpannedToken {
        kind: Colon,
        span: Span {
            start: 111,
            end: 112,
        },
    },
    SpannedToken {
        kind: Ident(
            "number",
        ),
        span: Span {
            start: 113,
            end: 119,
        },
    },
    SpannedToken {
        kind: Semicolon,
        span: Span {
            start: 119,
            end: 120,
        },
    },
    SpannedToken {
        kind: RightBrace,
        span: Span {
            start: 126,
            end: 127,
        },
    },
    SpannedToken {
        kind: Ident(
            "interface",
        ),
        span: Span {
            start: 133,
            end: 142,
        },
    },
    SpannedToken {
        kind: Ident(
            "ILineTokens",
        ),
        span: Span {
            start: 143,
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
        kind: Ident(
            "tokens",
        ),
        span: Span {
            start: 166,
            end: 172,
        },
    },
    SpannedToken {
        kind: Colon,
        span: Span {
            start: 172,
            end: 173,
        },
    },
    SpannedToken {
        kind: Ident(
            "IToken",
        ),
        span: Span {
            start: 174,
            end: 180,
        },
    },
    SpannedToken {
        kind: LeftBracket,
        span: Span {
            start: 180,
            end: 181,
        },
    },
    SpannedToken {
        kind: RightBracket,
        span: Span {
            start: 181,
            end: 182,
        },
    },
    SpannedToken {
        kind: Semicolon,
        span: Span {
            start: 182,
            end: 183,
        },
    },
    SpannedToken {
        kind: Ident(
```

#### ast

- ok: `False`
- truncated: `False`

```text
error: [UnsupportedModule] issue-399: TypeScript namespace/internal module declarations require module ownership before runtime lowering at 20..29
```

#### resolved

- ok: `False`
- truncated: `False`

```text
error: [UnsupportedModule] issue-399: TypeScript namespace/internal module declarations require module ownership before runtime lowering at 20..29
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
        "message": "Type 'null' is not assignable to type 'ILineTokens'.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/arrayAssignmentTest6.ts",
        "start": 475,
        "length": 6,
        "line": 17,
        "character": 13
      }
    ],
    "hints": [
      {
        "kind": "parameter",
        "typeText": "string",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/arrayAssignmentTest6.ts",
        "start": 259,
        "length": 4,
        "line": 13,
        "character": 18,
        "name": "line"
      },
      {
        "kind": "parameter",
        "typeText": "IState",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/arrayAssignmentTest6.ts",
        "start": 272,
        "length": 5,
        "line": 13,
        "character": 31,
        "name": "state"
      },
      {
        "kind": "parameter",
        "typeText": "boolean",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/arrayAssignmentTest6.ts",
        "start": 286,
        "length": 13,
        "line": 13,
        "character": 45,
        "name": "includeStates"
      },
      {
        "kind": "parameter",
        "typeText": "string",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/arrayAssignmentTest6.ts",
        "start": 395,
        "length": 4,
        "line": 16,
        "character": 25,
        "name": "line"
      },
      {
        "kind": "parameter",
        "typeText": "IToken[]",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/arrayAssignmentTest6.ts",
        "start": 408,
        "length": 6,
        "line": 16,
        "character": 38,
        "name": "tokens"
      },
      {
        "kind": "parameter",
        "typeText": "boolean",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/arrayAssignmentTest6.ts",
        "start": 425,
        "length": 13,
        "line": 16,
        "character": 55,
        "name": "includeStates"
      }
    ],
    "typescriptVersion": "6.0.3"
  },
  "ast": {
    "topLevel": [
      {
        "kind": "ModuleDeclaration",
        "text": "namespace Test {\r\n    interface IState {\r\n    }\r\n    interface IToken {\r\n        startIndex: number;\r\n    }\r\n    interfa",
        "line": 2,
        "character": 1
      }
    ],
    "pathToPosition": [
      {
        "kind": "SourceFile",
        "text": "namespace Test {\r\n    interface IState {\r\n    }\r\n    interface IToken {\r\n        startIndex: number;\r\n    }\r\n    interfa",
        "line": 2,
        "character": 1
      },
      {
        "kind": "ModuleDeclaration",
        "text": "namespace Test {\r\n    interface IState {\r\n    }\r\n    interface IToken {\r\n        startIndex: number;\r\n    }\r\n    interfa",
        "line": 2,
        "character": 1
      }
    ]
  }
}
```

Stack trace:

```text
error: [UnsupportedModule] issue-399: TypeScript namespace/internal module declarations require module ownership before runtime lowering at 20..29
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
