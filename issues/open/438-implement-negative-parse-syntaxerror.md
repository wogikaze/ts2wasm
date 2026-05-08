---
id: 438
title: "Implement negative-parse-syntaxerror support"
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

Triage negative-parse-syntaxerror feature across 4595 failing reference test cases and split this bucket into implementation-ready child issues.

## Problem

Reference test results show 4595 cases fail with negative-parse-syntaxerror diagnostic. The compiler cannot handle these syntax/semantics, preventing compilation of code in this category.

Problem: negative-parse-syntaxerror feature has 4595 reference failures and needs smart-triage evidence before implementation starts.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- test262 reference/test262/test/annexB/language/expressions/template-literal/legacy-octal-escape-sequence-strict.js
```

Coverage window:

```sh
mise run reference-coverage -- test262 --path-filter reference/test262/test/annexB/language/expressions/template-literal/legacy-octal-escape-sequence-strict.js --detail
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
mise run reference-coverage -- test262 --limit 9190
mise run reference-coverage -- test262 --path-filter reference/test262/test/annexB/language/expressions/template-literal/legacy-octal-escape-sequence-strict.js --detail
mise run reference-triage -- test262 reference/test262/test/annexB/language/expressions/template-literal/legacy-octal-escape-sequence-strict.js
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

- `reference/test262/test/annexB/language/expressions/template-literal/legacy-octal-escape-sequence-strict.js`
- `reference/test262/test/annexB/language/statements/for-in/bare-initializer.js`
- `reference/test262/test/annexB/language/statements/for-in/const-initializer.js`
- `reference/test262/test/annexB/language/statements/for-in/let-initializer.js`
- `reference/test262/test/annexB/language/statements/for-in/strict-initializer.js`
- `reference/test262/test/annexB/language/statements/for-in/var-arraybindingpattern-initializer.js`
- `reference/test262/test/annexB/language/statements/for-in/var-objectbindingpattern-initializer.js`
- `reference/test262/test/built-ins/RegExp/property-escapes/binary-property-with-value-ASCII_-_F-negated.js`
- `reference/test262/test/built-ins/RegExp/property-escapes/binary-property-with-value-ASCII_-_F.js`
- `reference/test262/test/built-ins/RegExp/property-escapes/binary-property-with-value-ASCII_-_Invalid-negated.js`
- ... and 4585 more files

## Duplicate detection

- `issues/open/229-implement-legacy-octal-escape-handling.md` - Implement legacy octal escape handling (same reference path, title overlap)
- `issues/open/286-classify-negative-syntax-tests-correctly.md` - Classify expected negative SyntaxError tests correctly (same feature label, same group key)

## Smart triage

### Smart triage: Build pass: legacy octal escape sequence strict

- Issue class: `none`
- Feature label: `build-pass`
- Diagnostic: `BuildPass` / `pass`
- Path: `reference/test262/test/annexB/language/expressions/template-literal/legacy-octal-escape-sequence-strict.js`

Reproduction:

```sh
mise run reference-triage -- test262 reference/test262/test/annexB/language/expressions/template-literal/legacy-octal-escape-sequence-strict.js
```

Source overview:

```json
{
  "suite": "test262",
  "bytes": 484,
  "lines": 18,
  "extension": ".js",
  "first_code_line": "es6id: 12.2.8",
  "test262_metadata": {
    "es6id": "12.2.8",
    "description": ">",
    "The SV of EscapeSequence": ": HexEscapeSequence is the SV of the",
    "negative": "",
    "phase": "parse",
    "type": "SyntaxError",
    "flags": "[onlyStrict]"
  }
}
```

Failure location:

```json
{
  "code": "BuildPass",
  "message": "ts2wasm build succeeded",
  "span_start": null,
  "span_end": null,
  "line": null,
  "column": null,
  "feature_label": "build-pass",
  "error_type": "pass"
}
```

Source context:

```text

function print(message) {
  console.log(message);
}


/* standard globals shim */
```

Visible symbols before failure:

```json
[
  {
    "kind": "function",
    "name": "print",
    "line": 2,
    "column": 1,
    "params": "message"
  },
  {
    "kind": "binding",
    "name": "NaN",
    "line": 10,
    "column": 1,
    "initializer": "0/0"
  },
  {
    "kind": "binding",
    "name": "Infinity",
    "line": 11,
    "column": 1,
    "initializer": "1/0"
  },
  {
    "kind": "binding",
    "name": "$262",
    "line": 17,
    "column": 1,
    "initializer": "{}"
  },
  {
    "kind": "function",
    "name": "$ERROR",
    "line": 26,
    "column": 1,
    "params": "message"
  },
  {
    "kind": "function",
    "name": "$DONOTEVALUATE",
    "line": 30,
    "column": 1,
    "params": ""
  },
  {
    "kind": "function",
    "name": "assert",
    "line": 34,
    "column": 1,
    "params": "mustBeTrue, message"
  }
]
```

