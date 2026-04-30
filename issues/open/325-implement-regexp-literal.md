---
id: 325
title: "Implement RegExp literal support"
type: spike
area: frontend/syntax
class: triage-needed
priority: P1
depends_on: []
blocks: []
created: 2026-04-30
updated: 2026-04-30
---

## Summary

Triage regexp-literal feature across 70 failing reference test cases and split this bucket into implementation-ready child issues.

## Problem

Reference test results show 70 cases fail with regexp-literal diagnostic. The compiler cannot handle these syntax/semantics, preventing compilation of code in this category.

Problem: regexp-literal feature has 70 reference failures and needs smart-triage evidence before implementation starts.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- test262 reference/test262/test/annexB/built-ins/RegExp/RegExp-control-escape-russian-letter.js
```

Coverage window:

```sh
mise run reference-coverage -- test262 --path-filter reference/test262/test/annexB/built-ins/RegExp/RegExp-control-escape-russian-letter.js --detail
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
mise run reference-coverage -- test262 --limit 140
mise run reference-coverage -- test262 --path-filter reference/test262/test/annexB/built-ins/RegExp/RegExp-control-escape-russian-letter.js --detail
mise run reference-triage -- test262 reference/test262/test/annexB/built-ins/RegExp/RegExp-control-escape-russian-letter.js
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

- `reference/test262/test/annexB/built-ins/RegExp/RegExp-control-escape-russian-letter.js`
- `reference/test262/test/annexB/built-ins/RegExp/RegExp-decimal-escape-class-range.js`
- `reference/test262/test/annexB/built-ins/RegExp/RegExp-decimal-escape-not-capturing.js`
- `reference/test262/test/annexB/built-ins/RegExp/RegExp-invalid-control-escape-character-class-range.js`
- `reference/test262/test/annexB/built-ins/RegExp/RegExp-invalid-control-escape-character-class.js`
- `reference/test262/test/annexB/built-ins/RegExp/RegExp-leading-escape-BMP.js`
- `reference/test262/test/annexB/built-ins/RegExp/RegExp-leading-escape.js`
- `reference/test262/test/annexB/built-ins/RegExp/RegExp-trailing-escape-BMP.js`
- `reference/test262/test/annexB/built-ins/RegExp/RegExp-trailing-escape.js`
- `reference/test262/test/annexB/built-ins/RegExp/incomplete_hex_unicode_escape.js`
- ... and 60 more files

## Duplicate detection

- `issues/open/066-implement-regexp-literal.md` - Implement RegExp literal support (same reference path, same feature label, same group key, title overlap)
- `issues/done/005-add-fine-grained-unsupported-feature-breakdown.md` - issues/done/005-add-fine-grained-unsupported-feature-breakdown.md (same feature label, same group key)
- `issues/done/009-select-first-coverage-improvement-feature-slice.md` - issues/done/009-select-first-coverage-improvement-feature-slice.md (same feature label, same group key)
- `issues/done/022-expand-test262-differential-coverage.md` - issues/done/022-expand-test262-differential-coverage.md (same feature label, same group key)
- `issues/done/051-implement-regexp.md` - Implement RegExp (same feature label, same group key, title overlap)
- `issues/done/060-investigate-unknown-unsupported-cases.md` - Investigate and classify unknown-unsupported diagnostic cases (same feature label, same group key)
- `issues/done/202-implement-regexp-literal-support.md` - issues/done/202-implement-regexp-literal-support.md (same feature label, same group key)
- `issues/done/224-implement-annexb-html-comments.md` - Implement Annex B HTML-like comments (same feature label, same group key, title overlap)
- `issues/done/228-implement-logical-assignment-operators.md` - Implement logical assignment operators (same feature label, same group key, title overlap)

## Smart triage

### Smart triage: Triage regexp literal: RegExp control escape russian letter

