---
id: 741
title: "Implement Assignmenttoconditionalbrandedstringtemplateormapping"
type: spike
area: frontend/semantics
class: blocked
priority: P1
depends_on: [5002]
blocks: []
created: 2026-05-01
updated: 2026-05-01
---

## Summary

Triage assignmentToConditionalBrandedStringTemplateOrMapping across 1 failing reference test cases and split this bucket into implementation-ready child issues.

## Problem

Reference test results show 1 cases fail in directory `assignmentToConditionalBrandedStringTemplateOrMapping` with diagnostics: type-system. The compiler cannot handle these syntax/semantics, preventing compilation of code in this category.

Problem: assignmentToConditionalBrandedStringTemplateOrMapping has 1 reference failures and needs smart-triage evidence before implementation starts.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/assignmentToConditionalBrandedStringTemplateOrMapping.ts
```

Coverage window:

```sh
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/assignmentToConditionalBrandedStringTemplateOrMapping.ts --detail
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
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/assignmentToConditionalBrandedStringTemplateOrMapping.ts --detail
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/assignmentToConditionalBrandedStringTemplateOrMapping.ts
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

- `reference/typescript/tests/cases/compiler/assignmentToConditionalBrandedStringTemplateOrMapping.ts`

## Duplicate detection

- none found by path/title/feature scan

## Smart triage

### Smart triage: Triage type system: assignmentToConditionalBrandedStringTemplateOrMapping

- Issue class: `triage-needed`
- Feature label: `type-system`
- Diagnostic: `UnsupportedSyntax` / `parser-or-frontend-unsupported`
- Path: `reference/typescript/tests/cases/compiler/assignmentToConditionalBrandedStringTemplateOrMapping.ts`

Reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/assignmentToConditionalBrandedStringTemplateOrMapping.ts
```

Source overview:

```json
{
  "suite": "tsc",
  "bytes": 308,
  "lines": 10,
  "extension": ".ts",
  "first_code_line": "let a: (<T>() => T extends `${'a' & { a: 1 }}` ? 1 : 2) = null!;"
}
```

Failure location:

```json
{
  "code": "UnsupportedSyntax",
  "message": "expected Semicolon, got Some(Bang) at 81..82",
  "span_start": 81,
  "span_end": 82,
  "line": 2,
  "column": 63,
  "feature_label": "type-system",
  "error_type": "parser-or-frontend-unsupported"
}
```

Source context:

```text
1 | // @target: es2015
2 | let a: (<T>() => T extends `${'a' & { a: 1 }}` ? 1 : 2) = null!;
3 | let b: (<T>() => T extends `${'a' & { a: 1 }}` ? 1 : 2) = null!;
4 | 
5 | a = b;
```

Visible symbols before failure:

```json
[
  {
    "kind": "binding",
    "name": "a",
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
    "path": "issues/open/345-implement-tsc-type-alias-coverage.md",
    "title": "Implement TypeScript type alias coverage for tsc suite (23 cases)",
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
- truncated: `True`

```text
== tokens ==
[
    SpannedToken {
        kind: Let,
        span: Span {
            start: 19,
            end: 22,
        },
    },
    SpannedToken {
        kind: Ident(
            "a",
        ),
        span: Span {
            start: 23,
            end: 24,
        },
    },
    SpannedToken {
        kind: Colon,
        span: Span {
            start: 24,
            end: 25,
        },
    },
    SpannedToken {
        kind: LeftParen,
        span: Span {
            start: 26,
            end: 27,
        },
    },
    SpannedToken {
        kind: Less,
        span: Span {
            start: 27,
            end: 28,
        },
    },
    SpannedToken {
        kind: Ident(
            "T",
        ),
        span: Span {
            start: 28,
            end: 29,
        },
    },
    SpannedToken {
        kind: Greater,
        span: Span {
            start: 29,
            end: 30,
        },
    },
    SpannedToken {
        kind: LeftParen,
        span: Span {
            start: 30,
            end: 31,
        },
    },
    SpannedToken {
        kind: RightParen,
        span: Span {
            start: 31,
            end: 32,
        },
    },
    SpannedToken {
        kind: Arrow,
        span: Span {
            start: 33,
            end: 35,
        },
    },
    SpannedToken {
        kind: Ident(
            "T",
        ),
        span: Span {
            start: 36,
            end: 37,
        },
    },
    SpannedToken {
        kind: Extends,
        span: Span {
            start: 38,
            end: 45,
        },
    },
    SpannedToken {
        kind: TemplateLiteral(
            "${'a' & { a: 1 }}",
        ),
        span: Span {
            start: 46,
            end: 65,
        },
    },
    SpannedToken {
        kind: Question,
        span: Span {
            start: 66,
            end: 67,
        },
    },
    SpannedToken {
        kind: Number(
            1,
        ),
        span: Span {
            start: 68,
            end: 69,
        },
    },
    SpannedToken {
        kind: Colon,
        span: Span {
            start: 70,
            end: 71,
        },
    },
    SpannedToken {
        kind: Number(
            2,
        ),
        span: Span {
            start: 72,
            end: 73,
        },
    },
    SpannedToken {
        kind: RightParen,
        span: Span {
            start: 73,
            end: 74,
        },
    },
    SpannedToken {
        kind: Equal,
        span: Span {
            start: 75,
            end: 76,
        },
    },
    SpannedToken {
        kind: Null,
        span: Span {
            start: 77,
            end: 81,
        },
    },
    SpannedToken {
        kind: Bang,
        span: Span {
            start: 81,
            end: 82,
        },
    },
    SpannedToken {
        kind: Semicolon,
        span: Span {
            start: 82,
            end: 83,
        },
    },
    SpannedToken {
        kind: Let,
        span: Span {
            start: 84,
            end: 87,
        },
    },
    SpannedToken {
        kind: Ident(
            "b",
        ),
        span: Span {
            start: 88,
            end: 89,
        },
    },
    SpannedToken {
        kind: Colon,
        span: Span {
            start: 89,
            end: 90,
        },
    },
    SpannedToken {
        kind: LeftParen,
        span: Span {
            start: 91,
            end: 92,
        },
    },
    SpannedToken {
        kind: Less,
```

#### ast

- ok: `False`
- truncated: `False`

```text
error: [UnsupportedSyntax] expected Semicolon, got Some(Bang) at 81..82
```

#### resolved

- ok: `False`
- truncated: `False`

```text
error: [UnsupportedSyntax] expected Semicolon, got Some(Bang) at 81..82
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
        "kind": "binding",
        "typeText": "<T>() => T extends `${\"a\" & { a: 1; }}` ? 1 : 2",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/assignmentToConditionalBrandedStringTemplateOrMapping.ts",
        "start": 23,
        "length": 1,
        "line": 2,
        "character": 5,
        "name": "a"
      },
      {
        "kind": "binding",
        "typeText": "<T>() => T extends `${\"a\" & { a: 1; }}` ? 1 : 2",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/assignmentToConditionalBrandedStringTemplateOrMapping.ts",
        "start": 88,
        "length": 1,
        "line": 3,
        "character": 5,
        "name": "b"
      },
      {
        "kind": "binding",
        "typeText": "<T>() => T extends Uppercase<`${\"a\" & { a: 1; }}`> ? 1 : 2",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/assignmentToConditionalBrandedStringTemplateOrMapping.ts",
        "start": 162,
        "length": 1,
        "line": 7,
        "character": 5,
        "name": "c"
      },
      {
        "kind": "binding",
        "typeText": "<T>() => T extends Uppercase<`${\"a\" & { a: 1; }}`> ? 1 : 2",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/assignmentToConditionalBrandedStringTemplateOrMapping.ts",
        "start": 233,
        "length": 1,
        "line": 8,
        "character": 5,
        "name": "d"
      }
    ],
    "typescriptVersion": "6.0.3"
  },
  "ast": {
    "topLevel": [
      {
        "kind": "FirstStatement",
        "text": "let a: (<T>() => T extends `${'a' & { a: 1 }}` ? 1 : 2) = null!;",
        "line": 2,
        "character": 1
      },
      {
        "kind": "FirstStatement",
        "text": "let b: (<T>() => T extends `${'a' & { a: 1 }}` ? 1 : 2) = null!;",
        "line": 3,
        "character": 1
      },
      {
        "kind": "ExpressionStatement",
        "text": "a = b;",
        "line": 5,
        "character": 1
      },
      {
        "kind": "FirstStatement",
        "text": "let c: (<T>() => T extends Uppercase<'a' & { a: 1 }> ? 1 : 2) = null!;",
        "line": 7,
        "character": 1
      },
      {
        "kind": "FirstStatement",
        "text": "let d: (<T>() => T extends Uppercase<'a' & { a: 1 }> ? 1 : 2) = null!;",
        "line": 8,
        "character": 1
      },
      {
        "kind": "ExpressionStatement",
        "text": "c = d;",
        "line": 10,
        "character": 1
      }
    ],
    "pathToPosition": [
      {
        "kind": "SourceFile",
        "text": "let a: (<T>() => T extends `${'a' & { a: 1 }}` ? 1 : 2) = null!;\nlet b: (<T>() => T extends `${'a' & { a: 1 }}` ? 1 : 2)",
        "line": 2,
        "character": 1
      },
      {
        "kind": "FirstStatement",
        "text": "let a: (<T>() => T extends `${'a' & { a: 1 }}` ? 1 : 2) = null!;",
        "line": 2,
        "character": 1
      },
      {
        "kind": "VariableDeclarationList",
        "text": "let a: (<T>() => T extends `${'a' & { a: 1 }}` ? 1 : 2) = null!",
        "line": 2,
        "character": 1
      },
      {
        "kind": "VariableDeclaration",
        "text": "a: (<T>() => T extends `${'a' & { a: 1 }}` ? 1 : 2) = null!",
        "line": 2,
        "character": 5
      },
      {
        "kind": "NonNullExpression",
        "text": "null!",
        "line": 2,
        "character": 59
      },
      {
        "kind": "NullKeyword",
        "text": "null",
        "line": 2,
        "character": 59
      }
    ]
  }
}
```

Stack trace:

```text
error: [UnsupportedSyntax] expected Semicolon, got Some(Bang) at 81..82
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
