---
id: 674
title: "Implement Arrayfakeflatnocrashinferencedeclarations"
type: spike
area: runtime/builtins
class: triage-needed
priority: P1
depends_on: []
blocks: []
created: 2026-05-01
updated: 2026-05-01
---

## Summary

Triage arrayFakeFlatNoCrashInferenceDeclarations across 1 failing reference test cases and split this bucket into implementation-ready child issues.

## Problem

Reference test results show 1 cases fail in directory `arrayFakeFlatNoCrashInferenceDeclarations` with diagnostics: parser-syntax. The compiler cannot handle these syntax/semantics, preventing compilation of code in this category.

Problem: arrayFakeFlatNoCrashInferenceDeclarations has 1 reference failures and needs smart-triage evidence before implementation starts.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/arrayFakeFlatNoCrashInferenceDeclarations.ts
```

Coverage window:

```sh
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/arrayFakeFlatNoCrashInferenceDeclarations.ts --detail
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
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/arrayFakeFlatNoCrashInferenceDeclarations.ts --detail
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/arrayFakeFlatNoCrashInferenceDeclarations.ts
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

- `reference/typescript/tests/cases/compiler/arrayFakeFlatNoCrashInferenceDeclarations.ts`

## Duplicate detection

- none found by path/title/feature scan

## Smart triage

### Smart triage: Triage parser syntax: arrayFakeFlatNoCrashInferenceDeclarations

- Issue class: `triage-needed`
- Feature label: `parser-syntax`
- Diagnostic: `UnsupportedTypeScriptSyntax` / `unsupported-feature-boundary`
- Path: `reference/typescript/tests/cases/compiler/arrayFakeFlatNoCrashInferenceDeclarations.ts`

Reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/arrayFakeFlatNoCrashInferenceDeclarations.ts
```

Source overview:

```json
{
  "suite": "tsc",
  "bytes": 548,
  "lines": 19,
  "extension": ".ts",
  "first_code_line": "type BadFlatArray<Arr, Depth extends number> = {obj: {"
}
```

Failure location:

```json
{
  "code": "UnsupportedTypeScriptSyntax",
  "message": "issue-400: unterminated ambient function declaration at 387..394",
  "span_start": 387,
  "span_end": 394,
  "line": 12,
  "column": 12,
  "feature_label": "parser-syntax",
  "error_type": "unsupported-feature-boundary"
}
```

Source context:

```text
 9 |     : Arr
