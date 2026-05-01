---
id: 597
title: "Implement Allowjsclassthistypecrash"
type: spike
area: reference/triage
class: triage-needed
priority: P2
depends_on: []
blocks: []
created: 2026-05-01
updated: 2026-05-01
---

## Summary

Triage allowJsClassThisTypeCrash across 1 failing reference test cases and split this bucket into implementation-ready child issues.

## Problem

Reference test results show 1 cases fail in directory `allowJsClassThisTypeCrash` with diagnostics: runtime-subset. The compiler cannot handle these syntax/semantics, preventing compilation of code in this category.

Problem: allowJsClassThisTypeCrash has 1 reference failures and needs smart-triage evidence before implementation starts.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/allowJsClassThisTypeCrash.ts
```

Coverage window:

```sh
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/allowJsClassThisTypeCrash.ts --detail
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

- `issues/open/`
- `scripts/run/reference-triage.py`
- `fixtures/`

Do not touch:

- implementation code until the triage report assigns a concrete frontend/runtime/backend owner

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
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/allowJsClassThisTypeCrash.ts --detail
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/allowJsClassThisTypeCrash.ts
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

- `reference/typescript/tests/cases/compiler/allowJsClassThisTypeCrash.ts`

## Duplicate detection

- `issues/open/132-implement-allowJsClassThisTypeCrash.md` - Implement Allowjsclassthistypecrash (same reference path, same group key, title overlap)
- `issues/open/511-implement-allowJsClassThisTypeCrash.md` - Implement Allowjsclassthistypecrash (same reference path, same feature label, same group key, title overlap)

## Smart triage

### Smart triage: Triage runtime subset: allowJsClassThisTypeCrash

- Issue class: `triage-needed`
- Feature label: `runtime-subset`
- Diagnostic: `UnsupportedRuntimeSubset` / `unsupported-feature-boundary`
- Path: `reference/typescript/tests/cases/compiler/allowJsClassThisTypeCrash.ts`

Reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/allowJsClassThisTypeCrash.ts
```

Source overview:

```json
{
  "suite": "tsc",
  "bytes": 170,
  "lines": 11,
  "extension": ".ts",
  "first_code_line": "const f = function() {};"
}
```

Failure location:

```json
{
  "code": "UnsupportedRuntimeSubset",
  "message": "issue-062e: nested function `` closures with `this` or `arguments` are not supported in this slice",
  "span_start": null,
  "span_end": null,
  "line": null,
  "column": null,
  "feature_label": "runtime-subset",
  "error_type": "unsupported-feature-boundary"
}
```

Source context:

```text
// @target: es2015
// @checkJs: true
// @allowJs: true
// @noEmit: true

