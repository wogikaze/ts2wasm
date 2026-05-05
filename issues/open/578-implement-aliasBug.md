---
id: 578
title: "Implement Aliasbug"
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

Triage aliasBug across 1 failing reference test cases and split this bucket into implementation-ready child issues.

## Problem

Reference test results show 1 cases fail in directory `aliasBug` with diagnostics: import-export. The compiler cannot handle these syntax/semantics, preventing compilation of code in this category.

Problem: aliasBug has 1 reference failures and needs smart-triage evidence before implementation starts.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/aliasBug.ts
```

Coverage window:

```sh
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/aliasBug.ts --detail
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
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/aliasBug.ts --detail
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/aliasBug.ts
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

- `reference/typescript/tests/cases/compiler/aliasBug.ts`

## Duplicate detection

- `issues/done/112-implement-aliasBug.md` - Implement Aliasbug (same reference path, same group key, title overlap)
- `issues/done/492-implement-aliasBug.md` - Implement Aliasbug (same reference path, same feature label, same group key, title overlap)

## Smart triage

### Smart triage: Triage import export: aliasBug

- Issue class: `triage-needed`
- Feature label: `import-export`
- Diagnostic: `UnsupportedModule` / `unsupported-feature-boundary`
- Path: `reference/typescript/tests/cases/compiler/aliasBug.ts`

Reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/aliasBug.ts
```

Source overview:

```json
{
  "suite": "tsc",
  "bytes": 408,
  "lines": 21,
  "extension": ".ts",
  "first_code_line": "namespace foo {"
}
```

Failure location:

```json
{
  "code": "UnsupportedModule",
  "message": "issue-399: TypeScript namespace/internal module declarations require module ownership before runtime lowering at 44..53",
  "span_start": 44,
  "span_end": 53,
  "line": 4,
  "column": 4,
  "feature_label": "import-export",
  "error_type": "unsupported-feature-boundary"
}
```

Source context:

```text
1 | // @target: es2015
2 | // @module: commonjs
3 | 
4 | namespace foo {    
5 |     export class Provide {
6 |     }
7 |
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
    "path": "issues/done/112-implement-aliasBug.md",
    "title": "Implement Aliasbug",
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
    "path": "issues/done/492-implement-aliasBug.md",
    "title": "Implement Aliasbug",
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
            "namespace",
        ),
        span: Span {
            start: 44,
            end: 53,
        },
    },
    SpannedToken {
        kind: Ident(
            "foo",
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
        kind: Export,
        span: Span {
            start: 69,
            end: 75,
        },
    },
    SpannedToken {
        kind: Class,
        span: Span {
            start: 76,
            end: 81,
        },
    },
    SpannedToken {
        kind: Ident(
            "Provide",
        ),
        span: Span {
            start: 82,
            end: 89,
        },
    },
    SpannedToken {
        kind: LeftBrace,
        span: Span {
            start: 90,
            end: 91,
        },
    },
    SpannedToken {
        kind: RightBrace,
        span: Span {
            start: 97,
            end: 98,
        },
    },
    SpannedToken {
        kind: Export,
        span: Span {
            start: 106,
            end: 112,
        },
    },
    SpannedToken {
        kind: Ident(
            "namespace",
        ),
        span: Span {
            start: 113,
            end: 122,
        },
    },
    SpannedToken {
        kind: Ident(
            "bar",
        ),
        span: Span {
            start: 123,
            end: 126,
        },
    },
    SpannedToken {
        kind: LeftBrace,
        span: Span {
            start: 127,
            end: 128,
        },
    },
    SpannedToken {
        kind: Export,
        span: Span {
            start: 129,
            end: 135,
        },
    },
    SpannedToken {
        kind: Ident(
            "namespace",
        ),
        span: Span {
            start: 136,
            end: 145,
        },
    },
    SpannedToken {
        kind: Ident(
            "baz",
        ),
        span: Span {
            start: 146,
            end: 149,
        },
    },
    SpannedToken {
        kind: LeftBrace,
        span: Span {
            start: 150,
            end: 151,
        },
    },
    SpannedToken {
        kind: Export,
        span: Span {
            start: 151,
            end: 157,
        },
    },
    SpannedToken {
        kind: Class,
        span: Span {
            start: 158,
            end: 163,
        },
    },
    SpannedToken {
        kind: Ident(
            "boo",
        ),
        span: Span {
            start: 164,
            end: 167,
        },
    },
    SpannedToken {
        kind: LeftBrace,
        span: Span {
            start: 168,
            end: 169,
        },
    },
    SpannedToken {
        kind: RightBrace,
        span: Span {
            start: 169,
            end: 170,
        },
    },
    SpannedToken {
        kind: RightBrace,
        span: Span {
            start: 170,
            end: 171,
        },
    },
    SpannedToken {
        kind: RightBrace,
        span: Span {
            start: 171,
            end: 172,
        },
    },
    SpannedToken {
        kind: RightBrace,
        span: Span {
            start: 174,
            end: 175,
        },
    },
    SpannedToken {
        kind: Import,
        span: Span {
            start: 179,
            end: 185,
        },
    },
    SpannedToken {
        kind: Ident(
            "provide",
```

#### ast

- ok: `False`
- truncated: `False`

```text
error: [UnsupportedModule] issue-399: TypeScript namespace/internal module declarations require module ownership before runtime lowering at 44..53
```

#### resolved

- ok: `False`
- truncated: `False`

```text
error: [UnsupportedModule] issue-399: TypeScript namespace/internal module declarations require module ownership before runtime lowering at 44..53
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
        "code": 2694,
        "category": "Error",
        "message": "Namespace 'foo.bar.baz' has no exported member 'bar'.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/aliasBug.ts",
        "start": 384,
        "length": 3,
        "line": 19,
        "character": 15
      }
    ],
    "hints": [
      {
        "kind": "binding",
        "typeText": "Provide",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/aliasBug.ts",
        "start": 236,
        "length": 1,
        "line": 14,
        "character": 5,
        "name": "p"
      },
      {
        "kind": "function",
        "typeText": "void",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/aliasBug.ts",
        "start": 275,
        "length": 3,
        "line": 16,
        "character": 10,
        "name": "use"
      },
      {
        "kind": "binding",
        "typeText": "Provide",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/aliasBug.ts",
        "start": 290,
        "length": 2,
        "line": 17,
        "character": 7,
        "name": "p1"
      },
      {
        "kind": "binding",
        "typeText": "Provide",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/aliasBug.ts",
        "start": 352,
        "length": 2,
        "line": 18,
        "character": 7,
        "name": "p2"
      },
      {
        "kind": "binding",
        "typeText": "booz.bar",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/aliasBug.ts",
        "start": 376,
        "length": 2,
        "line": 19,
        "character": 7,
        "name": "p3"
      },
      {
        "kind": "binding",
        "typeText": "Provide",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/aliasBug.ts",
        "start": 396,
        "length": 3,
        "line": 20,
        "character": 7,
        "name": "p22"
      }
    ],
    "typescriptVersion": "6.0.3"
  },
  "ast": {
    "topLevel": [
      {
        "kind": "ModuleDeclaration",
        "text": "namespace foo {    \r\n    export class Provide {\r\n    }\r\n\r\n    export namespace bar { export namespace baz {export class ",
        "line": 4,
        "character": 1
      },
      {
        "kind": "ImportEqualsDeclaration",
        "text": "import provide = foo;",
        "line": 11,
        "character": 1
      },
      {
        "kind": "ImportEqualsDeclaration",
        "text": "import booz = foo.bar.baz;",
        "line": 12,
        "character": 1
      },
      {
        "kind": "FirstStatement",
        "text": "var p = new provide.Provide();",
        "line": 14,
        "character": 1
      },
      {
        "kind": "FunctionDeclaration",
        "text": "function use() {\r\n  var p1: provide.Provide; // error here, but should be okay\r\n  var p2: foo.Provide;\r\n  var p3:booz.ba",
        "line": 16,
        "character": 1
      }
    ],
    "pathToPosition": [
      {
        "kind": "SourceFile",
        "text": "namespace foo {    \r\n    export class Provide {\r\n    }\r\n\r\n    export namespace bar { export namespace baz {export class ",
        "line": 4,
        "character": 1
      },
      {
        "kind": "ModuleDeclaration",
        "text": "namespace foo {    \r\n    export class Provide {\r\n    }\r\n\r\n    export namespace bar { export namespace baz {export class ",
        "line": 4,
        "character": 1
      }
    ]
  }
}
```

Stack trace:

```text
error: [UnsupportedModule] issue-399: TypeScript namespace/internal module declarations require module ownership before runtime lowering at 44..53
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
