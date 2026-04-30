---
id: 319
title: "Implement html-comment support"
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

Triage html-comment feature across 5 failing reference test cases and split this bucket into implementation-ready child issues.

## Problem

Reference test results show 5 cases fail with html-comment diagnostic. The compiler cannot handle these syntax/semantics, preventing compilation of code in this category.

Problem: html-comment feature has 5 reference failures and needs smart-triage evidence before implementation starts.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- test262 reference/test262/test/annexB/language/comments/multi-line-html-close.js
```

Coverage window:

```sh
mise run reference-coverage -- test262 --path-filter reference/test262/test/annexB/language/comments/multi-line-html-close.js --detail
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
mise run reference-coverage -- test262 --limit 10
mise run reference-coverage -- test262 --path-filter reference/test262/test/annexB/language/comments/multi-line-html-close.js --detail
mise run reference-triage -- test262 reference/test262/test/annexB/language/comments/multi-line-html-close.js
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

- `reference/test262/test/annexB/language/comments/multi-line-html-close.js`
- `reference/test262/test/annexB/language/comments/single-line-html-close-asi.js`
- `reference/test262/test/annexB/language/comments/single-line-html-close-unicode-separators.js`
- `reference/test262/test/annexB/language/comments/single-line-html-close.js`
- `reference/test262/test/annexB/language/comments/single-line-html-open.js`

## Duplicate detection

- `issues/done/060-investigate-unknown-unsupported-cases.md` - Investigate and classify unknown-unsupported diagnostic cases (same feature label, same group key)
- `issues/done/224-implement-annexb-html-comments.md` - Implement Annex B HTML-like comments (same reference path, same feature label, same group key, title overlap)

## Smart triage

### Smart triage: Triage name resolution: multi line html close

- Issue class: `triage-needed`
- Feature label: `name-resolution`
- Diagnostic: `UnresolvedName` / `resolver-symbol`
- Path: `reference/test262/test/annexB/language/comments/multi-line-html-close.js`

Reproduction:

```sh
mise run reference-triage -- test262 reference/test262/test/annexB/language/comments/multi-line-html-close.js
```

Source overview:

```json
{
  "suite": "test262",
  "bytes": 2523,
  "lines": 89,
  "extension": ".js",
  "first_code_line": "esid: sec-html-like-comments",
  "test262_metadata": {
    "esid": "sec-html-like-comments",
    "description": "Optional HTMLCloseComment following MultiLineComment",
    "info": "|",
    "Comment": ":",
    "MultiLineComment": ":",
    "HTMLCloseComment": ":",
    "negative": "",
    "phase": "runtime",
    "type": "Test262Error"
  }
}
```

Failure location:

```json
{
  "code": "UnresolvedName",
  "message": "unresolved name: `Test262Error` at 2505..2517",
  "span_start": 2505,
  "span_end": 2517,
  "line": 88,
  "column": 13,
  "feature_label": "name-resolution",
  "error_type": "resolver-symbol"
}
```

Source context:

```text
85 | // guarantees that test runners will only consider the test "passing" if
86 | // executable sequences are correctly interpreted as such.
87 | if (counter === 12) {
88 |   throw new Test262Error();
89 | }
```

Visible symbols before failure:

```json
[
  {
    "kind": "binding",
    "name": "counter",
    "line": 24,
    "column": 1,
    "initializer": "0"
  }
]
```

Duplicate candidates:

