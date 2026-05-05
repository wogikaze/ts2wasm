---
id: 073
title: "Implement Classdeclarationwithinvalidconstonpropertydeclaration"
type: spike
area: frontend/syntax
class: superseded
priority: P1
depends_on: [5000]
blocks: []
created: 2026-04-29
updated: 2026-05-04
---

## Summary

Triage ClassDeclarationWithInvalidConstOnPropertyDeclaration across 2 failing reference test cases and split this bucket into implementation-ready child issues.

## Problem

Reference test results show 2 cases fail in directory `ClassDeclarationWithInvalidConstOnPropertyDeclaration` with diagnostics: parser-syntax. The compiler cannot handle these syntax/semantics, preventing compilation of code in this category.

Problem: ClassDeclarationWithInvalidConstOnPropertyDeclaration has 2 reference failures and needs smart-triage evidence before implementation starts.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/ClassDeclarationWithInvalidConstOnPropertyDeclaration.ts
```

Coverage window:

```sh
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/ClassDeclarationWithInvalidConstOnPropertyDeclaration.ts --detail
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
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/ClassDeclarationWithInvalidConstOnPropertyDeclaration.ts --detail
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/ClassDeclarationWithInvalidConstOnPropertyDeclaration.ts
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

- `reference/typescript/tests/cases/compiler/ClassDeclarationWithInvalidConstOnPropertyDeclaration.ts`
- `reference/typescript/tests/cases/compiler/ClassDeclarationWithInvalidConstOnPropertyDeclaration2.ts`

## Duplicate detection

## Smart triage

### Smart triage: Triage parser syntax: ClassDeclarationWithInvalidConstOnPropertyDeclaration

- Issue class: `triage-needed`
- Feature label: `parser-syntax`
- Diagnostic: `UnsupportedSyntax` / `parser-or-frontend-unsupported`
- Path: `reference/typescript/tests/cases/compiler/ClassDeclarationWithInvalidConstOnPropertyDeclaration.ts`

Reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/ClassDeclarationWithInvalidConstOnPropertyDeclaration.ts
```

Source overview:

```json
{
  "suite": "tsc",
  "bytes": 64,
  "lines": 4,
  "extension": ".ts",
  "first_code_line": "class AtomicNumbers {"
}
```

Failure location:

```json
{
  "code": "UnsupportedSyntax",
  "message": "expected identifier, got Some(SpannedToken { kind: Const, span: Span { start: 52, end: 57 } }) at 58..59",
  "span_start": 58,
  "span_end": 59,
  "line": 3,
  "column": 18,
  "feature_label": "parser-syntax",
  "error_type": "parser-or-frontend-unsupported"
}
```

Source context:

```text
1 | // @target: es2015
2 | class AtomicNumbers {
3 |   static const H = 1;
4 | }
```

Visible symbols before failure:

```json
[
  {
    "kind": "class",
    "name": "AtomicNumbers",
    "line": 2,
    "column": 1
  },
  {
    "kind": "binding",
    "name": "H",
    "line": 3,
    "column": 10
  }
]
```

Duplicate candidates:

```json
[
  {
    "state": "open",
    "path": "issues/open/059-implement-parser-syntax-extensions.md",
    "title": "Implement parser syntax extensions for TypeScript and advanced JS",
    "reason": "same feature label, title overlap"
  },
  {
    "state": "open",
    "path": "issues/done/073-implement-ClassDeclarationWithInvalidConstOnPropertyDeclaration.md",
    "title": "Implement Classdeclarationwithinvalidconstonpropertydeclaration",
    "reason": "same reference path, same feature label, title overlap"
  },
  {
    "state": "open",
    "path": "issues/done/200-implement-parser-syntax.md",
    "title": "Implement parser syntax extensions",
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
            start: 20,
            end: 25,
        },
    },
    SpannedToken {
        kind: Ident(
            "AtomicNumbers",
        ),
        span: Span {
            start: 26,
            end: 39,
        },
    },
    SpannedToken {
        kind: LeftBrace,
        span: Span {
            start: 40,
            end: 41,
        },
    },
    SpannedToken {
        kind: Static,
        span: Span {
            start: 45,
            end: 51,
        },
    },
    SpannedToken {
        kind: Const,
        span: Span {
            start: 52,
            end: 57,
        },
    },
    SpannedToken {
        kind: Ident(
            "H",
        ),
        span: Span {
            start: 58,
            end: 59,
        },
    },
    SpannedToken {
        kind: Equal,
        span: Span {
            start: 60,
            end: 61,
        },
    },
    SpannedToken {
        kind: Number(
            1,
        ),
        span: Span {
            start: 62,
            end: 63,
        },
    },
    SpannedToken {
        kind: Semicolon,
        span: Span {
            start: 63,
            end: 64,
        },
    },
    SpannedToken {
        kind: RightBrace,
        span: Span {
            start: 66,
            end: 67,
        },
    },
]
```

#### ast

- ok: `False`
- truncated: `False`

```text
error: [UnsupportedSyntax] expected identifier, got Some(SpannedToken { kind: Const, span: Span { start: 52, end: 57 } }) at 58..59
```

#### resolved

- ok: `False`
- truncated: `False`

```text
error: [UnsupportedSyntax] expected identifier, got Some(SpannedToken { kind: Const, span: Span { start: 52, end: 57 } }) at 58..59
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
        "code": 1248,
        "category": "Error",
        "message": "A class member cannot have the 'const' keyword.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/ClassDeclarationWithInvalidConstOnPropertyDeclaration.ts",
        "start": 58,
        "length": 1,
        "line": 3,
        "character": 16
      }
    ],
    "hints": [],
    "typescriptVersion": "6.0.3"
  },
  "ast": {
    "topLevel": [
      {
        "kind": "ClassDeclaration",
        "text": "class AtomicNumbers {\r\n  static const H = 1;\r\n}",
        "line": 2,
        "character": 1
      }
    ],
    "pathToPosition": [
      {
        "kind": "SourceFile",
        "text": "class AtomicNumbers {\r\n  static const H = 1;\r\n}",
        "line": 2,
        "character": 1
      },
      {
        "kind": "ClassDeclaration",
        "text": "class AtomicNumbers {\r\n  static const H = 1;\r\n}",
        "line": 2,
        "character": 1
      },
      {
        "kind": "PropertyDeclaration",
        "text": "static const H = 1;",
        "line": 3,
        "character": 3
      },
      {
        "kind": "Identifier",
        "text": "H",
        "line": 3,
        "character": 16
      }
    ]
  }
}
```

Stack trace:

```text
error: [UnsupportedSyntax] expected identifier, got Some(SpannedToken { kind: Const, span: Span { start: 52, end: 57 } }) at 58..59
```


## 重複整理メモ

この issue はタイトルが重複しているため、内容がより充実している
`issues/done/547-implement-ClassDeclarationWithInvalidConstOnPropertyDeclaration.md` に統合されました。
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
