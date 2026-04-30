---
id: 314
title: "Implement Date object support"
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

Triage date feature across 618 failing reference test cases and split this bucket into implementation-ready child issues.

## Problem

Reference test results show 618 cases fail with date diagnostic. The compiler cannot handle these syntax/semantics, preventing compilation of code in this category.

Problem: date feature has 618 reference failures and needs smart-triage evidence before implementation starts.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- test262 reference/test262/test/annexB/built-ins/Date/prototype/getYear/B.2.4.js
```

Coverage window:

```sh
mise run reference-coverage -- test262 --path-filter reference/test262/test/annexB/built-ins/Date/prototype/getYear/B.2.4.js --detail
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
mise run reference-coverage -- test262 --limit 1236
mise run reference-coverage -- test262 --path-filter reference/test262/test/annexB/built-ins/Date/prototype/getYear/B.2.4.js --detail
mise run reference-triage -- test262 reference/test262/test/annexB/built-ins/Date/prototype/getYear/B.2.4.js
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

- `reference/test262/test/annexB/built-ins/Date/prototype/getYear/B.2.4.js`
- `reference/test262/test/annexB/built-ins/Date/prototype/getYear/length.js`
- `reference/test262/test/annexB/built-ins/Date/prototype/getYear/name.js`
- `reference/test262/test/annexB/built-ins/Date/prototype/getYear/nan.js`
- `reference/test262/test/annexB/built-ins/Date/prototype/getYear/not-a-constructor.js`
- `reference/test262/test/annexB/built-ins/Date/prototype/getYear/return-value.js`
- `reference/test262/test/annexB/built-ins/Date/prototype/getYear/this-not-date.js`
- `reference/test262/test/annexB/built-ins/Date/prototype/setYear/B.2.5.js`
- `reference/test262/test/annexB/built-ins/Date/prototype/setYear/date-value-read-before-tonumber-when-date-is-invalid.js`
- `reference/test262/test/annexB/built-ins/Date/prototype/setYear/date-value-read-before-tonumber-when-date-is-valid.js`
- ... and 608 more files

## Duplicate detection

- `issues/open/017b-implement-gc-strategy.md` - issues/open/017b-implement-gc-strategy.md (same feature label, same group key)
- `issues/open/021-implement-full-wasm-backend.md` - issues/open/021-implement-full-wasm-backend.md (same feature label, same group key)
- `issues/open/050-implement-date.md` - Implement Date (same feature label, same group key, title overlap)
- `issues/open/052-implement-json.md` - Implement JSON (same feature label, same group key, title overlap)
- `issues/open/052d-implement-json-stringify-broader-replacer-semantics.md` - Implement broader JSON.stringify replacer semantics (same feature label, same group key, title overlap)
- `issues/open/064-implement-name-resolution.md` - Implement name resolution (same reference path, same feature label, same group key, title overlap)
- `issues/open/066-implement-regexp-literal.md` - Implement RegExp literal support (same feature label, same group key, title overlap)
- `issues/open/067-implement-unknown-unsupported.md` - Investigate and classify unknown-unsupported cases (same feature label, same group key)
- `issues/open/068-implement-unsupported-expression.md` - Implement unsupported expression types (same feature label, same group key, title overlap)
- `issues/open/069-implement-APILibCheck.md` - Implement Apilibcheck (same feature label, same group key, title overlap)

## Smart triage

### Smart triage: Triage name resolution: B.2.4

- Issue class: `triage-needed`
- Feature label: `name-resolution`
- Diagnostic: `UnresolvedName` / `resolver-symbol`
- Path: `reference/test262/test/annexB/built-ins/Date/prototype/getYear/B.2.4.js`

Reproduction:

```sh
mise run reference-triage -- test262 reference/test262/test/annexB/built-ins/Date/prototype/getYear/B.2.4.js
```

Source overview:

```json
{
  "suite": "test262",
  "bytes": 425,
  "lines": 16,
  "extension": ".js",
  "first_code_line": "es5id: B.2.4",
  "test262_metadata": {
    "es5id": "B.2.4",
    "description": ">",
    "includes": "[propertyHelper.js]"
  }
}
```

Failure location:

```json
{
  "code": "UnresolvedName",
  "message": "unresolved name: `verifyProperty` at 317..331",
  "span_start": 317,
  "span_end": 331,
  "line": 12,
  "column": 1,
  "feature_label": "name-resolution",
  "error_type": "resolver-symbol"
}
```

Source context:

```text
 9 | includes: [propertyHelper.js]
10 | ---*/
11 | 
12 | verifyProperty(Date.prototype, "getYear", {
13 |   enumerable: false,
14 |   writable: true,
15 |   configurable: true
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
    "reason": "same reference path, same feature label, title overlap"
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
        kind: Ident(
            "verifyProperty",
        ),
        span: Span {
            start: 317,
            end: 331,
        },
    },
    SpannedToken {
        kind: LeftParen,
        span: Span {
            start: 331,
            end: 332,
        },
    },
    SpannedToken {
        kind: Ident(
            "Date",
        ),
        span: Span {
            start: 332,
            end: 336,
        },
    },
    SpannedToken {
        kind: Dot,
        span: Span {
            start: 336,
            end: 337,
        },
    },
    SpannedToken {
        kind: Ident(
            "prototype",
        ),
        span: Span {
            start: 337,
            end: 346,
        },
    },
    SpannedToken {
        kind: Comma,
        span: Span {
            start: 346,
            end: 347,
        },
    },
    SpannedToken {
        kind: String(
            "getYear",
        ),
        span: Span {
            start: 348,
            end: 357,
        },
    },
    SpannedToken {
        kind: Comma,
        span: Span {
            start: 357,
            end: 358,
        },
    },
    SpannedToken {
        kind: LeftBrace,
        span: Span {
            start: 359,
            end: 360,
        },
    },
    SpannedToken {
        kind: Ident(
            "enumerable",
        ),
        span: Span {
            start: 363,
            end: 373,
        },
    },
    SpannedToken {
        kind: Colon,
        span: Span {
            start: 373,
            end: 374,
        },
    },
    SpannedToken {
        kind: False,
        span: Span {
            start: 375,
            end: 380,
        },
    },
    SpannedToken {
        kind: Comma,
        span: Span {
            start: 380,
            end: 381,
        },
    },
    SpannedToken {
        kind: Ident(
            "writable",
        ),
        span: Span {
            start: 384,
            end: 392,
        },
    },
    SpannedToken {
        kind: Colon,
        span: Span {
            start: 392,
            end: 393,
        },
    },
    SpannedToken {
        kind: True,
        span: Span {
            start: 394,
            end: 398,
        },
    },
    SpannedToken {
        kind: Comma,
        span: Span {
            start: 398,
            end: 399,
        },
    },
    SpannedToken {
        kind: Ident(
            "configurable",
        ),
        span: Span {
            start: 402,
            end: 414,
        },
    },
    SpannedToken {
        kind: Colon,
        span: Span {
            start: 414,
            end: 415,
        },
    },
    SpannedToken {
        kind: True,
        span: Span {
            start: 416,
            end: 420,
        },
    },
    SpannedToken {
        kind: RightBrace,
        span: Span {
            start: 421,
            end: 422,
        },
    },
    SpannedToken {
        kind: RightParen,
        span: Span {
            start: 422,
            end: 423,
        },
    },
    SpannedToken {
        kind: Semicolon,
        span: Span {
            start: 423,
            end: 424,
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
    Expr {
        expr: Call {
            callee: Ident {
                name: "verifyProperty",
                span: Span {
                    start: 317,
                    end: 331,
                },
            },
            args: [
                Member {
                    object: Ident {
                        name: "Date",
                        span: Span {
                            start: 332,
                            end: 336,
                        },
                    },
                    property: "prototype",
                    span: Span {
                        start: 332,
                        end: 346,
                    },
                },
                String {
                    value: "getYear",
                    span: Span {
                        start: 348,
                        end: 357,
                    },
                },
                Object {
                    props: [
                        (
                            "enumerable",
                            Bool {
                                value: false,
                                span: Span {
                                    start: 375,
                                    end: 380,
                                },
                            },
                        ),
                        (
                            "writable",
                            Bool {
                                value: true,
                                span: Span {
                                    start: 394,
                                    end: 398,
                                },
                            },
                        ),
                        (
                            "configurable",
                            Bool {
                                value: true,
                                span: Span {
                                    start: 416,
                                    end: 420,
                                },
                            },
                        ),
                    ],
                    span: Span {
                        start: 359,
                        end: 422,
                    },
                },
            ],
            span: Span {
                start: 317,
                end: 423,
            },
        },
        span: Span {
            start: 317,
            end: 424,
        },
    },
]
```

#### resolved

- ok: `False`
- truncated: `False`

```text
error: [UnresolvedName] unresolved name: `verifyProperty` at 317..331
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
error: [UnresolvedName] unresolved name: `verifyProperty` at 317..331
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