10 | }[Depth extends -1 ? "done" : "recur"]}["obj"];
11 | 
12 | declare function flat<A, D extends number = 1>(
13 |     arr: A,
14 |     depth?: D
15 | ): BadFlatArray<A, D>[]
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
    "path": "issues/done/550-implement-FunctionDeclaration-parser-syntax.md",
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
            "type",
        ),
        span: Span {
            start: 78,
            end: 82,
        },
    },
    SpannedToken {
        kind: Ident(
            "BadFlatArray",
        ),
        span: Span {
            start: 83,
            end: 95,
        },
    },
    SpannedToken {
        kind: Less,
        span: Span {
            start: 95,
            end: 96,
        },
    },
    SpannedToken {
        kind: Ident(
            "Arr",
        ),
        span: Span {
            start: 96,
            end: 99,
        },
    },
    SpannedToken {
        kind: Comma,
        span: Span {
            start: 99,
            end: 100,
        },
    },
    SpannedToken {
        kind: Ident(
            "Depth",
        ),
        span: Span {
            start: 101,
            end: 106,
        },
    },
    SpannedToken {
        kind: Extends,
        span: Span {
            start: 107,
            end: 114,
        },
    },
    SpannedToken {
        kind: Ident(
            "number",
        ),
        span: Span {
            start: 115,
            end: 121,
        },
    },
    SpannedToken {
        kind: Greater,
        span: Span {
            start: 121,
            end: 122,
        },
    },
    SpannedToken {
        kind: Equal,
        span: Span {
            start: 123,
            end: 124,
        },
    },
    SpannedToken {
        kind: LeftBrace,
        span: Span {
            start: 125,
            end: 126,
        },
    },
    SpannedToken {
        kind: Ident(
            "obj",
        ),
        span: Span {
            start: 126,
            end: 129,
        },
    },
    SpannedToken {
        kind: Colon,
        span: Span {
            start: 129,
            end: 130,
        },
    },
    SpannedToken {
        kind: LeftBrace,
        span: Span {
            start: 131,
            end: 132,
        },
    },
    SpannedToken {
        kind: String(
            "done",
        ),
        span: Span {
            start: 138,
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
        kind: Ident(
            "Arr",
        ),
        span: Span {
            start: 146,
            end: 149,
        },
    },
    SpannedToken {
        kind: Comma,
        span: Span {
            start: 149,
            end: 150,
        },
    },
    SpannedToken {
        kind: String(
            "recur",
        ),
        span: Span {
            start: 156,
            end: 163,
        },
    },
    SpannedToken {
        kind: Colon,
        span: Span {
            start: 163,
            end: 164,
        },
    },
    SpannedToken {
        kind: Ident(
            "Arr",
        ),
        span: Span {
            start: 165,
            end: 168,
        },
    },
    SpannedToken {
        kind: Extends,
        span: Span {
            start: 169,
            end: 176,
        },
    },
    SpannedToken {
        kind: Ident(
            "ReadonlyArray",
        ),
        span: Span {
            start: 177,
            end: 190,
        },
    },
    SpannedToken {
        kind: Less,
        span: Span {
            start: 190,
            end: 191,
        },
    },
    SpannedToken {
        kind: Ident(
            "infer",
        ),
        span: Span {
            start: 191,
```

#### ast

- ok: `False`
- truncated: `False`

```text
error: [UnsupportedTypeScriptSyntax] issue-400: unterminated ambient function declaration at 387..394
```

#### resolved

- ok: `False`
- truncated: `False`

```text
error: [UnsupportedTypeScriptSyntax] issue-400: unterminated ambient function declaration at 387..394
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
        "typeText": "{ done: A; recur: A extends readonly (infer InnerArr)[] ? { done: InnerArr; recur: InnerArr extends readonly (infer InnerArr)[] ? { done: InnerArr; recur: InnerArr extends readonly (infer InnerArr)[] ? { ...; }[[...][[...][[...][D]]] extends -1 ? \"done\" : \"recur\"] : InnerArr; }[[...][[...][D]] extends -1 ? \"done\" : ...",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/arrayFakeFlatNoCrashInferenceDeclarations.ts",
        "start": 404,
        "length": 4,
        "line": 12,
        "character": 18,
        "name": "flat"
      },
      {
        "kind": "parameter",
        "typeText": "A",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/arrayFakeFlatNoCrashInferenceDeclarations.ts",
        "start": 440,
        "length": 3,
        "line": 13,
        "character": 5,
        "name": "arr"
      },
      {
        "kind": "parameter",
        "typeText": "D | undefined",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/arrayFakeFlatNoCrashInferenceDeclarations.ts",
        "start": 453,
        "length": 5,
        "line": 14,
        "character": 5,
        "name": "depth"
      },
      {
        "kind": "function",
        "typeText": "(T | (T extends readonly (infer InnerArr)[] ? InnerArr | (InnerArr extends readonly (infer InnerArr)[] ? ... : InnerArr) : T))[]",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/arrayFakeFlatNoCrashInferenceDeclarations.ts",
        "start": 500,
        "length": 3,
        "line": 17,
        "character": 10,
        "name": "foo"
      },
      {
        "kind": "parameter",
        "typeText": "T[]",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/arrayFakeFlatNoCrashInferenceDeclarations.ts",
        "start": 507,
        "length": 3,
        "line": 17,
        "character": 17,
        "name": "arr"
      },
      {
        "kind": "parameter",
        "typeText": "number",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/arrayFakeFlatNoCrashInferenceDeclarations.ts",
        "start": 517,
        "length": 5,
        "line": 17,
        "character": 27,
        "name": "depth"
      }
    ],
    "typescriptVersion": "6.0.3"
  },
  "ast": {
    "topLevel": [
      {
        "kind": "TypeAliasDeclaration",
        "text": "type BadFlatArray<Arr, Depth extends number> = {obj: {\r\n    \"done\": Arr,\r\n    \"recur\": Arr extends ReadonlyArray<infer I",
        "line": 5,
        "character": 1
      },
      {
        "kind": "FunctionDeclaration",
        "text": "declare function flat<A, D extends number = 1>(\r\n    arr: A,\r\n    depth?: D\r\n): BadFlatArray<A, D>[]",
        "line": 12,
        "character": 1
      },
      {
        "kind": "FunctionDeclaration",
        "text": "function foo<T>(arr: T[], depth: number) {\r\n    return flat(arr, depth);\r\n}",
        "line": 17,
        "character": 1
      }
    ],
    "pathToPosition": [
      {
        "kind": "SourceFile",
        "text": "type BadFlatArray<Arr, Depth extends number> = {obj: {\r\n    \"done\": Arr,\r\n    \"recur\": Arr extends ReadonlyArray<infer I",
        "line": 5,
        "character": 1
      },
      {
        "kind": "FunctionDeclaration",
        "text": "declare function flat<A, D extends number = 1>(\r\n    arr: A,\r\n    depth?: D\r\n): BadFlatArray<A, D>[]",
        "line": 12,
        "character": 1
      },
      {
        "kind": "DeclareKeyword",
        "text": "declare",
        "line": 12,
        "character": 1
      }
    ]
  }
}
```

Stack trace:

```text
error: [UnsupportedTypeScriptSyntax] issue-400: unterminated ambient function declaration at 387..394
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
