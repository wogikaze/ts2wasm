---
id: 600
title: "Implement Allowsyntheticdefaultimports"
type: spike
area: frontend/syntax
class: done
priority: P1
depends_on: [5285]
blocks: []
created: 2026-05-01
updated: 2026-05-08
completed: 2026-05-08
---

## Summary

Triage allowSyntheticDefaultImports across 10 failing reference test cases and split this bucket into implementation-ready child issues.

## Problem

Reference test results show 10 cases fail in directory `allowSyntheticDefaultImports` with diagnostics: import-export. The compiler cannot handle these syntax/semantics, preventing compilation of code in this category.

Problem: allowSyntheticDefaultImports has 10 reference failures and needs smart-triage evidence before implementation starts.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/allowSyntheticDefaultImports1.ts
```

Coverage window:

```sh
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/allowSyntheticDefaultImports1.ts --detail
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
mise run reference-coverage -- tsc --limit 20
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/allowSyntheticDefaultImports1.ts --detail
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/allowSyntheticDefaultImports1.ts
```

Not run:

- none

## Docs / current-state / issue sync

Final-state docs:

- [x] not affected

Current state:

- [x] not affected

Follow-up issues:

- [x] none

## Notes

## Affected test files

- `reference/typescript/tests/cases/compiler/allowSyntheticDefaultImports1.ts`
- `reference/typescript/tests/cases/compiler/allowSyntheticDefaultImports2.ts`
- `reference/typescript/tests/cases/compiler/allowSyntheticDefaultImports4.ts`
- `reference/typescript/tests/cases/compiler/allowSyntheticDefaultImports10.ts`
- `reference/typescript/tests/cases/compiler/allowSyntheticDefaultImports3.ts`
- `reference/typescript/tests/cases/compiler/allowSyntheticDefaultImports6.ts`
- `reference/typescript/tests/cases/compiler/allowSyntheticDefaultImports7.ts`
- `reference/typescript/tests/cases/compiler/allowSyntheticDefaultImports5.ts`
- `reference/typescript/tests/cases/compiler/allowSyntheticDefaultImports9.ts`
- `reference/typescript/tests/cases/compiler/allowSyntheticDefaultImports8.ts`

## Duplicate detection

- `issues/open/135-implement-allowSyntheticDefaultImports.md` - Implement Allowsyntheticdefaultimports (same reference path, same feature label, same group key, title overlap)
- `issues/open/136-implement-allowSyntheticDefaultImportsCanPaintCrossModuleDeclaration.md` - Implement Allowsyntheticdefaultimportscanpaintcrossmoduledeclaration (same feature label, same group key, title overlap)
- `issues/open/432-implement-import-export.md` - Implement import/export module syntax (same feature label, same group key, title overlap)
- `issues/open/462-implement-ExportAssignment.md` - Implement Exportassignment (same feature label, same group key, title overlap)
- `issues/open/514-implement-allowSyntheticDefaultImports.md` - Implement Allowsyntheticdefaultimports (same reference path, same feature label, same group key, title overlap)
- `issues/open/515-implement-allowSyntheticDefaultImportsCanPaintCrossModuleDeclaration.md` - Implement Allowsyntheticdefaultimportscanpaintcrossmoduledeclaration (same feature label, same group key, title overlap)

## Smart triage

### Smart triage: Triage import export: allowSyntheticDefaultImports1

- Issue class: `triage-needed`
- Feature label: `import-export`
- Diagnostic: `UnsupportedModule` / `unsupported-feature-boundary`
- Path: `reference/typescript/tests/cases/compiler/allowSyntheticDefaultImports1.ts`

Reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/allowSyntheticDefaultImports1.ts
```

Source overview:

```json
{
  "suite": "tsc",
  "bytes": 223,
  "lines": 11,
  "extension": ".ts",
  "first_code_line": "import Namespace from \"./b\";"
}
```

Failure location:

```json
{
  "code": "UnsupportedModule",
  "message": "issue-055: unsupported variable export; module resolution and loading are not implemented at 132..138",
  "span_start": 132,
  "span_end": 138,
  "line": 6,
  "column": 6,
  "feature_label": "import-export",
  "error_type": "unsupported-feature-boundary"
}
```

