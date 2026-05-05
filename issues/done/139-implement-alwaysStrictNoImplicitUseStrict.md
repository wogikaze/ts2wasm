---
id: 139
title: "Implement Alwaysstrictnoimplicitusestrict"
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

Triage alwaysStrictNoImplicitUseStrict across 1 failing reference test cases and split this bucket into implementation-ready child issues.

## Problem

Reference test results show 1 cases fail in directory `alwaysStrictNoImplicitUseStrict` with diagnostics: parser-syntax. The compiler cannot handle these syntax/semantics, preventing compilation of code in this category.

Problem: alwaysStrictNoImplicitUseStrict has 1 reference failures and needs smart-triage evidence before implementation starts.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/alwaysStrictNoImplicitUseStrict.ts
```

Coverage window:

```sh
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/alwaysStrictNoImplicitUseStrict.ts --detail
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
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/alwaysStrictNoImplicitUseStrict.ts --detail
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/alwaysStrictNoImplicitUseStrict.ts
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

- `reference/typescript/tests/cases/compiler/alwaysStrictNoImplicitUseStrict.ts`

## Duplicate detection

## Smart triage

### Smart triage: Triage parser syntax: alwaysStrictNoImplicitUseStrict

- Issue class: `triage-needed`
- Feature label: `parser-syntax`
- Diagnostic: `UnsupportedSyntax` / `parser-or-frontend-unsupported`
- Path: `reference/typescript/tests/cases/compiler/alwaysStrictNoImplicitUseStrict.ts`

Reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/alwaysStrictNoImplicitUseStrict.ts
```

Source overview:

```json
{
  "suite": "tsc",
  "bytes": 169,
  "lines": 10,
  "extension": ".ts",
  "first_code_line": "namespace M {"
}
```

Failure location:

```json
{
  "code": "UnsupportedSyntax",
  "message": "expected Semicolon, got Some(Ident(\"M\")) at 104..105",
  "span_start": 104,
  "span_end": 105,
  "line": 6,
  "column": 11,
  "feature_label": "parser-syntax",
  "error_type": "parser-or-frontend-unsupported"
}
```

Source context:

```text
3 | // @alwaysStrict: true
4 | // @noImplicitUseStrict: true
5 |
6 | namespace M {
7 |     export function f() {
8 |         var arguments = [];
9 |     }
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
    "path": "issues/open/059-implement-parser-syntax-extensions.md",
    "title": "Implement parser syntax extensions for TypeScript and advanced JS",
    "reason": "same feature label, title overlap"
  },
  {
    "state": "open",
    "path": "issues/done/139-implement-alwaysStrictNoImplicitUseStrict.md",
    "title": "Implement Alwaysstrictnoimplicitusestrict",
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
    "path": "issues/open/065-implement-parser-syntax.md",
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
        kind: Ident(
            "namespace",
        ),
        span: Span {
            start: 94,
            end: 103,
        },
    },
    SpannedToken {
        kind: Ident(
            "M",
        ),
        span: Span {
            start: 104,
            end: 105,
        },
    },
    SpannedToken {
        kind: LeftBrace,
        span: Span {
            start: 106,
            end: 107,
        },
    },
    SpannedToken {
        kind: Export,
        span: Span {
            start: 112,
            end: 118,
        },
    },
    SpannedToken {
        kind: Function,
        span: Span {
            start: 119,
            end: 127,
        },
    },
    SpannedToken {
        kind: Ident(
            "f",
        ),
        span: Span {
            start: 128,
            end: 129,
        },
    },
    SpannedToken {
        kind: LeftParen,
        span: Span {
            start: 129,
            end: 130,
        },
    },
    SpannedToken {
        kind: RightParen,
        span: Span {
            start: 130,
            end: 131,
        },
    },
    SpannedToken {
        kind: LeftBrace,
        span: Span {
            start: 132,
            end: 133,
        },
    },
    SpannedToken {
        kind: Var,
        span: Span {
            start: 142,
            end: 145,
        },
    },
    SpannedToken {
        kind: Ident(
            "arguments",
        ),
        span: Span {
            start: 146,
            end: 155,
        },
    },
    SpannedToken {
        kind: Equal,
        span: Span {
            start: 156,
            end: 157,
        },
    },
    SpannedToken {
        kind: LeftBracket,
        span: Span {
            start: 158,
            end: 159,
        },
    },
    Span
```

#### ast

- ok: `False`
- truncated: `False`

```text
error: [UnsupportedSyntax] expected Semicolon, got Some(Ident("M")) at 104..105
```

#### resolved

- ok: `False`
- truncated: `False`

```text
error: [UnsupportedSyntax] expected Semicolon, got Some(Ident("M")) at 104..105
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
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/alwaysStrictNoImplicitUseStrict.ts",
        "start": 146,
        "length": 9,
        "line": 8,
        "character": 13
      }
    ],
    "hints": [
      {
        "kind": "function",
        "typeText": "void",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/alwaysStrictNoImplicitUseStrict.ts",
        "start": 128,
        "length": 1,
        "line": 7,
        "character": 21,
        "name": "f"
      },
      {
        "kind": "binding",
        "typeText": "never[]",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/alwaysStrictNoImplicitUseStrict.ts",
        "start": 146,
        "length": 9,
        "line": 8,
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
        "line": 6,
        "character": 1
      }
    ],
    "pathToPosition": [
      {
        "kind": "SourceFile",
        "text": "namespace M {\n    export function f() {\n        var arguments = [];\n    }\n}",
        "line": 6,
        "character": 1
      },
      {
        "kind": "ModuleDeclaration",
        "text": "namespace M {\n    export function f() {\n        var arguments = [];\n    }\n}",
        "line": 6,
        "character": 1
      },
      {
        "kind": "Identifier",
        "text": "M",
        "line": 6,
        "character": 11
      }
    ]
  }
}
```

Stack trace:

```text
error: [UnsupportedSyntax] expected Semicolon, got Some(Ident("M")) at 104..105
```


## 重複整理メモ

この issue はタイトルが重複しているため、内容がより充実している
`issues/open/603-implement-alwaysStrictNoImplicitUseStrict.md` に統合されました。
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
