---
id: 497
title: "Implement Aliasonmergedmoduleinterface (dup)"
type: spike
area: frontend/syntax
class: superseded
priority: P1
depends_on: [5001]
blocks: []
created: 2026-05-01
updated: 2026-05-04
---

## Summary

Triage aliasOnMergedModuleInterface across 1 failing reference test cases and split this bucket into implementation-ready child issues.

## Problem

Reference test results show 1 cases fail in directory `aliasOnMergedModuleInterface` with diagnostics: import-export. The compiler cannot handle these syntax/semantics, preventing compilation of code in this category.

Problem: aliasOnMergedModuleInterface has 1 reference failures and needs smart-triage evidence before implementation starts.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/aliasOnMergedModuleInterface.ts
```

Coverage window:

```sh
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/aliasOnMergedModuleInterface.ts --detail
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
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/aliasOnMergedModuleInterface.ts --detail
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/aliasOnMergedModuleInterface.ts
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

- `reference/typescript/tests/cases/compiler/aliasOnMergedModuleInterface.ts`

## Duplicate detection

- `issues/open/118-implement-aliasOnMergedModuleInterface.md` - Implement Aliasonmergedmoduleinterface (same reference path, same group key, title overlap)

## Smart triage

### Smart triage: Triage import export: aliasOnMergedModuleInterface

- Issue class: `triage-needed`
- Feature label: `import-export`
- Diagnostic: `UnsupportedModule` / `unsupported-feature-boundary`
- Path: `reference/typescript/tests/cases/compiler/aliasOnMergedModuleInterface.ts`

Reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/aliasOnMergedModuleInterface.ts
```

Source overview:

```json
{
  "suite": "tsc",
  "bytes": 522,
  "lines": 21,
  "extension": ".ts",
  "first_code_line": "declare module \"foo\""
}
```

Failure location:

```json
{
  "code": "UnsupportedModule",
  "message": "issue-400: ambient module declarations require module ownership before runtime lowering at 98..104",
  "span_start": 98,
  "span_end": 104,
  "line": 4,
  "column": 12,
  "feature_label": "import-export",
  "error_type": "unsupported-feature-boundary"
}
```

Source context:

```text
1 | // @target: es2015
2 | //@module: commonjs
3 | // @Filename: aliasOnMergedModuleInterface_0.ts
4 | declare module "foo"
5 | {
6 |     namespace B {
7 |         export interface A {
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
    "path": "issues/open/118-implement-aliasOnMergedModuleInterface.md",
    "title": "Implement Aliasonmergedmoduleinterface",
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
            start: 90,
            end: 97,
        },
    },
    SpannedToken {
        kind: Ident(
            "module",
        ),
        span: Span {
            start: 98,
            end: 104,
        },
    },
    SpannedToken {
        kind: String(
            "foo",
        ),
        span: Span {
            start: 105,
            end: 110,
        },
    },
    SpannedToken {
        kind: LeftBrace,
        span: Span {
            start: 112,
            end: 113,
        },
    },
    SpannedToken {
        kind: Ident(
            "namespace",
        ),
        span: Span {
            start: 119,
            end: 128,
        },
    },
    SpannedToken {
        kind: Ident(
            "B",
        ),
        span: Span {
            start: 129,
            end: 130,
        },
    },
    SpannedToken {
        kind: LeftBrace,
        span: Span {
            start: 131,
            end: 132,
        },
    },
    SpannedToken {
        kind: Export,
        span: Span {
            start: 142,
            end: 148,
        },
    },
    SpannedToken {
        kind: Ident(
            "interface",
        ),
        span: Span {
            start: 149,
            end: 158,
        },
    },
    SpannedToken {
        kind: Ident(
            "A",
        ),
        span: Span {
            start: 159,
            end: 160,
        },
    },
    SpannedToken {
        kind: LeftBrace,
        span: Span {
            start: 161,
            end: 162,
        },
    },
    SpannedToken {
        kind: RightBrace,
        span: Span {
            start: 172,
            end: 173,
        },
    },
    SpannedToken {
        kind: RightBrace,
        span: Span {
            start: 179,
            end: 180,
        },
    },
    SpannedToken {
        kind: Ident(
            "interface",
        ),
        span: Span {
            start: 186,
            end: 195,
        },
    },
    SpannedToken {
        kind: Ident(
            "B",
        ),
        span: Span {
            start: 196,
            end: 197,
        },
    },
    SpannedToken {
        kind: LeftBrace,
        span: Span {
            start: 198,
            end: 199,
        },
    },
    SpannedToken {
        kind: Ident(
            "bar",
        ),
        span: Span {
            start: 209,
            end: 212,
        },
    },
    SpannedToken {
        kind: LeftParen,
        span: Span {
            start: 212,
            end: 213,
        },
    },
    SpannedToken {
        kind: Ident(
            "name",
        ),
        span: Span {
            start: 213,
            end: 217,
        },
    },
    SpannedToken {
        kind: Colon,
        span: Span {
            start: 217,
            end: 218,
        },
    },
    SpannedToken {
        kind: Ident(
            "string",
        ),
        span: Span {
            start: 219,
            end: 225,
        },
    },
    SpannedToken {
        kind: RightParen,
        span: Span {
            start: 225,
            end: 226,
        },
    },
    SpannedToken {
        kind: Colon,
        span: Span {
            start: 226,
            end: 227,
        },
    },
    SpannedToken {
        kind: Ident(
            "B",
        ),
        span: Span {
            start: 228,
            end: 229,
        },
    },
    SpannedToken {
        kind: Dot,
```

#### ast

- ok: `False`
- truncated: `False`

```text
error: [UnsupportedModule] issue-400: ambient module declarations require module ownership before runtime lowering at 98..104
```

#### resolved

- ok: `False`
- truncated: `False`

```text
error: [UnsupportedModule] issue-400: ambient module declarations require module ownership before runtime lowering at 98..104
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
        "code": 2664,
        "category": "Error",
        "message": "Invalid module name in augmentation, module 'foo' cannot be found.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/aliasOnMergedModuleInterface.ts",
        "start": 105,
        "length": 5,
        "line": 4,
        "character": 16
      },
      {
        "code": 2307,
        "category": "Error",
        "message": "Cannot find module 'foo' or its corresponding type declarations.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/aliasOnMergedModuleInterface.ts",
        "start": 392,
        "length": 5,
        "line": 18,
        "character": 22
      }
    ],
    "hints": [
      {
        "kind": "parameter",
        "typeText": "string",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/aliasOnMergedModuleInterface.ts",
        "start": 213,
        "length": 4,
        "line": 11,
        "character": 13,
        "name": "name"
      },
      {
        "kind": "binding",
        "typeText": "foo",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/aliasOnMergedModuleInterface.ts",
        "start": 412,
        "length": 1,
        "line": 19,
        "character": 13,
        "name": "z"
      },
      {
        "kind": "binding",
        "typeText": "foo.A",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/aliasOnMergedModuleInterface.ts",
        "start": 463,
        "length": 1,
        "line": 21,
        "character": 5,
        "name": "x"
      }
    ],
    "typescriptVersion": "6.0.3"
  },
  "ast": {
    "topLevel": [
      {
        "kind": "ModuleDeclaration",
        "text": "declare module \"foo\"\r\n{\r\n    namespace B {\r\n        export interface A {\r\n        }\r\n    }\r\n    interface B {\r\n        b",
        "line": 4,
        "character": 1
      },
      {
        "kind": "ImportEqualsDeclaration",
        "text": "import foo = require(\"foo\")",
        "line": 18,
        "character": 1
      },
      {
        "kind": "FirstStatement",
        "text": "declare var z: foo;",
        "line": 19,
        "character": 1
      },
      {
        "kind": "ExpressionStatement",
        "text": "z.bar(\"hello\");",
        "line": 20,
        "character": 1
      },
      {
        "kind": "FirstStatement",
        "text": "var x: foo.A = foo.bar(\"hello\");",
        "line": 21,
        "character": 1
      }
    ],
    "pathToPosition": [
      {
        "kind": "SourceFile",
        "text": "declare module \"foo\"\r\n{\r\n    namespace B {\r\n        export interface A {\r\n        }\r\n    }\r\n    interface B {\r\n        b",
        "line": 4,
        "character": 1
      },
      {
        "kind": "ModuleDeclaration",
        "text": "declare module \"foo\"\r\n{\r\n    namespace B {\r\n        export interface A {\r\n        }\r\n    }\r\n    interface B {\r\n        b",
        "line": 4,
        "character": 1
      }
    ]
  }
}
```

Stack trace:

```text
error: [UnsupportedModule] issue-400: ambient module declarations require module ownership before runtime lowering at 98..104
```


## 重複整理メモ

この issue はタイトルが重複しているため、内容がより充実している
`issues/open/583-implement-aliasOnMergedModuleInterface.md` に統合されました。
そちらを参照してください。
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
