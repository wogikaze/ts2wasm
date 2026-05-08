---
id: 648
title: "Implement Argumentsaspropertyname Name Resolution"
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

Triage argumentsAsPropertyName-name-resolution across 1 failing reference test cases and split this bucket into implementation-ready child issues.

## Problem

Reference test results show 1 cases fail in directory `argumentsAsPropertyName-name-resolution` with diagnostics: name-resolution. The compiler cannot handle these syntax/semantics, preventing compilation of code in this category.

Problem: argumentsAsPropertyName-name-resolution has 1 reference failures and needs smart-triage evidence before implementation starts.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/argumentsAsPropertyName.ts
```

Coverage window:

```sh
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/argumentsAsPropertyName.ts --detail
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
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/argumentsAsPropertyName.ts --detail
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/argumentsAsPropertyName.ts
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

- `reference/typescript/tests/cases/compiler/argumentsAsPropertyName.ts`

## Duplicate detection

- `issues/open/064-implement-name-resolution.md` - Implement name resolution (triaged - superseded by test262 metadata issues) (same feature label, title overlap)
- `issues/open/194-implement-argumentsAsPropertyName.md` - Implement Argumentsaspropertyname (same reference path, title overlap)
- `issues/open/437-implement-name-resolution.md` - Implement name resolution (same feature label, title overlap)

## Smart triage

### Smart triage: Triage name resolution: argumentsAsPropertyName

- Issue class: `triage-needed`
- Feature label: `name-resolution`
- Diagnostic: `UnresolvedName` / `resolver-symbol`
- Path: `reference/typescript/tests/cases/compiler/argumentsAsPropertyName.ts`

Reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/argumentsAsPropertyName.ts
```

Source overview:

```json
{
  "suite": "tsc",
  "bytes": 389,
  "lines": 17,
  "extension": ".ts",
  "first_code_line": "type MyType = {"
}
```

Failure location:

```json
{
  "code": "UnresolvedName",
  "message": "unresolved name: `use` at 223..226",
  "span_start": 223,
  "span_end": 226,
  "line": 12,
  "column": 20,
  "feature_label": "name-resolution",
  "error_type": "resolver-symbol"
}
```

Source context:

```text
 9 | 
