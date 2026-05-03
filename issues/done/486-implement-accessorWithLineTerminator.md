---
id: 486
title: "Implement Accessorwithlineterminator (dup)"
type: spike
area: reference/triage
class: superseded
priority: P2
depends_on: []
blocks: []
created: 2026-05-01
updated: 2026-05-04
---

## Summary

Triage accessorWithLineTerminator across 1 failing reference test cases and split this bucket into implementation-ready child issues.

## Problem

Reference test results show 1 cases fail in directory `accessorWithLineTerminator` with diagnostics: duplicate-function. The compiler cannot handle these syntax/semantics, preventing compilation of code in this category.

Problem: accessorWithLineTerminator has 1 reference failures and needs smart-triage evidence before implementation starts.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/accessorWithLineTerminator.ts
```

Coverage window:

```sh
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/accessorWithLineTerminator.ts --detail
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

- `issues/open/`
- `scripts/run/reference-triage.py`
- `fixtures/`

Do not touch:

- implementation code until the triage report assigns a concrete frontend/runtime/backend owner

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
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/accessorWithLineTerminator.ts --detail
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/accessorWithLineTerminator.ts
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

- `reference/typescript/tests/cases/compiler/accessorWithLineTerminator.ts`

## Duplicate detection

- `issues/open/103-implement-accessorWithLineTerminator.md` - Implement Accessorwithlineterminator (same reference path, same group key, title overlap)

## Smart triage

### Smart triage: Triage duplicate function: accessorWithLineTerminator

- Issue class: `triage-needed`
- Feature label: `duplicate-function`
- Diagnostic: `DuplicateFunction` / `compiler-diagnostic`
- Path: `reference/typescript/tests/cases/compiler/accessorWithLineTerminator.ts`

Reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/accessorWithLineTerminator.ts
```

Source overview:

```json
{
  "suite": "tsc",
  "bytes": 88,
  "lines": 9,
  "extension": ".ts",
  "first_code_line": "class C {"
}
```

Failure location:

```json
{
  "code": "DuplicateFunction",
  "message": "duplicate method definition: `C.x`",
  "span_start": null,
  "span_end": null,
  "line": null,
  "column": null,
  "feature_label": "duplicate-function",
  "error_type": "compiler-diagnostic"
}
```

Source context:

```text
// @target: es5, es2015

class C {
    get
    x() { return 1 }

    set
    x(v) {  }
```

Visible symbols before failure:

```json
[
  {
    "kind": "class",
    "name": "C",
    "line": 3,
    "column": 1
  }
]
```

Duplicate candidates:

```json
[
  {
    "state": "open",
    "path": "issues/open/103-implement-accessorWithLineTerminator.md",
    "title": "Implement Accessorwithlineterminator",
    "reason": "same reference path, title overlap"
  }
]
```

Error-specific suggestions:

- Create a child issue around this exact path and diagnostic before broadening the reference window.

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
            start: 27,
            end: 32,
        },
    },
    SpannedToken {
        kind: Ident(
            "C",
        ),
        span: Span {
            start: 33,
            end: 34,
        },
    },
    SpannedToken {
        kind: LeftBrace,
        span: Span {
            start: 35,
            end: 36,
        },
    },
    SpannedToken {
        kind: Ident(
            "get",
        ),
        span: Span {
            start: 42,
            end: 45,
        },
    },
    SpannedToken {
        kind: Ident(
            "x",
        ),
        span: Span {
            start: 51,
            end: 52,
        },
    },
    SpannedToken {
        kind: LeftParen,
        span: Span {
            start: 52,
            end: 53,
        },
    },
    SpannedToken {
        kind: RightParen,
        span: Span {
            start: 53,
            end: 54,
        },
    },
    SpannedToken {
        kind: LeftBrace,
        span: Span {
            start: 55,
            end: 56,
        },
    },
    SpannedToken {
        kind: Return,
        span: Span {
            start: 57,
            end: 63,
        },
    },
    SpannedToken {
        kind: Number(
            1,
        ),
        span: Span {
            start: 64,
            end: 65,
        },
    },
    SpannedToken {
        kind: RightBrace,
        span: Span {
            start: 66,
            end: 67,
        },
    },
    SpannedToken {
        kind: Ident(
            "set",
        ),
        span: Span {
            start: 75,
            end: 78,
        },
    },
    SpannedToken {
        kind: Ident(
            "x",
        ),
        span: Span {
            start: 84,
            end: 85,
        },
    },
    SpannedToken {
        kind: LeftParen,
        span: Span {
            start: 85,
            end: 86,
        },
    },
    SpannedToken {
        kind: Ident(
            "v",
        ),
        span: Span {
            start: 86,
            end: 87,
        },
    },
    SpannedToken {
        kind: RightParen,
        span: Span {
            start: 87,
            end: 88,
        },
    },
    SpannedToken {
        kind: LeftBrace,
        span: Span {
            start: 89,
            end: 90,
        },
    },
    SpannedToken {
        kind: RightBrace,
        span: Span {
            start: 92,
            end: 93,
        },
    },
    SpannedToken {
        kind: RightBrace,
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
        name: "C",
        extends: None,
        body: [
            Function {
                name: "x",
                params: [],
                body: [
                    Return {
                        expr: Number {
                            value: 1,
                            span: Span {
                                start: 64,
                                end: 65,
                            },
                        },
                        span: Span {
                            start: 57,
                            end: 65,
                        },
                    },
                ],
                is_generator: false,
                span: Span {
                    start: 51,
                    end: 65,
                },
            },
            Function {
                name: "x",
                params: [
                    (
                        "v",
                        None,
                        false,
                    ),
                ],
                body: [],
                is_generator: false,
                span: Span {
                    start: 84,
                    end: 85,
                },
            },
        ],
        static_blocks: [],
        private_elements: [],
        span: Span {
            start: 27,
            end: 96,
        },
    },
]
```

#### resolved

- ok: `False`
- truncated: `False`

```text
error: [DuplicateFunction] duplicate method definition: `C.x`
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
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/accessorWithLineTerminator.ts",
        "start": 86,
        "length": 1,
        "line": 8,
        "character": 7,
        "name": "v"
      }
    ],
    "typescriptVersion": "6.0.3"
  }
}
```


## 重複整理メモ

この issue はタイトルが重複しているため、内容がより充実している
`issues/open/572-implement-accessorWithLineTerminator.md` に統合されました。
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

## Status

Superseded by issue #103. Duplicate from separate coverage run.

---

## ⚠️ False-done audit (re-opened from issues/done/)

**Why this was false-done**: This issue has `class: triage-needed` in `issues/done/`.
The "Status" note claims supersedence by issue #103, but issue #103 was itself
identified as false-done and moved back to `issues/open/`. The supersedence chain
is therefore invalid. No implementation commits, no close note, no completion
evidence.

**True-done checklist** (all must pass):

1. Perform actual triage review
2. Either create child implementation issue(s) or confirm this issue is legitimately
   superseded by a truly resolved issue
3. Update `class` from `triage-needed` to appropriate value
4. Fill in completion evidence section with triage results

**Commands that must pass**:

```sh
cargo fmt --all --check
cargo nextest run
```
