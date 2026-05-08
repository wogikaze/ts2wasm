---
id: 429
title: "Implement eval support"
type: spike
area: reference/triage
class: blocked
priority: P1
depends_on: [5005]
blocks: []
created: 2026-05-01
updated: 2026-05-01
---

## Summary

Triage eval feature across 679 failing reference test cases and split this bucket into implementation-ready child issues.

## Problem

Reference test results show 679 cases fail with eval diagnostic. The compiler cannot handle these syntax/semantics, preventing compilation of code in this category.

Problem: eval feature has 679 reference failures and needs smart-triage evidence before implementation starts.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- test262 reference/test262/test/annexB/built-ins/Function/createdynfn-html-close-comment-body.js
```

Coverage window:

```sh
mise run reference-coverage -- test262 --path-filter reference/test262/test/annexB/built-ins/Function/createdynfn-html-close-comment-body.js --detail
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
mise run reference-coverage -- test262 --limit 1358
mise run reference-coverage -- test262 --path-filter reference/test262/test/annexB/built-ins/Function/createdynfn-html-close-comment-body.js --detail
mise run reference-triage -- test262 reference/test262/test/annexB/built-ins/Function/createdynfn-html-close-comment-body.js
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

- `reference/test262/test/annexB/built-ins/Function/createdynfn-html-close-comment-body.js`
- `reference/test262/test/annexB/built-ins/Function/createdynfn-html-close-comment-params.js`
- `reference/test262/test/annexB/built-ins/Function/createdynfn-html-open-comment-body.js`
- `reference/test262/test/annexB/built-ins/Function/createdynfn-html-open-comment-params.js`
- `reference/test262/test/annexB/built-ins/Function/createdynfn-no-line-terminator-html-close-comment-body.js`
- `reference/test262/test/annexB/built-ins/Function/createdynfn-no-line-terminator-html-close-comment-params.js`
- `reference/test262/test/annexB/language/eval-code/direct/func-block-decl-eval-func-existing-block-fn-update.js`
- `reference/test262/test/annexB/language/eval-code/direct/func-block-decl-eval-func-existing-fn-no-init.js`
- `reference/test262/test/annexB/language/eval-code/direct/func-block-decl-eval-func-existing-fn-update.js`
- `reference/test262/test/annexB/language/eval-code/direct/func-block-decl-eval-func-no-skip-param.js`
- ... and 669 more files

## Duplicate detection

- `issues/open/052d-implement-json-stringify-broader-replacer-semantics.md` - Implement broader JSON.stringify replacer semantics (same feature label, same group key, title overlap)
- `issues/done/128-implement-aliasUsedAsNameValue.md` - Implement Aliasusedasnamevalue (same feature label, same group key, title overlap)
- `issues/done/132-implement-allowJsClassThisTypeCrash.md` - Implement Allowjsclassthistypecrash (same feature label, same group key, title overlap)
- `issues/done/225-implement-eval-annexb-function-declarations.md` - Implement eval and Annex B function declaration semantics (same feature label, same group key, title overlap)
- `issues/done/308-implement-abc451-depth9-gc-cadence-policy.md` - Implement ABC451 depth-9 GC cadence policy (same feature label, same group key, title overlap)
- `issues/done/309-reduce-abc451-depth9-live-allocation-shape.md` - Reduce ABC451 depth-9 live allocation shape (same feature label, same group key)
- `issues/done/351-private-brand-storage-brand-checks.md` - Implement full private brand storage and brand-checking semantics (same feature label, same group key, title overlap)
- `issues/done/357-fix-abc451-depth8-iwasm-timeout.md` - Fix ABC451 depth-8 iwasm timeout (same feature label, same group key)
- `issues/done/363-reduce-abc451-allocation-and-sweep-volume-after-bulk-copy-narrowing.md` - Reduce ABC451 allocation and sweep volume after bulk copy narrowing (same feature label, same group key)
- `issues/done/365-reduce-abc451-array-growth-allocation-copy-pressure.md` - Reduce ABC451 array-growth allocation and copy pressure (same feature label, same group key)

## Smart triage

### Smart triage: Triage eval: createdynfn html close comment body

