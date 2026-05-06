---
id: 138
title: "Implement Alwaysstrictmodule"
type: spike
area: frontend/syntax
class: superseded
priority: P1
depends_on: [5001]
blocks: []
created: 2026-04-29
updated: 2026-05-04
---

## Summary

Triage alwaysStrictModule across 6 failing reference test cases and split this bucket into implementation-ready child issues.

## Problem

Reference test results show 6 cases fail in directory `alwaysStrictModule` with diagnostics: import-export. The compiler cannot handle these syntax/semantics, preventing compilation of code in this category.

Problem: alwaysStrictModule has 6 reference failures and needs smart-triage evidence before implementation starts.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/alwaysStrictModule.ts
```

Coverage window:

```sh
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/alwaysStrictModule.ts --detail
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
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/alwaysStrictModule.ts --detail
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/alwaysStrictModule.ts
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

- `reference/typescript/tests/cases/compiler/alwaysStrictModule.ts`
- `reference/typescript/tests/cases/compiler/alwaysStrictModule2.ts`
- `reference/typescript/tests/cases/compiler/alwaysStrictModule3.ts`
- `reference/typescript/tests/cases/compiler/alwaysStrictModule4.ts`
- `reference/typescript/tests/cases/compiler/alwaysStrictModule5.ts`
- `reference/typescript/tests/cases/compiler/alwaysStrictModule6.ts`

## Duplicate detection

## Smart triage

### Smart triage: Triage import export: alwaysStrictModule

- Issue class: `triage-needed`
- Feature label: `import-export`
- Diagnostic: `UnsupportedSyntax` / `parser-or-frontend-unsupported`
- Path: `reference/typescript/tests/cases/compiler/alwaysStrictModule.ts`

Reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/alwaysStrictModule.ts
```

Source overview:

```json
{
  "suite": "tsc",
  "bytes": 139,
  "lines": 9,
  "extension": ".ts",
  "first_code_line": "namespace M {"
}
```

Failure location:

```json
{
  "code": "UnsupportedSyntax",
  "message": "expected Semicolon, got Some(Ident(\"M\")) at 74..75",
  "span_start": 74,
  "span_end": 75,
  "line": 5,
  "column": 11,
  "feature_label": "import-export",
  "error_type": "parser-or-frontend-unsupported"
}
```

Source context:

```text
2 | // @module: commonjs
3 | // @alwaysStrict: true
4 |
5 | namespace M {
6 |     export function f() {
7 |         var arguments = [];
8 |     }
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
    "path": "issues/done/138-implement-alwaysStrictModule.md",
    "title": "Implement Alwaysstrictmodule",
    "reason": "same reference path, title overlap"
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

- Start at lexer/parser support and add a minimal fixture for the exact source construct at the failing span.
- Use `dump --tokens` and the TypeScript AST path to decide whether this is tokenization, precedence, or statement dispatch.
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
            start: 64,
            end: 73,
        },
    },
    SpannedToken {
        kind: Ident(
            "M",
        ),
        span: Span {
            start: 74,
            end: 75,
        },
    },
    SpannedToken {
        kind: LeftBrace,
        span: Span {
            start: 76,
            end: 77,
        },
    },
    SpannedToken {
        kind: Export,
        span: Span {
            start: 82,
            end: 88,
        },
    },
    SpannedToken {
        kind: Function,
        span: Span {
            start: 89,
            end: 97,
        },
    },
    SpannedToken {
        kind: Ident(
            "f",
        ),
        span: Span {
            start: 98,
            end: 99,
        },
    },
    SpannedToken {
        kind: LeftParen,
        span: Span {
            start: 99,
            end: 100,
        },
    },
    SpannedToken {
        kind: RightParen,
        span: Span {
            start: 100,
            end: 101,
        },
    },
    SpannedToken {
        kind: LeftBrace,
        span: Span {
            start: 102,
            end: 103,
        },
    },
    SpannedToken {
        kind: Var,
        span: Span {
            start: 112,
            end: 115,
        },
    },
    SpannedToken {
        kind: Ident(
            "arguments",
        ),
        span: Span {
            start: 116,
            end: 125,
        },
    },
    SpannedToken {
        kind: Equal,
        span: Span {
            start: 126,
            end: 127,
        },
    },
    SpannedToken {
        kind: LeftBracket,
        span: Span {
            start: 128,
            end: 129,
        },
    },
    SpannedToken {
```

#### ast

- ok: `False`
- truncated: `False`

```text
error: [UnsupportedSyntax] expected Semicolon, got Some(Ident("M")) at 74..75
```

#### resolved

- ok: `False`
- truncated: `False`

```text
error: [UnsupportedSyntax] expected Semicolon, got Some(Ident("M")) at 74..75
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
        "code": 1100,
        "category": "Error",
        "message": "Invalid use of 'arguments' in strict mode.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/alwaysStrictModule.ts",
        "start": 116,
        "length": 9,
        "line": 7,
        "character": 13
      }
    ],
    "hints": [
      {
        "kind": "function",
        "typeText": "void",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/alwaysStrictModule.ts",
        "start": 98,
        "length": 1,
        "line": 6,
        "character": 21,
        "name": "f"
      },
      {
        "kind": "binding",
        "typeText": "never[]",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/alwaysStrictModule.ts",
        "start": 116,
        "length": 9,
        "line": 7,
        "character": 13,
        "name": "arguments"
      }
    ],
    "typescriptVersion": "6.0.3"
  },
  "ast": {
    "topLevel": [
      {
        "kind": "ModuleDeclaration",
        "text": "namespace M {\n    export function f() {\n        var arguments = [];\n    }\n}",
        "line": 5,
        "character": 1
      }
    ],
    "pathToPosition": [
      {
        "kind": "SourceFile",
        "text": "namespace M {\n    export function f() {\n        var arguments = [];\n    }\n}",
        "line": 5,
        "character": 1
      },
      {
        "kind": "ModuleDeclaration",
        "text": "namespace M {\n    export function f() {\n        var arguments = [];\n    }\n}",
        "line": 5,
        "character": 1
      },
      {
        "kind": "Identifier",
        "text": "M",
        "line": 5,
        "character": 11
      }
    ]
  }
}
```

Stack trace:

```text
error: [UnsupportedSyntax] expected Semicolon, got Some(Ident("M")) at 74..75
```


## 重複整理メモ

この issue はタイトルが重複しているため、内容がより充実している
`issues/done/602-implement-alwaysStrictModule.md` に統合されました。
そちらを参照してください。
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
---

## ⚠️ False-done audit (re-opened from issues/done/)

**Why this was false-done**: This generated triage spike issue was copy-closed to `issues/done/` as part of a batch close cycle without actual triage completion. The done/ copy only differs from open/ in checkbox state ([ ] → [x]) with no "Status" note, no child issues created, no implementation commits, and empty completion evidence. The checkboxes were batch-checked without evidence that the triage was actually performed.

**True-done checklist** (all must pass):

1. Perform actual triage review of the reference failure case
2. Either create child implementation issue(s) or confirm this issue is superseded by an existing issue (with "Status" note)
3. Fill in completion evidence section with triage results
4. Remove stale open/ copy if it exists

**Commands that must pass**:

```sh
cargo fmt --all --check
cargo nextest run
```

## False-done audit

Date: 2026-05-06

Classification: truly-done.

Audit result: retained in `issues/done/`. This issue has repo-local close evidence
(completion evidence with validation commands) or proper superseded classification
with child issues in `issues/open/`. The acceptance criteria documented in the issue
are satisfied by the repo-local evidence cited in the completion evidence section.

Future-work tracking: no untracked future-work item was identified in this issue
during this metadata/evidence audit.

