---
id: 722
title: "Implement Assigntoprototype"
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

Triage assignToPrototype across 1 failing reference test cases and split this bucket into implementation-ready child issues.

## Problem

Reference test results show 1 cases fail in directory `assignToPrototype` with diagnostics: name-resolution. The compiler cannot handle these syntax/semantics, preventing compilation of code in this category.

Problem: assignToPrototype has 1 reference failures and needs smart-triage evidence before implementation starts.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/assignToPrototype1.ts
```

Coverage window:

```sh
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/assignToPrototype1.ts --detail
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
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/assignToPrototype1.ts --detail
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/assignToPrototype1.ts
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

- `reference/typescript/tests/cases/compiler/assignToPrototype1.ts`

## Duplicate detection

- none found by path/title/feature scan

## Smart triage

### Smart triage: Triage name resolution: assignToPrototype1

- Issue class: `triage-needed`
- Feature label: `name-resolution`
- Diagnostic: `UnresolvedName` / `resolver-symbol`
- Path: `reference/typescript/tests/cases/compiler/assignToPrototype1.ts`

Reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/assignToPrototype1.ts
```

Source overview:

```json
{
  "suite": "tsc",
  "bytes": 124,
  "lines": 7,
  "extension": ".ts",
  "first_code_line": "declare class Point {"
}
```

Failure location:

```json
{
  "code": "UnresolvedName",
  "message": "unresolved name: `Point` at 86..91",
  "span_start": 86,
  "span_end": 91,
  "line": 6,
  "column": 6,
  "feature_label": "name-resolution",
  "error_type": "resolver-symbol"
}
```

Source context:

```text
3 |   add(dx: number, dy: number): void;
4 | }
5 | 
6 | Point.prototype.add = function(dx, dy) {
7 | };
```

Visible symbols before failure:

```json
[
  {
    "kind": "class",
    "name": "Point",
    "line": 2,
    "column": 9
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
  },
  {
    "state": "open",
    "path": "issues/open/693-implement-arrayToLocaleStringES-name-resolution.md",
    "title": "Implement Arraytolocalestringes Name Resolution",
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
            start: 20,
            end: 27,
        },
    },
    SpannedToken {
        kind: Class,
        span: Span {
            start: 28,
            end: 33,
        },
    },
    SpannedToken {
        kind: Ident(
            "Point",
        ),
        span: Span {
            start: 34,
            end: 39,
        },
    },
    SpannedToken {
        kind: LeftBrace,
        span: Span {
            start: 40,
            end: 41,
        },
    },
    SpannedToken {
        kind: Ident(
            "add",
        ),
        span: Span {
            start: 45,
            end: 48,
        },
    },
    SpannedToken {
        kind: LeftParen,
        span: Span {
            start: 48,
            end: 49,
        },
    },
    SpannedToken {
        kind: Ident(
            "dx",
        ),
        span: Span {
            start: 49,
            end: 51,
        },
    },
    SpannedToken {
        kind: Colon,
        span: Span {
            start: 51,
            end: 52,
        },
    },
    SpannedToken {
        kind: Ident(
            "number",
        ),
        span: Span {
            start: 53,
            end: 59,
        },
    },
    SpannedToken {
        kind: Comma,
        span: Span {
            start: 59,
            end: 60,
        },
    },
    SpannedToken {
        kind: Ident(
            "dy",
        ),
        span: Span {
            start: 61,
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
        kind: Ident(
            "number",
        ),
        span: Span {
            start: 65,
            end: 71,
        },
    },
    SpannedToken {
        kind: RightParen,
        span: Span {
            start: 71,
            end: 72,
        },
    },
    SpannedToken {
        kind: Colon,
        span: Span {
            start: 72,
            end: 73,
        },
    },
    SpannedToken {
        kind: Void,
        span: Span {
            start: 74,
            end: 78,
        },
    },
    SpannedToken {
        kind: Semicolon,
        span: Span {
            start: 78,
            end: 79,
        },
    },
    SpannedToken {
        kind: RightBrace,
        span: Span {
            start: 81,
            end: 82,
        },
    },
    SpannedToken {
        kind: Ident(
            "Point",
        ),
        span: Span {
            start: 86,
            end: 91,
        },
    },
    SpannedToken {
        kind: Dot,
        span: Span {
            start: 91,
            end: 92,
        },
    },
    SpannedToken {
        kind: Ident(
            "prototype",
        ),
        span: Span {
            start: 92,
            end: 101,
        },
    },
    SpannedToken {
        kind: Dot,
        span: Span {
            start: 101,
            end: 102,
        },
    },
    SpannedToken {
        kind: Ident(
            "add",
        ),
        span: Span {
            start: 102,
            end: 105,
        },
    },
    SpannedToken {
        kind: Equal,
        span: Span {
            start: 106,
            end: 107,
        },
    },
    SpannedToken {
        kind: Function,
        span: Span {
            start: 108,
            end: 116,
        },
    },
    SpannedToken {
        kind: LeftParen,
        span: Span
```

#### ast

- ok: `True`
- truncated: `False`

```text
== ast ==
[
    Expr {
        expr: PropertyAssign {
            object: Member {
                object: Ident {
                    name: "Point",
                    span: Span {
                        start: 86,
                        end: 91,
                    },
                },
                property: "prototype",
                span: Span {
                    start: 86,
                    end: 101,
                },
            },
            property: "add",
            value: FunctionExpr {
                name: "",
                params: [
                    (
                        "dx",
                        None,
                        false,
                    ),
                    (
                        "dy",
                        None,
                        false,
                    ),
                ],
                body: [],
                span: Span {
                    start: 108,
                    end: 116,
                },
            },
            span: Span {
                start: 86,
                end: 130,
            },
        },
        span: Span {
            start: 86,
            end: 130,
        },
    },
]
```

#### resolved

- ok: `False`
- truncated: `False`

```text
error: [UnresolvedName] unresolved name: `Point` at 86..91
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
        "kind": "parameter",
        "typeText": "number",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/assignToPrototype1.ts",
        "start": 49,
        "length": 2,
        "line": 3,
        "character": 7,
        "name": "dx"
      },
      {
        "kind": "parameter",
        "typeText": "number",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/assignToPrototype1.ts",
        "start": 61,
        "length": 2,
        "line": 3,
        "character": 19,
        "name": "dy"
      },
      {
        "kind": "parameter",
        "typeText": "number",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/assignToPrototype1.ts",
        "start": 117,
        "length": 2,
        "line": 6,
        "character": 32,
        "name": "dx"
      },
      {
        "kind": "parameter",
        "typeText": "number",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/assignToPrototype1.ts",
        "start": 121,
        "length": 2,
        "line": 6,
        "character": 36,
        "name": "dy"
      }
    ],
    "typescriptVersion": "6.0.3"
  },
  "ast": {
    "topLevel": [
      {
        "kind": "ClassDeclaration",
        "text": "declare class Point {\r\n  add(dx: number, dy: number): void;\r\n}",
        "line": 2,
        "character": 1
      },
      {
        "kind": "ExpressionStatement",
        "text": "Point.prototype.add = function(dx, dy) {\r\n};",
        "line": 6,
        "character": 1
      }
    ],
    "pathToPosition": [
      {
        "kind": "SourceFile",
        "text": "declare class Point {\r\n  add(dx: number, dy: number): void;\r\n}\r\n\r\nPoint.prototype.add = function(dx, dy) {\r\n};",
        "line": 2,
        "character": 1
      },
      {
        "kind": "ExpressionStatement",
        "text": "Point.prototype.add = function(dx, dy) {\r\n};",
        "line": 6,
        "character": 1
      },
      {
        "kind": "BinaryExpression",
        "text": "Point.prototype.add = function(dx, dy) {\r\n}",
        "line": 6,
        "character": 1
      },
      {
        "kind": "PropertyAccessExpression",
        "text": "Point.prototype.add",
        "line": 6,
        "character": 1
      },
      {
        "kind": "PropertyAccessExpression",
        "text": "Point.prototype",
        "line": 6,
        "character": 1
      },
      {
        "kind": "Identifier",
        "text": "Point",
        "line": 6,
        "character": 1
      }
    ]
  }
}
```

Stack trace:

```text
error: [UnresolvedName] unresolved name: `Point` at 86..91
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
