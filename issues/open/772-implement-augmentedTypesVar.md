---
id: 772
title: "Implement Augmentedtypesvar"
type: spike
area: frontend/resolver
class: superseded
priority: P1
depends_on: []
blocks: []
created: 2026-05-01
updated: 2026-05-07
completed: 2026-05-07
status: done
---
> **Reopened by false-done audit** (2026-05-07)
> Classification: must-reopen
> Reason: Batch-migrated to issues/open/ without implementation commits.
> Evidence: Empty completion evidence. No feat/fix commit for #772.

## Summary

Closed this generated bucket as superseded by
`issues/open/5307-report-var-function-duplicate-identifier-diagnostics.md`.

## Problem

Fresh triage shows the old parser-syntax blocker is stale. The current first
blocker is a resolver duplicate-identifier boundary for `var x2 = 1;` followed
by `function x2() { }`, which belongs with issue 5307's var/function duplicate
diagnostic work.

Problem: the generated parser-syntax bucket remained blocked even though its
current executable work is already tracked by issue 5307.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/augmentedTypesVar.ts
```

Coverage window:

```sh
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/augmentedTypesVar.ts --detail
```

## Desired final state

This generated bucket is closed. Implement from
`issues/open/5307-report-var-function-duplicate-identifier-diagnostics.md`.

## Scope

In scope:

- [x] Inspect the smart triage report below
- [x] Confirm whether existing open/done issues already cover this bucket
- [x] Supersede this generated bucket with the existing implementation-ready issue
- [x] Preserve exact reproduction commands and representative diagnostic evidence

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

- [x] Duplicate candidates below are confirmed and this issue is superseded
- [x] Superseding issue 5307 contains the implementation scope
- [x] Current triage evidence is recorded
- [x] Superseding issue acceptance names the var/function diagnostic change

## Validation

Required commands:

```sh
python scripts/manager.py update-issue-index --check
python scripts/manager.py check-issue-health
python scripts/manager.py check-issue-readiness -- --fail-ready-below 80
```

Impacted commands:

```sh
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/augmentedTypesVar.ts --detail --no-dashboard-data
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/augmentedTypesVar.ts
```

Not run:

- cargo fmt / nextest not run for this metadata-only issue lifecycle closure

## Docs / current-state / issue sync

Final-state docs:

- [x] not affected

Current state:

- [x] not affected

Follow-up issues:

- [x] `issues/open/5307-report-var-function-duplicate-identifier-diagnostics.md`

## Notes

## Affected test files

- `reference/typescript/tests/cases/compiler/augmentedTypesVar.ts`

## Duplicate detection

Superseded by `issues/open/5307-report-var-function-duplicate-identifier-diagnostics.md`.

Evidence:

- Current diagnostic: `DuplicateLocal`
- Current message: top-level function `x2` conflicts with existing lexical
  binding at `109..117`
- Current source:

```text
var x2 = 1; // error
function x2() { } // error
```

- TypeScript oracle reports duplicate identifier TS2300 diagnostics at both
  `x2` identifiers.

Current coverage:

```text
executed=1
build_pass=0
unsupported=1
unsupported_diagcodes=DuplicateLocal:1
unsupported_features=duplicate-local:1
semantic_enabled=0
reference/typescript/tests/cases/compiler/augmentedTypesVar.ts: DuplicateLocal: duplicate-local
```

## Current smart triage

### Smart triage: Triage duplicate local: augmentedTypesVar

- Issue class: `triage-needed`
- Feature label: `duplicate-local`
- Diagnostic: `DuplicateLocal` / `compiler-diagnostic`
- Path: `reference/typescript/tests/cases/compiler/augmentedTypesVar.ts`

Reproduction:

```sh
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/augmentedTypesVar.ts
```

Failure location:

```json
{
  "code": "DuplicateLocal",
  "message": "top-level function `x2` conflicts with existing lexical binding at 109..117",
  "span_start": 109,
  "span_end": 117,
  "line": 8,
  "column": 8,
  "feature_label": "duplicate-local",
  "error_type": "compiler-diagnostic"
}
```

Source context:

```text
 5 |
 6 | // var then function
 7 | var x2 = 1; // error
 8 | function x2() { } // error
 9 |
10 | var x3 = 1;
11 | var x3 = () => { } // error
```

## Stale generated smart triage

### Smart triage: Triage parser syntax: augmentedTypesVar

- Issue class: `triage-needed`
- Feature label: `parser-syntax`
- Diagnostic: `UnsupportedSyntax` / `parser-or-frontend-unsupported`
- Path: `reference/typescript/tests/cases/compiler/augmentedTypesVar.ts`

Reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/augmentedTypesVar.ts
```

