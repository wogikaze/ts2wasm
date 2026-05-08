---
id: 156
title: "Implement Ambientgetters"
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

Triage ambientGetters across 1 failing reference test cases and split this bucket into implementation-ready child issues.

## Problem

Fresh reference evidence shows this bucket now builds successfully.

Problem: this generated bucket is no longer a build blocker; ambient class
getter declarations are parsed and erased. A remaining TypeScript TS1183
diagnostic parity gap for getter bodies in ambient class declarations was split
to issue 5407.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/ambientGetters.ts
```

Coverage window:

```sh
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/ambientGetters.ts --detail
```

## Desired final state

This generated bucket is closed after splitting the remaining diagnostic parity
gap to `issues/open/5407-report-ambient-accessor-implementation-bodies.md`.
Do not implement directly from this bucket.

## Scope

In scope:

- [x] Inspect the smart triage report below
- [x] Confirm whether existing open/done issues already cover this bucket
- [x] Split TS1183 ambient accessor implementation parity to issue 5407
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
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/ambientGetters.ts --detail
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/ambientGetters.ts
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

- [x] created: `issues/open/5407-report-ambient-accessor-implementation-bodies.md`

## Notes

## Affected test files

- `reference/typescript/tests/cases/compiler/ambientGetters.ts`

## Duplicate detection

## Smart triage

Fresh triage on 2026-05-08 shows this generated ambient declaration bucket now
builds successfully:

```text
reference/typescript/tests/cases/compiler/ambientGetters.ts: build_pass
```

Focused triage reports:

```text
BuildPass: ts2wasm build succeeded
```

Representative source context:

```ts
declare class A {
    get length() : number;
}

declare class B {
    get length() { return 0; }
}
```

The compiler tokenizes both ambient class getter forms and erases them from the
runtime AST/resolved output. The remaining TypeScript oracle diagnostic is
TS1183 for the getter implementation body in the ambient class; that narrower
diagnostic parity gap was split to
`issues/open/5407-report-ambient-accessor-implementation-bodies.md`.

### Smart triage: Triage ambient declaration: ambientGetters

- Issue class: `triage-needed`
- Feature label: `ambient-declaration`
- Diagnostic: `UnsupportedSyntax` / `parser-or-frontend-unsupported`
- Path: `reference/typescript/tests/cases/compiler/ambientGetters.ts`

Reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/ambientGetters.ts
```

Source overview:

```json
{
  "suite": "tsc",
  "bytes": 123,
  "lines": 9,
  "extension": ".ts",
  "first_code_line": "declare class A {"
}
```

Failure location:

```json
{
  "code": "UnsupportedSyntax",
  "message": "expected Semicolon, got Some(Class) at 35..40",
  "span_start": 35,
  "span_end": 40,
  "line": 3,
  "column": 11,
  "feature_label": "ambient-declaration",
  "error_type": "parser-or-frontend-unsupported"
}
```

Source context:

```text
1 | // @target: es5, es2015
2 |
3 | declare class A {
4 |     get length() : number;
5 | }
6 |
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
    "path": "issues/open/156-implement-ambientGetters.md",
    "title": "Implement Ambientgetters",
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
// Candidate source class: A
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
            start: 27,
            end: 34,
        },
    },
    SpannedToken {
        kind: Class,
        span: Span {
            start: 35,
            end: 40,
        },
    },
    SpannedToken {
        kind: Ident(
            "A",
        ),
        span: Span {
            start: 41,
            end: 42,
        },
    },
    SpannedToken {
        kind: LeftBrace,
        span: Span {
            start: 43,
            end: 44,
        },
    },
    SpannedToken {
        kind: Ident(
            "get",
        ),
        span: Span {
            start: 50,
            end: 53,
        },
    },
    SpannedToken {
        kind: Ident(
            "length",
        ),
        span: Span {
            start: 54,
            end: 60,
        },
    },
    SpannedToken {
        kind: LeftParen,
        span: Span {
            start: 60,
            end: 61,
        },
    },
    SpannedToken {
        kind: RightParen,
        span: Span {
            start: 61,
            end: 62,
        },
    },
    SpannedToken {
        kind: Colon,
        span: Span {
            start: 63,
            end: 64,
        },
    },
    SpannedToken {
        kind: Ident(
            "number",
        ),
        span: Span {
            start: 65,
            end: 71,
        },
    },
    SpannedToken {
        kind: Semicolon,
        span: Span {
            start: 71,
            end: 72,
        },
    },
    SpannedToken {
        kind: RightBrace,
        span: Span {
            start: 74,
            end: 75,
        },
    },
    SpannedToken {
        kind: Ident(
            "declare",
        ),
        span: Span {
            start: 79,
```

#### ast

- ok: `False`
- truncated: `False`

```text
error: [UnsupportedSyntax] expected Semicolon, got Some(Class) at 35..40
```

#### resolved

- ok: `False`
- truncated: `False`

```text
error: [UnsupportedSyntax] expected Semicolon, got Some(Class) at 35..40
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
        "code": 1183,
        "category": "Error",
        "message": "An implementation cannot be declared in ambient contexts.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/ambientGetters.ts",
        "start": 115,
        "length": 1,
        "line": 8,
        "character": 18
      }
    ],
    "hints": [],
    "typescriptVersion": "6.0.3"
  },
  "ast": {
    "topLevel": [
      {
        "kind": "ClassDeclaration",
        "text": "declare class A {\r\n    get length() : number;\r\n}",
        "line": 3,
        "character": 1
      },
      {
        "kind": "ClassDeclaration",
        "text": "declare class B {\r\n    get length() { return 0; }\r\n}",
        "line": 7,
        "character": 1
      }
    ],
    "pathToPosition": [
      {
        "kind": "SourceFile",
        "text": "declare class A {\r\n    get length() : number;\r\n}\r\n\r\ndeclare class B {\r\n    get length() { return 0; }\r\n}",
        "line": 3,
        "character": 1
      },
      {
        "kind": "ClassDeclaration",
        "text": "declare class A {\r\n    get length() : number;\r\n}",
        "line": 3,
        "character": 1
      }
    ]
  }
}
```

Stack trace:

```text
error: [UnsupportedSyntax] expected Semicolon, got Some(Class) at 35..40
```

## Completion evidence

Closed after fresh triage confirmed the generated ambient getter build blocker
is resolved by ambient declaration erasure and the remaining TypeScript
diagnostic parity gap is split to
`issues/open/5407-report-ambient-accessor-implementation-bodies.md`.

Fresh coverage:

```text
command: env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/ambientGetters.ts --detail --no-dashboard-data
result: pass; executed=1, build_pass=1, unsupported=0
date: 2026-05-08
```

Fresh triage:

```text
command: env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/ambientGetters.ts
result: pass; BuildPass; runtime AST/resolved output erased the ambient class getter declarations
date: 2026-05-08
```

The TypeScript oracle still reports TS1183 for
`declare class B { get length() { return 0; } }`; that diagnostic parity gap
remains open in issue 5407.

Commits:

- local issue cleanup commit that moves issue 156 to done and creates issue 5407

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

- TS1183 parity for ambient accessor bodies remains open in issue 5407.

## False-done audit resolution

This issue was re-triaged on 2026-05-08 with fresh coverage and smart triage
evidence. The generated bucket is now closed because the original parser/build
blocker is fixed, and the narrower remaining diagnostic mismatch has its own
implementation-ready child issue.
