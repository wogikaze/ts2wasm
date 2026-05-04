---
id: 193
title: "Implement Arguments (dup)"
type: spike
area: frontend/resolver
class: superseded
priority: P1
depends_on: [5005]
blocks: []
created: 2026-04-29
updated: 2026-05-04
---

## Summary

Triage arguments across 1 failing reference test cases and split this bucket into implementation-ready child issues.

## Problem

Reference test results show 1 cases fail in directory `arguments` with diagnostics: name-resolution. The compiler cannot handle these syntax/semantics, preventing compilation of code in this category.

Problem: arguments has 1 reference failures and needs smart-triage evidence before implementation starts.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/arguments.ts
```

Coverage window:

```sh
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/arguments.ts --detail
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
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/arguments.ts --detail
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/arguments.ts
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

- `reference/typescript/tests/cases/compiler/arguments.ts`

## Duplicate detection

- `issues/done/062d-function-this-and-arguments.md` - Implement function this and arguments semantics (same group key, title overlap)
- `issues/done/195-implement-argumentsBindsToFunctionScopeArgumentList.md` - Implement Argumentsbindstofunctionscopeargumentlist (same feature label, same group key, title overlap)
- `issues/done/039-implement-spread-arguments.md` - Implement spread arguments (same group key, title overlap)
- `issues/done/060-investigate-unknown-unsupported-cases.md` - Investigate and classify unknown-unsupported diagnostic cases (same feature label, same group key)

## Smart triage

### Smart triage: Triage name resolution: arguments

- Issue class: `triage-needed`
- Feature label: `name-resolution`
- Diagnostic: `UnresolvedName` / `resolver-symbol`
- Path: `reference/typescript/tests/cases/compiler/arguments.ts`

Reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/arguments.ts
```

Source overview:

```json
{
  "suite": "tsc",
  "bytes": 334,
  "lines": 15,
  "extension": ".ts",
  "first_code_line": "function f() {"
}
```

Failure location:

```json
{
  "code": "UnresolvedName",
  "message": "unresolved name: `arguments` at 43..52",
  "span_start": 43,
  "span_end": 52,
  "line": 3,
  "column": 13,
  "feature_label": "name-resolution",
  "error_type": "resolver-symbol"
}
```

Source context:

```text
1 | // @target: ES6
2 | function f() {
3 |     var x=arguments[12];
4 |     (() => arguments)();
5 | }
6 |
```

Visible symbols before failure:

```json
[
  {
    "kind": "function",
    "name": "f",
    "line": 2,
    "column": 1,
    "params": ""
  },
  {
    "kind": "binding",
    "name": "x",
    "line": 3,
    "column": 5,
    "initializer": "ar"
  }
]
```

Duplicate candidates:

```json
[
  {
    "state": "open",
    "path": "issues/open/064-implement-name-resolution.md",
    "title": "Implement name resolution",
    "reason": "same feature label, title overlap"
  },
  {
    "state": "open",
    "path": "issues/done/193-implement-arguments.md",
    "title": "Implement Arguments",
    "reason": "same reference path, title overlap"
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
        kind: Function,
        span: Span {
            start: 17,
            end: 25,
        },
    },
    SpannedToken {
        kind: Ident(
            "f",
        ),
        span: Span {
            start: 26,
            end: 27,
        },
    },
    SpannedToken {
        kind: LeftParen,
        span: Span {
            start: 27,
            end: 28,
        },
    },
    SpannedToken {
        kind: RightParen,
        span: Span {
            start: 28,
            end: 29,
        },
    },
    SpannedToken {
        kind: LeftBrace,
        span: Span {
            start: 30,
            end: 31,
        },
    },
    SpannedToken {
        kind: Var,
        span: Span {
            start: 37,
            end: 40,
        },
    },
    SpannedToken {
        kind: Ident(
            "x",
        ),
        span: Span {
            start: 41,
            end: 42,
        },
    },
    SpannedToken {
        kind: Equal,
        span: Span {
            start: 42,
            end: 43,
        },
    },
    SpannedToken {
        kind: Ident(
            "arguments",
        ),
        span: Span {
            start: 43,
            end: 52,
        },
    },
    SpannedToken {
        kind: LeftBracket,
        span: Span {
            start: 52,
            end: 53,
        },
    },
    SpannedToken {
        kind: Number(
            12,
        ),
        span: Span {
            start: 53,
            end: 55,
        },
    },
    SpannedToken {
        kind: RightBracket,
        span: Span {
            start: 55,
            end: 56,
        },
    },
    SpannedToken {
        kind: Semicolon,
        span: Span {
            start: 56,
            end: 57,
        },
    },
    SpannedToken {
        kind: Le
```

#### ast

- ok: `True`
- truncated: `True`

```text
== ast ==
[
    Function {
        name: "f",
        params: [],
        body: [
            Let {
                name: "x",
                expr: Index {
                    object: Ident {
                        name: "arguments",
                        span: Span {
                            start: 43,
                            end: 52,
                        },
                    },
                    index: Number {
                        value: 12,
                        span: Span {
                            start: 53,
                            end: 55,
                        },
                    },
                    span: Span {
                        start: 43,
                        end: 56,
                    },
                },
                span: Span {
                    start: 37,
                    end: 57,
                },
            },
            Expr {
                expr: Call {
                    callee: ArrowFn {
                        params: [],
                        body: Ident {
                            name: "arguments",
                            span: Span {
                                start: 70,
                                end: 79,
                            },
                        },
                        span: Span {
                            start: 64,
                            end: 79,
                        },
                    },
                    args: [],
                    span: Span {
                        start: 64,
                        end: 82,
                    },
                },
                span: Span {
                    start: 64,
                    end: 83,
                },
            },
        ],
        span: Span {
            start: 1
```

#### resolved

- ok: `False`
- truncated: `False`

```text
error: [UnresolvedName] unresolved name: `arguments` at 43..52
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
        "code": 2304,
        "category": "Error",
        "message": "Cannot find name 'arguments'.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/arguments.ts",
        "start": 97,
        "length": 9,
        "line": 7,
        "character": 8
      },
      {
        "code": 2304,
        "category": "Error",
        "message": "Cannot find name 'arguments'.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/arguments.ts",
        "start": 153,
        "length": 9,
        "line": 10,
        "character": 25
      },
      {
        "code": 2304,
        "category": "Error",
        "message": "Cannot find name 'arguments'.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/arguments.ts",
        "start": 194,
        "length": 9,
        "line": 11,
        "character": 23
      },
      {
        "code": 2304,
        "category": "Error",
        "message": "Cannot find name 'arguments'.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/arguments.ts",
        "start": 233,
        "length": 9,
        "line": 12,
        "character": 19
      },
      {
        "code": 2304,
        "category": "Error",
        "message": "Cannot find name 'arguments'.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/arguments.ts",
        "start": 274,
        "length": 9,
        "line": 13,
        "character": 23
      },
      {
        "code": 2304,
        "category": "Error",
        "message": "Cannot find name 'arguments'.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/arguments.ts",
        "start": 326,
        "length": 9,
        "line": 14,
        "character": 34
      }
    ],
    "hints": [
      {
        "kind": "function",
        "typeText": "void",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/arguments.ts",
        "start": 26,
        "length": 1,
        "line": 2,
        "character": 10,
        "name": "f"
      },
      {
        "kind": "binding",
        "typeText": "any",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/arguments.ts",
        "start": 41,
        "length": 1,
        "line": 3,
        "character": 9,
        "name": "x"
      },
      {
        "kind": "parameter",
        "typeText": "any",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/arguments.ts",
        "start": 140,
        "length": 4,
        "line": 10,
        "character": 12,
        "name": "args"
      },
      {
        "kind": "parameter",
        "typeText": "any",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/arguments.ts",
        "start": 181,
        "length": 4,
        "line": 11,
        "character": 10,
        "name": "args"
      },
      {
        "kind": "parameter",
        "typeText": "any",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/arguments.ts",
        "start": 220,
        "length": 4,
        "line": 12,
        "character": 6,
        "name": "args"
      },
      {
        "kind": "parameter",
        "typeText": "any",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/arguments.ts",
        "start": 261,
        "length": 4,
        "line": 13,
        "character": 10,
        "name": "args"
      },
      {
        "kind": "parameter",
        "typeText": "any",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/arguments.ts",
        "start": 313,
        "length": 4,
        "line": 14,
        "character": 21,
        "name": "args"
      }
    ],
    "typescriptVersion": "6.0.3"
  },
  "ast": {
    "topLevel": [
      {
        "kind": "FunctionDeclaration",
        "text": "function f() {\r\n    var x=arguments[12];\r\n    (() => arguments)();\r\n}",
        "line": 2,
        "character": 1
      },
      {
        "kind": "ExpressionStatement",
        "text": "(() => arguments)();",
        "line": 7,
        "character": 1
      },
      {
        "kind": "InterfaceDeclaration",
        "text": "interface I {\r\n    method(args: typeof arguments): void;\r\n    fn: (args: typeof arguments) => void;\r\n    (args: typeof a",
        "line": 9,
        "character": 1
      }
    ],
    "pathToPosition": [
      {
        "kind": "SourceFile",
        "text": "function f() {\r\n    var x=arguments[12];\r\n    (() => arguments)();\r\n}\r\n\r\n(() => arguments)();\r\n\r\ninterface I {\r\n    meth",
        "line": 2,
        "character": 1
      },
      {
        "kind": "FunctionDeclaration",
        "text": "function f() {\r\n    var x=arguments[12];\r\n    (() => arguments)();\r\n}",
        "line": 2,
        "character": 1
      },
      {
        "kind": "Block",
        "text": "{\r\n    var x=arguments[12];\r\n    (() => arguments)();\r\n}",
        "line": 2,
        "character": 14
      },
      {
        "kind": "FirstStatement",
        "text": "var x=arguments[12];",
        "line": 3,
        "character": 5
      },
      {
        "kind": "VariableDeclarationList",
        "text": "var x=arguments[12]",
        "line": 3,
        "character": 5
      },
      {
        "kind": "VariableDeclaration",
        "text": "x=arguments[12]",
        "line": 3,
        "character": 9
      },
      {
        "kind": "ElementAccessExpression",
        "text": "arguments[12]",
        "line": 3,
        "character": 11
      },
      {
        "kind": "Identifier",
        "text": "arguments",
        "line": 3,
        "character": 11
      }
    ]
  }
}
```

Stack trace:

```text
error: [UnresolvedName] unresolved name: `arguments` at 43..52
```


## 重複整理メモ

この issue はタイトルが重複しているため、内容がより充実している
`issues/open/646-implement-arguments.md` に統合されました。
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
