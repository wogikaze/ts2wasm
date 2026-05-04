---
id: 115
title: "Implement Aliasinaccessiblemodule"
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

Triage aliasInaccessibleModule across 2 failing reference test cases and split this bucket into implementation-ready child issues.

## Problem

Reference test results show 2 cases fail in directory `aliasInaccessibleModule` with diagnostics: type-alias. The compiler cannot handle these syntax/semantics, preventing compilation of code in this category.

Problem: aliasInaccessibleModule has 2 reference failures and needs smart-triage evidence before implementation starts.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/aliasInaccessibleModule.ts
```

Coverage window:

```sh
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/aliasInaccessibleModule.ts --detail
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
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/aliasInaccessibleModule.ts --detail
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/aliasInaccessibleModule.ts
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

- `reference/typescript/tests/cases/compiler/aliasInaccessibleModule.ts`
- `reference/typescript/tests/cases/compiler/aliasInaccessibleModule2.ts`

## Duplicate detection

## Smart triage

### Smart triage: Triage type alias: aliasInaccessibleModule

- Issue class: `triage-needed`
- Feature label: `type-alias`
- Diagnostic: `UnsupportedSyntax` / `parser-or-frontend-unsupported`
- Path: `reference/typescript/tests/cases/compiler/aliasInaccessibleModule.ts`

Reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/aliasInaccessibleModule.ts
```

Source overview:

```json
{
  "suite": "tsc",
  "bytes": 105,
  "lines": 7,
  "extension": ".ts",
  "first_code_line": "namespace M {"
}
```

Failure location:

```json
{
  "code": "UnsupportedSyntax",
  "message": "expected Semicolon, got Some(Ident(\"M\")) at 53..54",
  "span_start": 53,
  "span_end": 54,
  "line": 3,
  "column": 13,
  "feature_label": "type-alias",
  "error_type": "parser-or-frontend-unsupported"
}
```

Source context:

```text
1 | // @target: es2015
2 | // @declaration: true
3 | namespace M {
4 |     namespace N {
5 |     }
6 |     export import X = N;
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
    "path": "issues/done/115-implement-aliasInaccessibleModule.md",
    "title": "Implement Aliasinaccessiblemodule",
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
            "namespace",
        ),
        span: Span {
            start: 43,
            end: 52,
        },
    },
    SpannedToken {
        kind: Ident(
            "M",
        ),
        span: Span {
            start: 53,
            end: 54,
        },
    },
    SpannedToken {
        kind: LeftBrace,
        span: Span {
            start: 55,
            end: 56,
        },
    },
    SpannedToken {
        kind: Ident(
            "namespace",
        ),
        span: Span {
            start: 62,
            end: 71,
        },
    },
    SpannedToken {
        kind: Ident(
            "N",
        ),
        span: Span {
            start: 72,
            end: 73,
        },
    },
    SpannedToken {
        kind: LeftBrace,
        span: Span {
            start: 74,
            end: 75,
        },
    },
    SpannedToken {
        kind: RightBrace,
        span: Span {
            start: 81,
            end: 82,
        },
    },
    SpannedToken {
        kind: Export,
        span: Span {
            start: 88,
            end: 94,
        },
    },
    SpannedToken {
        kind: Import,
        span: Span {
            start: 95,
            end: 101,
        },
    },
    SpannedToken {
        kind: Ident(
            "X",
        ),
        span: Span {
            start: 102,
            end: 103,
        },
    },
    SpannedToken {
        kind: Equal,
        span: Span {
            start: 104,
            end: 105,
        },
    },
    SpannedToken {
        kind: Ident(
            "N",
        ),
        span: Span {
            start: 106,
            end: 107,
        },
    },
    SpannedToken {
        kind: Semicolon,
        span: Span {
            start: 107,
            en
```

#### ast

- ok: `False`
- truncated: `False`

```text
error: [UnsupportedSyntax] expected Semicolon, got Some(Ident("M")) at 53..54
```

#### resolved

- ok: `False`
- truncated: `False`

```text
error: [UnsupportedSyntax] expected Semicolon, got Some(Ident("M")) at 53..54
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
        "kind": "ModuleDeclaration",
        "text": "namespace M {\r\n    namespace N {\r\n    }\r\n    export import X = N;\r\n}",
        "line": 3,
        "character": 1
      }
    ],
    "pathToPosition": [
      {
        "kind": "SourceFile",
        "text": "namespace M {\r\n    namespace N {\r\n    }\r\n    export import X = N;\r\n}",
        "line": 3,
        "character": 1
      },
      {
        "kind": "ModuleDeclaration",
        "text": "namespace M {\r\n    namespace N {\r\n    }\r\n    export import X = N;\r\n}",
        "line": 3,
        "character": 1
      },
      {
        "kind": "Identifier",
        "text": "M",
        "line": 3,
        "character": 11
      }
    ]
  }
}
```

Stack trace:

```text
error: [UnsupportedSyntax] expected Semicolon, got Some(Ident("M")) at 53..54
```


## 重複整理メモ

この issue はタイトルが重複しているため、内容がより充実している
`issues/done/581-implement-aliasInaccessibleModule.md` に統合されました。
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
