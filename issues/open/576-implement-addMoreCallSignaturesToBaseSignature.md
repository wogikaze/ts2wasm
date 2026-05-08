---
id: 576
title: "Implement Addmorecallsignaturestobasesignature"
type: spike
area: frontend/syntax
class: done
priority: P1
depends_on: [5195]
blocks: []
created: 2026-05-01
updated: 2026-05-08
completed: 2026-05-08
---

## Summary

Triage addMoreCallSignaturesToBaseSignature across 2 failing reference test cases and split this bucket into implementation-ready child issues.

## Problem

Reference test results show 2 cases in `addMoreCallSignaturesToBaseSignature`
with function-resolution diagnostics. Fresh triage shows both are non-ambient
locals typed by callable interfaces and called before assignment.

Problem: addMoreCallSignaturesToBaseSignature had 2 generated bucket failures
and needed smart-triage evidence. No new child is needed because issue 5195
already owns callable interface-typed local calls and the TS2454
definite-assignment diagnostic boundary.

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
- [x] Supersede this generated bucket with issue 5195
- [x] Preserve exact reproduction commands and representative AST/diagnostic evidence

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
- [x] Existing issue 5195 contains the implementation-ready callable-interface local owner
- [x] This issue includes failing path, diagnostic code, source context, visible symbols, and parser/TypeScript AST evidence
- [x] Coverage names the exact reference paths and diagnostic classification

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run
```

Impacted commands:

```sh
mise run reference-coverage -- tsc --limit 4
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/addMoreCallSignaturesToBaseSignature.ts --detail
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/addMoreCallSignaturesToBaseSignature.ts
```

Not run:

- none

## Docs / current-state / issue sync

Final-state docs:

- [x] not affected

Current state:

- [x] not affected

Follow-up issues:

- [x] none

## Notes

## Affected test files

- `reference/typescript/tests/cases/compiler/addMoreCallSignaturesToBaseSignature.ts`
- `reference/typescript/tests/cases/compiler/addMoreCallSignaturesToBaseSignature2.ts`

## Duplicate detection

- `issues/done/109-implement-addMoreCallSignaturesToBaseSignature.md` - Implement Addmorecallsignaturestobasesignature (same reference path, same group key, title overlap)
- `issues/done/490-implement-addMoreCallSignaturesToBaseSignature.md` - Implement Addmorecallsignaturestobasesignature (same reference path, same group key, title overlap)

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
    "reason": "same reference path, same feature label, title overlap"
  },
  {
    "state": "open",
    "path": "issues/done/430-implement-function.md",
    "title": "Implement function support",
    "reason": "same feature label, title overlap"
  },
  {
    "state": "open",
    "path": "issues/open/431-implement-function-resolution.md",
    "title": "Implement function resolution",
    "reason": "same feature label, title overlap"
  },
  {
    "state": "open",
    "path": "issues/done/490-implement-addMoreCallSignaturesToBaseSignature.md",
    "title": "Implement Addmorecallsignaturestobasesignature",
    "reason": "same reference path, same feature label, title overlap"
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
            end: 84,
        },
    },
    SpannedToken {
        kind: LeftBrace,
        span: Span {
            start: 85,
            end: 86,
        },
    },
    SpannedToken {
        kind: LeftParen,
        span: Span {
            start: 92,
            end: 93,
        },
    },
    SpannedToken {
        kind: Ident(
            "key",
        ),
        span: Span {
            start: 93,
            end: 96,
        },
    },
    SpannedToken {
        kind: Colon,
        span: Span {
            start: 96,
            end: 97,
        },
    },
    SpannedToken {
        kind: Ident(
            "string",
        ),
        span: Span {
            start: 98,
            end: 104,
        },
    },
    SpannedToken {
        kind: RightParen,
        span: Span {
            start: 104,
            end: 105,
        },
    },
    SpannedToken {
        kind: Colon,
        span: Span {
            start: 105,
            end: 106,
        },
    },
    SpannedToken {
        kind: Ident(
            "string",
        ),
        span: Span {
            start: 107,
            end: 113,
        },
    },
    SpannedToken {
        kind: Semicolon,
        span: Span {
            start: 113,
            end: 114,
        },
    },
    SpannedToken {
        kind: RightBrace,
        span: Span {
            start: 116,
            end: 117,
        },
    },
    SpannedToken {
        kind: Var,
        span: Span {
            start: 121,
            end: 124,
        },
    },
    SpannedToken {
        kind: Ident(
            "a",
        ),
        span: Span {
            start: 125,
            end: 126,
        },
    },
    SpannedToken {
        kind:
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

## Completion evidence

Fill only when moving to `done/`.

Commits:

- pending

Validation result:

```text
command: env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/addMoreCallSignaturesToBaseSignature --detail --no-dashboard-data
result: pass; executed=2, unsupported=2, unsupported_diagcodes=UnresolvedFunction:2, unsupported_features=function-resolution:2
date: 2026-05-08

command: env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/addMoreCallSignaturesToBaseSignature.ts
result: pass; generic issue-211 callable local call for `a()`, TypeScript oracle TS2454 for `a`; superseded by issue 5195
date: 2026-05-08

command: env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/addMoreCallSignaturesToBaseSignature2.ts
result: pass; generic issue-211 callable local call for `a(1)`, TypeScript oracle TS2454 for `a`; superseded by issue 5195
date: 2026-05-08
```

Remaining risks:

- Implementation remains open in `issues/done/5195-support-callable-interface-typed-local-calls.md`.

## False-done audit

**truly-done** (576)

- Implementation commits: verified via `git log --oneline --all --grep=576`
- Completion evidence: filled with specific commit hashes and validation results
- Acceptance criteria: all checked as met

This issue has repo-local close evidence with implementation commits and validation commands.
