---
id: 683
title: "Implement Arrayiterationlibes"
type: spike
area: frontend/resolver
class: blocked
priority: P1
depends_on: [5005]
blocks: []
created: 2026-05-01
updated: 2026-05-01
---

## Summary

Triage arrayIterationLibES across 1 failing reference test cases and split this bucket into implementation-ready child issues.

## Problem

Reference test results show 1 cases fail in directory `arrayIterationLibES` with diagnostics: name-resolution. The compiler cannot handle these syntax/semantics, preventing compilation of code in this category.

Problem: arrayIterationLibES has 1 reference failures and needs smart-triage evidence before implementation starts.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/arrayIterationLibES5TargetDifferent.ts
```

Coverage window:

```sh
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/arrayIterationLibES5TargetDifferent.ts --detail
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
mise run reference-coverage -- tsc --limit 2
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/arrayIterationLibES5TargetDifferent.ts --detail
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/arrayIterationLibES5TargetDifferent.ts
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

- `reference/typescript/tests/cases/compiler/arrayIterationLibES5TargetDifferent.ts`

## Duplicate detection

- none found by path/title/feature scan

## Smart triage

### Smart triage: Triage name resolution: arrayIterationLibES5TargetDifferent

- Issue class: `triage-needed`
- Feature label: `name-resolution`
- Diagnostic: `UnresolvedName` / `resolver-symbol`
- Path: `reference/typescript/tests/cases/compiler/arrayIterationLibES5TargetDifferent.ts`

Reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/arrayIterationLibES5TargetDifferent.ts
```

Source overview:

```json
{
  "suite": "tsc",
  "bytes": 402,
  "lines": 28,
  "extension": ".ts",
  "first_code_line": "declare function log(message?: any): void;"
}
```

Failure location:

```json
{
  "code": "UnresolvedName",
  "message": "unresolved name: `log` at 160..163",
  "span_start": 160,
  "span_end": 163,
  "line": 9,
  "column": 5,
  "feature_label": "name-resolution",
  "error_type": "resolver-symbol"
}
```

Source context:

```text
 6 | declare function log(message?: any): void;
 7 | 
 8 | for (const x of [1, 2, 3]) {
 9 |     log(x);
10 | }
11 | 
12 | declare const aString: string;
```

Visible symbols before failure:

```json
[
  {
    "kind": "function",
    "name": "log",
    "line": 6,
    "column": 9,
    "params": "message?: any"
  },
  {
    "kind": "binding",
    "name": "x",
    "line": 8,
    "column": 6
  }
]
```

Duplicate candidates:

```json
[
  {
    "state": "open",
    "path": "issues/open/064-implement-name-resolution.md",
    "title": "Implement name resolution (triaged - superseded by test262 metadata issues)",
    "reason": "same feature label, title overlap"
  },
  {
    "state": "open",
    "path": "issues/open/437-implement-name-resolution.md",
    "title": "Implement name resolution",
    "reason": "same feature label, title overlap"
  },
  {
    "state": "open",
    "path": "issues/open/648-implement-argumentsAsPropertyName-name-resolution.md",
    "title": "Implement Argumentsaspropertyname Name Resolution",
    "reason": "same feature label, title overlap"
  },
  {
    "state": "open",
    "path": "issues/open/654-implement-argumentsReferenceInConstructor-name-resolution.md",
    "title": "Implement Argumentsreferenceinconstructor Name Resolution",
    "reason": "same feature label, title overlap"
  },
  {
    "state": "open",
    "path": "issues/open/657-implement-argumentsReferenceInMethod-name-resolution.md",
    "title": "Implement Argumentsreferenceinmethod Name Resolution",
    "reason": "same feature label, title overlap"
  }
]
```

Error-specific suggestions:

- Check whether the missing name should be a local binding, function binding, builtin, import binding, or runtime global.
- Acceptance should assert both the formerly missing symbol and an adjacent negative case.

Automatic repair sketch:

```rust
// Rough sketch only: make unresolved names inspectable at resolver failure.
if let Some(binding) = self.lookup_name(name) {
    return Ok(binding);
}
return Err(Diagnostic {
    code: DiagCode::UnresolvedName,
    message: format!("unresolved name `{name}`; visible bindings: {:?}", self.visible_names()),
    span,
});
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
            start: 83,
            end: 90,
        },
    },
    SpannedToken {
        kind: Function,
        span: Span {
            start: 91,
            end: 99,
        },
    },
    SpannedToken {
        kind: Ident(
            "log",
        ),
        span: Span {
            start: 100,
            end: 103,
        },
    },
    SpannedToken {
        kind: LeftParen,
        span: Span {
            start: 103,
            end: 104,
        },
    },
    SpannedToken {
        kind: Ident(
            "message",
        ),
        span: Span {
            start: 104,
            end: 111,
        },
    },
    SpannedToken {
        kind: Question,
        span: Span {
            start: 111,
            end: 112,
        },
    },
    SpannedToken {
        kind: Colon,
        span: Span {
            start: 112,
            end: 113,
        },
    },
    SpannedToken {
        kind: Ident(
            "any",
        ),
        span: Span {
            start: 114,
            end: 117,
        },
    },
    SpannedToken {
        kind: RightParen,
        span: Span {
            start: 117,
            end: 118,
        },
    },
    SpannedToken {
        kind: Colon,
        span: Span {
            start: 118,
            end: 119,
        },
    },
    SpannedToken {
        kind: Void,
        span: Span {
            start: 120,
            end: 124,
        },
    },
    SpannedToken {
        kind: Semicolon,
        span: Span {
            start: 124,
            end: 125,
        },
    },
    SpannedToken {
        kind: For,
        span: Span {
            start: 127,
            end: 130,
        },
    },
    SpannedToken {
        kind: LeftParen,
        span: Span {
            start: 131,
            end: 132,
        },
    },
    SpannedToken {
        kind: Const,
        span: Span {
            start: 132,
            end: 137,
        },
    },
    SpannedToken {
        kind: Ident(
            "x",
        ),
        span: Span {
            start: 138,
            end: 139,
        },
    },
    SpannedToken {
        kind: Of,
        span: Span {
            start: 140,
            end: 142,
        },
    },
    SpannedToken {
        kind: LeftBracket,
        span: Span {
            start: 143,
            end: 144,
        },
    },
    SpannedToken {
        kind: Number(
            1,
        ),
        span: Span {
            start: 144,
            end: 145,
        },
    },
    SpannedToken {
        kind: Comma,
        span: Span {
            start: 145,
            end: 146,
        },
    },
    SpannedToken {
        kind: Number(
            2,
        ),
        span: Span {
            start: 147,
            end: 148,
        },
    },
    SpannedToken {
        kind: Comma,
        span: Span {
            start: 148,
            end: 149,
        },
    },
    SpannedToken {
        kind: Number(
            3,
        ),
        span: Span {
            start: 150,
            end: 151,
        },
    },
    SpannedToken {
        kind: RightBracket,
        span: Span {
            start: 151,
            end: 152,
        },
    },
    SpannedToken {
        kind: RightParen,
        span: Span {
            start: 152,
            end: 153,
        },
    },
    SpannedToken {
        kind: LeftBrace,
        span: Span {
            start: 154,
```

#### ast

- ok: `True`
- truncated: `True`

```text
== ast ==
[
    ForOf {
        var: "x",
        iter: Array {
            elements: [
                Present(
                    Number {
                        value: 1,
                        span: Span {
                            start: 144,
                            end: 145,
                        },
                    },
                ),
                Present(
                    Number {
                        value: 2,
                        span: Span {
                            start: 147,
                            end: 148,
                        },
                    },
                ),
                Present(
                    Number {
                        value: 3,
                        span: Span {
                            start: 150,
                            end: 151,
                        },
                    },
                ),
            ],
            span: Span {
                start: 143,
                end: 152,
            },
        },
        body: [
            Expr {
                expr: Call {
                    callee: Ident {
                        name: "log",
                        span: Span {
                            start: 160,
                            end: 163,
                        },
                    },
                    args: [
                        Ident {
                            name: "x",
                            span: Span {
                                start: 164,
                                end: 165,
                            },
                        },
                    ],
                    span: Span {
                        start: 160,
                        end: 166,
                    },
                },
                span: Span {
                    start: 160,
                    end: 167,
                },
            },
        ],
        span: Span {
            start: 127,
            end: 167,
        },
    },
    ForOf {
        var: "x",
        iter: Ident {
            name: "aString",
            span: Span {
                start: 219,
                end: 226,
            },
        },
        body: [
            Expr {
                expr: Call {
                    callee: Ident {
                        name: "log",
                        span: Span {
                            start: 234,
                            end: 237,
                        },
                    },
                    args: [
                        Ident {
                            name: "x",
                            span: Span {
                                start: 238,
                                end: 239,
                            },
                        },
                    ],
                    span: Span {
                        start: 234,
                        end: 240,
                    },
                },
                span: Span {
                    start: 234,
                    end: 241,
                },
            },
        ],
        span: Span {
            start: 203,
            end: 241,
        },
    },
    ForOf {
        var: "x",
        iter: Ident {
            name: "aNumber",
            span: Span {
                start: 293,
                end: 300,
            },
        },
        body: [
            Expr {
                expr: Call {
                    callee: Ident {
                        name: "log",
```

#### resolved

- ok: `False`
- truncated: `False`

```text
error: [UnresolvedName] unresolved name: `log` at 160..163
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
        "code": 2488,
        "category": "Error",
        "message": "Type 'number' must have a '[Symbol.iterator]()' method that returns an iterator.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/arrayIterationLibES5TargetDifferent.ts",
        "start": 293,
        "length": 7,
        "line": 20,
        "character": 17
      },
      {
        "code": 2488,
        "category": "Error",
        "message": "Type '{ foo: string; }' must have a '[Symbol.iterator]()' method that returns an iterator.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/arrayIterationLibES5TargetDifferent.ts",
        "start": 377,
        "length": 8,
        "line": 26,
        "character": 17
      }
    ],
    "hints": [
      {
        "kind": "function",
        "typeText": "void",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/arrayIterationLibES5TargetDifferent.ts",
        "start": 100,
        "length": 3,
        "line": 6,
        "character": 18,
        "name": "log"
      },
      {
        "kind": "parameter",
        "typeText": "any",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/arrayIterationLibES5TargetDifferent.ts",
        "start": 104,
        "length": 7,
        "line": 6,
        "character": 22,
        "name": "message"
      },
      {
        "kind": "binding",
        "typeText": "number",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/arrayIterationLibES5TargetDifferent.ts",
        "start": 138,
        "length": 1,
        "line": 8,
        "character": 12,
        "name": "x"
      },
      {
        "kind": "binding",
        "typeText": "string",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/arrayIterationLibES5TargetDifferent.ts",
        "start": 185,
        "length": 7,
        "line": 12,
        "character": 15,
        "name": "aString"
      },
      {
        "kind": "binding",
        "typeText": "string",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/arrayIterationLibES5TargetDifferent.ts",
        "start": 214,
        "length": 1,
        "line": 14,
        "character": 12,
        "name": "x"
      },
      {
        "kind": "binding",
        "typeText": "number",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/arrayIterationLibES5TargetDifferent.ts",
        "start": 259,
        "length": 7,
        "line": 18,
        "character": 15,
        "name": "aNumber"
      },
      {
        "kind": "binding",
        "typeText": "any",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/arrayIterationLibES5TargetDifferent.ts",
        "start": 288,
        "length": 1,
        "line": 20,
        "character": 12,
        "name": "x"
      },
      {
        "kind": "binding",
        "typeText": "{ foo: string; }",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/arrayIterationLibES5TargetDifferent.ts",
        "start": 333,
        "length": 8,
        "line": 24,
        "character": 15,
        "name": "anObject"
      },
      {
        "kind": "binding",
        "typeText": "any",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/arrayIterationLibES5TargetDifferent.ts",
        "start": 372,
        "length": 1,
        "line": 26,
        "character": 12,
        "name": "x"
      }
    ],
    "typescriptVersion": "6.0.3"
  },
  "ast": {
    "topLevel": [
      {
        "kind": "FunctionDeclaration",
        "text": "declare function log(message?: any): void;",
        "line": 6,
        "character": 1
      },
      {
        "kind": "ForOfStatement",
        "text": "for (const x of [1, 2, 3]) {\n    log(x);\n}",
        "line": 8,
        "character": 1
      },
      {
        "kind": "FirstStatement",
        "text": "declare const aString: string;",
        "line": 12,
        "character": 1
      },
      {
        "kind": "ForOfStatement",
        "text": "for (const x of aString) {\n    log(x);\n}",
        "line": 14,
        "character": 1
      },
      {
        "kind": "FirstStatement",
        "text": "declare const aNumber: number;",
        "line": 18,
        "character": 1
      },
      {
        "kind": "ForOfStatement",
        "text": "for (const x of aNumber) {\n    log(x);\n}",
        "line": 20,
        "character": 1
      },
      {
        "kind": "FirstStatement",
        "text": "declare const anObject: { foo: string };",
        "line": 24,
        "character": 1
      },
      {
        "kind": "ForOfStatement",
        "text": "for (const x of anObject) {\n    log(x);\n}",
        "line": 26,
        "character": 1
      }
    ],
    "pathToPosition": [
      {
        "kind": "SourceFile",
        "text": "declare function log(message?: any): void;\n\nfor (const x of [1, 2, 3]) {\n    log(x);\n}\n\ndeclare const aString: string;\n\n",
        "line": 6,
        "character": 1
      },
      {
        "kind": "ForOfStatement",
        "text": "for (const x of [1, 2, 3]) {\n    log(x);\n}",
        "line": 8,
        "character": 1
      },
      {
        "kind": "Block",
        "text": "{\n    log(x);\n}",
        "line": 8,
        "character": 28
      },
      {
        "kind": "ExpressionStatement",
        "text": "log(x);",
        "line": 9,
        "character": 5
      },
      {
        "kind": "CallExpression",
        "text": "log(x)",
        "line": 9,
        "character": 5
      },
      {
        "kind": "Identifier",
        "text": "log",
        "line": 9,
        "character": 5
      }
    ]
  }
}
```

Stack trace:

```text
error: [UnresolvedName] unresolved name: `log` at 160..163
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
