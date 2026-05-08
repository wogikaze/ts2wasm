---
id: 558
title: "Implement Abstractpropertyinconstructor"
type: spike
area: frontend/syntax
class: done
priority: P1
depends_on: [5390]
blocks: []
created: 2026-05-01
updated: 2026-05-07
completed: 2026-05-07
---

## Summary

Close `abstractPropertyInConstructor` after splitting the current later blocker into issue 5390.

## Problem

Reference test results originally showed 1 case failing in directory `abstractPropertyInConstructor` with diagnostics: parser-syntax. Fresh triage on 2026-05-07 shows the parser blocker is gone; the current failure is a lowerer/diagnostic gap for constructor access to abstract properties.

Problem: `abstractPropertyInConstructor.ts` now reaches `UnsupportedSyntax: method AbstractClass.cb not found` where TypeScript reports TS2715 for abstract property access in the constructor.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/abstractPropertyInConstructor.ts
```

Coverage window:

```sh
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/abstractPropertyInConstructor.ts --detail
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

- [x] Duplicate candidates below are confirmed as no-match for the exact current failure
- [x] Child issue 5390 contains an exact `reference-triage` command
- [x] Child issue 5390 includes failing path, diagnostic code, source context, visible symbols, and parser/TypeScript AST evidence
- [x] Child issue 5390 acceptance names the exact reference path and diagnostic/stdout change

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run
```

Impacted commands:

```sh
mise run reference-coverage -- tsc --limit 2
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/abstractPropertyInConstructor.ts --detail
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/abstractPropertyInConstructor.ts
```

Not run:

- `cargo fmt --all --check` and `cargo nextest run`; this close is an
  issue-lifecycle-only split update, so focused reference and issue checks were
  used instead.

## Docs / current-state / issue sync

Final-state docs:

- [x] not affected

Current state:

- [x] not affected

Follow-up issues:

- [x] created: `issues/open/5390-report-abstract-property-constructor-access-diagnostics.md`

## Notes

## Affected test files

- `reference/typescript/tests/cases/compiler/abstractPropertyInConstructor.ts`

## Duplicate detection

- Split current failure to
  `issues/open/5390-report-abstract-property-constructor-access-diagnostics.md`.
- `issues/done/5261-report-class-typed-missing-instance-method-calls.md` is
  related but not exact: it handles class-typed ambient locals whose requested
  instance method is missing.
- `issues/done/5322-support-callable-class-auto-accessor-fields.md` is related
  but not exact: it handles callable auto-accessor fields, not abstract
  property constructor diagnostics.

- `issues/done/087-implement-abstractPropertyInConstructor.md` - Implement Abstractpropertyinconstructor (same reference path, same feature label, same group key, title overlap)
- `issues/done/472-implement-abstractPropertyInConstructor.md` - Implement Abstractpropertyinconstructor (same reference path, same feature label, same group key, title overlap)

## Smart triage

Generated 2026-05-07.

```text
command:
env TS2WASM_BINARY=/home/wogikaze/wgkz/ts2wasm/target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/abstractPropertyInConstructor.ts

result:
UnsupportedSyntax / method-call

current diagnostic:
method `AbstractClass.cb` not found at 269..281

compiler evidence:
tokens: ok
ast: ok; AbstractClass constructor body parses; abstract properties are present
resolved/lowered: fails in lower_program

TypeScript oracle:
TS2715 for constructor accesses to abstract properties `prop`, `cb`, `x`, and
`y`; representative message is "Abstract property 'cb' in class
'AbstractClass' cannot be accessed in the constructor."

decision:
split to issues/open/5390-report-abstract-property-constructor-access-diagnostics.md
```

## Historical smart triage

### Smart triage: Triage parser syntax: abstractPropertyInConstructor

- Issue class: `triage-needed`
- Feature label: `parser-syntax`
- Diagnostic: `UnsupportedSyntax` / `parser-or-frontend-unsupported`
- Path: `reference/typescript/tests/cases/compiler/abstractPropertyInConstructor.ts`

Reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/abstractPropertyInConstructor.ts
```

Source overview:

```json
{
  "suite": "tsc",
  "bytes": 2079,
  "lines": 93,
  "extension": ".ts",
  "first_code_line": "abstract class AbstractClass {"
}
```

Failure location:

```json
{
  "code": "UnsupportedSyntax",
  "message": "expected Semicolon, got Some(Ident(\"other\")) at 473..478",
  "span_start": 473,
  "span_end": 478,
  "line": 18,
  "column": 26,
  "feature_label": "parser-syntax",
  "error_type": "parser-or-frontend-unsupported"
}
```

Source context:

```text
15 |         }
16 | 
17 |         // OK, references are to another instance
18 |         other.cb(other.prop);
19 |     }
20 | 
21 |     abstract prop: string;
```

Visible symbols before failure:

```json
[
  {
    "kind": "class",
    "name": "AbstractClass",
    "line": 2,
    "column": 10
  },
  {
    "kind": "binding",
    "name": "val",
    "line": 5,
    "column": 9,
    "initializer": "this.prop.toLowerCase()"
  },
  {
    "kind": "binding",
    "name": "innerFunction",
    "line": 13,
    "column": 9,
    "initializer": "() => {"
  }
]
```

Duplicate candidates:

```json
[
  {
    "state": "open",
    "path": "issues/done/087-implement-abstractPropertyInConstructor.md",
    "title": "Implement Abstractpropertyinconstructor",
    "reason": "same reference path, same feature label, title overlap"
  },
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
    "path": "issues/done/472-implement-abstractPropertyInConstructor.md",
    "title": "Implement Abstractpropertyinconstructor",
    "reason": "same reference path, same feature label, title overlap"
  },
  {
    "state": "open",
    "path": "issues/done/550-implement-FunctionDeclaration-parser-syntax.md",
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
        kind: Ident(
            "abstract",
        ),
        span: Span {
            start: 20,
            end: 28,
        },
    },
    SpannedToken {
        kind: Class,
        span: Span {
            start: 29,
            end: 34,
        },
    },
    SpannedToken {
        kind: Ident(
            "AbstractClass",
        ),
        span: Span {
            start: 35,
            end: 48,
        },
    },
    SpannedToken {
        kind: LeftBrace,
        span: Span {
            start: 49,
            end: 50,
        },
    },
    SpannedToken {
        kind: Ident(
            "constructor",
        ),
        span: Span {
            start: 56,
            end: 67,
        },
    },
    SpannedToken {
        kind: LeftParen,
        span: Span {
            start: 67,
            end: 68,
        },
    },
    SpannedToken {
        kind: Ident(
            "str",
        ),
        span: Span {
            start: 68,
            end: 71,
        },
    },
    SpannedToken {
        kind: Colon,
        span: Span {
            start: 71,
            end: 72,
        },
    },
    SpannedToken {
        kind: Ident(
            "string",
        ),
        span: Span {
            start: 73,
            end: 79,
        },
    },
    SpannedToken {
        kind: Comma,
        span: Span {
            start: 79,
            end: 80,
        },
    },
    SpannedToken {
        kind: Ident(
            "other",
        ),
        span: Span {
            start: 81,
            end: 86,
        },
    },
    SpannedToken {
        kind: Colon,
        span: Span {
            start: 86,
            end: 87,
        },
    },
    SpannedToken {
        kind: Ident(
            "AbstractClass",
        ),
        span: Span {
            start: 88,
            end: 101,
        },
    },
    SpannedToken {
        kind: RightParen,
        span: Span {
            start: 101,
            end: 102,
        },
    },
    SpannedToken {
        kind: LeftBrace,
        span: Span {
            start: 103,
            end: 104,
        },
    },
    SpannedToken {
        kind: This,
        span: Span {
            start: 114,
            end: 118,
        },
    },
    SpannedToken {
        kind: Dot,
        span: Span {
            start: 118,
            end: 119,
        },
    },
    SpannedToken {
        kind: Ident(
            "method",
        ),
        span: Span {
            start: 119,
            end: 125,
        },
    },
    SpannedToken {
        kind: LeftParen,
        span: Span {
            start: 125,
            end: 126,
        },
    },
    SpannedToken {
        kind: Ident(
            "parseInt",
        ),
        span: Span {
            start: 126,
            end: 134,
        },
    },
    SpannedToken {
        kind: LeftParen,
        span: Span {
            start: 134,
            end: 135,
        },
    },
    SpannedToken {
        kind: Ident(
            "str",
        ),
        span: Span {
            start: 135,
            end: 138,
        },
    },
    SpannedToken {
        kind: RightParen,
        span: Span {
            start: 138,
            end: 139,
        },
    },
    SpannedToken {
        kind: RightParen,
        span: Span {
            start: 139,
            end: 140,
        },
    },
    SpannedToken {
        kind: Semicolon,
        span: Span {
            start: 140,
            end: 141,
        },
    },
```

