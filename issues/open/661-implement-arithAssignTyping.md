---
id: 661
title: "Implement Arithassigntyping"
type: spike
area: frontend/syntax
class: superseded
priority: P1
depends_on: [5349]
blocks: []
created: 2026-05-01
updated: 2026-05-07
completed: 2026-05-07
status: done
---
> **Reopened by false-done audit** (2026-05-07)
> Classification: must-reopen
> Reason: Batch-migrated to issues/done/ without implementation commits.
> Evidence: Empty completion evidence. No feat/fix commit for #661.

## Summary

Closed this generated arithmetic-assignment bucket after splitting the current
`*=`, `/=`, and `%=` parser boundary to
`issues/open/5349-parse-multiplicative-compound-assignment-operators.md`.

## Problem

Reference test results show 1 cases fail in directory `arithAssignTyping` with diagnostics: parser-syntax. The compiler cannot handle these syntax/semantics, preventing compilation of code in this category.

Problem: arithAssignTyping has 1 reference failures and needs smart-triage evidence before implementation starts.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/arithAssignTyping.ts
```

Coverage window:

```sh
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/arithAssignTyping.ts --detail
```

## Desired final state

This generated bucket is closed. Implement from
`issues/open/5349-parse-multiplicative-compound-assignment-operators.md`.

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
- [x] Child issue contains exact `reference-triage` commands
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
mise run reference-coverage -- tsc --limit 2
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/arithAssignTyping.ts --detail
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/arithAssignTyping.ts
```

Not run:

- none

## Docs / current-state / issue sync

Final-state docs:

- [x] not affected

Current state:

- [x] not affected

Follow-up issues:

- [x] created: `issues/open/5349-parse-multiplicative-compound-assignment-operators.md`

## Notes

## Affected test files

- `reference/typescript/tests/cases/compiler/arithAssignTyping.ts`

## Duplicate detection

Split to `issues/open/5349-parse-multiplicative-compound-assignment-operators.md`.

No exact existing implementation-ready owner covered identifier-target `*=`,
`/=`, and `%=`:

- `issues/done/5178-parse-bitwise-compound-assignment-operators.md` owns bitwise `^=`, `&=`, and `|=`.
- `issues/done/5164-parse-exponentiation-compound-assignment.md` owns `**=`.
- `issues/open/5311-parse-property-access-arithmetic-compound-assignments.md` owns namespace property `+=`.

Current evidence:

```text
arithAssignTyping.ts: UnsupportedSyntax expected Semicolon, got Some(StarEqual) at 92..94
source: f *= 1; // error
TypeScript oracle: TS2629 Cannot assign to 'f' because it is a class.
```

## Smart triage

### Smart triage: Triage parser syntax: arithAssignTyping

- Issue class: `triage-needed`
- Feature label: `parser-syntax`
- Diagnostic: `UnsupportedSyntax` / `parser-or-frontend-unsupported`
- Path: `reference/typescript/tests/cases/compiler/arithAssignTyping.ts`

Reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/arithAssignTyping.ts
```

Source overview:

```json
{
  "suite": "tsc",
  "bytes": 240,
  "lines": 15,
  "extension": ".ts",
  "first_code_line": "class f { }"
}
```

Failure location:

```json
{
  "code": "UnsupportedSyntax",
  "message": "expected Semicolon, got Some(StarEqual) at 92..94",
  "span_start": 92,
  "span_end": 94,
  "line": 7,
  "column": 9,
  "feature_label": "parser-syntax",
  "error_type": "parser-or-frontend-unsupported"
}
```

Source context:

```text
 4 | f += ''; // error
 5 | f += 1; // error
 6 | f -= 1; // error
 7 | f *= 1; // error
 8 | f /= 1; // error
 9 | f %= 1; // error
