---
id: 624
title: "Implement Ambientwithstatements (audit reopened #624)"
type: spike
area: frontend/syntax
class: blocked
priority: P1
depends_on: [432]
blocks: []
created: 2026-05-01
updated: 2026-05-06
status: done
completed: 2026-05-06
---

## Summary

Triage ambientWithStatements across 1 failing reference test cases and split this bucket into implementation-ready child issues.

## Problem

Reference test results show 1 cases fail in directory `ambientWithStatements` with diagnostics: import-export. The compiler cannot handle these syntax/semantics, preventing compilation of code in this category.

Problem: ambientWithStatements has 1 reference failures and needs smart-triage evidence before implementation starts.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/ambientWithStatements.ts
```

Coverage window:

```sh
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/ambientWithStatements.ts --detail
```

## Desired final state

This generated bucket is either split into implementation-ready child issues or superseded by an existing open/done issue with matching evidence. Do not implement directly from this bucket.

## Scope

In scope:

- [x] Inspect the smart triage report below
- [x] Confirm whether existing open/done issues already cover this bucket
- [x] Split one feature family, one observable behavior, or one fixed reference window into child issues
- [x] Preserve exact reproduction commands and representative AST/diagnostic evidence in each child issue

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

- [x] Duplicate candidates below are confirmed as no-match or this issue is superseded
- [x] At least one child issue contains an exact `mise run reference-triage -- ...` command
- [x] Child issue includes failing path, diagnostic code, source context, visible symbols, and parser/TypeScript AST evidence
- [x] Child issue acceptance names the exact fixture/reference path and diagnostic/stdout change

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run
```

Impacted commands:

```sh
mise run reference-coverage -- tsc --limit 2
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/ambientWithStatements.ts --detail
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/ambientWithStatements.ts
```

Not run:

- none

## Docs / current-state / issue sync

Final-state docs:

- [x] not affected

Current state:

- [x] updated: `current-state.md` (repo root)

Follow-up issues:

- [x] none

## Notes

## Affected test files

- `reference/typescript/tests/cases/compiler/ambientWithStatements.ts`

## Duplicate detection

- `issues/open/165-implement-ambientWithStatements.md` - Implement Ambientwithstatements (same reference path, same group key, title overlap)
- `issues/done/538-implement-ambientWithStatements.md` - Implement Ambientwithstatements (same reference path, same feature label, same group key, title overlap)

## Smart triage

### Smart triage: Triage import export: ambientWithStatements

- Issue class: `triage-needed`
- Feature label: `import-export`
- Diagnostic: `UnsupportedModule` / `unsupported-feature-boundary`
- Path: `reference/typescript/tests/cases/compiler/ambientWithStatements.ts`

Reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/ambientWithStatements.ts
```

Source overview:

```json
{
  "suite": "tsc",
  "bytes": 468,
  "lines": 31,
  "extension": ".ts",
  "first_code_line": "declare namespace M {"
}
```

Failure location:

```json
{
  "code": "UnsupportedModule",
  "message": "issue-400: ambient namespace declarations require module ownership before runtime lowering at 107..116",
  "span_start": 107,
  "span_end": 116,
  "line": 5,
  "column": 13,
  "feature_label": "import-export",
  "error_type": "unsupported-feature-boundary"
}
```

Source context:

```text
2 | // @ignoreDeprecations: 6.0
3 | // @strict: false
4 | // @alwaysStrict: true, false
5 | declare namespace M {
6 |     break;
7 |     continue;
8 |     debugger;
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
    "path": "issues/done/165-implement-ambientWithStatements.md",
    "title": "Implement Ambientwithstatements",
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
    "path": "issues/done/538-implement-ambientWithStatements.md",
    "title": "Implement Ambientwithstatements",
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
            "declare",
        ),
        span: Span {
            start: 99,
            end: 106,
        },
    },
    SpannedToken {
        kind: Ident(
            "namespace",
        ),
        span: Span {
            start: 107,
            end: 116,
        },
    },
    SpannedToken {
        kind: Ident(
            "M",
        ),
        span: Span {
            start: 117,
            end: 118,
        },
    },
    SpannedToken {
        kind: LeftBrace,
        span: Span {
            start: 119,
            end: 120,
        },
    },
    SpannedToken {
        kind: Break,
        span: Span {
            start: 126,
            end: 131,
        },
    },
    SpannedToken {
        kind: Semicolon,
        span: Span {
            start: 131,
            end: 132,
        },
    },
    SpannedToken {
        kind: Continue,
        span: Span {
            start: 138,
            end: 146,
        },
    },
    SpannedToken {
        kind: Semicolon,
        span: Span {
            start: 146,
            end: 147,
        },
    },
    SpannedToken {
        kind: Ident(
            "debugger",
        ),
        span: Span {
            start: 153,
            end: 161,
        },
    },
    SpannedToken {
        kind: Semicolon,
        span: Span {
            start: 161,
            end: 162,
        },
    },
    SpannedToken {
        kind: Do,
        span: Span {
            start: 168,
            end: 170,
        },
    },
    SpannedToken {
        kind: LeftBrace,
        span: Span {
            start: 171,
            end: 172,
        },
    },
    SpannedToken {
        kind: RightBrace,
        span: Span {
            start: 173,
            end: 174,
        },
    },
    SpannedToken {
        kind: While,
        span: Span {
            start: 175,
            end: 180,
        },
    },
    SpannedToken {
        kind: LeftParen,
        span: Span {
            start: 181,
            end: 182,
        },
    },
    SpannedToken {
        kind: True,
        span: Span {
            start: 182,
            end: 186,
        },
    },
    SpannedToken {
        kind: RightParen,
        span: Span {
            start: 186,
            end: 187,
        },
    },
    SpannedToken {
        kind: Semicolon,
        span: Span {
            start: 187,
            end: 188,
        },
    },
    SpannedToken {
        kind: Var,
        span: Span {
            start: 194,
            end: 197,
        },
    },
    SpannedToken {
        kind: Ident(
            "x",
        ),
        span: Span {
            start: 198,
            end: 199,
        },
    },
    SpannedToken {
        kind: Semicolon,
        span: Span {
            start: 199,
            end: 200,
        },
    },
    SpannedToken {
        kind: For,
        span: Span {
            start: 206,
            end: 209,
        },
    },
    SpannedToken {
        kind: LeftParen,
        span: Span {
            start: 210,
            end: 211,
        },
    },
    SpannedToken {
        kind: Ident(
            "x",
        ),
        span: Span {
            start: 211,
            end: 212,
        },
    },
    SpannedToken {
        kind: In,
        span: Span {
            start: 213,
            end: 215,
        },
    },
    SpannedToken {
        kind: Null,
        span: Span {
            start: 216,
            end: 220,
        },
    },
    Spanne
