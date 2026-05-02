---
id: 133
title: "Implement Allowjscrossmonorepopackage"
type: spike
area: frontend/syntax
class: blocked
priority: P2
depends_on: [5007]
blocks: []
created: 2026-04-29
updated: 2026-04-29
---

## Summary

Triage allowJsCrossMonorepoPackage across 1 failing reference test cases and split this bucket into implementation-ready child issues.

## Problem

Reference test results show 1 cases fail in directory `allowJsCrossMonorepoPackage` with diagnostics: module-resolution. The compiler cannot handle these syntax/semantics, preventing compilation of code in this category.

Problem: allowJsCrossMonorepoPackage has 1 reference failures and needs smart-triage evidence before implementation starts.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/allowJsCrossMonorepoPackage.ts
```

Coverage window:

```sh
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/allowJsCrossMonorepoPackage.ts --detail
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
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/allowJsCrossMonorepoPackage.ts --detail
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/allowJsCrossMonorepoPackage.ts
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

- `reference/typescript/tests/cases/compiler/allowJsCrossMonorepoPackage.ts`

## Duplicate detection

## Smart triage

### Smart triage: Triage module resolution: allowJsCrossMonorepoPackage

- Issue class: `triage-needed`
- Feature label: `module-resolution`
- Diagnostic: `UnsupportedSyntax` / `parser-or-frontend-unsupported`
- Path: `reference/typescript/tests/cases/compiler/allowJsCrossMonorepoPackage.ts`

Reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/allowJsCrossMonorepoPackage.ts
```

Source overview:

```json
{
  "suite": "tsc",
  "bytes": 970,
  "lines": 45,
  "extension": ".ts",
  "first_code_line": "export declare function pkg(): \"pkg\";"
}
```

Failure location:

```json
{
  "code": "UnsupportedSyntax",
  "message": "issue-055: unsupported static export; module resolution and loading are not implemented at 62..68",
  "span_start": 62,
  "span_end": 68,
  "line": 3,
  "column": 1,
  "feature_label": "module-resolution",
  "error_type": "parser-or-frontend-unsupported"
}
```

Source context:

```text
1 | // @target: es2015
2 | // @Filename: /node_modules/pkg/index.d.ts
3 | export declare function pkg(): "pkg";
4 |
5 | // @Filename: /packages/shared/package.json
6 | {
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
    "path": "issues/open/133-implement-allowJsCrossMonorepoPackage.md",
    "title": "Implement Allowjscrossmonorepopackage",
    "reason": "same reference path, title overlap"
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
        kind: Export,
        span: Span {
            start: 62,
            end: 68,
        },
    },
    SpannedToken {
        kind: Ident(
            "declare",
        ),
        span: Span {
            start: 69,
            end: 76,
        },
    },
    SpannedToken {
        kind: Function,
        span: Span {
            start: 77,
            end: 85,
        },
    },
    SpannedToken {
        kind: Ident(
            "pkg",
        ),
        span: Span {
            start: 86,
            end: 89,
        },
    },
    SpannedToken {
        kind: LeftParen,
        span: Span {
            start: 89,
            end: 90,
        },
    },
    SpannedToken {
        kind: RightParen,
        span: Span {
            start: 90,
            end: 91,
        },
    },
    SpannedToken {
        kind: Colon,
        span: Span {
            start: 91,
            end: 92,
        },
    },
    SpannedToken {
        kind: String(
            "pkg",
        ),
        span: Span {
            start: 93,
            end: 98,
        },
    },
    SpannedToken {
        kind: Semicolon,
        span: Span {
            start: 98,
            end: 99,
        },
    },
    SpannedToken {
        kind: LeftBrace,
        span: Span {
            start: 145,
            end: 146,
        },
    },
    SpannedToken {
        kind: String(
            "name",
        ),
        span: Span {
            start: 151,
            end: 157,
        },
    },
    SpannedToken {
        kind: Colon,
        span: Span {
            start: 157,
            end: 158,
        },
    },
    SpannedToken {
        kind: String(
            "shared",
        ),
        span: Span {
            start: 159,
            end: 167,
        },
```

#### ast

- ok: `False`
- truncated: `False`

```text
error: [UnsupportedSyntax] issue-055: unsupported static export; module resolution and loading are not implemented at 62..68
```

#### resolved

- ok: `False`
- truncated: `False`

```text
error: [UnsupportedSyntax] issue-055: unsupported static export; module resolution and loading are not implemented at 62..68
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
        "code": 1005,
        "category": "Error",
        "message": "';' expected.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/allowJsCrossMonorepoPackage.ts",
        "start": 157,
        "length": 1,
        "line": 7,
        "character": 11
      },
      {
        "code": 2695,
        "category": "Error",
        "message": "Left side of comma operator is unused and has no side effects.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/allowJsCrossMonorepoPackage.ts",
        "start": 159,
        "length": 8,
        "line": 7,
        "character": 13
      },
      {
        "code": 1005,
        "category": "Error",
        "message": "';' expected.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/allowJsCrossMonorepoPackage.ts",
        "start": 182,
        "length": 1,
        "line": 8,
        "character": 14
      },
      {
        "code": 2695,
        "category": "Error",
        "message": "Left side of comma operator is unused and has no side effects.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/allowJsCrossMonorepoPackage.ts",
        "start": 184,
        "length": 7,
        "line": 8,
        "character": 16
      },
      {
        "code": 1005,
        "category": "Error",
        "message": "';' expected.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/allowJsCrossMonorepoPackage.ts",
        "start": 203,
        "length": 1,
        "line": 9,
        "character": 11
      },
      {
        "code": 2695,
        "category": "Error",
        "message": "Left side of comma operator is unused and has no side effects.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/allowJsCrossMonorepoPackage.ts",
        "start": 205,
        "length": 8,
        "line": 9,
        "character": 13
      },
      {
        "code": 1005,
        "category": "Error",
        "message": "';' expected.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/allowJsCrossMonorepoPackage.ts",
        "start": 228,
        "length": 1,
        "line": 10,
        "character": 14
      },
      {
        "code": 2307,
        "category": "Error",
        "message": "Cannot find module 'pkg' or its corresponding type declarations.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/allowJsCrossMonorepoPackage.ts",
        "start": 306,
        "length": 5,
        "line": 14,
        "character": 21
      },
      {
        "code": 2307,
        "category": "Error",
        "message": "Cannot find module './utils.js' or its corresponding type declarations.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/allowJsCrossMonorepoPackage.ts",
        "start": 374,
        "length": 12,
        "line": 17,
        "character": 21
      },
      {
        "code": 2395,
        "category": "Error",
        "message": "Individual declarations in merged declaration 'x' must be all exported or all local.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/allowJsCrossMonorepoPackage.ts",
        "start": 401,
        "length": 1,
        "line": 18,
        "character": 14
      },
      {
        "code": 1005,
        "category": "Error",
        "message": "';' expected.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/allowJsCrossMonorepoPackage.ts",
        "start": 467,
        "length": 1,
        "line": 22,
        "character": 11
      },
      {
        "code": 2695,
        "category": "Error",
        "message": "Left side of comma operator is unused and has no side effects.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/allowJsCrossMonorepoPackage.ts",
        "start": 469,
        "length": 6,
        "line": 22,
        "character": 13
      },
      {
        "code": 1005,
        "category": "Error",
        "message": "';' expected.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/allowJsCrossMonorepoPackage.ts",
        "start": 490,
        "length": 1,
        "line": 23,
        "character": 14
      },
      {
        "code": 2695,
        "category": "Error",
        "message": "Left side of comma operator is unused and has no side effects.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/allowJsCrossMonorepoPackage.ts",
        "start": 492,
        "length": 7,
        "line": 23,
        "character": 16
      },
      {
        "code": 1005,
        "category": "Error",
        "message": "';' expected.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/allowJsCrossMonorepoPackage.ts",
        "start": 511,
        "length": 1,
        "line": 24,
        "character": 11
      },
      {
        "code": 2695,
        "category": "Error",
        "message": "Left side of comma operator is unused and has no side effects.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/allowJsCrossMonorepoPackage.ts",
        "start": 513,
        "length": 8,
        "line": 24,
        "character": 13
      },
      {
        "code": 1005,
        "category": "Error",
        "message": "';' expected.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/allowJsCrossMonorepoPackage.ts",
        "start": 541,
        "length": 1,
        "line": 25,
        "character": 19
      },
      {
        "code": 1005,
        "category": "Error",
        "message": "';' expected.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/allowJsCrossMonorepoPackage.ts",
        "start": 561,
        "length": 1,
        "line": 26,
        "character": 17
      },
      {
        "code": 1005,
        "category": "Error",
        "message": "';' expected.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/allowJsCrossMonorepoPackage.ts",
        "start": 652,
        "length": 1,
        "line": 32,
        "character": 22
      },
      {
        "code": 1005,
        "category": "Error",
        "message": "';' expected.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/allowJsCrossMonorepoPackage.ts",
        "start": 672,
        "length": 1,
        "line": 33,
        "character": 17
      },
      {
        "code": 2695,
        "category": "Error",
        "message": "Left side of comma operator is unused and has no side effects.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/allowJsCrossMonorepoPackage.ts",
        "start": 674,
        "length": 4,
        "line": 33,
        "character": 19
      },
      {
        "code": 1005,
        "category": "Error",
        "message": "';' expected.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/allowJsCrossMonorepoPackage.ts",
        "start": 697,
        "length": 1,
        "line": 34,
        "character": 18
      },
      {
        "code": 2695,
        "category": "Error",
        "message": "Left side of comma operator is unused and has no side effects.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/allowJsCrossMonorepoPackage.ts",
        "start": 699,
        "length": 4,
        "line": 34,
        "character": 20
      },
      {
        "code": 1005,
        "category": "Error",
        "message": "';' expected.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/allowJs
```

Stack trace:

```text
error: [UnsupportedSyntax] issue-055: unsupported static export; module resolution and loading are not implemented at 62..68
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


---

## ⚠️ False-done audit (re-opened from issues/done/)

**Why this was false-done**: This is a generated triage bucket issue. It was
created as a `class: blocked` spike with `depends_on` pointing to a parent
meta-issue (5004 or 5007). When the parent meta-issue was moved to
`issues/done/`, this child issue was dragged along without any implementation
or triage work. The `## Completion evidence` section is unfilled (commits
placeholder `...`, validation result empty). Zero implementation commits
reference this issue.

**True-done checklist** (all must pass):

1. **Triage the representative failure path**: Confirm it is superseded by an
   existing open/done issue OR split into implementation-ready child issues
   with exact reproduction commands.

2. **Commands that must pass**:
   ```sh
   cargo fmt --all --check
   cargo nextest run
   ```

3. **Specific evidence needed**:
   - Issue URL or child issue path documenting the triage outcome
   - Or: the exact failing reference path has a matching open/done issue
   - Or: the failing test case no longer reproduces the original diagnostic