10 | f &= 1; // error
```

Visible symbols before failure:

```json
[
  {
    "kind": "class",
    "name": "f",
    "line": 2,
    "column": 1
  }
]
```

Duplicate candidates:

```json
[
  {
    "state": "open",
    "path": "issues/open/442-implement-parser-syntax.md",
    "title": "Implement parser syntax extensions",
    "reason": "same feature label, title overlap"
  },
  {
    "state": "open",
    "path": "issues/done/464-implement-FunctionDeclaration-parser-syntax.md",
    "title": "Implement Functiondeclaration Parser Syntax",
    "reason": "same feature label, title overlap"
  },
  {
    "state": "open",
    "path": "issues/open/550-implement-FunctionDeclaration-parser-syntax.md",
    "title": "Implement Functiondeclaration Parser Syntax",
    "reason": "same feature label, title overlap"
  },
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
        kind: Class,
        span: Span {
            start: 20,
            end: 25,
        },
    },
    SpannedToken {
        kind: Ident(
            "f",
        ),
        span: Span {
            start: 26,
            end: 27,
        },
    },
    SpannedToken {
        kind: LeftBrace,
        span: Span {
            start: 28,
            end: 29,
        },
    },
    SpannedToken {
        kind: RightBrace,
        span: Span {
            start: 30,
            end: 31,
        },
    },
    SpannedToken {
        kind: Ident(
            "f",
        ),
        span: Span {
            start: 35,
            end: 36,
        },
    },
    SpannedToken {
        kind: PlusEqual,
        span: Span {
            start: 37,
            end: 39,
        },
    },
    SpannedToken {
        kind: String(
            "",
        ),
        span: Span {
            start: 40,
            end: 42,
        },
    },
    SpannedToken {
        kind: Semicolon,
        span: Span {
            start: 42,
            end: 43,
        },
    },
    SpannedToken {
        kind: Ident(
            "f",
        ),
        span: Span {
            start: 54,
            end: 55,
        },
    },
    SpannedToken {
        kind: PlusEqual,
        span: Span {
            start: 56,
            end: 58,
        },
    },
    SpannedToken {
        kind: Number(
            1,
        ),
        span: Span {
            start: 59,
            end: 60,
        },
    },
    SpannedToken {
        kind: Semicolon,
        span: Span {
            start: 60,
            end: 61,
        },
    },
    SpannedToken {
        kind: Ident(
            "f",
        ),
        span: Span {
            start: 72,
            end: 73,
        },
    },
    SpannedToken {
        kind: MinusEqual,
        span: Span {
            start: 74,
            end: 76,
        },
    },
    SpannedToken {
        kind: Number(
            1,
        ),
        span: Span {
            start: 77,
            end: 78,
        },
    },
    SpannedToken {
        kind: Semicolon,
        span: Span {
            start: 78,
            end: 79,
        },
    },
    SpannedToken {
        kind: Ident(
            "f",
        ),
        span: Span {
            start: 90,
            end: 91,
        },
    },
    SpannedToken {
        kind: StarEqual,
        span: Span {
            start: 92,
            end: 94,
        },
    },
    SpannedToken {
        kind: Number(
            1,
        ),
        span: Span {
            start: 95,
            end: 96,
        },
    },
    SpannedToken {
        kind: Semicolon,
        span: Span {
            start: 96,
            end: 97,
        },
    },
    SpannedToken {
        kind: Ident(
            "f",
        ),
        span: Span {
            start: 108,
            end: 109,
        },
    },
    SpannedToken {
        kind: SlashEqual,
        span: Span {
            start: 110,
            end: 112,
        },
    },
    SpannedToken {
        kind: Number(
            1,
        ),
        span: Span {
            start: 113,
            end: 114,
        },
    },
    SpannedToken {
        kind: Semicolon,
        span: Span {
            start: 114,
            end: 115,
        },
    },
    SpannedToken {
        kind: Ident(
            "f",
        ),
        span: Span {
            start: 126,
            end: 127,
        },
    },
    SpannedToken
