---
id: 714
title: "Implement Assertionfunctionscannarrowbydiscriminant"
type: spike
area: frontend/semantics
class: blocked
priority: P1
depends_on: [5001]
blocks: []
created: 2026-05-01
updated: 2026-05-01
---

## Summary

Triage assertionFunctionsCanNarrowByDiscriminant across 1 failing reference test cases and split this bucket into implementation-ready child issues.

## Problem

Reference test results show 1 cases fail in directory `assertionFunctionsCanNarrowByDiscriminant` with diagnostics: parser-syntax. The compiler cannot handle these syntax/semantics, preventing compilation of code in this category.

Problem: assertionFunctionsCanNarrowByDiscriminant has 1 reference failures and needs smart-triage evidence before implementation starts.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/assertionFunctionsCanNarrowByDiscriminant.ts
```

Coverage window:

```sh
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/assertionFunctionsCanNarrowByDiscriminant.ts --detail
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
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/assertionFunctionsCanNarrowByDiscriminant.ts --detail
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/assertionFunctionsCanNarrowByDiscriminant.ts
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

- `reference/typescript/tests/cases/compiler/assertionFunctionsCanNarrowByDiscriminant.ts`

## Duplicate detection

- none found by path/title/feature scan

## Smart triage

### Smart triage: Triage parser syntax: assertionFunctionsCanNarrowByDiscriminant

- Issue class: `triage-needed`
- Feature label: `parser-syntax`
- Diagnostic: `UnsupportedTypeScriptSyntax` / `unsupported-feature-boundary`
- Path: `reference/typescript/tests/cases/compiler/assertionFunctionsCanNarrowByDiscriminant.ts`

Reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/assertionFunctionsCanNarrowByDiscriminant.ts
```

Source overview:

```json
{
  "suite": "tsc",
  "bytes": 594,
  "lines": 25,
  "extension": ".ts",
  "first_code_line": "interface Cat {"
}
```

Failure location:

```json
{
  "code": "UnsupportedTypeScriptSyntax",
  "message": "expected TypeScript type after `as` at 352..354",
  "span_start": 352,
  "span_end": 354,
  "line": 20,
  "column": 6,
  "feature_label": "parser-syntax",
  "error_type": "unsupported-feature-boundary"
}
```

Source context:

```text
17 | const animal = { type: 'cat', canMeow: true } as Animal;
18 | assertEqual(animal.type, 'cat' as const);
19 | 
20 | animal.canMeow; // since is cat, should not be an error
21 | 
22 | const animalOrUndef = { type: 'cat', canMeow: true } as Animal | undefined;
23 | assertEqual(animalOrUndef?.type, 'cat' as const);
```

Visible symbols before failure:

```json
[
  {
    "kind": "binding",
    "name": "animal",
    "line": 17,
    "column": 1,
    "initializer": "{ type: 'cat', canMeow: true } as Animal"
  }
]
```

Duplicate candidates:

```json
[
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
    "reason": "same feature label, title overlap"
  },
  {
    "state": "open",
    "path": "issues/open/550-implement-FunctionDeclaration-parser-syntax.md",
    "title": "Implement Functiondeclaration Parser Syntax",
    "reason": "same feature label, title overlap"
  },
  {
    "state": "open",
    "path": "issues/open/663-implement-arrayAssignmentTest-parser-syntax.md",
    "title": "Implement Arrayassignmenttest Parser Syntax",
    "reason": "same feature label, title overlap"
  },
  {
    "state": "done",
    "path": "issues/open/059-implement-parser-syntax-extensions.md",
    "title": "Implement parser syntax extensions for TypeScript and advanced JS",
    "reason": "same feature label, title overlap"
  },
  {
    "state": "done",
    "path": "issues/open/065-implement-parser-syntax.md",
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
  }
]
```

Error-specific suggestions:

- Create a child issue around this exact path and diagnostic before broadening the reference window.

Compiler dumps:

#### tokens

- ok: `True`
- truncated: `True`

```text
== tokens ==
[
    SpannedToken {
        kind: Ident(
            "interface",
        ),
        span: Span {
            start: 38,
            end: 47,
        },
    },
    SpannedToken {
        kind: Ident(
            "Cat",
        ),
        span: Span {
            start: 48,
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
            "type",
        ),
        span: Span {
            start: 59,
            end: 63,
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
        kind: String(
            "cat",
        ),
        span: Span {
            start: 65,
            end: 70,
        },
    },
    SpannedToken {
        kind: Semicolon,
        span: Span {
            start: 70,
            end: 71,
        },
    },
    SpannedToken {
        kind: Ident(
            "canMeow",
        ),
        span: Span {
            start: 77,
            end: 84,
        },
    },
    SpannedToken {
        kind: Colon,
        span: Span {
            start: 84,
            end: 85,
        },
    },
    SpannedToken {
        kind: True,
        span: Span {
            start: 86,
            end: 90,
        },
    },
    SpannedToken {
        kind: Semicolon,
        span: Span {
            start: 90,
            end: 91,
        },
    },
    SpannedToken {
        kind: RightBrace,
        span: Span {
            start: 93,
            end: 94,
        },
    },
    SpannedToken {
        kind: Ident(
            "interface",
        ),
        span: Span {
            start: 98,
            end: 107,
        },
    },
    SpannedToken {
        kind: Ident(
            "Dog",
        ),
        span: Span {
            start: 108,
            end: 111,
        },
    },
    SpannedToken {
        kind: LeftBrace,
        span: Span {
            start: 112,
            end: 113,
        },
    },
    SpannedToken {
        kind: Ident(
            "type",
        ),
        span: Span {
            start: 119,
            end: 123,
        },
    },
    SpannedToken {
        kind: Colon,
        span: Span {
            start: 123,
            end: 124,
        },
    },
    SpannedToken {
        kind: String(
            "dog",
        ),
        span: Span {
            start: 125,
            end: 130,
        },
    },
    SpannedToken {
        kind: Semicolon,
        span: Span {
            start: 130,
            end: 131,
        },
    },
    SpannedToken {
        kind: Ident(
            "canBark",
        ),
        span: Span {
            start: 137,
            end: 144,
        },
    },
    SpannedToken {
        kind: Colon,
        span: Span {
            start: 144,
            end: 145,
        },
    },
    SpannedToken {
        kind: True,
        span: Span {
            start: 146,
            end: 150,
        },
    },
    SpannedToken {
        kind: Semicolon,
        span: Span {
            start: 150,
            end: 151,
        },
    },
    SpannedToken {
        kind: RightBrace,
        span: Span {
            start: 153,
            end: 154,
        },
    },
    SpannedToken {
        kind: Ident(
            "type",
        ),
        span: Span {
            start: 158,
            end: 162,
        },
    }
```

#### ast

- ok: `False`
- truncated: `False`

```text
error: [UnsupportedTypeScriptSyntax] expected TypeScript type after `as` at 352..354
```

#### resolved

- ok: `False`
- truncated: `False`

```text
error: [UnsupportedTypeScriptSyntax] expected TypeScript type after `as` at 352..354
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
        "kind": "function",
        "typeText": "void",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/assertionFunctionsCanNarrowByDiscriminant.ts",
        "start": 203,
        "length": 11,
        "line": 15,
        "character": 18,
        "name": "assertEqual"
      },
      {
        "kind": "parameter",
        "typeText": "any",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/assertionFunctionsCanNarrowByDiscriminant.ts",
        "start": 218,
        "length": 5,
        "line": 15,
        "character": 33,
        "name": "value"
      },
      {
        "kind": "parameter",
        "typeText": "T",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/assertionFunctionsCanNarrowByDiscriminant.ts",
        "start": 230,
        "length": 4,
        "line": 15,
        "character": 45,
        "name": "type"
      },
      {
        "kind": "binding",
        "typeText": "Animal",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/assertionFunctionsCanNarrowByDiscriminant.ts",
        "start": 269,
        "length": 6,
        "line": 17,
        "character": 7,
        "name": "animal"
      },
      {
        "kind": "binding",
        "typeText": "Animal | undefined",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/assertionFunctionsCanNarrowByDiscriminant.ts",
        "start": 431,
        "length": 13,
        "line": 22,
        "character": 7,
        "name": "animalOrUndef"
      }
    ],
    "typescriptVersion": "6.0.3"
  },
  "ast": {
    "topLevel": [
      {
        "kind": "InterfaceDeclaration",
        "text": "interface Cat {\r\n    type: 'cat';\r\n    canMeow: true;\r\n}",
        "line": 3,
        "character": 1
      },
      {
        "kind": "InterfaceDeclaration",
        "text": "interface Dog {\r\n    type: 'dog';\r\n    canBark: true;\r\n}",
        "line": 8,
        "character": 1
      },
      {
        "kind": "TypeAliasDeclaration",
        "text": "type Animal = Cat | Dog;",
        "line": 13,
        "character": 1
      },
      {
        "kind": "FunctionDeclaration",
        "text": "declare function assertEqual<T>(value: any, type: T): asserts value is T;",
        "line": 15,
        "character": 1
      },
      {
        "kind": "FirstStatement",
        "text": "const animal = { type: 'cat', canMeow: true } as Animal;",
        "line": 17,
        "character": 1
      },
      {
        "kind": "ExpressionStatement",
        "text": "assertEqual(animal.type, 'cat' as const);",
        "line": 18,
        "character": 1
      },
      {
        "kind": "ExpressionStatement",
        "text": "animal.canMeow;",
        "line": 20,
        "character": 1
      },
      {
        "kind": "FirstStatement",
        "text": "const animalOrUndef = { type: 'cat', canMeow: true } as Animal | undefined;",
        "line": 22,
        "character": 1
      },
      {
        "kind": "ExpressionStatement",
        "text": "assertEqual(animalOrUndef?.type, 'cat' as const);",
        "line": 23,
        "character": 1
      },
      {
        "kind": "ExpressionStatement",
        "text": "animalOrUndef.canMeow;",
        "line": 25,
        "character": 1
      }
    ],
    "pathToPosition": [
      {
        "kind": "SourceFile",
        "text": "interface Cat {\r\n    type: 'cat';\r\n    canMeow: true;\r\n}\r\n\r\ninterface Dog {\r\n    type: 'dog';\r\n    canBark: true;\r\n}\r\n\r\n",
        "line": 3,
        "character": 1
      },
      {
        "kind": "ExpressionStatement",
        "text": "assertEqual(animal.type, 'cat' as const);",
        "line": 18,
        "character": 1
      },
      {
        "kind": "CallExpression",
        "text": "assertEqual(animal.type, 'cat' as const)",
        "line": 18,
        "character": 1
      },
      {
        "kind": "AsExpression",
        "text": "'cat' as const",
        "line": 18,
        "character": 26
      }
    ]
  }
}
```

Stack trace:

```text
error: [UnsupportedTypeScriptSyntax] expected TypeScript type after `as` at 352..354
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