// @filename: app.js
const f = function() {};
var g = f;
```

Visible symbols before failure:

```json
[
  {
    "kind": "binding",
    "name": "f",
    "line": 7,
    "column": 1,
    "initializer": "function() {}"
  },
  {
    "kind": "binding",
    "name": "g",
    "line": 8,
    "column": 1,
    "initializer": "f"
  }
]
```

Duplicate candidates:

```json
[
  {
    "state": "open",
    "path": "issues/open/132-implement-allowJsClassThisTypeCrash.md",
    "title": "Implement Allowjsclassthistypecrash",
    "reason": "same reference path, title overlap"
  },
  {
    "state": "open",
    "path": "issues/open/511-implement-allowJsClassThisTypeCrash.md",
    "title": "Implement Allowjsclassthistypecrash",
    "reason": "same reference path, same feature label, title overlap"
  }
]
```

Error-specific suggestions:

- Create a child issue around this exact path and diagnostic before broadening the reference window.

Compiler dumps:

#### tokens

- ok: `True`
- truncated: `True`

```text
== tokens ==
[
    SpannedToken {
        kind: Const,
        span: Span {
            start: 100,
            end: 105,
        },
    },
    SpannedToken {
        kind: Ident(
            "f",
        ),
        span: Span {
            start: 106,
            end: 107,
        },
    },
    SpannedToken {
        kind: Equal,
        span: Span {
            start: 108,
            end: 109,
        },
    },
    SpannedToken {
        kind: Function,
        span: Span {
            start: 110,
            end: 118,
        },
    },
    SpannedToken {
        kind: LeftParen,
        span: Span {
            start: 118,
            end: 119,
        },
    },
    SpannedToken {
        kind: RightParen,
        span: Span {
            start: 119,
            end: 120,
        },
    },
    SpannedToken {
        kind: LeftBrace,
        span: Span {
            start: 121,
            end: 122,
        },
    },
    SpannedToken {
        kind: RightBrace,
        span: Span {
            start: 122,
            end: 123,
        },
    },
    SpannedToken {
        kind: Semicolon,
        span: Span {
            start: 123,
            end: 124,
        },
    },
    SpannedToken {
        kind: Var,
        span: Span {
            start: 126,
            end: 129,
        },
    },
    SpannedToken {
        kind: Ident(
            "g",
        ),
        span: Span {
            start: 130,
            end: 131,
        },
    },
    SpannedToken {
        kind: Equal,
        span: Span {
            start: 132,
            end: 133,
        },
    },
    SpannedToken {
        kind: Ident(
            "f",
        ),
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
        kind: Ident(
            "g",
        ),
        span: Span {
            start: 138,
            end: 139,
        },
    },
    SpannedToken {
        kind: Dot,
        span: Span {
            start: 139,
            end: 140,
        },
    },
    SpannedToken {
        kind: Ident(
            "prototype",
        ),
        span: Span {
            start: 140,
            end: 149,
        },
    },
    SpannedToken {
        kind: Dot,
        span: Span {
            start: 149,
            end: 150,
        },
    },
    SpannedToken {
        kind: Ident(
            "m",
        ),
        span: Span {
            start: 150,
            end: 151,
        },
    },
    SpannedToken {
        kind: Equal,
        span: Span {
            start: 152,
            end: 153,
        },
    },
    SpannedToken {
        kind: Function,
        span: Span {
            start: 154,
            end: 162,
        },
    },
    SpannedToken {
        kind: LeftParen,
        span: Span {
            start: 163,
            end: 164,
        },
    },
    SpannedToken {
        kind: RightParen,
        span: Span {
            start: 164,
            end: 165,
        },
    },
    SpannedToken {
        kind: LeftBrace,
        span: Span {
            start: 166,
            end: 167,
        },
    },
    SpannedToken {
        kind: This,
        span: Span {
            start: 171,
            end: 175,
        },
    },
    SpannedToken {
        kind: Semicolon,
        span: Span {
            start: 175,
            end: 176,
        },
    },
    SpannedToken
```

#### ast

- ok: `True`
- truncated: `False`

```text
== ast ==
[
    Let {
        name: "f",
        expr: FunctionExpr {
            name: "",
            params: [],
            body: [],
            span: Span {
                start: 110,
                end: 118,
            },
        },
        span: Span {
            start: 100,
            end: 124,
        },
    },
    Let {
        name: "g",
        expr: Ident {
            name: "f",
            span: Span {
                start: 134,
                end: 135,
            },
        },
        span: Span {
            start: 126,
            end: 136,
        },
    },
    Expr {
        expr: PropertyAssign {
            object: Member {
                object: Ident {
                    name: "g",
                    span: Span {
                        start: 138,
                        end: 139,
                    },
                },
                property: "prototype",
                span: Span {
                    start: 138,
                    end: 149,
                },
            },
            property: "m",
            value: FunctionExpr {
                name: "",
                params: [],
                body: [
                    Expr {
                        expr: This {
                            span: Span {
                                start: 171,
                                end: 175,
                            },
                        },
                        span: Span {
                            start: 171,
                            end: 176,
                        },
                    },
                ],
                span: Span {
                    start: 154,
                    end: 176,
                },
            },
            span: Span {
                start: 138,
                end: 180,
            },
        },
        span: Span {
            start: 138,
            end: 180,
        },
    },
]
```

#### resolved

- ok: `False`
- truncated: `False`

```text
error: [UnsupportedRuntimeSubset] issue-062e: nested function `` closures with `this` or `arguments` are not supported in this slice
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
        "typeText": "() => void",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/allowJsClassThisTypeCrash.ts",
        "start": 106,
        "length": 1,
        "line": 7,
        "character": 7,
        "name": "f"
      },
      {
        "kind": "binding",
        "typeText": "() => void",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/allowJsClassThisTypeCrash.ts",
        "start": 130,
        "length": 1,
        "line": 8,
        "character": 5,
        "name": "g"
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
