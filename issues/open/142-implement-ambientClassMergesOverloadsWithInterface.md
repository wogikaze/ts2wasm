---
id: 142
title: "Implement Ambientclassmergesoverloadswithinterface"
type: spike
area: frontend/syntax
class: superseded
priority: P1
depends_on: []
blocks: []
created: 2026-04-29
updated: 2026-05-08
completed: 2026-05-08
status: done
---

## Summary

Triage ambientClassMergesOverloadsWithInterface across 1 failing reference test cases and split this bucket into implementation-ready child issues.

## Problem

Fresh reference evidence shows this bucket now builds successfully.

Problem: this generated bucket is no longer a blocker; it was resolved by the
completed issue 400 ambient declaration erasure boundary.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/ambientClassMergesOverloadsWithInterface.ts
```

Coverage window:

```sh
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/ambientClassMergesOverloadsWithInterface.ts --detail
```

## Desired final state

This generated bucket is superseded by
`issues/done/400-implement-ambient-declaration-erasure-boundary.md`. Do not
implement directly from this bucket.

## Scope

In scope:

- [x] Inspect the smart triage report below
- [x] Confirm whether existing open/done issues already cover this bucket
- [x] Supersede this bucket with issue 400's ambient declaration erasure boundary
- [x] Preserve exact reproduction commands and representative build-pass evidence in this closed issue

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
- [x] This closed issue contains an exact `python scripts/manager.py reference-triage ...` command
- [x] This closed issue includes failing path, diagnostic code, source context, visible symbols, and parser/TypeScript AST evidence
- [x] Completion evidence names the exact fixture/reference path and diagnostic/stdout change

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
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/ambientClassMergesOverloadsWithInterface.ts --detail
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/ambientClassMergesOverloadsWithInterface.ts
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

- [x] none

## Notes

## Affected test files

- `reference/typescript/tests/cases/compiler/ambientClassMergesOverloadsWithInterface.ts`

## Duplicate detection

## Smart triage

Fresh triage on 2026-05-08 shows this generated ambient declaration bucket now
builds successfully:

```text
reference/typescript/tests/cases/compiler/ambientClassMergesOverloadsWithInterface.ts: build_pass
```

Focused triage reports:

```text
BuildPass: ts2wasm build succeeded
```

Representative source context:

```ts
declare class C {
    baz(): any;
    foo(n: number): any;
}
interface C {
    foo(n: number): any;
    bar(): any;
}
```

The compiler tokenizes the ambient class and interface declarations, erases
them from the runtime AST, and the resolved dump is empty. TypeScript accepts
the file with no diagnostics. This is covered by
`issues/done/400-implement-ambient-declaration-erasure-boundary.md`.

### Smart triage: Triage ambient declaration: ambientClassMergesOverloadsWithInterface

- Issue class: `triage-needed`
- Feature label: `ambient-declaration`
- Diagnostic: `UnsupportedSyntax` / `parser-or-frontend-unsupported`
- Path: `reference/typescript/tests/cases/compiler/ambientClassMergesOverloadsWithInterface.ts`

Reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/ambientClassMergesOverloadsWithInterface.ts
```

Source overview:

```json
{
  "suite": "tsc",
  "bytes": 140,
  "lines": 9,
  "extension": ".ts",
  "first_code_line": "﻿// @target: es2015"
}
```

Failure location:

```json
{
  "code": "UnsupportedSyntax",
  "message": "expected Semicolon, got Some(Class) at 31..36",
  "span_start": 31,
  "span_end": 36,
  "line": 2,
  "column": 12,
  "feature_label": "ambient-declaration",
  "error_type": "parser-or-frontend-unsupported"
}
```

Source context:

```text
1 | ﻿// @target: es2015
2 | declare class C {
3 |     baz(): any;
4 |     foo(n: number): any;
5 | }
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
    "path": "issues/done/142-implement-ambientClassMergesOverloadsWithInterface.md",
    "title": "Implement Ambientclassmergesoverloadswithinterface",
    "reason": "same reference path, title overlap"
  }
]
```

Error-specific suggestions:

- Start at lexer/parser support and add a minimal fixture for the exact source construct at the failing span.
- Use `dump --tokens` and the TypeScript AST path to decide whether this is tokenization, precedence, or statement dispatch.

Automatic repair sketch:

```rust
// Rough sketch only: make class syntax observable before lowering full semantics.
// Candidate source class: C
#[derive(Debug, Clone, PartialEq)]
pub struct ClassDecl {
    pub name: String,
    pub constructor: Option<FunctionDecl>,
    pub methods: Vec<MethodDecl>,
    pub span: Span,
}

fn class_statement(&mut self) -> Result<Stmt, Diagnostic> {
    let span = self.expect(TokenKind::Class)?;
    let name = self.expect_ident()?;
    self.expect(TokenKind::LeftBrace)?;
    let mut methods = Vec::new();
    while !self.consume(TokenKind::RightBrace) {
        methods.push(self.class_method()?);
    }
    Ok(Stmt::ClassDecl(ClassDecl { name, constructor: None, methods, span }))
}
```

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
            start: 23,
            end: 30,
        },
    },
    SpannedToken {
        kind: Class,
        span: Span {
            start: 31,
            end: 36,
        },
    },
    SpannedToken {
        kind: Ident(
            "C",
        ),
        span: Span {
            start: 37,
            end: 38,
        },
    },
    SpannedToken {
        kind: LeftBrace,
        span: Span {
            start: 39,
            end: 40,
        },
    },
    SpannedToken {
        kind: Ident(
            "baz",
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
        kind: Colon,
        span: Span {
            start: 51,
            end: 52,
        },
    },
    SpannedToken {
        kind: Ident(
            "any",
        ),
        span: Span {
            start: 53,
            end: 56,
        },
    },
    SpannedToken {
        kind: Semicolon,
        span: Span {
            start: 56,
            end: 57,
        },
    },
    SpannedToken {
        kind: Ident(
            "foo",
        ),
        span: Span {
            start: 63,
            end: 66,
        },
    },
    SpannedToken {
        kind: LeftParen,
        span: Span {
            start: 66,
            end: 67,
        },
    },
    SpannedToken {
        kind: Ident(
            "n",
        ),
        span: Span {
            start: 67,
            end: 68,
```

#### ast

- ok: `False`
- truncated: `False`

```text
error: [UnsupportedSyntax] expected Semicolon, got Some(Class) at 31..36
```

#### resolved

- ok: `False`
- truncated: `False`

```text
error: [UnsupportedSyntax] expected Semicolon, got Some(Class) at 31..36
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
        "kind": "parameter",
        "typeText": "number",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/ambientClassMergesOverloadsWithInterface.ts",
        "start": 64,
        "length": 1,
        "line": 4,
        "character": 9,
        "name": "n"
      },
      {
        "kind": "parameter",
        "typeText": "number",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/ambientClassMergesOverloadsWithInterface.ts",
        "start": 108,
        "length": 1,
        "line": 7,
        "character": 9,
        "name": "n"
      }
    ],
    "typescriptVersion": "6.0.3"
  },
  "ast": {
    "topLevel": [
      {
        "kind": "ClassDeclaration",
        "text": "declare class C {\r\n    baz(): any;\r\n    foo(n: number): any;\r\n}",
        "line": 2,
        "character": 1
      },
      {
        "kind": "InterfaceDeclaration",
        "text": "interface C {\r\n    foo(n: number): any;\r\n    bar(): any;\r\n}",
        "line": 6,
        "character": 1
      }
    ],
    "pathToPosition": [
      {
        "kind": "SourceFile",
        "text": "declare class C {\r\n    baz(): any;\r\n    foo(n: number): any;\r\n}\r\ninterface C {\r\n    foo(n: number): any;\r\n    bar(): any",
        "line": 2,
        "character": 1
      },
      {
        "kind": "ClassDeclaration",
        "text": "declare class C {\r\n    baz(): any;\r\n    foo(n: number): any;\r\n}",
        "line": 2,
        "character": 1
      }
    ]
  }
}
```

Stack trace:

```text
error: [UnsupportedSyntax] expected Semicolon, got Some(Class) at 31..36
```

## Completion evidence

Closed as superseded by
`issues/done/400-implement-ambient-declaration-erasure-boundary.md`.

Fresh coverage with the current binary:

```text
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/ambientClassMergesOverloadsWithInterface.ts --detail --no-dashboard-data
suite=tsc
executed=1
build_pass=1
unsupported=0
reference/typescript/tests/cases/compiler/ambientClassMergesOverloadsWithInterface.ts: build_pass
```

Fresh triage:

```text
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/ambientClassMergesOverloadsWithInterface.ts
```

Observed owner boundary:

```text
BuildPass: ts2wasm build succeeded
```

Commits:

- superseded by `issues/done/400-implement-ambient-declaration-erasure-boundary.md`

Validation result:

```text
command: env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/ambientClassMergesOverloadsWithInterface.ts --detail --no-dashboard-data
result: pass; executed=1, build_pass=1
date: 2026-05-08

command: env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/ambientClassMergesOverloadsWithInterface.ts
result: pass; BuildPass, tokens/AST/resolved succeed and runtime AST is erased
date: 2026-05-08

command: python scripts/manager.py update-issue-index --check
result: pass
date: 2026-05-08
```

Remaining risks:

- none

## False-done audit resolution

The previous false-done audit noted that this generated bucket had been closed
without evidence. Fresh triage on 2026-05-08 now provides repo-local evidence:
the reference path is a build pass and is covered by the completed issue 400
ambient declaration erasure boundary.