```json
[
  {
    "state": "open",
    "path": "issues/open/064-implement-name-resolution.md",
    "title": "Implement name resolution",
    "reason": "same feature label, title overlap"
  },
  {
    "state": "done",
    "path": "issues/done/224-implement-annexb-html-comments.md",
    "title": "Implement Annex B HTML-like comments",
    "reason": "same reference path, same feature label"
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
- truncated: `True`

```text
== tokens ==
[
    SpannedToken {
        kind: Var,
        span: Span {
            start: 713,
            end: 716,
        },
    },
    SpannedToken {
        kind: Ident(
            "counter",
        ),
        span: Span {
            start: 717,
            end: 724,
        },
    },
    SpannedToken {
        kind: Equal,
        span: Span {
            start: 725,
            end: 726,
        },
    },
    SpannedToken {
        kind: Number(
            0,
        ),
        span: Span {
            start: 727,
            end: 728,
        },
    },
    SpannedToken {
        kind: Semicolon,
        span: Span {
            start: 728,
            end: 729,
        },
    },
    SpannedToken {
        kind: Semicolon,
        span: Span {
            start: 735,
            end: 738,
        },
    },
    SpannedToken {
        kind: Ident(
            "counter",
        ),
        span: Span {
            start: 739,
            end: 746,
        },
    },
    SpannedToken {
        kind: PlusEqual,
        span: Span {
            start: 747,
            end: 749,
        },
    },
    SpannedToken {
        kind: Number(
            1,
        ),
        span: Span {
            start: 750,
            end: 751,
        },
    },
    SpannedToken {
        kind: Semicolon,
        span: Span {
            start: 751,
            end: 752,
        },
    },
    SpannedToken {
        kind: Semicolon,
        span: Span {
            start: 759,
            end: 801,
        },
    },
    SpannedToken {
        kind: Ident(
            "counter",
        ),
        span: Span {
            start: 802,
            end: 809,
        },
    },
    SpannedToken {
        kind: PlusEqual,
        span: Span {
            start: 810,
            end: 812,
        },
    },
    SpannedToken {
        kind: Number(
            1,
        ),
        span: Span {
            start: 813,
            end: 814,
        },
    },
    SpannedToken {
        kind: Semicolon,
        span: Span {
            start: 814,
            end: 815,
        },
    },
    SpannedToken {
        kind: Semicolon,
        span: Span {
            start: 848,
            end: 890,
        },
    },
    SpannedToken {
        kind: Ident(
            "counter",
        ),
        span: Span {
            start: 891,
            end: 898,
        },
    },
    SpannedToken {
        kind: PlusEqual,
        span: Span {
            start: 899,
            end: 901,
        },
    },
    SpannedToken {
        kind: Number(
            1,
        ),
        span: Span {
            start: 902,
            end: 903,
        },
    },
    SpannedToken {
        kind: Semicolon,
        span: Span {
            start: 903,
            end: 904,
        },
    },
    SpannedToken {
        kind: Semicolon,
        span: Span {
            start: 942,
            end: 984,
        },
    },
    SpannedToken {
        kind: Ident(
            "counter",
        ),
        span: Span {
            start: 985,
            end: 992,
        },
    },
    SpannedToken {
        kind: PlusEqual,
        span: Span {
            start: 993,
            end: 995,
        },
    },
    SpannedToken {
        kind: Number(
            1,
        ),
        span: Span {
            start: 996,
            end: 997,
        },
    },
    SpannedToken {
        kind: Semicolon,
        span: Span {
            start: 997,
            end: 998,
        },
    },
    S
```

#### ast

- ok: `True`
- truncated: `True`

```text
== ast ==
[
    Let {
        name: "counter",
        expr: Number {
            value: 0,
            span: Span {
                start: 727,
                end: 728,
            },
        },
        span: Span {
            start: 713,
            end: 729,
        },
    },
    Assign {
        name: "counter",
        expr: Binary {
            left: Ident {
                name: "counter",
                span: Span {
                    start: 739,
                    end: 746,
                },
            },
            op: Add,
            right: Number {
                value: 1,
                span: Span {
                    start: 750,
                    end: 751,
                },
            },
            span: Span {
                start: 739,
                end: 751,
            },
        },
        span: Span {
            start: 739,
            end: 752,
        },
    },
    Assign {
        name: "counter",
        expr: Binary {
            left: Ident {
                name: "counter",
                span: Span {
                    start: 802,
                    end: 809,
                },
            },
            op: Add,
            right: Number {
                value: 1,
                span: Span {
                    start: 813,
                    end: 814,
                },
            },
            span: Span {
                start: 802,
                end: 814,
            },
        },
        span: Span {
            start: 802,
            end: 815,
        },
    },
    Assign {
        name: "counter",
        expr: Binary {
            left: Ident {
                name: "counter",
                span: Span {
                    start: 891,
                    end: 898,
                },
            },
            op: Add,
            right: Number {
                value: 1,
                span: Span {
                    start: 902,
                    end: 903,
                },
            },
            span: Span {
                start: 891,
                end: 903,
            },
        },
        span: Span {
            start: 891,
            end: 904,
        },
    },
    Assign {
        name: "counter",
        expr: Binary {
            left: Ident {
                name: "counter",
                span: Span {
                    start: 985,
                    end: 992,
                },
            },
            op: Add,
            right: Number {
                value: 1,
                span: Span {
                    start: 996,
                    end: 997,
                },
            },
            span: Span {
                start: 985,
                end: 997,
            },
        },
        span: Span {
            start: 985,
            end: 998,
        },
    },
    Assign {
        name: "counter",
        expr: Binary {
            left: Ident {
                name: "counter",
                span: Span {
                    start: 1098,
                    end: 1105,
                },
            },
            op: Add,
            right: Number {
                value: 1,
                span: Span {
                    start: 1109,
                    end: 1110,
                },
            },
            span: Span {
                start: 1098,
                end: 1110,
            },
        },
        span: Span {
            start: 1098,
            end: 1111,
        },
    },
    Assign {
        name: "counter
```

#### resolved

- ok: `False`
- truncated: `False`

```text
error: [UnresolvedName] unresolved name: `Test262Error` at 2505..2517
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
error: [UnresolvedName] unresolved name: `Test262Error` at 2505..2517
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
