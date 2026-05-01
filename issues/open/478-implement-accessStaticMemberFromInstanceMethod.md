---
id: 478
title: "Implement Accessstaticmemberfrominstancemethod"
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

Triage accessStaticMemberFromInstanceMethod across 1 failing reference test cases and split this bucket into implementation-ready child issues.

## Problem

Reference test results show 1 cases fail in directory `accessStaticMemberFromInstanceMethod` with diagnostics: name-resolution. The compiler cannot handle these syntax/semantics, preventing compilation of code in this category.

Problem: accessStaticMemberFromInstanceMethod has 1 reference failures and needs smart-triage evidence before implementation starts.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/accessStaticMemberFromInstanceMethod01.ts
```

Coverage window:

```sh
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/accessStaticMemberFromInstanceMethod01.ts --detail
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
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/accessStaticMemberFromInstanceMethod01.ts --detail
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/accessStaticMemberFromInstanceMethod01.ts
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

- `reference/typescript/tests/cases/compiler/accessStaticMemberFromInstanceMethod01.ts`

## Duplicate detection

- `issues/open/093-implement-accessStaticMemberFromInstanceMethod.md` - Implement Accessstaticmemberfrominstancemethod (same reference path, same group key, title overlap)

## Smart triage

### Smart triage: Triage name resolution: accessStaticMemberFromInstanceMethod01

- Issue class: `triage-needed`
- Feature label: `name-resolution`
- Diagnostic: `UnresolvedName` / `resolver-symbol`
- Path: `reference/typescript/tests/cases/compiler/accessStaticMemberFromInstanceMethod01.ts`

Reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/accessStaticMemberFromInstanceMethod01.ts
```

Source overview:

```json
{
  "suite": "tsc",
  "bytes": 94,
  "lines": 8,
  "extension": ".ts",
  "first_code_line": "class C {"
}
```

Failure location:

```json
{
  "code": "UnresolvedName",
  "message": "unresolved name: `foo`",
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
class C {
    foo: string;

    static bar() {
        let k = foo;
    }
}
```

Visible symbols before failure:

```json
[
  {
    "kind": "class",
    "name": "C",
    "line": 2,
    "column": 1
  },
  {
    "kind": "binding",
    "name": "k",
    "line": 6,
    "column": 9,
    "initializer": "foo"
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
    "path": "issues/open/093-implement-accessStaticMemberFromInstanceMethod.md",
    "title": "Implement Accessstaticmemberfrominstancemethod",
    "reason": "same reference path"
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
            "C",
        ),
        span: Span {
            start: 26,
            end: 27,
        },
    },
    SpannedToken {
        kind: LeftBrace,
        span: Span {
            start: 28,
            end: 29,
        },
    },
    SpannedToken {
        kind: Ident(
            "foo",
        ),
        span: Span {
            start: 35,
            end: 38,
        },
    },
    SpannedToken {
        kind: Colon,
        span: Span {
            start: 38,
            end: 39,
        },
    },
    SpannedToken {
        kind: Ident(
            "string",
        ),
        span: Span {
            start: 40,
            end: 46,
        },
    },
    SpannedToken {
        kind: Semicolon,
        span: Span {
            start: 46,
            end: 47,
        },
    },
    SpannedToken {
        kind: Static,
        span: Span {
            start: 55,
            end: 61,
        },
    },
    SpannedToken {
        kind: Ident(
            "bar",
        ),
        span: Span {
            start: 62,
            end: 65,
        },
    },
    SpannedToken {
        kind: LeftParen,
        span: Span {
            start: 65,
            end: 66,
        },
    },
    SpannedToken {
        kind: RightParen,
        span: Span {
            start: 66,
            end: 67,
        },
    },
    SpannedToken {
        kind: LeftBrace,
        span: Span {
            start: 68,
            end: 69,
        },
    },
    SpannedToken {
        kind: Let,
        span: Span {
            start: 79,
            end: 82,
        },
    },
    SpannedToken {
        kind: Ident(
            "k",
        ),
        span: Span {
            start: 83,
            end: 84,
        },
    },
    SpannedToken {
        kind: Equal,
        span: Span {
            start: 85,
            end: 86,
        },
    },
    SpannedToken {
        kind: Ident(
            "foo",
        ),
        span: Span {
            start: 87,
            end: 90,
        },
    },
    SpannedToken {
        kind: Semicolon,
        span: Span {
            start: 90,
            end: 91,
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
        kind: RightBrace,
        span: Span {
            start: 100,
            end: 101,
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
        name: "C",
        extends: None,
        body: [
            Function {
                name: "static::bar",
                params: [],
                body: [
                    Let {
                        name: "k",
                        expr: Ident {
                            name: "foo",
                            span: Span {
                                start: 87,
                                end: 90,
                            },
                        },
                        span: Span {
                            start: 79,
                            end: 91,
                        },
                    },
                ],
                is_generator: false,
                span: Span {
                    start: 62,
                    end: 91,
                },
            },
        ],
        static_blocks: [],
        private_elements: [],
        span: Span {
            start: 20,
            end: 101,
        },
    },
]
```

#### resolved

- ok: `False`
- truncated: `False`

```text
error: [UnresolvedName] unresolved name: `foo`
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
        "message": "Property 'foo' has no initializer and is not definitely assigned in the constructor.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/accessStaticMemberFromInstanceMethod01.ts",
        "start": 35,
        "length": 3,
        "line": 3,
        "character": 5
      },
      {
        "code": 2304,
        "category": "Error",
        "message": "Cannot find name 'foo'.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/accessStaticMemberFromInstanceMethod01.ts",
        "start": 87,
        "length": 3,
        "line": 6,
        "character": 17
      }
    ],
    "hints": [
      {
        "kind": "binding",
        "typeText": "any",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/accessStaticMemberFromInstanceMethod01.ts",
        "start": 83,
        "length": 1,
        "line": 6,
        "character": 13,
        "name": "k"
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
