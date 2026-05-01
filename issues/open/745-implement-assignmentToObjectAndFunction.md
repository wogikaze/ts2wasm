---
id: 745
title: "Implement Assignmenttoobjectandfunction"
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

Triage assignmentToObjectAndFunction across 1 failing reference test cases and split this bucket into implementation-ready child issues.

## Problem

Reference test results show 1 cases fail in directory `assignmentToObjectAndFunction` with diagnostics: import-export. The compiler cannot handle these syntax/semantics, preventing compilation of code in this category.

Problem: assignmentToObjectAndFunction has 1 reference failures and needs smart-triage evidence before implementation starts.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/assignmentToObjectAndFunction.ts
```

Coverage window:

```sh
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/assignmentToObjectAndFunction.ts --detail
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
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/assignmentToObjectAndFunction.ts --detail
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/assignmentToObjectAndFunction.ts
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

- `reference/typescript/tests/cases/compiler/assignmentToObjectAndFunction.ts`

## Duplicate detection

- none found by path/title/feature scan

## Smart triage

### Smart triage: Triage import export: assignmentToObjectAndFunction

- Issue class: `triage-needed`
- Feature label: `import-export`
- Diagnostic: `UnsupportedModule` / `unsupported-feature-boundary`
- Path: `reference/typescript/tests/cases/compiler/assignmentToObjectAndFunction.ts`

Reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/assignmentToObjectAndFunction.ts
```

Source overview:

```json
{
  "suite": "tsc",
  "bytes": 648,
  "lines": 31,
  "extension": ".ts",
  "first_code_line": "var errObj: Object = { toString: 0 }; // Error, incompatible toString"
}
```

Failure location:

```json
{
  "code": "UnsupportedModule",
  "message": "issue-399: TypeScript namespace/internal module declarations require module ownership before runtime lowering at 327..336",
  "span_start": 327,
  "span_end": 336,
  "line": 13,
  "column": 13,
  "feature_label": "import-export",
  "error_type": "unsupported-feature-boundary"
}
```

Source context:

```text
10 | var errFun: Function = {}; // Error for no call signature
11 | 
12 | function foo() { }
13 | namespace foo {
14 |     export var boom = 0;
15 | }
16 |
```

Visible symbols before failure:

```json
[
  {
    "kind": "binding",
    "name": "errObj",
    "line": 3,
    "column": 1
  },
  {
    "kind": "binding",
    "name": "goodObj",
    "line": 4,
    "column": 1
  },
  {
    "kind": "binding",
    "name": "errFun",
    "line": 10,
    "column": 1
  },
  {
    "kind": "function",
    "name": "foo",
    "line": 12,
    "column": 1,
    "params": ""
  }
]
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
    "state": "open",
    "path": "issues/open/732-implement-assignmentCompatability-import-export.md",
    "title": "Implement Assignmentcompatability Import Export",
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
        kind: Var,
        span: Span {
            start: 39,
            end: 42,
        },
    },
    SpannedToken {
        kind: Ident(
            "errObj",
        ),
        span: Span {
            start: 43,
            end: 49,
        },
    },
    SpannedToken {
        kind: Colon,
        span: Span {
            start: 49,
            end: 50,
        },
    },
    SpannedToken {
        kind: Ident(
            "Object",
        ),
        span: Span {
            start: 51,
            end: 57,
        },
    },
    SpannedToken {
        kind: Equal,
        span: Span {
            start: 58,
            end: 59,
        },
    },
    SpannedToken {
        kind: LeftBrace,
        span: Span {
            start: 60,
            end: 61,
        },
    },
    SpannedToken {
        kind: Ident(
            "toString",
        ),
        span: Span {
            start: 62,
            end: 70,
        },
    },
    SpannedToken {
        kind: Colon,
        span: Span {
            start: 70,
            end: 71,
        },
    },
    SpannedToken {
        kind: Number(
            0,
        ),
        span: Span {
            start: 72,
            end: 73,
        },
    },
    SpannedToken {
        kind: RightBrace,
        span: Span {
            start: 74,
            end: 75,
        },
    },
    SpannedToken {
        kind: Semicolon,
        span: Span {
            start: 75,
            end: 76,
        },
    },
    SpannedToken {
        kind: Var,
        span: Span {
            start: 110,
            end: 113,
        },
    },
    SpannedToken {
        kind: Ident(
            "goodObj",
        ),
        span: Span {
            start: 114,
            end: 121,
        },
    },
    SpannedToken {
        kind: Colon,
        span: Span {
            start: 121,
            end: 122,
        },
    },
    SpannedToken {
        kind: Ident(
            "Object",
        ),
        span: Span {
            start: 123,
            end: 129,
        },
    },
    SpannedToken {
        kind: Equal,
        span: Span {
            start: 130,
            end: 131,
        },
    },
    SpannedToken {
        kind: LeftBrace,
        span: Span {
            start: 132,
            end: 133,
        },
    },
    SpannedToken {
        kind: Ident(
            "toString",
        ),
        span: Span {
            start: 139,
            end: 147,
        },
    },
    SpannedToken {
        kind: LeftParen,
        span: Span {
            start: 147,
            end: 148,
        },
    },
    SpannedToken {
        kind: Ident(
            "x",
        ),
        span: Span {
            start: 148,
            end: 149,
        },
    },
    SpannedToken {
        kind: Question,
        span: Span {
            start: 149,
            end: 150,
        },
    },
    SpannedToken {
        kind: RightParen,
        span: Span {
            start: 150,
            end: 151,
        },
    },
    SpannedToken {
        kind: LeftBrace,
        span: Span {
            start: 152,
            end: 153,
        },
    },
    SpannedToken {
        kind: Return,
        span: Span {
            start: 163,
            end: 169,
        },
    },
    SpannedToken {
        kind: String(
            "",
        ),
        span: Span {
            start: 170,
            end: 172,
        },
    },
    SpannedToken {
        kind: Semicolon,
        span: Span {
```

#### ast

- ok: `False`
- truncated: `False`

```text
error: [UnsupportedModule] issue-399: TypeScript namespace/internal module declarations require module ownership before runtime lowering at 327..336
```

#### resolved

- ok: `False`
- truncated: `False`

```text
error: [UnsupportedModule] issue-399: TypeScript namespace/internal module declarations require module ownership before runtime lowering at 327..336
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
        "code": 2322,
        "category": "Error",
        "message": "Type 'number' is not assignable to type '() => string'.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/assignmentToObjectAndFunction.ts",
        "start": 62,
        "length": 8,
        "line": 3,
        "character": 24
      },
      {
        "code": 2740,
        "category": "Error",
        "message": "Type '{}' is missing the following properties from type 'Function': apply, call, bind, prototype, and 5 more.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/assignmentToObjectAndFunction.ts",
        "start": 250,
        "length": 6,
        "line": 10,
        "character": 5
      },
      {
        "code": 2322,
        "category": "Error",
        "message": "Type 'typeof bad' is not assignable to type 'Function'.\n  Types of property 'apply' are incompatible.\n    Type 'number' is not assignable to type '(this: Function, thisArg: any, argArray?: any) => any'.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/assignmentToObjectAndFunction.ts",
        "start": 642,
        "length": 10,
        "line": 31,
        "character": 5
      }
    ],
    "hints": [
      {
        "kind": "binding",
        "typeText": "Object",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/assignmentToObjectAndFunction.ts",
        "start": 43,
        "length": 6,
        "line": 3,
        "character": 5,
        "name": "errObj"
      },
      {
        "kind": "binding",
        "typeText": "Object",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/assignmentToObjectAndFunction.ts",
        "start": 114,
        "length": 7,
        "line": 4,
        "character": 5,
        "name": "goodObj"
      },
      {
        "kind": "parameter",
        "typeText": "any",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/assignmentToObjectAndFunction.ts",
        "start": 148,
        "length": 1,
        "line": 5,
        "character": 14,
        "name": "x"
      },
      {
        "kind": "binding",
        "typeText": "Function",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/assignmentToObjectAndFunction.ts",
        "start": 250,
        "length": 6,
        "line": 10,
        "character": 5,
        "name": "errFun"
      },
      {
        "kind": "function",
        "typeText": "void",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/assignmentToObjectAndFunction.ts",
        "start": 316,
        "length": 3,
        "line": 12,
        "character": 10,
        "name": "foo"
      },
      {
        "kind": "binding",
        "typeText": "number",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/assignmentToObjectAndFunction.ts",
        "start": 359,
        "length": 4,
        "line": 14,
        "character": 16,
        "name": "boom"
      },
      {
        "kind": "binding",
        "typeText": "Function",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/assignmentToObjectAndFunction.ts",
        "start": 379,
        "length": 11,
        "line": 17,
        "character": 5,
        "name": "goodFundule"
      },
      {
        "kind": "function",
        "typeText": "void",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/assignmentToObjectAndFunction.ts",
        "start": 426,
        "length": 3,
        "line": 19,
        "character": 10,
        "name": "bar"
      },
      {
        "kind": "function",
        "typeText": "void",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/assignmentToObjectAndFunction.ts",
        "start": 474,
        "length": 5,
        "line": 21,
        "character": 21,
        "name": "apply"
      },
      {
        "kind": "parameter",
        "typeText": "string",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/assignmentToObjectAndFunction.ts",
        "start": 480,
        "length": 7,
        "line": 21,
        "character": 27,
        "name": "thisArg"
      },
      {
        "kind": "parameter",
        "typeText": "string | undefined",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/assignmentToObjectAndFunction.ts",
        "start": 497,
        "length": 8,
        "line": 21,
        "character": 44,
        "name": "argArray"
      },
      {
        "kind": "binding",
        "typeText": "Function",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/assignmentToObjectAndFunction.ts",
        "start": 530,
        "length": 12,
        "line": 24,
        "character": 5,
        "name": "goodFundule2"
      },
      {
        "kind": "function",
        "typeText": "void",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/assignmentToObjectAndFunction.ts",
        "start": 578,
        "length": 3,
        "line": 26,
        "character": 10,
        "name": "bad"
      },
      {
        "kind": "binding",
        "typeText": "number",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/assignmentToObjectAndFunction.ts",
        "start": 621,
        "length": 5,
        "line": 28,
        "character": 16,
        "name": "apply"
      },
      {
        "kind": "binding",
        "typeText": "Function",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/assignmentToObjectAndFunction.ts",
        "start": 642,
        "length": 10,
        "line": 31,
        "character": 5,
        "name": "badFundule"
      }
    ],
    "typescriptVersion": "6.0.3"
  },
  "ast": {
    "topLevel": [
      {
        "kind": "FirstStatement",
        "text": "var errObj: Object = { toString: 0 };",
        "line": 3,
        "character": 1
      },
      {
        "kind": "FirstStatement",
        "text": "var goodObj: Object = {\r\n    toString(x?) {\r\n        return \"\";\r\n    }\r\n};",
        "line": 4,
        "character": 1
      },
      {
        "kind": "FirstStatement",
        "text": "var errFun: Function = {};",
        "line": 10,
        "character": 1
      },
      {
        "kind": "FunctionDeclaration",
        "text": "function foo() { }",
        "line": 12,
        "character": 1
      },
      {
        "kind": "ModuleDeclaration",
        "text": "namespace foo {\r\n    export var boom = 0;\r\n}",
        "line": 13,
        "character": 1
      },
      {
        "kind": "FirstStatement",
        "text": "var goodFundule: Function = foo;",
        "line": 17,
        "character": 1
      },
      {
        "kind": "FunctionDeclaration",
        "text": "function bar() { }",
        "line": 19,
        "character": 1
      },
      {
        "kind": "ModuleDeclaration",
        "text": "namespace bar {\r\n    export function apply(thisArg: string, argArray?: string) { }\r\n}",
        "line": 20,
        "character": 1
      },
      {
        "kind": "FirstStatement",
        "text": "var goodFundule2: Function = bar;",
        "line": 24,
        "character": 1
      },
      {
        "kind": "FunctionDeclaration",
        "text": "function bad() { }",
        "line": 26,
        "character": 1
      },
      {
        "kind": "ModuleDeclaration",
        "text": "namespace bad {\r\n    export var apply = 0;\r\n}",
        "line": 27,
        "character": 1
      },
      {
        "kind": "FirstStatement",
        "text": "var badFundule: Function = bad;",
        "line": 31,
        "character": 1
      }
    ],
    "pathToPosition": [
```

Stack trace:

```text
error: [UnsupportedModule] issue-399: TypeScript namespace/internal module declarations require module ownership before runtime lowering at 327..336
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
