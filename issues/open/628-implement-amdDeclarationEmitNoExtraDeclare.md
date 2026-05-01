---
id: 628
title: "Implement Amddeclarationemitnoextradeclare"
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

Triage amdDeclarationEmitNoExtraDeclare across 1 failing reference test cases and split this bucket into implementation-ready child issues.

## Problem

Reference test results show 1 cases fail in directory `amdDeclarationEmitNoExtraDeclare` with diagnostics: declaration-emit. The compiler cannot handle these syntax/semantics, preventing compilation of code in this category.

Problem: amdDeclarationEmitNoExtraDeclare has 1 reference failures and needs smart-triage evidence before implementation starts.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/amdDeclarationEmitNoExtraDeclare.ts
```

Coverage window:

```sh
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/amdDeclarationEmitNoExtraDeclare.ts --detail
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
mise run reference-coverage -- tsc --limit 2
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/amdDeclarationEmitNoExtraDeclare.ts --detail
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/amdDeclarationEmitNoExtraDeclare.ts
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

- `reference/typescript/tests/cases/compiler/amdDeclarationEmitNoExtraDeclare.ts`

## Duplicate detection

- `issues/open/170-implement-amdDeclarationEmitNoExtraDeclare.md` - Implement Amddeclarationemitnoextradeclare (same reference path, same feature label, same group key, title overlap)
- `issues/open/424-implement-declaration-emit.md` - Implement declaration-emit support (same feature label, same group key, title overlap)

## Smart triage

### Smart triage: Triage declaration emit: amdDeclarationEmitNoExtraDeclare

- Issue class: `triage-needed`
- Feature label: `declaration-emit`
- Diagnostic: `UnsupportedSyntax` / `parser-or-frontend-unsupported`
- Path: `reference/typescript/tests/cases/compiler/amdDeclarationEmitNoExtraDeclare.ts`

Reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/amdDeclarationEmitNoExtraDeclare.ts
```

Source overview:

```json
{
  "suite": "tsc",
  "bytes": 519,
  "lines": 23,
  "extension": ".ts",
  "first_code_line": "import { Configurable } from \"./Configurable\""
}
```

Failure location:

```json
{
  "code": "UnsupportedSyntax",
  "message": "expected Semicolon, got Some(Export) at 155..161",
  "span_start": 155,
  "span_end": 161,
  "line": 8,
  "column": 8,
  "feature_label": "declaration-emit",
  "error_type": "parser-or-frontend-unsupported"
}
```

Source context:

```text
 5 | // @filename: Class.ts
 6 | import { Configurable } from "./Configurable"
 7 | 
 8 | export class HiddenClass {}
 9 | 