- Issue class: `triage-needed`
- Feature label: `regexp-literal`
- Diagnostic: `UnsupportedSyntax` / `parser-or-frontend-unsupported`
- Path: `reference/test262/test/annexB/built-ins/RegExp/RegExp-control-escape-russian-letter.js`

Reproduction:

```sh
mise run reference-triage -- test262 reference/test262/test/annexB/built-ins/RegExp/RegExp-control-escape-russian-letter.js
```

Source overview:

```json
{
  "suite": "test262",
  "bytes": 1597,
  "lines": 53,
  "extension": ".js",
  "first_code_line": "info: \"CharacterEscape :: c ControlLetter\"",
  "test262_metadata": {
    "info": "\"CharacterEscape :: c ControlLetter\"",
    "es5id": "15.10.2.10_A2.1_T3",
    "es6id": "B.1.4",
    "description": ">",
    "\"ControlLetter": ": RUSSIAN ALPHABET is incorrect\"",
    "features": "[generators]"
  }
}
```

Failure location:

```json
{
  "code": "UnsupportedSyntax",
  "message": "expected identifier, got Some(SpannedToken { kind: Star, span: Span { start: 382, end: 383 } }) at 384..399",
  "span_start": 384,
  "span_end": 399,
  "line": 14,
  "column": 11,
  "feature_label": "regexp-literal",
  "error_type": "parser-or-frontend-unsupported"
}
```

Source context:

```text
11 | features: [generators]
12 | ---*/
13 | 
14 | function* invalidControls() {
15 |   // Check upper case Cyrillic
16 |   for (var alpha = 0x0410; alpha <= 0x042F; alpha++) {
17 |     yield String.fromCharCode(alpha);
```

Visible symbols before failure:

```json
[]
```

Duplicate candidates:

```json
[
  {
    "state": "open",
    "path": "issues/open/066-implement-regexp-literal.md",
    "title": "Implement RegExp literal support",
    "reason": "same reference path, same feature label, title overlap"
  },
  {
    "state": "done",
    "path": "issues/done/051-implement-regexp.md",
    "title": "Implement RegExp",
    "reason": "same feature label, title overlap"
  }
]
```

Error-specific suggestions:

- Start at lexer/parser support and add a minimal fixture for the exact source construct at the failing span.
- Use `dump --tokens` and the TypeScript AST path to decide whether this is tokenization, precedence, or statement dispatch.

Compiler dumps:

#### tokens

- ok: `True`
- truncated: `True`

