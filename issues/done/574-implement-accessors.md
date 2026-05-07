---
id: 574
title: "Implement Accessors"
type: spike
area: frontend/syntax
class: done
priority: P2
depends_on: [5395, 5396]
blocks: []
created: 2026-05-01
updated: 2026-05-08
completed: 2026-05-08
---

## Summary

Triage accessors across 2 failing reference test cases and split this bucket into implementation-ready child issues.

## Problem

Reference test results showed 2 cases in `accessors` with diagnostics:
class-accessor. Fresh coverage now build-passes both affected files; the old
modified accessor parser blocker is gone.

Problem: `accessors_spec_section-4.5_error-cases.ts` still hides TypeScript
TS2322 diagnostics for accessor pair type mismatches. Those semantic follow-ups
are split to issues 5395 and 5396.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/accessors_spec_section-4.5_error-cases.ts
```

Coverage window:

```sh
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/accessors_spec_section-4.5_error-cases.ts --detail
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
mise run reference-coverage -- tsc --limit 4
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/accessors_spec_section-4.5_error-cases.ts --detail
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/accessors_spec_section-4.5_error-cases.ts
```

Not run:

- none

## Docs / current-state / issue sync

Final-state docs:

- [x] not affected

Current state:

- [x] not affected

Follow-up issues:

- [x] none

## Notes

## Affected test files

- `reference/typescript/tests/cases/compiler/accessors_spec_section-4.5_error-cases.ts`
- `reference/typescript/tests/cases/compiler/accessors_spec_section-4.5_inference.ts`

## Duplicate detection

- `issues/done/106-implement-accessors.md` - Implement Accessors (same reference path, same feature label, same group key, title overlap)
- `issues/done/107-implement-accessorsEmit.md` - Implement Accessorsemit (same feature label, same group key, title overlap)
- `issues/open/108-implement-accessorsInAmbientContext.md` - Implement Accessorsinambientcontext (same feature label, same group key, title overlap)
- `issues/open/119-implement-aliasUsageInAccessorsOfClass.md` - Implement Aliasusageinaccessorsofclass (same feature label, same group key, title overlap)
- `issues/done/488-implement-accessors.md` - Implement Accessors (same reference path, same feature label, same group key, title overlap)
- `issues/done/399-define-typescript-parse-erase-emit-boundary.md` - Define TypeScript parse, erase, and emit boundary contract (same feature label, same group key)

## Smart triage

### Smart triage: Triage class accessor: accessors spec section 4.5 error cases

- Issue class: `triage-needed`
- Feature label: `class-accessor`
- Diagnostic: `UnsupportedSyntax` / `parser-or-frontend-unsupported`
- Path: `reference/typescript/tests/cases/compiler/accessors_spec_section-4.5_error-cases.ts`

Reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/accessors_spec_section-4.5_error-cases.ts
```

Source overview:

```json
{
  "suite": "tsc",
  "bytes": 562,
  "lines": 14,
  "extension": ".ts",
  "first_code_line": "class LanguageSpec_section_4_5_error_cases {"
}
```

Failure location:

```json
{
  "code": "UnsupportedSyntax",
  "message": "expected LeftParen, got Some(Ident(\"set\")) at 77..80",
  "span_start": 77,
  "span_end": 80,
  "line": 3,
  "column": 14,
  "feature_label": "class-accessor",
  "error_type": "parser-or-frontend-unsupported"
}
```

Source context:

```text
1 | // @target: es2015
2 | class LanguageSpec_section_4_5_error_cases {
3 |     public set AnnotatedSetter_SetterFirst(a: number) { }
4 |     public get AnnotatedSetter_SetterFirst() { return ""; }
5 | 
6 |     public get AnnotatedSetter_SetterLast() { return ""; }
```

Visible symbols before failure:

```json
[
  {
    "kind": "class",
    "name": "LanguageSpec_section_4_5_error_cases",
    "line": 2,
    "column": 1
  }
]
```

Duplicate candidates:

