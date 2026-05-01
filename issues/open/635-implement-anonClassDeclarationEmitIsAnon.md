---
id: 635
title: "Implement Anonclassdeclarationemitisanon"
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

Triage anonClassDeclarationEmitIsAnon across 1 failing reference test cases and split this bucket into implementation-ready child issues.

## Problem

Reference test results show 1 cases fail in directory `anonClassDeclarationEmitIsAnon` with diagnostics: import-export. The compiler cannot handle these syntax/semantics, preventing compilation of code in this category.

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
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/anonClassDeclarationEmitIsAnon.ts --detail
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/anonClassDeclarationEmitIsAnon.ts
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

- `reference/typescript/tests/cases/compiler/anonClassDeclarationEmitIsAnon.ts`

## Duplicate detection

- `issues/open/177-implement-anonClassDeclarationEmitIsAnon.md` - Implement Anonclassdeclarationemitisanon (same reference path, same group key, title overlap)

## Smart triage

### Smart triage: Triage import export: anonClassDeclarationEmitIsAnon

- Issue class: `triage-needed`
- Feature label: `import-export`
- Diagnostic: `UnsupportedModule` / `unsupported-feature-boundary`
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
  "code": "UnsupportedModule",
  "message": "issue-055: unsupported function export; module resolution and loading are not implemented at 92..98",
  "span_start": 92,
  "span_end": 98,
  "line": 5,
  "column": 4,
  "feature_label": "import-export",
  "error_type": "unsupported-feature-boundary"
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
    "path": "issues/open/177-implement-anonClassDeclarationEmitIsAnon.md",
    "title": "Implement Anonclassdeclarationemitisanon",
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
    SpannedToken {
        kind: Ident(
            "foo",
        ),
        span: Span {
            start: 169,
            end: 172,
        },
    },
    SpannedToken {
        kind: LeftParen,
        span: Span {
            start: 172,
            end: 173,
        },
    },
    SpannedToken {
        kind: RightParen,
        span: Span {
            start: 173,
            end: 174,
        },
    },
    SpannedToken {
        kind: LeftBrace,
        span: Span {
            start: 175,
            end: 176,
        },
    },
    SpannedToken {
        kind: Return,
        span: Span {
            start: 190,
            end: 196,
        },
    },
    SpannedToken {
        kind: Ident(
            "param",
        ),
        span: Span {
            start: 197,
            end: 202,
        },
    },
    SpannedToken {
        kind: Semicolon,
        span: Span {
            start: 202,
            end: 203,
        },
    },
    SpannedToken {
        kind: RightBrace,
        span: Span {
            start: 213,
            end: 214,
        },
    },
    SpannedToken {
        kind: RightBrace,
        span: Span {
            start: 220,
            end: 221,
        },
    },
    SpannedToken {
        kind: RightBrace,
        span: Span {
            start: 223,
            end: 224,
        },
    },
    SpannedToken {
        kind: Export,
        span: Span {
            start: 228,
            end: 234,
        },
    },
    SpannedToken {
        kind: Ident(
            "type",
        ),
        span: Span {
            start: 235,
            end: 239,
        },
    },
    SpannedToken {
        kind: Ident(
            "Constructor",
        ),
```

#### ast

- ok: `False`
- truncated: `False`

```text
error: [UnsupportedModule] issue-055: unsupported function export; module resolution and loading are not implemented at 92..98
```

#### resolved

- ok: `False`
- truncated: `False`

```text
error: [UnsupportedModule] issue-055: unsupported function export; module resolution and loading are not implemented at 92..98
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
error: [UnsupportedModule] issue-055: unsupported function export; module resolution and loading are not implemented at 92..98
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