Source overview:

```json
{
  "suite": "tsc",
  "bytes": 673,
  "lines": 36,
  "extension": ".ts",
  "first_code_line": "var x1 = 1;"
}
```

Failure location:

```json
{
  "code": "UnsupportedSyntax",
  "message": "expected Semicolon, got Some(Var) at 203..206",
  "span_start": 203,
  "span_end": 206,
  "line": 14,
  "column": 14,
  "feature_label": "parser-syntax",
  "error_type": "parser-or-frontend-unsupported"
}
```

Source context:

```text
11 | var x3 = () => { } // error
12 | 
13 | // var then class
14 | var x4 = 1; // error
15 | class x4 { } // error
16 | 
17 | var x4a = 1; // error
```

Visible symbols before failure:

```json
[
  {
    "kind": "binding",
    "name": "then",
    "line": 2,
    "column": 4
  },
  {
    "kind": "binding",
    "name": "var",
    "line": 2,
    "column": 13
  },
  {
    "kind": "binding",
    "name": "x1",
    "line": 4,
    "column": 1,
    "initializer": "2"
  },
  {
    "kind": "binding",
    "name": "then",
    "line": 6,
    "column": 4
  },
  {
    "kind": "binding",
    "name": "x2",
    "line": 7,
    "column": 1,
    "initializer": "1"
  },
  {
    "kind": "function",
    "name": "x2",
    "line": 8,
    "column": 1,
    "params": ""
  },
  {
    "kind": "binding",
    "name": "x3",
    "line": 10,
    "column": 1,
    "initializer": "1"
  },
  {
    "kind": "binding",
    "name": "x3",
    "line": 11,
    "column": 1,
    "initializer": "() => { } // error"
  },
  {
    "kind": "binding",
    "name": "then",
    "line": 13,
    "column": 4
  },
  {
    "kind": "class",
    "name": "var",
    "line": 13,
    "column": 13
  },
  {
    "kind": "binding",
    "name": "x4",
    "line": 14,
    "column": 1,
    "initializer": "1"
  }
]
```

Duplicate candidates:

```json
[
  {
    "state": "open",
    "path": "issues/open/442-implement-parser-syntax.md",
    "title": "Implement parser syntax extensions",
    "reason": "same feature label, title overlap"
  },
  {
    "state": "open",
    "path": "issues/open/464-implement-FunctionDeclaration-parser-syntax.md",
    "title": "Implement Functiondeclaration Parser Syntax",
    "reason": "same feature label, title overlap"
  },
  {
    "state": "open",
    "path": "issues/open/550-implement-FunctionDeclaration-parser-syntax.md",
    "title": "Implement Functiondeclaration Parser Syntax",
    "reason": "same feature label, title overlap"
  },
  {
    "state": "open",
    "path": "issues/open/663-implement-arrayAssignmentTest-parser-syntax.md",
    "title": "Implement Arrayassignmenttest Parser Syntax",
    "reason": "same feature label, title overlap"
  },
  {
    "state": "open",
    "path": "issues/open/734-implement-assignmentCompatability-parser-syntax.md",
    "title": "Implement Assignmentcompatability Parser Syntax",
    "reason": "same feature label, title overlap"
  },
  {
    "state": "open",
    "path": "issues/open/753-implement-asyncFunctionReturnType-parser-syntax.md",
    "title": "Implement Asyncfunctionreturntype Parser Syntax",
    "reason": "same feature label, title overlap"
  },
  {
    "state": "open",
    "path": "issues/open/767-implement-augmentedTypesEnum-parser-syntax.md",
    "title": "Implement Augmentedtypesenum Parser Syntax",
    "reason": "same feature label, title overlap"
  },
  {
    "state": "done",
    "path": "issues/open/059-implement-parser-syntax-extensions.md",
    "title": "Implement parser syntax extensions for TypeScript and advanced JS",
    "reason": "same feature label, title overlap"
  },
  {
    "state": "done",
    "path": "issues/open/065-implement-parser-syntax.md",
    "title": "Implement parser syntax extensions",
    "reason": "same feature label, title overlap"
  },
  {
    "state": "done",
    "path": "issues/open/065a-merge-duplicate-parser-syntax-issue-into-059.md",
    "title": "Merge duplicate parser syntax issue into 059",
    "reason": "same feature label, title overlap"
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
        kind: Var,
        span: Span {
            start: 37,
            end: 40,
        },
    },
    SpannedToken {
        kind: Ident(
            "x1",
        ),
        span: Span {
            start: 41,
            end: 43,
        },
    },
    SpannedToken {
        kind: Equal,
        span: Span {
            start: 44,
            end: 45,
        },
    },
    SpannedToken {
        kind: Number(
            1,
        ),
        span: Span {
            start: 46,
            end: 47,
        },
    },
    SpannedToken {
        kind: Semicolon,
        span: Span {
            start: 47,
            end: 48,
        },
    },
    SpannedToken {
        kind: Var,
        span: Span {
            start: 50,
            end: 53,
        },
    },
    SpannedToken {
        kind: Ident(
            "x1",
        ),
        span: Span {
            start: 54,
            end: 56,
        },
    },
    SpannedToken {
        kind: Equal,
        span: Span {
            start: 57,
            end: 58,
        },
    },
    SpannedToken {
        kind: Number(
            2,
        ),
        span: Span {
            start: 59,
            end: 60,
        },
    },
    SpannedToken {
        kind: Semicolon,
        span: Span {
            start: 60,
            end: 61,
        },
    },
    SpannedToken {
        kind: Var,
        span: Span {
            start: 87,
            end: 90,
        },
    },
    SpannedToken {
        kind: Ident(
            "x2",
        ),
        span: Span {
            start: 91,
            end: 93,
        },
    },
    SpannedToken {
        kind: Equal,
        span: Span {
            start: 94,
            end: 95,
        },
    },
    SpannedToken {
        kind: Number(
            1,
        ),
        span: Span {
            start: 96,
            end: 97,
        },
    },
    SpannedToken {
        kind: Semicolon,
        span: Span {
            start: 97,
            end: 98,
        },
    },
    SpannedToken {
        kind: Function,
        span: Span {
            start: 109,
            end: 117,
        },
    },
    SpannedToken {
        kind: Ident(
            "x2",
        ),
        span: Span {
            start: 118,
            end: 120,
        },
    },
    SpannedToken {
        kind: LeftParen,
        span: Span {
            start: 120,
            end: 121,
        },
    },
    SpannedToken {
        kind: RightParen,
        span: Span {
            start: 121,
            end: 122,
        },
    },
    SpannedToken {
        kind: LeftBrace,
        span: Span {
            start: 123,
            end: 124,
        },
    },
    SpannedToken {
        kind: RightBrace,
        span: Span {
            start: 125,
            end: 126,
        },
    },
    SpannedToken {
        kind: Var,
        span: Span {
            start: 139,
            end: 142,
        },
    },
    SpannedToken {
        kind: Ident(
            "x3",
        ),
        span: Span {
            start: 143,
            end: 145,
        },
    },
    SpannedToken {
        kind: Equal,
        span: Span {
            start: 146,
            end: 147,
        },
    },
    SpannedToken {
        kind: Number(
            1,
        ),
        span: Span {
            start: 148,
            end: 149,
        },
    },
    SpannedToken {
        kind: Semicolon,
        span: Span {
            start: 149,
            end: 150,
```

#### ast

- ok: `False`
- truncated: `False`

```text
error: [UnsupportedSyntax] expected Semicolon, got Some(Var) at 203..206
```

#### resolved

- ok: `False`
- truncated: `False`

