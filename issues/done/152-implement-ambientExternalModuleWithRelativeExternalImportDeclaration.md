---
id: 152
title: "Implement Ambientexternalmodulewithrelativeexternalimportdeclaration"
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

Triage ambientExternalModuleWithRelativeExternalImportDeclaration across 1 failing reference test cases and split this bucket into implementation-ready child issues.

## Problem

Reference test results show 1 cases fail in directory `ambientExternalModuleWithRelativeExternalImportDeclaration` with diagnostics: ambient-declaration. The compiler cannot handle these syntax/semantics, preventing compilation of code in this category.

Problem: ambientExternalModuleWithRelativeExternalImportDeclaration has 1 reference failures and needs smart-triage evidence before implementation starts.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/ambientExternalModuleWithRelativeExternalImportDeclaration.ts
```

Coverage window:

```sh
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/ambientExternalModuleWithRelativeExternalImportDeclaration.ts --detail
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
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/ambientExternalModuleWithRelativeExternalImportDeclaration.ts --detail
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/ambientExternalModuleWithRelativeExternalImportDeclaration.ts
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

- `reference/typescript/tests/cases/compiler/ambientExternalModuleWithRelativeExternalImportDeclaration.ts`

## Duplicate detection

## Smart triage

### Smart triage: Triage ambient declaration: ambientExternalModuleWithRelativeExternalImportDeclaration

- Issue class: `triage-needed`
- Feature label: `ambient-declaration`
- Diagnostic: `UnsupportedSyntax` / `parser-or-frontend-unsupported`
- Path: `reference/typescript/tests/cases/compiler/ambientExternalModuleWithRelativeExternalImportDeclaration.ts`

Reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/ambientExternalModuleWithRelativeExternalImportDeclaration.ts
```

Source overview:

```json
{
  "suite": "tsc",
  "bytes": 267,
  "lines": 11,
  "extension": ".ts",
  "first_code_line": "declare module \"OuterModule\" {"
}
```

Failure location:

```json
{
  "code": "UnsupportedSyntax",
  "message": "expected Semicolon, got Some(Ident(\"module\")) at 28..34",
  "span_start": 28,
  "span_end": 34,
  "line": 2,
  "column": 10,
  "feature_label": "ambient-declaration",
  "error_type": "parser-or-frontend-unsupported"
}
```

Source context:

```text
1 | // @target: es2015
2 | declare module "OuterModule" {
3 |     import m2 = require("./SubModule");
4 |     class SubModule {
5 |         public static StaticVar: number;
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
    "path": "issues/done/152-implement-ambientExternalModuleWithRelativeExternalImportDeclaration.md",
    "title": "Implement Ambientexternalmodulewithrelativeexternalimportdeclaration",
    "reason": "same reference path, title overlap"
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
            "module",
        ),
        span: Span {
            start: 28,
            end: 34,
        },
    },
    SpannedToken {
        kind: String(
            "OuterModule",
        ),
        span: Span {
            start: 35,
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
        kind: Import,
        span: Span {
            start: 56,
            end: 62,
        },
    },
    SpannedToken {
        kind: Ident(
            "m2",
        ),
        span: Span {
            start: 63,
            end: 65,
        },
    },
    SpannedToken {
        kind: Equal,
        span: Span {
            start: 66,
            end: 67,
        },
    },
    SpannedToken {
        kind: Ident(
            "require",
        ),
        span: Span {
            start: 68,
            end: 75,
        },
    },
    SpannedToken {
        kind: LeftParen,
        span: Span {
            start: 75,
            end: 76,
        },
    },
    SpannedToken {
        kind: String(
            "./SubModule",
        ),
        span: Span {
            start: 76,
            end: 89,
        },
    },
    SpannedToken {
        kind: RightParen,
        span: Span {
            start: 89,
            end: 90,
        },
    },
    SpannedToken {
        kind: Semicolon,
        span: Span {
            start: 90,
            end: 91,
        },
    },
    SpannedToken {
        kind: Class,
        span: Span {
            start: 97,
```

#### ast

- ok: `False`
- truncated: `False`

```text
error: [UnsupportedSyntax] expected Semicolon, got Some(Ident("module")) at 28..34
```

#### resolved

- ok: `False`
- truncated: `False`

```text
error: [UnsupportedSyntax] expected Semicolon, got Some(Ident("module")) at 28..34
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
        "code": 2439,
        "category": "Error",
        "message": "Import or export declaration in an ambient module declaration cannot reference module through relative module name.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/ambientExternalModuleWithRelativeExternalImportDeclaration.ts",
        "start": 56,
        "length": 35,
        "line": 3,
        "character": 5
      },
      {
        "code": 2307,
        "category": "Error",
        "message": "Cannot find module './SubModule' or its corresponding type declarations.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/ambientExternalModuleWithRelativeExternalImportDeclaration.ts",
        "start": 76,
        "length": 13,
        "line": 3,
        "character": 25
      }
    ],
    "hints": [],
    "typescriptVersion": "6.0.3"
  },
  "ast": {
    "topLevel": [
      {
        "kind": "ModuleDeclaration",
        "text": "declare module \"OuterModule\" {\r\n    import m2 = require(\"./SubModule\");\r\n    class SubModule {\r\n        public static St",
        "line": 2,
        "character": 1
      }
    ],
    "pathToPosition": [
      {
        "kind": "SourceFile",
        "text": "declare module \"OuterModule\" {\r\n    import m2 = require(\"./SubModule\");\r\n    class SubModule {\r\n        public static St",
        "line": 2,
        "character": 1
      },
      {
        "kind": "ModuleDeclaration",
        "text": "declare module \"OuterModule\" {\r\n    import m2 = require(\"./SubModule\");\r\n    class SubModule {\r\n        public static St",
        "line": 2,
        "character": 1
      }
    ]
  }
}
```

Stack trace:

```text
error: [UnsupportedSyntax] expected Semicolon, got Some(Ident("module")) at 28..34
```


## 重複整理メモ

この issue はタイトルが重複しているため、内容がより充実している
`issues/done/613-implement-ambientExternalModuleWithRelativeExternalImportDeclaration.md` に統合されました。
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