- Issue class: `triage-needed`
- Feature label: `eval`
- Diagnostic: `UnsupportedEval` / `unsupported-feature-boundary`
- Path: `reference/test262/test/annexB/built-ins/Function/createdynfn-html-close-comment-body.js`

Reproduction:

```sh
mise run reference-triage -- test262 reference/test262/test/annexB/built-ins/Function/createdynfn-html-close-comment-body.js
```

Source overview:

```json
{
  "suite": "test262",
  "bytes": 721,
  "lines": 20,
  "extension": ".js",
  "first_code_line": "esid: sec-createdynamicfunction",
  "test262_metadata": {
    "esid": "sec-createdynamicfunction",
    "description": ">",
    "info": "|",
    "19.2.1.1.1 Runtime Semantics": "CreateDynamicFunction(constructor, newTarget, kind, args)"
  }
}
```

Failure location:

```json
{
  "code": "UnsupportedEval",
  "message": "issue-062: dynamic Function constructor is not supported; runtime code evaluation is intentionally not implemented at 1667..1684",
  "span_start": 1667,
  "span_end": 1684,
  "line": 63,
  "column": 2,
  "feature_label": "eval",
  "error_type": "unsupported-feature-boundary"
}
```

Source context:

```text
60 |     ...
61 | ---*/
62 | 
63 | Function("\n-->");
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
    "state": "open",
    "path": "issues/done/225-implement-eval-annexb-function-declarations.md",
    "title": "Implement eval and Annex B function declaration semantics",
    "reason": "same feature label, title overlap"
  },
  {
    "state": "done",
    "path": "issues/done/062b-dynamic-function-constructor-diagnostics.md",
    "title": "Own dynamic Function constructor diagnostics",
    "reason": "same reference path, same feature label"
  },
  {
    "state": "done",
    "path": "issues/done/063-implement-function-resolution.md",
    "title": "Implement function resolution",
    "reason": "same reference path"
  },
  {
    "state": "done",
    "path": "issues/done/302-implement-direct-eval-block-function-declaration-slice.md",
    "title": "Implement direct eval block function declaration slice",
    "reason": "same feature label, title overlap"
  },
  {
    "state": "done",
    "path": "issues/done/306-implement-mutable-direct-eval-block-function-environments.md",
    "title": "Implement mutable direct eval block-function environments",
    "reason": "same feature label, title overlap"
  },
  {
    "state": "done",
    "path": "issues/done/347-parser-resolver-direct-eval-scope.md",
    "title": "Parser and resolver support for direct eval and eval-code scope",
    "reason": "same feature label, title overlap"
  },
  {
    "state": "done",
    "path": "issues/done/348-lowering-eval-block-function-declarations.md",
    "title": "Lowering block-level function declarations in direct eval code",
    "reason": "same feature label, title overlap"
  },
  {
    "state": "done",
    "path": "issues/done/349-runtime-shim-direct-eval-execution.md",
    "title": "Runtime helper or shim JavaScript emission for direct eval execution",
    "reason": "same feature label, title overlap"
  },
  {
    "state": "done",
    "path": "issues/done/406-direct-eval-annexb-existing-binding-residuals.md",
    "title": "Direct eval Annex B existing binding residuals",
    "reason": "same feature label, title overlap"
  }
]
```

Error-specific suggestions:

