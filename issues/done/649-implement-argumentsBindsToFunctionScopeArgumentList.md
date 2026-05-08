---
id: 649
title: "Implement Argumentsbindstofunctionscopeargumentlist"
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

Triage argumentsBindsToFunctionScopeArgumentList across 1 failing reference test cases and split this bucket into implementation-ready child issues.

## Problem

Reference test results show 1 cases fail in directory `argumentsBindsToFunctionScopeArgumentList` with diagnostics: name-resolution. The compiler cannot handle these syntax/semantics, preventing compilation of code in this category.

Problem: argumentsBindsToFunctionScopeArgumentList has 1 reference failures and needs smart-triage evidence before implementation starts.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/argumentsBindsToFunctionScopeArgumentList.ts
```

Coverage window:

```sh
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/argumentsBindsToFunctionScopeArgumentList.ts --detail
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
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/argumentsBindsToFunctionScopeArgumentList.ts --detail
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/argumentsBindsToFunctionScopeArgumentList.ts
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

- `reference/typescript/tests/cases/compiler/argumentsBindsToFunctionScopeArgumentList.ts`

## Duplicate detection

- `issues/done/193-implement-arguments.md` - Implement Arguments (same feature label, same group key, title overlap)
- `issues/done/195-implement-argumentsBindsToFunctionScopeArgumentList.md` - Implement Argumentsbindstofunctionscopeargumentlist (same reference path, same feature label, same group key, title overlap)
- `issues/done/437-implement-name-resolution.md` - Implement name resolution (same feature label, same group key, title overlap)

## Smart triage

### Smart triage: Triage name resolution: argumentsBindsToFunctionScopeArgumentList

- Issue class: `triage-needed`
- Feature label: `name-resolution`
- Diagnostic: `UnresolvedName` / `resolver-symbol`
- Path: `reference/typescript/tests/cases/compiler/argumentsBindsToFunctionScopeArgumentList.ts`

Reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/argumentsBindsToFunctionScopeArgumentList.ts
```

Source overview:

```json
{
  "suite": "tsc",
  "bytes": 212,
  "lines": 8,
  "extension": ".ts",
  "first_code_line": "var arguments = 10;"
}
```

Failure location:

```json
{
  "code": "UnresolvedName",
  "message": "unresolved name: `arguments`",
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
// @ignoreDeprecations: 6.0
// @strict: false
// @alwaysStrict: true, false
var arguments = 10;
function foo(a) {
    arguments = 10;  /// This shouldnt be of type number and result in error.
}
```

Visible symbols before failure:

```json
[
  {
    "kind": "binding",
    "name": "arguments",
    "line": 5,
    "column": 1,
    "initializer": "10"
  },
  {
    "kind": "function",
    "name": "foo",
    "line": 6,
    "column": 1,
    "params": "a"
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
    "path": "issues/done/195-implement-argumentsBindsToFunctionScopeArgumentList.md",
    "title": "Implement Argumentsbindstofunctionscopeargumentlist",
    "reason": "same reference path, same feature label, title overlap"
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
        kind: Var,
        span: Span {
            start: 99,
            end: 102,
        },
    },
    SpannedToken {
        kind: Ident(
            "arguments",
        ),
        span: Span {
            start: 103,
            end: 112,
        },
    },
    SpannedToken {
        kind: Equal,
        span: Span {
            start: 113,
            end: 114,
        },
    },
    SpannedToken {
        kind: Number(
            10,
        ),
        span: Span {
            start: 115,
            end: 117,
        },
    },
    SpannedToken {
        kind: Semicolon,
        span: Span {
            start: 117,
            end: 118,
        },
    },
    SpannedToken {
        kind: Function,
        span: Span {
            start: 120,
            end: 128,
        },
    },
    SpannedToken {
        kind: Ident(
            "foo",
        ),
        span: Span {
            start: 129,
            end: 132,
        },
    },
    SpannedToken {
        kind: LeftParen,
        span: Span {
            start: 132,
            end: 133,
        },
    },
    SpannedToken {
        kind: Ident(
            "a",
        ),
        span: Span {
            start: 133,
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
        kind: LeftBrace,
        span: Span {
            start: 136,
            end: 137,
        },
    },
    SpannedToken {
        kind: Ident(
            "arguments",
        ),
        span: Span {
            start: 143,
            end: 152,
        },
    },
    SpannedToken {
        kind: Equal,
        span: Span {
            start: 153,
            end: 154,
        },
    },
    SpannedToken {
        kind: Number(
            10,
        ),
        span: Span {
            start: 155,
            end: 157,
        },
    },
    SpannedToken {
        kind: Semicolon,
        span: Span {
            start: 157,
            end: 158,
        },
    },
    SpannedToken {
        kind: RightBrace,
        span: Span {
            start: 218,
            end: 219,
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
    Let {
        name: "arguments",
        expr: Number {
            value: 10,
            span: Span {
                start: 115,
                end: 117,
            },
        },
        span: Span {
            start: 99,
            end: 118,
        },
    },
    Function {
        name: "foo",
        params: [
            (
                "a",
                None,
                false,
            ),
        ],
        body: [
            Assign {
                name: "arguments",
                expr: Number {
                    value: 10,
                    span: Span {
                        start: 155,
                        end: 157,
                    },
                },
                span: Span {
                    start: 143,
                    end: 158,
                },
            },
        ],
        is_generator: false,
        span: Span {
            start: 120,
            end: 158,
        },
    },
]
```

#### resolved

- ok: `False`
- truncated: `False`

```text
error: [UnresolvedName] unresolved name: `arguments`
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
        "code": 1100,
        "category": "Error",
        "message": "Invalid use of 'arguments' in strict mode.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/argumentsBindsToFunctionScopeArgumentList.ts",
        "start": 103,
        "length": 9,
        "line": 5,
        "character": 5
      },
      {
        "code": 1100,
        "category": "Error",
        "message": "Invalid use of 'arguments' in strict mode.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/argumentsBindsToFunctionScopeArgumentList.ts",
        "start": 143,
        "length": 9,
        "line": 7,
        "character": 5
      },
      {
        "code": 2322,
        "category": "Error",
        "message": "Type 'number' is not assignable to type 'IArguments'.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/argumentsBindsToFunctionScopeArgumentList.ts",
        "start": 143,
        "length": 9,
        "line": 7,
        "character": 5
      }
    ],
    "hints": [
      {
        "kind": "binding",
        "typeText": "number",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/argumentsBindsToFunctionScopeArgumentList.ts",
        "start": 103,
        "length": 9,
        "line": 5,
        "character": 5,
        "name": "arguments"
      },
      {
        "kind": "function",
        "typeText": "void",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/argumentsBindsToFunctionScopeArgumentList.ts",
        "start": 129,
        "length": 3,
        "line": 6,
        "character": 10,
        "name": "foo"
      },
      {
        "kind": "parameter",
        "typeText": "any",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/argumentsBindsToFunctionScopeArgumentList.ts",
        "start": 133,
        "length": 1,
        "line": 6,
        "character": 14,
        "name": "a"
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
