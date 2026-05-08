---
id: 721
title: "Implement Assigntoobjecttypewithprototypeproperty"
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

Triage assignToObjectTypeWithPrototypeProperty across 1 failing reference test cases and split this bucket into implementation-ready child issues.

## Problem

Reference test results show 1 cases fail in directory `assignToObjectTypeWithPrototypeProperty` with diagnostics: name-resolution. The compiler cannot handle these syntax/semantics, preventing compilation of code in this category.

Problem: assignToObjectTypeWithPrototypeProperty has 1 reference failures and needs smart-triage evidence before implementation starts.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/assignToObjectTypeWithPrototypeProperty.ts
```

Coverage window:

```sh
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/assignToObjectTypeWithPrototypeProperty.ts --detail
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
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/assignToObjectTypeWithPrototypeProperty.ts --detail
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/assignToObjectTypeWithPrototypeProperty.ts
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

- `reference/typescript/tests/cases/compiler/assignToObjectTypeWithPrototypeProperty.ts`

## Duplicate detection

- none found by path/title/feature scan

## Smart triage

### Smart triage: Triage name resolution: assignToObjectTypeWithPrototypeProperty

- Issue class: `triage-needed`
- Feature label: `name-resolution`
- Diagnostic: `UnresolvedName` / `resolver-symbol`
- Path: `reference/typescript/tests/cases/compiler/assignToObjectTypeWithPrototypeProperty.ts`

Reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/assignToObjectTypeWithPrototypeProperty.ts
```

Source overview:

```json
{
  "suite": "tsc",
  "bytes": 105,
  "lines": 4,
  "extension": ".ts",
  "first_code_line": "class XEvent {}"
}
```

Failure location:

```json
{
  "code": "UnresolvedName",
  "message": "unresolved name: `XEvent`",
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
class XEvent {}
var p: XEvent = XEvent.prototype;
var x: {prototype: XEvent} = XEvent;
```

Visible symbols before failure:

```json
[
  {
    "kind": "class",
    "name": "XEvent",
    "line": 2,
    "column": 1
  },
  {
    "kind": "binding",
    "name": "p",
    "line": 3,
    "column": 1
  },
  {
    "kind": "binding",
    "name": "x",
    "line": 4,
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
            start: 20,
            end: 25,
        },
    },
    SpannedToken {
        kind: Ident(
            "XEvent",
        ),
        span: Span {
            start: 26,
            end: 32,
        },
    },
    SpannedToken {
        kind: LeftBrace,
        span: Span {
            start: 33,
            end: 34,
        },
    },
    SpannedToken {
        kind: RightBrace,
        span: Span {
            start: 34,
            end: 35,
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
            "p",
        ),
        span: Span {
            start: 41,
            end: 42,
        },
    },
    SpannedToken {
        kind: Colon,
        span: Span {
            start: 42,
            end: 43,
        },
    },
    SpannedToken {
        kind: Ident(
            "XEvent",
        ),
        span: Span {
            start: 44,
            end: 50,
        },
    },
    SpannedToken {
        kind: Equal,
        span: Span {
            start: 51,
            end: 52,
        },
    },
    SpannedToken {
        kind: Ident(
            "XEvent",
        ),
        span: Span {
            start: 53,
            end: 59,
        },
    },
    SpannedToken {
        kind: Dot,
        span: Span {
            start: 59,
            end: 60,
        },
    },
    SpannedToken {
        kind: Ident(
            "prototype",
        ),
        span: Span {
            start: 60,
            end: 69,
        },
    },
    SpannedToken {
        kind: Semicolon,
        span: Span {
            start: 69,
            end: 70,
        },
    },
    SpannedToken {
        kind: Var,
        span: Span {
            start: 72,
            end: 75,
        },
    },
    SpannedToken {
        kind: Ident(
            "x",
        ),
        span: Span {
            start: 76,
            end: 77,
        },
    },
    SpannedToken {
        kind: Colon,
        span: Span {
            start: 77,
            end: 78,
        },
    },
    SpannedToken {
        kind: LeftBrace,
        span: Span {
            start: 79,
            end: 80,
        },
    },
    SpannedToken {
        kind: Ident(
            "prototype",
        ),
        span: Span {
            start: 80,
            end: 89,
        },
    },
    SpannedToken {
        kind: Colon,
        span: Span {
            start: 89,
            end: 90,
        },
    },
    SpannedToken {
        kind: Ident(
            "XEvent",
        ),
        span: Span {
            start: 91,
            end: 97,
        },
    },
    SpannedToken {
        kind: RightBrace,
        span: Span {
            start: 97,
            end: 98,
        },
    },
    SpannedToken {
        kind: Equal,
        span: Span {
            start: 99,
            end: 100,
        },
    },
    SpannedToken {
        kind: Ident(
            "XEvent",
        ),
        span: Span {
            start: 101,
            end: 107,
        },
    },
    SpannedToken {
        kind: Semicolon,
        span: Span {
            start: 107,
            end: 108,
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
        name: "XEvent",
        extends: None,
        body: [],
        static_blocks: [],
        private_elements: [],
        span: Span {
            start: 20,
            end: 35,
        },
    },
    Let {
        name: "p",
        expr: Member {
            object: Ident {
                name: "XEvent",
                span: Span {
                    start: 53,
                    end: 59,
                },
            },
            property: "prototype",
            span: Span {
                start: 53,
                end: 69,
            },
        },
        span: Span {
            start: 37,
            end: 70,
        },
    },
    Let {
        name: "x",
        expr: Ident {
            name: "XEvent",
            span: Span {
                start: 101,
                end: 107,
            },
        },
        span: Span {
            start: 72,
            end: 108,
        },
    },
]
```

#### resolved

- ok: `False`
- truncated: `False`

```text
error: [UnresolvedName] unresolved name: `XEvent`
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
        "kind": "binding",
        "typeText": "XEvent",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/assignToObjectTypeWithPrototypeProperty.ts",
        "start": 41,
        "length": 1,
        "line": 3,
        "character": 5,
        "name": "p"
      },
      {
        "kind": "binding",
        "typeText": "{ prototype: XEvent; }",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/assignToObjectTypeWithPrototypeProperty.ts",
        "start": 76,
        "length": 1,
        "line": 4,
        "character": 5,
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
