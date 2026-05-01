---
id: 775
title: "Implement Autotypeassignedusingdestructuringfromnevernocrash"
type: spike
area: frontend/resolver
class: triage-needed
priority: P1
depends_on: []
blocks: []
created: 2026-05-01
updated: 2026-05-01
---

## Summary

Triage autoTypeAssignedUsingDestructuringFromNeverNoCrash across 1 failing reference test cases and split this bucket into implementation-ready child issues.

## Problem

Reference test results show 1 cases fail in directory `autoTypeAssignedUsingDestructuringFromNeverNoCrash` with diagnostics: name-resolution. The compiler cannot handle these syntax/semantics, preventing compilation of code in this category.

Problem: autoTypeAssignedUsingDestructuringFromNeverNoCrash has 1 reference failures and needs smart-triage evidence before implementation starts.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/autoTypeAssignedUsingDestructuringFromNeverNoCrash.ts
```

Coverage window:

```sh
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/autoTypeAssignedUsingDestructuringFromNeverNoCrash.ts --detail
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
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/autoTypeAssignedUsingDestructuringFromNeverNoCrash.ts --detail
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/autoTypeAssignedUsingDestructuringFromNeverNoCrash.ts
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

- `reference/typescript/tests/cases/compiler/autoTypeAssignedUsingDestructuringFromNeverNoCrash.ts`

## Duplicate detection

- none found by path/title/feature scan

## Smart triage

### Smart triage: Triage name resolution: autoTypeAssignedUsingDestructuringFromNeverNoCrash

- Issue class: `triage-needed`
- Feature label: `name-resolution`
- Diagnostic: `UnresolvedName` / `resolver-symbol`
- Path: `reference/typescript/tests/cases/compiler/autoTypeAssignedUsingDestructuringFromNeverNoCrash.ts`

Reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/autoTypeAssignedUsingDestructuringFromNeverNoCrash.ts
```

Source overview:

```json
{
  "suite": "tsc",
  "bytes": 232,
  "lines": 16,
  "extension": ".ts",
  "first_code_line": "declare const b: null;"
}
```

Failure location:

```json
{
  "code": "UnresolvedName",
  "message": "unresolved name: `b` at 149..150",
  "span_start": 149,
  "span_end": 150,
  "line": 10,
  "column": 5,
  "feature_label": "name-resolution",
  "error_type": "resolver-symbol"
}
```

Source context:

```text
 7 | declare const b: null;
 8 | let file;
 9 | 
