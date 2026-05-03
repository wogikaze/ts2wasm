---
id: 579
title: "Implement Aliasdoesnotduplicatesignatures"
type: spike
area: frontend/syntax
class: blocked
priority: P1
depends_on: [5001]
blocks: []
created: 2026-05-01
updated: 2026-05-01
---

## Summary

Triage aliasDoesNotDuplicateSignatures across 1 failing reference test cases and split this bucket into implementation-ready child issues.

## Problem

Reference test results show 1 cases fail in directory `aliasDoesNotDuplicateSignatures` with diagnostics: import-export. The compiler cannot handle these syntax/semantics, preventing compilation of code in this category.

Problem: aliasDoesNotDuplicateSignatures has 1 reference failures and needs smart-triage evidence before implementation starts.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/aliasDoesNotDuplicateSignatures.ts
```

Coverage window:

```sh
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/aliasDoesNotDuplicateSignatures.ts --detail
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
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/aliasDoesNotDuplicateSignatures.ts --detail
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/aliasDoesNotDuplicateSignatures.ts
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

- `reference/typescript/tests/cases/compiler/aliasDoesNotDuplicateSignatures.ts`

## Duplicate detection

- `issues/open/113-implement-aliasDoesNotDuplicateSignatures.md` - Implement Aliasdoesnotduplicatesignatures (same reference path, same group key, title overlap)
- `issues/open/493-implement-aliasDoesNotDuplicateSignatures.md` - Implement Aliasdoesnotduplicatesignatures (same reference path, same feature label, same group key, title overlap)

## Smart triage

### Smart triage: Triage import export: aliasDoesNotDuplicateSignatures

- Issue class: `triage-needed`
- Feature label: `import-export`
- Diagnostic: `UnsupportedModule` / `unsupported-feature-boundary`
- Path: `reference/typescript/tests/cases/compiler/aliasDoesNotDuplicateSignatures.ts`

Reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/aliasDoesNotDuplicateSignatures.ts
```

Source overview:

```json
{
  "suite": "tsc",
  "bytes": 353,
  "lines": 15,
  "extension": ".ts",
  "first_code_line": "declare namespace demoNS {"
}
```

Failure location:

```json
{
  "code": "UnsupportedModule",
  "message": "issue-400: ambient namespace declarations require module ownership before runtime lowering at 74..83",
  "span_start": 74,
  "span_end": 83,
  "line": 4,
  "column": 11,
  "feature_label": "import-export",
  "error_type": "unsupported-feature-boundary"
}
```

Source context:

```text
1 | // @module: commonjs
2 | // @target: es2015
3 | // @filename: demo.d.ts
4 | declare namespace demoNS {
5 |     function f(): void;
6 | }
7 | declare module 'demoModule' {
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
    "path": "issues/open/113-implement-aliasDoesNotDuplicateSignatures.md",
    "title": "Implement Aliasdoesnotduplicatesignatures",
    "reason": "same reference path, title overlap"
  },
  {
    "state": "open",
    "path": "issues/open/432-implement-import-export.md",
    "title": "Implement import/export module syntax",
    "reason": "same feature label, title overlap"
  },
  {
    "state": "open",
    "path": "issues/open/457-implement-APISample-import-export.md",
    "title": "Implement Apisample Import Export",
    "reason": "same feature label, title overlap"
  },
  {
    "state": "open",
    "path": "issues/open/463-implement-FunctionDeclaration-import-export.md",
    "title": "Implement Functiondeclaration Import Export",
    "reason": "same feature label, title overlap"
  },
  {
    "state": "open",
    "path": "issues/open/493-implement-aliasDoesNotDuplicateSignatures.md",
    "title": "Implement Aliasdoesnotduplicatesignatures",
    "reason": "same reference path, same feature label, title overlap"
  },
  {
    "state": "open",
    "path": "issues/open/543-implement-APISample-import-export.md",
    "title": "Implement Apisample Import Export",
    "reason": "same feature label, title overlap"
  },
  {
    "state": "open",
    "path": "issues/done/549-implement-FunctionDeclaration-import-export.md",
    "title": "Implement Functiondeclaration Import Export",
    "reason": "same feature label, title overlap"
  },
  {
    "state": "done",
    "path": "issues/done/055-implement-import-export.md",
    "title": "Umbrella: implement import and export",
    "reason": "same feature label, title overlap"
  }
]
```

Error-specific suggestions:

- Keep module graph behavior separate from parser syntax unless the diagnostic proves syntax is the blocker.

Compiler dumps:

#### tokens

- ok: `True`
- truncated: `True`

```text
== tokens ==
[
    SpannedToken {
        kind: Ident(
            "declare",
        ),
        span: Span {
            start: 66,
            end: 73,
        },
    },
    SpannedToken {
        kind: Ident(
            "namespace",
        ),
        span: Span {
            start: 74,
            end: 83,
        },
    },
    SpannedToken {
        kind: Ident(
            "demoNS",
        ),
        span: Span {
            start: 84,
            end: 90,
        },
    },
    SpannedToken {
        kind: LeftBrace,
        span: Span {
            start: 91,
            end: 92,
        },
    },
    SpannedToken {
        kind: Function,
        span: Span {
            start: 98,
            end: 106,
        },
    },
    SpannedToken {
        kind: Ident(
            "f",
        ),
        span: Span {
            start: 107,
            end: 108,
        },
    },
    SpannedToken {
        kind: LeftParen,
        span: Span {
            start: 108,
            end: 109,
        },
    },
    SpannedToken {
        kind: RightParen,
        span: Span {
            start: 109,
            end: 110,
        },
    },
    SpannedToken {
        kind: Colon,
        span: Span {
            start: 110,
            end: 111,
        },
    },
    SpannedToken {
        kind: Void,
        span: Span {
            start: 112,
            end: 116,
        },
    },
    SpannedToken {
        kind: Semicolon,
        span: Span {
            start: 116,
            end: 117,
        },
    },
    SpannedToken {
        kind: RightBrace,
        span: Span {
            start: 119,
            end: 120,
        },
    },
    SpannedToken {
        kind: Ident(
            "declare",
        ),
        span: Span {
            start: 122,
            end: 129,
        },
    },
    SpannedToken {
        kind: Ident(
            "module",
        ),
        span: Span {
            start: 130,
            end: 136,
        },
    },
    SpannedToken {
        kind: String(
            "demoModule",
        ),
        span: Span {
            start: 137,
            end: 149,
        },
    },
    SpannedToken {
        kind: LeftBrace,
        span: Span {
            start: 150,
            end: 151,
        },
    },
    SpannedToken {
        kind: Import,
        span: Span {
            start: 157,
            end: 163,
        },
    },
    SpannedToken {
        kind: Ident(
            "alias",
        ),
        span: Span {
            start: 164,
            end: 169,
        },
    },
    SpannedToken {
        kind: Equal,
        span: Span {
            start: 170,
            end: 171,
        },
    },
    SpannedToken {
        kind: Ident(
            "demoNS",
        ),
        span: Span {
            start: 172,
            end: 178,
        },
    },
    SpannedToken {
        kind: Semicolon,
        span: Span {
            start: 178,
            end: 179,
        },
    },
    SpannedToken {
        kind: Export,
        span: Span {
            start: 185,
            end: 191,
        },
    },
    SpannedToken {
        kind: Equal,
        span: Span {
            start: 192,
            end: 193,
        },
    },
    SpannedToken {
        kind: Ident(
            "alias",
        ),
        span: Span {
            start: 194,
            end: 199,
        },
    },
    SpannedToken {
        kind: Semicolon,
        span: Span {
            start: 199,
            end: 200,
        },
    }
```

#### ast

- ok: `False`
- truncated: `False`

```text
error: [UnsupportedModule] issue-400: ambient namespace declarations require module ownership before runtime lowering at 74..83
```

#### resolved

- ok: `False`
- truncated: `False`

```text
error: [UnsupportedModule] issue-400: ambient namespace declarations require module ownership before runtime lowering at 74..83
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
        "code": 2664,
        "category": "Error",
        "message": "Invalid module name in augmentation, module 'demoModule' cannot be found.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/aliasDoesNotDuplicateSignatures.ts",
        "start": 137,
        "length": 12,
        "line": 7,
        "character": 16
      },
      {
        "code": 2307,
        "category": "Error",
        "message": "Cannot find module 'demoModule' or its corresponding type declarations.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/aliasDoesNotDuplicateSignatures.ts",
        "start": 246,
        "length": 12,
        "line": 12,
        "character": 19
      },
      {
        "code": 2322,
        "category": "Error",
        "message": "Type '() => void' is not assignable to type 'string'.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/aliasDoesNotDuplicateSignatures.ts",
        "start": 323,
        "length": 2,
        "line": 14,
        "character": 5
      }
    ],
    "hints": [
      {
        "kind": "function",
        "typeText": "void",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/aliasDoesNotDuplicateSignatures.ts",
        "start": 107,
        "length": 1,
        "line": 5,
        "character": 14,
        "name": "f"
      },
      {
        "kind": "binding",
        "typeText": "string",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/aliasDoesNotDuplicateSignatures.ts",
        "start": 323,
        "length": 2,
        "line": 14,
        "character": 5,
        "name": "x1"
      },
      {
        "kind": "binding",
        "typeText": "string",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/aliasDoesNotDuplicateSignatures.ts",
        "start": 351,
        "length": 2,
        "line": 15,
        "character": 5,
        "name": "x2"
      }
    ],
    "typescriptVersion": "6.0.3"
  },
  "ast": {
    "topLevel": [
      {
        "kind": "ModuleDeclaration",
        "text": "declare namespace demoNS {\r\n    function f(): void;\r\n}",
        "line": 4,
        "character": 1
      },
      {
        "kind": "ModuleDeclaration",
        "text": "declare module 'demoModule' {\r\n    import alias = demoNS;\r\n    export = alias;\r\n}",
        "line": 7,
        "character": 1
      },
      {
        "kind": "ImportDeclaration",
        "text": "import { f } from 'demoModule';",
        "line": 12,
        "character": 1
      },
      {
        "kind": "FirstStatement",
        "text": "let x1: string = demoNS.f;",
        "line": 14,
        "character": 1
      },
      {
        "kind": "FirstStatement",
        "text": "let x2: string = f;",
        "line": 15,
        "character": 1
      }
    ],
    "pathToPosition": [
      {
        "kind": "SourceFile",
        "text": "declare namespace demoNS {\r\n    function f(): void;\r\n}\r\ndeclare module 'demoModule' {\r\n    import alias = demoNS;\r\n    e",
        "line": 4,
        "character": 1
      },
      {
        "kind": "ModuleDeclaration",
        "text": "declare namespace demoNS {\r\n    function f(): void;\r\n}",
        "line": 4,
        "character": 1
      }
    ]
  }
}
```

Stack trace:

```text
error: [UnsupportedModule] issue-400: ambient namespace declarations require module ownership before runtime lowering at 74..83
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
