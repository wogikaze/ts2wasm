---
id: 733
title: "Implement Assignmentcompatability Name Resolution"
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

Triage assignmentCompatability-name-resolution across 4 failing reference test cases and split this bucket into implementation-ready child issues.

## Problem

Reference test results show 4 cases fail in directory `assignmentCompatability-name-resolution` with diagnostics: name-resolution. The compiler cannot handle these syntax/semantics, preventing compilation of code in this category.

Problem: assignmentCompatability-name-resolution has 4 reference failures and needs smart-triage evidence before implementation starts.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/assignmentCompatability44.ts
```

Coverage window:

```sh
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/assignmentCompatability44.ts --detail
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
mise run reference-coverage -- tsc --limit 8
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/assignmentCompatability44.ts --detail
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/assignmentCompatability44.ts
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

- `reference/typescript/tests/cases/compiler/assignmentCompatability44.ts`
- `reference/typescript/tests/cases/compiler/assignmentCompatability45.ts`
- `reference/typescript/tests/cases/compiler/assignmentCompatability_checking-apply-member-off-of-function-interface.ts`
- `reference/typescript/tests/cases/compiler/assignmentCompatability_checking-call-member-off-of-function-interface.ts`

## Duplicate detection

- `issues/open/064-implement-name-resolution.md` - Implement name resolution (triaged - superseded by test262 metadata issues) (same feature label, title overlap)
- `issues/open/437-implement-name-resolution.md` - Implement name resolution (same feature label, title overlap)

## Smart triage

### Smart triage: Triage name resolution: assignmentCompatability44

- Issue class: `triage-needed`
- Feature label: `name-resolution`
- Diagnostic: `UnresolvedName` / `resolver-symbol`
- Path: `reference/typescript/tests/cases/compiler/assignmentCompatability44.ts`

Reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/assignmentCompatability44.ts
```

Source overview:

```json
{
  "suite": "tsc",
  "bytes": 97,
  "lines": 6,
  "extension": ".ts",
  "first_code_line": "class Foo {"
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
class Foo {
    constructor(x: number) {}
}

const foo: { new(): Foo } = Foo;
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
    "kind": "binding",
    "name": "foo",
    "line": 6,
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
            start: 19,
            end: 24,
        },
    },
    SpannedToken {
        kind: Ident(
            "Foo",
        ),
        span: Span {
            start: 25,
            end: 28,
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
        kind: Ident(
            "constructor",
        ),
        span: Span {
            start: 35,
            end: 46,
        },
    },
    SpannedToken {
        kind: LeftParen,
        span: Span {
            start: 46,
            end: 47,
        },
    },
    SpannedToken {
        kind: Ident(
            "x",
        ),
        span: Span {
            start: 47,
            end: 48,
        },
    },
    SpannedToken {
        kind: Colon,
        span: Span {
            start: 48,
            end: 49,
        },
    },
    SpannedToken {
        kind: Ident(
            "number",
        ),
        span: Span {
            start: 50,
            end: 56,
        },
    },
    SpannedToken {
        kind: RightParen,
        span: Span {
            start: 56,
            end: 57,
        },
    },
    SpannedToken {
        kind: LeftBrace,
        span: Span {
            start: 58,
            end: 59,
        },
    },
    SpannedToken {
        kind: RightBrace,
        span: Span {
            start: 59,
            end: 60,
        },
    },
    SpannedToken {
        kind: RightBrace,
        span: Span {
            start: 61,
            end: 62,
        },
    },
    SpannedToken {
        kind: Const,
        span: Span {
            start: 64,
            end: 69,
        },
    },
    SpannedToken {
        kind: Ident(
            "foo",
        ),
        span: Span {
            start: 70,
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
        kind: LeftBrace,
        span: Span {
            start: 75,
            end: 76,
        },
    },
    SpannedToken {
        kind: New,
        span: Span {
            start: 77,
            end: 80,
        },
    },
    SpannedToken {
        kind: LeftParen,
        span: Span {
            start: 80,
            end: 81,
        },
    },
    SpannedToken {
        kind: RightParen,
        span: Span {
            start: 81,
            end: 82,
        },
    },
    SpannedToken {
        kind: Colon,
        span: Span {
            start: 82,
            end: 83,
        },
    },
    SpannedToken {
        kind: Ident(
            "Foo",
        ),
        span: Span {
            start: 84,
            end: 87,
        },
    },
    SpannedToken {
        kind: RightBrace,
        span: Span {
            start: 88,
            end: 89,
        },
    },
    SpannedToken {
        kind: Equal,
        span: Span {
            start: 90,
            end: 91,
        },
    },
    SpannedToken {
        kind: Ident(
            "Foo",
        ),
        span: Span {
            start: 92,
            end: 95,
        },
    },
    SpannedToken {
        kind: Semicolon,
        span: Span {
            start: 95,
            end: 96,
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
        body: [
            Function {
                name: "constructor",
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
                    start: 35,
                    end: 46,
                },
            },
        ],
        static_blocks: [],
        private_elements: [],
        span: Span {
            start: 19,
            end: 62,
        },
    },
    Let {
        name: "foo",
        expr: Ident {
            name: "Foo",
            span: Span {
                start: 92,
                end: 95,
            },
        },
        span: Span {
            start: 64,
            end: 96,
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
    "ok": false,
    "diagnostics": [
      {
        "code": 2322,
        "category": "Error",
        "message": "Type 'typeof Foo' is not assignable to type 'new () => Foo'.\n  Types of construct signatures are incompatible.\n    Type 'new (x: number) => Foo' is not assignable to type 'new () => Foo'.\n      Target signature provides too few arguments. Expected 1 or more, but got 0.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/assignmentCompatability44.ts",
        "start": 70,
        "length": 3,
        "line": 6,
        "character": 7
      }
    ],
    "hints": [
      {
        "kind": "parameter",
        "typeText": "number",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/assignmentCompatability44.ts",
        "start": 47,
        "length": 1,
        "line": 3,
        "character": 17,
        "name": "x"
      },
      {
        "kind": "binding",
        "typeText": "new () => Foo",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/assignmentCompatability44.ts",
        "start": 70,
        "length": 3,
        "line": 6,
        "character": 7,
        "name": "foo"
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