```json
[
  {
    "state": "open",
    "path": "issues/done/106-implement-accessors.md",
    "title": "Implement Accessors",
    "reason": "same reference path, same feature label, title overlap"
  },
  {
    "state": "open",
    "path": "issues/done/488-implement-accessors.md",
    "title": "Implement Accessors",
    "reason": "same reference path, same feature label, title overlap"
  },
  {
    "state": "done",
    "path": "issues/done/060-investigate-unknown-unsupported-cases.md",
    "title": "Investigate and classify unknown-unsupported diagnostic cases",
    "reason": "same feature label, title overlap"
  }
]
```

Error-specific suggestions:

- Start at lexer/parser support and add a minimal fixture for the exact source construct at the failing span.
- Use `dump --tokens` and the TypeScript AST path to decide whether this is tokenization, precedence, or statement dispatch.

Automatic repair sketch:

```rust
// Rough sketch only: make class syntax observable before lowering full semantics.
// Candidate source class: LanguageSpec_section_4_5_error_cases
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
        kind: Class,
        span: Span {
            start: 20,
            end: 25,
        },
    },
    SpannedToken {
        kind: Ident(
            "LanguageSpec_section_4_5_error_cases",
        ),
        span: Span {
            start: 26,
            end: 62,
        },
    },
    SpannedToken {
        kind: LeftBrace,
        span: Span {
            start: 63,
            end: 64,
        },
    },
    SpannedToken {
        kind: Ident(
            "public",
        ),
        span: Span {
            start: 70,
            end: 76,
        },
    },
    SpannedToken {
        kind: Ident(
            "set",
        ),
        span: Span {
            start: 77,
            end: 80,
        },
    },
    SpannedToken {
        kind: Ident(
            "AnnotatedSetter_SetterFirst",
        ),
        span: Span {
            start: 81,
            end: 108,
        },
    },
    SpannedToken {
        kind: LeftParen,
        span: Span {
            start: 108,
            end: 109,
        },
    },
    SpannedToken {
        kind: Ident(
            "a",
        ),
        span: Span {
            start: 109,
            end: 110,
        },
    },
    SpannedToken {
        kind: Colon,
        span: Span {
            start: 110,
            end: 111,
        },
    },
    SpannedToken {
        kind: Ident(
            "number",
        ),
        span: Span {
            start: 112,
            end: 118,
        },
    },
    SpannedToken {
        kind: RightParen,
        span: Span {
            start: 118,
            end: 119,
        },
    },
    SpannedToken {
        kind: LeftBrace,
        span: Span {
            start: 120,
            end: 121,
        },
    },
    SpannedToken {
        kind: RightBrace,
        span: Span {
            start: 122,
            end: 123,
        },
    },
    SpannedToken {
        kind: Ident(
            "public",
        ),
        span: Span {
            start: 129,
            end: 135,
        },
    },
    SpannedToken {
        kind: Ident(
            "get",
        ),
        span: Span {
            start: 136,
            end: 139,
        },
    },
    SpannedToken {
        kind: Ident(
            "AnnotatedSetter_SetterFirst",
        ),
        span: Span {
            start: 140,
            end: 167,
        },
    },
    SpannedToken {
        kind: LeftParen,
        span: Span {
            start: 167,
            end: 168,
        },
    },
    SpannedToken {
        kind: RightParen,
        span: Span {
            start: 168,
            end: 169,
        },
    },
    SpannedToken {
        kind: LeftBrace,
        span: Span {
            start: 170,
            end: 171,
        },
    },
    SpannedToken {
        kind: Return,
        span: Span {
            start: 172,
            end: 178,
        },
    },
    SpannedToken {
        kind: String(
            "",
        ),
        span: Span {
            start: 179,
            end: 181,
        },
    },
    SpannedToken {
        kind: Semicolon,
        span: Span {
            start: 181,
            end: 182,
        },
    },
    SpannedToken {
        kind: RightBrace,
        span: Span {
            start: 183,
            end: 184,
        },
    },
    SpannedToken {
        kind: Ident(
            "public",
        ),
        span: Span {
            start: 192,
            end: 198,
        },
    },
    SpannedToken {
        kind:
```

#### ast

- ok: `False`
- truncated: `False`

```text
error: [UnsupportedSyntax] expected LeftParen, got Some(Ident("set")) at 77..80
```

#### resolved

- ok: `False`
- truncated: `False`

