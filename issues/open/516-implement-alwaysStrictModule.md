---
id: 516
title: "Implement Alwaysstrictmodule"
type: spike
area: frontend/syntax
class: triage-needed
priority: P1
depends_on: []
blocks: []
created: 2026-05-01
updated: 2026-05-01
---

## Summary

Triage alwaysStrictModule across 6 failing reference test cases and split this bucket into implementation-ready child issues.

## Problem

Reference test results show 6 cases fail in directory `alwaysStrictModule` with diagnostics: import-export. The compiler cannot handle these syntax/semantics, preventing compilation of code in this category.

Problem: alwaysStrictModule has 6 reference failures and needs smart-triage evidence before implementation starts.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/alwaysStrictModule.ts
```

Coverage window:

```sh
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/alwaysStrictModule.ts --detail
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
mise run reference-coverage -- tsc --limit 12
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/alwaysStrictModule.ts --detail
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/alwaysStrictModule.ts
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

- `reference/typescript/tests/cases/compiler/alwaysStrictModule.ts`
- `reference/typescript/tests/cases/compiler/alwaysStrictModule2.ts`
- `reference/typescript/tests/cases/compiler/alwaysStrictModule3.ts`
- `reference/typescript/tests/cases/compiler/alwaysStrictModule4.ts`
- `reference/typescript/tests/cases/compiler/alwaysStrictModule5.ts`
- `reference/typescript/tests/cases/compiler/alwaysStrictModule6.ts`

## Duplicate detection

- `issues/open/138-implement-alwaysStrictModule.md` - Implement Alwaysstrictmodule (same reference path, same feature label, same group key, title overlap)
- `issues/open/432-implement-import-export.md` - Implement import/export module syntax (same feature label, same group key, title overlap)

## Smart triage

### Smart triage: Triage import export: alwaysStrictModule

- Issue class: `triage-needed`
- Feature label: `import-export`
- Diagnostic: `UnsupportedModule` / `unsupported-feature-boundary`
- Path: `reference/typescript/tests/cases/compiler/alwaysStrictModule.ts`

Reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/alwaysStrictModule.ts
```

Source overview:

```json
{
  "suite": "tsc",
  "bytes": 139,
  "lines": 9,
  "extension": ".ts",
  "first_code_line": "namespace M {"
}
```

Failure location:

```json
{
  "code": "UnsupportedModule",
  "message": "issue-399: TypeScript namespace/internal module declarations require module ownership before runtime lowering at 64..73",
  "span_start": 64,
  "span_end": 73,
  "line": 5,
  "column": 1,
  "feature_label": "import-export",
  "error_type": "unsupported-feature-boundary"
}
```

Source context:

```text
2 | // @module: commonjs
3 | // @alwaysStrict: true
4 | 
5 | namespace M {
6 |     export function f() {
7 |         var arguments = [];
8 |     }
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
    "path": "issues/open/138-implement-alwaysStrictModule.md",
    "title": "Implement Alwaysstrictmodule",
    "reason": "same reference path, same feature label, title overlap"
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
- truncated: `False`

```text
== tokens ==
[
    SpannedToken {
        kind: Ident(
            "namespace",
        ),
        span: Span {
            start: 64,
            end: 73,
        },
    },
    SpannedToken {
        kind: Ident(
            "M",
        ),
        span: Span {
            start: 74,
            end: 75,
        },
    },
    SpannedToken {
        kind: LeftBrace,
        span: Span {
            start: 76,
            end: 77,
        },
    },
    SpannedToken {
        kind: Export,
        span: Span {
            start: 82,
            end: 88,
        },
    },
    SpannedToken {
        kind: Function,
        span: Span {
            start: 89,
            end: 97,
        },
    },
    SpannedToken {
        kind: Ident(
            "f",
        ),
        span: Span {
            start: 98,
            end: 99,
        },
    },
    SpannedToken {
        kind: LeftParen,
        span: Span {
            start: 99,
            end: 100,
        },
    },
    SpannedToken {
        kind: RightParen,
        span: Span {
            start: 100,
            end: 101,
        },
    },
    SpannedToken {
        kind: LeftBrace,
        span: Span {
            start: 102,
            end: 103,
        },
    },
    SpannedToken {
        kind: Var,
        span: Span {
            start: 112,
            end: 115,
        },
    },
    SpannedToken {
        kind: Ident(
            "arguments",
        ),
        span: Span {
            start: 116,
            end: 125,
        },
    },
    SpannedToken {
        kind: Equal,
        span: Span {
            start: 126,
            end: 127,
        },
    },
    SpannedToken {
        kind: LeftBracket,
        span: Span {
            start: 128,
            end: 129,
        },
    },
    SpannedToken {
        kind: RightBracket,
        span: Span {
            start: 129,
            end: 130,
        },
    },
    SpannedToken {
        kind: Semicolon,
        span: Span {
            start: 130,
            end: 131,
        },
    },
    SpannedToken {
        kind: RightBrace,
        span: Span {
            start: 136,
            end: 137,
        },
    },
    SpannedToken {
        kind: RightBrace,
        span: Span {
            start: 138,
            end: 139,
        },
    },
]
```

#### ast

- ok: `False`
- truncated: `False`

```text
error: [UnsupportedModule] issue-399: TypeScript namespace/internal module declarations require module ownership before runtime lowering at 64..73
```

#### resolved

- ok: `False`
- truncated: `False`

```text
error: [UnsupportedModule] issue-399: TypeScript namespace/internal module declarations require module ownership before runtime lowering at 64..73
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
        "code": 1100,
        "category": "Error",
        "message": "Invalid use of 'arguments' in strict mode.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/alwaysStrictModule.ts",
        "start": 116,
        "length": 9,
        "line": 7,
        "character": 13
      }
    ],
    "hints": [
      {
        "kind": "function",
        "typeText": "void",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/alwaysStrictModule.ts",
        "start": 98,
        "length": 1,
        "line": 6,
        "character": 21,
        "name": "f"
      },
      {
        "kind": "binding",
        "typeText": "never[]",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/alwaysStrictModule.ts",
        "start": 116,
        "length": 9,
        "line": 7,
        "character": 13,
        "name": "arguments"
      }
    ],
    "typescriptVersion": "6.0.3"
  },
  "ast": {
    "topLevel": [
      {
        "kind": "ModuleDeclaration",
        "text": "namespace M {\n    export function f() {\n        var arguments = [];\n    }\n}",
        "line": 5,
        "character": 1
      }
    ],
    "pathToPosition": [
      {
        "kind": "SourceFile",
        "text": "namespace M {\n    export function f() {\n        var arguments = [];\n    }\n}",
        "line": 5,
        "character": 1
      },
      {
        "kind": "ModuleDeclaration",
        "text": "namespace M {\n    export function f() {\n        var arguments = [];\n    }\n}",
        "line": 5,
        "character": 1
      }
    ]
  }
}
```

Stack trace:

```text
error: [UnsupportedModule] issue-399: TypeScript namespace/internal module declarations require module ownership before runtime lowering at 64..73
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
