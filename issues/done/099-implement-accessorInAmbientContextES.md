---
id: 099
title: "Implement Accessorinambientcontextes"
type: spike
area: frontend/syntax
class: superseded
priority: P1
depends_on: [5001]
blocks: []
created: 2026-04-29
updated: 2026-05-07
---
> **Reopened by false-done audit** (2026-05-07)
> Classification: must-reopen
> Reason: Zero implementation commits. Batch-closed without evidence. Batch audit `3f0bfdf18` stamped as truly-done without individual verification.
> Evidence: `git log --oneline --all --grep=099` shows only creation/chore commits — no feat/fix commit.

## Summary

Triage accessorInAmbientContextES across 1 failing reference test cases and split this bucket into implementation-ready child issues.

## Problem

Reference test results show 1 cases fail in directory `accessorInAmbientContextES` with diagnostics: class-accessor. The compiler cannot handle these syntax/semantics, preventing compilation of code in this category.

Problem: accessorInAmbientContextES has 1 reference failures and needs smart-triage evidence before implementation starts.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/accessorInAmbientContextES5.ts
```

Coverage window:

```sh
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/accessorInAmbientContextES5.ts --detail
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

Failing test: `accessorInAmbientContextES5.ts` — accessor in ambient context

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
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/accessorInAmbientContextES5.ts --detail
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/accessorInAmbientContextES5.ts
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

- `reference/typescript/tests/cases/compiler/accessorInAmbientContextES5.ts`

## Duplicate detection

## Smart triage

### Smart triage: Triage class accessor: accessorInAmbientContextES5

- Issue class: `triage-needed`
- Feature label: `class-accessor`
- Diagnostic: `UnsupportedSyntax` / `parser-or-frontend-unsupported`
- Path: `reference/typescript/tests/cases/compiler/accessorInAmbientContextES5.ts`

Reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/accessorInAmbientContextES5.ts
```

Source overview:

```json
{
  "suite": "tsc",
  "bytes": 678,
  "lines": 29,
  "extension": ".ts",
  "first_code_line": "declare class AmbientClass {"
}
```

Failure location:

```json
{
  "code": "UnsupportedSyntax",
  "message": "expected Semicolon, got Some(Class) at 125..130",
  "span_start": 125,
  "span_end": 130,
  "line": 6,
  "column": 9,
  "feature_label": "class-accessor",
  "error_type": "parser-or-frontend-unsupported"
}
```

Source context:

```text
3 |
4 | // Should allow accessor in ambient contexts even when targeting ES5
5 |
6 | declare class AmbientClass {
7 |     accessor prop1: string;
8 |     static accessor prop2: number;
9 |     private accessor prop3: boolean;
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
    "path": "issues/done/099-implement-accessorInAmbientContextES.md",
    "title": "Implement Accessorinambientcontextes",
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
// Candidate source class: AmbientClass
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
            start: 117,
            end: 124,
        },
    },
    SpannedToken {
        kind: Class,
        span: Span {
            start: 125,
            end: 130,
        },
    },
    SpannedToken {
        kind: Ident(
            "AmbientClass",
        ),
        span: Span {
            start: 131,
            end: 143,
        },
    },
    SpannedToken {
        kind: LeftBrace,
        span: Span {
            start: 144,
            end: 145,
        },
    },
    SpannedToken {
        kind: Ident(
            "accessor",
        ),
        span: Span {
            start: 150,
            end: 158,
        },
    },
    SpannedToken {
        kind: Ident(
            "prop1",
        ),
        span: Span {
            start: 159,
            end: 164,
        },
    },
    SpannedToken {
        kind: Colon,
        span: Span {
            start: 164,
            end: 165,
        },
    },
    SpannedToken {
        kind: Ident(
            "string",
        ),
        span: Span {
            start: 166,
            end: 172,
        },
    },
    SpannedToken {
        kind: Semicolon,
        span: Span {
            start: 172,
            end: 173,
        },
    },
    SpannedToken {
        kind: Static,
        span: Span {
            start: 178,
            end: 184,
        },
    },
    SpannedToken {
        kind: Ident(
            "accessor",
        ),
        span: Span {
            start: 185,
            end: 193,
        },
    },
    SpannedToken {
        kind: Ident(
            "prop2",
        ),
        span: Span {
            start: 194,
            end: 199,
        },
    },
    SpannedToken {
        kind: Col
```

#### ast

- ok: `False`
- truncated: `False`

```text
error: [UnsupportedSyntax] expected Semicolon, got Some(Class) at 125..130
```

#### resolved

- ok: `False`
- truncated: `False`

```text
error: [UnsupportedSyntax] expected Semicolon, got Some(Class) at 125..130
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
        "code": 2564,
        "category": "Error",
        "message": "Property 'shouldError' has no initializer and is not definitely assigned in the constructor.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/accessorInAmbientContextES5.ts",
        "start": 634,
        "length": 11,
        "line": 28,
        "character": 14
      }
    ],
    "hints": [],
    "typescriptVersion": "6.0.3"
  },
  "ast": {
    "topLevel": [
      {
        "kind": "ClassDeclaration",
        "text": "declare class AmbientClass {\n    accessor prop1: string;\n    static accessor prop2: number;\n    private accessor prop3: ",
        "line": 6,
        "character": 1
      },
      {
        "kind": "ModuleDeclaration",
        "text": "declare namespace AmbientNamespace {\n    class C {\n        accessor prop: string;\n    }\n}",
        "line": 13,
        "character": 1
      },
      {
        "kind": "ModuleDeclaration",
        "text": "declare module \"some-module\" {\n    export class ExportedClass {\n        accessor value: any;\n    }\n}",
        "line": 20,
        "character": 1
      },
      {
        "kind": "ClassDeclaration",
        "text": "class RegularClass {\n    accessor shouldError: string; // Should still error\n}",
        "line": 27,
        "character": 1
      }
    ],
    "pathToPosition": [
      {
        "kind": "SourceFile",
        "text": "declare class AmbientClass {\n    accessor prop1: string;\n    static accessor prop2: number;\n    private accessor prop3: ",
        "line": 6,
        "character": 1
      },
      {
        "kind": "ClassDeclaration",
        "text": "declare class AmbientClass {\n    accessor prop1: string;\n    static accessor prop2: number;\n    private accessor prop3: ",
        "line": 6,
        "character": 1
      }
    ]
  }
}
```

Stack trace:

```text
error: [UnsupportedSyntax] expected Semicolon, got Some(Class) at 125..130
```


## 重複整理メモ

この issue はタイトルが重複しているため、内容がより充実している
`issues/done/569-implement-accessorInAmbientContextES.md` に統合されました。
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

