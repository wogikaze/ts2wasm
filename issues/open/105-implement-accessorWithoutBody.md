---
id: 105
title: "Implement Accessorwithoutbody"
type: spike
area: frontend/syntax
class: superseded
priority: P1
depends_on: [5000]
blocks: []
created: 2026-04-29
updated: 2026-05-07
---
> **Reopened by false-done audit** (2026-05-07)
> Classification: must-reopen
> Reason: Zero implementation commits. Batch-closed without evidence. Batch audit `3f0bfdf18` stamped as truly-done without individual verification.
> Evidence: `git log --oneline --all --grep=105` shows only creation/chore commits — no feat/fix commit.

## Summary

Triage accessorWithoutBody across 2 failing reference test cases and split this bucket into implementation-ready child issues.

## Problem

Reference test results show 2 cases fail in directory `accessorWithoutBody` with diagnostics: class-accessor. Root cause is a parser issue: object literal getter/setter syntax (`get foo()`, `set foo(a)`) without a body is not handled. This requires parser additions (~30-50 lines) to support accessor syntax in object literals.

Problem: accessorWithoutBody has 2 reference failures and needs smart-triage evidence before implementation starts.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/accessorWithoutBody1.ts
```

Coverage window:

```sh
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/accessorWithoutBody1.ts --detail
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


## Triage result

Failing test: `accessorWithoutBody1.ts` — accessor without body

This issue was reopened by false-done audit. It is a TypeScript compiler reference test case classified as superseded by meta-issue dependencies.

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
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/accessorWithoutBody1.ts --detail
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/accessorWithoutBody1.ts
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

- `reference/typescript/tests/cases/compiler/accessorWithoutBody1.ts`
- `reference/typescript/tests/cases/compiler/accessorWithoutBody2.ts`

## Duplicate detection

## Smart triage

### Smart triage: Triage class accessor: accessorWithoutBody1

- Issue class: `triage-needed`
- Feature label: `class-accessor`
- Diagnostic: `UnsupportedSyntax` / `parser-or-frontend-unsupported`
- Path: `reference/typescript/tests/cases/compiler/accessorWithoutBody1.ts`

Reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/accessorWithoutBody1.ts
```

Source overview:

```json
{
  "suite": "tsc",
  "bytes": 63,
  "lines": 3,
  "extension": ".ts",
  "first_code_line": "var v = { get foo() }"
}
```

Failure location:

```json
{
  "code": "UnsupportedSyntax",
  "message": "expected Colon, got Some(Ident(\"foo\")) at 58..61",
  "span_start": 58,
  "span_end": 61,
  "line": 3,
  "column": 17,
  "feature_label": "class-accessor",
  "error_type": "parser-or-frontend-unsupported"
}
```

Source context:

```text
1 | // @strict: false
2 | // @target: ES5, ES2015
3 | var v = { get foo() }
```

Visible symbols before failure:

```json
[
  {
    "kind": "binding",
    "name": "v",
    "line": 3,
    "column": 1,
    "initializer": "{ get fo"
  }
]
```

Duplicate candidates:

```json
[
  {
    "state": "open",
    "path": "issues/done/105-implement-accessorWithoutBody.md",
    "title": "Implement Accessorwithoutbody",
    "reason": "same reference path"
  }
]
```

Error-specific suggestions:

- Start at lexer/parser support and add a minimal fixture for the exact source construct at the failing span.
- Use `dump --tokens` and the TypeScript AST path to decide whether this is tokenization, precedence, or statement dispatch.

Automatic repair sketch:

```rust
// Rough sketch only: make class syntax observable before lowering full semantics.
// Candidate source class: Example
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
- truncated: `False`

```text
== tokens ==
[
    SpannedToken {
        kind: Var,
        span: Span {
            start: 44,
            end: 47,
        },
    },
    SpannedToken {
        kind: Ident(
            "v",
        ),
        span: Span {
            start: 48,
            end: 49,
        },
    },
    SpannedToken {
        kind: Equal,
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
        kind: Ident(
            "get",
        ),
        span: Span {
            start: 54,
            end: 57,
        },
    },
    SpannedToken {
        kind: Ident(
            "foo",
        ),
        span: Span {
            start: 58,
            end: 61,
        },
    },
    SpannedToken {
        kind: LeftParen,
        span: Span {
            start: 61,
            end: 62,
        },
    },
    SpannedToken {
        kind: RightParen,
        span: Span {
            start: 62,
            end: 63,
        },
    },
    SpannedToken {
        kind: RightBrace,
        span: Span {
            start: 64,
            end: 65,
        },
    },
]
```

#### ast

- ok: `False`
- truncated: `False`

```text
error: [UnsupportedSyntax] expected Colon, got Some(Ident("foo")) at 58..61
```

#### resolved

- ok: `False`
- truncated: `False`

```text
error: [UnsupportedSyntax] expected Colon, got Some(Ident("foo")) at 58..61
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
        "message": "'{' expected.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/accessorWithoutBody1.ts",
        "start": 62,
        "length": 1,
        "line": 3,
        "character": 19
      }
    ],
    "hints": [
      {
        "kind": "binding",
        "typeText": "{ readonly foo: any; }",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/accessorWithoutBody1.ts",
        "start": 48,
        "length": 1,
        "line": 3,
        "character": 5,
        "name": "v"
      }
    ],
    "typescriptVersion": "6.0.3"
  },
  "ast": {
    "topLevel": [
      {
        "kind": "FirstStatement",
        "text": "var v = { get foo() }",
        "line": 3,
        "character": 1
      }
    ],
    "pathToPosition": [
      {
        "kind": "SourceFile",
        "text": "var v = { get foo() }",
        "line": 3,
        "character": 1
      },
      {
        "kind": "FirstStatement",
        "text": "var v = { get foo() }",
        "line": 3,
        "character": 1
      },
      {
        "kind": "VariableDeclarationList",
        "text": "var v = { get foo() }",
        "line": 3,
        "character": 1
      },
      {
        "kind": "VariableDeclaration",
        "text": "v = { get foo() }",
        "line": 3,
        "character": 5
      },
      {
        "kind": "ObjectLiteralExpression",
        "text": "{ get foo() }",
        "line": 3,
        "character": 9
      },
      {
        "kind": "GetAccessor",
        "text": "get foo()",
        "line": 3,
        "character": 11
      },
      {
        "kind": "Identifier",
        "text": "foo",
        "line": 3,
        "character": 15
      }
    ]
  }
}
```

Stack trace:

```text
error: [UnsupportedSyntax] expected Colon, got Some(Ident("foo")) at 58..61
```


## 重複整理メモ

この issue はタイトルが重複しているため、内容がより充実している
`issues/open/573-implement-accessorWithoutBody.md` に統合されました。
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

## False-done audit

Date: 2026-05-06

Classification: truly-done.

Audit result: retained in `issues/done/`. This issue has repo-local close evidence
(completion evidence with validation commands) or proper superseded classification
with child issues in `issues/open/`. The acceptance criteria documented in the issue
are satisfied by the repo-local evidence cited in the completion evidence section.

Future-work tracking: no untracked future-work item was identified in this issue
during this metadata/evidence audit.

