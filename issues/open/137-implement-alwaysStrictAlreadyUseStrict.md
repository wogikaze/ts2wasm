---
id: 137
title: "Implement Alwaysstrictalreadyusestrict"
type: spike
area: frontend/syntax
class: done
priority: P1
depends_on: []
blocks: []
created: 2026-04-29
updated: 2026-05-07
completed: 2026-05-07
---

## Summary

Close alwaysStrictAlreadyUseStrict as a stale generated blocker: fresh triage shows the affected reference file now builds successfully.

## Problem

Reference test results used to show 1 case failing in directory `alwaysStrictAlreadyUseStrict` with diagnostics: parser-syntax. Fresh triage on 2026-05-07 shows the affected file now builds successfully and TypeScript reports no oracle diagnostics.

Problem: alwaysStrictAlreadyUseStrict is stale. The current compiler parses the `"use strict"` directive prologue without a semicolon before the following function declaration.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/alwaysStrictAlreadyUseStrict.ts
```

Coverage window:

```sh
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/alwaysStrictAlreadyUseStrict.ts --detail
```

## Desired final state

This generated bucket is either split into implementation-ready child issues or superseded by an existing open/done issue with matching evidence. Do not implement directly from this bucket.

## Scope

In scope:

- [x] Inspect the smart triage report below
- [x] Confirm whether existing open/done issues already cover this bucket
- [x] Close as stale build-pass bucket
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

- [x] Duplicate candidates below are confirmed as no-match for required implementation work
- [x] No child issue needed because the affected file is build-pass
- [x] This issue includes affected path, diagnostic classification, and parser/TypeScript AST evidence
- [x] Coverage names the exact reference window and build-pass result

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run
```

Impacted commands:

```sh
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/alwaysStrictAlreadyUseStrict.ts --detail
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/alwaysStrictAlreadyUseStrict.ts
```

Not run:

- `cargo fmt --all --check` and `cargo nextest run`; this close is an
  issue-lifecycle-only stale build-pass update, so focused reference and issue
  checks were used instead.

## Docs / current-state / issue sync

Final-state docs:

- [x] not affected

Current state:

- [x] not affected

Follow-up issues:

- [x] none

## Notes

## Affected test files

- `reference/typescript/tests/cases/compiler/alwaysStrictAlreadyUseStrict.ts`

## Duplicate detection

- No implementation child issue is needed for the affected file.
- `issues/open/059-implement-parser-syntax-extensions.md` and historical parser
  syntax buckets are broad same-label candidates, but the exact
  `alwaysStrictAlreadyUseStrict.ts` reference now builds successfully.

## Smart triage

Generated 2026-05-07.

```text
command:
env TS2WASM_BINARY=/home/wogikaze/wgkz/ts2wasm/target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/alwaysStrictAlreadyUseStrict.ts

result:
BuildPass / pass; ts2wasm build succeeded

compiler evidence:
tokens: ok; String("use strict") at 44..56 followed by Function at 58..66
ast: ok; top-level Expr(String("use strict")) followed by Function f()
resolved: ok; top-level Expr(String("use strict")) followed by Function f()
TypeScript oracle: ok; diagnostics []
```

## Historical smart triage

### Smart triage: Triage parser syntax: alwaysStrictAlreadyUseStrict

- Issue class: `triage-needed`
- Feature label: `parser-syntax`
- Diagnostic: `UnsupportedSyntax` / `parser-or-frontend-unsupported`
- Path: `reference/typescript/tests/cases/compiler/alwaysStrictAlreadyUseStrict.ts`

Reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/alwaysStrictAlreadyUseStrict.ts
```

Source overview:

```json
{
  "suite": "tsc",
  "bytes": 87,
  "lines": 6,
  "extension": ".ts",
  "first_code_line": "\"use strict\""
}
```

Failure location:

```json
{
  "code": "UnsupportedSyntax",
  "message": "expected Semicolon, got Some(Function) at 58..66",
  "span_start": 58,
  "span_end": 66,
  "line": 4,
  "column": 4,
  "feature_label": "parser-syntax",
  "error_type": "parser-or-frontend-unsupported"
}
```

Source context:

```text
1 | // @target: es2015
2 | // @alwaysStrict: true
3 | "use strict"
4 | function f() {
5 |     var a = [];
6 | }
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
    "path": "issues/open/137-implement-alwaysStrictAlreadyUseStrict.md",
    "title": "Implement Alwaysstrictalreadyusestrict",
    "reason": "same reference path, same feature label, title overlap"
  },
  {
    "state": "open",
    "path": "issues/open/200-implement-parser-syntax.md",
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
    "path": "issues/open/065a-merge-duplicate-parser-syntax-issue-into-059.md",
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
        kind: String(
            "use strict",
        ),
        span: Span {
            start: 44,
            end: 56,
        },
    },
    SpannedToken {
        kind: Function,
        span: Span {
            start: 58,
            end: 66,
        },
    },
    SpannedToken {
        kind: Ident(
            "f",
        ),
        span: Span {
            start: 67,
            end: 68,
        },
    },
    SpannedToken {
        kind: LeftParen,
        span: Span {
            start: 68,
            end: 69,
        },
    },
    SpannedToken {
        kind: RightParen,
        span: Span {
            start: 69,
            end: 70,
        },
    },
    SpannedToken {
        kind: LeftBrace,
        span: Span {
            start: 71,
            end: 72,
        },
    },
    SpannedToken {
        kind: Var,
        span: Span {
            start: 78,
            end: 81,
        },
    },
    SpannedToken {
        kind: Ident(
            "a",
        ),
        span: Span {
            start: 82,
            end: 83,
        },
    },
    SpannedToken {
        kind: Equal,
        span: Span {
            start: 84,
            end: 85,
        },
    },
    SpannedToken {
        kind: LeftBracket,
        span: Span {
            start: 86,
            end: 87,
        },
    },
    SpannedToken {
        kind: RightBracket,
        span: Span {
            start: 87,
            end: 88,
        },
    },
    SpannedToken {
        kind: Semicolon,
        span: Span {
            start: 88,
            end: 89,
        },
    },
    SpannedToken {
        kind: RightBrace,
        span: Span {
            start: 91,
            end: 92,
        },
    },
]
```

#### ast

- ok: `False`
- truncated: `False`

```text
error: [UnsupportedSyntax] expected Semicolon, got Some(Function) at 58..66
```

#### resolved

- ok: `False`
- truncated: `False`

```text
error: [UnsupportedSyntax] expected Semicolon, got Some(Function) at 58..66
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
        "typeText": "void",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/alwaysStrictAlreadyUseStrict.ts",
        "start": 67,
        "length": 1,
        "line": 4,
        "character": 10,
        "name": "f"
      },
      {
        "kind": "binding",
        "typeText": "never[]",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/alwaysStrictAlreadyUseStrict.ts",
        "start": 82,
        "length": 1,
        "line": 5,
        "character": 9,
        "name": "a"
      }
    ],
    "typescriptVersion": "6.0.3"
  },
  "ast": {
    "topLevel": [
      {
        "kind": "ExpressionStatement",
        "text": "\"use strict\"",
        "line": 3,
        "character": 1
      },
      {
        "kind": "FunctionDeclaration",
        "text": "function f() {\r\n    var a = [];\r\n}",
        "line": 4,
        "character": 1
      }
    ],
    "pathToPosition": [
      {
        "kind": "SourceFile",
        "text": "\"use strict\"\r\nfunction f() {\r\n    var a = [];\r\n}",
        "line": 3,
        "character": 1
      },
      {
        "kind": "FunctionDeclaration",
        "text": "function f() {\r\n    var a = [];\r\n}",
        "line": 4,
        "character": 1
      }
    ]
  }
}
```

Stack trace:

```text
error: [UnsupportedSyntax] expected Semicolon, got Some(Function) at 58..66
```

## Completion evidence

Fill only when moving to `done/`.

Commits:

- Closed as stale build-pass bucket; no child issue created.

Validation result:

```text
command:
env TS2WASM_BINARY=/home/wogikaze/wgkz/ts2wasm/target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/alwaysStrictAlreadyUseStrict.ts --detail --no-dashboard-data
result:
pass; executed=1, build_pass=1, unsupported=0
date:
2026-05-07

command:
env TS2WASM_BINARY=/home/wogikaze/wgkz/ts2wasm/target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/alwaysStrictAlreadyUseStrict.ts
result:
pass; BuildPass / pass, ast/resolved dumps succeed, TypeScript oracle diagnostics []
date:
2026-05-07
```

Remaining risks:

- none
---

## ⚠️ False-done audit (re-opened from issues/open/)

**Why this was false-done**: This generated triage spike issue was copy-closed to `issues/open/` as part of a batch close cycle without actual triage completion. The old done/ copy only changed checkbox state without adding a status note, child issues, implementation commits, or completion evidence. This pass fills the missing evidence and closes it as stale build-pass.

**True-done checklist** (all must pass):

1. [x] Perform actual triage review of the reference failure case
2. [x] Either create child implementation issue(s) or confirm this issue is superseded by an existing issue (with "Status" note)
3. [x] Fill in completion evidence section with triage results
4. [x] Remove stale open/ copy if it exists

**Commands that must pass**:

```sh
cargo fmt --all --check
cargo nextest run
```
