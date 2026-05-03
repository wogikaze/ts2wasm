---
id: 636
title: "Implement Anonterface"
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

Triage anonterface across 1 failing reference test cases and split this bucket into implementation-ready child issues.

## Problem

Reference test results show 1 cases fail in directory `anonterface` with diagnostics: import-export. The compiler cannot handle these syntax/semantics, preventing compilation of code in this category.

Problem: anonterface has 1 reference failures and needs smart-triage evidence before implementation starts.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/anonterface.ts
```

Coverage window:

```sh
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/anonterface.ts --detail
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
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/anonterface.ts --detail
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/anonterface.ts
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

- `reference/typescript/tests/cases/compiler/anonterface.ts`

## Duplicate detection

- `issues/open/178-implement-anonterface.md` - Implement Anonterface (same reference path, same group key, title overlap)

## Smart triage

### Smart triage: Triage import export: anonterface

- Issue class: `triage-needed`
- Feature label: `import-export`
- Diagnostic: `UnsupportedModule` / `unsupported-feature-boundary`
- Path: `reference/typescript/tests/cases/compiler/anonterface.ts`

Reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/anonterface.ts
```

Source overview:

```json
{
  "suite": "tsc",
  "bytes": 221,
  "lines": 14,
  "extension": ".ts",
  "first_code_line": "namespace M {"
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
2 | namespace M {
3 |     export class C {
4 |         m(fn:{ (n:number):string; },n2:number):string {
5 |             return fn(n2);
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
    "path": "issues/open/178-implement-anonterface.md",
    "title": "Implement Anonterface",
    "reason": "same reference path, title overlap"
  },
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
    "path": "issues/open/549-implement-FunctionDeclaration-import-export.md",
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
            "M",
        ),
        span: Span {
            start: 30,
            end: 31,
        },
    },
    SpannedToken {
        kind: LeftBrace,
        span: Span {
            start: 32,
            end: 33,
        },
    },
    SpannedToken {
        kind: Export,
        span: Span {
            start: 39,
            end: 45,
        },
    },
    SpannedToken {
        kind: Class,
        span: Span {
            start: 46,
            end: 51,
        },
    },
    SpannedToken {
        kind: Ident(
            "C",
        ),
        span: Span {
            start: 52,
            end: 53,
        },
    },
    SpannedToken {
        kind: LeftBrace,
        span: Span {
            start: 54,
            end: 55,
        },
    },
    SpannedToken {
        kind: Ident(
            "m",
        ),
        span: Span {
            start: 65,
            end: 66,
        },
    },
    SpannedToken {
        kind: LeftParen,
        span: Span {
            start: 66,
            end: 67,
        },
    },
    SpannedToken {
        kind: Ident(
            "fn",
        ),
        span: Span {
            start: 67,
            end: 69,
        },
    },
    SpannedToken {
        kind: Colon,
        span: Span {
            start: 69,
            end: 70,
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
        kind: LeftParen,
        span: Span {
            start: 72,
            end: 73,
        },
    },
    SpannedToken {
        kind: Ident(
            "n",
        ),
        span: Span {
            start: 73,
            end: 74,
        },
    },
    SpannedToken {
        kind: Colon,
        span: Span {
            start: 74,
            end: 75,
        },
    },
    SpannedToken {
        kind: Ident(
            "number",
        ),
        span: Span {
            start: 75,
            end: 81,
        },
    },
    SpannedToken {
        kind: RightParen,
        span: Span {
            start: 81,
            end: 82,
        },
    },
    SpannedToken {
        kind: Colon,
        span: Span {
            start: 82,
            end: 83,
        },
    },
    SpannedToken {
        kind: Ident(
            "string",
        ),
        span: Span {
            start: 83,
            end: 89,
        },
    },
    SpannedToken {
        kind: Semicolon,
        span: Span {
            start: 89,
            end: 90,
        },
    },
    SpannedToken {
        kind: RightBrace,
        span: Span {
            start: 91,
            end: 92,
        },
    },
    SpannedToken {
        kind: Comma,
        span: Span {
            start: 92,
            end: 93,
        },
    },
    SpannedToken {
        kind: Ident(
            "n2",
        ),
        span: Span {
            start: 93,
            end: 95,
        },
    },
    SpannedToken {
        kind: Colon,
        span: Span {
            start: 95,
            end: 96,
        },
    },
    SpannedToken {
        kind: Ident(
            "number",
        ),
        span: Span {
            start: 96,
            end: 102,
        },
    },
    SpannedToken {
        kind: RightParen,
        span: Span {
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
    "ok": true,
    "diagnostics": [],
    "hints": [
      {
        "kind": "parameter",
        "typeText": "(n: number) => string",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/anonterface.ts",
        "start": 67,
        "length": 2,
        "line": 4,
        "character": 11,
        "name": "fn"
      },
      {
        "kind": "parameter",
        "typeText": "number",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/anonterface.ts",
        "start": 73,
        "length": 1,
        "line": 4,
        "character": 17,
        "name": "n"
      },
      {
        "kind": "parameter",
        "typeText": "number",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/anonterface.ts",
        "start": 93,
        "length": 2,
        "line": 4,
        "character": 37,
        "name": "n2"
      },
      {
        "kind": "binding",
        "typeText": "C",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/anonterface.ts",
        "start": 169,
        "length": 1,
        "line": 10,
        "character": 5,
        "name": "c"
      },
      {
        "kind": "parameter",
        "typeText": "number",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/anonterface.ts",
        "start": 196,
        "length": 1,
        "line": 11,
        "character": 14,
        "name": "n"
      },
      {
        "kind": "binary-expression",
        "typeText": "string",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/anonterface.ts",
        "start": 208,
        "length": 11,
        "line": 11,
        "character": 26,
        "operator": "+",
        "leftType": "\"hello: \"",
        "rightType": "number"
      }
    ],
    "typescriptVersion": "6.0.3"
  },
  "ast": {
    "topLevel": [
      {
        "kind": "ModuleDeclaration",
        "text": "namespace M {\r\n    export class C {\r\n        m(fn:{ (n:number):string; },n2:number):string {\r\n            return fn(n2);",
        "line": 2,
        "character": 1
      },
      {
        "kind": "FirstStatement",
        "text": "var c=new M.C();",
        "line": 10,
        "character": 1
      },
      {
        "kind": "ExpressionStatement",
        "text": "c.m(function(n) { return \"hello: \"+n; },18);",
        "line": 11,
        "character": 1
      }
    ],
    "pathToPosition": [
      {
        "kind": "SourceFile",
        "text": "namespace M {\r\n    export class C {\r\n        m(fn:{ (n:number):string; },n2:number):string {\r\n            return fn(n2);",
        "line": 2,
        "character": 1
      },
      {
        "kind": "ModuleDeclaration",
        "text": "namespace M {\r\n    export class C {\r\n        m(fn:{ (n:number):string; },n2:number):string {\r\n            return fn(n2);",
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
