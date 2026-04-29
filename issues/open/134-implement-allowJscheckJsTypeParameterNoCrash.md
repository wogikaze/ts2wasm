---
id: 134
title: "Implement Allowjscheckjstypeparameternocrash"
type: spike
area: reference
class: triage-needed
priority: P1
depends_on: []
blocks: []
created: 2026-04-29
updated: 2026-04-29
---

## Summary

Triage allowJscheckJsTypeParameterNoCrash across 1 failing reference test cases and split this bucket into implementation-ready child issues.

## Problem

Reference test results show 1 cases fail in directory `allowJscheckJsTypeParameterNoCrash` with diagnostics: parser-syntax. The compiler cannot handle these syntax/semantics, preventing compilation of code in this category.

Problem: allowJscheckJsTypeParameterNoCrash has 1 reference failures and needs smart-triage evidence before implementation starts.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/allowJscheckJsTypeParameterNoCrash.ts
```

Coverage window:

```sh
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/allowJscheckJsTypeParameterNoCrash.ts --detail
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

- unrelated runtime/backend code unless the triage report proves the failure is not parser/frontend

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
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/allowJscheckJsTypeParameterNoCrash.ts --detail
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/allowJscheckJsTypeParameterNoCrash.ts
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

- `reference/typescript/tests/cases/compiler/allowJscheckJsTypeParameterNoCrash.ts`

## Duplicate detection

## Smart triage

### Smart triage: Triage parser syntax: allowJscheckJsTypeParameterNoCrash

- Issue class: `triage-needed`
- Feature label: `parser-syntax`
- Diagnostic: `UnsupportedSyntax` / `parser-or-frontend-unsupported`
- Path: `reference/typescript/tests/cases/compiler/allowJscheckJsTypeParameterNoCrash.ts`

Reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/allowJscheckJsTypeParameterNoCrash.ts
```

Source overview:

```json
{
  "suite": "tsc",
  "bytes": 529,
  "lines": 24,
  "extension": ".ts",
  "first_code_line": "interface ComponentOptions<V> {"
}
```

Failure location:

```json
{
  "code": "UnsupportedSyntax",
  "message": "expected Equal, got Some(Less) at 241..242",
  "span_start": 241,
  "span_end": 242,
  "line": 11,
  "column": 27,
  "feature_label": "parser-syntax",
  "error_type": "parser-or-frontend-unsupported"
}
```

Source context:

```text
 8 | interface ComponentOptions<V> {
 9 |     watch: Record<string, WatchHandler<any>>;
10 | }
11 | type WatchHandler<T> = (val: T) => void;
12 | declare function extend(options: ComponentOptions<{}>): void;
13 | export var vextend = extend;
14 | // @filename: app.js
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
    "path": "issues/open/059-implement-parser-syntax-extensions.md",
    "title": "Implement parser syntax extensions for TypeScript and advanced JS",
    "reason": "same feature label, title overlap"
  },
  {
    "state": "open",
    "path": "issues/open/134-implement-allowJscheckJsTypeParameterNoCrash.md",
    "title": "Implement Allowjscheckjstypeparameternocrash",
    "reason": "same reference path, same feature label, title overlap"
  },
  {
    "state": "open",
    "path": "issues/open/200-implement-parser-syntax.md",
    "title": "Implement parser syntax extensions",
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
            "interface",
        ),
        span: Span {
            start: 141,
            end: 150,
        },
    },
    SpannedToken {
        kind: Ident(
            "ComponentOptions",
        ),
        span: Span {
            start: 151,
            end: 167,
        },
    },
    SpannedToken {
        kind: Less,
        span: Span {
            start: 167,
            end: 168,
        },
    },
    SpannedToken {
        kind: Ident(
            "V",
        ),
        span: Span {
            start: 168,
            end: 169,
        },
    },
    SpannedToken {
        kind: Greater,
        span: Span {
            start: 169,
            end: 170,
        },
    },
    SpannedToken {
        kind: LeftBrace,
        span: Span {
            start: 171,
            end: 172,
        },
    },
    SpannedToken {
        kind: Ident(
            "watch",
        ),
        span: Span {
            start: 178,
            end: 183,
        },
    },
    SpannedToken {
        kind: Colon,
        span: Span {
            start: 183,
            end: 184,
        },
    },
    SpannedToken {
        kind: Ident(
            "Record",
        ),
        span: Span {
            start: 185,
            end: 191,
        },
    },
    SpannedToken {
        kind: Less,
        span: Span {
            start: 191,
            end: 192,
        },
    },
    SpannedToken {
        kind: Ident(
            "string",
        ),
        span: Span {
            start: 192,
            end: 198,
        },
    },
    SpannedToken {
        kind: Comma,
        span: Span {
            start: 198,
            end: 199,
        },
    },
    SpannedToken {
        kind: Ident(
            "WatchHandler",
```

#### ast

- ok: `False`
- truncated: `False`

```text
error: [UnsupportedSyntax] expected Equal, got Some(Less) at 241..242
```

#### resolved

- ok: `False`
- truncated: `False`

```text
error: [UnsupportedSyntax] expected Equal, got Some(Less) at 241..242
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
        "code": 2395,
        "category": "Error",
        "message": "Individual declarations in merged declaration 'vextend' must be all exported or all local.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/allowJscheckJsTypeParameterNoCrash.ts",
        "start": 340,
        "length": 7,
        "line": 13,
        "character": 12
      },
      {
        "code": 2395,
        "category": "Error",
        "message": "Individual declarations in merged declaration 'vextend' must be all exported or all local.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/allowJscheckJsTypeParameterNoCrash.ts",
        "start": 389,
        "length": 7,
        "line": 15,
        "character": 9
      },
      {
        "code": 2307,
        "category": "Error",
        "message": "Cannot find module './func' or its corresponding type declarations.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/allowJscheckJsTypeParameterNoCrash.ts",
        "start": 403,
        "length": 8,
        "line": 15,
        "character": 23
      },
      {
        "code": 2322,
        "category": "Error",
        "message": "Type 'number' is not assignable to type 'WatchHandler<any>'.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/allowJscheckJsTypeParameterNoCrash.ts",
        "start": 497,
        "length": 10,
        "line": 20,
        "character": 7
      }
    ],
    "hints": [
      {
        "kind": "parameter",
        "typeText": "T",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/allowJscheckJsTypeParameterNoCrash.ts",
        "start": 248,
        "length": 3,
        "line": 11,
        "character": 25,
        "name": "val"
      },
      {
        "kind": "function",
        "typeText": "void",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/allowJscheckJsTypeParameterNoCrash.ts",
        "start": 283,
        "length": 6,
        "line": 12,
        "character": 18,
        "name": "extend"
      },
      {
        "kind": "parameter",
        "typeText": "ComponentOptions<{}>",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/allowJscheckJsTypeParameterNoCrash.ts",
        "start": 290,
        "length": 7,
        "line": 12,
        "character": 25,
        "name": "options"
      },
      {
        "kind": "binding",
        "typeText": "(options: ComponentOptions<{}>) => void",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/allowJscheckJsTypeParameterNoCrash.ts",
        "start": 340,
        "length": 7,
        "line": 13,
        "character": 12,
        "name": "vextend"
      },
      {
        "kind": "binding",
        "typeText": "void",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/allowJscheckJsTypeParameterNoCrash.ts",
        "start": 446,
        "length": 1,
        "line": 17,
        "character": 12,
        "name": "a"
      },
      {
        "kind": "parameter",
        "typeText": "any",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/allowJscheckJsTypeParameterNoCrash.ts",
        "start": 483,
        "length": 3,
        "line": 19,
        "character": 11,
        "name": "val"
      },
      {
        "kind": "parameter",
        "typeText": "any",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/allowJscheckJsTypeParameterNoCrash.ts",
        "start": 532,
        "length": 3,
        "line": 22,
        "character": 11,
        "name": "val"
      }
    ],
    "typescriptVersion": "6.0.3"
  },
  "ast": {
    "topLevel": [
      {
        "kind": "InterfaceDeclaration",
        "text": "interface ComponentOptions<V> {\r\n    watch: Record<string, WatchHandler<any>>;\r\n}",
        "line": 8,
        "character": 1
      },
      {
        "kind": "TypeAliasDeclaration",
        "text": "type WatchHandler<T> = (val: T) => void;",
        "line": 11,
        "character": 1
      },
      {
        "kind": "FunctionDeclaration",
        "text": "declare function extend(options: ComponentOptions<{}>): void;",
        "line": 12,
        "character": 1
      },
      {
        "kind": "FirstStatement",
        "text": "export var vextend = extend;",
        "line": 13,
        "character": 1
      },
      {
        "kind": "ImportDeclaration",
        "text": "import {vextend} from './func';",
        "line": 15,
        "character": 1
      },
      {
        "kind": "FirstStatement",
        "text": "export var a = vextend({\r\n  watch: {\r\n    data1(val) {\r\n      this.data2 = 1;\r\n    },\r\n    data2(val) { },\r\n  }\r\n});",
        "line": 17,
        "character": 1
      }
    ],
    "pathToPosition": [
      {
        "kind": "SourceFile",
        "text": "interface ComponentOptions<V> {\r\n    watch: Record<string, WatchHandler<any>>;\r\n}\r\ntype WatchHandler<T> = (val: T) => vo",
        "line": 8,
        "character": 1
      },
      {
        "kind": "TypeAliasDeclaration",
        "text": "type WatchHandler<T> = (val: T) => void;",
        "line": 11,
        "character": 1
      },
      {
        "kind": "Identifier",
        "text": "WatchHandler",
        "line": 11,
        "character": 6
      }
    ]
  }
}
```

Stack trace:

```text
error: [UnsupportedSyntax] expected Equal, got Some(Less) at 241..242
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
