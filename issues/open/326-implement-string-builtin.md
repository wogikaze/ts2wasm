---
id: 326
title: "Implement string-builtin support"
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

Triage string-builtin feature across 111 failing reference test cases and split this bucket into implementation-ready child issues.

## Problem

Reference test results show 111 cases fail with string-builtin diagnostic. The compiler cannot handle these syntax/semantics, preventing compilation of code in this category.

Problem: string-builtin feature has 111 reference failures and needs smart-triage evidence before implementation starts.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- test262 reference/test262/test/annexB/built-ins/String/prototype/anchor/B.2.3.2.js
```

Coverage window:

```sh
mise run reference-coverage -- test262 --path-filter reference/test262/test/annexB/built-ins/String/prototype/anchor/B.2.3.2.js --detail
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
mise run reference-coverage -- test262 --limit 222
mise run reference-coverage -- test262 --path-filter reference/test262/test/annexB/built-ins/String/prototype/anchor/B.2.3.2.js --detail
mise run reference-triage -- test262 reference/test262/test/annexB/built-ins/String/prototype/anchor/B.2.3.2.js
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

- `reference/test262/test/annexB/built-ins/String/prototype/anchor/B.2.3.2.js`
- `reference/test262/test/annexB/built-ins/String/prototype/anchor/attr-tostring-err.js`
- `reference/test262/test/annexB/built-ins/String/prototype/anchor/length.js`
- `reference/test262/test/annexB/built-ins/String/prototype/anchor/name.js`
- `reference/test262/test/annexB/built-ins/String/prototype/anchor/not-a-constructor.js`
- `reference/test262/test/annexB/built-ins/String/prototype/anchor/prop-desc.js`
- `reference/test262/test/annexB/built-ins/String/prototype/anchor/this-val-tostring-err.js`
- `reference/test262/test/annexB/built-ins/String/prototype/big/B.2.3.3.js`
- `reference/test262/test/annexB/built-ins/String/prototype/big/length.js`
- `reference/test262/test/annexB/built-ins/String/prototype/big/name.js`
- ... and 101 more files

## Duplicate detection

- `issues/open/067-implement-unknown-unsupported.md` - Investigate and classify unknown-unsupported cases (same reference path, same feature label, same group key)
- `issues/open/068-implement-unsupported-expression.md` - Implement unsupported expression types (same reference path, title overlap)
- `issues/done/060-investigate-unknown-unsupported-cases.md` - Investigate and classify unknown-unsupported diagnostic cases (same feature label, same group key)
- `issues/done/062-implement-function.md` - Implement function support (same reference path, title overlap)
- `issues/done/065-implement-parser-syntax.md` - Implement parser syntax extensions (same reference path, title overlap)
- `issues/done/224-implement-annexb-html-comments.md` - Implement Annex B HTML-like comments (same feature label, same group key, title overlap)
- `issues/done/228-implement-logical-assignment-operators.md` - Implement logical assignment operators (same feature label, same group key, title overlap)

## Smart triage

### Smart triage: Triage name resolution: B.2.3.2

- Issue class: `triage-needed`
- Feature label: `name-resolution`
- Diagnostic: `UnresolvedName` / `resolver-symbol`
- Path: `reference/test262/test/annexB/built-ins/String/prototype/anchor/B.2.3.2.js`

Reproduction:

```sh
mise run reference-triage -- test262 reference/test262/test/annexB/built-ins/String/prototype/anchor/B.2.3.2.js
```

Source overview:

```json
{
  "suite": "test262",
  "bytes": 1050,
  "lines": 26,
  "extension": ".js",
  "first_code_line": "description: >",
  "test262_metadata": {
    "description": ">",
    "es6id": "B.2.3.2"
  }
}
```

Failure location:

```json
{
  "code": "UnresolvedName",
  "message": "unresolved name: `assert` at 885..891",
  "span_start": 885,
  "span_end": 891,
  "line": 21,
  "column": 1,
  "feature_label": "name-resolution",
  "error_type": "resolver-symbol"
}
```

Source context:

