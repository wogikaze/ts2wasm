---
id: 672
title: "Implement Arraydestructuringinswitch"
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

Triage arrayDestructuringInSwitch across 1 failing reference test cases and split this bucket into implementation-ready child issues.

## Problem

Reference test results show 1 cases fail in directory `arrayDestructuringInSwitch` with diagnostics: import-export. The compiler cannot handle these syntax/semantics, preventing compilation of code in this category.

Problem: arrayDestructuringInSwitch has 1 reference failures and needs smart-triage evidence before implementation starts.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/arrayDestructuringInSwitch1.ts
```

Coverage window:

```sh
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/arrayDestructuringInSwitch1.ts --detail
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
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/arrayDestructuringInSwitch1.ts --detail
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/arrayDestructuringInSwitch1.ts
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

- `reference/typescript/tests/cases/compiler/arrayDestructuringInSwitch1.ts`

## Duplicate detection

- none found by path/title/feature scan

## Smart triage

### Smart triage: Triage import export: arrayDestructuringInSwitch1

- Issue class: `triage-needed`
- Feature label: `import-export`
- Diagnostic: `UnsupportedModule` / `unsupported-feature-boundary`
- Path: `reference/typescript/tests/cases/compiler/arrayDestructuringInSwitch1.ts`

Reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/arrayDestructuringInSwitch1.ts
```

Source overview:

```json
{
  "suite": "tsc",
  "bytes": 643,
  "lines": 22,
  "extension": ".ts",
  "first_code_line": "export type Expression = BooleanLogicExpression | 'true' | 'false';"
}
```

Failure location:

```json
{
  "code": "UnsupportedModule",
  "message": "issue-055: unsupported function export; module resolution and loading are not implemented at 177..183",
  "span_start": 177,
  "span_end": 183,
  "line": 5,
  "column": 5,
  "feature_label": "import-export",
  "error_type": "unsupported-feature-boundary"
}
```

Source context:

```text
2 | export type Expression = BooleanLogicExpression | 'true' | 'false';
3 | export type BooleanLogicExpression = ['and', ...Expression[]] | ['not', Expression];
4 | 
5 | export function evaluate(expression: Expression): boolean {
6 |   if (Array.isArray(expression)) {
7 |     const [operator, ...operands] = expression;
8 |     switch (operator) {
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
        kind: Export,
        span: Span {
            start: 20,
            end: 26,
        },
    },
    SpannedToken {
        kind: Ident(
            "type",
        ),
        span: Span {
            start: 27,
            end: 31,
        },
    },
    SpannedToken {
        kind: Ident(
            "Expression",
        ),
        span: Span {
            start: 32,
            end: 42,
        },
    },
    SpannedToken {
        kind: Equal,
        span: Span {
            start: 43,
            end: 44,
        },
    },
    SpannedToken {
        kind: Ident(
            "BooleanLogicExpression",
        ),
        span: Span {
            start: 45,
            end: 67,
        },
    },
    SpannedToken {
        kind: Pipe,
        span: Span {
            start: 68,
            end: 69,
        },
    },
    SpannedToken {
        kind: String(
            "true",
        ),
        span: Span {
            start: 70,
            end: 76,
        },
    },
    SpannedToken {
        kind: Pipe,
        span: Span {
            start: 77,
            end: 78,
        },
    },
    SpannedToken {
        kind: String(
            "false",
        ),
        span: Span {
            start: 79,
            end: 86,
        },
    },
    SpannedToken {
        kind: Semicolon,
        span: Span {
            start: 86,
            end: 87,
        },
    },
    SpannedToken {
        kind: Export,
        span: Span {
            start: 89,
            end: 95,
        },
    },
    SpannedToken {
        kind: Ident(
            "type",
        ),
        span: Span {
            start: 96,
            end: 100,
        },
    },
    SpannedToken {
        kind: Ident(
            "BooleanLogicExpression",
        ),
        span: Span {
            start: 101,
            end: 123,
        },
    },
    SpannedToken {
        kind: Equal,
        span: Span {
            start: 124,
            end: 125,
        },
    },
    SpannedToken {
        kind: LeftBracket,
        span: Span {
            start: 126,
            end: 127,
        },
    },
    SpannedToken {
        kind: String(
            "and",
        ),
        span: Span {
            start: 127,
            end: 132,
        },
    },
    SpannedToken {
        kind: Comma,
        span: Span {
            start: 132,
            end: 133,
        },
    },
    SpannedToken {
        kind: DotDotDot,
        span: Span {
            start: 134,
            end: 137,
        },
    },
    SpannedToken {
        kind: Ident(
            "Expression",
        ),
        span: Span {
            start: 137,
            end: 147,
        },
    },
    SpannedToken {
        kind: LeftBracket,
        span: Span {
            start: 147,
            end: 148,
        },
    },
    SpannedToken {
        kind: RightBracket,
        span: Span {
            start: 148,
            end: 149,
        },
    },
    SpannedToken {
        kind: RightBracket,
        span: Span {
            start: 149,
            end: 150,
        },
    },
    SpannedToken {
        kind: Pipe,
        span: Span {
            start: 151,
            end: 152,
        },
    },
    SpannedToken {
        kind: LeftBracket,
        span: Span {
            start: 153,
            end: 154,
        },
    },
    SpannedToken {
        kind: String(
            "not",
        ),
        span: Span {
            start: 154,
            end: 159,
```

#### ast

- ok: `False`
- truncated: `False`

```text
error: [UnsupportedModule] issue-055: unsupported function export; module resolution and loading are not implemented at 177..183
```

#### resolved

- ok: `False`
- truncated: `False`

```text
error: [UnsupportedModule] issue-055: unsupported function export; module resolution and loading are not implemented at 177..183
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
        "kind": "function",
        "typeText": "boolean",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/arrayDestructuringInSwitch1.ts",
        "start": 193,
        "length": 8,
        "line": 5,
        "character": 17,
        "name": "evaluate"
      },
      {
        "kind": "parameter",
        "typeText": "Expression",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/arrayDestructuringInSwitch1.ts",
        "start": 202,
        "length": 10,
        "line": 5,
        "character": 26,
        "name": "expression"
      },
      {
        "kind": "parameter",
        "typeText": "Expression",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/arrayDestructuringInSwitch1.ts",
        "start": 400,
        "length": 5,
        "line": 10,
        "character": 32,
        "name": "child"
      }
    ],
    "typescriptVersion": "6.0.3"
  },
  "ast": {
    "topLevel": [
      {
        "kind": "TypeAliasDeclaration",
        "text": "export type Expression = BooleanLogicExpression | 'true' | 'false';",
        "line": 2,
        "character": 1
      },
      {
        "kind": "TypeAliasDeclaration",
        "text": "export type BooleanLogicExpression = ['and', ...Expression[]] | ['not', Expression];",
        "line": 3,
        "character": 1
      },
      {
        "kind": "FunctionDeclaration",
        "text": "export function evaluate(expression: Expression): boolean {\r\n  if (Array.isArray(expression)) {\r\n    const [operator, ..",
        "line": 5,
        "character": 1
      }
    ],
    "pathToPosition": [
      {
        "kind": "SourceFile",
        "text": "export type Expression = BooleanLogicExpression | 'true' | 'false';\r\nexport type BooleanLogicExpression = ['and', ...Exp",
        "line": 2,
        "character": 1
      },
      {
        "kind": "FunctionDeclaration",
        "text": "export function evaluate(expression: Expression): boolean {\r\n  if (Array.isArray(expression)) {\r\n    const [operator, ..",
        "line": 5,
        "character": 1
      },
      {
        "kind": "ExportKeyword",
        "text": "export",
        "line": 5,
        "character": 1
      }
    ]
  }
}
```

Stack trace:

```text
error: [UnsupportedModule] issue-055: unsupported function export; module resolution and loading are not implemented at 177..183
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
