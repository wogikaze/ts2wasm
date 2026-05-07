---
id: 769
title: "Implement Augmentedtypesfunction"
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

## Summary

Triage augmentedTypesFunction across 1 failing reference test cases and split this bucket into implementation-ready child issues.

## Problem

Closed this generated bucket as superseded by
`issues/open/5307-report-var-function-duplicate-identifier-diagnostics.md`.

Fresh triage shows the old parser-syntax blocker is stale. The current first
blocker is a resolver duplicate-identifier boundary for `function y1() { }`
followed by `var y1 = 1;`, which belongs with issue 5307's var/function
duplicate diagnostic work.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/augmentedTypesFunction.ts
```

Coverage window:

```sh
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/augmentedTypesFunction.ts --detail
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
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/augmentedTypesFunction.ts --detail --no-dashboard-data
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/augmentedTypesFunction.ts
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

- `reference/typescript/tests/cases/compiler/augmentedTypesFunction.ts`

## Duplicate detection

Superseded by `issues/open/5307-report-var-function-duplicate-identifier-diagnostics.md`.

Evidence:

- Current diagnostic: `DuplicateLocal`
- Current message: top-level lexical binding `y1` conflicts with function
  declaration at `70..81`
- Current source:

```text
function y1() { } // error
var y1 = 1; // error
```

- TypeScript oracle reports duplicate identifier TS2300 diagnostics at both
  `y1` identifiers.
- Related issue `issues/done/5199-report-function-overload-list-class-merge-diagnostics.md`
  owns the later class/function merge diagnostics in this same reference file
  family, but not this first function/var duplicate-identifier blocker.

Current coverage:

```text
executed=1
build_pass=0
unsupported=1
unsupported_diagcodes=DuplicateLocal:1
unsupported_features=duplicate-local:1
semantic_enabled=0
reference/typescript/tests/cases/compiler/augmentedTypesFunction.ts: DuplicateLocal: duplicate-local
```

## Current smart triage

### Smart triage: Triage duplicate local: augmentedTypesFunction

- Issue class: `triage-needed`
- Feature label: `duplicate-local`
- Diagnostic: `DuplicateLocal` / `compiler-diagnostic`
- Path: `reference/typescript/tests/cases/compiler/augmentedTypesFunction.ts`

Reproduction:

```sh
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/augmentedTypesFunction.ts
```

Failure location:

```json
{
  "code": "DuplicateLocal",
  "message": "top-level lexical binding `y1` conflicts with function declaration at 70..81",
  "span_start": 70,
  "span_end": 81,
  "line": 4,
  "column": 4,
  "feature_label": "duplicate-local",
  "error_type": "compiler-diagnostic"
}
```

Source context:

```text
1 | // @target: es2015
2 | // function then var
3 | function y1() { } // error
4 | var y1 = 1; // error
5 |
6 | // function then function
7 | function y2() { } // error
```

## Stale generated smart triage

### Smart triage: Triage parser syntax: augmentedTypesFunction

- Issue class: `triage-needed`
- Feature label: `parser-syntax`
- Diagnostic: `UnsupportedSyntax` / `parser-or-frontend-unsupported`
- Path: `reference/typescript/tests/cases/compiler/augmentedTypesFunction.ts`

Reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/augmentedTypesFunction.ts
```

Source overview:

```json
{
  "suite": "tsc",
  "bytes": 894,
  "lines": 39,
  "extension": ".ts",
  "first_code_line": "function y1() { } // error"
}
```

Failure location:

```json
{
  "code": "UnsupportedSyntax",
  "message": "expected Semicolon, got Some(Function) at 265..273",
  "span_start": 265,
  "span_end": 273,
  "line": 14,
  "column": 14,
  "feature_label": "parser-syntax",
  "error_type": "parser-or-frontend-unsupported"
}
```

Source context:

```text
11 | var y2a = () => { } // error
12 | 
13 | // function then class
14 | function y3() { } // error
15 | class y3 { } // error
16 | 
17 | function y3a() { } // error
```

Visible symbols before failure:

```json
[
  {
    "kind": "binding",
    "name": "function",
    "line": 2,
    "column": 18
  },
  {
    "kind": "function",
    "name": "y1",
    "line": 3,
    "column": 1,
    "params": ""
  },
  {
    "kind": "binding",
    "name": "y1",
    "line": 4,
    "column": 1,
    "initializer": "1"
  },
  {
    "kind": "function",
    "name": "y2",
    "line": 7,
    "column": 1,
    "params": ""
  },
  {
    "kind": "function",
    "name": "y2",
    "line": 8,
    "column": 1,
    "params": ""
  },
  {
    "kind": "function",
    "name": "y2a",
    "line": 10,
    "column": 1,
    "params": ""
  },
  {
    "kind": "binding",
    "name": "y2a",
    "line": 11,
    "column": 1,
    "initializer": "() => { } // error"
  },
  {
    "kind": "class",
    "name": "function",
    "line": 13,
    "column": 18
  },
  {
    "kind": "function",
    "name": "y3",
    "line": 14,
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
    "path": "issues/open/442-implement-parser-syntax.md",
    "title": "Implement parser syntax extensions",
    "reason": "same feature label, title overlap"
  },
  {
    "state": "open",
    "path": "issues/done/464-implement-FunctionDeclaration-parser-syntax.md",
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
    "path": "issues/done/767-implement-augmentedTypesEnum-parser-syntax.md",
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
    "path": "issues/done/065-implement-parser-syntax.md",
    "title": "Implement parser syntax extensions",
    "reason": "same feature label, title overlap"
  },
  {
    "state": "done",
    "path": "issues/done/065a-merge-duplicate-parser-syntax-issue-into-059.md",
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
        kind: Function,
        span: Span {
            start: 42,
            end: 50,
        },
    },
    SpannedToken {
        kind: Ident(
            "y1",
        ),
        span: Span {
            start: 51,
            end: 53,
        },
    },
    SpannedToken {
        kind: LeftParen,
        span: Span {
            start: 53,
            end: 54,
        },
    },
    SpannedToken {
        kind: RightParen,
        span: Span {
            start: 54,
            end: 55,
        },
    },
    SpannedToken {
        kind: LeftBrace,
        span: Span {
            start: 56,
            end: 57,
        },
    },
    SpannedToken {
        kind: RightBrace,
        span: Span {
            start: 58,
            end: 59,
        },
    },
    SpannedToken {
        kind: Var,
        span: Span {
            start: 70,
            end: 73,
        },
    },
    SpannedToken {
        kind: Ident(
            "y1",
        ),
        span: Span {
            start: 74,
            end: 76,
        },
    },
    SpannedToken {
        kind: Equal,
        span: Span {
            start: 77,
            end: 78,
        },
    },
    SpannedToken {
        kind: Number(
            1,
        ),
        span: Span {
            start: 79,
            end: 80,
        },
    },
    SpannedToken {
        kind: Semicolon,
        span: Span {
            start: 80,
            end: 81,
        },
    },
    SpannedToken {
        kind: Function,
        span: Span {
            start: 121,
            end: 129,
        },
    },
    SpannedToken {
        kind: Ident(
            "y2",
        ),
        span: Span {
            start: 130,
            end: 132,
        },
    },
    SpannedToken {
        kind: LeftParen,
        span: Span {
            start: 132,
            end: 133,
        },
    },
    SpannedToken {
        kind: RightParen,
        span: Span {
            start: 133,
            end: 134,
        },
    },
    SpannedToken {
        kind: LeftBrace,
        span: Span {
            start: 135,
            end: 136,
        },
    },
    SpannedToken {
        kind: RightBrace,
        span: Span {
            start: 137,
            end: 138,
        },
    },
    SpannedToken {
        kind: Function,
        span: Span {
            start: 149,
            end: 157,
        },
    },
    SpannedToken {
        kind: Ident(
            "y2",
        ),
        span: Span {
            start: 158,
            end: 160,
        },
    },
    SpannedToken {
        kind: LeftParen,
        span: Span {
            start: 160,
            end: 161,
        },
    },
    SpannedToken {
        kind: RightParen,
        span: Span {
            start: 161,
            end: 162,
        },
    },
    SpannedToken {
        kind: LeftBrace,
        span: Span {
            start: 163,
            end: 164,
        },
    },
    SpannedToken {
        kind: RightBrace,
        span: Span {
            start: 165,
            end: 166,
        },
    },
    SpannedToken {
        kind: Function,
        span: Span {
            start: 179,
            end: 187,
        },
    },
    SpannedToken {
        kind: Ident(
            "y2a",
        ),
        span: Span {
            start: 188,
            end: 191,
        },
    },
    SpannedToken {
        kind: LeftParen,
        span: Span {
            start: 191,
            end: 192,
        },
    },
    SpannedT
```

#### ast

- ok: `False`
- truncated: `False`

```text
error: [UnsupportedSyntax] expected Semicolon, got Some(Function) at 265..273
```

#### resolved

- ok: `False`
- truncated: `False`

```text
error: [UnsupportedSyntax] expected Semicolon, got Some(Function) at 265..273
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
        "message": "Duplicate identifier 'y1'.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/augmentedTypesFunction.ts",
        "start": 51,
        "length": 2,
        "line": 3,
        "character": 10
      },
      {
        "code": 2300,
        "category": "Error",
        "message": "Duplicate identifier 'y1'.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/augmentedTypesFunction.ts",
        "start": 74,
        "length": 2,
        "line": 4,
        "character": 5
      },
      {
        "code": 2393,
        "category": "Error",
        "message": "Duplicate function implementation.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/augmentedTypesFunction.ts",
        "start": 130,
        "length": 2,
        "line": 7,
        "character": 10
      },
      {
        "code": 2393,
        "category": "Error",
        "message": "Duplicate function implementation.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/augmentedTypesFunction.ts",
        "start": 158,
        "length": 2,
        "line": 8,
        "character": 10
      },
      {
        "code": 2300,
        "category": "Error",
        "message": "Duplicate identifier 'y2a'.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/augmentedTypesFunction.ts",
        "start": 188,
        "length": 3,
        "line": 10,
        "character": 10
      },
      {
        "code": 2300,
        "category": "Error",
        "message": "Duplicate identifier 'y2a'.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/augmentedTypesFunction.ts",
        "start": 213,
        "length": 3,
        "line": 11,
        "character": 5
      },
      {
        "code": 2814,
        "category": "Error",
        "message": "Function with bodies can only merge with classes that are ambient.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/augmentedTypesFunction.ts",
        "start": 274,
        "length": 2,
        "line": 14,
        "character": 10
      },
      {
        "code": 2813,
        "category": "Error",
        "message": "Class declaration cannot implement overload list for 'y3'.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/augmentedTypesFunction.ts",
        "start": 299,
        "length": 2,
        "line": 15,
        "character": 7
      },
      {
        "code": 2814,
        "category": "Error",
        "message": "Function with bodies can only merge with classes that are ambient.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/augmentedTypesFunction.ts",
        "start": 327,
        "length": 3,
        "line": 17,
        "character": 10
      },
      {
        "code": 2813,
        "category": "Error",
        "message": "Class declaration cannot implement overload list for 'y3a'.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/augmentedTypesFunction.ts",
        "start": 353,
        "length": 3,
        "line": 18,
        "character": 7
      },
      {
        "code": 2567,
        "category": "Error",
        "message": "Enum declarations can only merge with namespace or other enum declarations.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/augmentedTypesFunction.ts",
        "start": 422,
        "length": 2,
        "line": 21,
        "character": 10
      },
      {
        "code": 2567,
        "category": "Error",
        "message": "Enum declarations can only merge with namespace or other enum declarations.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/augmentedTypesFunction.ts",
        "start": 446,
        "length": 2,
        "line": 22,
        "character": 6
      }
    ],
    "hints": [
      {
        "kind": "function",
        "typeText": "void",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/augmentedTypesFunction.ts",
        "start": 51,
        "length": 2,
        "line": 3,
        "character": 10,
        "name": "y1"
      },
      {
        "kind": "binding",
        "typeText": "number",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/augmentedTypesFunction.ts",
        "start": 74,
        "length": 2,
        "line": 4,
        "character": 5,
        "name": "y1"
      },
      {
        "kind": "function",
        "typeText": "void",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/augmentedTypesFunction.ts",
        "start": 130,
        "length": 2,
        "line": 7,
        "character": 10,
        "name": "y2"
      },
      {
        "kind": "function",
        "typeText": "void",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/augmentedTypesFunction.ts",
        "start": 158,
        "length": 2,
        "line": 8,
        "character": 10,
        "name": "y2"
      },
      {
        "kind": "function",
        "typeText": "void",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/augmentedTypesFunction.ts",
        "start": 188,
        "length": 3,
        "line": 10,
        "character": 10,
        "name": "y2a"
      },
      {
        "kind": "binding",
        "typeText": "() => void",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/augmentedTypesFunction.ts",
        "start": 213,
        "length": 3,
        "line": 11,
        "character": 5,
        "name": "y2a"
      },
      {
        "kind": "function",
        "typeText": "void",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/augmentedTypesFunction.ts",
        "start": 274,
        "length": 2,
        "line": 14,
        "character": 10,
        "name": "y3"
      },
      {
        "kind": "function",
        "typeText": "void",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/augmentedTypesFunction.ts",
        "start": 327,
        "length": 3,
        "line": 17,
        "character": 10,
        "name": "y3a"
      },
      {
        "kind": "function",
        "typeText": "void",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/augmentedTypesFunction.ts",
        "start": 422,
        "length": 2,
        "line": 21,
        "character": 10,
        "name": "y4"
      },
      {
        "kind": "function",
        "typeText": "void",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/augmentedTypesFunction.ts",
        "start": 512,
        "length": 2,
        "line": 25,
        "character": 10,
        "name": "y5"
      },
      {
        "kind": "function",
        "typeText": "void",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/augmentedTypesFunction.ts",
        "start": 590,
        "length": 3,
        "line": 28,
        "character": 10,
        "name": "y5a"
      },
      {
        "kind": "binding",
        "typeText": "number",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/augmentedTypesFunction.ts",
        "start": 621,
        "length": 1,
        "line": 29,
        "character": 21,
        "name": "y"
      },
      {
        "kind": "function",
        "typeText": "void",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/augmentedTypesFunction.ts",
        "start": 664,
        "length": 3,
        "line": 31,
```

Stack trace:

```text
error: [UnsupportedSyntax] expected Semicolon, got Some(Function) at 265..273
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
