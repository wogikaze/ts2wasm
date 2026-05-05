---
id: 534
title: "Implement Ambientmodules (dup)"
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

Triage ambientModules across 1 failing reference test cases and split this bucket into implementation-ready child issues.

## Problem

Reference test results show 1 cases fail in directory `ambientModules` with diagnostics: import-export. The compiler cannot handle these syntax/semantics, preventing compilation of code in this category.

Problem: ambientModules has 1 reference failures and needs smart-triage evidence before implementation starts.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/ambientModules.ts
```

Coverage window:

```sh
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/ambientModules.ts --detail
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
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/ambientModules.ts --detail
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/ambientModules.ts
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

- `reference/typescript/tests/cases/compiler/ambientModules.ts`

## Duplicate detection

- `issues/done/160-implement-ambientModules.md` - Implement Ambientmodules (same reference path, same group key, title overlap)

## Smart triage

### Smart triage: Triage import export: ambientModules

- Issue class: `triage-needed`
- Feature label: `import-export`
- Diagnostic: `UnsupportedModule` / `unsupported-feature-boundary`
- Path: `reference/typescript/tests/cases/compiler/ambientModules.ts`

Reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/ambientModules.ts
```

Source overview:

```json
{
  "suite": "tsc",
  "bytes": 104,
  "lines": 4,
  "extension": ".ts",
  "first_code_line": "﻿// @strict: false"
}
```

Failure location:

```json
{
  "code": "UnsupportedModule",
  "message": "issue-400: ambient namespace declarations require module ownership before runtime lowering at 50..59",
  "span_start": 50,
  "span_end": 59,
  "line": 3,
  "column": 13,
  "feature_label": "import-export",
  "error_type": "unsupported-feature-boundary"
}
```

Source context:

```text
1 | ﻿// @strict: false
2 | // @target: es2015
3 | declare namespace Foo.Bar { export var foo; };
4 | Foo.Bar.foo = 5;
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
    "path": "issues/done/160-implement-ambientModules.md",
    "title": "Implement Ambientmodules",
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
- truncated: `False`

```text
== tokens ==
[
    SpannedToken {
        kind: Ident(
            "declare",
        ),
        span: Span {
            start: 42,
            end: 49,
        },
    },
    SpannedToken {
        kind: Ident(
            "namespace",
        ),
        span: Span {
            start: 50,
            end: 59,
        },
    },
    SpannedToken {
        kind: Ident(
            "Foo",
        ),
        span: Span {
            start: 60,
            end: 63,
        },
    },
    SpannedToken {
        kind: Dot,
        span: Span {
            start: 63,
            end: 64,
        },
    },
    SpannedToken {
        kind: Ident(
            "Bar",
        ),
        span: Span {
            start: 64,
            end: 67,
        },
    },
    SpannedToken {
        kind: LeftBrace,
        span: Span {
            start: 68,
            end: 69,
        },
    },
    SpannedToken {
        kind: Export,
        span: Span {
            start: 70,
            end: 76,
        },
    },
    SpannedToken {
        kind: Var,
        span: Span {
            start: 77,
            end: 80,
        },
    },
    SpannedToken {
        kind: Ident(
            "foo",
        ),
        span: Span {
            start: 81,
            end: 84,
        },
    },
    SpannedToken {
        kind: Semicolon,
        span: Span {
            start: 84,
            end: 85,
        },
    },
    SpannedToken {
        kind: RightBrace,
        span: Span {
            start: 86,
            end: 87,
        },
    },
    SpannedToken {
        kind: Semicolon,
        span: Span {
            start: 87,
            end: 88,
        },
    },
    SpannedToken {
        kind: Ident(
            "Foo",
        ),
        span: Span {
            start: 90,
            end: 93,
        },
    },
    SpannedToken {
        kind: Dot,
        span: Span {
            start: 93,
            end: 94,
        },
    },
    SpannedToken {
        kind: Ident(
            "Bar",
        ),
        span: Span {
            start: 94,
            end: 97,
        },
    },
    SpannedToken {
        kind: Dot,
        span: Span {
            start: 97,
            end: 98,
        },
    },
    SpannedToken {
        kind: Ident(
            "foo",
        ),
        span: Span {
            start: 98,
            end: 101,
        },
    },
    SpannedToken {
        kind: Equal,
        span: Span {
            start: 102,
            end: 103,
        },
    },
    SpannedToken {
        kind: Number(
            5,
        ),
        span: Span {
            start: 104,
            end: 105,
        },
    },
    SpannedToken {
        kind: Semicolon,
        span: Span {
            start: 105,
            end: 106,
        },
    },
]
```

#### ast

- ok: `False`
- truncated: `False`

```text
error: [UnsupportedModule] issue-400: ambient namespace declarations require module ownership before runtime lowering at 50..59
```

#### resolved

- ok: `False`
- truncated: `False`

```text
error: [UnsupportedModule] issue-400: ambient namespace declarations require module ownership before runtime lowering at 50..59
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
        "kind": "binding",
        "typeText": "any",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/ambientModules.ts",
        "start": 78,
        "length": 3,
        "line": 3,
        "character": 40,
        "name": "foo"
      }
    ],
    "typescriptVersion": "6.0.3"
  },
  "ast": {
    "topLevel": [
      {
        "kind": "ModuleDeclaration",
        "text": "declare namespace Foo.Bar { export var foo; }",
        "line": 3,
        "character": 1
      },
      {
        "kind": "EmptyStatement",
        "text": ";",
        "line": 3,
        "character": 46
      },
      {
        "kind": "ExpressionStatement",
        "text": "Foo.Bar.foo = 5;",
        "line": 4,
        "character": 1
      }
    ],
    "pathToPosition": [
      {
        "kind": "SourceFile",
        "text": "declare namespace Foo.Bar { export var foo; };\r\nFoo.Bar.foo = 5; ",
        "line": 3,
        "character": 1
      },
      {
        "kind": "ModuleDeclaration",
        "text": "declare namespace Foo.Bar { export var foo; }",
        "line": 3,
        "character": 1
      }
    ]
  }
}
```

Stack trace:

```text
error: [UnsupportedModule] issue-400: ambient namespace declarations require module ownership before runtime lowering at 50..59
```


## 重複整理メモ

この issue はタイトルが重複しているため、内容がより充実している
`issues/open/620-implement-ambientModules.md` に統合されました。
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
- `issues/done/534-implement-ambientModules.md`
- `issues/index.md` after regeneration

Future-work tracking: no untracked future-work item was identified in this issue during this metadata/evidence audit.