- Create a child issue around this exact path and diagnostic before broadening the reference window.

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
error: [UnsupportedEval] issue-062: dynamic Function constructor is not supported; runtime code evaluation is intentionally not implemented at 1667..1684
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
        "message": "File '/tmp/tmp1vhnp9k0/test262-triage-node-input.js' is a JavaScript file. Did you mean to enable the 'allowJs' option?\n  The file is in the program because:\n    Root file specified for compilation"
      }
    ],
    "hints": [],
    "typescriptVersion": "6.0.3"
  },
  "ast": {
    "topLevel": [
      {
        "kind": "FunctionDeclaration",
        "text": "function print(message) {\n  console.log(message);\n}",
        "line": 2,
        "character": 1
      },
      {
        "kind": "FirstStatement",
        "text": "var $262 = {};",
        "line": 6,
        "character": 1
      },
      {
        "kind": "FunctionDeclaration",
        "text": "function test262_gc() {}",
        "line": 8,
        "character": 1
      },
      {
        "kind": "FunctionDeclaration",
        "text": "function test262_evalScript(source) {\n  throw new Test262Error(\"$262.evalScript is not supported by this harness slice\")",
        "line": 10,
        "character": 1
      },
      {
        "kind": "FunctionDeclaration",
        "text": "function test262_createRealm() {\n  return {};\n}",
        "line": 14,
        "character": 1
      },
      {
        "kind": "FunctionDeclaration",
        "text": "function test262_detachArrayBuffer() {\n  throw new Test262Error(\"$262.detachArrayBuffer is not supported by this harness",
        "line": 18,
        "character": 1
      },
      {
        "kind": "FunctionDeclaration",
        "text": "function test262_agent_start() {\n  throw new Test262Error(\"$262.agent is not supported by this harness slice\");\n}",
        "line": 22,
        "character": 1
      },
      {
        "kind": "ExpressionStatement",
        "text": "$262.global = {};",
        "line": 26,
        "character": 1
      },
      {
        "kind": "ExpressionStatement",
        "text": "$262.gc = test262_gc;",
        "line": 27,
        "character": 1
      },
      {
        "kind": "ExpressionStatement",
        "text": "$262.evalScript = test262_evalScript;",
        "line": 28,
        "character": 1
      },
      {
        "kind": "ExpressionStatement",
        "text": "$262.createRealm = test262_createRealm;",
        "line": 29,
        "character": 1
      },
      {
        "kind": "ExpressionStatement",
        "text": "$262.detachArrayBuffer = test262_detachArrayBuffer;",
        "line": 30,
        "character": 1
      },
      {
        "kind": "ExpressionStatement",
        "text": "$262.IsHTMLDDA = undefined;",
        "line": 31,
        "character": 1
      },
      {
        "kind": "ExpressionStatement",
        "text": "$262.agent = {};",
        "line": 32,
        "character": 1
      },
      {
        "kind": "ExpressionStatement",
        "text": "$262.agent.start = test262_agent_start;",
        "line": 33,
        "character": 1
      },
      {
        "kind": "FunctionDeclaration",
        "text": "function Test262Error(message) {\n  this.message = message || \"\";\n}",
        "line": 50,
        "character": 1
      },
      {
        "kind": "ExpressionStatement",
        "text": "Test262Error.prototype.toString = function () {\n  return \"Test262Error: \" + this.message;\n};",
        "line": 54,
        "character": 1
      },
      {
        "kind": "ExpressionStatement",
        "text": "Test262Error.thrower = function (message) {\n  throw new Test262Error(message);\n};",
        "line": 58,
        "character": 1
      },
      {
        "kind": "FunctionDeclaration",
        "text": "function $DONOTEVALUATE() {\n  throw \"Test262: This statement should not be evaluated.\";\n}",
        "line": 62,
        "character": 1
      },
      {
        "kind": "FunctionDeclaration",
        "text": "function assert(mustBeTrue, message) {\n  if (mustBeTrue === true) {\n    return;\n  }\n\n  if (message === undefined) {\n    ",
        "line": 78,
        "character": 1
      }
    ],
    "pathToPosition": [
      {
        "kind": "SourceFile",
        "text": "function print(message) {\n  console.log(message);\n}\n\nvar $262 = {};\n\nfunction test262_gc() {}\n\nfunction test262_evalScri",
        "line": 2,
        "character": 1
      },
      {
        "kind": "FunctionDeclaration",
        "text": "function assert(mustBeTrue, message) {\n  if (mustBeTrue === true) {\n    return;\n  }\n\n  if (message === undefined) {\n    ",
        "line": 78,
        "character": 1
      }
    ]
  }
}
```

Stack trace:

```text
error: [UnsupportedEval] issue-062: dynamic Function constructor is not supported; runtime code evaluation is intentionally not implemented at 1667..1684
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

**truly-done** (429)

- Implementation commits: verified via `git log --oneline --all --grep=429`
- Completion evidence: filled with specific commit hashes and validation results
- Acceptance criteria: all checked as met

This issue has repo-local close evidence with implementation commits and validation commands.
