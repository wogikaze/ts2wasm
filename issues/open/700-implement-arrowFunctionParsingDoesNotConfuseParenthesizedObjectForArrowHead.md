---
id: 700
title: "Implement Arrowfunctionparsingdoesnotconfuseparenthesizedobjectforarrowhead"
type: spike
area: frontend/syntax
class: triage-needed
priority: P1
depends_on: []
blocks: []
created: 2026-05-01
updated: 2026-05-01
---

## Summary

Triage arrowFunctionParsingDoesNotConfuseParenthesizedObjectForArrowHead across 1 failing reference test cases and split this bucket into implementation-ready child issues.

## Problem

Reference test results show 1 cases fail in directory `arrowFunctionParsingDoesNotConfuseParenthesizedObjectForArrowHead` with diagnostics: object-literal. The compiler cannot handle these syntax/semantics, preventing compilation of code in this category.

Problem: arrowFunctionParsingDoesNotConfuseParenthesizedObjectForArrowHead has 1 reference failures and needs smart-triage evidence before implementation starts.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/arrowFunctionParsingDoesNotConfuseParenthesizedObjectForArrowHead.ts
```

Coverage window:

```sh
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/arrowFunctionParsingDoesNotConfuseParenthesizedObjectForArrowHead.ts --detail
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
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/arrowFunctionParsingDoesNotConfuseParenthesizedObjectForArrowHead.ts --detail
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/arrowFunctionParsingDoesNotConfuseParenthesizedObjectForArrowHead.ts
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

- `reference/typescript/tests/cases/compiler/arrowFunctionParsingDoesNotConfuseParenthesizedObjectForArrowHead.ts`

## Duplicate detection

- none found by path/title/feature scan

## Smart triage

### Smart triage: Triage object literal: arrowFunctionParsingDoesNotConfuseParenthesizedObjectForArrowHead

- Issue class: `triage-needed`
- Feature label: `object-literal`
- Diagnostic: `UnsupportedSyntax` / `parser-or-frontend-unsupported`
- Path: `reference/typescript/tests/cases/compiler/arrowFunctionParsingDoesNotConfuseParenthesizedObjectForArrowHead.ts`

Reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/arrowFunctionParsingDoesNotConfuseParenthesizedObjectForArrowHead.ts
```

Source overview:

```json
{
  "suite": "tsc",
  "bytes": 583,
  "lines": 15,
  "extension": ".ts",
  "first_code_line": "declare var value: boolean;"
}
```

Failure location:

```json
{
  "code": "UnsupportedSyntax",
  "message": "arrow function block bodies support a single return statement in this milestone at 349..350",
  "span_start": 349,
  "span_end": 350,
  "line": 9,
  "column": 18,
  "feature_label": "object-literal",
  "error_type": "parser-or-frontend-unsupported"
}
```

Source context:

```text
 6 | const test = () => ({
 7 |     // "Identifier expected." error on "!" and two "Duplicate identifier '(Missing)'." errors on space.
 8 |     prop: !value, // remove ! to see that errors will be gone
 9 |     run: () => { //replace arrow function with regular function to see that errors will be gone
10 |         // comment next line or remove "()" to see that errors will be gone
11 |         if(!a.b()) { return 'special'; }
12 |
```

Visible symbols before failure:

```json
[
  {
    "kind": "binding",
    "name": "value",
    "line": 3,
    "column": 9
  },
  {
    "kind": "binding",
    "name": "a",
    "line": 4,
    "column": 9
  },
  {
    "kind": "binding",
    "name": "test",
    "line": 6,
    "column": 1,
    "initializer": "() => ({"
  }
]
```

Duplicate candidates:

```json
[
  {
    "state": "open",
    "path": "issues/open/374-design-broader-object-toprimitive-for-bigint-comparisons.md",
    "title": "Design broader object ToPrimitive for mixed BigInt comparisons",
    "reason": "same feature label, title overlap"
  },
  {
    "state": "open",
    "path": "issues/open/441-implement-object-literal.md",
    "title": "Implement object literal enhancements",
    "reason": "same feature label, title overlap"
  },
  {
    "state": "done",
    "path": "issues/done/355-dynamic-object-enumeration-spread.md",
    "title": "Implement dynamic object property enumeration spread",
    "reason": "same feature label, title overlap"
  },
  {
    "state": "done",
    "path": "issues/done/372-implement-bigint-object-toprimitive-non-bigint-primitive-returns.md",
    "title": "Implement BigInt object ToPrimitive non-BigInt primitive returns",
    "reason": "same feature label, title overlap"
  },
  {
    "state": "done",
    "path": "issues/done/373-handle-bigint-object-toprimitive-invalid-out-of-range-string-returns.md",
    "title": "Handle BigInt object ToPrimitive invalid and out-of-range string returns",
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
        kind: Ident(
            "declare",
        ),
        span: Span {
            start: 97,
            end: 104,
        },
    },
    SpannedToken {
        kind: Var,
        span: Span {
            start: 105,
            end: 108,
        },
    },
    SpannedToken {
        kind: Ident(
            "value",
        ),
        span: Span {
            start: 109,
            end: 114,
        },
    },
    SpannedToken {
        kind: Colon,
        span: Span {
            start: 114,
            end: 115,
        },
    },
    SpannedToken {
        kind: Ident(
            "boolean",
        ),
        span: Span {
            start: 116,
            end: 123,
        },
    },
    SpannedToken {
        kind: Semicolon,
        span: Span {
            start: 123,
            end: 124,
        },
    },
    SpannedToken {
        kind: Ident(
            "declare",
        ),
        span: Span {
            start: 126,
            end: 133,
        },
    },
    SpannedToken {
        kind: Var,
        span: Span {
            start: 134,
            end: 137,
        },
    },
    SpannedToken {
        kind: Ident(
            "a",
        ),
        span: Span {
            start: 138,
            end: 139,
        },
    },
    SpannedToken {
        kind: Colon,
        span: Span {
            start: 139,
            end: 140,
        },
    },
    SpannedToken {
        kind: Ident(
            "any",
        ),
        span: Span {
            start: 141,
            end: 144,
        },
    },
    SpannedToken {
        kind: Semicolon,
        span: Span {
            start: 144,
            end: 145,
        },
    },
    SpannedToken {
        kind: Const,
        span: Span {
            start: 149,
            end: 154,
        },
    },
    SpannedToken {
        kind: Ident(
            "test",
        ),
        span: Span {
            start: 155,
            end: 159,
        },
    },
    SpannedToken {
        kind: Equal,
        span: Span {
            start: 160,
            end: 161,
        },
    },
    SpannedToken {
        kind: LeftParen,
        span: Span {
            start: 162,
            end: 163,
        },
    },
    SpannedToken {
        kind: RightParen,
        span: Span {
            start: 163,
            end: 164,
        },
    },
    SpannedToken {
        kind: Arrow,
        span: Span {
            start: 165,
            end: 167,
        },
    },
    SpannedToken {
        kind: LeftParen,
        span: Span {
            start: 168,
            end: 169,
        },
    },
    SpannedToken {
        kind: LeftBrace,
        span: Span {
            start: 169,
            end: 170,
        },
    },
    SpannedToken {
        kind: Ident(
            "prop",
        ),
        span: Span {
            start: 281,
            end: 285,
        },
    },
    SpannedToken {
        kind: Colon,
        span: Span {
            start: 285,
            end: 286,
        },
    },
    SpannedToken {
        kind: Bang,
        span: Span {
            start: 287,
            end: 288,
        },
    },
    SpannedToken {
        kind: Ident(
            "value",
        ),
        span: Span {
            start: 288,
            end: 293,
        },
    },
    SpannedToken {
        kind: Comma,
        span: Span {
            start: 293,
            end: 294,
        },
    },
    SpannedToken {
        kind: Ident(
            "ru
```

#### ast

- ok: `False`
- truncated: `False`

```text
error: [UnsupportedSyntax] arrow function block bodies support a single return statement in this milestone at 349..350
```

#### resolved

- ok: `False`
- truncated: `False`

```text
error: [UnsupportedSyntax] arrow function block bodies support a single return statement in this milestone at 349..350
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
        "typeText": "boolean",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/arrowFunctionParsingDoesNotConfuseParenthesizedObjectForArrowHead.ts",
        "start": 109,
        "length": 5,
        "line": 3,
        "character": 13,
        "name": "value"
      },
      {
        "kind": "binding",
        "typeText": "any",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/arrowFunctionParsingDoesNotConfuseParenthesizedObjectForArrowHead.ts",
        "start": 138,
        "length": 1,
        "line": 4,
        "character": 13,
        "name": "a"
      },
      {
        "kind": "binding",
        "typeText": "() => { prop: boolean; run: () => \"special\" | \"default\"; }",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/arrowFunctionParsingDoesNotConfuseParenthesizedObjectForArrowHead.ts",
        "start": 155,
        "length": 4,
        "line": 6,
        "character": 7,
        "name": "test"
      }
    ],
    "typescriptVersion": "6.0.3"
  },
  "ast": {
    "topLevel": [
      {
        "kind": "FirstStatement",
        "text": "declare var value: boolean;",
        "line": 3,
        "character": 1
      },
      {
        "kind": "FirstStatement",
        "text": "declare var a: any;",
        "line": 4,
        "character": 1
      },
      {
        "kind": "FirstStatement",
        "text": "const test = () => ({\r\n    // \"Identifier expected.\" error on \"!\" and two \"Duplicate identifier '(Missing)'.\" errors on ",
        "line": 6,
        "character": 1
      }
    ],
    "pathToPosition": [
      {
        "kind": "SourceFile",
        "text": "declare var value: boolean;\r\ndeclare var a: any;\r\n\r\nconst test = () => ({\r\n    // \"Identifier expected.\" error on \"!\" an",
        "line": 3,
        "character": 1
      },
      {
        "kind": "FirstStatement",
        "text": "const test = () => ({\r\n    // \"Identifier expected.\" error on \"!\" and two \"Duplicate identifier '(Missing)'.\" errors on ",
        "line": 6,
        "character": 1
      },
      {
        "kind": "VariableDeclarationList",
        "text": "const test = () => ({\r\n    // \"Identifier expected.\" error on \"!\" and two \"Duplicate identifier '(Missing)'.\" errors on ",
        "line": 6,
        "character": 1
      },
      {
        "kind": "VariableDeclaration",
        "text": "test = () => ({\r\n    // \"Identifier expected.\" error on \"!\" and two \"Duplicate identifier '(Missing)'.\" errors on space.",
        "line": 6,
        "character": 7
      },
      {
        "kind": "ArrowFunction",
        "text": "() => ({\r\n    // \"Identifier expected.\" error on \"!\" and two \"Duplicate identifier '(Missing)'.\" errors on space.\r\n    p",
        "line": 6,
        "character": 14
      },
      {
        "kind": "ParenthesizedExpression",
        "text": "({\r\n    // \"Identifier expected.\" error on \"!\" and two \"Duplicate identifier '(Missing)'.\" errors on space.\r\n    prop: !",
        "line": 6,
        "character": 20
      },
      {
        "kind": "ObjectLiteralExpression",
        "text": "{\r\n    // \"Identifier expected.\" error on \"!\" and two \"Duplicate identifier '(Missing)'.\" errors on space.\r\n    prop: !v",
        "line": 6,
        "character": 21
      },
      {
        "kind": "PropertyAssignment",
        "text": "run: () => { //replace arrow function with regular function to see that errors will be gone\r\n        // comment next lin",
        "line": 9,
        "character": 5
      },
      {
        "kind": "ArrowFunction",
        "text": "() => { //replace arrow function with regular function to see that errors will be gone\r\n        // comment next line or ",
        "line": 9,
        "character": 10
      }
    ]
  }
}
```

Stack trace:

```text
error: [UnsupportedSyntax] arrow function block bodies support a single return statement in this milestone at 349..350
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
