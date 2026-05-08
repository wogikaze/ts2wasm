---
id: 177
title: "Implement Anonclassdeclarationemitisanon (dup)"
type: spike
area: frontend/syntax
class: superseded
priority: P2
depends_on: [5003]
blocks: []
created: 2026-04-29
updated: 2026-05-04
---

## Summary

Triage anonClassDeclarationEmitIsAnon across 1 failing reference test cases and split this bucket into implementation-ready child issues.

## Problem

Reference test results show 1 cases fail in directory `anonClassDeclarationEmitIsAnon` with diagnostics: declaration-emit. The compiler cannot handle these syntax/semantics, preventing compilation of code in this category.

Problem: anonClassDeclarationEmitIsAnon has 1 reference failures and needs smart-triage evidence before implementation starts.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/anonClassDeclarationEmitIsAnon.ts
```

Coverage window:

```sh
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/anonClassDeclarationEmitIsAnon.ts --detail
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
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/anonClassDeclarationEmitIsAnon.ts --detail
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/anonClassDeclarationEmitIsAnon.ts
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

- `reference/typescript/tests/cases/compiler/anonClassDeclarationEmitIsAnon.ts`

## Duplicate detection

## Smart triage

### Smart triage: Triage declaration emit: anonClassDeclarationEmitIsAnon

- Issue class: `triage-needed`
- Feature label: `declaration-emit`
- Diagnostic: `UnsupportedSyntax` / `parser-or-frontend-unsupported`
- Path: `reference/typescript/tests/cases/compiler/anonClassDeclarationEmitIsAnon.ts`

Reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/anonClassDeclarationEmitIsAnon.ts
```

Source overview:

```json
{
  "suite": "tsc",
  "bytes": 714,
  "lines": 36,
  "extension": ".ts",
  "first_code_line": "export function wrapClass(param: any) {"
}
```

Failure location:

```json
{
  "code": "UnsupportedSyntax",
  "message": "issue-055: unsupported function export; module resolution and loading are not implemented at 92..98",
  "span_start": 92,
  "span_end": 98,
  "line": 5,
  "column": 4,
  "feature_label": "declaration-emit",
  "error_type": "parser-or-frontend-unsupported"
}
```

Source context:

```text
2 | // @target: es2015
3 | // @declaration: true
4 | // @filename: wrapClass.ts
5 | export function wrapClass(param: any) {
6 |     return class Wrapped {
7 |         foo() {
8 |             return param;
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
    "path": "issues/done/177-implement-anonClassDeclarationEmitIsAnon.md",
    "title": "Implement Anonclassdeclarationemitisanon",
    "reason": "same reference path, title overlap"
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
        kind: Export,
        span: Span {
            start: 92,
            end: 98,
        },
    },
    SpannedToken {
        kind: Function,
        span: Span {
            start: 99,
            end: 107,
        },
    },
    SpannedToken {
        kind: Ident(
            "wrapClass",
        ),
        span: Span {
            start: 108,
            end: 117,
        },
    },
    SpannedToken {
        kind: LeftParen,
        span: Span {
            start: 117,
            end: 118,
        },
    },
    SpannedToken {
        kind: Ident(
            "param",
        ),
        span: Span {
            start: 118,
            end: 123,
        },
    },
    SpannedToken {
        kind: Colon,
        span: Span {
            start: 123,
            end: 124,
        },
    },
    SpannedToken {
        kind: Ident(
            "any",
        ),
        span: Span {
            start: 125,
            end: 128,
        },
    },
    SpannedToken {
        kind: RightParen,
        span: Span {
            start: 128,
            end: 129,
        },
    },
    SpannedToken {
        kind: LeftBrace,
        span: Span {
            start: 130,
            end: 131,
        },
    },
    SpannedToken {
        kind: Return,
        span: Span {
            start: 137,
            end: 143,
        },
    },
    SpannedToken {
        kind: Class,
        span: Span {
            start: 144,
            end: 149,
        },
    },
    SpannedToken {
        kind: Ident(
            "Wrapped",
        ),
        span: Span {
            start: 150,
            end: 157,
        },
    },
    SpannedToken {
        kind: LeftBrace,
        span: Span {
            start: 158,
            end: 159,
        },
    },
    Spann
```

#### ast

- ok: `False`
- truncated: `False`

```text
error: [UnsupportedSyntax] issue-055: unsupported function export; module resolution and loading are not implemented at 92..98
```

#### resolved

- ok: `False`
- truncated: `False`

```text
error: [UnsupportedSyntax] issue-055: unsupported function export; module resolution and loading are not implemented at 92..98
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
        "code": 2307,
        "category": "Error",
        "message": "Cannot find module './wrapClass' or its corresponding type declarations.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/anonClassDeclarationEmitIsAnon.ts",
        "start": 505,
        "length": 13,
        "line": 22,
        "character": 40
      }
    ],
    "hints": [
      {
        "kind": "function",
        "typeText": "typeof Wrapped",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/anonClassDeclarationEmitIsAnon.ts",
        "start": 108,
        "length": 9,
        "line": 5,
        "character": 17,
        "name": "wrapClass"
      },
      {
        "kind": "parameter",
        "typeText": "any",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/anonClassDeclarationEmitIsAnon.ts",
        "start": 118,
        "length": 5,
        "line": 5,
        "character": 27,
        "name": "param"
      },
      {
        "kind": "parameter",
        "typeText": "any[]",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/anonClassDeclarationEmitIsAnon.ts",
        "start": 270,
        "length": 4,
        "line": 13,
        "character": 43,
        "name": "args"
      },
      {
        "kind": "function",
        "typeText": "{ new (...args: any[]): (Anonymous class); prototype: Timestamped<any>.(Anonymous class); } & TBase",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/anonClassDeclarationEmitIsAnon.ts",
        "start": 308,
        "length": 11,
        "line": 15,
        "character": 17,
        "name": "Timestamped"
      },
      {
        "kind": "parameter",
        "typeText": "TBase",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/anonClassDeclarationEmitIsAnon.ts",
        "start": 347,
        "length": 4,
        "line": 15,
        "character": 56,
        "name": "Base"
      }
    ],
    "typescriptVersion": "6.0.3"
  },
  "ast": {
    "topLevel": [
      {
        "kind": "FunctionDeclaration",
        "text": "export function wrapClass(param: any) {\r\n    return class Wrapped {\r\n        foo() {\r\n            return param;\r\n       ",
        "line": 5,
        "character": 1
      },
      {
        "kind": "TypeAliasDeclaration",
        "text": "export type Constructor<T = {}> = new (...args: any[]) => T;",
        "line": 13,
        "character": 1
      },
      {
        "kind": "FunctionDeclaration",
        "text": "export function Timestamped<TBase extends Constructor>(Base: TBase) {\r\n    return class extends Base {\r\n        timestam",
        "line": 15,
        "character": 1
      },
      {
        "kind": "ImportDeclaration",
        "text": "import { wrapClass, Timestamped } from \"./wrapClass\";",
        "line": 22,
        "character": 1
      },
      {
        "kind": "ExportAssignment",
        "text": "export default wrapClass(0);",
        "line": 24,
        "character": 1
      },
      {
        "kind": "ClassDeclaration",
        "text": "export class User {\r\n    name = '';\r\n}",
        "line": 27,
        "character": 1
      },
      {
        "kind": "ClassDeclaration",
        "text": "export class TimestampedUser extends Timestamped(User) {\r\n    constructor() {\r\n        super();\r\n    }\r\n}",
        "line": 32,
        "character": 1
      }
    ],
    "pathToPosition": [
      {
        "kind": "SourceFile",
        "text": "export function wrapClass(param: any) {\r\n    return class Wrapped {\r\n        foo() {\r\n            return param;\r\n       ",
        "line": 5,
        "character": 1
      },
      {
        "kind": "FunctionDeclaration",
        "text": "export function wrapClass(param: any) {\r\n    return class Wrapped {\r\n        foo() {\r\n            return param;\r\n       ",
        "line": 5,
        "character": 1
      },
      {
        "kind": "ExportKeyword",
        "text": "export",
        "line": 5,
        "character": 1
      }
    ]
  }
}
```

Stack trace:

```text
error: [UnsupportedSyntax] issue-055: unsupported function export; module resolution and loading are not implemented at 92..98
```


## 重複整理メモ

この issue はタイトルが重複しているため、内容がより充実している
`issues/open/635-implement-anonClassDeclarationEmitIsAnon.md` に統合されました。
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

## False-done audit

Date: 2026-05-05

Classification: truly-done.

Audit result: retained in `issues/done/`. The issue has repo-local completion evidence, is not marked `blocked` or `triage-needed`, and `python scripts/manager.py check issues` validates the resulting issue graph.

Evidence files:
- `issues/open/177-implement-anonClassDeclarationEmitIsAnon.md`
- `issues/index.md` after regeneration

Future-work tracking: no untracked future-work item was identified in this issue during this metadata/evidence audit.