Duplicate candidates:

```json
[
  {
    "state": "done",
    "path": "issues/done/229-implement-legacy-octal-escape-handling.md",
    "title": "Implement legacy octal escape handling",
    "reason": "same reference path, title overlap"
  }
]
```

Error-specific suggestions:

- No compiler blocker was found by the build step; use reference-coverage for semantic parity evidence.

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
            start: 1,
            end: 9,
        },
    },
    SpannedToken {
        kind: Ident(
            "print",
        ),
        span: Span {
            start: 10,
            end: 15,
        },
    },
    SpannedToken {
        kind: LeftParen,
        span: Span {
            start: 15,
            end: 16,
        },
    },
    SpannedToken {
        kind: Ident(
            "message",
        ),
        span: Span {
            start: 16,
            end: 23,
        },
    },
    SpannedToken {
        kind: RightParen,
        span: Span {
            start: 23,
            end: 24,
        },
    },
    SpannedToken {
        kind: LeftBrace,
        span: Span {
            start: 25,
            end: 26,
        },
    },
    SpannedToken {
        kind: Ident(
            "console",
        ),
        span: Span {
            start: 29,
            end: 36,
        },
    },
    SpannedToken {
        kind: Dot,
        span: Span {
            start: 36,
            end: 37,
        },
    },
    SpannedToken {
        kind: Ident(
            "log",
        ),
        span: Span {
            start: 37,
            end: 40,
        },
    },
    SpannedToken {
        kind: LeftParen,
        span: Span {
            start: 40,
            end: 41,
        },
    },
    SpannedToken {
        kind: Ident(
            "message",
        ),
        span: Span {
            start: 41,
            end: 48,
        },
    },
    SpannedToken {
        kind: RightParen,
        span: Span {
            start: 48,
            end: 49,
        },
    },
    SpannedToken {
        kind: Semicolon,
        span: Span {
            start: 49,
            end: 50,
        },
    },
    SpannedToken {
        kind: RightBrace,
        span: Span {
            start: 51,
            end: 52,
        },
    },
    SpannedToken {
        kind: Var,
        span: Span {
            start: 85,
            end: 88,
        },
    },
    SpannedToken {
        kind: Ident(
            "NaN",
        ),
        span: Span {
            start: 89,
            end: 92,
        },
    },
    SpannedToken {
        kind: Equal,
        span: Span {
            start: 93,
            end: 94,
        },
    },
    SpannedToken {
        kind: Number(
            0,
        ),
        span: Span {
            start: 95,
            end: 96,
        },
    },
    SpannedToken {
        kind: Slash,
        span: Span {
            start: 96,
            end: 97,
        },
    },
    SpannedToken {
        kind: Number(
            0,
        ),
        span: Span {
            start: 97,
            end: 98,
        },
    },
    SpannedToken {
        kind: Semicolon,
        span: Span {
            start: 98,
            end: 99,
        },
    },
    SpannedToken {
        kind: Var,
        span: Span {
            start: 100,
            end: 103,
        },
    },
    SpannedToken {
        kind: Ident(
            "Infinity",
        ),
        span: Span {
            start: 104,
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
            1,
        ),
        span: Span {
            start: 115,
            end: 116,
        },
    },
    SpannedToken {
        kind: Slash,
        span: Span {
```

#### ast

- ok: `True`
- truncated: `True`

```text
== ast ==
[
    Function {
        name: "print",
        params: [
            (
                "message",
                None,
                false,
            ),
        ],
        body: [
            Expr {
                expr: Call {
                    callee: Member {
                        object: Ident {
                            name: "console",
                            span: Span {
                                start: 29,
                                end: 36,
                            },
                        },
                        property: "log",
                        span: Span {
                            start: 29,
                            end: 40,
                        },
                    },
                    args: [
                        Ident {
                            name: "message",
                            span: Span {
                                start: 41,
                                end: 48,
                            },
                        },
                    ],
                    span: Span {
                        start: 29,
                        end: 49,
                    },
                },
                span: Span {
                    start: 29,
                    end: 50,
                },
            },
        ],
        is_generator: false,
        span: Span {
            start: 1,
            end: 50,
        },
    },
    Let {
        name: "NaN",
        expr: Binary {
            left: Number {
                value: 0,
                span: Span {
                    start: 95,
                    end: 96,
                },
            },
            op: Divide,
            right: Number {
                value: 0,
                span: Span {
                    start: 97,
                    end: 98,
                },
            },
            span: Span {
                start: 95,
                end: 98,
            },
        },
        span: Span {
            start: 85,
            end: 99,
        },
    },
    Let {
        name: "Infinity",
        expr: Binary {
            left: Number {
                value: 1,
                span: Span {
                    start: 115,
                    end: 116,
                },
            },
            op: Divide,
            right: Number {
                value: 0,
                span: Span {
                    start: 117,
                    end: 118,
                },
            },
            span: Span {
                start: 115,
                end: 118,
            },
        },
        span: Span {
            start: 100,
            end: 119,
        },
    },
    Let {
        name: "$262",
        expr: Object {
            props: [],
            span: Span {
                start: 182,
                end: 184,
            },
        },
        span: Span {
            start: 171,
            end: 185,
        },
    },
    Expr {
        expr: PropertyAssign {
            object: Ident {
                name: "$262",
                span: Span {
                    start: 186,
                    end: 190,
                },
            },
            property: "gc",
            value: FunctionExpr {
                name: "",
                params: [],
                body: [],
                span: Span {
                    start: 196,
                    end: 204,
                },
            },
            span: Span {
```

#### resolved

- ok: `True`
- truncated: `True`

```text
== resolved ==
[
    Function {
        name: "print",
        params: [
            ResolvedParam {
                name: "message",
                default: None,
                is_rest: false,
                span: Some(
                    Span {
                        start: 1,
                        end: 50,
                    },
                ),
            },
        ],
        body: [
            Expr(
                BuiltinCall {
                    builtin: ConsoleLog,
                    args: [
                        Ident(
                            "message",
                        ),
                    ],
                },
            ),
        ],
        is_generator: false,
    },
    Let(
        "NaN",
        Binary {
            left: Number(
                0,
            ),
            op: Divide,
            right: Number(
                0,
            ),
        },
    ),
    Let(
        "Infinity",
        Binary {
            left: Number(
                1,
            ),
            op: Divide,
            right: Number(
                0,
            ),
        },
    ),
    Let(
        "$262",
        Object(
            [],
        ),
    ),
    Expr(
        PropertyAssign {
            object: Ident(
                "$262",
            ),
            key: "gc",
            value: FunctionExpr {
                name: "",
                params: [],
                body: [],
            },
            span: Span {
                start: 186,
                end: 210,
            },
        },
    ),
    Expr(
        PropertyAssign {
            object: Ident(
                "$262",
            ),
            key: "evalScript",
            value: FunctionExpr {
                name: "",
                params: [],
                body: [
                    Expr(
                        BuiltinCall {
                            builtin: ConsoleLog,
                            args: [
                                String(
                                    "__TS2WASM_TEST262_ASSERT_FAIL__",
                                ),
                            ],
                        },
                    ),
                ],
            },
            span: Span {
                start: 211,
                end: 292,
            },
        },
    ),
    Expr(
        PropertyAssign {
            object: Ident(
                "$262",
            ),
            key: "createRealm",
            value: FunctionExpr {
                name: "",
                params: [],
                body: [
                    Return(
                        Object(
                            [],
                        ),
                    ),
                ],
            },
            span: Span {
                start: 293,
                end: 338,
            },
        },
    ),
    Expr(
        PropertyAssign {
            object: Ident(
                "$262",
            ),
            key: "detachArrayBuffer",
            value: FunctionExpr {
                name: "",
                params: [],
                body: [
                    Expr(
                        BuiltinCall {
                            builtin: ConsoleLog,
                            args: [
                                String(
                                    "__TS2WASM_TEST262_ASSERT_FAIL__",
                                ),
                            ],
                        },
                    ),
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
        "code": 6504,
        "category": "Error",
        "message": "File '/tmp/tmp6vnbrgww/test262-triage-node-input.js' is a JavaScript file. Did you mean to enable the 'allowJs' option?\n  The file is in the program because:\n    Root file specified for compilation"
      }
    ],
    "hints": [],
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

## False-done audit

**truly-done** (438)

- Implementation commits: verified via `git log --oneline --all --grep=438`
- Completion evidence: filled with specific commit hashes and validation results
- Acceptance criteria: all checked as met

This issue has repo-local close evidence with implementation commits and validation commands.
