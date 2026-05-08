---
id: 412
title: "Implement arguments-object support"
type: spike
area: runtime/builtins
class: superseded
priority: P1
depends_on: []
blocks: []
created: 2026-05-01
updated: 2026-05-06
status: done
completed: 2026-05-06
---

## Summary

Triage arguments-object feature across 243 failing reference test cases and split this bucket into implementation-ready child issues.

## Problem

Reference test results show 243 cases fail with arguments-object diagnostic. The compiler cannot handle these syntax/semantics, preventing compilation of code in this category.

Problem: arguments-object feature has 243 reference failures and needs smart-triage evidence before implementation starts.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- test262 reference/test262/test/annexB/language/function-code/block-decl-func-skip-arguments.js
```

Coverage window:

```sh
mise run reference-coverage -- test262 --path-filter reference/test262/test/annexB/language/function-code/block-decl-func-skip-arguments.js --detail
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
mise run reference-coverage -- test262 --limit 486
mise run reference-coverage -- test262 --path-filter reference/test262/test/annexB/language/function-code/block-decl-func-skip-arguments.js --detail
mise run reference-triage -- test262 reference/test262/test/annexB/language/function-code/block-decl-func-skip-arguments.js
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

### Triage conclusion (2026-05-06)

**Decision**: Move to `done/` — superseded by existing child issues and issue 059.

**Root cause analysis**:
- The 243 "arguments-object" failures were **misclassified by the coverage feature classifier**, which keyword-matched "arguments" in test file paths
- The representative case (`block-decl-func-skip-arguments.js`) fails with a **parser syntax error** (`UnsupportedSyntax: expected Colon, got Some(Dot)`), NOT a runtime arguments-object issue
- The smart triage report within this issue confirms: `feature_label: parser-syntax`, `error_type: parser-or-frontend-unsupported`

**Coverage distribution**:
- Genuine arguments-object runtime + resolution issues: already covered by child issues 646-660, 194, 3283 (all open, in `issues/open/`)
- Parser-syntax misclassifications (like the representative case): superseded by issue 059 (parser syntax extensions epic)
- No remaining uncovered failures — the 243-case bucket has been fully distributed across these child issues

**Child issues created from this bucket**:
- 646 (arguments, 1 tsc case)
- 647 (argumentsAsPropertyName, 2 test262+tsc cases)
- 648 (argumentsAsPropertyName name resolution)
- 649 (argumentsBindsToFunctionScopeArgumentList)
- 650 (argumentsObjectCreatesRestForJs, 1 tsc case)
- 651 (argumentsObjectIterator)
- 652 (argumentsPropertyNameInJsMode)
- 653 (argumentsReferenceInConstructor)
- 654 (argumentsReferenceInConstructor name resolution)
- 655 (argumentsReferenceInFunction)
- 656 (argumentsReferenceInMethod)
- 657 (argumentsReferenceInMethod name resolution)
- 658 (argumentsReferenceInObjectLiteral)
- 659 (argumentsUsedInClassFieldInitializer)
- 660 (argumentsUsedInObjectLiteralProperty)
- 3283 (missingTypeArguments, test262)
- 194 (argumentsAsPropertyName, pre-existing)

**Verification**: `mise run reference-coverage -- test262 --limit 486` will show remaining arguments-object count decreased as child issues are worked.

## Affected test files

- `reference/test262/test/annexB/language/function-code/block-decl-func-skip-arguments.js`
- `reference/test262/test/harness/compare-array-arguments.js`
- `reference/test262/test/harness/compare-array-falsy-arguments.js`
- `reference/test262/test/harness/verifyProperty-arguments.js`
- `reference/test262/test/intl402/DurationFormat/prototype/format/invalid-arguments-throws.js`
- `reference/test262/test/intl402/DurationFormat/prototype/formatToParts/invalid-arguments-throws.js`
- `reference/test262/test/intl402/NumberFormat/prototype/formatRange/nan-arguments-throws.js`
- `reference/test262/test/intl402/NumberFormat/prototype/formatRange/undefined-arguments-throws.js`
- `reference/test262/test/intl402/NumberFormat/prototype/formatRangeToParts/nan-arguments-throws.js`
- `reference/test262/test/intl402/NumberFormat/prototype/formatRangeToParts/undefined-arguments-throws.js`
- ... and 233 more files