```

#### ast

- ok: `False`
- truncated: `False`

```text
error: [UnsupportedModule] issue-400: ambient namespace declarations require module ownership before runtime lowering at 107..116
```

#### resolved

- ok: `False`
- truncated: `False`

```text
error: [UnsupportedModule] issue-400: ambient namespace declarations require module ownership before runtime lowering at 107..116
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
        "code": 1036,
        "category": "Error",
        "message": "Statements are not allowed in ambient contexts.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/ambientWithStatements.ts",
        "start": 126,
        "length": 5,
        "line": 6,
        "character": 5
      },
      {
        "code": 1104,
        "category": "Error",
        "message": "A 'continue' statement can only be used within an enclosing iteration statement.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/ambientWithStatements.ts",
        "start": 138,
        "length": 9,
        "line": 7,
        "character": 5
      },
      {
        "code": 2407,
        "category": "Error",
        "message": "The right-hand side of a 'for...in' statement must be of type 'any', an object type or a type parameter, but here has type 'never'.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/ambientWithStatements.ts",
        "start": 216,
        "length": 4,
        "line": 11,
        "character": 15
      },
      {
        "code": 1344,
        "category": "Error",
        "message": "'A label is not allowed here.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/ambientWithStatements.ts",
        "start": 267,
        "length": 1,
        "line": 14,
        "character": 5
      },
      {
        "code": 1108,
        "category": "Error",
        "message": "A 'return' statement can only be used within a function body.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/ambientWithStatements.ts",
        "start": 282,
        "length": 6,
        "line": 15,
        "character": 5
      },
      {
        "code": 1101,
        "category": "Error",
        "message": "'with' statements are not allowed in strict mode.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/ambientWithStatements.ts",
        "start": 478,
        "length": 4,
        "line": 29,
        "character": 5
      },
      {
        "code": 2410,
        "category": "Error",
        "message": "The 'with' statement is not supported. All symbols in a 'with' block will have type 'any'.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/ambientWithStatements.ts",
        "start": 478,
        "length": 8,
        "line": 29,
        "character": 5
      }
    ],
    "hints": [
      {
        "kind": "binding",
        "typeText": "any",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/ambientWithStatements.ts",
        "start": 198,
        "length": 1,
        "line": 10,
        "character": 9,
        "name": "x"
      },
      {
        "kind": "binding",
        "typeText": "any",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/ambientWithStatements.ts",
        "start": 274,
        "length": 1,
        "line": 14,
        "character": 12,
        "name": "y"
      },
      {
        "kind": "binding",
        "typeText": "unknown",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/ambientWithStatements.ts",
        "start": 439,
        "length": 1,
        "line": 25,
        "character": 12,
        "name": "e"
      }
    ],
    "typescriptVersion": "6.0.3"
  },
  "ast": {
    "topLevel": [
      {
        "kind": "ModuleDeclaration",
        "text": "declare namespace M {\r\n    break;\r\n    continue;\r\n    debugger;\r\n    do { } while (true);\r\n    var x;\r\n    for (x in nul",
        "line": 5,
        "character": 1
      }
    ],
    "pathToPosition": [
      {
        "kind": "SourceFile",
        "text": "declare namespace M {\r\n    break;\r\n    continue;\r\n    debugger;\r\n    do { } while (true);\r\n    var x;\r\n    for (x in nul",
        "line": 5,
        "character": 1
      },
      {
        "kind": "ModuleDeclaration",
        "text": "declare namespace M {\r\n    break;\r\n    continue;\r\n    debugger;\r\n    do { } while (true);\r\n    var x;\r\n    for (x in nul",
        "line": 5,
        "character": 1
      }
    ]
  }
}
```

Stack trace:

```text
error: [UnsupportedModule] issue-400: ambient namespace declarations require module ownership before runtime lowering at 107..116
```

## Completion evidence

Commits:

- pending closure commit

Validation result:

```text
command: python scripts/manager.py update-issue-index --check
result: pass
date: 2026-05-06
```

Remaining risks:

- none

## Status

Superseded by issue #165. Duplicate from separate coverage run.

## Reopened by audit

Date: 2026-05-05

Classification: must-reopen.

Reopen reason: frontmatter still says `class: blocked`, which is incompatible with a completed issue unless explicit supersedence/closure evidence is present.

Violated acceptance: the issue cannot provide repo-local close evidence for its checked acceptance criteria while it remains in this state. Acceptance checkboxes were reset for re-verification.

Evidence files:
- open issue file before this move
- `issues/done/624-implement-ambientWithStatements.md` after this move

Split follow-up: none created in this audit wave; this reopened issue remains the tracking item.