```text
18 | assert.sameValue('_'.anchor(0x2A), '<a name="42">_</a>');
19 | assert.sameValue('_'.anchor('\x22'), '<a name="&quot;">_</a>');
20 | assert.sameValue(String.prototype.anchor.call(0x2A, 0x2A), '<a name="42">42</a>');
21 | assert.throws(TypeError, function() {
22 |   String.prototype.anchor.call(undefined);
23 | });
24 | assert.throws(TypeError, function() {
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
    "path": "issues/open/064-implement-name-resolution.md",
    "title": "Implement name resolution",
    "reason": "same feature label, title overlap"
  },
  {
    "state": "open",
    "path": "issues/open/067-implement-unknown-unsupported.md",
    "title": "Investigate and classify unknown-unsupported cases",
    "reason": "same reference path"
  },
  {
    "state": "open",
    "path": "issues/open/321-implement-name-resolution.md",
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
- truncated: `True`

```text
== tokens ==
[
    SpannedToken {
        kind: Ident(
            "assert",
        ),
        span: Span {
            start: 568,
            end: 574,
        },
    },
    SpannedToken {
        kind: Dot,
        span: Span {
            start: 574,
            end: 575,
        },
    },
    SpannedToken {
        kind: Ident(
            "sameValue",
        ),
        span: Span {
            start: 575,
            end: 584,
        },
    },
    SpannedToken {
        kind: LeftParen,
        span: Span {
            start: 584,
            end: 585,
        },
    },
    SpannedToken {
        kind: String(
            "_",
        ),
        span: Span {
            start: 585,
            end: 588,
        },
    },
    SpannedToken {
        kind: Dot,
        span: Span {
            start: 588,
            end: 589,
        },
    },
    SpannedToken {
        kind: Ident(
            "anchor",
        ),
        span: Span {
            start: 589,
            end: 595,
        },
    },
    SpannedToken {
        kind: LeftParen,
        span: Span {
            start: 595,
            end: 596,
        },
    },
    SpannedToken {
        kind: String(
            "b",
        ),
        span: Span {
            start: 596,
            end: 599,
        },
    },
    SpannedToken {
        kind: RightParen,
        span: Span {
            start: 599,
            end: 600,
        },
    },
    SpannedToken {
        kind: Comma,
        span: Span {
            start: 600,
            end: 601,
        },
    },
    SpannedToken {
        kind: String(
            "<a name=\"b\">_</a>",
        ),
        span: Span {
            start: 602,
            end: 621,
        },
    },
    SpannedToken {
        kind: RightParen,
        span: Span {
            start: 621,
            end: 622,
        },
    },
    SpannedToken {
        kind: Semicolon,
        span: Span {
            start: 622,
            end: 623,
        },
    },
    SpannedToken {
        kind: Ident(
            "assert",
        ),
        span: Span {
            start: 624,
            end: 630,
        },
    },
    SpannedToken {
        kind: Dot,
        span: Span {
            start: 630,
            end: 631,
        },
    },
    SpannedToken {
        kind: Ident(
            "sameValue",
        ),
        span: Span {
            start: 631,
            end: 640,
        },
    },
    SpannedToken {
        kind: LeftParen,
        span: Span {
            start: 640,
            end: 641,
        },
    },
    SpannedToken {
        kind: String(
            "<",
        ),
        span: Span {
            start: 641,
            end: 644,
        },
    },
    SpannedToken {
        kind: Dot,
        span: Span {
            start: 644,
            end: 645,
        },
    },
    SpannedToken {
        kind: Ident(
            "anchor",
        ),
        span: Span {
            start: 645,
            end: 651,
        },
    },
    SpannedToken {
        kind: LeftParen,
        span: Span {
            start: 651,
            end: 652,
        },
    },
    SpannedToken {
        kind: String(
            "<",
        ),
        span: Span {
            start: 652,
            end: 655,
        },
    },
    SpannedToken {
        kind: RightParen,
        span: Span {
            start: 655,
            end: 656,
        },
    },
    SpannedToken {
        kind: Comma,
        span: Span {
            start: 656,
```

#### ast

- ok: `True`
- truncated: `True`

```text
== ast ==
[
    Expr {
        expr: Call {
            callee: Member {
                object: Ident {
                    name: "assert",
                    span: Span {
                        start: 568,
                        end: 574,
                    },
                },
                property: "sameValue",
                span: Span {
                    start: 568,
                    end: 584,
                },
            },
            args: [
                Call {
                    callee: Member {
                        object: String {
                            value: "_",
                            span: Span {
                                start: 585,
                                end: 588,
                            },
                        },
                        property: "anchor",
                        span: Span {
                            start: 585,
                            end: 595,
                        },
                    },
                    args: [
                        String {
                            value: "b",
                            span: Span {
                                start: 596,
                                end: 599,
                            },
                        },
                    ],
                    span: Span {
                        start: 585,
                        end: 600,
                    },
                },
                String {
                    value: "<a name=\"b\">_</a>",
                    span: Span {
                        start: 602,
                        end: 621,
                    },
                },
            ],
            span: Span {
                start: 568,
                end: 622,
            },
        },
        span: Span {
            start: 568,
            end: 623,
        },
    },
    Expr {
        expr: Call {
            callee: Member {
                object: Ident {
                    name: "assert",
                    span: Span {
                        start: 624,
                        end: 630,
                    },
                },
                property: "sameValue",
                span: Span {
                    start: 624,
                    end: 640,
                },
            },
            args: [
                Call {
                    callee: Member {
                        object: String {
                            value: "<",
                            span: Span {
                                start: 641,
                                end: 644,
                            },
                        },
                        property: "anchor",
                        span: Span {
                            start: 641,
                            end: 651,
                        },
                    },
                    args: [
                        String {
                            value: "<",
                            span: Span {
                                start: 652,
                                end: 655,
                            },
                        },
                    ],
                    span: Span {
                        start: 641,
                        end: 656,
                    },
                },
                String {
                    value: "<a name=\"<\"><</a>",
                    span: Span {
                        start: 658,
                        e
```

#### resolved

- ok: `False`
- truncated: `False`

```text
error: [UnresolvedName] unresolved name: `assert` at 885..891
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
error: [UnresolvedName] unresolved name: `assert` at 885..891
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