10 | function myFunction(myType: MyType) {
11 |     for (let i = 0; i < 10; i++) {
12 |         use(myType.arguments[i]);
13 |         // create closure so that tsc will turn loop body into function
14 |         const x = 5;
15 |         [1, 2, 3].forEach(function(j) { use(x); })
```

Visible symbols before failure:

```json
[
  {
    "kind": "function",
    "name": "use",
    "line": 8,
    "column": 9,
    "params": "s: any"
  },
  {
    "kind": "function",
    "name": "myFunction",
    "line": 10,
    "column": 1,
    "params": "myType: MyType"
  },
  {
    "kind": "binding",
    "name": "i",
    "line": 11,
    "column": 10,
    "initializer": "0"
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
    "path": "issues/open/194-implement-argumentsAsPropertyName.md",
    "title": "Implement Argumentsaspropertyname",
    "reason": "same reference path, title overlap"
  },
  {
    "state": "open",
    "path": "issues/open/437-implement-name-resolution.md",
    "title": "Implement name resolution",
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
            "type",
        ),
        span: Span {
            start: 55,
            end: 59,
        },
    },
    SpannedToken {
        kind: Ident(
            "MyType",
        ),
        span: Span {
            start: 60,
            end: 66,
        },
    },
    SpannedToken {
        kind: Equal,
        span: Span {
            start: 67,
            end: 68,
        },
    },
    SpannedToken {
        kind: LeftBrace,
        span: Span {
            start: 69,
            end: 70,
        },
    },
    SpannedToken {
        kind: Ident(
            "arguments",
        ),
        span: Span {
            start: 76,
            end: 85,
        },
    },
    SpannedToken {
        kind: Colon,
        span: Span {
            start: 85,
            end: 86,
        },
    },
    SpannedToken {
        kind: Ident(
            "Array",
        ),
        span: Span {
            start: 87,
            end: 92,
        },
    },
    SpannedToken {
        kind: Less,
        span: Span {
            start: 92,
            end: 93,
        },
    },
    SpannedToken {
        kind: Ident(
            "string",
        ),
        span: Span {
            start: 93,
            end: 99,
        },
    },
    SpannedToken {
        kind: Greater,
        span: Span {
            start: 99,
            end: 100,
        },
    },
    SpannedToken {
        kind: RightBrace,
        span: Span {
            start: 102,
            end: 103,
        },
    },
    SpannedToken {
        kind: Ident(
            "declare",
        ),
        span: Span {
            start: 107,
            end: 114,
        },
    },
    SpannedToken {
        kind: Function,
        span: Span {
            start: 115,
            end: 123,
        },
    },
    SpannedToken {
        kind: Ident(
            "use",
        ),
        span: Span {
            start: 124,
            end: 127,
        },
    },
    SpannedToken {
        kind: LeftParen,
        span: Span {
            start: 127,
            end: 128,
        },
    },
    SpannedToken {
        kind: Ident(
            "s",
        ),
        span: Span {
            start: 128,
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
        kind: Ident(
            "any",
        ),
        span: Span {
            start: 131,
            end: 134,
        },
    },
    SpannedToken {
        kind: RightParen,
        span: Span {
            start: 134,
            end: 135,
        },
    },
    SpannedToken {
        kind: Semicolon,
        span: Span {
            start: 135,
            end: 136,
        },
    },
    SpannedToken {
        kind: Function,
        span: Span {
            start: 140,
            end: 148,
        },
    },
    SpannedToken {
        kind: Ident(
            "myFunction",
        ),
        span: Span {
            start: 149,
            end: 159,
        },
    },
    SpannedToken {
        kind: LeftParen,
        span: Span {
            start: 159,
            end: 160,
        },
    },
    SpannedToken {
        kind: Ident(
            "myType",
        ),
        span: Span {
            start: 160,
            end: 166,
        },
    },
    SpannedToken {
        kind: Colon,
        span: Span {
            start: 166,
            end: 167,
```

#### ast

- ok: `True`
- truncated: `True`

```text
== ast ==
[
    Function {
        name: "myFunction",
        params: [
            (
                "myType",
                None,
                false,
            ),
        ],
        body: [
            For {
                init: Some(
                    Let {
                        name: "i",
                        expr: Number {
                            value: 0,
                            span: Span {
                                start: 196,
                                end: 197,
                            },
                        },
                        span: Span {
                            start: 188,
                            end: 198,
                        },
                    },
                ),
                condition: Some(
                    Binary {
                        left: Ident {
                            name: "i",
                            span: Span {
                                start: 199,
                                end: 200,
                            },
                        },
                        op: Less,
                        right: Number {
                            value: 10,
                            span: Span {
                                start: 203,
                                end: 205,
                            },
                        },
                        span: Span {
                            start: 199,
                            end: 205,
                        },
                    },
                ),
                update: Some(
                    Unary {
                        op: Increment,
                        expr: Ident {
                            name: "i",
                            span: Span {
                                start: 207,
                                end: 208,
                            },
                        },
                        span: Span {
                            start: 207,
                            end: 210,
                        },
                    },
                ),
                body: [
                    Expr {
                        expr: Call {
                            callee: Ident {
                                name: "use",
                                span: Span {
                                    start: 223,
                                    end: 226,
                                },
                            },
                            args: [
                                Index {
                                    object: Member {
                                        object: Ident {
                                            name: "myType",
                                            span: Span {
                                                start: 227,
                                                end: 233,
                                            },
                                        },
                                        property: "arguments",
                                        span: Span {
                                            start: 227,
                                            end: 243,
                                        },
                                    },
                                    index: Ident {
                                        name: "i",
                                        span: Span {
```

#### resolved

- ok: `False`
- truncated: `False`

```text
error: [UnresolvedName] unresolved name: `use` at 223..226
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
        "typeText": "any",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/argumentsAsPropertyName.ts",
        "start": 124,
        "length": 3,
        "line": 8,
        "character": 18,
        "name": "use"
      },
      {
        "kind": "parameter",
        "typeText": "any",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/argumentsAsPropertyName.ts",
        "start": 128,
        "length": 1,
        "line": 8,
        "character": 22,
        "name": "s"
      },
      {
        "kind": "function",
        "typeText": "void",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/argumentsAsPropertyName.ts",
        "start": 149,
        "length": 10,
        "line": 10,
        "character": 10,
        "name": "myFunction"
      },
      {
        "kind": "parameter",
        "typeText": "MyType",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/argumentsAsPropertyName.ts",
        "start": 160,
        "length": 6,
        "line": 10,
        "character": 21,
        "name": "myType"
      },
      {
        "kind": "binding",
        "typeText": "number",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/argumentsAsPropertyName.ts",
        "start": 192,
        "length": 1,
        "line": 11,
        "character": 14,
        "name": "i"
      },
      {
        "kind": "binding",
        "typeText": "5",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/argumentsAsPropertyName.ts",
        "start": 337,
        "length": 1,
        "line": 14,
        "character": 15,
        "name": "x"
      },
      {
        "kind": "parameter",
        "typeText": "number",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/argumentsAsPropertyName.ts",
        "start": 380,
        "length": 1,
        "line": 15,
        "character": 36,
        "name": "j"
      }
    ],
    "typescriptVersion": "6.0.3"
  },
  "ast": {
    "topLevel": [
      {
        "kind": "TypeAliasDeclaration",
        "text": "type MyType = {\r\n    arguments: Array<string>\r\n}",
        "line": 4,
        "character": 1
      },
      {
        "kind": "FunctionDeclaration",
        "text": "declare function use(s: any);",
        "line": 8,
        "character": 1
      },
      {
        "kind": "FunctionDeclaration",
        "text": "function myFunction(myType: MyType) {\r\n    for (let i = 0; i < 10; i++) {\r\n        use(myType.arguments[i]);\r\n        //",
        "line": 10,
        "character": 1
      }
    ],
    "pathToPosition": [
      {
        "kind": "SourceFile",
        "text": "type MyType = {\r\n    arguments: Array<string>\r\n}\r\n\r\ndeclare function use(s: any);\r\n\r\nfunction myFunction(myType: MyType)",
        "line": 4,
        "character": 1
      },
      {
        "kind": "FunctionDeclaration",
        "text": "function myFunction(myType: MyType) {\r\n    for (let i = 0; i < 10; i++) {\r\n        use(myType.arguments[i]);\r\n        //",
        "line": 10,
        "character": 1
      },
      {
        "kind": "Block",
        "text": "{\r\n    for (let i = 0; i < 10; i++) {\r\n        use(myType.arguments[i]);\r\n        // create closure so that tsc will tur",
        "line": 10,
        "character": 37
      },
      {
        "kind": "ForStatement",
        "text": "for (let i = 0; i < 10; i++) {\r\n        use(myType.arguments[i]);\r\n        // create closure so that tsc will turn loop ",
        "line": 11,
        "character": 5
      },
      {
        "kind": "Block",
        "text": "{\r\n        use(myType.arguments[i]);\r\n        // create closure so that tsc will turn loop body into function\r\n        c",
        "line": 11,
        "character": 34
      },
      {
        "kind": "ExpressionStatement",
        "text": "use(myType.arguments[i]);",
        "line": 12,
        "character": 9
      },
      {
        "kind": "CallExpression",
        "text": "use(myType.arguments[i])",
        "line": 12,
        "character": 9
      },
      {
        "kind": "Identifier",
        "text": "use",
        "line": 12,
        "character": 9
      }
    ]
  }
}
```

Stack trace:

```text
error: [UnresolvedName] unresolved name: `use` at 223..226
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
