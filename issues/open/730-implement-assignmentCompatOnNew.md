---
id: 730
title: "Implement Assignmentcompatonnew"
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

Triage assignmentCompatOnNew across 1 failing reference test cases and split this bucket into implementation-ready child issues.

## Problem

Reference test results show 1 cases fail in directory `assignmentCompatOnNew` with diagnostics: name-resolution. The compiler cannot handle these syntax/semantics, preventing compilation of code in this category.

Problem: assignmentCompatOnNew has 1 reference failures and needs smart-triage evidence before implementation starts.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/assignmentCompatOnNew.ts
```

Coverage window:

```sh
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/assignmentCompatOnNew.ts --detail
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
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/assignmentCompatOnNew.ts --detail
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/assignmentCompatOnNew.ts
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

- `reference/typescript/tests/cases/compiler/assignmentCompatOnNew.ts`

## Duplicate detection

- none found by path/title/feature scan

## Smart triage

### Smart triage: Triage name resolution: assignmentCompatOnNew

- Issue class: `triage-needed`
- Feature label: `name-resolution`
- Diagnostic: `UnresolvedName` / `resolver-symbol`
- Path: `reference/typescript/tests/cases/compiler/assignmentCompatOnNew.ts`

Reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/assignmentCompatOnNew.ts
```

Source overview:

```json
{
  "suite": "tsc",
  "bytes": 109,
  "lines": 6,
  "extension": ".ts",
  "first_code_line": "class Foo{};"
}
```

Failure location:

```json
{
  "code": "UnresolvedName",
  "message": "unresolved name: `Foo`",
  "span_start": null,
  "span_end": null,
  "line": null,
  "column": null,
  "feature_label": "name-resolution",
  "error_type": "resolver-symbol"
}
```

Source context:

```text
// @target: es2015
class Foo{};

function bar(x: {new(): Foo;}){}

bar(Foo); // Error, but should be allowed
```

Visible symbols before failure:

```json
[
  {
    "kind": "class",
    "name": "Foo",
    "line": 2,
    "column": 1
  },
  {
    "kind": "function",
    "name": "bar",
    "line": 4,
    "column": 1,
    "params": "x: {new("
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
- truncated: `False`

```text
== tokens ==
[
    SpannedToken {
        kind: Class,
        span: Span {
            start: 20,
            end: 25,
        },
    },
    SpannedToken {
        kind: Ident(
            "Foo",
        ),
        span: Span {
            start: 26,
            end: 29,
        },
    },
    SpannedToken {
        kind: LeftBrace,
        span: Span {
            start: 29,
            end: 30,
        },
    },
    SpannedToken {
        kind: RightBrace,
        span: Span {
            start: 30,
            end: 31,
        },
    },
    SpannedToken {
        kind: Semicolon,
        span: Span {
            start: 31,
            end: 32,
        },
    },
    SpannedToken {
        kind: Function,
        span: Span {
            start: 36,
            end: 44,
        },
    },
    SpannedToken {
        kind: Ident(
            "bar",
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
            "x",
        ),
        span: Span {
            start: 49,
            end: 50,
        },
    },
    SpannedToken {
        kind: Colon,
        span: Span {
            start: 50,
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
        kind: New,
        span: Span {
            start: 53,
            end: 56,
        },
    },
    SpannedToken {
        kind: LeftParen,
        span: Span {
            start: 56,
            end: 57,
        },
    },
    SpannedToken {
        kind: RightParen,
        span: Span {
            start: 57,
            end: 58,
        },
    },
    SpannedToken {
        kind: Colon,
        span: Span {
            start: 58,
            end: 59,
        },
    },
    SpannedToken {
        kind: Ident(
            "Foo",
        ),
        span: Span {
            start: 60,
            end: 63,
        },
    },
    SpannedToken {
        kind: Semicolon,
        span: Span {
            start: 63,
            end: 64,
        },
    },
    SpannedToken {
        kind: RightBrace,
        span: Span {
            start: 64,
            end: 65,
        },
    },
    SpannedToken {
        kind: RightParen,
        span: Span {
            start: 65,
            end: 66,
        },
    },
    SpannedToken {
        kind: LeftBrace,
        span: Span {
            start: 66,
            end: 67,
        },
    },
    SpannedToken {
        kind: RightBrace,
        span: Span {
            start: 67,
            end: 68,
        },
    },
    SpannedToken {
        kind: Ident(
            "bar",
        ),
        span: Span {
            start: 72,
            end: 75,
        },
    },
    SpannedToken {
        kind: LeftParen,
        span: Span {
            start: 75,
            end: 76,
        },
    },
    SpannedToken {
        kind: Ident(
            "Foo",
        ),
        span: Span {
            start: 76,
            end: 79,
        },
    },
    SpannedToken {
        kind: RightParen,
        span: Span {
            start: 79,
            end: 80,
        },
    },
    SpannedToken {
        kind: Semicolon,
        span: Span {
            start: 80,
            end: 81,
        },
    },
]
```

#### ast

- ok: `True`
- truncated: `False`

```text
== ast ==
[
    ClassDecl {
        name: "Foo",
        extends: None,
        body: [],
        static_blocks: [],
        private_elements: [],
        span: Span {
            start: 20,
            end: 31,
        },
    },
    Function {
        name: "bar",
        params: [
            (
                "x",
                None,
                false,
            ),
        ],
        body: [],
        is_generator: false,
        span: Span {
            start: 36,
            end: 44,
        },
    },
    Expr {
        expr: Call {
            callee: Ident {
                name: "bar",
                span: Span {
                    start: 72,
                    end: 75,
                },
            },
            args: [
                Ident {
                    name: "Foo",
                    span: Span {
                        start: 76,
                        end: 79,
                    },
                },
            ],
            span: Span {
                start: 72,
                end: 80,
            },
        },
        span: Span {
            start: 72,
            end: 81,
        },
    },
]
```

#### resolved

- ok: `False`
- truncated: `False`

```text
error: [UnresolvedName] unresolved name: `Foo`
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
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/assignmentCompatOnNew.ts",
        "start": 45,
        "length": 3,
        "line": 4,
        "character": 10,
        "name": "bar"
      },
      {
        "kind": "parameter",
        "typeText": "new () => Foo",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/assignmentCompatOnNew.ts",
        "start": 49,
        "length": 1,
        "line": 4,
        "character": 14,
        "name": "x"
      }
    ],
    "typescriptVersion": "6.0.3"
  }
}
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
