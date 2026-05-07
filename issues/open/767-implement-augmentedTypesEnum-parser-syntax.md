---
id: 767
title: "Implement Augmentedtypesenum Parser Syntax"
type: spike
area: frontend/syntax
class: blocked
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
> Reason: Batch-migrated to issues/done/ without implementation commits.
> Evidence: Empty completion evidence. No feat/fix commit for #767.

## Summary

Closed this generated parser-syntax bucket as stale: fresh triage and focused
coverage show the representative reference file now builds successfully.

## Problem

Fresh reference evidence no longer shows a parser-syntax blocker for
`augmentedTypesEnum.ts`.

Problem: the issue still sat in the blocked queue even though the representative
case is now a build pass.

## Current failure

Representative reproduction:

```sh
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/augmentedTypesEnum.ts
```

Coverage window:

```sh
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/augmentedTypesEnum.ts --detail --no-dashboard-data
```

## Desired final state

The generated bucket is closed; no child issue is needed for the previous
parser-syntax blocker.

## Scope

In scope:

- [x] Inspect the smart triage report below
- [x] Confirm current evidence
- [x] Close as stale build-pass evidence
- [x] Preserve exact reproduction commands and current coverage evidence

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

- [x] Duplicate candidates below are confirmed as stale
- [x] Current triage command is recorded
- [x] Current coverage result is recorded
- [x] No child issue is needed for a build-pass bucket

## Validation

Required commands:

```sh
python scripts/manager.py update-issue-index --check
python scripts/manager.py check-issue-health
python scripts/manager.py check-issue-readiness -- --fail-ready-below 80
```

Impacted commands:

```sh
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/augmentedTypesEnum.ts --detail --no-dashboard-data
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/augmentedTypesEnum.ts
```

Not run:

- cargo fmt / nextest not run for this metadata-only issue lifecycle closure

## Docs / current-state / issue sync

Final-state docs:

- [x] not affected

Current state:

- [x] not affected

Follow-up issues:

- [x] none

## Notes

## Affected test files

- `reference/typescript/tests/cases/compiler/augmentedTypesEnum.ts`
- `reference/typescript/tests/cases/compiler/augmentedTypesEnum2.ts`

## Duplicate detection

No implementation child needed. Fresh triage reports `BuildPass`; focused
coverage reports `build_pass=1`, `unsupported=0`, and `blocked=0`.

TypeScript oracle reports enum merge and multiple-enum-declaration diagnostics,
but semantic parity is outside this stale parser-syntax bucket closure.

## Smart triage

### Smart triage: Triage parser syntax: augmentedTypesEnum

- Issue class: `triage-needed`
- Feature label: `parser-syntax`
- Diagnostic: `UnsupportedTypeScriptSyntax` / `unsupported-feature-boundary`
- Path: `reference/typescript/tests/cases/compiler/augmentedTypesEnum.ts`

Reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/augmentedTypesEnum.ts
```

Source overview:

```json
{
  "suite": "tsc",
  "bytes": 736,
  "lines": 36,
  "extension": ".ts",
  "first_code_line": "enum e1111 { One } // error"
}
```

Failure location:

```json
{
  "code": "UnsupportedTypeScriptSyntax",
  "message": "TypeScript enum declarations require an explicit frontend transform before runtime lowering at 38..42",
  "span_start": 38,
  "span_end": 42,
  "line": 3,
  "column": 3,
  "feature_label": "parser-syntax",
  "error_type": "unsupported-feature-boundary"
}
```

Source context:

```text
1 | // @target: es2015
2 | // enum then var
3 | enum e1111 { One } // error
4 | var e1111 = 1; // error
5 | 
6 | // enum then function
```

Visible symbols before failure:

```json
[
  {
    "kind": "binding",
    "name": "en",
    "line": 2,
    "column": 14
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
  },
  {
    "state": "done",
    "path": "issues/done/200-implement-parser-syntax.md",
    "title": "Implement parser syntax extensions",
    "reason": "same feature label, title overlap"
  }
]
```

Error-specific suggestions:

- Create a child issue around this exact path and diagnostic before broadening the reference window.

Compiler dumps:

#### tokens

- ok: `True`
- truncated: `True`

```text
== tokens ==
[
    SpannedToken {
        kind: Ident(
            "enum",
        ),
        span: Span {
            start: 38,
            end: 42,
        },
    },
    SpannedToken {
        kind: Ident(
            "e1111",
        ),
        span: Span {
            start: 43,
            end: 48,
        },
    },
    SpannedToken {
        kind: LeftBrace,
        span: Span {
            start: 49,
            end: 50,
        },
    },
    SpannedToken {
        kind: Ident(
            "One",
        ),
        span: Span {
            start: 51,
            end: 54,
        },
    },
    SpannedToken {
        kind: RightBrace,
        span: Span {
            start: 55,
            end: 56,
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
            "e1111",
        ),
        span: Span {
            start: 71,
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
        kind: Ident(
            "enum",
        ),
        span: Span {
            start: 117,
            end: 121,
        },
    },
    SpannedToken {
        kind: Ident(
            "e2",
        ),
        span: Span {
            start: 122,
            end: 124,
        },
    },
    SpannedToken {
        kind: LeftBrace,
        span: Span {
            start: 125,
            end: 126,
        },
    },
    SpannedToken {
        kind: Ident(
            "One",
        ),
        span: Span {
            start: 127,
            end: 130,
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
        kind: Function,
        span: Span {
            start: 143,
            end: 151,
        },
    },
    SpannedToken {
        kind: Ident(
            "e2",
        ),
        span: Span {
            start: 152,
            end: 154,
        },
    },
    SpannedToken {
        kind: LeftParen,
        span: Span {
            start: 154,
            end: 155,
        },
    },
    SpannedToken {
        kind: RightParen,
        span: Span {
            start: 155,
            end: 156,
        },
    },
    SpannedToken {
        kind: LeftBrace,
        span: Span {
            start: 157,
            end: 158,
        },
    },
    SpannedToken {
        kind: RightBrace,
        span: Span {
            start: 159,
            end: 160,
        },
    },
    SpannedToken {
        kind: Ident(
            "enum",
        ),
        span: Span {
            start: 173,
            end: 177,
        },
    },
    SpannedToken {
        kind: Ident(
            "e3",
        ),
        span: Span {
            start: 178,
            end: 180,
        },
    },
    SpannedToken {
        kind: LeftBrace,
        span: Span {
            start: 181,
            end: 182,
        },
    },
    SpannedToken {
        kind: Ident(
            "One",
        ),
        span: Span {
            start: 183,
            en
```

#### ast

- ok: `False`
- truncated: `False`

```text
error: [UnsupportedTypeScriptSyntax] TypeScript enum declarations require an explicit frontend transform before runtime lowering at 38..42
```

#### resolved

- ok: `False`
- truncated: `False`

```text
error: [UnsupportedTypeScriptSyntax] TypeScript enum declarations require an explicit frontend transform before runtime lowering at 38..42
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
        "code": 2567,
        "category": "Error",
        "message": "Enum declarations can only merge with namespace or other enum declarations.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/augmentedTypesEnum.ts",
        "start": 43,
        "length": 5,
        "line": 3,
        "character": 6
      },
      {
        "code": 2567,
        "category": "Error",
        "message": "Enum declarations can only merge with namespace or other enum declarations.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/augmentedTypesEnum.ts",
        "start": 71,
        "length": 5,
        "line": 4,
        "character": 5
      },
      {
        "code": 2567,
        "category": "Error",
        "message": "Enum declarations can only merge with namespace or other enum declarations.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/augmentedTypesEnum.ts",
        "start": 122,
        "length": 2,
        "line": 7,
        "character": 6
      },
      {
        "code": 2567,
        "category": "Error",
        "message": "Enum declarations can only merge with namespace or other enum declarations.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/augmentedTypesEnum.ts",
        "start": 152,
        "length": 2,
        "line": 8,
        "character": 10
      },
      {
        "code": 2567,
        "category": "Error",
        "message": "Enum declarations can only merge with namespace or other enum declarations.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/augmentedTypesEnum.ts",
        "start": 178,
        "length": 2,
        "line": 10,
        "character": 6
      },
      {
        "code": 2567,
        "category": "Error",
        "message": "Enum declarations can only merge with namespace or other enum declarations.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/augmentedTypesEnum.ts",
        "start": 203,
        "length": 2,
        "line": 11,
        "character": 5
      },
      {
        "code": 2567,
        "category": "Error",
        "message": "Enum declarations can only merge with namespace or other enum declarations.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/augmentedTypesEnum.ts",
        "start": 255,
        "length": 2,
        "line": 14,
        "character": 6
      },
      {
        "code": 2567,
        "category": "Error",
        "message": "Enum declarations can only merge with namespace or other enum declarations.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/augmentedTypesEnum.ts",
        "start": 282,
        "length": 2,
        "line": 15,
        "character": 7
      },
      {
        "code": 2432,
        "category": "Error",
        "message": "In an enum with multiple declarations, only one declaration can omit an initializer for its first enum element.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/augmentedTypesEnum.ts",
        "start": 364,
        "length": 3,
        "line": 19,
        "character": 11
      },
      {
        "code": 2300,
        "category": "Error",
        "message": "Duplicate identifier 'One'.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/augmentedTypesEnum.ts",
        "start": 393,
        "length": 3,
        "line": 21,
        "character": 12
      },
      {
        "code": 2300,
        "category": "Error",
        "message": "Duplicate identifier 'One'.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/augmentedTypesEnum.ts",
        "start": 420,
        "length": 3,
        "line": 22,
        "character": 12
      },
      {
        "code": 2432,
        "category": "Error",
        "message": "In an enum with multiple declarations, only one declaration can omit an initializer for its first enum element.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/augmentedTypesEnum.ts",
        "start": 420,
        "length": 3,
        "line": 22,
        "character": 12
      }
    ],
    "hints": [
      {
        "kind": "binding",
        "typeText": "number",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/augmentedTypesEnum.ts",
        "start": 71,
        "length": 5,
        "line": 4,
        "character": 5,
        "name": "e1111"
      },
      {
        "kind": "function",
        "typeText": "void",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/augmentedTypesEnum.ts",
        "start": 152,
        "length": 2,
        "line": 8,
        "character": 10,
        "name": "e2"
      },
      {
        "kind": "binding",
        "typeText": "() => void",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/augmentedTypesEnum.ts",
        "start": 203,
        "length": 2,
        "line": 11,
        "character": 5,
        "name": "e3"
      },
      {
        "kind": "binding",
        "typeText": "number",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/augmentedTypesEnum.ts",
        "start": 550,
        "length": 1,
        "line": 29,
        "character": 21,
        "name": "y"
      },
      {
        "kind": "binding",
        "typeText": "number",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/augmentedTypesEnum.ts",
        "start": 626,
        "length": 1,
        "line": 32,
        "character": 28,
        "name": "y"
      }
    ],
    "typescriptVersion": "6.0.3"
  },
  "ast": {
    "topLevel": [
      {
        "kind": "EnumDeclaration",
        "text": "enum e1111 { One }",
        "line": 3,
        "character": 1
      },
      {
        "kind": "FirstStatement",
        "text": "var e1111 = 1;",
        "line": 4,
        "character": 1
      },
      {
        "kind": "EnumDeclaration",
        "text": "enum e2 { One }",
        "line": 7,
        "character": 1
      },
      {
        "kind": "FunctionDeclaration",
        "text": "function e2() { }",
        "line": 8,
        "character": 1
      },
      {
        "kind": "EnumDeclaration",
        "text": "enum e3 { One }",
        "line": 10,
        "character": 1
      },
      {
        "kind": "FirstStatement",
        "text": "var e3 = () => { }",
        "line": 11,
        "character": 1
      },
      {
        "kind": "EnumDeclaration",
        "text": "enum e4 { One }",
        "line": 14,
        "character": 1
      },
      {
        "kind": "ClassDeclaration",
        "text": "class e4 { public foo() { } }",
        "line": 15,
        "character": 1
      },
      {
        "kind": "EnumDeclaration",
        "text": "enum e5 { One }",
        "line": 18,
        "character": 1
      },
      {
        "kind": "EnumDeclaration",
        "text": "enum e5 { Two }",
        "line": 19,
        "character": 1
      },
      {
        "kind": "EnumDeclaration",
        "text": "enum e5a { One }",
        "line": 21,
        "character": 1
      },
      {
        "kind": "EnumDeclaration",
        "text": "enum e5a { One }",
        "line": 22,
        "character": 1
      },
      {
        "kind": "EnumDeclaration",
        "text": "enum e6 { One }",
        "line": 25,
        "character": 1
      },
      {
        "kind": "ModuleDeclaration",
        "text": "namespace e6 { }",
        "line": 26,
        "character": 1
      },
      {
        "kind": "EnumDeclaration",
        "text": "enum e6a { One }",
        "line": 28,
        "character": 1
      },
      {
        "kind": "ModuleDeclarat
```

Stack trace:

```text
error: [UnsupportedTypeScriptSyntax] TypeScript enum declarations require an explicit frontend transform before runtime lowering at 38..42
```

## Completion evidence

Commits:

- this commit

Validation result:

```text
command: env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/augmentedTypesEnum.ts
result: pass; BuildPass, no compiler blocker
date: 2026-05-07

command: env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/augmentedTypesEnum.ts --detail --no-dashboard-data
result: pass; executed=1, build_pass=1, unsupported=0, blocked=0
date: 2026-05-07
```

Remaining risks:

- TypeScript oracle reports enum merge and TS2432 diagnostics; semantic parity is outside this stale parser-syntax blocker closure.