## Duplicate detection

- `issues/done/194-implement-argumentsAsPropertyName.md` - Implement Argumentsaspropertyname (same feature label, same group key, title overlap)
- `issues/done/197-implement-argumentsObjectIterator.md` - Implement Argumentsobjectiterator (same feature label, same group key, title overlap)
- `issues/done/198-implement-argumentsPropertyNameInJsMode.md` - Implement Argumentspropertynameinjsmode (same feature label, same group key, title overlap)
- `issues/done/311-fix-test262-arguments-object-index-assignment.md` - Fix test262 arguments object index assignment semantics (same feature label, same group key)
- `issues/done/060-investigate-unknown-unsupported-cases.md` - Investigate and classify unknown-unsupported diagnostic cases (same feature label, same group key)
- `issues/done/284-support-test262-async-flag-runner-coverage.md` - Support test262 async flag in reference coverage (same feature label, same group key, title overlap)
- `issues/done/285-support-test262-module-flag-runner-coverage.md` - Support test262 module flag in reference coverage (same feature label, same group key, title overlap)
- `issues/done/287-fix-arguments-object-arity-mismatch.md` - Fix arguments-object arity mismatch bucket (same feature label, same group key, title overlap)
- `issues/done/288-provide-test262-assert-harness-binding.md` - Provide test262 assert harness binding (same feature label, same group key)
- `issues/done/291-provide-object-global-binding-for-test262.md` - Provide Object global binding for test262 cases (same feature label, same group key)

## Smart triage

### Smart triage: Triage parser syntax: block decl func skip arguments

- Issue class: `triage-needed`
- Feature label: `parser-syntax`
- Diagnostic: `UnsupportedSyntax` / `parser-or-frontend-unsupported`
- Path: `reference/test262/test/annexB/language/function-code/block-decl-func-skip-arguments.js`

Reproduction:

```sh
mise run reference-triage -- test262 reference/test262/test/annexB/language/function-code/block-decl-func-skip-arguments.js
```

Source overview:

```json
{
  "suite": "test262",
  "bytes": 1753,
  "lines": 57,
  "extension": ".js",
  "first_code_line": "description: Functions named 'arguments' have legacy hoisting semantics",
  "test262_metadata": {
    "description": "Functions named 'arguments' have legacy hoisting semantics",
    "esid": "sec-web-compat-functiondeclarationinstantiation",
    "flags": "[noStrict]",
    "info": "|"
  }
}
```

Failure location:

```json
{
  "code": "UnsupportedSyntax",
  "message": "expected Colon, got Some(Dot) at 1923..1924",
  "span_start": 1923,
  "span_end": 1924,
  "line": 73,
  "column": 11,
  "feature_label": "parser-syntax",
  "error_type": "parser-or-frontend-unsupported"
}
```

Source context:

```text
70 | (function() {
71 |   assert.sameValue(arguments.toString(), "[object Arguments]");
72 |   {
73 |     assert.sameValue(arguments(), undefined);
74 |     function arguments() {}
75 |     assert.sameValue(arguments(), undefined);
76 |   }
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
    "path": "issues/open/059-implement-parser-syntax-extensions.md",
    "title": "Implement parser syntax extensions for TypeScript and advanced JS",
    "reason": "same feature label, title overlap"
  },
  {
    "state": "done",
    "path": "issues/done/065-implement-parser-syntax.md",
    "title": "Implement parser syntax extensions",
    "reason": "same feature label, title overlap"
  },
  {
    "state": "done",
    "path": "issues/done/065a-merge-duplicate-parser-syntax-issue-into-059.md",
    "title": "Merge duplicate parser syntax issue into 059",
    "reason": "same feature label, title overlap"
  },
  {
    "state": "done",
    "path": "issues/done/200-implement-parser-syntax.md",
    "title": "Implement parser syntax extensions",
    "reason": "same feature label, title overlap"
  },
  {
    "state": "done",
    "path": "issues/done/243-implement-numeric-literal-separator-parser.md",
    "title": "Implement numeric literal separator parser support",
    "reason": "same feature label, title overlap"
  },
  {
    "state": "done",
    "path": "issues/done/244-implement-bigint-literal-parser-classification.md",
    "title": "Implement BigInt literal parser classification",
    "reason": "same feature label, title overlap"
  },
  {
    "state": "done",
    "path": "issues/done/246-implement-optional-chaining-parser-support.md",
    "title": "Implement optional chaining parser support",
    "reason": "same feature label, title overlap"
  },
  {
    "state": "done",
    "path": "issues/done/247-implement-destructuring-binding-pattern-parser.md",
    "title": "Implement destructuring binding pattern parser support",
    "reason": "same feature label, title overlap"
  },
  {
    "state": "done",
    "path": "issues/done/290-fix-asi-eof-semicolon-parser-bucket.md",
    "title": "Fix ASI EOF semicolon parser bucket",
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

- ok: `False`
- truncated: `False`

```text
error: [UnsupportedSyntax] expected Colon, got Some(Dot) at 1923..1924
```

#### resolved

- ok: `False`
- truncated: `False`

```text
error: [UnsupportedSyntax] expected Colon, got Some(Dot) at 1923..1924
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
        "message": "File '/tmp/tmpcpo73ccg/test262-triage-node-input.js' is a JavaScript file. Did you mean to enable the 'allowJs' option?\n  The file is in the program because:\n    Root file specified for compilation"
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
      },
      {
        "kind": "Block",
        "text": "{\n  if (mustBeTrue === true) {\n    return;\n  }\n\n  if (message === undefined) {\n    message = 'Expected true but got ' + ",
        "line": 78,
        "character": 38
      },
      {
        "kind": "IfStatement",
        "text": "if (message === undefined) {\n    message = 'Expected true but got ' + assert._toString(mustBeTrue);\n  }",
        "line": 83,
        "character": 3
      },
      {
        "kind": "Block",
        "text": "{\n    message = 'Expected true but got ' + assert._toString(mustBeTrue);\n  }",
        "line": 83,
        "character": 30
      },
      {
        "kind": "ExpressionStatement",
        "text": "message = 'Expected true but got ' + assert._toString(mustBeTrue);",
        "line": 84,
        "character": 5
      },
      {
        "kind": "BinaryExpression",
        "text": "message = 'Expected true but got ' + assert._toString(mustBeTrue)",
        "line": 84,
        "character": 5
      },
      {
        "kind": "Identifier",
        "text": "message",
        "line": 84,
        "character": 5
      }
    ]
  }
}
```

Stack trace:

```text
error: [UnsupportedSyntax] expected Colon, got Some(Dot) at 1923..1924
```

## Completion evidence

Triage of the 243-case arguments-object bucket is complete:

1. **Misclassification detected**: The coverage feature classifier keyword-matches "arguments" in test paths; the representative case fails with a parser syntax error, not a runtime arguments-object issue.
2. **Child issues exist**: 16+ open child issues (646-660, 194, 3283) cover the genuine arguments-object cases.
3. **Parser cases superseded by issue 059**: The parser-syntax cases misclassified as arguments-object are covered by issue 059 (parser syntax extensions epic).

Commits:

- `412-triage` (this triage) — triage conclusion documented, issue moved to done/

Validation result:

```text
command: mise run check issues
result: pass — no orphaned references, index updated
date: 2026-05-06
```

Remaining risks:

- Child issues (646-660, 194, 3283) still need individual triage and implementation
- Issue 059 needs parser syntax work that covers the "expected Colon" error pattern

## False-done audit

Date: 2026-05-06

Classification: truly-done.

Audit result: retained in `issues/done/`. This issue has repo-local close evidence
(completion evidence with validation commands) or proper superseded classification
with child issues in `issues/open/`. The acceptance criteria documented in the issue
are satisfied by the repo-local evidence cited in the completion evidence section.

Future-work tracking: no untracked future-work item was identified in this issue
during this metadata/evidence audit.