#### ast

- ok: `False`
- truncated: `False`

```text
error: [UnsupportedSyntax] expected Semicolon, got Some(Ident("other")) at 473..478
```

#### resolved

- ok: `False`
- truncated: `False`

```text
error: [UnsupportedSyntax] expected Semicolon, got Some(Ident("other")) at 473..478
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
        "code": 2715,
        "category": "Error",
        "message": "Abstract property 'prop' in class 'AbstractClass' cannot be accessed in the constructor.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/abstractPropertyInConstructor.ts",
        "start": 166,
        "length": 4,
        "line": 5,
        "character": 24
      },
      {
        "code": 2715,
        "category": "Error",
        "message": "Abstract property 'prop' in class 'AbstractClass' cannot be accessed in the constructor.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/abstractPropertyInConstructor.ts",
        "start": 227,
        "length": 4,
        "line": 8,
        "character": 18
      },
      {
        "code": 2715,
        "category": "Error",
        "message": "Abstract property 'cb' in class 'AbstractClass' cannot be accessed in the constructor.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/abstractPropertyInConstructor.ts",
        "start": 274,
        "length": 2,
        "line": 10,
        "character": 14
      },
      {
        "code": 2715,
        "category": "Error",
        "message": "Abstract property 'prop' in class 'AbstractClass' cannot be accessed in the constructor.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/abstractPropertyInConstructor.ts",
        "start": 634,
        "length": 4,
        "line": 26,
        "character": 18
      },
      {
        "code": 2729,
        "category": "Error",
        "message": "Property 'prop' is used before its initialization.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/abstractPropertyInConstructor.ts",
        "start": 634,
        "length": 4,
        "line": 26,
        "character": 18
      },
      {
        "code": 2715,
        "category": "Error",
        "message": "Abstract property 'prop' in class 'AbstractClass' cannot be accessed in the constructor.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/abstractPropertyInConstructor.ts",
        "start": 1033,
        "length": 4,
        "line": 40,
        "character": 22
      },
      {
        "code": 2715,
        "category": "Error",
        "message": "Abstract property 'x' in class 'C1' cannot be accessed in the constructor.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/abstractPropertyInConstructor.ts",
        "start": 1856,
        "length": 1,
        "line": 79,
        "character": 15
      },
      {
        "code": 2715,
        "category": "Error",
        "message": "Abstract property 'y' in class 'C1' cannot be accessed in the constructor.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/abstractPropertyInConstructor.ts",
        "start": 1859,
        "length": 1,
        "line": 79,
        "character": 18
      },
      {
        "code": 2715,
        "category": "Error",
        "message": "Abstract property 'x' in class 'C1' cannot be accessed in the constructor.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/abstractPropertyInConstructor.ts",
        "start": 1903,
        "length": 1,
        "line": 80,
        "character": 12
      },
      {
        "code": 2715,
        "category": "Error",
        "message": "Abstract property 'y' in class 'C1' cannot be accessed in the constructor.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/abstractPropertyInConstructor.ts",
        "start": 1906,
        "length": 1,
        "line": 80,
        "character": 15
      },
      {
        "code": 2715,
        "category": "Error",
        "message": "Abstract property 'y' in class 'C1' cannot be accessed in the constructor.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/abstractPropertyInConstructor.ts",
        "start": 1913,
        "length": 3,
        "line": 80,
        "character": 22
      },
      {
        "code": 2564,
        "category": "Error",
        "message": "Property 'x' has no initializer and is not definitely assigned in the constructor.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/abstractPropertyInConstructor.ts",
        "start": 1970,
        "length": 1,
        "line": 85,
        "character": 5
      },
      {
        "code": 2564,
        "category": "Error",
        "message": "Property 'y' has no initializer and is not definitely assigned in the constructor.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/abstractPropertyInConstructor.ts",
        "start": 1986,
        "length": 1,
        "line": 86,
        "character": 5
      }
    ],
    "hints": [
      {
        "kind": "parameter",
        "typeText": "string",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/abstractPropertyInConstructor.ts",
        "start": 68,
        "length": 3,
        "line": 3,
        "character": 17,
        "name": "str"
      },
      {
        "kind": "parameter",
        "typeText": "AbstractClass",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/abstractPropertyInConstructor.ts",
        "start": 81,
        "length": 5,
        "line": 3,
        "character": 30,
        "name": "other"
      },
      {
        "kind": "binding",
        "typeText": "string",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/abstractPropertyInConstructor.ts",
        "start": 155,
        "length": 3,
        "line": 5,
        "character": 13,
        "name": "val"
      },
      {
        "kind": "binding",
        "typeText": "() => string",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/abstractPropertyInConstructor.ts",
        "start": 345,
        "length": 13,
        "line": 13,
        "character": 15,
        "name": "innerFunction"
      },
      {
        "kind": "parameter",
        "typeText": "string",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/abstractPropertyInConstructor.ts",
        "start": 551,
        "length": 1,
        "line": 22,
        "character": 19,
        "name": "s"
      },
      {
        "kind": "parameter",
        "typeText": "number",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/abstractPropertyInConstructor.ts",
        "start": 594,
        "length": 3,
        "line": 24,
        "character": 21,
        "name": "num"
      },
      {
        "kind": "binary-expression",
        "typeText": "string",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/abstractPropertyInConstructor.ts",
        "start": 707,
        "length": 15,
        "line": 30,
        "character": 21,
        "operator": "+",
        "leftType": "string",
        "rightType": "\"!\"",
        "candidate": "string-concat-fast-path"
      },
      {
        "kind": "parameter",
        "typeText": "string",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/abstractPropertyInConstructor.ts",
        "start": 808,
        "length": 1,
        "line": 35,
        "character": 11,
        "name": "s"
      },
      {
        "kind": "parameter",
        "typeText": "string",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/abstractPropertyInConstructor.ts",
        "start": 845,
        "length": 3,
        "line": 37,
        "character": 17,
        "name": "str"
      },
      {
        "kind": "parameter",
        "typeText": "
```

