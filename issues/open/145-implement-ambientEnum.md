---
id: 145
title: "Implement Ambientenum"
type: spike
area: frontend/syntax
class: done
priority: P1
depends_on: []
blocks: []
created: 2026-04-29
updated: 2026-05-08
completed: 2026-05-08
status: done
---

## Summary

Triage ambientEnum across 1 failing reference test cases and split this bucket into implementation-ready child issues.

## Problem

Fresh reference evidence shows this bucket now builds successfully.

Problem: this generated bucket is no longer a build blocker; it was resolved
by the completed issue 400 ambient declaration erasure boundary. A remaining
TypeScript diagnostic parity gap was split to issue 5406.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/ambientEnum1.ts
```

Coverage window:

```sh
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/ambientEnum1.ts --detail
```

## Desired final state

This generated bucket is closed after splitting the remaining diagnostic parity
gap to `issues/open/5406-report-ambient-enum-nonconstant-initializers.md`. Do
not implement directly from this bucket.

## Scope

In scope:

- [x] Inspect the smart triage report below
- [x] Confirm whether existing open/done issues already cover this bucket
- [x] Split TS1066 ambient enum initializer parity to issue 5406
- [x] Preserve exact reproduction commands and representative AST/diagnostic evidence in the child issue

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

- [x] Duplicate candidates below are confirmed as no-match or this issue is split
- [x] Child issue contains an exact `python scripts/manager.py reference-triage ...` command
- [x] Child issue includes failing path, diagnostic code, source context, visible symbols, and parser/TypeScript AST evidence
- [x] Child issue acceptance names the exact fixture/reference path and diagnostic/stdout change

## Validation

Required commands:

```sh
python scripts/manager.py update-issue-index --check
python scripts/manager.py check-issue-health
python scripts/manager.py check-issue-readiness -- --fail-ready-below 80
git diff --check
```

Impacted commands:

```sh
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/ambientEnum1.ts --detail
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/ambientEnum1.ts
```

Not run:

- `cargo fmt --all --check`; issue cleanup only, no Rust code changed
- `cargo nextest run`; issue cleanup only, no implementation changed

## Docs / current-state / issue sync

Final-state docs:

- [x] not affected

Current state:

- [x] not affected

Follow-up issues:

- [x] created: `issues/open/5406-report-ambient-enum-nonconstant-initializers.md`

## Notes

## Affected test files

- `reference/typescript/tests/cases/compiler/ambientEnum1.ts`

## Duplicate detection

## Smart triage

Fresh triage on 2026-05-08 shows this generated ambient declaration bucket now
builds successfully:

```text
reference/typescript/tests/cases/compiler/ambientEnum1.ts: build_pass
```

Focused triage reports:

```text
BuildPass: ts2wasm build succeeded
```

Representative source context:

```ts
declare enum E1 {
    y = 4.23
}

declare enum E2 {
    x = 'foo'.length
}
```

The compiler tokenizes the ambient enum declarations and erases them from the
runtime AST/resolved output. The remaining TypeScript oracle diagnostic is
TS1066 for the non-constant ambient enum initializer; that narrower diagnostic
parity gap was split to
`issues/open/5406-report-ambient-enum-nonconstant-initializers.md`.

### Smart triage: Triage ambient declaration: ambientEnum1

- Issue class: `triage-needed`
- Feature label: `ambient-declaration`
- Diagnostic: `UnsupportedSyntax` / `parser-or-frontend-unsupported`
- Path: `reference/typescript/tests/cases/compiler/ambientEnum1.ts`

Reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/ambientEnum1.ts
```

Source overview:

```json
{
  "suite": "tsc",
  "bytes": 162,
  "lines": 9,
  "extension": ".ts",
  "first_code_line": "declare enum E1 {"
}
```

Failure location:

```json
{
  "code": "UnsupportedSyntax",
  "message": "expected Semicolon, got Some(Ident(\"enum\")) at 32..36",
  "span_start": 32,
  "span_end": 36,
  "line": 2,
  "column": 14,
  "feature_label": "ambient-declaration",
  "error_type": "parser-or-frontend-unsupported"
}
```

Source context:

```text
1 | // @target: es2015
2 |     declare enum E1 {
3 |         y = 4.23
4 |     }
5 |
```

Visible symbols before failure:

```json
[]
```

Duplicate candidates:

```json
[
  {
    "state": "done",
    "path": "issues/open/145-implement-ambientEnum.md",
    "title": "Implement Ambientenum",
    "reason": "same reference path"
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
            start: 24,
            end: 31,
        },
    },
    SpannedToken {
        kind: Ident(
            "enum",
        ),
        span: Span {
            start: 32,
            end: 36,
        },
    },
    SpannedToken {
        kind: Ident(
            "E1",
        ),
        span: Span {
            start: 37,
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
        kind: Ident(
            "y",
        ),
        span: Span {
            start: 51,
            end: 52,
        },
    },
    SpannedToken {
        kind: Equal,
        span: Span {
            start: 53,
            end: 54,
        },
    },
    SpannedToken {
        kind: Number(
            4,
        ),
        span: Span {
            start: 55,
            end: 56,
        },
    },
    SpannedToken {
        kind: Dot,
        span: Span {
            start: 56,
            end: 57,
        },
    },
    SpannedToken {
        kind: Number(
            23,
        ),
        span: Span {
            start: 57,
            end: 59,
        },
    },
    SpannedToken {
        kind: RightBrace,
        span: Span {
            start: 65,
            end: 66,
        },
    },
    SpannedToken {
        kind: Ident(
            "declare",
        ),
        span: Span {
            start: 120,
            end: 127,
        },
    },
    SpannedToken {
        kind: Ident(
            "enum",
        ),
        span: Span {
            start: 128,
            end: 132,
        },
    },
    SpannedToken {
        kind: Ident(
            "E2"
```

#### ast

- ok: `False`
- truncated: `False`

```text
error: [UnsupportedSyntax] expected Semicolon, got Some(Ident("enum")) at 32..36
```

#### resolved

- ok: `False`
- truncated: `False`

```text
error: [UnsupportedSyntax] expected Semicolon, got Some(Ident("enum")) at 32..36
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
        "code": 1066,
        "category": "Error",
        "message": "In ambient enum declarations member initializer must be constant expression.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/ambientEnum1.ts",
        "start": 151,
        "length": 12,
        "line": 8,
        "character": 13
      }
    ],
    "hints": [],
    "typescriptVersion": "6.0.3"
  },
  "ast": {
    "topLevel": [
      {
        "kind": "EnumDeclaration",
        "text": "declare enum E1 {\r\n        y = 4.23\r\n    }",
        "line": 2,
        "character": 5
      },
      {
        "kind": "EnumDeclaration",
        "text": "declare enum E2 {\r\n        x = 'foo'.length\r\n    }",
        "line": 7,
        "character": 5
      }
    ],
    "pathToPosition": [
      {
        "kind": "SourceFile",
        "text": "declare enum E1 {\r\n        y = 4.23\r\n    }\r\n    \r\n    // Ambient enum with computer member\r\n    declare enum E2 {\r\n     ",
        "line": 2,
        "character": 5
      },
      {
        "kind": "EnumDeclaration",
        "text": "declare enum E1 {\r\n        y = 4.23\r\n    }",
        "line": 2,
        "character": 5
      }
    ]
  }
}
```

Stack trace:

```text
error: [UnsupportedSyntax] expected Semicolon, got Some(Ident("enum")) at 32..36
```

## Completion evidence

Closed after fresh triage confirmed the generated ambient enum build blocker is
resolved by issue 400 and the remaining TypeScript diagnostic parity gap is
split to `issues/open/5406-report-ambient-enum-nonconstant-initializers.md`.

Fresh coverage:

```text
command: env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/ambientEnum1.ts --detail --no-dashboard-data
result: pass; executed=1, build_pass=1, unsupported=0
date: 2026-05-08
```

Fresh triage:

```text
command: env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/ambientEnum1.ts
result: pass; BuildPass; runtime AST/resolved output erased the ambient enum declarations
date: 2026-05-08
```

The TypeScript oracle still reports TS1066 for
`declare enum E2 { x = 'foo'.length }`; that diagnostic parity gap remains open
in issue 5406.

Commits:

- local issue cleanup commit that moves issue 145 to done and creates issue 5406

Validation result:

```text
command: python scripts/manager.py update-issue-index --check
result: pass
date: 2026-05-08

command: python scripts/manager.py check-issue-health
result: pass
date: 2026-05-08

command: python scripts/manager.py check-issue-readiness -- --fail-ready-below 80
result: pass
date: 2026-05-08

command: git diff --check
result: pass
date: 2026-05-08
```

Remaining risks:

- TS1066 parity remains open in issue 5406.

## False-done audit resolution

This issue was re-triaged on 2026-05-08 with fresh coverage and smart triage
evidence. The generated bucket is now closed because the original parser/build
blocker is fixed by issue 400, and the narrower remaining diagnostic mismatch
has its own implementation-ready child issue.
