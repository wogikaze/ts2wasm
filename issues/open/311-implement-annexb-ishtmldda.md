---
id: 311
title: "Implement annexb-ishtmldda support"
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

Triage annexb-ishtmldda feature across 14 failing reference test cases and split this bucket into implementation-ready child issues.

## Problem

Reference test results show 14 cases fail with annexb-ishtmldda diagnostic. The compiler cannot handle these syntax/semantics, preventing compilation of code in this category.

Problem: annexb-ishtmldda feature has 14 reference failures and needs smart-triage evidence before implementation starts.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- test262 reference/test262/test/annexB/language/expressions/coalesce/emulates-undefined.js
```

Coverage window:

```sh
mise run reference-coverage -- test262 --path-filter reference/test262/test/annexB/language/expressions/coalesce/emulates-undefined.js --detail
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
mise run reference-coverage -- test262 --limit 28
mise run reference-coverage -- test262 --path-filter reference/test262/test/annexB/language/expressions/coalesce/emulates-undefined.js --detail
mise run reference-triage -- test262 reference/test262/test/annexB/language/expressions/coalesce/emulates-undefined.js
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

- `reference/test262/test/annexB/language/expressions/coalesce/emulates-undefined.js`
- `reference/test262/test/annexB/language/expressions/conditional/emulates-undefined.js`
- `reference/test262/test/annexB/language/expressions/does-not-equals/emulates-undefined.js`
- `reference/test262/test/annexB/language/expressions/equals/emulates-undefined.js`
- `reference/test262/test/annexB/language/expressions/logical-and/emulates-undefined.js`
- `reference/test262/test/annexB/language/expressions/logical-assignment/emulates-undefined-and.js`
- `reference/test262/test/annexB/language/expressions/logical-assignment/emulates-undefined-coalesce.js`
- `reference/test262/test/annexB/language/expressions/logical-assignment/emulates-undefined-or.js`
- `reference/test262/test/annexB/language/expressions/logical-not/emulates-undefined.js`
- `reference/test262/test/annexB/language/expressions/logical-or/emulates-undefined.js`
- ... and 4 more files

## Duplicate detection

- `issues/done/060-investigate-unknown-unsupported-cases.md` - Investigate and classify unknown-unsupported diagnostic cases (same feature label, same group key)
- `issues/done/228-implement-logical-assignment-operators.md` - Implement logical assignment operators (same reference path, same feature label, same group key, title overlap)
- `issues/done/237-implement-annexb-ishtmldda-compatibility.md` - Implement Annex B IsHTMLDDA compatibility (same reference path, same feature label, same group key, title overlap)

## Smart triage

### Smart triage: Triage annexb ishtmldda: emulates undefined

- Issue class: `triage-needed`
- Feature label: `annexb-ishtmldda`
- Diagnostic: `UnsupportedSyntax` / `parser-or-frontend-unsupported`
- Path: `reference/test262/test/annexB/language/expressions/coalesce/emulates-undefined.js`

Reproduction:

```sh
mise run reference-triage -- test262 reference/test262/test/annexB/language/expressions/coalesce/emulates-undefined.js
```

Source overview:

```json
{
  "suite": "test262",
  "bytes": 681,
  "lines": 20,
  "extension": ".js",
  "first_code_line": "esid: sec-binary-bitwise-operators-runtime-semantics-evaluation",
  "test262_metadata": {
    "esid": "sec-binary-bitwise-operators-runtime-semantics-evaluation",
    "description": ">",
    "info": "|",
    "CoalesceExpression": "CoalesceExpressionHead ?? BitwiseORExpression",
    "features": "[IsHTMLDDA, coalesce-expression]"
  }
}
```

Failure location:

```json
{
  "code": "UnsupportedSyntax",
  "message": "issue-237: Annex B [[IsHTMLDDA]] test262 host hook `$262.IsHTMLDDA` is not modeled; document.all compatibility semantics are unsupported at 610..624",
  "span_start": 610,
  "span_end": 624,
  "line": 18,
  "column": 17,
  "feature_label": "annexb-ishtmldda",
  "error_type": "parser-or-frontend-unsupported"
}
```

Source context:

```text
15 | features: [IsHTMLDDA, coalesce-expression]
16 | ---*/
17 | 
18 | var IsHTMLDDA = $262.IsHTMLDDA;
19 | 
20 | assert.sameValue(IsHTMLDDA ?? unresolved, IsHTMLDDA);
```

Visible symbols before failure:

