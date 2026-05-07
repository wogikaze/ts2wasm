---
id: 620
title: "Implement Ambientmodules"
type: spike
area: frontend/syntax
class: done
priority: P1
depends_on: []
blocks: []
created: 2026-05-01
updated: 2026-05-08
completed: 2026-05-08
status: done
---

## Summary

Triage ambientModules across 1 failing reference test cases and split this bucket into implementation-ready child issues.

## Problem

Fresh reference evidence shows this bucket now reaches a dotted ambient
namespace qualified value-access blocker.

Problem: this generated import/export bucket is not a standalone
implementation order; its current blocker is `UnresolvedName` for the erased
dotted ambient namespace root `Foo`.

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

This generated bucket was split to
`issues/open/5404-bind-dotted-ambient-namespace-qualified-access.md`. Do not
implement directly from this bucket.

## Scope

In scope:

- [x] Inspect the smart triage report below
- [x] Confirm whether existing open/done issues already cover this bucket
- [x] Split dotted ambient namespace qualified value access to issue 5404
- [x] Preserve exact reproduction commands and representative AST/diagnostic evidence in the child issue

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

- [x] Duplicate candidates below are confirmed as no-match or this issue is split
- [x] Child issue contains an exact `python scripts/manager.py reference-triage ...` command
- [x] Child issue includes failing path, diagnostic code, source context, visible symbols, and parser/TypeScript AST evidence
- [x] Child issue acceptance names the exact fixture/reference path and diagnostic/stdout change

## Validation

Required commands:

```sh
python scripts/manager.py update-issue-index --check
python scripts/manager.py check-issue-health
python scripts/manager.py check-issue-readiness -- --fail-ready-below 80
git diff --check
```

Impacted commands:

```sh
mise run reference-coverage -- tsc --limit 2
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/ambientModules.ts --detail
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/ambientModules.ts
```

Not run:

- `cargo fmt --all --check`; issue cleanup only, no Rust code changed
- `cargo nextest run`; issue cleanup only, no implementation changed

## Docs / current-state / issue sync

Final-state docs:

- [x] not affected

Current state:

- [x] not affected

Follow-up issues:

- [x] created: `issues/open/5404-bind-dotted-ambient-namespace-qualified-access.md`

## Notes

## Affected test files

- `reference/typescript/tests/cases/compiler/ambientModules.ts`

## Duplicate detection

- `issues/done/160-implement-ambientModules.md` - Implement Ambientmodules (same reference path, same group key, title overlap)
- `issues/done/534-implement-ambientModules.md` - Implement Ambientmodules (same reference path, same feature label, same group key, title overlap)

## Smart triage

Fresh triage on 2026-05-08 shows this generated import/export bucket now
reaches a name-resolution blocker:

```text
reference/typescript/tests/cases/compiler/ambientModules.ts: UnresolvedName: name-resolution
```

Focused triage reports:

```text
UnresolvedName: unresolved name: `Foo` at 90..93
```

Representative source context:

```ts
declare namespace Foo.Bar { export var foo; };
Foo.Bar.foo = 5;
```

The compiler tokenizes and parses the dotted ambient namespace declaration and
qualified assignment. The AST keeps the outside `Foo.Bar.foo = 5` assignment,
then `resolve_names` cannot find the top-level ambient namespace identifier
`Foo`. TypeScript accepts the file with no diagnostics.

This is related to
`issues/open/5370-bind-ambient-namespace-declarations-for-qualified-value-access.md`,
but the source uses a dotted ambient namespace declaration. The specific
dotted-path work was split to
`issues/open/5404-bind-dotted-ambient-namespace-qualified-access.md`.

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
    "state": "open",
    "path": "issues/done/534-implement-ambientModules.md",
    "title": "Implement Ambientmodules",
    "reason": "same reference path, same feature label, title overlap"
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

## Completion evidence

Closed after splitting the current blocker to
`issues/open/5404-bind-dotted-ambient-namespace-qualified-access.md`.

Fresh coverage with the current binary:

```text
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/ambientModules.ts --detail --no-dashboard-data
suite=tsc
executed=1
unsupported=1
unsupported_diagcodes=UnresolvedName:1
unsupported_features=name-resolution:1
reference/typescript/tests/cases/compiler/ambientModules.ts: UnresolvedName: name-resolution
```

Fresh triage:

```text
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/ambientModules.ts
```

Observed owner boundary:

```text
[pipeline] resolve_names
error: [UnresolvedName] unresolved name: `Foo` at 90..93
```

Commits:

- split to `issues/open/5404-bind-dotted-ambient-namespace-qualified-access.md`

Validation result:

```text
command: env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/ambientModules.ts --detail --no-dashboard-data
result: pass; executed=1, unsupported=1, unsupported_diagcodes=UnresolvedName:1
date: 2026-05-08

command: env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/ambientModules.ts
result: pass; resolved dump reaches dotted ambient namespace qualified value-access blocker split to issue 5404
date: 2026-05-08
```

Remaining risks:

- none