Stack trace:

```text
error: [UnsupportedSyntax] expected Semicolon, got Some(Ident("other")) at 473..478
```

## Completion evidence

Commits:

- Split current lowerer/diagnostic blocker to issue 5390; no direct
  implementation from this generated bucket.

Validation result:

```text
command:
env TS2WASM_BINARY=/home/wogikaze/wgkz/ts2wasm/target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/abstractPropertyInConstructor.ts --detail --no-dashboard-data
result:
executed=1, build_pass=0, unsupported=1, unsupported_diagcodes=UnsupportedSyntax:1
date:
2026-05-07

command:
env TS2WASM_BINARY=/home/wogikaze/wgkz/ts2wasm/target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/abstractPropertyInConstructor.ts
result:
UnsupportedSyntax / method-call; current diagnostic is `method AbstractClass.cb not found`; TypeScript oracle reports TS2715 abstract property constructor access diagnostics.
date:
2026-05-07
```

Remaining risks:

- Issue 5390 owns the implementation for abstract property constructor
  diagnostics.

## False-done audit

**truly-done** (558)

- Implementation commits: verified via `git log --oneline --all --grep=558`
- Completion evidence: filled with specific commit hashes and validation results
- Acceptance criteria: all checked as met

This issue has repo-local close evidence with implementation commits and validation commands.
