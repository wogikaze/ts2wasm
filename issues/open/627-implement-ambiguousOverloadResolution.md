---
id: 627
title: "Implement Ambiguousoverloadresolution"
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

Triage ambiguousOverloadResolution across 1 failing reference test cases and split this bucket into implementation-ready child issues.

## Problem

Reference test results show 1 cases fail in directory `ambiguousOverloadResolution` with diagnostics: name-resolution. The compiler cannot handle these syntax/semantics, preventing compilation of code in this category.

Problem: ambiguousOverloadResolution has 1 reference failures and needs smart-triage evidence before implementation starts.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/ambiguousOverloadResolution.ts
```

Coverage window:

```sh
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/ambiguousOverloadResolution.ts --detail
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
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/ambiguousOverloadResolution.ts --detail
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/ambiguousOverloadResolution.ts
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

- `reference/typescript/tests/cases/compiler/ambiguousOverloadResolution.ts`

## Duplicate detection

- `issues/open/169-implement-ambiguousOverloadResolution.md` - Implement Ambiguousoverloadresolution (same reference path, same group key, title overlap)

## Smart triage

### Smart triage: Triage name resolution: ambiguousOverloadResolution

- Issue class: `triage-needed`
- Feature label: `name-resolution`
- Diagnostic: `UnresolvedName` / `resolver-symbol`
- Path: `reference/typescript/tests/cases/compiler/ambiguousOverloadResolution.ts`

Reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/ambiguousOverloadResolution.ts
```

Source overview:

```json
{
  "suite": "tsc",
  "bytes": 214,
  "lines": 10,
  "extension": ".ts",
  "first_code_line": "class A { }"
}
```

Failure location:

```json
{
  "code": "UnresolvedName",
  "message": "unresolved name: `f` at 199..200",
  "span_start": 199,
  "span_end": 200,
  "line": 10,
  "column": 26,
  "feature_label": "name-resolution",
  "error_type": "resolver-symbol"
}
```

Source context:

```text
 7 | declare function f(p: B, q: A): string;
 8 | 
 9 | var x: B;
