---
id: 770
title: "Implement Augmentedtypesinterface"
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

## Summary

Closed this generated parser-syntax bucket as stale: fresh triage and focused
coverage show the representative reference file now builds successfully.

## Problem

Fresh reference evidence no longer shows a parser-syntax blocker for
`augmentedTypesInterface.ts`.

Problem: the issue still sat in the blocked queue even though the representative
case is now a build pass.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/augmentedTypesInterface.ts
```

Coverage window:

```sh
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/augmentedTypesInterface.ts --detail
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
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/augmentedTypesInterface.ts --detail --no-dashboard-data
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/augmentedTypesInterface.ts
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

- `reference/typescript/tests/cases/compiler/augmentedTypesInterface.ts`

## Duplicate detection

No implementation child needed. Fresh triage reports `BuildPass`; focused
coverage reports `build_pass=1`, `unsupported=0`, and `blocked=0`.

TypeScript oracle reports interface/enum merge diagnostics TS2567, but semantic
parity is outside this stale parser-syntax bucket closure.

Current coverage:

```text
executed=1
build_pass=1
semantic_pass=0
unsupported=0
blocked=0
unsupported_diagcodes=
unsupported_features=
semantic_enabled=0
reference/typescript/tests/cases/compiler/augmentedTypesInterface.ts: build_pass
```

## Current smart triage

### Smart triage: Build pass: augmentedTypesInterface

- Issue class: `none`
- Feature label: `build-pass`
- Diagnostic: `BuildPass` / `pass`
- Path: `reference/typescript/tests/cases/compiler/augmentedTypesInterface.ts`

Reproduction:

```sh
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/augmentedTypesInterface.ts
```

Failure location:

```json
{
  "code": "BuildPass",
  "message": "ts2wasm build succeeded",
  "span_start": null,
  "span_end": null,
  "line": null,
  "column": null,
  "feature_label": "build-pass",
  "error_type": "pass"
}
```

## Stale generated smart triage

### Smart triage: Triage parser syntax: augmentedTypesInterface

- Issue class: `triage-needed`
- Feature label: `parser-syntax`
- Diagnostic: `UnsupportedTypeScriptSyntax` / `unsupported-feature-boundary`
- Path: `reference/typescript/tests/cases/compiler/augmentedTypesInterface.ts`

Reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/augmentedTypesInterface.ts
```

Source overview:

```json
{
  "suite": "tsc",
  "bytes": 416,
  "lines": 34,
  "extension": ".ts",
  "first_code_line": "interface i {"
}
```

Failure location:

```json
{
  "code": "UnsupportedTypeScriptSyntax",
  "message": "TypeScript enum declarations require an explicit frontend transform before runtime lowering at 319..323",
  "span_start": 319,
  "span_end": 323,
  "line": 28,
  "column": 1,
  "feature_label": "parser-syntax",
  "error_type": "unsupported-feature-boundary"
}
```

Source context:

```text
25 |     foo(): void;
26 | }
27 | enum i3 { One }; // error
28 | 
29 | // interface then import
30 | interface i4 {
31 |     foo(): void;
```

Visible symbols before failure:

```json
[
  {
    "kind": "class",
    "name": "interface",
    "line": 12,
    "column": 19
  },
  {
    "kind": "class",
    "name": "i2",
    "line": 17,
    "column": 1
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
            "interface",
        ),
        span: Span {
            start: 51,
            end: 60,
        },
    },
    SpannedToken {
        kind: Ident(
            "i",
        ),
        span: Span {
            start: 61,
            end: 62,
        },
    },
    SpannedToken {
        kind: LeftBrace,
        span: Span {
            start: 63,
            end: 64,
        },
    },
    SpannedToken {
        kind: Ident(
            "foo",
        ),
        span: Span {
            start: 70,
            end: 73,
        },
    },
    SpannedToken {
        kind: LeftParen,
        span: Span {
            start: 73,
            end: 74,
        },
    },
    SpannedToken {
        kind: RightParen,
        span: Span {
            start: 74,
            end: 75,
        },
    },
    SpannedToken {
        kind: Colon,
        span: Span {
            start: 75,
            end: 76,
        },
    },
    SpannedToken {
        kind: Void,
        span: Span {
            start: 77,
            end: 81,
        },
    },
    SpannedToken {
        kind: Semicolon,
        span: Span {
            start: 81,
            end: 82,
        },
    },
    SpannedToken {
        kind: RightBrace,
        span: Span {
            start: 84,
            end: 85,
        },
    },
    SpannedToken {
        kind: Ident(
            "interface",
        ),
        span: Span {
            start: 89,
            end: 98,
        },
    },
    SpannedToken {
        kind: Ident(
            "i",
        ),
        span: Span {
            start: 99,
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
        kind: Ident(
            "bar",
        ),
        span: Span {
            start: 108,
            end: 111,
        },
    },
    SpannedToken {
        kind: LeftParen,
        span: Span {
            start: 111,
            end: 112,
        },
    },
    SpannedToken {
        kind: RightParen,
        span: Span {
            start: 112,
            end: 113,
        },
    },
    SpannedToken {
        kind: Colon,
        span: Span {
            start: 113,
            end: 114,
        },
    },
    SpannedToken {
        kind: Ident(
            "number",
        ),
        span: Span {
            start: 115,
            end: 121,
        },
    },
    SpannedToken {
        kind: Semicolon,
        span: Span {
            start: 121,
            end: 122,
        },
    },
    SpannedToken {
        kind: RightBrace,
        span: Span {
            start: 124,
            end: 125,
        },
    },
    SpannedToken {
        kind: Ident(
            "interface",
        ),
        span: Span {
            start: 154,
            end: 163,
        },
    },
    SpannedToken {
        kind: Ident(
            "i2",
        ),
        span: Span {
            start: 164,
            end: 166,
        },
    },
    SpannedToken {
        kind: LeftBrace,
        span: Span {
            start: 167,
            end: 168,
        },
    },
    SpannedToken {
        kind: Ident(
            "foo",
        ),
        span: Span {
            start: 174,
            end: 177,
        },
    },
    SpannedToken {
        kind: LeftParen,
        span: Span {
            start: 177,
            end: 178,
        },
    },
    SpannedToken
```

#### ast

- ok: `False`
- truncated: `False`

```text
error: [UnsupportedTypeScriptSyntax] TypeScript enum declarations require an explicit frontend transform before runtime lowering at 319..323
```

#### resolved

- ok: `False`
- truncated: `False`

```text
error: [UnsupportedTypeScriptSyntax] TypeScript enum declarations require an explicit frontend transform before runtime lowering at 319..323
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
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/augmentedTypesInterface.ts",
        "start": 283,
        "length": 2,
        "line": 24,
        "character": 11
      },
      {
        "code": 2567,
        "category": "Error",
        "message": "Enum declarations can only merge with namespace or other enum declarations.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/augmentedTypesInterface.ts",
        "start": 324,
        "length": 2,
        "line": 27,
        "character": 6
      }
    ],
    "hints": [],
    "typescriptVersion": "6.0.3"
  },
  "ast": {
    "topLevel": [
      {
        "kind": "InterfaceDeclaration",
        "text": "interface i {\r\n    foo(): void;\r\n}",
        "line": 4,
        "character": 1
      },
      {
        "kind": "InterfaceDeclaration",
        "text": "interface i {\r\n    bar(): number;\r\n}",
        "line": 8,
        "character": 1
      },
      {
        "kind": "InterfaceDeclaration",
        "text": "interface i2 {\r\n    foo(): void;\r\n}",
        "line": 13,
        "character": 1
      },
      {
        "kind": "ClassDeclaration",
        "text": "class i2 {\r\n    bar() {\r\n        return 1;\r\n    }\r\n}",
        "line": 17,
        "character": 1
      },
      {
        "kind": "InterfaceDeclaration",
        "text": "interface i3 { // error\r\n    foo(): void;\r\n}",
        "line": 24,
        "character": 1
      },
      {
        "kind": "EnumDeclaration",
        "text": "enum i3 { One }",
        "line": 27,
        "character": 1
      },
      {
        "kind": "EmptyStatement",
        "text": ";",
        "line": 27,
        "character": 16
      },
      {
        "kind": "InterfaceDeclaration",
        "text": "interface i4 {\r\n    foo(): void;\r\n}",
        "line": 30,
        "character": 1
      }
    ],
    "pathToPosition": [
      {
        "kind": "SourceFile",
        "text": "interface i {\r\n    foo(): void;\r\n}\r\n\r\ninterface i {\r\n    bar(): number;\r\n}\r\n\r\n// interface then class\r\ninterface i2 {\r\n ",
        "line": 4,
        "character": 1
      },
      {
        "kind": "EnumDeclaration",
        "text": "enum i3 { One }",
        "line": 27,
        "character": 1
      }
    ]
  }
}
```

Stack trace:

```text
error: [UnsupportedTypeScriptSyntax] TypeScript enum declarations require an explicit frontend transform before runtime lowering at 319..323
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