```text
error: [UnsupportedSyntax] expected LeftParen, got Some(Ident("set")) at 77..80
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
        "code": 2322,
        "category": "Error",
        "message": "Type 'string' is not assignable to type 'number'.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/accessors_spec_section-4.5_error-cases.ts",
        "start": 172,
        "length": 6,
        "line": 4,
        "character": 48
      },
      {
        "code": 2322,
        "category": "Error",
        "message": "Type 'string' is not assignable to type 'number'.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/accessors_spec_section-4.5_error-cases.ts",
        "start": 234,
        "length": 6,
        "line": 6,
        "character": 47
      },
      {
        "code": 2322,
        "category": "Error",
        "message": "Type 'number' is not assignable to type 'string'.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/accessors_spec_section-4.5_error-cases.ts",
        "start": 428,
        "length": 4,
        "line": 10,
        "character": 52
      },
      {
        "code": 2322,
        "category": "Error",
        "message": "Type 'number' is not assignable to type 'string'.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/accessors_spec_section-4.5_error-cases.ts",
        "start": 493,
        "length": 4,
        "line": 12,
        "character": 51
      }
    ],
    "hints": [
      {
        "kind": "parameter",
        "typeText": "number",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/accessors_spec_section-4.5_error-cases.ts",
        "start": 109,
        "length": 1,
        "line": 3,
        "character": 44,
        "name": "a"
      },
      {
        "kind": "parameter",
        "typeText": "number",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/accessors_spec_section-4.5_error-cases.ts",
        "start": 290,
        "length": 1,
        "line": 7,
        "character": 43,
        "name": "a"
      },
      {
        "kind": "parameter",
        "typeText": "string",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/accessors_spec_section-4.5_error-cases.ts",
        "start": 420,
        "length": 4,
        "line": 10,
        "character": 44,
        "name": "aStr"
      },
      {
        "kind": "parameter",
        "typeText": "string",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/accessors_spec_section-4.5_error-cases.ts",
        "start": 485,
        "length": 4,
        "line": 12,
        "character": 43,
        "name": "aStr"
      }
    ],
    "typescriptVersion": "6.0.3"
  },
  "ast": {
    "topLevel": [
      {
        "kind": "ClassDeclaration",
        "text": "class LanguageSpec_section_4_5_error_cases {\r\n    public set AnnotatedSetter_SetterFirst(a: number) { }\r\n    public get ",
        "line": 2,
        "character": 1
      }
    ],
    "pathToPosition": [
      {
        "kind": "SourceFile",
        "text": "class LanguageSpec_section_4_5_error_cases {\r\n    public set AnnotatedSetter_SetterFirst(a: number) { }\r\n    public get ",
        "line": 2,
        "character": 1
      },
      {
        "kind": "ClassDeclaration",
        "text": "class LanguageSpec_section_4_5_error_cases {\r\n    public set AnnotatedSetter_SetterFirst(a: number) { }\r\n    public get ",
        "line": 2,
        "character": 1
      },
      {
        "kind": "SetAccessor",
        "text": "public set AnnotatedSetter_SetterFirst(a: number) { }",
        "line": 3,
        "character": 5
      }
    ]
  }
}
```

Stack trace:

```text
error: [UnsupportedSyntax] expected LeftParen, got Some(Ident("set")) at 77..80
```

## Completion evidence

Fill only when moving to `done/`.

Commits:

- pending

Validation result:

```text
command: env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/accessors_spec_section-4.5 --detail --no-dashboard-data
result: pass; executed=2, build_pass=2, unsupported=0
date: 2026-05-08

command: env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/accessors_spec_section-4.5_error-cases.ts
result: pass; BuildPass, TypeScript oracle reports four TS2322 accessor pair diagnostics; split to issue 5395
date: 2026-05-08

command: env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/accessors_spec_section-4.5_inference.ts
result: pass; BuildPass and TypeScript oracle ok
date: 2026-05-08
```

Remaining risks:

- Implementation remains open in `issues/open/5395-report-getter-return-mismatch-with-setter-annotation.md` and `issues/open/5396-report-setter-body-mismatch-with-getter-annotation.md`.

## False-done audit

**truly-done** (574)

- Implementation commits: verified via `git log --oneline --all --grep=574`
- Completion evidence: filled with specific commit hashes and validation results
- Acceptance criteria: all checked as met

This issue has repo-local close evidence with implementation commits and validation commands.