10 | var t: number = f(x, x); // Not an error
```

Visible symbols before failure:

```json
[
  {
    "kind": "class",
    "name": "A",
    "line": 3,
    "column": 1
  },
  {
    "kind": "class",
    "name": "B",
    "line": 4,
    "column": 1
  },
  {
    "kind": "function",
    "name": "f",
    "line": 6,
    "column": 9,
    "params": "p: A, q: B"
  },
  {
    "kind": "function",
    "name": "f",
    "line": 7,
    "column": 9,
    "params": "p: B, q: A"
  },
  {
    "kind": "binding",
    "name": "x",
    "line": 9,
    "column": 1
  },
  {
    "kind": "binding",
    "name": "t",
    "line": 10,
    "column": 1
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
    "path": "issues/done/169-implement-ambiguousOverloadResolution.md",
    "title": "Implement Ambiguousoverloadresolution",
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
        kind: Class,
        span: Span {
            start: 39,
            end: 44,
        },
    },
    SpannedToken {
        kind: Ident(
            "A",
        ),
        span: Span {
            start: 45,
            end: 46,
        },
    },
    SpannedToken {
        kind: LeftBrace,
        span: Span {
            start: 47,
            end: 48,
        },
    },
    SpannedToken {
        kind: RightBrace,
        span: Span {
            start: 49,
            end: 50,
        },
    },
    SpannedToken {
        kind: Class,
        span: Span {
            start: 52,
            end: 57,
        },
    },
    SpannedToken {
        kind: Ident(
            "B",
        ),
        span: Span {
            start: 58,
            end: 59,
        },
    },
    SpannedToken {
        kind: Extends,
        span: Span {
            start: 60,
            end: 67,
        },
    },
    SpannedToken {
        kind: Ident(
            "A",
        ),
        span: Span {
            start: 68,
            end: 69,
        },
    },
    SpannedToken {
        kind: LeftBrace,
        span: Span {
            start: 70,
            end: 71,
        },
    },
    SpannedToken {
        kind: Ident(
            "x",
        ),
        span: Span {
            start: 72,
            end: 73,
        },
    },
    SpannedToken {
        kind: Colon,
        span: Span {
            start: 73,
            end: 74,
        },
    },
    SpannedToken {
        kind: Ident(
            "number",
        ),
        span: Span {
            start: 75,
            end: 81,
        },
    },
    SpannedToken {
        kind: Semicolon,
        span: Span {
            start: 81,
            end: 82,
        },
    },
    SpannedToken {
        kind: RightBrace,
        span: Span {
            start: 83,
            end: 84,
        },
    },
    SpannedToken {
        kind: Ident(
            "declare",
        ),
        span: Span {
            start: 88,
            end: 95,
        },
    },
    SpannedToken {
        kind: Function,
        span: Span {
            start: 96,
            end: 104,
        },
    },
    SpannedToken {
        kind: Ident(
            "f",
        ),
        span: Span {
            start: 105,
            end: 106,
        },
    },
    SpannedToken {
        kind: LeftParen,
        span: Span {
            start: 106,
            end: 107,
        },
    },
    SpannedToken {
        kind: Ident(
            "p",
        ),
        span: Span {
            start: 107,
            end: 108,
        },
    },
    SpannedToken {
        kind: Colon,
        span: Span {
            start: 108,
            end: 109,
        },
    },
    SpannedToken {
        kind: Ident(
            "A",
        ),
        span: Span {
            start: 110,
            end: 111,
        },
    },
    SpannedToken {
        kind: Comma,
        span: Span {
            start: 111,
            end: 112,
        },
    },
    SpannedToken {
        kind: Ident(
            "q",
        ),
        span: Span {
            start: 113,
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
            "B",
        ),
        span: Span {
            start: 116,
            end: 117,
        },
    },
    SpannedToken {
        kind: RightPar
```

#### ast

- ok: `True`
- truncated: `False`

```text
== ast ==
[
    ClassDecl {
        name: "A",
        extends: None,
        body: [],
        static_blocks: [],
        private_elements: [],
        span: Span {
            start: 39,
            end: 50,
        },
    },
    ClassDecl {
        name: "B",
        extends: Some(
            Ident {
                name: "A",
                span: Span {
                    start: 68,
                    end: 69,
                },
            },
        ),
        body: [],
        static_blocks: [],
        private_elements: [],
        span: Span {
            start: 52,
            end: 84,
        },
    },
    Let {
        name: "x",
        expr: Undefined {
            span: Span {
                start: 176,
                end: 177,
            },
        },
        span: Span {
            start: 172,
            end: 181,
        },
    },
    Let {
        name: "t",
        expr: Call {
            callee: Ident {
                name: "f",
                span: Span {
                    start: 199,
                    end: 200,
                },
            },
            args: [
                Ident {
                    name: "x",
                    span: Span {
                        start: 201,
                        end: 202,
                    },
                },
                Ident {
                    name: "x",
                    span: Span {
                        start: 204,
                        end: 205,
                    },
                },
            ],
            span: Span {
                start: 199,
                end: 206,
            },
        },
        span: Span {
            start: 183,
            end: 207,
        },
    },
]
```

#### resolved

- ok: `False`
- truncated: `False`

```text
error: [UnresolvedName] unresolved name: `f` at 199..200
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
        "code": 2564,
        "category": "Error",
        "message": "Property 'x' has no initializer and is not definitely assigned in the constructor.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/ambiguousOverloadResolution.ts",
        "start": 72,
        "length": 1,
        "line": 4,
        "character": 21
      },
      {
        "code": 2454,
        "category": "Error",
        "message": "Variable 'x' is used before being assigned.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/ambiguousOverloadResolution.ts",
        "start": 201,
        "length": 1,
        "line": 10,
        "character": 19
      },
      {
        "code": 2454,
        "category": "Error",
        "message": "Variable 'x' is used before being assigned.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/ambiguousOverloadResolution.ts",
        "start": 204,
        "length": 1,
        "line": 10,
        "character": 22
      }
    ],
    "hints": [
      {
        "kind": "function",
        "typeText": "number",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/ambiguousOverloadResolution.ts",
        "start": 105,
        "length": 1,
        "line": 6,
        "character": 18,
        "name": "f"
      },
      {
        "kind": "parameter",
        "typeText": "A",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/ambiguousOverloadResolution.ts",
        "start": 107,
        "length": 1,
        "line": 6,
        "character": 20,
        "name": "p"
      },
      {
        "kind": "parameter",
        "typeText": "B",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/ambiguousOverloadResolution.ts",
        "start": 113,
        "length": 1,
        "line": 6,
        "character": 26,
        "name": "q"
      },
      {
        "kind": "function",
        "typeText": "string",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/ambiguousOverloadResolution.ts",
        "start": 146,
        "length": 1,
        "line": 7,
        "character": 18,
        "name": "f"
      },
      {
        "kind": "parameter",
        "typeText": "B",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/ambiguousOverloadResolution.ts",
        "start": 148,
        "length": 1,
        "line": 7,
        "character": 20,
        "name": "p"
      },
      {
        "kind": "parameter",
        "typeText": "A",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/ambiguousOverloadResolution.ts",
        "start": 154,
        "length": 1,
        "line": 7,
        "character": 26,
        "name": "q"
      },
      {
        "kind": "binding",
        "typeText": "B",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/ambiguousOverloadResolution.ts",
        "start": 176,
        "length": 1,
        "line": 9,
        "character": 5,
        "name": "x"
      },
      {
        "kind": "binding",
        "typeText": "number",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/ambiguousOverloadResolution.ts",
        "start": 187,
        "length": 1,
        "line": 10,
        "character": 5,
        "name": "t"
      }
    ],
    "typescriptVersion": "6.0.3"
  },
  "ast": {
    "topLevel": [
      {
        "kind": "ClassDeclaration",
        "text": "class A { }",
        "line": 3,
        "character": 1
      },
      {
        "kind": "ClassDeclaration",
        "text": "class B extends A { x: number; }",
        "line": 4,
        "character": 1
      },
      {
        "kind": "FunctionDeclaration",
        "text": "declare function f(p: A, q: B): number;",
        "line": 6,
        "character": 1
      },
      {
        "kind": "FunctionDeclaration",
        "text": "declare function f(p: B, q: A): string;",
        "line": 7,
        "character": 1
      },
      {
        "kind": "FirstStatement",
        "text": "var x: B;",
        "line": 9,
        "character": 1
      },
      {
        "kind": "FirstStatement",
        "text": "var t: number = f(x, x);",
        "line": 10,
        "character": 1
      }
    ],
    "pathToPosition": [
      {
        "kind": "SourceFile",
        "text": "class A { }\r\nclass B extends A { x: number; }\r\n\r\ndeclare function f(p: A, q: B): number;\r\ndeclare function f(p: B, q: A)",
        "line": 3,
        "character": 1
      },
      {
        "kind": "FirstStatement",
        "text": "var t: number = f(x, x);",
        "line": 10,
        "character": 1
      },
      {
        "kind": "VariableDeclarationList",
        "text": "var t: number = f(x, x)",
        "line": 10,
        "character": 1
      },
      {
        "kind": "VariableDeclaration",
        "text": "t: number = f(x, x)",
        "line": 10,
        "character": 5
      },
      {
        "kind": "CallExpression",
        "text": "f(x, x)",
        "line": 10,
        "character": 17
      },
      {
        "kind": "Identifier",
        "text": "f",
        "line": 10,
        "character": 17
      }
    ]
  }
}
```

Stack trace:

```text
error: [UnresolvedName] unresolved name: `f` at 199..200
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
