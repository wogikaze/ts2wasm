---
id: 485
title: "Implement Accessorparameteraccessibilitymodifier (audit reopened #485)"
type: spike
area: frontend/syntax
class: blocked
priority: P1
depends_on: [5001]
blocks: []
created: 2026-05-01
updated: 2026-05-05status: open
---

## Summary

Triage accessorParameterAccessibilityModifier across 1 failing reference test cases and split this bucket into implementation-ready child issues.

## Problem

Reference test results show 1 cases fail in directory `accessorParameterAccessibilityModifier` with diagnostics: class-accessor. The compiler cannot handle these syntax/semantics, preventing compilation of code in this category.

Problem: accessorParameterAccessibilityModifier has 1 reference failures and needs smart-triage evidence before implementation starts.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/accessorParameterAccessibilityModifier.ts
```

Coverage window:

```sh
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/accessorParameterAccessibilityModifier.ts --detail
```

## Desired final state

This generated bucket is either split into implementation-ready child issues or superseded by an existing open/done issue with matching evidence. Do not implement directly from this bucket.

## Scope

In scope:

- [ ] Inspect the smart triage report below
- [ ] Confirm whether existing open/done issues already cover this bucket
- [ ] Split one feature family, one observable behavior, or one fixed reference window into child issues
- [ ] Preserve exact reproduction commands and representative AST/diagnostic evidence in each child issue

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

- [ ] Duplicate candidates below are confirmed as no-match or this issue is superseded
- [ ] At least one child issue contains an exact `mise run reference-triage -- ...` command
- [ ] Child issue includes failing path, diagnostic code, source context, visible symbols, and parser/TypeScript AST evidence
- [ ] Child issue acceptance names the exact fixture/reference path and diagnostic/stdout change

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run
```

Impacted commands:

```sh
mise run reference-coverage -- tsc --limit 2
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/accessorParameterAccessibilityModifier.ts --detail
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/accessorParameterAccessibilityModifier.ts
```

Not run:

- none

## Docs / current-state / issue sync

Final-state docs:

- [ ] not affected

Current state:

- [ ] updated: `current-state.md` (repo root)

Follow-up issues:

- [ ] none

## Notes

## Affected test files

- `reference/typescript/tests/cases/compiler/accessorParameterAccessibilityModifier.ts`

## Duplicate detection

- `issues/done/101-implement-accessorParameterAccessibilityModifier.md` - Implement Accessorparameteraccessibilitymodifier (same reference path, same feature label, same group key, title overlap)
- `issues/open/422-implement-class-accessor.md` - Implement class-accessor support (same feature label, same group key, title overlap)

## Smart triage

### Smart triage: Triage class accessor: accessorParameterAccessibilityModifier

- Issue class: `triage-needed`
- Feature label: `class-accessor`
- Diagnostic: `UnsupportedSyntax` / `parser-or-frontend-unsupported`
- Path: `reference/typescript/tests/cases/compiler/accessorParameterAccessibilityModifier.ts`

Reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/accessorParameterAccessibilityModifier.ts
```

Source overview:

```json
{
  "suite": "tsc",
  "bytes": 110,
  "lines": 7,
  "extension": ".ts",
  "first_code_line": "class C {"
}
```

Failure location:

```json
{
  "code": "UnsupportedSyntax",
  "message": "expected Comma, got Some(Ident(\"v\")) at 74..75",
  "span_start": 74,
  "span_end": 75,
  "line": 5,
  "column": 22,
  "feature_label": "class-accessor",
  "error_type": "parser-or-frontend-unsupported"
}
```

Source context:

```text
2 | // @target: es5, es2015
3 | 
4 | class C {
5 |     set X(public v) { }
6 |     static set X(public v2) { }
7 | }
```

Visible symbols before failure:

```json
[
  {
    "kind": "class",
    "name": "C",
    "line": 4,
    "column": 1
  }
]
```

Duplicate candidates:

```json
[
  {
    "state": "open",
    "path": "issues/done/101-implement-accessorParameterAccessibilityModifier.md",
    "title": "Implement Accessorparameteraccessibilitymodifier",
    "reason": "same reference path, same feature label, title overlap"
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
- truncated: `False`

```text
== tokens ==
[
    SpannedToken {
        kind: Class,
        span: Span {
            start: 46,
            end: 51,
        },
    },
    SpannedToken {
        kind: Ident(
            "C",
        ),
        span: Span {
            start: 52,
            end: 53,
        },
    },
    SpannedToken {
        kind: LeftBrace,
        span: Span {
            start: 54,
            end: 55,
        },
    },
    SpannedToken {
        kind: Ident(
            "set",
        ),
        span: Span {
            start: 61,
            end: 64,
        },
    },
    SpannedToken {
        kind: Ident(
            "X",
        ),
        span: Span {
            start: 65,
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
            "public",
        ),
        span: Span {
            start: 67,
            end: 73,
        },
    },
    SpannedToken {
        kind: Ident(
            "v",
        ),
        span: Span {
            start: 74,
            end: 75,
        },
    },
    SpannedToken {
        kind: RightParen,
        span: Span {
            start: 75,
            end: 76,
        },
    },
    SpannedToken {
        kind: LeftBrace,
        span: Span {
            start: 77,
            end: 78,
        },
    },
    SpannedToken {
        kind: RightBrace,
        span: Span {
            start: 79,
            end: 80,
        },
    },
    SpannedToken {
        kind: Static,
        span: Span {
            start: 86,
            end: 92,
        },
    },
    SpannedToken {
        kind: Ident(
            "set",
        ),
        span: Span {
            start: 93,
            end: 96,
        },
    },
    SpannedToken {
        kind: Ident(
            "X",
        ),
        span: Span {
            start: 97,
            end: 98,
        },
    },
    SpannedToken {
        kind: LeftParen,
        span: Span {
            start: 98,
            end: 99,
        },
    },
    SpannedToken {
        kind: Ident(
            "public",
        ),
        span: Span {
            start: 99,
            end: 105,
        },
    },
    SpannedToken {
        kind: Ident(
            "v2",
        ),
        span: Span {
            start: 106,
            end: 108,
        },
    },
    SpannedToken {
        kind: RightParen,
        span: Span {
            start: 108,
            end: 109,
        },
    },
    SpannedToken {
        kind: LeftBrace,
        span: Span {
            start: 110,
            end: 111,
        },
    },
    SpannedToken {
        kind: RightBrace,
        span: Span {
            start: 112,
            end: 113,
        },
    },
    SpannedToken {
        kind: RightBrace,
        span: Span {
            start: 115,
            end: 116,
        },
    },
]
```

#### ast

- ok: `False`
- truncated: `False`

```text
error: [UnsupportedSyntax] expected Comma, got Some(Ident("v")) at 74..75
```

#### resolved

- ok: `False`
- truncated: `False`

```text
error: [UnsupportedSyntax] expected Comma, got Some(Ident("v")) at 74..75
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
        "code": 2369,
        "category": "Error",
        "message": "A parameter property is only allowed in a constructor implementation.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/accessorParameterAccessibilityModifier.ts",
        "start": 67,
        "length": 8,
        "line": 5,
        "character": 11
      },
      {
        "code": 2369,
        "category": "Error",
        "message": "A parameter property is only allowed in a constructor implementation.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/accessorParameterAccessibilityModifier.ts",
        "start": 99,
        "length": 9,
        "line": 6,
        "character": 18
      }
    ],
    "hints": [
      {
        "kind": "parameter",
        "typeText": "any",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/accessorParameterAccessibilityModifier.ts",
        "start": 74,
        "length": 1,
        "line": 5,
        "character": 18,
        "name": "v"
      },
      {
        "kind": "parameter",
        "typeText": "any",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/accessorParameterAccessibilityModifier.ts",
        "start": 106,
        "length": 2,
        "line": 6,
        "character": 25,
        "name": "v2"
      }
    ],
    "typescriptVersion": "6.0.3"
  },
  "ast": {
    "topLevel": [
      {
        "kind": "ClassDeclaration",
        "text": "class C {\r\n    set X(public v) { }\r\n    static set X(public v2) { }\r\n}",
        "line": 4,
        "character": 1
      }
    ],
    "pathToPosition": [
      {
        "kind": "SourceFile",
        "text": "class C {\r\n    set X(public v) { }\r\n    static set X(public v2) { }\r\n}",
        "line": 4,
        "character": 1
      },
      {
        "kind": "ClassDeclaration",
        "text": "class C {\r\n    set X(public v) { }\r\n    static set X(public v2) { }\r\n}",
        "line": 4,
        "character": 1
      },
      {
        "kind": "SetAccessor",
        "text": "set X(public v) { }",
        "line": 5,
        "character": 5
      },
      {
        "kind": "Parameter",
        "text": "public v",
        "line": 5,
        "character": 11
      },
      {
        "kind": "Identifier",
        "text": "v",
        "line": 5,
        "character": 18
      }
    ]
  }
}
```

Stack trace:

```text
error: [UnsupportedSyntax] expected Comma, got Some(Ident("v")) at 74..75
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

## Status

Superseded by issue #101. Duplicate from separate coverage run.

## Reopened by audit

Date: 2026-05-05

Classification: must-reopen.

Reopen reason: frontmatter still says `class: blocked`, which is incompatible with a completed issue unless explicit supersedence/closure evidence is present.

Violated acceptance: the issue cannot provide repo-local close evidence for its checked acceptance criteria while it remains in this state. Acceptance checkboxes were reset for re-verification.

Evidence files:
- `issues/open/485-implement-accessorParameterAccessibilityModifier.md` before this move
- `issues/open/485-implement-accessorParameterAccessibilityModifier.md` after this move

Split follow-up: none created in this audit wave; this reopened issue remains the tracking item.