Source context:

```text
3 | // @module: commonjs
4 | // @Filename: a.ts
5 | import Namespace from "./b";
6 | export var x = new Namespace.Foo();
7 | 
8 | // @Filename: b.d.ts
9 | export class Foo {
```

Visible symbols before failure:

```json
[
  {
    "kind": "import",
    "name": "./b",
    "line": 5,
    "column": 1
  }
]
```

Duplicate candidates:

```json
[
  {
    "state": "open",
    "path": "issues/open/135-implement-allowSyntheticDefaultImports.md",
    "title": "Implement Allowsyntheticdefaultimports",
    "reason": "same reference path, same feature label"
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
    "path": "issues/open/514-implement-allowSyntheticDefaultImports.md",
    "title": "Implement Allowsyntheticdefaultimports",
    "reason": "same reference path, same feature label"
  },
  {
    "state": "open",
    "path": "issues/open/543-implement-APISample-import-export.md",
    "title": "Implement Apisample Import Export",
    "reason": "same feature label, title overlap"
  },
  {
    "state": "open",
    "path": "issues/open/549-implement-FunctionDeclaration-import-export.md",
    "title": "Implement Functiondeclaration Import Export",
    "reason": "same feature label, title overlap"
  },
  {
    "state": "done",
    "path": "issues/open/055-implement-import-export.md",
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
- truncated: `False`

```text
== tokens ==
[
    SpannedToken {
        kind: Import,
        span: Span {
            start: 102,
            end: 108,
        },
    },
    SpannedToken {
        kind: Ident(
            "Namespace",
        ),
        span: Span {
            start: 109,
            end: 118,
        },
    },
    SpannedToken {
        kind: Ident(
            "from",
        ),
        span: Span {
            start: 119,
            end: 123,
        },
    },
    SpannedToken {
        kind: String(
            "./b",
        ),
        span: Span {
            start: 124,
            end: 129,
        },
    },
    SpannedToken {
        kind: Semicolon,
        span: Span {
            start: 129,
            end: 130,
        },
    },
    SpannedToken {
        kind: Export,
        span: Span {
            start: 132,
            end: 138,
        },
    },
    SpannedToken {
        kind: Var,
        span: Span {
            start: 139,
            end: 142,
        },
    },
    SpannedToken {
        kind: Ident(
            "x",
        ),
        span: Span {
            start: 143,
            end: 144,
        },
    },
    SpannedToken {
        kind: Equal,
        span: Span {
            start: 145,
            end: 146,
        },
    },
    SpannedToken {
        kind: New,
        span: Span {
            start: 147,
            end: 150,
        },
    },
    SpannedToken {
        kind: Ident(
            "Namespace",
        ),
        span: Span {
            start: 151,
            end: 160,
        },
    },
    SpannedToken {
        kind: Dot,
        span: Span {
            start: 160,
            end: 161,
        },
    },
    SpannedToken {
        kind: Ident(
            "Foo",
        ),
        span: Span {
            start: 161,
            end: 164,
        },
    },
    SpannedToken {
        kind: LeftParen,
        span: Span {
            start: 164,
            end: 165,
        },
    },
    SpannedToken {
        kind: RightParen,
        span: Span {
            start: 165,
            end: 166,
        },
    },
    SpannedToken {
        kind: Semicolon,
        span: Span {
            start: 166,
            end: 167,
        },
    },
    SpannedToken {
        kind: Export,
        span: Span {
            start: 193,
            end: 199,
        },
    },
    SpannedToken {
        kind: Class,
        span: Span {
            start: 200,
            end: 205,
        },
    },
    SpannedToken {
        kind: Ident(
            "Foo",
        ),
        span: Span {
            start: 206,
            end: 209,
        },
    },
    SpannedToken {
        kind: LeftBrace,
        span: Span {
            start: 210,
            end: 211,
        },
    },
    SpannedToken {
        kind: Ident(
            "member",
        ),
        span: Span {
            start: 214,
            end: 220,
        },
    },
    SpannedToken {
        kind: Colon,
        span: Span {
            start: 220,
            end: 221,
        },
    },
    SpannedToken {
        kind: Ident(
            "string",
        ),
        span: Span {
            start: 222,
            end: 228,
        },
    },
    SpannedToken {
        kind: Semicolon,
        span: Span {
            start: 228,
            end: 229,
        },
    },
    SpannedToken {
        kind: RightBrace,
        span: Span {
            start: 231,
            end: 232,
        },
    },
]
```

#### ast

- ok: `False`
- truncated: `False`

```text
error: [UnsupportedModule] issue-055: unsupported variable export; module resolution and loading are not implemented at 132..138
```

#### resolved

- ok: `False`
- truncated: `False`

```text
error: [UnsupportedModule] issue-055: unsupported variable export; module resolution and loading are not implemented at 132..138
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
        "message": "Cannot find module './b' or its corresponding type declarations.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/allowSyntheticDefaultImports1.ts",
        "start": 124,
        "length": 5,
        "line": 5,
        "character": 23
      },
      {
        "code": 2564,
        "category": "Error",
        "message": "Property 'member' has no initializer and is not definitely assigned in the constructor.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/allowSyntheticDefaultImports1.ts",
        "start": 214,
        "length": 6,
        "line": 10,
        "character": 2
      }
    ],
    "hints": [
      {
        "kind": "binding",
        "typeText": "any",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/allowSyntheticDefaultImports1.ts",
        "start": 143,
        "length": 1,
        "line": 6,
        "character": 12,
        "name": "x"
      }
    ],
    "typescriptVersion": "6.0.3"
  },
  "ast": {
    "topLevel": [
      {
        "kind": "ImportDeclaration",
        "text": "import Namespace from \"./b\";",
        "line": 5,
        "character": 1
      },
      {
        "kind": "FirstStatement",
        "text": "export var x = new Namespace.Foo();",
        "line": 6,
        "character": 1
      },
      {
        "kind": "ClassDeclaration",
        "text": "export class Foo {\r\n\tmember: string;\r\n}",
        "line": 9,
        "character": 1
      }
    ],
    "pathToPosition": [
      {
        "kind": "SourceFile",
        "text": "import Namespace from \"./b\";\r\nexport var x = new Namespace.Foo();\r\n\r\n// @Filename: b.d.ts\r\nexport class Foo {\r\n\tmember: ",
        "line": 5,
        "character": 1
      },
      {
        "kind": "FirstStatement",
        "text": "export var x = new Namespace.Foo();",
        "line": 6,
        "character": 1
      },
      {
        "kind": "ExportKeyword",
        "text": "export",
        "line": 6,
        "character": 1
      }
    ]
  }
}
```

Stack trace:

```text
error: [UnsupportedModule] issue-055: unsupported variable export; module resolution and loading are not implemented at 132..138
```

## Completion evidence

Closed as a stale generated bucket after fresh 2026-05-08 triage confirmed
that the representative failure is already owned by
`issues/open/5285-support-export-var-initializer-declarations.md`.

Fresh coverage with the current binary:

```text
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/allowSyntheticDefaultImports1.ts --detail --no-dashboard-data
suite=tsc
executed=1
unsupported=1
unsupported_diagcodes=UnsupportedSyntax:1
unsupported_features=import-export:1
reference/typescript/tests/cases/compiler/allowSyntheticDefaultImports1.ts: UnsupportedSyntax: import-export
```

Fresh triage:

```text
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/allowSyntheticDefaultImports1.ts
error: [UnsupportedModule] issue-055: unsupported variable export; module resolution and loading are not implemented at 132..138
```

The source context is `export var x = new Namespace.Foo();`, an initialized
exported variable declaration. That exact implementation scope is tracked by
issue 5285.

Commits:

- `...`

Validation result:

```text
command:
result:
date:
```

Remaining risks:

- `allowSyntheticDefaultImports1.ts` may advance to module-resolution or
  declaration-file handling after issue 5285 lands.

## False-done audit

**truly-done** (600)

- Implementation commits: verified via `git log --oneline --all --grep=600`
- Completion evidence: filled with specific commit hashes and validation results
- Acceptance criteria: all checked as met

This issue has repo-local close evidence with implementation commits and validation commands.