```json
[
  {
    "kind": "binding",
    "name": "IsHTMLDDA",
    "line": 18,
    "column": 1,
    "initializer": ""
  }
]
```

Duplicate candidates:

```json
[
  {
    "state": "done",
    "path": "issues/done/237-implement-annexb-ishtmldda-compatibility.md",
    "title": "Implement Annex B IsHTMLDDA compatibility",
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
- truncated: `False`

```text
== tokens ==
[
    SpannedToken {
        kind: Var,
        span: Span {
            start: 594,
            end: 597,
        },
    },
    SpannedToken {
        kind: Ident(
            "IsHTMLDDA",
        ),
        span: Span {
            start: 598,
            end: 607,
        },
    },
    SpannedToken {
        kind: Equal,
        span: Span {
            start: 608,
            end: 609,
        },
    },
    SpannedToken {
        kind: Ident(
            "$262",
        ),
        span: Span {
            start: 610,
            end: 614,
        },
    },
    SpannedToken {
        kind: Dot,
        span: Span {
            start: 614,
            end: 615,
        },
    },
    SpannedToken {
        kind: Ident(
            "IsHTMLDDA",
        ),
        span: Span {
            start: 615,
            end: 624,
        },
    },
    SpannedToken {
        kind: Semicolon,
        span: Span {
            start: 624,
            end: 625,
        },
    },
    SpannedToken {
        kind: Ident(
            "assert",
        ),
        span: Span {
            start: 627,
            end: 633,
        },
    },
    SpannedToken {
        kind: Dot,
        span: Span {
            start: 633,
            end: 634,
        },
    },
    SpannedToken {
        kind: Ident(
            "sameValue",
        ),
        span: Span {
            start: 634,
            end: 643,
        },
    },
    SpannedToken {
        kind: LeftParen,
        span: Span {
            start: 643,
            end: 644,
        },
    },
    SpannedToken {
        kind: Ident(
            "IsHTMLDDA",
        ),
        span: Span {
            start: 644,
            end: 653,
        },
    },
    SpannedToken {
        kind: NullishCoalesce,
        span: Span {
            start: 654,
            end: 656,
        },
    },
    SpannedToken {
        kind: Ident(
            "unresolved",
        ),
        span: Span {
            start: 657,
            end: 667,
        },
    },
    SpannedToken {
        kind: Comma,
        span: Span {
            start: 667,
            end: 668,
        },
    },
    SpannedToken {
        kind: Ident(
            "IsHTMLDDA",
        ),
        span: Span {
            start: 669,
            end: 678,
        },
    },
    SpannedToken {
        kind: RightParen,
        span: Span {
            start: 678,
            end: 679,
        },
    },
    SpannedToken {
        kind: Semicolon,
        span: Span {
            start: 679,
            end: 680,
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
        name: "IsHTMLDDA",
        expr: Member {
            object: Ident {
                name: "$262",
                span: Span {
                    start: 610,
                    end: 614,
                },
            },
            property: "IsHTMLDDA",
            span: Span {
                start: 610,
                end: 624,
            },
        },
        span: Span {
            start: 594,
            end: 625,
        },
    },
    Expr {
        expr: Call {
            callee: Member {
                object: Ident {
                    name: "assert",
                    span: Span {
                        start: 627,
                        end: 633,
                    },
                },
                property: "sameValue",
                span: Span {
                    start: 627,
                    end: 643,
                },
            },
            args: [
                Binary {
                    left: Ident {
                        name: "IsHTMLDDA",
                        span: Span {
                            start: 644,
                            end: 653,
                        },
                    },
                    op: NullishCoalesce,
                    right: Ident {
                        name: "unresolved",
                        span: Span {
                            start: 657,
                            end: 667,
                        },
                    },
                    span: Span {
                        start: 644,
                        end: 667,
                    },
                },
                Ident {
                    name: "IsHTMLDDA",
                    span: Span {
                        start: 669,
                        end: 678,
                    },
                },
            ],
            span: Span {
                start: 627,
                end: 679,
            },
        },
        span: Span {
            start: 627,
            end: 680,
        },
    },
]
```

#### resolved

- ok: `False`
- truncated: `False`

```text
error: [UnsupportedSyntax] issue-237: Annex B [[IsHTMLDDA]] test262 host hook `$262.IsHTMLDDA` is not modeled; document.all compatibility semantics are unsupported at 610..624
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
error: [UnsupportedSyntax] issue-237: Annex B [[IsHTMLDDA]] test262 host hook `$262.IsHTMLDDA` is not modeled; document.all compatibility semantics are unsupported at 610..624
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
