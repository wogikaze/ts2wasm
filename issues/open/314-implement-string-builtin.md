---
id: 314
title: "Implement string-builtin support"
type: feature
area: runtime/builtins
class: superseded
priority: P1
depends_on: [5004]
blocks: []
created: 2026-04-30
updated: 2026-05-04
---

## Summary

Implement String builtin method support for runtime (test262 coverage). Consolidated from generated fixture-bucket issues.

## Problem

Reference test results show 10 cases fail with string-builtin diagnostic. The compiler cannot handle these syntax/semantics, preventing compilation of code in this category.

Problem: string-builtin feature has 10 reference failures and needs smart-triage evidence before implementation starts.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- test262 reference/test262/test/built-ins/String/fromCharCode/S15.5.3.2_A2.js
```

Coverage window:

```sh
mise run reference-coverage -- test262 --path-filter reference/test262/test/built-ins/String/fromCharCode/S15.5.3.2_A2.js --detail
```

## Desired final state

This generated bucket is either split into implementation-ready child issues or superseded by an existing open/done issue with matching evidence. Do not implement directly from this bucket.

## Scope

This issue tracks String builtin API implementation. Generated child fixture-bucket issues (3140-3149) have been consolidated back into this parent and archived (now in `issues/open/`).

In scope:
- [x] Implement String.prototype methods and String constructor features
- [x] Add Node/iwasm differential fixture coverage for supported String methods
- [x] Reduce test262 `string-builtin` unsupported count

Out of scope:
- Array builtins (tracked by issue 313)
- Object builtins (tracked by issue 342)
- keyof/keyword/lambda residual fixture buckets (originally misclassified under 314 prefix)

## Affected paths

Expected:

- `crates/backend-wasm/src/`
- `crates/runtime-abi/src/`
- `crates/cli/src/`
- `fixtures/`
- `scripts/run/reference-triage.py`

Do not touch:

- parser/resolver code unless `reference-triage` proves the failure happens before runtime lowering

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
mise run reference-coverage -- test262 --limit 20
mise run reference-coverage -- test262 --path-filter reference/test262/test/built-ins/String/fromCharCode/S15.5.3.2_A2.js --detail
mise run reference-triage -- test262 reference/test262/test/built-ins/String/fromCharCode/S15.5.3.2_A2.js
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

- `reference/test262/test/built-ins/String/fromCharCode/S15.5.3.2_A2.js`
- `reference/test262/test/built-ins/String/fromCharCode/S15.5.3.2_A3_T1.js`
- `reference/test262/test/built-ins/String/prototype/charAt/S15.5.4.4_A1_T4.js`
- `reference/test262/test/built-ins/String/prototype/charCodeAt/S15.5.4.5_A1_T4.js`
- `reference/test262/test/built-ins/String/prototype/concat/S15.5.4.6_A1_T4.js`
- `reference/test262/test/built-ins/String/prototype/indexOf/S15.5.4.7_A1_T4.js`
- `reference/test262/test/built-ins/String/prototype/indexOf/S15.5.4.7_A2_T2.js`
- `reference/test262/test/built-ins/String/prototype/indexOf/S15.5.4.7_A2_T3.js`
- `reference/test262/test/built-ins/String/prototype/slice/S15.5.4.13_A1_T14.js`
- `reference/test262/test/built-ins/String/prototype/substring/S15.5.4.15_A1_T14.js`

## Duplicate detection

- `issues/open/067-implement-unknown-unsupported.md` - Investigate and classify unknown-unsupported cases (same feature label, same group key)
- `issues/open/060-investigate-unknown-unsupported-cases.md` - Investigate and classify unknown-unsupported diagnostic cases (same feature label, same group key)
- `issues/open/224-implement-annexb-html-comments.md` - Implement Annex B HTML-like comments (same feature label, same group key, title overlap)
- `issues/done/228-implement-logical-assignment-operators.md` - Implement logical assignment operators (same feature label, same group key, title overlap)

## Smart triage

### Smart triage: Triage name resolution: S15.5.3.2 A2

- Issue class: `triage-needed`
- Feature label: `name-resolution`
- Diagnostic: `UnresolvedName` / `resolver-symbol`
- Path: `reference/test262/test/built-ins/String/fromCharCode/S15.5.3.2_A2.js`

Reproduction:

```sh
mise run reference-triage -- test262 reference/test262/test/built-ins/String/fromCharCode/S15.5.3.2_A2.js
```

Source overview:

```json
{
  "suite": "test262",
  "bytes": 575,
  "lines": 16,
  "extension": ".js",
  "first_code_line": "info: String.fromCharCode () returns empty string",
  "test262_metadata": {
    "info": "String.fromCharCode () returns empty string",
    "es5id": "15.5.3.2_A2",
    "description": "Call String.fromCharCode()"
  }
}
```

Failure location:

```json
{
  "code": "UnresolvedName",
  "message": "unresolved name: `Test262Error` at 392..404",
  "span_start": 392,
  "span_end": 404,
  "line": 13,
  "column": 13,
  "feature_label": "name-resolution",
  "error_type": "resolver-symbol"
}
```

Source context:

```text
10 | //////////////////////////////////////////////////////////////////////////////
11 | //CHECK#1
12 | if (String.fromCharCode() !== "") {
13 |   throw new Test262Error('#1: String.fromCharCode () returns empty string. Actual: ' + String.fromCharCode());
14 | }
15 | //
16 | //////////////////////////////////////////////////////////////////////////////
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
        kind: If,
        span: Span {
            start: 344,
            end: 346,
        },
    },
    SpannedToken {
        kind: LeftParen,
        span: Span {
            start: 347,
            end: 348,
        },
    },
    SpannedToken {
        kind: Ident(
            "String",
        ),
        span: Span {
            start: 348,
            end: 354,
        },
    },
    SpannedToken {
        kind: Dot,
        span: Span {
            start: 354,
            end: 355,
        },
    },
    SpannedToken {
        kind: Ident(
            "fromCharCode",
        ),
        span: Span {
            start: 355,
            end: 367,
        },
    },
    SpannedToken {
        kind: LeftParen,
        span: Span {
            start: 367,
            end: 368,
        },
    },
    SpannedToken {
        kind: RightParen,
        span: Span {
            start: 368,
            end: 369,
        },
    },
    SpannedToken {
        kind: StrictNotEqual,
        span: Span {
            start: 370,
            end: 373,
        },
    },
    SpannedToken {
        kind: String(
            "",
        ),
        span: Span {
            start: 374,
            end: 376,
        },
    },
    SpannedToken {
        kind: RightParen,
        span: Span {
            start: 376,
            end: 377,
        },
    },
    SpannedToken {
        kind: LeftBrace,
        span: Span {
            start: 378,
            end: 379,
        },
    },
    SpannedToken {
        kind: Throw,
        span: Span {
            start: 382,
            end: 387,
        },
    },
    SpannedToken {
        kind: New,
        span: Span {
            start: 388,
            end: 391,
        },
    },
    SpannedToken {
        kind: Ident(
            "Test262Error",
        ),
        span: Span {
            start: 392,
            end: 404,
        },
    },
    SpannedToken {
        kind: LeftParen,
        span: Span {
            start: 404,
            end: 405,
        },
    },
    SpannedToken {
        kind: String(
            "#1: String.fromCharCode () returns empty string. Actual: ",
        ),
        span: Span {
            start: 405,
            end: 464,
        },
    },
    SpannedToken {
        kind: Plus,
        span: Span {
            start: 465,
            end: 466,
        },
    },
    SpannedToken {
        kind: Ident(
            "String",
        ),
        span: Span {
            start: 467,
            end: 473,
        },
    },
    SpannedToken {
        kind: Dot,
        span: Span {
            start: 473,
            end: 474,
        },
    },
    SpannedToken {
        kind: Ident(
            "fromCharCode",
        ),
        span: Span {
            start: 474,
            end: 486,
        },
    },
    SpannedToken {
        kind: LeftParen,
        span: Span {
            start: 486,
            end: 487,
        },
    },
    SpannedToken {
        kind: RightParen,
        span: Span {
            start: 487,
            end: 488,
        },
    },
    SpannedToken {
        kind: RightParen,
        span: Span {
            start: 488,
            end: 489,
        },
    },
    SpannedToken {
        kind: Semicolon,
        span: Span {
            start: 489,
            end: 490,
        },
    },
    SpannedToken {
        kind: RightBrace,
        span: Span {
            start: 491,
            end: 492,
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
    If {
        condition: Binary {
            left: Call {
                callee: Member {
                    object: Ident {
                        name: "String",
                        span: Span {
                            start: 348,
                            end: 354,
                        },
                    },
                    property: "fromCharCode",
                    span: Span {
                        start: 348,
                        end: 367,
                    },
                },
                args: [],
                span: Span {
                    start: 348,
                    end: 369,
                },
            },
            op: StrictNotEqual,
            right: String {
                value: "",
                span: Span {
                    start: 374,
                    end: 376,
                },
            },
            span: Span {
                start: 348,
                end: 376,
            },
        },
        then_body: [
            Throw {
                expr: New {
                    expr: Ident {
                        name: "Test262Error",
                        span: Span {
                            start: 392,
                            end: 404,
                        },
                    },
                    args: [
                        Binary {
                            left: String {
                                value: "#1: String.fromCharCode () returns empty string. Actual: ",
                                span: Span {
                                    start: 405,
                                    end: 464,
                                },
                            },
                            op: Add,
                            right: Call {
                                callee: Member {
                                    object: Ident {
                                        name: "String",
                                        span: Span {
                                            start: 467,
                                            end: 473,
                                        },
                                    },
                                    property: "fromCharCode",
                                    span: Span {
                                        start: 467,
                                        end: 486,
                                    },
                                },
                                args: [],
                                span: Span {
                                    start: 467,
                                    end: 488,
                                },
                            },
                            span: Span {
                                start: 405,
                                end: 488,
                            },
                        },
                    ],
                    span: Span {
                        start: 388,
                        end: 489,
                    },
                },
                span: Span {
                    start: 382,
                    end: 490,
                },
            },
        ],
        else_body: [],
        span: Span {
            start: 344,
            end: 490,
        },
    },
]
```

#### resolved

- ok: `False`
- truncated: `False`

```text
error: [UnresolvedName] unresolved name: `Test262Error` at 392..404
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
        "message": "File '/home/wogikaze/wgkz/ts2wasm/reference/test262/test/built-ins/String/fromCharCode/S15.5.3.2_A2.js' is a JavaScript file. Did you mean to enable the 'allowJs' option?\n  The file is in the program because:\n    Root file specified for compilation"
      }
    ],
    "hints": [],
    "typescriptVersion": "6.0.3"
  },
  "ast": {
    "topLevel": [
      {
        "kind": "IfStatement",
        "text": "if (String.fromCharCode() !== \"\") {\n  throw new Test262Error('#1: String.fromCharCode () returns empty string. Actual: '",
        "line": 12,
        "character": 1
      }
    ],
    "pathToPosition": [
      {
        "kind": "SourceFile",
        "text": "if (String.fromCharCode() !== \"\") {\n  throw new Test262Error('#1: String.fromCharCode () returns empty string. Actual: '",
        "line": 12,
        "character": 1
      },
      {
        "kind": "IfStatement",
        "text": "if (String.fromCharCode() !== \"\") {\n  throw new Test262Error('#1: String.fromCharCode () returns empty string. Actual: '",
        "line": 12,
        "character": 1
      },
      {
        "kind": "Block",
        "text": "{\n  throw new Test262Error('#1: String.fromCharCode () returns empty string. Actual: ' + String.fromCharCode());\n}",
        "line": 12,
        "character": 35
      },
      {
        "kind": "ThrowStatement",
        "text": "throw new Test262Error('#1: String.fromCharCode () returns empty string. Actual: ' + String.fromCharCode());",
        "line": 13,
        "character": 3
      },
      {
        "kind": "NewExpression",
        "text": "new Test262Error('#1: String.fromCharCode () returns empty string. Actual: ' + String.fromCharCode())",
        "line": 13,
        "character": 9
      },
      {
        "kind": "Identifier",
        "text": "Test262Error",
        "line": 13,
        "character": 13
      }
    ]
  }
}
```

Stack trace:

```text
error: [UnresolvedName] unresolved name: `Test262Error` at 392..404
```


## 重複整理メモ

この issue はタイトルが重複しているため、内容がより充実している
`issues/done/5021-implement-string-builtin.md` に統合されました。
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
- `issues/open/314-implement-string-builtin.md`
- `issues/index.md` after regeneration

Future-work tracking: no untracked future-work item was identified in this issue during this metadata/evidence audit.
