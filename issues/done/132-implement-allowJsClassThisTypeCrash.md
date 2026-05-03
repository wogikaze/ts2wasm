---
id: 132
title: "Implement Allowjsclassthistypecrash (dup)"
type: spike
area: runtime/builtins
class: superseded
priority: P1
depends_on: [5004]
blocks: []
created: 2026-04-29
updated: 2026-05-04
---

## Summary

Triage allowJsClassThisTypeCrash across 1 failing reference test cases and split this bucket into implementation-ready child issues.

## Problem

Reference test results show 1 cases fail in directory `allowJsClassThisTypeCrash` with diagnostics: function. Root cause is a runtime issue (issue-062e): nested function `this` closure capture is not supported. This is not a parser/semantics issue but a runtime closure limitation.

Problem: allowJsClassThisTypeCrash has 1 reference failures and needs smart-triage evidence before implementation starts.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/allowJsClassThisTypeCrash.ts
```

Coverage window:

```sh
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/allowJsClassThisTypeCrash.ts --detail
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
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/allowJsClassThisTypeCrash.ts --detail
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/allowJsClassThisTypeCrash.ts
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

- `reference/typescript/tests/cases/compiler/allowJsClassThisTypeCrash.ts`

## Duplicate detection

## Smart triage

### Smart triage: Triage function: allowJsClassThisTypeCrash

- Issue class: `triage-needed`
- Feature label: `function`
- Diagnostic: `UnsupportedSyntax` / `parser-or-frontend-unsupported`
- Path: `reference/typescript/tests/cases/compiler/allowJsClassThisTypeCrash.ts`

Reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/allowJsClassThisTypeCrash.ts
```

Source overview:

```json
{
  "suite": "tsc",
  "bytes": 170,
  "lines": 11,
  "extension": ".ts",
  "first_code_line": "const f = function() {};"
}
```

Failure location:

```json
{
  "code": "UnsupportedSyntax",
  "message": "unsupported expression: Some(SpannedToken { kind: Function, span: Span { start: 110, end: 118 } }) at 118..119",
  "span_start": 118,
  "span_end": 119,
  "line": 7,
  "column": 25,
  "feature_label": "function",
  "error_type": "parser-or-frontend-unsupported"
}
```

Source context:

```text
 4 | // @noEmit: true
 5 |
 6 | // @filename: app.js
 7 | const f = function() {};
 8 | var g = f;
 9 | g.prototype.m = function () {
10 |   this;
```

Visible symbols before failure:

```json
[
  {
    "kind": "binding",
    "name": "f",
    "line": 7,
    "column": 1,
    "initializer": "function() {}"
  }
]
```

Duplicate candidates:

```json
[
  {
    "state": "open",
    "path": "issues/open/062-implement-function.md",
    "title": "Implement function support",
    "reason": "same feature label, title overlap"
  },
  {
    "state": "open",
    "path": "issues/open/062c-ordinary-function-declarations-and-calls.md",
    "title": "Implement ordinary function declarations and direct calls",
    "reason": "same feature label, title overlap"
  },
  {
    "state": "open",
    "path": "issues/open/062d-function-this-and-arguments.md",
    "title": "Implement function this and arguments semantics",
    "reason": "same feature label, title overlap"
  },
  {
    "state": "done",
    "path": "issues/done/062e-function-closures.md",
    "title": "Implement function closures",
    "reason": "same feature label, title overlap"
  },
  {
    "state": "open",
    "path": "issues/open/062f-function-object-metadata.md",
    "title": "Implement function object metadata",
    "reason": "same feature label, title overlap"
  },
  {
    "state": "open",
    "path": "issues/open/132-implement-allowJsClassThisTypeCrash.md",
    "title": "Implement Allowjsclassthistypecrash",
    "reason": "same reference path, same feature label, title overlap"
  },
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
    "path": "issues/done/057-implement-function-resolution.md",
    "title": "Implement function resolution for function calls",
    "reason": "same feature label, title overlap"
  },
  {
    "state": "done",
    "path": "issues/done/062a-split-function-epic-into-callable-child-issues.md",
    "title": "Split function epic into callable child issues",
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
        kind: Const,
        span: Span {
            start: 100,
            end: 105,
        },
    },
    SpannedToken {
        kind: Ident(
            "f",
        ),
        span: Span {
            start: 106,
            end: 107,
        },
    },
    SpannedToken {
        kind: Equal,
        span: Span {
            start: 108,
            end: 109,
        },
    },
    SpannedToken {
        kind: Function,
        span: Span {
            start: 110,
            end: 118,
        },
    },
    SpannedToken {
        kind: LeftParen,
        span: Span {
            start: 118,
            end: 119,
        },
    },
    SpannedToken {
        kind: RightParen,
        span: Span {
            start: 119,
            end: 120,
        },
    },
    SpannedToken {
        kind: LeftBrace,
        span: Span {
            start: 121,
            end: 122,
        },
    },
    SpannedToken {
        kind: RightBrace,
        span: Span {
            start: 122,
            end: 123,
        },
    },
    SpannedToken {
        kind: Semicolon,
        span: Span {
            start: 123,
            end: 124,
        },
    },
    SpannedToken {
        kind: Var,
        span: Span {
            start: 126,
            end: 129,
        },
    },
    SpannedToken {
        kind: Ident(
            "g",
        ),
        span: Span {
            start: 130,
            end: 131,
        },
    },
    SpannedToken {
        kind: Equal,
        span: Span {
            start: 132,
            end: 133,
        },
    },
    SpannedToken {
        kind: Ident(
            "f",
        ),
        span: Span {
            start: 134,
            end: 135,
        },
    },
    SpannedToken {
        kind: Semicolon,
        s
```

#### ast

- ok: `False`
- truncated: `False`

```text
error: [UnsupportedSyntax] unsupported expression: Some(SpannedToken { kind: Function, span: Span { start: 110, end: 118 } }) at 118..119
```

#### resolved

- ok: `False`
- truncated: `False`

```text
error: [UnsupportedSyntax] unsupported expression: Some(SpannedToken { kind: Function, span: Span { start: 110, end: 118 } }) at 118..119
```

TypeScript/JavaScript oracle:

```json
{
  "ok": true,
  "returncode": 0,
  "typescript": {
    "ok": true,
    "diagnostics": [],
    "hints": [
      {
        "kind": "binding",
        "typeText": "() => void",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/allowJsClassThisTypeCrash.ts",
        "start": 106,
        "length": 1,
        "line": 7,
        "character": 7,
        "name": "f"
      },
      {
        "kind": "binding",
        "typeText": "() => void",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/allowJsClassThisTypeCrash.ts",
        "start": 130,
        "length": 1,
        "line": 8,
        "character": 5,
        "name": "g"
      }
    ],
    "typescriptVersion": "6.0.3"
  },
  "ast": {
    "topLevel": [
      {
        "kind": "FirstStatement",
        "text": "const f = function() {};",
        "line": 7,
        "character": 1
      },
      {
        "kind": "FirstStatement",
        "text": "var g = f;",
        "line": 8,
        "character": 1
      },
      {
        "kind": "ExpressionStatement",
        "text": "g.prototype.m = function () {\r\n  this;\r\n};",
        "line": 9,
        "character": 1
      }
    ],
    "pathToPosition": [
      {
        "kind": "SourceFile",
        "text": "const f = function() {};\r\nvar g = f;\r\ng.prototype.m = function () {\r\n  this;\r\n};",
        "line": 7,
        "character": 1
      },
      {
        "kind": "FirstStatement",
        "text": "const f = function() {};",
        "line": 7,
        "character": 1
      },
      {
        "kind": "VariableDeclarationList",
        "text": "const f = function() {}",
        "line": 7,
        "character": 1
      },
      {
        "kind": "VariableDeclaration",
        "text": "f = function() {}",
        "line": 7,
        "character": 7
      },
      {
        "kind": "FunctionExpression",
        "text": "function() {}",
        "line": 7,
        "character": 11
      }
    ]
  }
}
```

Stack trace:

```text
error: [UnsupportedSyntax] unsupported expression: Some(SpannedToken { kind: Function, span: Span { start: 110, end: 118 } }) at 118..119
```


## 重複整理メモ

この issue はタイトルが重複しているため、内容がより充実している
`issues/open/597-implement-allowJsClassThisTypeCrash.md` に統合されました。
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