10 | export class ActualClass extends Configurable(HiddenClass) {}
11 | // @filename: Configurable.ts
```

Visible symbols before failure:

```json
[
  {
    "kind": "import",
    "name": "./Configurable",
    "line": 6,
    "column": 1
  }
]
```

Duplicate candidates:

```json
[
  {
    "state": "open",
    "path": "issues/open/170-implement-amdDeclarationEmitNoExtraDeclare.md",
    "title": "Implement Amddeclarationemitnoextradeclare",
    "reason": "same reference path, same feature label, title overlap"
  },
  {
    "state": "open",
    "path": "issues/open/408-implement-tsgo-declaration-emit-asconst-and-generic-methods.md",
    "title": "Implement tsgo declaration emit: AsConstSatisfies/const generic method cases",
    "reason": "same feature label, title overlap"
  },
  {
    "state": "open",
    "path": "issues/open/409-implement-tsgo-declaration-emit-package-json-subpath.md",
    "title": "Implement tsgo declaration emit: package-json exports and subpath reexport cases",
    "reason": "same feature label, title overlap"
  },
  {
    "state": "open",
    "path": "issues/done/410-implement-tsgo-declaration-emit-subpath-import-links.md",
    "title": "Implement tsgo declaration emit: subpath import declaration emit cases",
    "reason": "same feature label, title overlap"
  },
  {
    "state": "done",
    "path": "issues/done/346-implement-tsgo-declaration-emit.md",
    "title": "Implement TypeScript declaration emit coverage for tsgo suite (16 cases)",
    "reason": "same feature label, title overlap"
  },
  {
    "state": "done",
    "path": "issues/done/399-define-typescript-parse-erase-emit-boundary.md",
    "title": "Define TypeScript parse, erase, and emit boundary contract",
    "reason": "same feature label, title overlap"
  },
  {
    "state": "done",
    "path": "issues/done/400-implement-ambient-declaration-erasure-boundary.md",
    "title": "Implement ambient declaration erasure and rejection boundary",
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
        kind: Import,
        span: Span {
            start: 106,
            end: 112,
        },
    },
    SpannedToken {
        kind: LeftBrace,
        span: Span {
            start: 113,
            end: 114,
        },
    },
    SpannedToken {
        kind: Ident(
            "Configurable",
        ),
        span: Span {
            start: 115,
            end: 127,
        },
    },
    SpannedToken {
        kind: RightBrace,
        span: Span {
            start: 128,
            end: 129,
        },
    },
    SpannedToken {
        kind: Ident(
            "from",
        ),
        span: Span {
            start: 130,
            end: 134,
        },
    },
    SpannedToken {
        kind: String(
            "./Configurable",
        ),
        span: Span {
            start: 135,
            end: 151,
        },
    },
    SpannedToken {
        kind: Export,
        span: Span {
            start: 155,
            end: 161,
        },
    },
    SpannedToken {
        kind: Class,
        span: Span {
            start: 162,
            end: 167,
        },
    },
    SpannedToken {
        kind: Ident(
            "HiddenClass",
        ),
        span: Span {
            start: 168,
            end: 179,
        },
    },
    SpannedToken {
        kind: LeftBrace,
        span: Span {
            start: 180,
            end: 181,
        },
    },
    SpannedToken {
        kind: RightBrace,
        span: Span {
            start: 181,
            end: 182,
        },
    },
    SpannedToken {
        kind: Export,
        span: Span {
            start: 186,
            end: 192,
        },
    },
    SpannedToken {
        kind: Class,
        span: Span {
            start: 193,
            end: 198,
        },
    },
    SpannedToken {
        kind: Ident(
            "ActualClass",
        ),
        span: Span {
            start: 199,
            end: 210,
        },
    },
    SpannedToken {
        kind: Extends,
        span: Span {
            start: 211,
            end: 218,
        },
    },
    SpannedToken {
        kind: Ident(
            "Configurable",
        ),
        span: Span {
            start: 219,
            end: 231,
        },
    },
    SpannedToken {
        kind: LeftParen,
        span: Span {
            start: 231,
            end: 232,
        },
    },
    SpannedToken {
        kind: Ident(
            "HiddenClass",
        ),
        span: Span {
            start: 232,
            end: 243,
        },
    },
    SpannedToken {
        kind: RightParen,
        span: Span {
            start: 243,
            end: 244,
        },
    },
    SpannedToken {
        kind: LeftBrace,
        span: Span {
            start: 245,
            end: 246,
        },
    },
    SpannedToken {
        kind: RightBrace,
        span: Span {
            start: 246,
            end: 247,
        },
    },
    SpannedToken {
        kind: Export,
        span: Span {
            start: 280,
            end: 286,
        },
    },
    SpannedToken {
        kind: Ident(
            "type",
        ),
        span: Span {
            start: 287,
            end: 291,
        },
    },
    SpannedToken {
        kind: Ident(
            "Constructor",
        ),
        span: Span {
            start: 292,
            end: 303,
        },
    },
    SpannedToken {
        kind: Less,
        span: Span {
            start: 303,
            end: 304,
```

#### ast

- ok: `False`
- truncated: `False`

```text
error: [UnsupportedSyntax] expected Semicolon, got Some(Export) at 155..161
```

#### resolved

- ok: `False`
- truncated: `False`

```text
error: [UnsupportedSyntax] expected Semicolon, got Some(Export) at 155..161
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
        "code": 2307,
        "category": "Error",
        "message": "Cannot find module './Configurable' or its corresponding type declarations.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/amdDeclarationEmitNoExtraDeclare.ts",
        "start": 135,
        "length": 16,
        "line": 6,
        "character": 30
      }
    ],
    "hints": [
      {
        "kind": "parameter",
        "typeText": "any[]",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/amdDeclarationEmitNoExtraDeclare.ts",
        "start": 323,
        "length": 4,
        "line": 13,
        "character": 12,
        "name": "args"
      },
      {
        "kind": "function",
        "typeText": "T",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/amdDeclarationEmitNoExtraDeclare.ts",
        "start": 360,
        "length": 12,
        "line": 15,
        "character": 17,
        "name": "Configurable"
      },
      {
        "kind": "parameter",
        "typeText": "T",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/amdDeclarationEmitNoExtraDeclare.ts",
        "start": 400,
        "length": 4,
        "line": 15,
        "character": 57,
        "name": "base"
      },
      {
        "kind": "parameter",
        "typeText": "any[]",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/amdDeclarationEmitNoExtraDeclare.ts",
        "start": 473,
        "length": 4,
        "line": 18,
        "character": 24,
        "name": "args"
      }
    ],
    "typescriptVersion": "6.0.3"
  },
  "ast": {
    "topLevel": [
      {
        "kind": "ImportDeclaration",
        "text": "import { Configurable } from \"./Configurable\"",
        "line": 6,
        "character": 1
      },
      {
        "kind": "ClassDeclaration",
        "text": "export class HiddenClass {}",
        "line": 8,
        "character": 1
      },
      {
        "kind": "ClassDeclaration",
        "text": "export class ActualClass extends Configurable(HiddenClass) {}",
        "line": 10,
        "character": 1
      },
      {
        "kind": "TypeAliasDeclaration",
        "text": "export type Constructor<T> = {\r\n    new(...args: any[]): T;\r\n}",
        "line": 12,
        "character": 1
      },
      {
        "kind": "FunctionDeclaration",
        "text": "export function Configurable<T extends Constructor<{}>>(base: T): T {\r\n    return class extends base {\r\n\r\n        constr",
        "line": 15,
        "character": 1
      }
    ],
    "pathToPosition": [
      {
        "kind": "SourceFile",
        "text": "import { Configurable } from \"./Configurable\"\r\n\r\nexport class HiddenClass {}\r\n\r\nexport class ActualClass extends Configu",
        "line": 6,
        "character": 1
      },
      {
        "kind": "ClassDeclaration",
        "text": "export class HiddenClass {}",
        "line": 8,
        "character": 1
      },
      {
        "kind": "ExportKeyword",
        "text": "export",
        "line": 8,
        "character": 1
      }
    ]
  }
}
```

Stack trace:

```text
error: [UnsupportedSyntax] expected Semicolon, got Some(Export) at 155..161
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
