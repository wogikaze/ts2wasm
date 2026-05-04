---
id: 580
title: "Implement Aliaserrors"
type: spike
area: frontend/syntax
class: blocked
priority: P1
depends_on: [5001]
blocks: []
created: 2026-05-01
updated: 2026-05-01
---

## Summary

Triage aliasErrors across 1 failing reference test cases and split this bucket into implementation-ready child issues.

## Problem

Reference test results show 1 cases fail in directory `aliasErrors` with diagnostics: import-export. The compiler cannot handle these syntax/semantics, preventing compilation of code in this category.

Problem: aliasErrors has 1 reference failures and needs smart-triage evidence before implementation starts.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/aliasErrors.ts
```

Coverage window:

```sh
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/aliasErrors.ts --detail
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
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/aliasErrors.ts --detail
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/aliasErrors.ts
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

- `reference/typescript/tests/cases/compiler/aliasErrors.ts`

## Duplicate detection

- `issues/done/114-implement-aliasErrors.md` - Implement Aliaserrors (same reference path, same group key, title overlap)
- `issues/done/494-implement-aliasErrors.md` - Implement Aliaserrors (same reference path, same feature label, same group key, title overlap)

## Smart triage

### Smart triage: Triage import export: aliasErrors

- Issue class: `triage-needed`
- Feature label: `import-export`
- Diagnostic: `UnsupportedModule` / `unsupported-feature-boundary`
- Path: `reference/typescript/tests/cases/compiler/aliasErrors.ts`

Reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/aliasErrors.ts
```

Source overview:

```json
{
  "suite": "tsc",
  "bytes": 503,
  "lines": 30,
  "extension": ".ts",
  "first_code_line": "namespace foo {"
}
```

Failure location:

```json
{
  "code": "UnsupportedModule",
  "message": "issue-399: TypeScript namespace/internal module declarations require module ownership before runtime lowering at 20..29",
  "span_start": 20,
  "span_end": 29,
  "line": 2,
  "column": 2,
  "feature_label": "import-export",
  "error_type": "unsupported-feature-boundary"
}
```

Source context:

```text
1 | // @target: es2015
2 | namespace foo {    
3 |     export class Provide {
4 |     }
5 |     export namespace bar { export namespace baz {export class boo {}}}
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
    "path": "issues/done/114-implement-aliasErrors.md",
    "title": "Implement Aliaserrors",
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
    "path": "issues/done/494-implement-aliasErrors.md",
    "title": "Implement Aliaserrors",
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
            start: 20,
            end: 29,
        },
    },
    SpannedToken {
        kind: Ident(
            "foo",
        ),
        span: Span {
            start: 30,
            end: 33,
        },
    },
    SpannedToken {
        kind: LeftBrace,
        span: Span {
            start: 34,
            end: 35,
        },
    },
    SpannedToken {
        kind: Export,
        span: Span {
            start: 45,
            end: 51,
        },
    },
    SpannedToken {
        kind: Class,
        span: Span {
            start: 52,
            end: 57,
        },
    },
    SpannedToken {
        kind: Ident(
            "Provide",
        ),
        span: Span {
            start: 58,
            end: 65,
        },
    },
    SpannedToken {
        kind: LeftBrace,
        span: Span {
            start: 66,
            end: 67,
        },
    },
    SpannedToken {
        kind: RightBrace,
        span: Span {
            start: 73,
            end: 74,
        },
    },
    SpannedToken {
        kind: Export,
        span: Span {
            start: 80,
            end: 86,
        },
    },
    SpannedToken {
        kind: Ident(
            "namespace",
        ),
        span: Span {
            start: 87,
            end: 96,
        },
    },
    SpannedToken {
        kind: Ident(
            "bar",
        ),
        span: Span {
            start: 97,
            end: 100,
        },
    },
    SpannedToken {
        kind: LeftBrace,
        span: Span {
            start: 101,
            end: 102,
        },
    },
    SpannedToken {
        kind: Export,
        span: Span {
            start: 103,
            end: 109,
        },
    },
    SpannedToken {
        kind: Ident(
            "namespace",
        ),
        span: Span {
            start: 110,
            end: 119,
        },
    },
    SpannedToken {
        kind: Ident(
            "baz",
        ),
        span: Span {
            start: 120,
            end: 123,
        },
    },
    SpannedToken {
        kind: LeftBrace,
        span: Span {
            start: 124,
            end: 125,
        },
    },
    SpannedToken {
        kind: Export,
        span: Span {
            start: 125,
            end: 131,
        },
    },
    SpannedToken {
        kind: Class,
        span: Span {
            start: 132,
            end: 137,
        },
    },
    SpannedToken {
        kind: Ident(
            "boo",
        ),
        span: Span {
            start: 138,
            end: 141,
        },
    },
    SpannedToken {
        kind: LeftBrace,
        span: Span {
            start: 142,
            end: 143,
        },
    },
    SpannedToken {
        kind: RightBrace,
        span: Span {
            start: 143,
            end: 144,
        },
    },
    SpannedToken {
        kind: RightBrace,
        span: Span {
            start: 144,
            end: 145,
        },
    },
    SpannedToken {
        kind: RightBrace,
        span: Span {
            start: 145,
            end: 146,
        },
    },
    SpannedToken {
        kind: RightBrace,
        span: Span {
            start: 148,
            end: 149,
        },
    },
    SpannedToken {
        kind: Import,
        span: Span {
            start: 153,
            end: 159,
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
error: [UnsupportedModule] issue-399: TypeScript namespace/internal module declarations require module ownership before runtime lowering at 20..29
```

#### resolved

- ok: `False`
- truncated: `False`

```text
error: [UnsupportedModule] issue-399: TypeScript namespace/internal module declarations require module ownership before runtime lowering at 20..29
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
        "code": 2503,
        "category": "Error",
        "message": "Cannot find namespace 'no'.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/aliasErrors.ts",
        "start": 241,
        "length": 2,
        "line": 12,
        "character": 12
      },
      {
        "code": 2503,
        "category": "Error",
        "message": "Cannot find namespace 'no'.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/aliasErrors.ts",
        "start": 258,
        "length": 2,
        "line": 13,
        "character": 13
      },
      {
        "code": 1003,
        "category": "Error",
        "message": "Identifier expected.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/aliasErrors.ts",
        "start": 278,
        "length": 1,
        "line": 14,
        "character": 12
      },
      {
        "code": 1003,
        "category": "Error",
        "message": "Identifier expected.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/aliasErrors.ts",
        "start": 293,
        "length": 3,
        "line": 15,
        "character": 12
      },
      {
        "code": 1359,
        "category": "Error",
        "message": "Identifier expected. 'null' is a reserved word that cannot be used here.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/aliasErrors.ts",
        "start": 310,
        "length": 4,
        "line": 16,
        "character": 12
      },
      {
        "code": 2503,
        "category": "Error",
        "message": "Cannot find namespace 'undefined'.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/aliasErrors.ts",
        "start": 328,
        "length": 9,
        "line": 17,
        "character": 12
      },
      {
        "code": 2694,
        "category": "Error",
        "message": "Namespace 'foo.bar.baz' has no exported member 'bar'.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/aliasErrors.ts",
        "start": 486,
        "length": 3,
        "line": 27,
        "character": 15
      }
    ],
    "hints": [
      {
        "kind": "binding",
        "typeText": "Provide",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/aliasErrors.ts",
        "start": 348,
        "length": 1,
        "line": 20,
        "character": 5,
        "name": "p"
      },
      {
        "kind": "function",
        "typeText": "void",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/aliasErrors.ts",
        "start": 387,
        "length": 3,
        "line": 22,
        "character": 10,
        "name": "use"
      },
      {
        "kind": "binding",
        "typeText": "Provide",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/aliasErrors.ts",
        "start": 425,
        "length": 2,
        "line": 25,
        "character": 7,
        "name": "p1"
      },
      {
        "kind": "binding",
        "typeText": "Provide",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/aliasErrors.ts",
        "start": 454,
        "length": 2,
        "line": 26,
        "character": 7,
        "name": "p2"
      },
      {
        "kind": "binding",
        "typeText": "booz.bar",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/aliasErrors.ts",
        "start": 478,
        "length": 2,
        "line": 27,
        "character": 7,
        "name": "p3"
      },
      {
        "kind": "binding",
        "typeText": "Provide",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/aliasErrors.ts",
        "start": 498,
        "length": 3,
        "line": 28,
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
        "text": "namespace foo {    \r\n    export class Provide {\r\n    }\r\n    export namespace bar { export namespace baz {export class bo",
        "line": 2,
        "character": 1
      },
      {
        "kind": "ImportEqualsDeclaration",
        "text": "import provide = foo;",
        "line": 8,
        "character": 1
      },
      {
        "kind": "ImportEqualsDeclaration",
        "text": "import booz = foo.bar.baz;",
        "line": 9,
        "character": 1
      },
      {
        "kind": "ImportEqualsDeclaration",
        "text": "import beez = foo.bar;",
        "line": 10,
        "character": 1
      },
      {
        "kind": "ImportEqualsDeclaration",
        "text": "import m = no;",
        "line": 12,
        "character": 1
      },
      {
        "kind": "ImportEqualsDeclaration",
        "text": "import m2 = no.mod;",
        "line": 13,
        "character": 1
      },
      {
        "kind": "ImportEqualsDeclaration",
        "text": "import n =",
        "line": 14,
        "character": 1
      },
      {
        "kind": "ExpressionStatement",
        "text": "5;",
        "line": 14,
        "character": 12
      },
      {
        "kind": "ImportEqualsDeclaration",
        "text": "import o =",
        "line": 15,
        "character": 1
      },
      {
        "kind": "ExpressionStatement",
        "text": "\"s\";",
        "line": 15,
        "character": 12
      },
      {
        "kind": "ImportEqualsDeclaration",
        "text": "import q =",
        "line": 16,
        "character": 1
      },
      {
        "kind": "ExpressionStatement",
        "text": "null;",
        "line": 16,
        "character": 12
      },
      {
        "kind": "ImportEqualsDeclaration",
        "text": "import r = undefined;",
        "line": 17,
        "character": 1
      },
      {
        "kind": "FirstStatement",
        "text": "var p = new provide.Provide();",
        "line": 20,
        "character": 1
      },
      {
        "kind": "FunctionDeclaration",
        "text": "function use() {\r\n    \r\n  beez.baz.boo;\r\n  var p1: provide.Provide; \r\n  var p2: foo.Provide;\r\n  var p3:booz.bar;\r\n  var ",
        "line": 22,
        "character": 1
      }
    ],
    "pathToPosition": [
      {
        "kind": "SourceFile",
        "text": "namespace foo {    \r\n    export class Provide {\r\n    }\r\n    export namespace bar { export namespace baz {export class bo",
        "line": 2,
        "character": 1
      },
      {
        "kind": "ModuleDeclaration",
        "text": "namespace foo {    \r\n    export class Provide {\r\n    }\r\n    export namespace bar { export namespace baz {export class bo",
        "line": 2,
        "character": 1
      }
    ]
  }
}
```

Stack trace:

```text
error: [UnsupportedModule] issue-399: TypeScript namespace/internal module declarations require module ownership before runtime lowering at 20..29
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
