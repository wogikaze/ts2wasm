---
id: 5014
title: "Implement eval support (dup)"
type: spike
area: frontend/syntax
class: superseded
priority: P1
depends_on: []
blocks: []
created: 2026-05-03
updated: 2026-05-04
---

## Summary

Triage eval feature across 161 failing reference test cases and split this bucket into implementation-ready child issues.

## Problem

Reference test results show 161 cases fail with eval diagnostic. The compiler cannot handle these syntax/semantics, preventing compilation of code in this category.

Problem: eval feature has 161 reference failures and needs smart-triage evidence before implementation starts.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- test262 reference/test262/test/annexB/language/eval-code/direct/func-block-decl-eval-func-block-scoping.js
```

Coverage window:

```sh
mise run reference-coverage -- test262 --path-filter reference/test262/test/annexB/language/eval-code/direct/func-block-decl-eval-func-block-scoping.js --detail
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
mise run reference-coverage -- test262 --limit 322
mise run reference-coverage -- test262 --path-filter reference/test262/test/annexB/language/eval-code/direct/func-block-decl-eval-func-block-scoping.js --detail
mise run reference-triage -- test262 reference/test262/test/annexB/language/eval-code/direct/func-block-decl-eval-func-block-scoping.js
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

- `reference/test262/test/annexB/language/eval-code/direct/func-block-decl-eval-func-block-scoping.js`
- `reference/test262/test/annexB/language/eval-code/direct/func-block-decl-eval-func-existing-fn-no-init.js`
- `reference/test262/test/annexB/language/eval-code/direct/func-block-decl-eval-func-existing-block-fn-update.js`
- `reference/test262/test/annexB/language/eval-code/direct/func-block-decl-eval-func-existing-var-update.js`
- `reference/test262/test/annexB/language/eval-code/direct/func-block-decl-eval-func-no-skip-try.js`
- `reference/test262/test/annexB/language/eval-code/direct/func-block-decl-eval-func-init.js`
- `reference/test262/test/annexB/language/eval-code/direct/func-block-decl-eval-func-no-skip-param.js`
- `reference/test262/test/annexB/language/eval-code/direct/func-block-decl-eval-func-skip-early-err-block.js`
- `reference/test262/test/annexB/language/eval-code/direct/func-block-decl-eval-func-existing-block-fn-no-init.js`
- `reference/test262/test/annexB/language/eval-code/direct/func-block-decl-eval-func-skip-early-err.js`
- ... and 151 more files

## Duplicate detection

- `issues/done/1170-implement-class.md` - Implement Class (same feature label, same group key, title overlap; split to issue 5246)
- `issues/done/128-implement-aliasUsedAsNameValue.md` - Implement Aliasusedasnamevalue (same feature label, same group key, title overlap)
- `issues/done/132-implement-allowJsClassThisTypeCrash.md` - Implement Allowjsclassthistypecrash (same feature label, same group key, title overlap)
- `issues/open/1988-implement-discriminantUsingEvaluatableTemplateExpression.md` - Implement Discriminantusingevaluatabletemplateexpression (same feature label, same group key, title overlap)
- `issues/open/2068-implement-dynamicImportEvaluateSpecifier.md` - Implement Dynamicimportevaluatespecifier (same feature label, same group key, title overlap)
- `issues/open/2214-implement-evalAfter.md` - Implement Evalafter (same feature label, same group key, title overlap)
- `issues/open/2215-implement-evalOrArgumentsInDeclarationFunctions.md` - Implement Evalorargumentsindeclarationfunctions (same feature label, same group key, title overlap)
- `issues/open/2410-implement-for.md` - Implement For (same feature label, same group key, title overlap)
- `issues/open/2465-implement-functionType.md` - Implement Functiontype (same feature label, same group key, title overlap)
- `issues/open/308-implement-abc451-depth9-gc-cadence-policy.md` - Implement ABC451 depth-9 GC cadence policy (same feature label, same group key, title overlap)

## Smart triage

### Smart triage: Triage unknown unsupported: func block decl eval func block scoping

- Issue class: `triage-needed`
- Feature label: `unknown-unsupported`
- Diagnostic: `UnsupportedSyntax` / `parser-or-frontend-unsupported`
- Path: `reference/test262/test/annexB/language/eval-code/direct/func-block-decl-eval-func-block-scoping.js`

Reproduction:

```sh
mise run reference-triage -- test262 reference/test262/test/annexB/language/eval-code/direct/func-block-decl-eval-func-block-scoping.js
```

Source overview:

```json
{
  "suite": "test262",
  "bytes": 1536,
  "lines": 46,
  "extension": ".js",
  "first_code_line": "description: A block-scoped binding is created (Block statement in eval code containing a function declaration)",
  "test262_metadata": {
    "description": "A block-scoped binding is created (Block statement in eval code containing a function declaration)",
    "esid": "sec-web-compat-evaldeclarationinstantiation",
    "flags": "[generated, noStrict]",
    "info": "|",
    "13.2.14 Runtime Semantics": "BlockDeclarationInstantiation"
  }
}
```

Failure location:

```json
{
  "code": "UnsupportedSyntax",
  "message": "issue-292: top-level function mutation of outer binding `varBinding` requires mutable outer environment lowering at 0..15",
  "span_start": 0,
  "span_end": 15,
  "line": 1,
  "column": 1,
  "feature_label": "unknown-unsupported",
  "error_type": "parser-or-frontend-unsupported"
}
```

Source context:

```text
1 | 
2 | function print(message) {
3 |   console.log(message);
4 | }
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
    "path": "issues/open/1189-implement-classExpressionWithStaticProperties-unknown-unsupported.md",
    "title": "Implement Classexpressionwithstaticproperties Unknown Unsupported",
    "reason": "same feature label, title overlap"
  },
  {
    "state": "open",
    "path": "issues/open/1191-implement-classExpressionWithStaticPropertiesES-unknown-unsupported.md",
    "title": "Implement Classexpressionwithstaticpropertieses Unknown Unsupported",
    "reason": "same feature label, title overlap"
  },
  {
    "state": "open",
    "path": "issues/open/1200-implement-classExtendsInterface-unknown-unsupported.md",
    "title": "Implement Classextendsinterface Unknown Unsupported",
    "reason": "same feature label, title overlap"
  },
  {
    "state": "open",
    "path": "issues/done/1432-implement-conflictMarkerDiff-unknown-unsupported.md",
    "title": "Implement Conflictmarkerdiff Unknown Unsupported",
    "reason": "same feature label, title overlap"
  },
  {
    "state": "open",
    "path": "issues/open/1434-implement-conflictMarkerTrivia-unknown-unsupported.md",
    "title": "Implement Conflictmarkertrivia Unknown Unsupported",
    "reason": "same feature label, title overlap"
  },
  {
    "state": "open",
    "path": "issues/open/1444-implement-constDeclarations-unknown-unsupported.md",
    "title": "Implement Constdeclarations Unknown Unsupported",
    "reason": "same feature label, title overlap"
  },
  {
    "state": "open",
    "path": "issues/open/1496-implement-contextualReturnTypeOfIIFE-unknown-unsupported.md",
    "title": "Implement Contextualreturntypeofiife Unknown Unsupported",
    "reason": "same feature label, title overlap"
  },
  {
    "state": "open",
    "path": "issues/open/1502-implement-contextualSignatureInstantiation-unknown-unsupported.md",
    "title": "Implement Contextualsignatureinstantiation Unknown Unsupported",
    "reason": "same feature label, title overlap"
  },
  {
    "state": "open",
    "path": "issues/open/1508-implement-contextualTypeBasedOnIntersectionWithAnyInTheMix-unknown-unsupported.md",
    "title": "Implement Contextualtypebasedonintersectionwithanyinthemix Unknown Unsupported",
    "reason": "same feature label, title overlap"
  },
  {
    "state": "open",
    "path": "issues/open/1520-implement-contextualTyping-unknown-unsupported.md",
    "title": "Implement Contextualtyping Unknown Unsupported",
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
error: [UnsupportedSyntax] issue-292: top-level function mutation of outer binding `varBinding` requires mutable outer environment lowering at 0..15
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
        "message": "File '/tmp/tmpnar3lsxs/test262-triage-node-input.js' is a JavaScript file. Did you mean to enable the 'allowJs' option?\n  The file is in the program because:\n    Root file specified for compilation"
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
        "text": "function print(message) {\n  console.log(message);\n}",
        "line": 2,
        "character": 1
      }
    ]
  }
}
```

Stack trace:

```text
error: [UnsupportedSyntax] issue-292: top-level function mutation of outer binding `varBinding` requires mutable outer environment lowering at 0..15
```


## 重複整理メモ

この issue はタイトルが重複しているため、内容がより充実している
`issues/open/429-implement-eval.md` に統合されました。
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
- `issues/done/5014-implement-eval.md`
- `issues/index.md` after regeneration

Future-work tracking: no untracked future-work item was identified in this issue during this metadata/evidence audit.
