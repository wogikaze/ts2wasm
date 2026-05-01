---
id: 554
title: "Implement Abstractclassinlocalscope"
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

Triage abstractClassInLocalScope across 1 failing reference test cases and split this bucket into implementation-ready child issues.

## Problem

Reference test results show 1 cases fail in directory `abstractClassInLocalScope` with diagnostics: scope-analysis. The compiler cannot handle these syntax/semantics, preventing compilation of code in this category.

Problem: abstractClassInLocalScope has 1 reference failures and needs smart-triage evidence before implementation starts.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/abstractClassInLocalScope.ts
```

Coverage window:

```sh
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/abstractClassInLocalScope.ts --detail
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
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/abstractClassInLocalScope.ts --detail
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/abstractClassInLocalScope.ts
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

- `reference/typescript/tests/cases/compiler/abstractClassInLocalScope.ts`

## Duplicate detection

- `issues/open/082-implement-abstractClassInLocalScope.md` - Implement Abstractclassinlocalscope (same reference path, same feature label, same group key, title overlap)
- `issues/open/083-implement-abstractClassInLocalScopeIsAbstract.md` - Implement Abstractclassinlocalscopeisabstract (same feature label, same group key, title overlap)
- `issues/open/446-implement-scope-analysis.md` - Implement scope-analysis support (same feature label, same group key, title overlap)
- `issues/open/468-implement-abstractClassInLocalScope.md` - Implement Abstractclassinlocalscope (same reference path, same feature label, same group key, title overlap)
- `issues/open/469-implement-abstractClassInLocalScopeIsAbstract.md` - Implement Abstractclassinlocalscopeisabstract (same feature label, same group key, title overlap)

## Smart triage

### Smart triage: Triage scope analysis: abstractClassInLocalScope

- Issue class: `triage-needed`
- Feature label: `scope-analysis`
- Diagnostic: `UnsupportedSyntax` / `parser-or-frontend-unsupported`
- Path: `reference/typescript/tests/cases/compiler/abstractClassInLocalScope.ts`

Reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/abstractClassInLocalScope.ts
```

Source overview:

```json
{
  "suite": "tsc",
  "bytes": 110,
  "lines": 7,
  "extension": ".ts",
  "first_code_line": "(() => {"
}
```

Failure location:

```json
{
  "code": "UnsupportedSyntax",
  "message": "arrow function block bodies support a single return statement in this milestone at 21..22",
  "span_start": 21,
  "span_end": 22,
  "line": 2,
  "column": 3,
  "feature_label": "scope-analysis",
  "error_type": "parser-or-frontend-unsupported"
}
```

Source context:

```text
1 | // @target: es2015
2 | (() => {
3 |     abstract class A {}
4 |     class B extends A {}
5 |     new B();
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
    "path": "issues/open/082-implement-abstractClassInLocalScope.md",
    "title": "Implement Abstractclassinlocalscope",
    "reason": "same reference path, same feature label, title overlap"
  },
  {
    "state": "open",
    "path": "issues/open/468-implement-abstractClassInLocalScope.md",
    "title": "Implement Abstractclassinlocalscope",
    "reason": "same reference path, same feature label, title overlap"
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
        kind: LeftParen,
        span: Span {
            start: 20,
            end: 21,
        },
    },
    SpannedToken {
        kind: LeftParen,
        span: Span {
            start: 21,
            end: 22,
        },
    },
    SpannedToken {
        kind: RightParen,
        span: Span {
            start: 22,
            end: 23,
        },
    },
    SpannedToken {
        kind: Arrow,
        span: Span {
            start: 24,
            end: 26,
        },
    },
    SpannedToken {
        kind: LeftBrace,
        span: Span {
            start: 27,
            end: 28,
        },
    },
    SpannedToken {
        kind: Ident(
            "abstract",
        ),
        span: Span {
            start: 34,
            end: 42,
        },
    },
    SpannedToken {
        kind: Class,
        span: Span {
            start: 43,
            end: 48,
        },
    },
    SpannedToken {
        kind: Ident(
            "A",
        ),
        span: Span {
            start: 49,
            end: 50,
        },
    },
    SpannedToken {
        kind: LeftBrace,
        span: Span {
            start: 51,
            end: 52,
        },
    },
    SpannedToken {
        kind: RightBrace,
        span: Span {
            start: 52,
            end: 53,
        },
    },
    SpannedToken {
        kind: Class,
        span: Span {
            start: 59,
            end: 64,
        },
    },
    SpannedToken {
        kind: Ident(
            "B",
        ),
        span: Span {
            start: 65,
            end: 66,
        },
    },
    SpannedToken {
        kind: Extends,
        span: Span {
            start: 67,
            end: 74,
        },
    },
    SpannedToken {
        kind: Ident(
            "A",
        ),
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
            start: 78,
            end: 79,
        },
    },
    SpannedToken {
        kind: New,
        span: Span {
            start: 85,
            end: 88,
        },
    },
    SpannedToken {
        kind: Ident(
            "B",
        ),
        span: Span {
            start: 89,
            end: 90,
        },
    },
    SpannedToken {
        kind: LeftParen,
        span: Span {
            start: 90,
            end: 91,
        },
    },
    SpannedToken {
        kind: RightParen,
        span: Span {
            start: 91,
            end: 92,
        },
    },
    SpannedToken {
        kind: Semicolon,
        span: Span {
            start: 92,
            end: 93,
        },
    },
    SpannedToken {
        kind: Return,
        span: Span {
            start: 99,
            end: 105,
        },
    },
    SpannedToken {
        kind: Ident(
            "A",
        ),
        span: Span {
            start: 106,
            end: 107,
        },
    },
    SpannedToken {
        kind: Semicolon,
        span: Span {
            start: 107,
            end: 108,
        },
    },
    SpannedToken {
        kind: RightBrace,
        span: Span {
            start: 110,
            end: 111,
        },
    },
    SpannedToken {
        kind: RightParen,
        span: Span {
            start: 111,
            end: 112,
        },
    },
    SpannedToken {
        kind: LeftPar
```

#### ast

- ok: `False`
- truncated: `False`

```text
error: [UnsupportedSyntax] arrow function block bodies support a single return statement in this milestone at 21..22
```

#### resolved

- ok: `False`
- truncated: `False`

```text
error: [UnsupportedSyntax] arrow function block bodies support a single return statement in this milestone at 21..22
```

TypeScript/JavaScript oracle:

```json
{
  "ok": true,
  "returncode": 0,
  "typescript": {
    "ok": true,
    "diagnostics": [],
    "hints": [],
    "typescriptVersion": "6.0.3"
  },
  "ast": {
    "topLevel": [
      {
        "kind": "ExpressionStatement",
        "text": "(() => {\r\n    abstract class A {}\r\n    class B extends A {}\r\n    new B();\r\n    return A;\r\n})();",
        "line": 2,
        "character": 1
      }
    ],
    "pathToPosition": [
      {
        "kind": "SourceFile",
        "text": "(() => {\r\n    abstract class A {}\r\n    class B extends A {}\r\n    new B();\r\n    return A;\r\n})();\r\n",
        "line": 2,
        "character": 1
      },
      {
        "kind": "ExpressionStatement",
        "text": "(() => {\r\n    abstract class A {}\r\n    class B extends A {}\r\n    new B();\r\n    return A;\r\n})();",
        "line": 2,
        "character": 1
      },
      {
        "kind": "CallExpression",
        "text": "(() => {\r\n    abstract class A {}\r\n    class B extends A {}\r\n    new B();\r\n    return A;\r\n})()",
        "line": 2,
        "character": 1
      },
      {
        "kind": "ParenthesizedExpression",
        "text": "(() => {\r\n    abstract class A {}\r\n    class B extends A {}\r\n    new B();\r\n    return A;\r\n})",
        "line": 2,
        "character": 1
      },
      {
        "kind": "ArrowFunction",
        "text": "() => {\r\n    abstract class A {}\r\n    class B extends A {}\r\n    new B();\r\n    return A;\r\n}",
        "line": 2,
        "character": 2
      }
    ]
  }
}
```

Stack trace:

```text
error: [UnsupportedSyntax] arrow function block bodies support a single return statement in this milestone at 21..22
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
