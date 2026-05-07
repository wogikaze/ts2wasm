---
id: 773
title: "Implement Autoasiforstaticsinclassdeclaration"
type: spike
area: frontend/parser
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
> Reason: Batch-migrated to issues/done/ without implementation commits.
> Evidence: Empty completion evidence. No feat/fix commit for #773.

## Summary

Closed this generated parser-syntax bucket as superseded by
`issues/done/5254-parse-asi-between-static-class-fields.md`.

## Problem

Fresh triage confirms the current blocker is the `static x` followed by
later-line `static y` class field ASI parser boundary, already owned by issue
5254.

Problem: the generated bucket remained triage-needed even though the executable
parser work is already tracked by issue 5254.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/autoAsiForStaticsInClassDeclaration.ts
```

Coverage window:

```sh
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/autoAsiForStaticsInClassDeclaration.ts --detail
```

## Desired final state

This generated bucket is closed. Implement from
`issues/done/5254-parse-asi-between-static-class-fields.md`.

## Scope

In scope:

- [x] Inspect the smart triage report below
- [x] Confirm whether existing open/done issues already cover this bucket
- [x] Supersede this generated bucket with the existing implementation-ready issue
- [x] Preserve exact reproduction commands and representative AST/diagnostic evidence

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
- [x] Superseding issue 5254 contains the implementation scope
- [x] Current triage evidence is recorded
- [x] Superseding issue acceptance names this exact reference path and diagnostic change

## Validation

Required commands:

```sh
python scripts/manager.py update-issue-index --check
python scripts/manager.py check-issue-health
python scripts/manager.py check-issue-readiness -- --fail-ready-below 80
```

Impacted commands:

```sh
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/autoAsiForStaticsInClassDeclaration.ts --detail --no-dashboard-data
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/autoAsiForStaticsInClassDeclaration.ts
```

Not run:

- cargo fmt / nextest not run for this metadata-only issue lifecycle closure

## Docs / current-state / issue sync

Final-state docs:

- [x] not affected

Current state:

- [x] not affected

Follow-up issues:

- [x] `issues/done/5254-parse-asi-between-static-class-fields.md`

## Notes

## Affected test files

- `reference/typescript/tests/cases/compiler/autoAsiForStaticsInClassDeclaration.ts`

## Duplicate detection

Superseded by `issues/done/5254-parse-asi-between-static-class-fields.md`.

Evidence:

- Current diagnostic: `UnsupportedSyntax`
- Current message: `expected LeftParen, got Some(Static) at 71..77`
- Current source:

```text
class C {
    static x
    static y
}
```

- Existing issue 5254 scope covers ASI after `static name` before a later-line
  `static` class element for both class declarations and class expressions.
- Existing issue 5254 acceptance already names
  `autoAsiForStaticsInClassDeclaration.ts` and this exact diagnostic.

Current coverage:

```text
executed=1
build_pass=0
unsupported=1
unsupported_diagcodes=UnsupportedSyntax:1
unsupported_features=unknown-unsupported:1
semantic_enabled=0
reference/typescript/tests/cases/compiler/autoAsiForStaticsInClassDeclaration.ts: UnsupportedSyntax: unknown-unsupported
```

## Current smart triage

### Smart triage: Triage parser syntax: autoAsiForStaticsInClassDeclaration

- Issue class: `triage-needed`
- Feature label: `parser-syntax`
- Diagnostic: `UnsupportedSyntax` / `parser-or-frontend-unsupported`
- Path: `reference/typescript/tests/cases/compiler/autoAsiForStaticsInClassDeclaration.ts`

Reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/autoAsiForStaticsInClassDeclaration.ts
```

Source overview:

```json
{
  "suite": "tsc",
  "bytes": 78,
  "lines": 6,
  "extension": ".ts",
  "first_code_line": "﻿// @strict: false"
}
```

Failure location:

```json
{
  "code": "UnsupportedSyntax",
  "message": "expected LeftParen, got Some(Static) at 71..77",
  "span_start": 71,
  "span_end": 77,
  "line": 5,
  "column": 11,
  "feature_label": "parser-syntax",
  "error_type": "parser-or-frontend-unsupported"
}
```

Source context:

```text
2 | // @target: es2015
3 | class C {
4 |     static x
5 |     static y
6 | }
```

Visible symbols before failure:

```json
[
  {
    "kind": "class",
    "name": "C",
    "line": 3,
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
    "path": "issues/done/550-implement-FunctionDeclaration-parser-syntax.md",
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
- truncated: `False`

```text
== tokens ==
[
    SpannedToken {
        kind: Class,
        span: Span {
            start: 42,
            end: 47,
        },
    },
    SpannedToken {
        kind: Ident(
            "C",
        ),
        span: Span {
            start: 48,
            end: 49,
        },
    },
    SpannedToken {
        kind: LeftBrace,
        span: Span {
            start: 50,
            end: 51,
        },
    },
    SpannedToken {
        kind: Static,
        span: Span {
            start: 57,
            end: 63,
        },
    },
    SpannedToken {
        kind: Ident(
            "x",
        ),
        span: Span {
            start: 64,
            end: 65,
        },
    },
    SpannedToken {
        kind: Static,
        span: Span {
            start: 71,
            end: 77,
        },
    },
    SpannedToken {
        kind: Ident(
            "y",
        ),
        span: Span {
            start: 78,
            end: 79,
        },
    },
    SpannedToken {
        kind: RightBrace,
        span: Span {
            start: 81,
            end: 82,
        },
    },
]
```

#### ast

- ok: `False`
- truncated: `False`

```text
error: [UnsupportedSyntax] expected LeftParen, got Some(Static) at 71..77
```

#### resolved

- ok: `False`
- truncated: `False`

```text
error: [UnsupportedSyntax] expected LeftParen, got Some(Static) at 71..77
```

TypeScript/JavaScript oracle:

```json
{
  "ok": true,
  "returncode": 0,
  "typescript": {
    "ok": true,
    "diagnostics": [],
    "hints": [],
    "typescriptVersion": "6.0.3"
  },
  "ast": {
    "topLevel": [
      {
        "kind": "ClassDeclaration",
        "text": "class C {\r\n    static x\r\n    static y\r\n}",
        "line": 3,
        "character": 1
      }
    ],
    "pathToPosition": [
      {
        "kind": "SourceFile",
        "text": "class C {\r\n    static x\r\n    static y\r\n} ",
        "line": 3,
        "character": 1
      },
      {
        "kind": "ClassDeclaration",
        "text": "class C {\r\n    static x\r\n    static y\r\n}",
        "line": 3,
        "character": 1
      },
      {
        "kind": "PropertyDeclaration",
        "text": "static y",
        "line": 5,
        "character": 5
      },
      {
        "kind": "StaticKeyword",
        "text": "static",
        "line": 5,
        "character": 5
      }
    ]
  }
}
```

Stack trace:

```text
error: [UnsupportedSyntax] expected LeftParen, got Some(Static) at 71..77
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
