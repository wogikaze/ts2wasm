---
id: 631
title: "Implement Amdlikeinputdeclarationemit"
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

Triage amdLikeInputDeclarationEmit across 1 failing reference test cases and split this bucket into implementation-ready child issues.

## Problem

Reference test results show 1 cases fail in directory `amdLikeInputDeclarationEmit` with diagnostics: import-export. The compiler cannot handle these syntax/semantics, preventing compilation of code in this category.

Problem: amdLikeInputDeclarationEmit has 1 reference failures and needs smart-triage evidence before implementation starts.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/amdLikeInputDeclarationEmit.ts
```

Coverage window:

```sh
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/amdLikeInputDeclarationEmit.ts --detail
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
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/amdLikeInputDeclarationEmit.ts --detail
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/amdLikeInputDeclarationEmit.ts
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

- `reference/typescript/tests/cases/compiler/amdLikeInputDeclarationEmit.ts`

## Duplicate detection

- `issues/open/173-implement-amdLikeInputDeclarationEmit.md` - Implement Amdlikeinputdeclarationemit (same reference path, same group key, title overlap)

## Smart triage

### Smart triage: Triage import export: amdLikeInputDeclarationEmit

- Issue class: `triage-needed`
- Feature label: `import-export`
- Diagnostic: `UnsupportedModule` / `unsupported-feature-boundary`
- Path: `reference/typescript/tests/cases/compiler/amdLikeInputDeclarationEmit.ts`

Reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/amdLikeInputDeclarationEmit.ts
```

Source overview:

```json
{
  "suite": "tsc",
  "bytes": 961,
  "lines": 36,
  "extension": ".ts",
  "first_code_line": "declare function define<T=unknown>(name: string, modules: string[], ready: (...modules: unknown[]) => T);"
}
```

Failure location:

```json
{
  "code": "UnsupportedModule",
  "message": "issue-400: ambient module declarations require module ownership before runtime lowering at 341..347",
  "span_start": 341,
  "span_end": 347,
  "line": 11,
  "column": 19,
  "feature_label": "import-export",
  "error_type": "unsupported-feature-boundary"
}
```

Source context:

```text
 8 | // @filename: typing.d.ts
 9 | declare function define<T=unknown>(name: string, modules: string[], ready: (...modules: unknown[]) => T);
10 | // @filename: deps/BaseClass.d.ts
11 | declare module "deps/BaseClass" {
12 |     class BaseClass {
13 |         static extends<A>(a: A): new () => A & BaseClass;
14 |     }
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
    "path": "issues/done/173-implement-amdLikeInputDeclarationEmit.md",
    "title": "Implement Amdlikeinputdeclarationemit",
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
            "declare",
        ),
        span: Span {
            start: 191,
            end: 198,
        },
    },
    SpannedToken {
        kind: Function,
        span: Span {
            start: 199,
            end: 207,
        },
    },
    SpannedToken {
        kind: Ident(
            "define",
        ),
        span: Span {
            start: 208,
            end: 214,
        },
    },
    SpannedToken {
        kind: Less,
        span: Span {
            start: 214,
            end: 215,
        },
    },
    SpannedToken {
        kind: Ident(
            "T",
        ),
        span: Span {
            start: 215,
            end: 216,
        },
    },
    SpannedToken {
        kind: Equal,
        span: Span {
            start: 216,
            end: 217,
        },
    },
    SpannedToken {
        kind: Ident(
            "unknown",
        ),
        span: Span {
            start: 217,
            end: 224,
        },
    },
    SpannedToken {
        kind: Greater,
        span: Span {
            start: 224,
            end: 225,
        },
    },
    SpannedToken {
        kind: LeftParen,
        span: Span {
            start: 225,
            end: 226,
        },
    },
    SpannedToken {
        kind: Ident(
            "name",
        ),
        span: Span {
            start: 226,
            end: 230,
        },
    },
    SpannedToken {
        kind: Colon,
        span: Span {
            start: 230,
            end: 231,
        },
    },
    SpannedToken {
        kind: Ident(
            "string",
        ),
        span: Span {
            start: 232,
            end: 238,
        },
    },
    SpannedToken {
        kind: Comma,
        span: Span {
            start: 238,
            end: 239,
        },
    },
    SpannedToken {
        kind: Ident(
            "modules",
        ),
        span: Span {
            start: 240,
            end: 247,
        },
    },
    SpannedToken {
        kind: Colon,
        span: Span {
            start: 247,
            end: 248,
        },
    },
    SpannedToken {
        kind: Ident(
            "string",
        ),
        span: Span {
            start: 249,
            end: 255,
        },
    },
    SpannedToken {
        kind: LeftBracket,
        span: Span {
            start: 255,
            end: 256,
        },
    },
    SpannedToken {
        kind: RightBracket,
        span: Span {
            start: 256,
            end: 257,
        },
    },
    SpannedToken {
        kind: Comma,
        span: Span {
            start: 257,
            end: 258,
        },
    },
    SpannedToken {
        kind: Ident(
            "ready",
        ),
        span: Span {
            start: 259,
            end: 264,
        },
    },
    SpannedToken {
        kind: Colon,
        span: Span {
            start: 264,
            end: 265,
        },
    },
    SpannedToken {
        kind: LeftParen,
        span: Span {
            start: 266,
            end: 267,
        },
    },
    SpannedToken {
        kind: DotDotDot,
        span: Span {
            start: 267,
            end: 270,
        },
    },
    SpannedToken {
        kind: Ident(
            "modules",
        ),
        span: Span {
            start: 270,
            end: 277,
        },
    },
    SpannedToken {
        kind: Colon,
        span: Span {
            start: 277,
            end: 278,
        },
    },
    S
