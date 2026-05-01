---
id: 451
title: "Implement try-catch-finally"
type: spike
area: frontend/syntax
class: triage-needed
priority: P1
depends_on: []
blocks: []
created: 2026-05-01
updated: 2026-05-01
---

## Summary

Triage try-catch feature across 7 failing reference test cases and split this bucket into implementation-ready child issues.

## Problem

Reference test results show 7 cases fail with try-catch diagnostic. The compiler cannot handle these syntax/semantics, preventing compilation of code in this category.

Problem: try-catch feature has 7 reference failures and needs smart-triage evidence before implementation starts.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- test262 reference/test262/test/language/statements/try/completion-values-fn-finally-abrupt.js
```

Coverage window:

```sh
mise run reference-coverage -- test262 --path-filter reference/test262/test/language/statements/try/completion-values-fn-finally-abrupt.js --detail
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
mise run reference-coverage -- test262 --limit 14
mise run reference-coverage -- test262 --path-filter reference/test262/test/language/statements/try/completion-values-fn-finally-abrupt.js --detail
mise run reference-triage -- test262 reference/test262/test/language/statements/try/completion-values-fn-finally-abrupt.js
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

- `reference/test262/test/language/statements/try/completion-values-fn-finally-abrupt.js`
- `reference/test262/test/language/statements/try/completion-values-fn-finally-normal.js`
- `reference/test262/test/language/statements/try/completion-values-fn-finally-return.js`
- `reference/test262/test/language/statements/try/optional-catch-binding-finally.js`
- `reference/test262/test/language/statements/try/optional-catch-binding-lexical.js`
- `reference/test262/test/language/statements/try/optional-catch-binding-throws.js`
- `reference/test262/test/language/statements/try/optional-catch-binding.js`

## Duplicate detection

- none found by path/title/feature scan

## Smart triage

### Smart triage: Triage try catch: completion values fn finally abrupt

- Issue class: `triage-needed`
- Feature label: `try-catch`
- Diagnostic: `UnsupportedSyntax` / `parser-or-frontend-unsupported`
- Path: `reference/test262/test/language/statements/try/completion-values-fn-finally-abrupt.js`

Reproduction:

```sh
mise run reference-triage -- test262 reference/test262/test/language/statements/try/completion-values-fn-finally-abrupt.js
```

Source overview:

```json
{
  "suite": "test262",
  "bytes": 2331,
  "lines": 79,
  "extension": ".js",
  "first_code_line": "esid: sec-try-statement-runtime-semantics-evaluation",
  "test262_metadata": {
    "esid": "sec-try-statement-runtime-semantics-evaluation",
    "description": ">",
    "info": "|",
    "TryStatement": "try Block Catch Finally"
  }
}
```

Failure location:

```json
{
  "code": "UnsupportedSyntax",
  "message": "expected member property name, got Some(SpannedToken { kind: Catch, span: Span { start: 1702, end: 1707 } }) at 1708..1709",
  "span_start": 1708,
  "span_end": 1709,
  "line": 65,
  "column": 13,
  "feature_label": "try-catch",
  "error_type": "parser-or-frontend-unsupported"
}
```

Source context:

```text
62 | var fn, count = {};
63 | 
64 | // 1: try Abrupt, catch Abrupt, finally Abrupt; Completion: finally
65 | count.catch = 0;
66 | count.finally = 0;
67 | fn = function() {
68 |   try {
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
    "name": "C",
    "line": 55,
    "column": 29
  },
  {
    "kind": "binding",
    "name": "C",
    "line": 56,
    "column": 11
  },
  {
    "kind": "binding",
    "name": "fn",
    "line": 62,
    "column": 1
  }
]
```

Duplicate candidates:

```json
[]
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

- ok: `False`
- truncated: `False`

```text
error: [UnsupportedSyntax] expected member property name, got Some(SpannedToken { kind: Catch, span: Span { start: 1702, end: 1707 } }) at 1708..1709
```

#### resolved

- ok: `False`
- truncated: `False`

```text
error: [UnsupportedSyntax] expected member property name, got Some(SpannedToken { kind: Catch, span: Span { start: 1702, end: 1707 } }) at 1708..1709
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
        "message": "File '/tmp/tmp_zl5ihn4/test262-triage-node-input.js' is a JavaScript file. Did you mean to enable the 'allowJs' option?\n  The file is in the program because:\n    Root file specified for compilation"
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
error: [UnsupportedSyntax] expected member property name, got Some(SpannedToken { kind: Catch, span: Span { start: 1702, end: 1707 } }) at 1708..1709
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
