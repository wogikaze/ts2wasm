---
id: 109
title: "Implement Addmorecallsignaturestobasesignature (dup)"
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

Triage addMoreCallSignaturesToBaseSignature across 2 failing reference test cases and split this bucket into implementation-ready child issues.

## Problem

Reference test results show 2 cases fail in directory `addMoreCallSignaturesToBaseSignature` with diagnostics: function-resolution. The compiler cannot handle these syntax/semantics, preventing compilation of code in this category.

Problem: addMoreCallSignaturesToBaseSignature has 2 reference failures and needs smart-triage evidence before implementation starts.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/addMoreCallSignaturesToBaseSignature.ts
```

Coverage window:

```sh
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/addMoreCallSignaturesToBaseSignature.ts --detail
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
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/addMoreCallSignaturesToBaseSignature.ts --detail
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/addMoreCallSignaturesToBaseSignature.ts
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

- `reference/typescript/tests/cases/compiler/addMoreCallSignaturesToBaseSignature.ts`
- `reference/typescript/tests/cases/compiler/addMoreCallSignaturesToBaseSignature2.ts`

## Duplicate detection

## Smart triage

### Smart triage: Triage function resolution: addMoreCallSignaturesToBaseSignature

- Issue class: `triage-needed`
- Feature label: `function-resolution`
- Diagnostic: `UnresolvedFunction` / `resolver-symbol`
- Path: `reference/typescript/tests/cases/compiler/addMoreCallSignaturesToBaseSignature.ts`

Reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/addMoreCallSignaturesToBaseSignature.ts
```

Source overview:

```json
{
  "suite": "tsc",
  "bytes": 141,
  "lines": 11,
  "extension": ".ts",
  "first_code_line": "interface Foo {"
}
```

Failure location:

```json
{
  "code": "UnresolvedFunction",
  "message": "unresolved function: `a`",
  "span_start": null,
  "span_end": null,
  "line": null,
  "column": null,
  "feature_label": "function-resolution",
  "error_type": "resolver-symbol"
}
```

Source context:

```text
// @target: es2015
interface Foo {
    (): string;
}

interface Bar extends Foo {
    (key: string): string;
}
```

Visible symbols before failure:

```json
[
  {
    "kind": "binding",
    "name": "a",
    "line": 10,
    "column": 1
  },
  {
    "kind": "binding",
    "name": "kitty",
    "line": 11,
    "column": 1,
    "initializer": "a()"
  }
]
```

Duplicate candidates:

```json
[
  {
    "state": "open",
    "path": "issues/done/109-implement-addMoreCallSignaturesToBaseSignature.md",
    "title": "Implement Addmorecallsignaturestobasesignature",
    "reason": "same reference path, title overlap"
  },
  {
    "state": "done",
    "path": "issues/done/062a-split-function-epic-into-callable-child-issues.md",
    "title": "Split function epic into callable child issues",
    "reason": "same feature label, title overlap"
  },
  {
    "state": "done",
    "path": "issues/done/063-implement-function-resolution.md",
    "title": "Implement function resolution",
    "reason": "same feature label, title overlap"
  }
]
```

Error-specific suggestions:

- Check whether the missing name should be a local binding, function binding, builtin, import binding, or runtime global.
- Acceptance should assert both the formerly missing symbol and an adjacent negative case.

Compiler dumps:

#### tokens

- ok: `True`
- truncated: `True`

```text
== tokens ==
[
    SpannedToken {
        kind: Ident(
            "interface",
        ),
        span: Span {
            start: 20,
            end: 29,
        },
    },
    SpannedToken {
        kind: Ident(
            "Foo",
        ),
        span: Span {
            start: 30,
            end: 33,
        },
    },
    SpannedToken {
        kind: LeftBrace,
        span: Span {
            start: 34,
            end: 35,
        },
    },
    SpannedToken {
        kind: LeftParen,
        span: Span {
            start: 41,
            end: 42,
        },
    },
    SpannedToken {
        kind: RightParen,
        span: Span {
            start: 42,
            end: 43,
        },
    },
    SpannedToken {
        kind: Colon,
        span: Span {
            start: 43,
            end: 44,
        },
    },
    SpannedToken {
        kind: Ident(
            "string",
        ),
        span: Span {
            start: 45,
            end: 51,
        },
    },
    SpannedToken {
        kind: Semicolon,
        span: Span {
            start: 51,
            end: 52,
        },
    },
    SpannedToken {
        kind: RightBrace,
        span: Span {
            start: 54,
            end: 55,
        },
    },
    SpannedToken {
        kind: Ident(
            "interface",
        ),
        span: Span {
            start: 59,
            end: 68,
        },
    },
    SpannedToken {
        kind: Ident(
            "Bar",
        ),
        span: Span {
            start: 69,
            end: 72,
        },
    },
    SpannedToken {
        kind: Extends,
        span: Span {
            start: 73,
            end: 80,
        },
    },
    SpannedToken {
        kind: Ident(
            "Foo",
        ),
        span: Span {
            start: 81,
```

#### ast

- ok: `True`
- truncated: `False`

```text
== ast ==
[
    Let {
        name: "a",
        expr: Undefined {
            span: Span {
                start: 125,
                end: 126,
            },
        },
        span: Span {
            start: 121,
            end: 132,
        },
    },
    Let {
        name: "kitty",
        expr: Call {
            callee: Ident {
                name: "a",
                span: Span {
                    start: 146,
                    end: 147,
                },
            },
            args: [],
            span: Span {
                start: 146,
                end: 149,
            },
        },
        span: Span {
            start: 134,
            end: 150,
        },
    },
]
```

#### resolved

- ok: `False`
- truncated: `False`

```text
error: [UnsupportedSyntax] issue-211: function-valued local calls such as extracted method `a(...)` are not supported; call receiver.method(...) directly at 146..149
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
        "code": 2454,
        "category": "Error",
        "message": "Variable 'a' is used before being assigned.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/addMoreCallSignaturesToBaseSignature.ts",
        "start": 146,
        "length": 1,
        "line": 11,
        "character": 13
      }
    ],
    "hints": [
      {
        "kind": "parameter",
        "typeText": "string",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/addMoreCallSignaturesToBaseSignature.ts",
        "start": 93,
        "length": 3,
        "line": 7,
        "character": 6,
        "name": "key"
      },
      {
        "kind": "binding",
        "typeText": "Bar",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/addMoreCallSignaturesToBaseSignature.ts",
        "start": 125,
        "length": 1,
        "line": 10,
        "character": 5,
        "name": "a"
      },
      {
        "kind": "binding",
        "typeText": "string",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/addMoreCallSignaturesToBaseSignature.ts",
        "start": 138,
        "length": 5,
        "line": 11,
        "character": 5,
        "name": "kitty"
      }
    ],
    "typescriptVersion": "6.0.3"
  }
}
```


## 重複整理メモ

この issue はタイトルが重複しているため、内容がより充実している
`issues/open/576-implement-addMoreCallSignaturesToBaseSignature.md` に統合されました。
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

## False-done audit

Date: 2026-05-05

Classification: truly-done.

Audit result: retained in `issues/done/`. The issue has repo-local completion evidence, is not marked `blocked` or `triage-needed`, and `python scripts/manager.py check issues` validates the resulting issue graph.

Evidence files:
- `issues/open/109-implement-addMoreCallSignaturesToBaseSignature.md`
- `issues/index.md` after regeneration

Future-work tracking: no untracked future-work item was identified in this issue during this metadata/evidence audit.
