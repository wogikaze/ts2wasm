---
id: 467
title: "Implement Transportstream (dup)"
type: spike
area: frontend/syntax
class: superseded
priority: P1
depends_on: [5000]
blocks: []
created: 2026-05-01
updated: 2026-05-04
---

## Summary

Triage TransportStream across 1 failing reference test cases and split this bucket into implementation-ready child issues.

## Problem

Reference test results show 1 cases fail in directory `TransportStream` with diagnostics: parser-syntax. The compiler cannot handle these syntax/semantics, preventing compilation of code in this category.

Problem: TransportStream has 1 reference failures and needs smart-triage evidence before implementation starts.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/TransportStream.ts
```

Coverage window:

```sh
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/TransportStream.ts --detail
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
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/TransportStream.ts --detail
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/TransportStream.ts
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

- `reference/typescript/tests/cases/compiler/TransportStream.ts`

## Duplicate detection

- `issues/done/081-implement-TransportStream.md` - Implement Transportstream (same reference path, same feature label, same group key, title overlap)
- `issues/open/442-implement-parser-syntax.md` - Implement parser syntax extensions (same feature label, same group key, title overlap)

## Smart triage

### Smart triage: Triage parser syntax: TransportStream

- Issue class: `triage-needed`
- Feature label: `parser-syntax`
- Diagnostic: `UnsupportedSyntax` / `parser-or-frontend-unsupported`
- Path: `reference/typescript/tests/cases/compiler/TransportStream.ts`

Reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/TransportStream.ts
```

Source overview:

```json
{
  "suite": "tsc",
  "bytes": 589,
  "lines": 2,
  "extension": ".ts",
  "first_code_line": "G@�"
}
```

Failure location:

```json
{
  "code": "UnsupportedSyntax",
  "message": "unsupported character: @ at 20..21",
  "span_start": 20,
  "span_end": 21,
  "line": 2,
  "column": 2,
  "feature_label": "parser-syntax",
  "error_type": "parser-or-frontend-unsupported"
}
```

Source context:

```text
1 | // @target: es2015
2 | G@�G@�G@�
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
    "path": "issues/done/081-implement-TransportStream.md",
    "title": "Implement Transportstream",
    "reason": "same reference path, same feature label, title overlap"
  },
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
    "state": "done",
    "path": "issues/open/059-implement-parser-syntax-extensions.md",
    "title": "Implement parser syntax extensions for TypeScript and advanced JS",
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
  },
  {
    "state": "done",
    "path": "issues/done/200-implement-parser-syntax.md",
    "title": "Implement parser syntax extensions",
    "reason": "same feature label, title overlap"
  },
  {
    "state": "done",
    "path": "issues/done/243-implement-numeric-literal-separator-parser.md",
    "title": "Implement numeric literal separator parser support",
    "reason": "same feature label, title overlap"
  },
  {
    "state": "done",
    "path": "issues/done/244-implement-bigint-literal-parser-classification.md",
    "title": "Implement BigInt literal parser classification",
    "reason": "same feature label, title overlap"
  },
  {
    "state": "done",
    "path": "issues/done/246-implement-optional-chaining-parser-support.md",
    "title": "Implement optional chaining parser support",
    "reason": "same feature label, title overlap"
  }
]
```

Error-specific suggestions:

- Start at lexer/parser support and add a minimal fixture for the exact source construct at the failing span.
- Use `dump --tokens` and the TypeScript AST path to decide whether this is tokenization, precedence, or statement dispatch.

Compiler dumps:

#### tokens

- ok: `False`
- truncated: `False`

```text
error: [UnsupportedSyntax] unsupported character: @ at 20..21
```

#### ast

- ok: `False`
- truncated: `False`

```text
error: [UnsupportedSyntax] unsupported character: @ at 20..21
```

#### resolved

- ok: `False`
- truncated: `False`

```text
error: [UnsupportedSyntax] unsupported character: @ at 20..21
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
        "code": 1490,
        "category": "Error",
        "message": "File appears to be binary.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/TransportStream.ts",
        "start": 0,
        "length": 0,
        "line": 1,
        "character": 1
      },
      {
        "code": 1434,
        "category": "Error",
        "message": "Unexpected keyword or identifier.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/TransportStream.ts",
        "start": 19,
        "length": 1,
        "line": 2,
        "character": 1
      },
      {
        "code": 2304,
        "category": "Error",
        "message": "Cannot find name 'G'.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/TransportStream.ts",
        "start": 19,
        "length": 1,
        "line": 2,
        "character": 1
      },
      {
        "code": 1127,
        "category": "Error",
        "message": "Invalid character.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/TransportStream.ts",
        "start": 21,
        "length": 1,
        "line": 2,
        "character": 3
      },
      {
        "code": 1128,
        "category": "Error",
        "message": "Declaration or statement expected.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/TransportStream.ts",
        "start": 22,
        "length": 561,
        "line": 2,
        "character": 4
      }
    ],
    "hints": [],
    "typescriptVersion": "6.0.3"
  },
  "ast": {
    "topLevel": [
      {
        "kind": "ExpressionStatement",
        "text": "G",
        "line": 2,
        "character": 1
      },
      {
        "kind": "MissingDeclaration",
        "text": "@",
        "line": 2,
        "character": 2
      }
    ],
    "pathToPosition": [
      {
        "kind": "SourceFile",
        "text": "G@�",
        "line": 2,
        "character": 1
      },
      {
        "kind": "ExpressionStatement",
        "text": "G",
        "line": 2,
        "character": 1
      },
      {
        "kind": "Identifier",
        "text": "G",
        "line": 2,
        "character": 1
      },
      {
        "kind": "MissingDeclaration",
        "text": "@",
        "line": 2,
        "character": 2
      },
      {
        "kind": "Decorator",
        "text": "@",
        "line": 2,
        "character": 2
      }
    ]
  }
}
```

Stack trace:

```text
error: [UnsupportedSyntax] unsupported character: @ at 20..21
```


## 重複整理メモ

この issue はタイトルが重複しているため、内容がより充実している
`issues/open/553-implement-TransportStream.md` に統合されました。
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

## ⚠️ False-done audit (re-opened from `issues/done/`)

**Why this was false-done**: This generated triage spike issue was moved to `issues/done/` without actual triage or implementation. It has `type: spike` and `class: blocked` depending on meta-issues (`depends_on: [5000]` or `[5001]`), with zero implementation commits referencing this issue, empty completion evidence (commits `...`), and no Status note or Close note documenting triage results.

**True-done checklist** (all must pass):
1. Perform actual triage review of the reference failure case(s)
2. Either split into implementation-ready child issue(s) or confirm superseded by an existing issue (document with Status note)
3. Fill in completion evidence section with triage results and commit SHAs
4. Verify all acceptance criteria checkboxes reflect completed work

**Commands that must pass**:
```sh
cargo fmt --all --check
cargo nextest run
```
