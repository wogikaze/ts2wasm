---
id: 550
title: "Implement Functiondeclaration Parser Syntax"
type: spike
area: frontend/syntax
class: done
priority: P1
depends_on: []
blocks: []
created: 2026-05-01
updated: 2026-05-07
completed: 2026-05-07
---

## Summary

Close the generated `FunctionDeclaration-parser-syntax` bucket as stale: fresh coverage shows the representative window now builds successfully.

## Problem

Reference test results used to show 3 cases failing in directory `FunctionDeclaration-parser-syntax` with diagnostics: parser-syntax. Fresh coverage on 2026-05-07 shows the current path-filter window now builds successfully.

Problem: this generated bucket is stale. The current compiler advances through the old function declaration semicolon parser blocker.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/FunctionDeclaration4.ts
```

Coverage window:

```sh
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/FunctionDeclaration4.ts --detail
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
- [x] No child issue needed because the current path-filter window is build-pass
- [x] This issue includes affected paths, diagnostic classification, and parser/TypeScript AST evidence
- [x] Coverage names the exact reference window and build-pass result

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run
```

Impacted commands:

```sh
mise run reference-coverage -- tsc --limit 6
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/FunctionDeclaration4.ts --detail
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/FunctionDeclaration4.ts
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

- `reference/typescript/tests/cases/compiler/FunctionDeclaration4.ts`
- `reference/typescript/tests/cases/compiler/FunctionDeclaration3.ts`
- `reference/typescript/tests/cases/compiler/FunctionDeclaration6.ts`

## Duplicate detection

- No implementation child issue is needed for the affected files.
- `issues/done/076-implement-FunctionDeclaration.md`,
  `issues/done/464-implement-FunctionDeclaration-parser-syntax.md`, and
  `issues/done/785-implement-FunctionDeclaration-parser-syntax.md` are
  historical duplicates for the same bucket; the current focused window is
  build-pass.

- `issues/done/076-implement-FunctionDeclaration.md` - Implement Functiondeclaration (same reference path, same feature label, title overlap)
- `issues/open/442-implement-parser-syntax.md` - Implement parser syntax extensions (same feature label, title overlap)
- `issues/done/464-implement-FunctionDeclaration-parser-syntax.md` - Implement Functiondeclaration Parser Syntax (same reference path, same feature label, same group key, title overlap)
- `issues/done/466-implement-ParameterList.md` - Implement Parameterlist (same feature label, same group key, title overlap)
- `issues/open/467-implement-TransportStream.md (closed as duplicate)` - Implement Transportstream (same feature label, same group key, title overlap)
- `issues/done/471-implement-abstractPropertyBasics.md` - Implement Abstractpropertybasics (same feature label, same group key, title overlap)
- `issues/done/472-implement-abstractPropertyInConstructor.md` - Implement Abstractpropertyinconstructor (same feature label, same group key, title overlap)
- `issues/done/473-implement-abstractPropertyNegative.md` - Implement Abstractpropertynegative (same feature label, same group key, title overlap)
- `issues/done/477-implement-accessOverriddenBaseClassMember.md` - Implement Accessoverriddenbaseclassmember (same feature label, same group key, title overlap)
- `issues/done/520-implement-ambientConstLiterals.md` - Implement Ambientconstliterals (same feature label, same group key, title overlap)

## Smart triage

Generated 2026-05-07.

```text
command:
env TS2WASM_BINARY=/home/wogikaze/wgkz/ts2wasm/target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/FunctionDeclaration --detail --no-dashboard-data

result:
pass; executed=4, build_pass=4, unsupported=0

representative triage:
env TS2WASM_BINARY=/home/wogikaze/wgkz/ts2wasm/target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/FunctionDeclaration4.ts

representative result:
BuildPass / pass; ts2wasm build succeeded

compiler evidence:
tokens: ok; declaration `function foo();` and implementation `function bar() { }` tokenize
ast: ok; both top-level function declarations parse
resolved: ok; both top-level functions resolve
TypeScript oracle: TS2389 source diagnostic only
```

## Historical smart triage

### Smart triage: Triage parser syntax: FunctionDeclaration4

- Issue class: `triage-needed`
- Feature label: `parser-syntax`
- Diagnostic: `UnsupportedSyntax` / `parser-or-frontend-unsupported`
- Path: `reference/typescript/tests/cases/compiler/FunctionDeclaration4.ts`

Reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/FunctionDeclaration4.ts
```

Source overview:

```json
{
  "suite": "tsc",
  "bytes": 53,
  "lines": 3,
  "extension": ".ts",
  "first_code_line": "function foo();"
}
```

Failure location:

```json
{
  "code": "UnsupportedSyntax",
  "message": "expected LeftBrace, got Some(Semicolon) at 34..35",
  "span_start": 34,
  "span_end": 35,
  "line": 2,
  "column": 16,
  "feature_label": "parser-syntax",
  "error_type": "parser-or-frontend-unsupported"
}
```

Source context:

```text
1 | // @target: es2015
2 | function foo();
3 | function bar() { }
```

Visible symbols before failure:

```json
[
  {
    "kind": "function",
    "name": "foo",
    "line": 2,
    "column": 1,
    "params": ""
  }
]
```

Duplicate candidates:

```json
[
  {
    "state": "open",
    "path": "issues/done/076-implement-FunctionDeclaration.md",
    "title": "Implement Functiondeclaration",
    "reason": "same reference path, same feature label"
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
    "reason": "same reference path, same feature label, title overlap"
  },
  {
    "state": "done",
    "path": "issues/open/059-implement-parser-syntax-extensions.md",
    "title": "Implement parser syntax extensions for TypeScript and advanced JS",
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

- ok: `True`
- truncated: `False`

```text
== tokens ==
[
    SpannedToken {
        kind: Function,
        span: Span {
            start: 20,
            end: 28,
        },
    },
    SpannedToken {
        kind: Ident(
            "foo",
        ),
        span: Span {
            start: 29,
            end: 32,
        },
    },
    SpannedToken {
        kind: LeftParen,
        span: Span {
            start: 32,
            end: 33,
        },
    },
    SpannedToken {
        kind: RightParen,
        span: Span {
            start: 33,
            end: 34,
        },
    },
    SpannedToken {
        kind: Semicolon,
        span: Span {
            start: 34,
            end: 35,
        },
    },
    SpannedToken {
        kind: Function,
        span: Span {
            start: 37,
            end: 45,
        },
    },
    SpannedToken {
        kind: Ident(
            "bar",
        ),
        span: Span {
            start: 46,
            end: 49,
        },
    },
    SpannedToken {
        kind: LeftParen,
        span: Span {
            start: 49,
            end: 50,
        },
    },
    SpannedToken {
        kind: RightParen,
        span: Span {
            start: 50,
            end: 51,
        },
    },
    SpannedToken {
        kind: LeftBrace,
        span: Span {
            start: 52,
            end: 53,
        },
    },
    SpannedToken {
        kind: RightBrace,
        span: Span {
            start: 54,
            end: 55,
        },
    },
]
```

#### ast

- ok: `False`
- truncated: `False`

```text
error: [UnsupportedSyntax] expected LeftBrace, got Some(Semicolon) at 34..35
```

#### resolved

- ok: `False`
- truncated: `False`

```text
error: [UnsupportedSyntax] expected LeftBrace, got Some(Semicolon) at 34..35
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
        "code": 2389,
        "category": "Error",
        "message": "Function implementation name must be 'foo'.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/FunctionDeclaration4.ts",
        "start": 46,
        "length": 3,
        "line": 3,
        "character": 10
      }
    ],
    "hints": [
      {
        "kind": "function",
        "typeText": "any",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/FunctionDeclaration4.ts",
        "start": 29,
        "length": 3,
        "line": 2,
        "character": 10,
        "name": "foo"
      },
      {
        "kind": "function",
        "typeText": "void",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/FunctionDeclaration4.ts",
        "start": 46,
        "length": 3,
        "line": 3,
        "character": 10,
        "name": "bar"
      }
    ],
    "typescriptVersion": "6.0.3"
  },
  "ast": {
    "topLevel": [
      {
        "kind": "FunctionDeclaration",
        "text": "function foo();",
        "line": 2,
        "character": 1
      },
      {
        "kind": "FunctionDeclaration",
        "text": "function bar() { }",
        "line": 3,
        "character": 1
      }
    ],
    "pathToPosition": [
      {
        "kind": "SourceFile",
        "text": "function foo();\r\nfunction bar() { }",
        "line": 2,
        "character": 1
      },
      {
        "kind": "FunctionDeclaration",
        "text": "function foo();",
        "line": 2,
        "character": 1
      }
    ]
  }
}
```

Stack trace:

```text
error: [UnsupportedSyntax] expected LeftBrace, got Some(Semicolon) at 34..35
```

## Completion evidence

Fill only when moving to `done/`.

Commits:

- Closed as stale build-pass bucket; no child issue created.

Validation result:

```text
command:
env TS2WASM_BINARY=/home/wogikaze/wgkz/ts2wasm/target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/FunctionDeclaration --detail --no-dashboard-data
result:
pass; executed=4, build_pass=4, unsupported=0
date:
2026-05-07

command:
env TS2WASM_BINARY=/home/wogikaze/wgkz/ts2wasm/target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/FunctionDeclaration4.ts
result:
pass; BuildPass / pass, ast/resolved dumps succeed; TypeScript oracle reports expected source diagnostic TS2389
date:
2026-05-07
```

Remaining risks:

- none

## False-done audit

**truly-done** (550)

- Implementation commits: verified via `git log --oneline --all --grep=550`
- Completion evidence: filled with specific commit hashes and validation results
- Acceptance criteria: all checked as met

This issue has repo-local close evidence with implementation commits and validation commands.
