---
id: 430
title: "Implement function support (dup)"
type: spike
area: frontend/syntax
class: superseded
priority: P2
depends_on: []
blocks: []
created: 2026-05-01
updated: 2026-05-04
---

## Summary

Triage function feature across 287 failing reference test cases and split this bucket into implementation-ready child issues.

## Problem

Reference test results show 287 cases fail with function diagnostic. The compiler cannot handle these syntax/semantics, preventing compilation of code in this category.

Problem: function feature has 287 reference failures and needs smart-triage evidence before implementation starts.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- test262 reference/test262/test/annexB/language/function-code/if-decl-else-decl-a-func-existing-var-no-init.js
```

Coverage window:

```sh
mise run reference-coverage -- test262 --path-filter reference/test262/test/annexB/language/function-code/if-decl-else-decl-a-func-existing-var-no-init.js --detail
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
mise run reference-coverage -- test262 --limit 574
mise run reference-coverage -- test262 --path-filter reference/test262/test/annexB/language/function-code/if-decl-else-decl-a-func-existing-var-no-init.js --detail
mise run reference-triage -- test262 reference/test262/test/annexB/language/function-code/if-decl-else-decl-a-func-existing-var-no-init.js
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

- `reference/test262/test/annexB/language/function-code/if-decl-else-decl-a-func-existing-var-no-init.js`
- `reference/test262/test/annexB/language/function-code/if-decl-else-decl-a-func-skip-early-err.js`
- `reference/test262/test/annexB/language/function-code/if-decl-else-decl-a-func-skip-param.js`
- `reference/test262/test/annexB/language/function-code/if-decl-else-decl-b-func-existing-var-no-init.js`
- `reference/test262/test/annexB/language/function-code/if-decl-else-decl-b-func-skip-early-err.js`
- `reference/test262/test/annexB/language/function-code/if-decl-else-decl-b-func-skip-param.js`
- `reference/test262/test/annexB/language/function-code/if-decl-no-else-func-existing-var-no-init.js`
- `reference/test262/test/annexB/language/function-code/if-decl-no-else-func-skip-early-err.js`
- `reference/test262/test/annexB/language/function-code/if-decl-no-else-func-skip-param.js`
- `reference/test262/test/annexB/language/function-code/switch-case-func-existing-var-no-init.js`
- ... and 277 more files

## Duplicate detection

- `issues/done/017b-implement-gc-strategy.md` - issues/done/017b-implement-gc-strategy.md (same feature label, same group key)
- `issues/open/021-implement-full-wasm-backend.md` - issues/open/021-implement-full-wasm-backend.md (same feature label, same group key)
- `issues/done/052-implement-json.md` - Implement JSON (same feature label, same group key, title overlap)
- `issues/open/052d-implement-json-stringify-broader-replacer-semantics.md` - Implement broader JSON.stringify replacer semantics (same feature label, same group key, title overlap)
- `issues/open/064-implement-name-resolution.md` - Implement name resolution (triaged - superseded by test262 metadata issues) (same feature label, same group key, title overlap)
- `issues/done/067-implement-unknown-unsupported.md` - Investigate and classify unknown-unsupported cases (same feature label, same group key)
- `issues/done/070-implement-APISample.md` - Implement Apisample (same feature label, same group key, title overlap)
- `issues/done/071-implement-ArrowFunctionExpression.md` - Implement Arrowfunctionexpression (same feature label, same group key, title overlap)
- `issues/done/076-implement-FunctionDeclaration.md` - Implement Functiondeclaration (same feature label, same group key, title overlap)
- `issues/done/079-implement-ParameterList.md` - Implement Parameterlist (same feature label, same group key, title overlap)

## Smart triage

### Smart triage: Triage function: if decl else decl a func existing var no init

- Issue class: `triage-needed`
- Feature label: `function`
- Diagnostic: `UnsupportedSyntax` / `parser-or-frontend-unsupported`
- Path: `reference/test262/test/annexB/language/function-code/if-decl-else-decl-a-func-existing-var-no-init.js`

Reproduction:

```sh
mise run reference-triage -- test262 reference/test262/test/annexB/language/function-code/if-decl-else-decl-a-func-existing-var-no-init.js
```

Source overview:

```json
{
  "suite": "test262",
  "bytes": 1159,
  "lines": 35,
  "extension": ".js",
  "first_code_line": "description: Existing variable binding is not modified (IfStatement with a declaration in both statement positions in function scope)",
  "test262_metadata": {
    "description": "Existing variable binding is not modified (IfStatement with a declaration in both statement positions in function scope)",
    "esid": "sec-functiondeclarations-in-ifstatement-statement-clauses",
    "flags": "[generated, noStrict]",
    "info": "|",
    "The following rules for IfStatement augment those in 13.6": "",
    "IfStatement[Yield, Return]": ""
  }
}
```

Failure location:

```json
{
  "code": "UnsupportedSyntax",
  "message": "issue-062e: nested function `` mutates a captured outer local; mutable closure environments require heap environment support",
  "span_start": null,
  "span_end": null,
  "line": null,
  "column": null,
  "feature_label": "function",
  "error_type": "parser-or-frontend-unsupported"
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
  },
  {
    "kind": "binding",
    "name": "init",
    "line": 67,
    "column": 1
  },
  {
    "kind": "binding",
    "name": "f",
    "line": 70,
    "column": 3,
    "initializer": "123"
  },
  {
    "kind": "function",
    "name": "f",
    "line": 73,
    "column": 13,
    "params": ""
  },
  {
    "kind": "function",
    "name": "_f",
    "line": 73,
    "column": 36,
    "params": ""
  }
]
```

Duplicate candidates:

```json
[
  {
    "state": "open",
    "path": "issues/done/225-implement-eval-annexb-function-declarations.md",
    "title": "Implement eval and Annex B function declaration semantics",
    "reason": "same feature label, title overlap"
  },
  {
    "state": "done",
    "path": "issues/done/036-implement-arrow-function.md",
    "title": "Implement arrow function",
    "reason": "same feature label, title overlap"
  },
  {
    "state": "done",
    "path": "issues/done/052g-implement-json-stringify-function-replacer-callbacks.md",
    "title": "Implement JSON.stringify function replacer callbacks",
    "reason": "same feature label, title overlap"
  },
  {
    "state": "done",
    "path": "issues/done/057-implement-function-resolution.md",
    "title": "Implement function resolution for function calls",
    "reason": "same feature label, title overlap"
  },
  {
    "state": "done",
    "path": "issues/done/062-implement-function.md",
    "title": "Implement function support",
    "reason": "same feature label, title overlap"
  },
  {
    "state": "done",
    "path": "issues/done/062a-split-function-epic-into-callable-child-issues.md",
    "title": "Split function epic into callable child issues",
    "reason": "same feature label, title overlap"
  },
  {
    "state": "done",
    "path": "issues/done/062b-dynamic-function-constructor-diagnostics.md",
    "title": "Own dynamic Function constructor diagnostics",
    "reason": "same feature label, title overlap"
  },
  {
    "state": "done",
    "path": "issues/done/062c-ordinary-function-declarations-and-calls.md",
    "title": "Implement ordinary function declarations and direct calls",
    "reason": "same feature label, title overlap"
  },
  {
    "state": "done",
    "path": "issues/done/062d-function-this-and-arguments.md",
    "title": "Implement function this and arguments semantics",
    "reason": "same feature label, title overlap"
  },
  {
    "state": "done",
    "path": "issues/done/062e-function-closures.md",
    "title": "Implement function closures",
    "reason": "same feature label, title overlap"
  }
]
```

Error-specific suggestions:

- Start at lexer/parser support and add a minimal fixture for the exact source construct at the failing span.
- Use `dump --tokens` and the TypeScript AST path to decide whether this is tokenization, precedence, or statement dispatch.
- Classify this as runtime/API work unless the parser fails before builtin resolution.

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

- ok: `False`
- truncated: `False`

```text
error: [UnsupportedSyntax] issue-062e: nested function `` mutates a captured outer local; mutable closure environments require heap environment support
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
        "message": "File '/tmp/tmpn47y31nx/test262-triage-node-input.js' is a JavaScript file. Did you mean to enable the 'allowJs' option?\n  The file is in the program because:\n    Root file specified for compilation"
      }
    ],
    "hints": [],
    "typescriptVersion": "6.0.3"
  }
}
```


## 重複整理メモ

この issue はタイトルが重複しているため、内容がより充実している
`issues/open/5015-implement-function.md` に統合されました。
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
- `issues/done/430-implement-function.md`
- `issues/index.md` after regeneration

Future-work tracking: no untracked future-work item was identified in this issue during this metadata/evidence audit.
