---
id: 619
title: "Implement Ambientmodulewithtemplateliterals"
type: spike
area: frontend/syntax
class: superseded
priority: P1
depends_on: [5370]
blocks: []
created: 2026-05-01
updated: 2026-05-08
completed: 2026-05-08
status: done
---

## Summary

Triage ambientModuleWithTemplateLiterals across 1 failing reference test cases and split this bucket into implementation-ready child issues.

## Problem

Fresh reference evidence shows this bucket now reaches the existing issue 5370
ambient namespace qualified value-access blocker.

Problem: this generated import/export bucket is not a standalone
implementation order; its current blocker is `UnresolvedName` for the erased
ambient namespace root `Foo`.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/ambientModuleWithTemplateLiterals.ts
```

Coverage window:

```sh
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/ambientModuleWithTemplateLiterals.ts --detail
```

## Desired final state

This generated bucket is superseded by
`issues/open/5370-bind-ambient-namespace-declarations-for-qualified-value-access.md`.
Do not implement directly from this bucket.

## Scope

In scope:

- [x] Inspect the smart triage report below
- [x] Confirm whether existing open/done issues already cover this bucket
- [x] Supersede this bucket with issue 5370's ambient namespace qualified value-access slice
- [x] Preserve exact reproduction commands and representative AST/diagnostic evidence in this closed issue

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
- [x] This closed issue contains an exact `python scripts/manager.py reference-triage ...` command
- [x] This closed issue includes failing path, diagnostic code, source context, visible symbols, and parser/TypeScript AST evidence
- [x] Completion evidence names the exact fixture/reference path and diagnostic/stdout change

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
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/ambientModuleWithTemplateLiterals.ts --detail
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/ambientModuleWithTemplateLiterals.ts
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

- [x] existing: `issues/open/5370-bind-ambient-namespace-declarations-for-qualified-value-access.md`

## Notes

## Affected test files

- `reference/typescript/tests/cases/compiler/ambientModuleWithTemplateLiterals.ts`

## Duplicate detection

- `issues/done/159-implement-ambientModuleWithTemplateLiterals.md` - Implement Ambientmodulewithtemplateliterals (same reference path, same group key, title overlap)
- `issues/done/533-implement-ambientModuleWithTemplateLiterals.md` - Implement Ambientmodulewithtemplateliterals (same reference path, same feature label, same group key, title overlap)

## Smart triage

Fresh triage on 2026-05-08 shows this generated import/export bucket now
reaches a name-resolution blocker:

```text
reference/typescript/tests/cases/compiler/ambientModuleWithTemplateLiterals.ts: UnresolvedName: name-resolution
```

Focused triage reports:

```text
UnresolvedName: unresolved name: `Foo` at 289..292
```

Representative source context:

```ts
declare namespace Foo {
    enum Bar {
        a = `1`,
        b = '2',
        c = '3'
    }

    export const a = 'string';
    export const b = `template`;
}

Foo.a;
Foo.b;
Foo.c;
```

The compiler tokenizes and parses the ambient namespace, enum members with
template/string literal initializers, exported const declarations, and later
qualified expressions. The AST keeps only the outside runtime statements, then
`resolve_names` cannot find the top-level ambient namespace identifier `Foo`.
TypeScript accepts the file with no diagnostics.

This is covered by
`issues/open/5370-bind-ambient-namespace-declarations-for-qualified-value-access.md`,
which owns binding same-file ambient `declare namespace` declarations as
resolver-visible namespace values without emitting runtime namespace
initialization.

### Smart triage: Triage import export: ambientModuleWithTemplateLiterals

- Issue class: `triage-needed`
- Feature label: `import-export`
- Diagnostic: `UnsupportedModule` / `unsupported-feature-boundary`
- Path: `reference/typescript/tests/cases/compiler/ambientModuleWithTemplateLiterals.ts`

Reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/ambientModuleWithTemplateLiterals.ts
```

Source overview:

```json
{
  "suite": "tsc",
  "bytes": 307,
  "lines": 21,
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
3 |     enum Bar {
4 |         a = `1`,
5 |         b = '2',
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
    "path": "issues/done/159-implement-ambientModuleWithTemplateLiterals.md",
    "title": "Implement Ambientmodulewithtemplateliterals",
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
    "path": "issues/done/533-implement-ambientModuleWithTemplateLiterals.md",
    "title": "Implement Ambientmodulewithtemplateliterals",
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
        kind: Ident(
            "enum",
        ),
        span: Span {
            start: 49,
            end: 53,
        },
    },
    SpannedToken {
        kind: Ident(
            "Bar",
        ),
        span: Span {
            start: 54,
            end: 57,
        },
    },
    SpannedToken {
        kind: LeftBrace,
        span: Span {
            start: 58,
            end: 59,
        },
    },
    SpannedToken {
        kind: Ident(
            "a",
        ),
        span: Span {
            start: 69,
            end: 70,
        },
    },
    SpannedToken {
        kind: Equal,
        span: Span {
            start: 71,
            end: 72,
        },
    },
    SpannedToken {
        kind: TemplateLiteral(
            "1",
        ),
        span: Span {
            start: 73,
            end: 76,
        },
    },
    SpannedToken {
        kind: Comma,
        span: Span {
            start: 76,
            end: 77,
        },
    },
    SpannedToken {
        kind: Ident(
            "b",
        ),
        span: Span {
            start: 87,
            end: 88,
        },
    },
    SpannedToken {
        kind: Equal,
        span: Span {
            start: 89,
            end: 90,
        },
    },
    SpannedToken {
        kind: String(
            "2",
        ),
        span: Span {
            start: 91,
            end: 94,
        },
    },
    SpannedToken {
        kind: Comma,
        span: Span {
            start: 94,
            end: 95,
        },
    },
    SpannedToken {
        kind: Ident(
            "c",
        ),
        span: Span {
            start: 105,
            end: 106,
        },
    },
    SpannedToken {
        kind: Equal,
        span: Span {
            start: 107,
            end: 108,
        },
    },
    SpannedToken {
        kind: String(
            "3",
        ),
        span: Span {
            start: 109,
            end: 112,
        },
    },
    SpannedToken {
        kind: RightBrace,
        span: Span {
            start: 118,
            end: 119,
        },
    },
    SpannedToken {
        kind: Export,
        span: Span {
            start: 127,
            end: 133,
        },
    },
    SpannedToken {
        kind: Const,
        span: Span {
            start: 134,
            end: 139,
        },
    },
    SpannedToken {
        kind: Ident(
            "a",
        ),
        span: Span {
            start: 140,
            end: 141,
        },
    },
    SpannedToken {
        kind: Equal,
        span: Span {
            start: 142,
            end: 143,
        },
    },
    SpannedToken {
        kind: String(
            "string",
        ),
        span: Span {
            start: 144,
            end: 152,
        },
    },
    SpannedToken {
        kind: Semicolon,
        span: Span {
            start: 152,
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
        "kind": "binding",
        "typeText": "\"string\"",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/ambientModuleWithTemplateLiterals.ts",
        "start": 140,
        "length": 1,
        "line": 9,
        "character": 18,
        "name": "a"
      },
      {
        "kind": "binding",
        "typeText": "\"template\"",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/ambientModuleWithTemplateLiterals.ts",
        "start": 172,
        "length": 1,
        "line": 10,
        "character": 18,
        "name": "b"
      },
      {
        "kind": "binding",
        "typeText": "Bar.a",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/ambientModuleWithTemplateLiterals.ts",
        "start": 208,
        "length": 1,
        "line": 12,
        "character": 18,
        "name": "c"
      },
      {
        "kind": "binding",
        "typeText": "Bar.b",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/ambientModuleWithTemplateLiterals.ts",
        "start": 237,
        "length": 1,
        "line": 13,
        "character": 18,
        "name": "d"
      },
      {
        "kind": "binding",
        "typeText": "Bar.c",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/ambientModuleWithTemplateLiterals.ts",
        "start": 269,
        "length": 1,
        "line": 14,
        "character": 18,
        "name": "e"
      }
    ],
    "typescriptVersion": "6.0.3"
  },
  "ast": {
    "topLevel": [
      {
        "kind": "ModuleDeclaration",
        "text": "declare namespace Foo {\r\n    enum Bar {\r\n        a = `1`,\r\n        b = '2',\r\n        c = '3'\r\n    }\r\n\r\n    export const ",
        "line": 2,
        "character": 1
      },
      {
        "kind": "ExpressionStatement",
        "text": "Foo.a;",
        "line": 17,
        "character": 1
      },
      {
        "kind": "ExpressionStatement",
        "text": "Foo.b;",
        "line": 18,
        "character": 1
      },
      {
        "kind": "ExpressionStatement",
        "text": "Foo.c;",
        "line": 19,
        "character": 1
      },
      {
        "kind": "ExpressionStatement",
        "text": "Foo.d;",
        "line": 20,
        "character": 1
      },
      {
        "kind": "ExpressionStatement",
        "text": "Foo.e;",
        "line": 21,
        "character": 1
      }
    ],
    "pathToPosition": [
      {
        "kind": "SourceFile",
        "text": "declare namespace Foo {\r\n    enum Bar {\r\n        a = `1`,\r\n        b = '2',\r\n        c = '3'\r\n    }\r\n\r\n    export const ",
        "line": 2,
        "character": 1
      },
      {
        "kind": "ModuleDeclaration",
        "text": "declare namespace Foo {\r\n    enum Bar {\r\n        a = `1`,\r\n        b = '2',\r\n        c = '3'\r\n    }\r\n\r\n    export const ",
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

## Completion evidence

Closed as superseded by
`issues/open/5370-bind-ambient-namespace-declarations-for-qualified-value-access.md`.

Fresh coverage with the current binary:

```text
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/ambientModuleWithTemplateLiterals.ts --detail --no-dashboard-data
suite=tsc
executed=1
unsupported=1
unsupported_diagcodes=UnresolvedName:1
unsupported_features=name-resolution:1
reference/typescript/tests/cases/compiler/ambientModuleWithTemplateLiterals.ts: UnresolvedName: name-resolution
```

Fresh triage:

```text
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/ambientModuleWithTemplateLiterals.ts
```

Observed owner boundary:

```text
[pipeline] resolve_names
error: [UnresolvedName] unresolved name: `Foo` at 289..292
```

Issue 5370 already tracks resolving ambient namespace roots for qualified
value access while preserving ambient erasure. This reference is the same
observable behavior with `Foo.a`, `Foo.b`, `Foo.c`, `Foo.d`, and `Foo.e`, so
no new child issue is created.

Commits:

- superseded by `issues/open/5370-bind-ambient-namespace-declarations-for-qualified-value-access.md`

Validation result:

```text
command: env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/ambientModuleWithTemplateLiterals.ts --detail --no-dashboard-data
result: pass; executed=1, unsupported=1, unsupported_diagcodes=UnresolvedName:1
date: 2026-05-08

command: env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/ambientModuleWithTemplateLiterals.ts
result: pass; resolved dump reaches issue-5370 ambient namespace qualified value-access boundary
date: 2026-05-08
```

Remaining risks:

- none
