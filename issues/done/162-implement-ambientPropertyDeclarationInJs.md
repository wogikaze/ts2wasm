---
id: 162
title: "Implement Ambientpropertydeclarationinjs"
type: spike
area: frontend/syntax
class: superseded
priority: P1
depends_on: []
blocks: []
created: 2026-04-29
updated: 2026-05-08
completed: 2026-05-08
status: done
---

## Summary

Triage ambientPropertyDeclarationInJs across 1 failing reference test cases and split this bucket into implementation-ready child issues.

## Problem

Fresh reference evidence shows this bucket now builds successfully in coverage.

Problem: this generated bucket is no longer an ambient-declaration blocker; the
class element `declare prop: string;` erasure boundary is covered by issue 400.
Fresh smart triage now reports the separate JS noEmit class constructor FuncId
invariant, already tracked by issue 5247.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/ambientPropertyDeclarationInJs.ts
```

Coverage window:

```sh
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/ambientPropertyDeclarationInJs.ts --detail
```

## Desired final state

This generated bucket is closed as superseded. The ambient class element erasure
behavior is covered by issue 400, and the residual compiler invariant is owned
by `issues/open/5247-fix-js-noemit-class-constructor-funcid-invariant.md`.
Do not implement directly from this bucket.

## Scope

In scope:

- [x] Inspect the smart triage report below
- [x] Confirm whether existing open/done issues already cover this bucket
- [x] Confirm residual invariant is already owned by issue 5247
- [x] Preserve exact reproduction commands and representative AST/diagnostic evidence in completion evidence

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

- [x] Duplicate candidates below are confirmed as this issue is superseded
- [x] Existing issue 5247 contains an exact `python scripts/manager.py reference-triage ...` command for the same invariant family
- [x] Completion evidence includes path, diagnostic code, source context, visible symbols, and parser/TypeScript AST evidence
- [x] Existing issue 5247 acceptance names the invariant diagnostic boundary

## Validation

Required commands:

```sh
python scripts/manager.py update-issue-index --check
python scripts/manager.py check-issue-health
python scripts/manager.py check-issue-readiness -- --fail-ready-below 80
git diff --check
```

Impacted commands:

```sh
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/ambientPropertyDeclarationInJs.ts --detail
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/ambientPropertyDeclarationInJs.ts
```

Not run:

- `cargo fmt --all --check`; issue cleanup only, no Rust code changed
- `cargo nextest run`; issue cleanup only, no implementation changed

## Docs / current-state / issue sync

Final-state docs:

- [x] not affected

Current state:

- [x] not affected

Follow-up issues:

- [x] existing owner: `issues/open/5247-fix-js-noemit-class-constructor-funcid-invariant.md`

## Notes

## Affected test files

- `reference/typescript/tests/cases/compiler/ambientPropertyDeclarationInJs.ts`

## Duplicate detection

## Smart triage

Fresh coverage on 2026-05-08 shows this generated ambient declaration bucket now
builds successfully:

```text
reference/typescript/tests/cases/compiler/ambientPropertyDeclarationInJs.ts: build_pass
```

Focused triage reports a different residual blocker:

```text
InvariantViolation: ClassDecl constructor FuncId 0 is out of range (program has 0 function(s))
```

Representative source context:

```ts
class Foo {
    constructor() {
        this.prop = {};
    }

    declare prop: string;

    method() {
        this.prop.foo
    }
}
```

The compiler now parses `declare prop: string;` inside the class and erases that
ambient class element. The residual invariant is the same JS noEmit class
constructor FuncId family tracked by
`issues/open/5247-fix-js-noemit-class-constructor-funcid-invariant.md`.

### Smart triage: Triage ambient declaration: ambientPropertyDeclarationInJs

- Issue class: `triage-needed`
- Feature label: `ambient-declaration`
- Diagnostic: `UnsupportedSyntax` / `parser-or-frontend-unsupported`
- Path: `reference/typescript/tests/cases/compiler/ambientPropertyDeclarationInJs.ts`

Reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/ambientPropertyDeclarationInJs.ts
```

Source overview:

```json
{
  "suite": "tsc",
  "bytes": 231,
  "lines": 17,
  "extension": ".ts",
  "first_code_line": "class Foo {"
}
```

Failure location:

```json
{
  "code": "UnsupportedSyntax",
  "message": "expected LeftParen, got Some(Ident(\"prop\")) at 171..175",
  "span_start": 171,
  "span_end": 175,
  "line": 12,
  "column": 13,
  "feature_label": "ambient-declaration",
  "error_type": "parser-or-frontend-unsupported"
}
```

Source context:

```text
 9 |         this.prop = {};
10 |     }
11 |
12 |     declare prop: string;
13 |
14 |     method() {
15 |         this.prop.foo
```

Visible symbols before failure:

```json
[
  {
    "kind": "class",
    "name": "Foo",
    "line": 7,
    "column": 1
  }
]
```

Duplicate candidates:

```json
[
  {
    "state": "done",
    "path": "issues/done/162-implement-ambientPropertyDeclarationInJs.md",
    "title": "Implement Ambientpropertydeclarationinjs",
    "reason": "same reference path, title overlap"
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
            start: 96,
            end: 101,
        },
    },
    SpannedToken {
        kind: Ident(
            "Foo",
        ),
        span: Span {
            start: 102,
            end: 105,
        },
    },
    SpannedToken {
        kind: LeftBrace,
        span: Span {
            start: 106,
            end: 107,
        },
    },
    SpannedToken {
        kind: Ident(
            "constructor",
        ),
        span: Span {
            start: 112,
            end: 123,
        },
    },
    SpannedToken {
        kind: LeftParen,
        span: Span {
            start: 123,
            end: 124,
        },
    },
    SpannedToken {
        kind: RightParen,
        span: Span {
            start: 124,
            end: 125,
        },
    },
    SpannedToken {
        kind: LeftBrace,
        span: Span {
            start: 126,
            end: 127,
        },
    },
    SpannedToken {
        kind: This,
        span: Span {
            start: 136,
            end: 140,
        },
    },
    SpannedToken {
        kind: Dot,
        span: Span {
            start: 140,
            end: 141,
        },
    },
    SpannedToken {
        kind: Ident(
            "prop",
        ),
        span: Span {
            start: 141,
            end: 145,
        },
    },
    SpannedToken {
        kind: Equal,
        span: Span {
            start: 146,
            end: 147,
        },
    },
    SpannedToken {
        kind: LeftBrace,
        span: Span {
            start: 148,
            end: 149,
        },
    },
    SpannedToken {
        kind: RightBrace,
        span: Span {
            start: 149,
            end: 150,
        },
    },
    SpannedToken {
        kind: Semico
```

#### ast

- ok: `False`
- truncated: `False`

```text
error: [UnsupportedSyntax] expected LeftParen, got Some(Ident("prop")) at 171..175
```

#### resolved

- ok: `False`
- truncated: `False`

```text
error: [UnsupportedSyntax] expected LeftParen, got Some(Ident("prop")) at 171..175
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
        "code": 2322,
        "category": "Error",
        "message": "Type '{}' is not assignable to type 'string'.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/ambientPropertyDeclarationInJs.ts",
        "start": 136,
        "length": 9,
        "line": 9,
        "character": 9
      },
      {
        "code": 2339,
        "category": "Error",
        "message": "Property 'foo' does not exist on type 'string'.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/ambientPropertyDeclarationInJs.ts",
        "start": 219,
        "length": 3,
        "line": 15,
        "character": 19
      }
    ],
    "hints": [],
    "typescriptVersion": "6.0.3"
  },
  "ast": {
    "topLevel": [
      {
        "kind": "ClassDeclaration",
        "text": "class Foo {\n    constructor() {\n        this.prop = {};\n    }\n\n    declare prop: string;\n\n    method() {\n        this.pr",
        "line": 7,
        "character": 1
      }
    ],
    "pathToPosition": [
      {
        "kind": "SourceFile",
        "text": "class Foo {\n    constructor() {\n        this.prop = {};\n    }\n\n    declare prop: string;\n\n    method() {\n        this.pr",
        "line": 7,
        "character": 1
      },
      {
        "kind": "ClassDeclaration",
        "text": "class Foo {\n    constructor() {\n        this.prop = {};\n    }\n\n    declare prop: string;\n\n    method() {\n        this.pr",
        "line": 7,
        "character": 1
      },
      {
        "kind": "PropertyDeclaration",
        "text": "declare prop: string;",
        "line": 12,
        "character": 5
      },
      {
        "kind": "Identifier",
        "text": "prop",
        "line": 12,
        "character": 13
      }
    ]
  }
}
```

Stack trace:

```text
error: [UnsupportedSyntax] expected LeftParen, got Some(Ident("prop")) at 171..175
```

## Completion evidence

Closed after fresh triage confirmed the generated ambient property parser
blocker is resolved by issue 400 and the residual compiler invariant already
has an open owner in issue 5247.

Fresh coverage:

```text
command: env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/ambientPropertyDeclarationInJs.ts --detail --no-dashboard-data
result: pass; executed=1, build_pass=1, unsupported=0
date: 2026-05-08
```

Fresh triage:

```text
command: env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/ambientPropertyDeclarationInJs.ts
result: pass; residual `InvariantViolation: ClassDecl constructor FuncId 0 is out of range (program has 0 function(s))`
date: 2026-05-08
```

The ambient class element declaration itself now parses and erases. The
residual invariant belongs to
`issues/open/5247-fix-js-noemit-class-constructor-funcid-invariant.md`.

Commits:

- local issue cleanup commit that moves issue 162 to done and updates issue 400 references

Validation result:

```text
command: python scripts/manager.py update-issue-index --check
result: pass
date: 2026-05-08

command: python scripts/manager.py check-issue-health
result: pass
date: 2026-05-08

command: python scripts/manager.py check-issue-readiness -- --fail-ready-below 80
result: pass
date: 2026-05-08

command: git diff --check
result: pass
date: 2026-05-08
```

Remaining risks:

- Residual JS noEmit class constructor FuncId invariant remains open in issue 5247.

## False-done audit resolution

This issue was re-triaged on 2026-05-08 with fresh coverage and smart triage
evidence. The generated ambient-declaration blocker is closed because the
original parser failure is fixed, and the remaining non-ambient compiler
invariant already has a focused implementation-ready owner.