```text
== tokens ==
[
    SpannedToken {
        kind: Function,
        span: Span {
            start: 374,
            end: 382,
        },
    },
    SpannedToken {
        kind: Star,
        span: Span {
            start: 382,
            end: 383,
        },
    },
    SpannedToken {
        kind: Ident(
            "invalidControls",
        ),
        span: Span {
            start: 384,
            end: 399,
        },
    },
    SpannedToken {
        kind: LeftParen,
        span: Span {
            start: 399,
            end: 400,
        },
    },
    SpannedToken {
        kind: RightParen,
        span: Span {
            start: 400,
            end: 401,
        },
    },
    SpannedToken {
        kind: LeftBrace,
        span: Span {
            start: 402,
            end: 403,
        },
    },
    SpannedToken {
        kind: For,
        span: Span {
            start: 437,
            end: 440,
        },
    },
    SpannedToken {
        kind: LeftParen,
        span: Span {
            start: 441,
            end: 442,
        },
    },
    SpannedToken {
        kind: Var,
        span: Span {
            start: 442,
            end: 445,
        },
    },
    SpannedToken {
        kind: Ident(
            "alpha",
        ),
        span: Span {
            start: 446,
            end: 451,
        },
    },
    SpannedToken {
        kind: Equal,
        span: Span {
            start: 452,
            end: 453,
        },
    },
    SpannedToken {
        kind: Number(
            1040,
        ),
        span: Span {
            start: 454,
            end: 460,
        },
    },
    SpannedToken {
        kind: Semicolon,
        span: Span {
            start: 460,
            end: 461,
        },
    },
    SpannedToken {
        kind: Ident(
            "alpha",
        ),
        span: Span {
            start: 462,
            end: 467,
        },
    },
    SpannedToken {
        kind: LessEqual,
        span: Span {
            start: 468,
            end: 470,
        },
    },
    SpannedToken {
        kind: Number(
            1071,
        ),
        span: Span {
            start: 471,
            end: 477,
        },
    },
    SpannedToken {
        kind: Semicolon,
        span: Span {
            start: 477,
            end: 478,
        },
    },
    SpannedToken {
        kind: Ident(
            "alpha",
        ),
        span: Span {
            start: 479,
            end: 484,
        },
    },
    SpannedToken {
        kind: Increment,
        span: Span {
            start: 484,
            end: 486,
        },
    },
    SpannedToken {
        kind: RightParen,
        span: Span {
            start: 486,
            end: 487,
        },
    },
    SpannedToken {
        kind: LeftBrace,
        span: Span {
            start: 488,
            end: 489,
        },
    },
    SpannedToken {
        kind: Ident(
            "yield",
        ),
        span: Span {
            start: 494,
            end: 499,
        },
    },
    SpannedToken {
        kind: Ident(
            "String",
        ),
        span: Span {
            start: 500,
            end: 506,
        },
    },
    SpannedToken {
        kind: Dot,
        span: Span {
            start: 506,
            end: 507,
        },
    },
    SpannedToken {
        kind: Ident(
            "fromCharCode",
        ),
        span: Span {
            start: 507,
            end: 519,
        },
    },
    SpannedToken {
```

#### ast

- ok: `False`
- truncated: `False`

```text
error: [UnsupportedSyntax] expected identifier, got Some(SpannedToken { kind: Star, span: Span { start: 382, end: 383 } }) at 384..399
```

#### resolved

- ok: `False`
- truncated: `False`

```text
error: [UnsupportedSyntax] expected identifier, got Some(SpannedToken { kind: Star, span: Span { start: 382, end: 383 } }) at 384..399
```

TypeScript/JavaScript oracle:

```json
{
  "ok": false,
  "returncode": 2,
  "typescript": {
    "ok": false,
    "error": "failed to load TypeScript compiler API: Cannot find module 'typescript'\nRequire stack:\n- /home/wogikaze/ts2wasm/scripts/check/typescript-oracle.js",
    "diagnostics": [],
    "hints": []
  },
  "ast_error": "node:internal/modules/cjs/loader:1423\n  throw err;\n  ^\n\nError: Cannot find module 'typescript'\nRequire stack:\n- /home/wogikaze/ts2wasm/[eval]\n    at Module._resolveFilename (node:internal/modules/cjs/loader:1420:15)\n    at defaultResolveImpl (node:internal/modules/cjs/loader:1058:19)\n    at resolveForCJSWithHooks (node:internal/modules/cjs/loader:1063:22)\n    at Module._load (node:internal/modules/cjs/loader:1226:37)\n    at TracingChannel.traceSync (node:diagnostics_channel:328:14)\n    at wrapModuleLoad (node:internal/modules/cjs/loader:245:24)\n    at Module.require (node:internal/modules/cjs/loader:1503:12)\n    at require (node:internal/modules/helpers:152:16)\n    at [eval]:3:12\n    at runScriptInThisContext (node:internal/vm:219:10) {\n  code: 'MODULE_NOT_FOUND',\n  requireStack: [ '/home/wogikaze/ts2wasm/[eval]' ]\n}\n\nNode.js v25.2.1\n"
}
```

Stack trace:

```text
error: [UnsupportedSyntax] expected identifier, got Some(SpannedToken { kind: Star, span: Span { start: 382, end: 383 } }) at 384..399
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
