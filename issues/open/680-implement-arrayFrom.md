---
id: 680
title: "Implement Arrayfrom"
type: spike
area: runtime/builtins
class: blocked
priority: P1
depends_on: [5004]
blocks: []
created: 2026-05-01
updated: 2026-05-01
---

## Summary

Triage arrayFrom across 1 failing reference test cases and split this bucket into implementation-ready child issues.

## Problem

Reference test results show 1 cases fail in directory `arrayFrom` with diagnostics: parser-syntax. The compiler cannot handle these syntax/semantics, preventing compilation of code in this category.

Problem: arrayFrom has 1 reference failures and needs smart-triage evidence before implementation starts.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/arrayFrom.ts
```

Coverage window:

```sh
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/arrayFrom.ts --detail
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
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/arrayFrom.ts --detail
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/arrayFrom.ts
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

- `reference/typescript/tests/cases/compiler/arrayFrom.ts`

## Duplicate detection

- none found by path/title/feature scan

## Smart triage

### Smart triage: Triage parser syntax: arrayFrom

- Issue class: `triage-needed`
- Feature label: `parser-syntax`
- Diagnostic: `UnsupportedSyntax` / `parser-or-frontend-unsupported`
- Path: `reference/typescript/tests/cases/compiler/arrayFrom.ts`

Reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/arrayFrom.ts
```

Source overview:

```json
{
  "suite": "tsc",
  "bytes": 1306,
  "lines": 38,
  "extension": ".ts",
  "first_code_line": "interface A {"
}
```

Failure location:

```json
{
  "code": "UnsupportedSyntax",
  "message": "expected member property name, got Some(SpannedToken { kind: Number(5), span: Span { start: 1325, end: 1326 } }) at 1327..1328",
  "span_start": 1327,
  "span_end": 1328,
  "line": 39,
  "column": 1,
  "feature_label": "parser-syntax",
  "error_type": "parser-or-frontend-unsupported"
}
```

Source context:

```text
36 | function getEither<T> (in1: Iterable<T>, in2: ArrayLike<T>) {
37 |   return Math.random() > 0.5 ? in1 : in2;
38 | }
```

Visible symbols before failure:

```json
[
  {
    "kind": "binding",
    "name": "inputA",
    "line": 15,
    "column": 1
  },
  {
    "kind": "binding",
    "name": "inputB",
    "line": 16,
    "column": 1
  },
  {
    "kind": "binding",
    "name": "inputALike",
    "line": 17,
    "column": 1
  },
  {
    "kind": "binding",
    "name": "inputARand",
    "line": 18,
    "column": 1,
    "initializer": "getEither(inputA, inputALike)"
  },
  {
    "kind": "binding",
    "name": "inputASet",
    "line": 19,
    "column": 1,
    "initializer": "new Set<A>()"
  },
  {
    "kind": "binding",
    "name": "result1",
    "line": 21,
    "column": 1
  },
  {
    "kind": "binding",
    "name": "result2",
    "line": 22,
    "column": 1
  },
  {
    "kind": "binding",
    "name": "result3",
    "line": 23,
    "column": 1
  },
  {
    "kind": "binding",
    "name": "result4",
    "line": 24,
    "column": 1
  },
  {
    "kind": "binding",
    "name": "result5",
    "line": 25,
    "column": 1
  },
  {
    "kind": "binding",
    "name": "result6",
    "line": 26,
    "column": 1
  },
  {
    "kind": "binding",
    "name": "result7",
    "line": 27,
    "column": 1
  },
  {
    "kind": "binding",
    "name": "result8",
    "line": 28,
    "column": 1
  },
  {
    "kind": "binding",
    "name": "result9",
    "line": 29,
    "column": 1
  },
  {
    "kind": "binding",
    "name": "result10",
    "line": 30,
    "column": 1
  },
  {
    "kind": "binding",
    "name": "result11",
    "line": 31,
    "column": 1
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
    "path": "issues/open/464-implement-FunctionDeclaration-parser-syntax.md",
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
    "path": "issues/done/059-implement-parser-syntax-extensions.md",
    "title": "Implement parser syntax extensions for TypeScript and advanced JS",
    "reason": "same feature label, title overlap"
  },
  {
    "state": "done",
    "path": "issues/done/065-implement-parser-syntax.md",
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
            "interface",
        ),
        span: Span {
            start: 137,
            end: 146,
        },
    },
    SpannedToken {
        kind: Ident(
            "A",
        ),
        span: Span {
            start: 147,
            end: 148,
        },
    },
    SpannedToken {
        kind: LeftBrace,
        span: Span {
            start: 149,
            end: 150,
        },
    },
    SpannedToken {
        kind: Ident(
            "a",
        ),
        span: Span {
            start: 154,
            end: 155,
        },
    },
    SpannedToken {
        kind: Colon,
        span: Span {
            start: 155,
            end: 156,
        },
    },
    SpannedToken {
        kind: Ident(
            "string",
        ),
        span: Span {
            start: 157,
            end: 163,
        },
    },
    SpannedToken {
        kind: Semicolon,
        span: Span {
            start: 163,
            end: 164,
        },
    },
    SpannedToken {
        kind: RightBrace,
        span: Span {
            start: 166,
            end: 167,
        },
    },
    SpannedToken {
        kind: Ident(
            "interface",
        ),
        span: Span {
            start: 171,
            end: 180,
        },
    },
    SpannedToken {
        kind: Ident(
            "B",
        ),
        span: Span {
            start: 181,
            end: 182,
        },
    },
    SpannedToken {
        kind: LeftBrace,
        span: Span {
            start: 183,
            end: 184,
        },
    },
    SpannedToken {
        kind: Ident(
            "b",
        ),
        span: Span {
            start: 188,
            end: 189,
        },
    },
    SpannedToken {
        kind: Colon,
        span: Span {
            start: 189,
            end: 190,
        },
    },
    SpannedToken {
        kind: Ident(
            "string",
        ),
        span: Span {
            start: 191,
            end: 197,
        },
    },
    SpannedToken {
        kind: Semicolon,
        span: Span {
            start: 197,
            end: 198,
        },
    },
    SpannedToken {
        kind: RightBrace,
        span: Span {
            start: 200,
            end: 201,
        },
    },
    SpannedToken {
        kind: Const,
        span: Span {
            start: 205,
            end: 210,
        },
    },
    SpannedToken {
        kind: Ident(
            "inputA",
        ),
        span: Span {
            start: 211,
            end: 217,
        },
    },
    SpannedToken {
        kind: Colon,
        span: Span {
            start: 217,
            end: 218,
        },
    },
    SpannedToken {
        kind: Ident(
            "A",
        ),
        span: Span {
            start: 219,
            end: 220,
        },
    },
    SpannedToken {
        kind: LeftBracket,
        span: Span {
            start: 220,
            end: 221,
        },
    },
    SpannedToken {
        kind: RightBracket,
        span: Span {
            start: 221,
            end: 222,
        },
    },
    SpannedToken {
        kind: Equal,
        span: Span {
            start: 223,
            end: 224,
        },
    },
    SpannedToken {
        kind: LeftBracket,
        span: Span {
            start: 225,
            end: 226,
        },
    },
    SpannedToken {
        kind: RightBracket,
        span: Span {
            start: 226,
            end: 227,
        },
```

#### ast

- ok: `False`
- truncated: `False`

```text
error: [UnsupportedSyntax] expected member property name, got Some(SpannedToken { kind: Number(5), span: Span { start: 1325, end: 1326 } }) at 1327..1328
```

#### resolved

- ok: `False`
- truncated: `False`

```text
error: [UnsupportedSyntax] expected member property name, got Some(SpannedToken { kind: Number(5), span: Span { start: 1325, end: 1326 } }) at 1327..1328
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
        "message": "Type 'A[]' is not assignable to type 'B[]'.\n  Property 'b' is missing in type 'A' but required in type 'B'.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/arrayFrom.ts",
        "start": 489,
        "length": 7,
        "line": 23,
        "character": 7
      },
      {
        "code": 2322,
        "category": "Error",
        "message": "Type 'A[]' is not assignable to type 'B[]'.\n  Property 'b' is missing in type 'A' but required in type 'B'.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/arrayFrom.ts",
        "start": 670,
        "length": 7,
        "line": 26,
        "character": 7
      }
    ],
    "hints": [
      {
        "kind": "binding",
        "typeText": "A[]",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/arrayFrom.ts",
        "start": 211,
        "length": 6,
        "line": 15,
        "character": 7,
        "name": "inputA"
      },
      {
        "kind": "binding",
        "typeText": "B[]",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/arrayFrom.ts",
        "start": 236,
        "length": 6,
        "line": 16,
        "character": 7,
        "name": "inputB"
      },
      {
        "kind": "binding",
        "typeText": "ArrayLike<A>",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/arrayFrom.ts",
        "start": 261,
        "length": 10,
        "line": 17,
        "character": 7,
        "name": "inputALike"
      },
      {
        "kind": "binding",
        "typeText": "ArrayLike<A> | Iterable<A>",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/arrayFrom.ts",
        "start": 310,
        "length": 10,
        "line": 18,
        "character": 7,
        "name": "inputARand"
      },
      {
        "kind": "binding",
        "typeText": "Set<A>",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/arrayFrom.ts",
        "start": 361,
        "length": 9,
        "line": 19,
        "character": 7,
        "name": "inputASet"
      },
      {
        "kind": "binding",
        "typeText": "A[]",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/arrayFrom.ts",
        "start": 396,
        "length": 7,
        "line": 21,
        "character": 7,
        "name": "result1"
      },
      {
        "kind": "binding",
        "typeText": "A[]",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/arrayFrom.ts",
        "start": 438,
        "length": 7,
        "line": 22,
        "character": 7,
        "name": "result2"
      },
      {
        "kind": "binding",
        "typeText": "B[]",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/arrayFrom.ts",
        "start": 489,
        "length": 7,
        "line": 23,
        "character": 7,
        "name": "result3"
      },
      {
        "kind": "binding",
        "typeText": "A[]",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/arrayFrom.ts",
        "start": 556,
        "length": 7,
        "line": 24,
        "character": 7,
        "name": "result4"
      },
      {
        "kind": "binding",
        "typeText": "A[]",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/arrayFrom.ts",
        "start": 624,
        "length": 7,
        "line": 25,
        "character": 7,
        "name": "result5"
      },
      {
        "kind": "binding",
        "typeText": "B[]",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/arrayFrom.ts",
        "start": 670,
        "length": 7,
        "line": 26,
        "character": 7,
        "name": "result6"
      },
      {
        "kind": "binding",
        "typeText": "B[]",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/arrayFrom.ts",
        "start": 732,
        "length": 7,
        "line": 27,
        "character": 7,
        "name": "result7"
      },
      {
        "kind": "binding",
        "typeText": "A[]",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/arrayFrom.ts",
        "start": 804,
        "length": 7,
        "line": 28,
        "character": 7,
        "name": "result8"
      },
      {
        "kind": "binding",
        "typeText": "B[]",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/arrayFrom.ts",
        "start": 850,
        "length": 7,
        "line": 29,
        "character": 7,
        "name": "result9"
      },
      {
        "kind": "binding",
        "typeText": "A[]",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/arrayFrom.ts",
        "start": 922,
        "length": 8,
        "line": 30,
        "character": 7,
        "name": "result10"
      },
      {
        "kind": "binding",
        "typeText": "B[]",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/arrayFrom.ts",
        "start": 971,
        "length": 8,
        "line": 31,
        "character": 7,
        "name": "result11"
      },
      {
        "kind": "function",
        "typeText": "ArrayLike<T> | Iterable<T>",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/arrayFrom.ts",
        "start": 1244,
        "length": 9,
        "line": 36,
        "character": 10,
        "name": "getEither"
      },
      {
        "kind": "parameter",
        "typeText": "Iterable<T>",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/arrayFrom.ts",
        "start": 1258,
        "length": 3,
        "line": 36,
        "character": 24,
        "name": "in1"
      },
      {
        "kind": "parameter",
        "typeText": "ArrayLike<T>",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/arrayFrom.ts",
        "start": 1276,
        "length": 3,
        "line": 36,
        "character": 42,
        "name": "in2"
      }
    ],
    "typescriptVersion": "6.0.3"
  },
  "ast": {
    "topLevel": [
      {
        "kind": "InterfaceDeclaration",
        "text": "interface A {\r\n  a: string;\r\n}",
        "line": 7,
        "character": 1
      },
      {
        "kind": "InterfaceDeclaration",
        "text": "interface B {\r\n  b: string;\r\n}",
        "line": 11,
        "character": 1
      },
      {
        "kind": "FirstStatement",
        "text": "const inputA: A[] = [];",
        "line": 15,
        "character": 1
      },
      {
        "kind": "FirstStatement",
        "text": "const inputB: B[] = [];",
        "line": 16,
        "character": 1
      },
      {
        "kind": "FirstStatement",
        "text": "const inputALike: ArrayLike<A> = { length: 0 };",
        "line": 17,
        "character": 1
      },
      {
        "kind": "FirstStatement",
        "text": "const inputARand = getEither(inputA, inputALike);",
        "line": 18,
        "character": 1
      },
      {
        "kind": "FirstStatement",
        "text": "const inputASet = new Set<A>();",
        "line": 19,
        "character": 1
      },
      {
        "kind": "FirstStatement",
        "text": "const result1: A[] = Array.from(inputA);",
        "line": 21,
        "character": 1
      },
      {
        "kind": "FirstStatement",
        "text": "const result2: A[] = Array.from(inputA.values());",
        "line": 22,
        "character": 1
      },
      {
        "kind": "FirstStatement",
        "text": "const result3: B[] = Array.from(inputA.values());",
        "line": 23,
        "character"
```

Stack trace:

```text
error: [UnsupportedSyntax] expected member property name, got Some(SpannedToken { kind: Number(5), span: Span { start: 1325, end: 1326 } }) at 1327..1328
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
