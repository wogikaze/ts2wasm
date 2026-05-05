---
id: 531
title: "Implement Ambientmoduleexports (dup)"
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

Triage ambientModuleExports across 1 failing reference test cases and split this bucket into implementation-ready child issues.

## Problem

Reference test results show 1 cases fail in directory `ambientModuleExports` with diagnostics: import-export. The compiler cannot handle these syntax/semantics, preventing compilation of code in this category.

Problem: ambientModuleExports has 1 reference failures and needs smart-triage evidence before implementation starts.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/ambientModuleExports.ts
```

Coverage window:

```sh
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/ambientModuleExports.ts --detail
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
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/ambientModuleExports.ts --detail
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/ambientModuleExports.ts
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

- `reference/typescript/tests/cases/compiler/ambientModuleExports.ts`

## Duplicate detection

- `issues/done/157-implement-ambientModuleExports.md` - Implement Ambientmoduleexports (same reference path, same group key, title overlap)

## Smart triage

### Smart triage: Triage import export: ambientModuleExports

- Issue class: `triage-needed`
- Feature label: `import-export`
- Diagnostic: `UnsupportedModule` / `unsupported-feature-boundary`
- Path: `reference/typescript/tests/cases/compiler/ambientModuleExports.ts`

Reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/ambientModuleExports.ts
```

Source overview:

```json
{
  "suite": "tsc",
  "bytes": 279,
  "lines": 20,
  "extension": ".ts",
  "first_code_line": "declare namespace Foo {"
}
```

Failure location:

```json
{
  "code": "UnsupportedModule",
  "message": "issue-400: ambient namespace declarations require module ownership before runtime lowering at 28..37",
  "span_start": 28,
  "span_end": 37,
  "line": 2,
  "column": 10,
  "feature_label": "import-export",
  "error_type": "unsupported-feature-boundary"
}
```

Source context:

```text
1 | // @target: es2015
2 | declare namespace Foo {
3 |  function a():void;
4 |  var b:number;
5 |  class C {}
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
    "path": "issues/done/157-implement-ambientModuleExports.md",
    "title": "Implement Ambientmoduleexports",
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
            start: 20,
            end: 27,
        },
    },
    SpannedToken {
        kind: Ident(
            "namespace",
        ),
        span: Span {
            start: 28,
            end: 37,
        },
    },
    SpannedToken {
        kind: Ident(
            "Foo",
        ),
        span: Span {
            start: 38,
            end: 41,
        },
    },
    SpannedToken {
        kind: LeftBrace,
        span: Span {
            start: 42,
            end: 43,
        },
    },
    SpannedToken {
        kind: Function,
        span: Span {
            start: 46,
            end: 54,
        },
    },
    SpannedToken {
        kind: Ident(
            "a",
        ),
        span: Span {
            start: 55,
            end: 56,
        },
    },
    SpannedToken {
        kind: LeftParen,
        span: Span {
            start: 56,
            end: 57,
        },
    },
    SpannedToken {
        kind: RightParen,
        span: Span {
            start: 57,
            end: 58,
        },
    },
    SpannedToken {
        kind: Colon,
        span: Span {
            start: 58,
            end: 59,
        },
    },
    SpannedToken {
        kind: Void,
        span: Span {
            start: 59,
            end: 63,
        },
    },
    SpannedToken {
        kind: Semicolon,
        span: Span {
            start: 63,
            end: 64,
        },
    },
    SpannedToken {
        kind: Var,
        span: Span {
            start: 67,
            end: 70,
        },
    },
    SpannedToken {
        kind: Ident(
            "b",
        ),
        span: Span {
            start: 71,
            end: 72,
        },
    },
    SpannedToken {
        kind: Colon,
        span: Span {
            start: 72,
            end: 73,
        },
    },
    SpannedToken {
        kind: Ident(
            "number",
        ),
        span: Span {
            start: 73,
            end: 79,
        },
    },
    SpannedToken {
        kind: Semicolon,
        span: Span {
            start: 79,
            end: 80,
        },
    },
    SpannedToken {
        kind: Class,
        span: Span {
            start: 83,
            end: 88,
        },
    },
    SpannedToken {
        kind: Ident(
            "C",
        ),
        span: Span {
            start: 89,
            end: 90,
        },
    },
    SpannedToken {
        kind: LeftBrace,
        span: Span {
            start: 91,
            end: 92,
        },
    },
    SpannedToken {
        kind: RightBrace,
        span: Span {
            start: 92,
            end: 93,
        },
    },
    SpannedToken {
        kind: RightBrace,
        span: Span {
            start: 95,
            end: 96,
        },
    },
    SpannedToken {
        kind: Ident(
            "Foo",
        ),
        span: Span {
            start: 100,
            end: 103,
        },
    },
    SpannedToken {
        kind: Dot,
        span: Span {
            start: 103,
            end: 104,
        },
    },
    SpannedToken {
        kind: Ident(
            "a",
        ),
        span: Span {
            start: 104,
            end: 105,
        },
    },
    SpannedToken {
        kind: LeftParen,
        span: Span {
            start: 105,
            end: 106,
        },
    },
    SpannedToken {
        kind: RightParen,
        span: Span {
            start: 106,
```

#### ast

- ok: `False`
- truncated: `False`

```text
error: [UnsupportedModule] issue-400: ambient namespace declarations require module ownership before runtime lowering at 28..37
```

#### resolved

- ok: `False`
- truncated: `False`

```text
error: [UnsupportedModule] issue-400: ambient namespace declarations require module ownership before runtime lowering at 28..37
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
        "kind": "function",
        "typeText": "void",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/ambientModuleExports.ts",
        "start": 55,
        "length": 1,
        "line": 3,
        "character": 11,
        "name": "a"
      },
      {
        "kind": "binding",
        "typeText": "number",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/ambientModuleExports.ts",
        "start": 71,
        "length": 1,
        "line": 4,
        "character": 6,
        "name": "b"
      },
      {
        "kind": "binding",
        "typeText": "C",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/ambientModuleExports.ts",
        "start": 122,
        "length": 1,
        "line": 10,
        "character": 5,
        "name": "c"
      },
      {
        "kind": "function",
        "typeText": "void",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/ambientModuleExports.ts",
        "start": 188,
        "length": 1,
        "line": 13,
        "character": 21,
        "name": "a"
      },
      {
        "kind": "binding",
        "typeText": "number",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/ambientModuleExports.ts",
        "start": 215,
        "length": 1,
        "line": 14,
        "character": 16,
        "name": "b"
      },
      {
        "kind": "binding",
        "typeText": "C",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/ambientModuleExports.ts",
        "start": 280,
        "length": 2,
        "line": 20,
        "character": 5,
        "name": "c2"
      }
    ],
    "typescriptVersion": "6.0.3"
  },
  "ast": {
    "topLevel": [
      {
        "kind": "ModuleDeclaration",
        "text": "declare namespace Foo {\r\n\tfunction a():void;\r\n\tvar b:number;\r\n\tclass C {}\r\n}",
        "line": 2,
        "character": 1
      },
      {
        "kind": "ExpressionStatement",
        "text": "Foo.a();",
        "line": 8,
        "character": 1
      },
      {
        "kind": "ExpressionStatement",
        "text": "Foo.b;",
        "line": 9,
        "character": 1
      },
      {
        "kind": "FirstStatement",
        "text": "var c = new Foo.C();",
        "line": 10,
        "character": 1
      },
      {
        "kind": "ModuleDeclaration",
        "text": "declare namespace Foo2 {\r\n    export function a(): void;\r\n    export var b: number;\r\n    export class C { }\r\n}",
        "line": 12,
        "character": 1
      },
      {
        "kind": "ExpressionStatement",
        "text": "Foo2.a();",
        "line": 18,
        "character": 1
      },
      {
        "kind": "ExpressionStatement",
        "text": "Foo2.b;",
        "line": 19,
        "character": 1
      },
      {
        "kind": "FirstStatement",
        "text": "var c2 = new Foo2.C();",
        "line": 20,
        "character": 1
      }
    ],
    "pathToPosition": [
      {
        "kind": "SourceFile",
        "text": "declare namespace Foo {\r\n\tfunction a():void;\r\n\tvar b:number;\r\n\tclass C {}\r\n}\r\n\r\nFoo.a();\r\nFoo.b;\r\nvar c = new Foo.C();\r\n",
        "line": 2,
        "character": 1
      },
      {
        "kind": "ModuleDeclaration",
        "text": "declare namespace Foo {\r\n\tfunction a():void;\r\n\tvar b:number;\r\n\tclass C {}\r\n}",
        "line": 2,
        "character": 1
      }
    ]
  }
}
```

Stack trace:

```text
error: [UnsupportedModule] issue-400: ambient namespace declarations require module ownership before runtime lowering at 28..37
```


## 重複整理メモ

この issue はタイトルが重複しているため、内容がより充実している
`issues/open/617-implement-ambientModuleExports.md` に統合されました。
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
- `issues/done/531-implement-ambientModuleExports.md`
- `issues/index.md` after regeneration

Future-work tracking: no untracked future-work item was identified in this issue during this metadata/evidence audit.