```text
error: [UnsupportedSyntax] expected Semicolon, got Some(Var) at 203..206
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
        "code": 2300,
        "category": "Error",
        "message": "Duplicate identifier 'x2'.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/augmentedTypesVar.ts",
        "start": 91,
        "length": 2,
        "line": 7,
        "character": 5
      },
      {
        "code": 2300,
        "category": "Error",
        "message": "Duplicate identifier 'x2'.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/augmentedTypesVar.ts",
        "start": 118,
        "length": 2,
        "line": 8,
        "character": 10
      },
      {
        "code": 2403,
        "category": "Error",
        "message": "Subsequent variable declarations must have the same type.  Variable 'x3' must be of type 'number', but here has type '() => void'.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/augmentedTypesVar.ts",
        "start": 157,
        "length": 2,
        "line": 11,
        "character": 5
      },
      {
        "code": 2300,
        "category": "Error",
        "message": "Duplicate identifier 'x4'.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/augmentedTypesVar.ts",
        "start": 207,
        "length": 2,
        "line": 14,
        "character": 5
      },
      {
        "code": 2300,
        "category": "Error",
        "message": "Duplicate identifier 'x4'.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/augmentedTypesVar.ts",
        "start": 231,
        "length": 2,
        "line": 15,
        "character": 7
      },
      {
        "code": 2300,
        "category": "Error",
        "message": "Duplicate identifier 'x4a'.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/augmentedTypesVar.ts",
        "start": 254,
        "length": 3,
        "line": 17,
        "character": 5
      },
      {
        "code": 2300,
        "category": "Error",
        "message": "Duplicate identifier 'x4a'.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/augmentedTypesVar.ts",
        "start": 279,
        "length": 3,
        "line": 18,
        "character": 7
      },
      {
        "code": 2567,
        "category": "Error",
        "message": "Enum declarations can only merge with namespace or other enum declarations.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/augmentedTypesVar.ts",
        "start": 338,
        "length": 2,
        "line": 21,
        "character": 5
      },
      {
        "code": 2567,
        "category": "Error",
        "message": "Enum declarations can only merge with namespace or other enum declarations.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/augmentedTypesVar.ts",
        "start": 352,
        "length": 2,
        "line": 22,
        "character": 6
      },
      {
        "code": 2300,
        "category": "Error",
        "message": "Duplicate identifier 'x6a'.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/augmentedTypesVar.ts",
        "start": 461,
        "length": 3,
        "line": 28,
        "character": 5
      },
      {
        "code": 2300,
        "category": "Error",
        "message": "Duplicate identifier 'x6a'.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/augmentedTypesVar.ts",
        "start": 490,
        "length": 3,
        "line": 29,
        "character": 11
      },
      {
        "code": 2300,
        "category": "Error",
        "message": "Duplicate identifier 'x6b'.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/augmentedTypesVar.ts",
        "start": 544,
        "length": 3,
        "line": 31,
        "character": 5
      },
      {
        "code": 2300,
        "category": "Error",
        "message": "Duplicate identifier 'x6b'.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/augmentedTypesVar.ts",
        "start": 573,
        "length": 3,
        "line": 32,
        "character": 11
      }
    ],
    "hints": [
      {
        "kind": "binding",
        "typeText": "number",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/augmentedTypesVar.ts",
        "start": 41,
        "length": 2,
        "line": 3,
        "character": 5,
        "name": "x1"
      },
      {
        "kind": "binding",
        "typeText": "number",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/augmentedTypesVar.ts",
        "start": 54,
        "length": 2,
        "line": 4,
        "character": 5,
        "name": "x1"
      },
      {
        "kind": "binding",
        "typeText": "number",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/augmentedTypesVar.ts",
        "start": 91,
        "length": 2,
        "line": 7,
        "character": 5,
        "name": "x2"
      },
      {
        "kind": "function",
        "typeText": "void",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/augmentedTypesVar.ts",
        "start": 118,
        "length": 2,
        "line": 8,
        "character": 10,
        "name": "x2"
      },
      {
        "kind": "binding",
        "typeText": "number",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/augmentedTypesVar.ts",
        "start": 143,
        "length": 2,
        "line": 10,
        "character": 5,
        "name": "x3"
      },
      {
        "kind": "binding",
        "typeText": "number",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/augmentedTypesVar.ts",
        "start": 157,
        "length": 2,
        "line": 11,
        "character": 5,
        "name": "x3"
      },
      {
        "kind": "binding",
        "typeText": "number",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/augmentedTypesVar.ts",
        "start": 207,
        "length": 2,
        "line": 14,
        "character": 5,
        "name": "x4"
      },
      {
        "kind": "binding",
        "typeText": "number",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/augmentedTypesVar.ts",
        "start": 254,
        "length": 3,
        "line": 17,
        "character": 5,
        "name": "x4a"
      },
      {
        "kind": "binding",
        "typeText": "number",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/augmentedTypesVar.ts",
        "start": 338,
        "length": 2,
        "line": 21,
        "character": 5,
        "name": "x5"
      },
      {
        "kind": "binding",
        "typeText": "number",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/augmentedTypesVar.ts",
        "start": 399,
        "length": 2,
        "line": 25,
        "character": 5,
        "name": "x6"
      },
      {
        "kind": "binding",
        "typeText": "number",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/augmentedTypesVar.ts",
        "start": 461,
        "length": 3,
        "line": 28,
        "character": 5,
        "name": "x6a"
      },
      {
        "kind": "binding",
        "typeText": "number",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/augmentedTypesVar.ts",
        "start": 500,
        "length": 1,
        "line": 29,
        "character": 21,
        "name": "y"
      },
      {
        "kind": "binding",
        "typeText": "number",
        "file": "/home/wogikaze/wgkz/ts2wasm/re
```

Stack trace:

```text
error: [UnsupportedSyntax] expected Semicolon, got Some(Var) at 203..206
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
