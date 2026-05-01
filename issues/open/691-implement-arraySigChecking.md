---
id: 691
title: "Implement Arraysigchecking"
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

Triage arraySigChecking across 1 failing reference test cases and split this bucket into implementation-ready child issues.

## Problem

Reference test results show 1 cases fail in directory `arraySigChecking` with diagnostics: import-export. The compiler cannot handle these syntax/semantics, preventing compilation of code in this category.

Problem: arraySigChecking has 1 reference failures and needs smart-triage evidence before implementation starts.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/arraySigChecking.ts
```

Coverage window:

```sh
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/arraySigChecking.ts --detail
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
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/arraySigChecking.ts --detail
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/arraySigChecking.ts
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

- `reference/typescript/tests/cases/compiler/arraySigChecking.ts`

## Duplicate detection

- none found by path/title/feature scan

## Smart triage

### Smart triage: Triage import export: arraySigChecking

- Issue class: `triage-needed`
- Feature label: `import-export`
- Diagnostic: `UnsupportedModule` / `unsupported-feature-boundary`
- Path: `reference/typescript/tests/cases/compiler/arraySigChecking.ts`

Reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/arraySigChecking.ts
```

Source overview:

```json
{
  "suite": "tsc",
  "bytes": 542,
  "lines": 32,
  "extension": ".ts",
  "first_code_line": "declare namespace M {"
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
2 | declare namespace M {
3 |     interface iBar { t: any; }
4 |     interface iFoo extends iBar {
5 |         s: any;
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
    "state": "open",
    "path": "issues/open/662-implement-arrayAssignmentTest-import-export.md",
    "title": "Implement Arrayassignmenttest Import Export",
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
            "M",
        ),
        span: Span {
            start: 38,
            end: 39,
        },
    },
    SpannedToken {
        kind: LeftBrace,
        span: Span {
            start: 40,
            end: 41,
        },
    },
    SpannedToken {
        kind: Ident(
            "interface",
        ),
        span: Span {
            start: 47,
            end: 56,
        },
    },
    SpannedToken {
        kind: Ident(
            "iBar",
        ),
        span: Span {
            start: 57,
            end: 61,
        },
    },
    SpannedToken {
        kind: LeftBrace,
        span: Span {
            start: 62,
            end: 63,
        },
    },
    SpannedToken {
        kind: Ident(
            "t",
        ),
        span: Span {
            start: 64,
            end: 65,
        },
    },
    SpannedToken {
        kind: Colon,
        span: Span {
            start: 65,
            end: 66,
        },
    },
    SpannedToken {
        kind: Ident(
            "any",
        ),
        span: Span {
            start: 67,
            end: 70,
        },
    },
    SpannedToken {
        kind: Semicolon,
        span: Span {
            start: 70,
            end: 71,
        },
    },
    SpannedToken {
        kind: RightBrace,
        span: Span {
            start: 72,
            end: 73,
        },
    },
    SpannedToken {
        kind: Ident(
            "interface",
        ),
        span: Span {
            start: 79,
            end: 88,
        },
    },
    SpannedToken {
        kind: Ident(
            "iFoo",
        ),
        span: Span {
            start: 89,
            end: 93,
        },
    },
    SpannedToken {
        kind: Extends,
        span: Span {
            start: 94,
            end: 101,
        },
    },
    SpannedToken {
        kind: Ident(
            "iBar",
        ),
        span: Span {
            start: 102,
            end: 106,
        },
    },
    SpannedToken {
        kind: LeftBrace,
        span: Span {
            start: 107,
            end: 108,
        },
    },
    SpannedToken {
        kind: Ident(
            "s",
        ),
        span: Span {
            start: 118,
            end: 119,
        },
    },
    SpannedToken {
        kind: Colon,
        span: Span {
            start: 119,
            end: 120,
        },
    },
    SpannedToken {
        kind: Ident(
            "any",
        ),
        span: Span {
            start: 121,
            end: 124,
        },
    },
    SpannedToken {
        kind: Semicolon,
        span: Span {
            start: 124,
            end: 125,
        },
    },
    SpannedToken {
        kind: RightBrace,
        span: Span {
            start: 131,
            end: 132,
        },
    },
    SpannedToken {
        kind: Class,
        span: Span {
            start: 140,
            end: 145,
        },
    },
    SpannedToken {
        kind: Ident(
            "cFoo",
        ),
        span: Span {
            start: 146,
            end: 150,
        },
    },
    SpannedToken {
        kind: LeftBrace,
        span: Span {
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
    "ok": false,
    "diagnostics": [
      {
        "code": 1268,
        "category": "Error",
        "message": "An index signature parameter type must be 'string', 'number', 'symbol', or a template literal type.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/arraySigChecking.ts",
        "start": 196,
        "length": 5,
        "line": 12,
        "character": 17
      },
      {
        "code": 2322,
        "category": "Error",
        "message": "Type 'void' is not assignable to type 'string'.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/arraySigChecking.ts",
        "start": 338,
        "length": 14,
        "line": 19,
        "character": 27
      },
      {
        "code": 2322,
        "category": "Error",
        "message": "Type 'number' is not assignable to type 'number[]'.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/arraySigChecking.ts",
        "start": 400,
        "length": 1,
        "line": 23,
        "character": 13
      },
      {
        "code": 2322,
        "category": "Error",
        "message": "Type 'number' is not assignable to type 'number[]'.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/arraySigChecking.ts",
        "start": 403,
        "length": 1,
        "line": 23,
        "character": 16
      }
    ],
    "hints": [
      {
        "kind": "binding",
        "typeText": "{}",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/arraySigChecking.ts",
        "start": 188,
        "length": 3,
        "line": 12,
        "character": 9,
        "name": "foo"
      },
      {
        "kind": "parameter",
        "typeText": "any",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/arraySigChecking.ts",
        "start": 196,
        "length": 5,
        "line": 12,
        "character": 17,
        "name": "index"
      },
      {
        "kind": "binding",
        "typeText": "myInt",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/arraySigChecking.ts",
        "start": 297,
        "length": 5,
        "line": 18,
        "character": 13,
        "name": "myVar"
      },
      {
        "kind": "binding",
        "typeText": "string[]",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/arraySigChecking.ts",
        "start": 316,
        "length": 8,
        "line": 19,
        "character": 5,
        "name": "strArray"
      },
      {
        "kind": "binding",
        "typeText": "number[][][]",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/arraySigChecking.ts",
        "start": 364,
        "length": 7,
        "line": 22,
        "character": 5,
        "name": "myArray"
      },
      {
        "kind": "function",
        "typeText": "boolean",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/arraySigChecking.ts",
        "start": 420,
        "length": 7,
        "line": 25,
        "character": 10,
        "name": "isEmpty"
      },
      {
        "kind": "parameter",
        "typeText": "{ length: number; }",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/arraySigChecking.ts",
        "start": 428,
        "length": 1,
        "line": 25,
        "character": 18,
        "name": "l"
      }
    ],
    "typescriptVersion": "6.0.3"
  },
  "ast": {
    "topLevel": [
      {
        "kind": "ModuleDeclaration",
        "text": "declare namespace M {\r\n    interface iBar { t: any; }\r\n    interface iFoo extends iBar {\r\n        s: any;\r\n    }\r\n\r\n    ",
        "line": 2,
        "character": 1
      },
      {
        "kind": "InterfaceDeclaration",
        "text": "interface myInt {\r\n    voidFn(): void;\r\n}",
        "line": 15,
        "character": 1
      },
      {
        "kind": "FirstStatement",
        "text": "declare var myVar: myInt;",
        "line": 18,
        "character": 1
      },
      {
        "kind": "FirstStatement",
        "text": "var strArray: string[] = [myVar.voidFn()];",
        "line": 19,
        "character": 1
      },
      {
        "kind": "FirstStatement",
        "text": "var myArray: number[][][];",
        "line": 22,
        "character": 1
      },
      {
        "kind": "ExpressionStatement",
        "text": "myArray = [[1, 2]];",
        "line": 23,
        "character": 1
      },
      {
        "kind": "FunctionDeclaration",
        "text": "function isEmpty(l: { length: number }) {\r\n    return l.length === 0;\r\n}",
        "line": 25,
        "character": 1
      },
      {
        "kind": "ExpressionStatement",
        "text": "isEmpty([]);",
        "line": 29,
        "character": 1
      },
      {
        "kind": "ExpressionStatement",
        "text": "isEmpty(new Array(3));",
        "line": 30,
        "character": 1
      },
      {
        "kind": "ExpressionStatement",
        "text": "isEmpty(new Array<string>(3));",
        "line": 31,
        "character": 1
      },
      {
        "kind": "ExpressionStatement",
        "text": "isEmpty(['a']);",
        "line": 32,
        "character": 1
      }
    ],
    "pathToPosition": [
      {
        "kind": "SourceFile",
        "text": "declare namespace M {\r\n    interface iBar { t: any; }\r\n    interface iFoo extends iBar {\r\n        s: any;\r\n    }\r\n\r\n    ",
        "line": 2,
        "character": 1
      },
      {
        "kind": "ModuleDeclaration",
        "text": "declare namespace M {\r\n    interface iBar { t: any; }\r\n    interface iFoo extends iBar {\r\n        s: any;\r\n    }\r\n\r\n    ",
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