10 | if (b === null) {
11 |   // empty
12 | } else {
13 |   [file] = b;
```

Visible symbols before failure:

```json
[
  {
    "kind": "binding",
    "name": "b",
    "line": 7,
    "column": 9
  },
  {
    "kind": "binding",
    "name": "file",
    "line": 8,
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
  },
  {
    "state": "open",
    "path": "issues/open/733-implement-assignmentCompatability-name-resolution.md",
    "title": "Implement Assignmentcompatability Name Resolution",
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
            start: 111,
            end: 118,
        },
    },
    SpannedToken {
        kind: Const,
        span: Span {
            start: 119,
            end: 124,
        },
    },
    SpannedToken {
        kind: Ident(
            "b",
        ),
        span: Span {
            start: 125,
            end: 126,
        },
    },
    SpannedToken {
        kind: Colon,
        span: Span {
            start: 126,
            end: 127,
        },
    },
    SpannedToken {
        kind: Null,
        span: Span {
            start: 128,
            end: 132,
        },
    },
    SpannedToken {
        kind: Semicolon,
        span: Span {
            start: 132,
            end: 133,
        },
    },
    SpannedToken {
        kind: Let,
        span: Span {
            start: 134,
            end: 137,
        },
    },
    SpannedToken {
        kind: Ident(
            "file",
        ),
        span: Span {
            start: 138,
            end: 142,
        },
    },
    SpannedToken {
        kind: Semicolon,
        span: Span {
            start: 142,
            end: 143,
        },
    },
    SpannedToken {
        kind: If,
        span: Span {
            start: 145,
            end: 147,
        },
    },
    SpannedToken {
        kind: LeftParen,
        span: Span {
            start: 148,
            end: 149,
        },
    },
    SpannedToken {
        kind: Ident(
            "b",
        ),
        span: Span {
            start: 149,
            end: 150,
        },
    },
    SpannedToken {
        kind: StrictEqual,
        span: Span {
            start: 151,
            end: 154,
        },
    },
    SpannedToken {
        kind: Null,
        span: Span {
            start: 155,
            end: 159,
        },
    },
    SpannedToken {
        kind: RightParen,
        span: Span {
            start: 159,
            end: 160,
        },
    },
    SpannedToken {
        kind: LeftBrace,
        span: Span {
            start: 161,
            end: 162,
        },
    },
    SpannedToken {
        kind: RightBrace,
        span: Span {
            start: 174,
            end: 175,
        },
    },
    SpannedToken {
        kind: Else,
        span: Span {
            start: 176,
            end: 180,
        },
    },
    SpannedToken {
        kind: LeftBrace,
        span: Span {
            start: 181,
            end: 182,
        },
    },
    SpannedToken {
        kind: LeftBracket,
        span: Span {
            start: 185,
            end: 186,
        },
    },
    SpannedToken {
        kind: Ident(
            "file",
        ),
        span: Span {
            start: 186,
            end: 190,
        },
    },
    SpannedToken {
        kind: RightBracket,
        span: Span {
            start: 190,
            end: 191,
        },
    },
    SpannedToken {
        kind: Equal,
        span: Span {
            start: 192,
            end: 193,
        },
    },
    SpannedToken {
        kind: Ident(
            "b",
        ),
        span: Span {
            start: 194,
            end: 195,
        },
    },
    SpannedToken {
        kind: Semicolon,
        span: Span {
            start: 195,
            end: 196,
        },
    },
    SpannedToken {
        kind: RightBrace,
        span: Span {
            start: 197,
            end: 198,
        },
    },
    S
```

#### ast

- ok: `True`
- truncated: `False`

```text
== ast ==
[
    Let {
        name: "file",
        expr: Undefined {
            span: Span {
                start: 138,
                end: 142,
            },
        },
        span: Span {
            start: 134,
            end: 143,
        },
    },
    If {
        condition: Binary {
            left: Ident {
                name: "b",
                span: Span {
                    start: 149,
                    end: 150,
                },
            },
            op: StrictEqual,
            right: Null {
                span: Span {
                    start: 155,
                    end: 159,
                },
            },
            span: Span {
                start: 149,
                end: 159,
            },
        },
        then_body: [],
        else_body: [
            Expr {
                expr: Assign {
                    name: "[file]",
                    expr: Ident {
                        name: "b",
                        span: Span {
                            start: 194,
                            end: 195,
                        },
                    },
                    span: Span {
                        start: 185,
                        end: 195,
                    },
                },
                span: Span {
                    start: 185,
                    end: 196,
                },
            },
        ],
        span: Span {
            start: 145,
            end: 196,
        },
    },
    Expr {
        expr: Ident {
            name: "file",
            span: Span {
                start: 200,
                end: 204,
            },
        },
        span: Span {
            start: 200,
            end: 205,
        },
    },
]
```

#### resolved

- ok: `False`
- truncated: `False`

```text
error: [UnresolvedName] unresolved name: `b` at 149..150
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
        "message": "Type 'never' must have a '[Symbol.iterator]()' method that returns an iterator.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/autoTypeAssignedUsingDestructuringFromNeverNoCrash.ts",
        "start": 185,
        "length": 6,
        "line": 13,
        "character": 3
      }
    ],
    "hints": [
      {
        "kind": "binding",
        "typeText": "null",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/autoTypeAssignedUsingDestructuringFromNeverNoCrash.ts",
        "start": 125,
        "length": 1,
        "line": 7,
        "character": 15,
        "name": "b"
      },
      {
        "kind": "binding",
        "typeText": "any",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/autoTypeAssignedUsingDestructuringFromNeverNoCrash.ts",
        "start": 138,
        "length": 4,
        "line": 8,
        "character": 5,
        "name": "file"
      }
    ],
    "typescriptVersion": "6.0.3"
  },
  "ast": {
    "topLevel": [
      {
        "kind": "FirstStatement",
        "text": "declare const b: null;",
        "line": 7,
        "character": 1
      },
      {
        "kind": "FirstStatement",
        "text": "let file;",
        "line": 8,
        "character": 1
      },
      {
        "kind": "IfStatement",
        "text": "if (b === null) {\n  // empty\n} else {\n  [file] = b;\n}",
        "line": 10,
        "character": 1
      },
      {
        "kind": "ExpressionStatement",
        "text": "file;",
        "line": 16,
        "character": 1
      }
    ],
    "pathToPosition": [
      {
        "kind": "SourceFile",
        "text": "declare const b: null;\nlet file;\n\nif (b === null) {\n  // empty\n} else {\n  [file] = b;\n}\n\nfile; // request flow type here",
        "line": 7,
        "character": 1
      },
      {
        "kind": "IfStatement",
        "text": "if (b === null) {\n  // empty\n} else {\n  [file] = b;\n}",
        "line": 10,
        "character": 1
      },
      {
        "kind": "BinaryExpression",
        "text": "b === null",
        "line": 10,
        "character": 5
      },
      {
        "kind": "Identifier",
        "text": "b",
        "line": 10,
        "character": 5
      }
    ]
  }
}
```

Stack trace:

```text
error: [UnresolvedName] unresolved name: `b` at 149..150
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
