---
id: 743
title: "Implement Assignmenttofunction"
type: spike
area: frontend/syntax
class: blocked
priority: P1
depends_on: [432]
blocks: []
created: 2026-05-01
updated: 2026-05-01
---

## Summary

Triage assignmentToFunction across 1 failing reference test cases and split this bucket into implementation-ready child issues.

## Problem

Reference test results show 1 cases fail in directory `assignmentToFunction` with diagnostics: import-export. The compiler cannot handle these syntax/semantics, preventing compilation of code in this category.

Problem: assignmentToFunction has 1 reference failures and needs smart-triage evidence before implementation starts.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/assignmentToFunction.ts
```

Coverage window:

```sh
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/assignmentToFunction.ts --detail
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
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/assignmentToFunction.ts --detail
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/assignmentToFunction.ts
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

- `reference/typescript/tests/cases/compiler/assignmentToFunction.ts`

## Duplicate detection

- none found by path/title/feature scan

## Smart triage

### Smart triage: Triage import export: assignmentToFunction

- Issue class: `triage-needed`
- Feature label: `import-export`
- Diagnostic: `UnsupportedModule` / `unsupported-feature-boundary`
- Path: `reference/typescript/tests/cases/compiler/assignmentToFunction.ts`

Reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/assignmentToFunction.ts
```

Source overview:

```json
{
  "suite": "tsc",
  "bytes": 151,
  "lines": 11,
  "extension": ".ts",
  "first_code_line": "function fn() { }"
}
```

Failure location:

```json
{
  "code": "UnsupportedModule",
  "message": "issue-399: TypeScript namespace/internal module declarations require module ownership before runtime lowering at 56..65",
  "span_start": 56,
  "span_end": 65,
  "line": 5,
  "column": 5,
  "feature_label": "import-export",
  "error_type": "unsupported-feature-boundary"
}
```

Source context:

```text
2 | function fn() { }
3 | fn = () => 3;
4 | 
5 | namespace foo {
6 |     function xyz() {
7 |         function bar() {
8 |         }
```

Visible symbols before failure:

```json
[
  {
    "kind": "function",
    "name": "fn",
    "line": 2,
    "column": 1,
    "params": ""
  }
]
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
    "path": "issues/done/457-implement-APISample-import-export.md",
    "title": "Implement Apisample Import Export",
    "reason": "same feature label, title overlap"
  },
  {
    "state": "open",
    "path": "issues/done/463-implement-FunctionDeclaration-import-export.md",
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
    "path": "issues/open/549-implement-FunctionDeclaration-import-export.md",
    "title": "Implement Functiondeclaration Import Export",
    "reason": "same feature label, title overlap"
  },
  {
    "state": "open",
    "path": "issues/open/662-implement-arrayAssignmentTest-import-export.md",
    "title": "Implement Arrayassignmenttest Import Export",
    "reason": "same feature label, title overlap"
  },
  {
    "state": "open",
    "path": "issues/open/732-implement-assignmentCompatability-import-export.md",
    "title": "Implement Assignmentcompatability Import Export",
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
        kind: Function,
        span: Span {
            start: 20,
            end: 28,
        },
    },
    SpannedToken {
        kind: Ident(
            "fn",
        ),
        span: Span {
            start: 29,
            end: 31,
        },
    },
    SpannedToken {
        kind: LeftParen,
        span: Span {
            start: 31,
            end: 32,
        },
    },
    SpannedToken {
        kind: RightParen,
        span: Span {
            start: 32,
            end: 33,
        },
    },
    SpannedToken {
        kind: LeftBrace,
        span: Span {
            start: 34,
            end: 35,
        },
    },
    SpannedToken {
        kind: RightBrace,
        span: Span {
            start: 36,
            end: 37,
        },
    },
    SpannedToken {
        kind: Ident(
            "fn",
        ),
        span: Span {
            start: 39,
            end: 41,
        },
    },
    SpannedToken {
        kind: Equal,
        span: Span {
            start: 42,
            end: 43,
        },
    },
    SpannedToken {
        kind: LeftParen,
        span: Span {
            start: 44,
            end: 45,
        },
    },
    SpannedToken {
        kind: RightParen,
        span: Span {
            start: 45,
            end: 46,
        },
    },
    SpannedToken {
        kind: Arrow,
        span: Span {
            start: 47,
            end: 49,
        },
    },
    SpannedToken {
        kind: Number(
            3,
        ),
        span: Span {
            start: 50,
            end: 51,
        },
    },
    SpannedToken {
        kind: Semicolon,
        span: Span {
            start: 51,
            end: 52,
        },
    },
    SpannedToken {
        kind: Ident(
            "namespace",
        ),
        span: Span {
            start: 56,
            end: 65,
        },
    },
    SpannedToken {
        kind: Ident(
            "foo",
        ),
        span: Span {
            start: 66,
            end: 69,
        },
    },
    SpannedToken {
        kind: LeftBrace,
        span: Span {
            start: 70,
            end: 71,
        },
    },
    SpannedToken {
        kind: Function,
        span: Span {
            start: 77,
            end: 85,
        },
    },
    SpannedToken {
        kind: Ident(
            "xyz",
        ),
        span: Span {
            start: 86,
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
        kind: LeftBrace,
        span: Span {
            start: 92,
            end: 93,
        },
    },
    SpannedToken {
        kind: Function,
        span: Span {
            start: 103,
            end: 111,
        },
    },
    SpannedToken {
        kind: Ident(
            "bar",
        ),
        span: Span {
            start: 112,
            end: 115,
        },
    },
    SpannedToken {
        kind: LeftParen,
        span: Span {
            start: 115,
            end: 116,
        },
    },
    SpannedToken {
        kind: RightParen,
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
```

#### ast

- ok: `False`
- truncated: `False`

```text
error: [UnsupportedModule] issue-399: TypeScript namespace/internal module declarations require module ownership before runtime lowering at 56..65
```

#### resolved

- ok: `False`
- truncated: `False`

```text
error: [UnsupportedModule] issue-399: TypeScript namespace/internal module declarations require module ownership before runtime lowering at 56..65
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
        "code": 2630,
        "category": "Error",
        "message": "Cannot assign to 'fn' because it is a function.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/assignmentToFunction.ts",
        "start": 39,
        "length": 2,
        "line": 3,
        "character": 1
      },
      {
        "code": 2630,
        "category": "Error",
        "message": "Cannot assign to 'bar' because it is a function.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/assignmentToFunction.ts",
        "start": 140,
        "length": 3,
        "line": 9,
        "character": 9
      }
    ],
    "hints": [
      {
        "kind": "function",
        "typeText": "void",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/assignmentToFunction.ts",
        "start": 29,
        "length": 2,
        "line": 2,
        "character": 10,
        "name": "fn"
      },
      {
        "kind": "function",
        "typeText": "void",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/assignmentToFunction.ts",
        "start": 86,
        "length": 3,
        "line": 6,
        "character": 14,
        "name": "xyz"
      },
      {
        "kind": "function",
        "typeText": "void",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/assignmentToFunction.ts",
        "start": 112,
        "length": 3,
        "line": 7,
        "character": 18,
        "name": "bar"
      }
    ],
    "typescriptVersion": "6.0.3"
  },
  "ast": {
    "topLevel": [
      {
        "kind": "FunctionDeclaration",
        "text": "function fn() { }",
        "line": 2,
        "character": 1
      },
      {
        "kind": "ExpressionStatement",
        "text": "fn = () => 3;",
        "line": 3,
        "character": 1
      },
      {
        "kind": "ModuleDeclaration",
        "text": "namespace foo {\r\n    function xyz() {\r\n        function bar() {\r\n        }\r\n        bar = null;\r\n    }\r\n}",
        "line": 5,
        "character": 1
      }
    ],
    "pathToPosition": [
      {
        "kind": "SourceFile",
        "text": "function fn() { }\r\nfn = () => 3;\r\n\r\nnamespace foo {\r\n    function xyz() {\r\n        function bar() {\r\n        }\r\n        ",
        "line": 2,
        "character": 1
      },
      {
        "kind": "ModuleDeclaration",
        "text": "namespace foo {\r\n    function xyz() {\r\n        function bar() {\r\n        }\r\n        bar = null;\r\n    }\r\n}",
        "line": 5,
        "character": 1
      }
    ]
  }
}
```

Stack trace:

```text
error: [UnsupportedModule] issue-399: TypeScript namespace/internal module declarations require module ownership before runtime lowering at 56..65
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