```

#### ast

- ok: `False`
- truncated: `False`

```text
error: [UnsupportedSyntax] expected Semicolon, got Some(StarEqual) at 92..94
```

#### resolved

- ok: `False`
- truncated: `False`

```text
error: [UnsupportedSyntax] expected Semicolon, got Some(StarEqual) at 92..94
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
        "code": 2629,
        "category": "Error",
        "message": "Cannot assign to 'f' because it is a class.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/arithAssignTyping.ts",
        "start": 35,
        "length": 1,
        "line": 4,
        "character": 1
      },
      {
        "code": 2629,
        "category": "Error",
        "message": "Cannot assign to 'f' because it is a class.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/arithAssignTyping.ts",
        "start": 54,
        "length": 1,
        "line": 5,
        "character": 1
      },
      {
        "code": 2629,
        "category": "Error",
        "message": "Cannot assign to 'f' because it is a class.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/arithAssignTyping.ts",
        "start": 72,
        "length": 1,
        "line": 6,
        "character": 1
      },
      {
        "code": 2629,
        "category": "Error",
        "message": "Cannot assign to 'f' because it is a class.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/arithAssignTyping.ts",
        "start": 90,
        "length": 1,
        "line": 7,
        "character": 1
      },
      {
        "code": 2629,
        "category": "Error",
        "message": "Cannot assign to 'f' because it is a class.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/arithAssignTyping.ts",
        "start": 108,
        "length": 1,
        "line": 8,
        "character": 1
      },
      {
        "code": 2629,
        "category": "Error",
        "message": "Cannot assign to 'f' because it is a class.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/arithAssignTyping.ts",
        "start": 126,
        "length": 1,
        "line": 9,
        "character": 1
      },
      {
        "code": 2629,
        "category": "Error",
        "message": "Cannot assign to 'f' because it is a class.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/arithAssignTyping.ts",
        "start": 144,
        "length": 1,
        "line": 10,
        "character": 1
      },
      {
        "code": 2629,
        "category": "Error",
        "message": "Cannot assign to 'f' because it is a class.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/arithAssignTyping.ts",
        "start": 162,
        "length": 1,
        "line": 11,
        "character": 1
      },
      {
        "code": 2629,
        "category": "Error",
        "message": "Cannot assign to 'f' because it is a class.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/arithAssignTyping.ts",
        "start": 180,
        "length": 1,
        "line": 12,
        "character": 1
      },
      {
        "code": 2629,
        "category": "Error",
        "message": "Cannot assign to 'f' because it is a class.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/arithAssignTyping.ts",
        "start": 199,
        "length": 1,
        "line": 13,
        "character": 1
      },
      {
        "code": 2629,
        "category": "Error",
        "message": "Cannot assign to 'f' because it is a class.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/arithAssignTyping.ts",
        "start": 218,
        "length": 1,
        "line": 14,
        "character": 1
      },
      {
        "code": 2629,
        "category": "Error",
        "message": "Cannot assign to 'f' because it is a class.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/arithAssignTyping.ts",
        "start": 238,
        "length": 1,
        "line": 15,
        "character": 1
      }
    ],
    "hints": [],
    "typescriptVersion": "6.0.3"
  },
  "ast": {
    "topLevel": [
      {
        "kind": "ClassDeclaration",
        "text": "class f { }",
        "line": 2,
        "character": 1
      },
      {
        "kind": "ExpressionStatement",
        "text": "f += '';",
        "line": 4,
        "character": 1
      },
      {
        "kind": "ExpressionStatement",
        "text": "f += 1;",
        "line": 5,
        "character": 1
      },
      {
        "kind": "ExpressionStatement",
        "text": "f -= 1;",
        "line": 6,
        "character": 1
      },
      {
        "kind": "ExpressionStatement",
        "text": "f *= 1;",
        "line": 7,
        "character": 1
      },
      {
        "kind": "ExpressionStatement",
        "text": "f /= 1;",
        "line": 8,
        "character": 1
      },
      {
        "kind": "ExpressionStatement",
        "text": "f %= 1;",
        "line": 9,
        "character": 1
      },
      {
        "kind": "ExpressionStatement",
        "text": "f &= 1;",
        "line": 10,
        "character": 1
      },
      {
        "kind": "ExpressionStatement",
        "text": "f |= 1;",
        "line": 11,
        "character": 1
      },
      {
        "kind": "ExpressionStatement",
        "text": "f <<= 1;",
        "line": 12,
        "character": 1
      },
      {
        "kind": "ExpressionStatement",
        "text": "f >>= 1;",
        "line": 13,
        "character": 1
      },
      {
        "kind": "ExpressionStatement",
        "text": "f >>>= 1;",
        "line": 14,
        "character": 1
      },
      {
        "kind": "ExpressionStatement",
        "text": "f ^= 1;",
        "line": 15,
        "character": 1
      }
    ],
    "pathToPosition": [
      {
        "kind": "SourceFile",
        "text": "class f { }\r\n\r\nf += ''; // error\r\nf += 1; // error\r\nf -= 1; // error\r\nf *= 1; // error\r\nf /= 1; // error\r\nf %= 1; // err",
        "line": 2,
        "character": 1
      },
      {
        "kind": "ExpressionStatement",
        "text": "f *= 1;",
        "line": 7,
        "character": 1
      },
      {
        "kind": "BinaryExpression",
        "text": "f *= 1",
        "line": 7,
        "character": 1
      },
      {
        "kind": "AsteriskEqualsToken",
        "text": "*=",
        "line": 7,
        "character": 3
      }
    ]
  }
}
```

Stack trace:

```text
error: [UnsupportedSyntax] expected Semicolon, got Some(StarEqual) at 92..94
```

## Completion evidence

Fill only when moving to `done/`.

Commits:

- `pending`

Validation result:

```text
command: env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/arithAssignTyping.ts
result: pass; current blocker identified as identifier-target multiplicative compound assignment parser syntax, split to issue 5349
date: 2026-05-07
```

Remaining risks:

- Later triage may expose bitwise/shift compound assignment or final
  class-binding assignment diagnostics after issue 5349 advances past `*=`.