```

#### ast

- ok: `False`
- truncated: `False`

```text
error: [UnsupportedModule] issue-400: ambient module declarations require module ownership before runtime lowering at 341..347
```

#### resolved

- ok: `False`
- truncated: `False`

```text
error: [UnsupportedModule] issue-400: ambient module declarations require module ownership before runtime lowering at 341..347
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
        "code": 18046,
        "category": "Error",
        "message": "'BaseClass' is of type 'unknown'.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/amdLikeInputDeclarationEmit.ts",
        "start": 740,
        "length": 9,
        "line": 26,
        "character": 27
      },
      {
        "code": 2339,
        "category": "Error",
        "message": "Property 'exports' does not exist on type '{}'.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/amdLikeInputDeclarationEmit.ts",
        "start": 940,
        "length": 7,
        "line": 34,
        "character": 12
      },
      {
        "code": 2339,
        "category": "Error",
        "message": "Property 'exports' does not exist on type '{}'.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/amdLikeInputDeclarationEmit.ts",
        "start": 983,
        "length": 7,
        "line": 35,
        "character": 19
      }
    ],
    "hints": [
      {
        "kind": "function",
        "typeText": "any",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/amdLikeInputDeclarationEmit.ts",
        "start": 208,
        "length": 6,
        "line": 9,
        "character": 18,
        "name": "define"
      },
      {
        "kind": "parameter",
        "typeText": "string",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/amdLikeInputDeclarationEmit.ts",
        "start": 226,
        "length": 4,
        "line": 9,
        "character": 36,
        "name": "name"
      },
      {
        "kind": "parameter",
        "typeText": "string[]",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/amdLikeInputDeclarationEmit.ts",
        "start": 240,
        "length": 7,
        "line": 9,
        "character": 50,
        "name": "modules"
      },
      {
        "kind": "parameter",
        "typeText": "(...modules: unknown[]) => T",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/amdLikeInputDeclarationEmit.ts",
        "start": 259,
        "length": 5,
        "line": 9,
        "character": 69,
        "name": "ready"
      },
      {
        "kind": "parameter",
        "typeText": "unknown[]",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/amdLikeInputDeclarationEmit.ts",
        "start": 270,
        "length": 7,
        "line": 9,
        "character": 80,
        "name": "modules"
      },
      {
        "kind": "parameter",
        "typeText": "A",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/amdLikeInputDeclarationEmit.ts",
        "start": 417,
        "length": 1,
        "line": 13,
        "character": 27,
        "name": "a"
      },
      {
        "kind": "parameter",
        "typeText": "unknown",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/amdLikeInputDeclarationEmit.ts",
        "start": 687,
        "length": 9,
        "line": 24,
        "character": 2,
        "name": "BaseClass"
      },
      {
        "kind": "binding",
        "typeText": "any",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/amdLikeInputDeclarationEmit.ts",
        "start": 724,
        "length": 13,
        "line": 26,
        "character": 11,
        "name": "ExtendedClass"
      },
      {
        "kind": "binding",
        "typeText": "{}",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/amdLikeInputDeclarationEmit.ts",
        "start": 915,
        "length": 6,
        "line": 33,
        "character": 11,
        "name": "module"
      }
    ],
    "typescriptVersion": "6.0.3"
  },
  "ast": {
    "topLevel": [
      {
        "kind": "FunctionDeclaration",
        "text": "declare function define<T=unknown>(name: string, modules: string[], ready: (...modules: unknown[]) => T);",
        "line": 9,
        "character": 1
      },
      {
        "kind": "ModuleDeclaration",
        "text": "declare module \"deps/BaseClass\" {\r\n    class BaseClass {\r\n        static extends<A>(a: A): new () => A & BaseClass;\r\n   ",
        "line": 11,
        "character": 1
      },
      {
        "kind": "ExpressionStatement",
        "text": "define(\"lib/ExtendedClass\", [\"deps/BaseClass\"], \r\n/**\r\n * {typeof import(\"deps/BaseClass\")}\r\n * @param  {typeof import(\"",
        "line": 18,
        "character": 1
      }
    ],
    "pathToPosition": [
      {
        "kind": "SourceFile",
        "text": "declare function define<T=unknown>(name: string, modules: string[], ready: (...modules: unknown[]) => T);\r\n// @filename:",
        "line": 9,
        "character": 1
      },
      {
        "kind": "ModuleDeclaration",
        "text": "declare module \"deps/BaseClass\" {\r\n    class BaseClass {\r\n        static extends<A>(a: A): new () => A & BaseClass;\r\n   ",
        "line": 11,
        "character": 1
      }
    ]
  }
}
```

Stack trace:

```text
error: [UnsupportedModule] issue-400: ambient module declarations require module ownership before runtime lowering at 341..347
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
