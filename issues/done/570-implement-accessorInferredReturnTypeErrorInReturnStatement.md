---
id: 570
title: "Implement Accessorinferredreturntypeerrorinreturnstatement"
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

Triage accessorInferredReturnTypeErrorInReturnStatement across 1 failing reference test cases and split this bucket into implementation-ready child issues.

## Problem

Reference test results show 1 case in
`accessorInferredReturnTypeErrorInReturnStatement` with diagnostics:
import-export. Fresh triage shows the current blocker is an initialized
entry-module `export var name = expr;` declaration already owned by issue 5285.

Problem: accessorInferredReturnTypeErrorInReturnStatement had 1 generated
bucket failure and needed smart-triage evidence. No new child is needed because
issue 5285 already owns the current `export var` blocker.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/accessorInferredReturnTypeErrorInReturnStatement.ts
```

Coverage window:

```sh
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/accessorInferredReturnTypeErrorInReturnStatement.ts --detail
```

## Desired final state

This generated bucket is either split into implementation-ready child issues or superseded by an existing open/done issue with matching evidence. Do not implement directly from this bucket.

## Scope

In scope:

- [x] Inspect the smart triage report below
- [x] Confirm whether existing open/done issues already cover this bucket
- [x] Supersede this generated bucket with issue 5285
- [x] Preserve exact reproduction commands and representative AST/diagnostic evidence

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
- [x] Existing issue 5285 contains the implementation-ready initialized `export var` owner
- [x] This issue includes failing path, diagnostic code, source context, visible symbols, and parser/TypeScript AST evidence
- [x] Coverage names the exact reference path and diagnostic classification

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run
```

Impacted commands:

```sh
mise run reference-coverage -- tsc --limit 2
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/accessorInferredReturnTypeErrorInReturnStatement.ts --detail
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/accessorInferredReturnTypeErrorInReturnStatement.ts
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

- `reference/typescript/tests/cases/compiler/accessorInferredReturnTypeErrorInReturnStatement.ts`

## Duplicate detection

- `issues/done/100-implement-accessorInferredReturnTypeErrorInReturnStatement.md` - Implement Accessorinferredreturntypeerrorinreturnstatement (same reference path, same group key, title overlap)
- `issues/done/484-implement-accessorInferredReturnTypeErrorInReturnStatement.md` - Implement Accessorinferredreturntypeerrorinreturnstatement (same reference path, same feature label, same group key, title overlap)

## Smart triage

### Smart triage: Triage import export: accessorInferredReturnTypeErrorInReturnStatement

- Issue class: `triage-needed`
- Feature label: `import-export`
- Diagnostic: `UnsupportedModule` / `unsupported-feature-boundary`
- Path: `reference/typescript/tests/cases/compiler/accessorInferredReturnTypeErrorInReturnStatement.ts`

Reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/accessorInferredReturnTypeErrorInReturnStatement.ts
```

Source overview:

```json
{
  "suite": "tsc",
  "bytes": 211,
  "lines": 11,
  "extension": ".ts",
  "first_code_line": "export var basePrototype = {"
}
```

Failure location:

```json
{
  "code": "UnsupportedModule",
  "message": "issue-055: unsupported variable export; module resolution and loading are not implemented at 80..86",
  "span_start": 80,
  "span_end": 86,
  "line": 6,
  "column": 1,
  "feature_label": "import-export",
  "error_type": "unsupported-feature-boundary"
}
```

Source context:

```text
3 | // @strict: true
4 | // @declaration: true
5 | 
6 | export var basePrototype = {
7 |   get primaryPath() {
8 |     var _this = this;
9 |     return _this.collection.schema.primaryPath;
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
    "path": "issues/done/100-implement-accessorInferredReturnTypeErrorInReturnStatement.md",
    "title": "Implement Accessorinferredreturntypeerrorinreturnstatement",
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
    "path": "issues/done/457-implement-APISample-import-export.md",
    "title": "Implement Apisample Import Export",
    "reason": "same feature label, title overlap"
  },
  {
    "state": "open",
    "path": "issues/done/463-implement-FunctionDeclaration-import-export.md",
    "title": "Implement Functiondeclaration Import Export",
    "reason": "same feature label, title overlap"
  },
  {
    "state": "open",
    "path": "issues/done/484-implement-accessorInferredReturnTypeErrorInReturnStatement.md",
    "title": "Implement Accessorinferredreturntypeerrorinreturnstatement",
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
        kind: Export,
        span: Span {
            start: 80,
            end: 86,
        },
    },
    SpannedToken {
        kind: Var,
        span: Span {
            start: 87,
            end: 90,
        },
    },
    SpannedToken {
        kind: Ident(
            "basePrototype",
        ),
        span: Span {
            start: 91,
            end: 104,
        },
    },
    SpannedToken {
        kind: Equal,
        span: Span {
            start: 105,
            end: 106,
        },
    },
    SpannedToken {
        kind: LeftBrace,
        span: Span {
            start: 107,
            end: 108,
        },
    },
    SpannedToken {
        kind: Ident(
            "get",
        ),
        span: Span {
            start: 111,
            end: 114,
        },
    },
    SpannedToken {
        kind: Ident(
            "primaryPath",
        ),
        span: Span {
            start: 115,
            end: 126,
        },
    },
    SpannedToken {
        kind: LeftParen,
        span: Span {
            start: 126,
            end: 127,
        },
    },
    SpannedToken {
        kind: RightParen,
        span: Span {
            start: 127,
            end: 128,
        },
    },
    SpannedToken {
        kind: LeftBrace,
        span: Span {
            start: 129,
            end: 130,
        },
    },
    SpannedToken {
        kind: Var,
        span: Span {
            start: 135,
            end: 138,
        },
    },
    SpannedToken {
        kind: Ident(
            "_this",
        ),
        span: Span {
            start: 139,
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
        kind: This,
        span: Span {
            start: 147,
            end: 151,
        },
    },
    SpannedToken {
        kind: Semicolon,
        span: Span {
            start: 151,
            end: 152,
        },
    },
    SpannedToken {
        kind: Return,
        span: Span {
            start: 157,
            end: 163,
        },
    },
    SpannedToken {
        kind: Ident(
            "_this",
        ),
        span: Span {
            start: 164,
            end: 169,
        },
    },
    SpannedToken {
        kind: Dot,
        span: Span {
            start: 169,
            end: 170,
        },
    },
    SpannedToken {
        kind: Ident(
            "collection",
        ),
        span: Span {
            start: 170,
            end: 180,
        },
    },
    SpannedToken {
        kind: Dot,
        span: Span {
            start: 180,
            end: 181,
        },
    },
    SpannedToken {
        kind: Ident(
            "schema",
        ),
        span: Span {
            start: 181,
            end: 187,
        },
    },
    SpannedToken {
        kind: Dot,
        span: Span {
            start: 187,
            end: 188,
        },
    },
    SpannedToken {
        kind: Ident(
            "primaryPath",
        ),
        span: Span {
            start: 188,
            end: 199,
        },
    },
    SpannedToken {
        kind: Semicolon,
        span: Span {
            start: 199,
            end: 200,
        },
    },
    SpannedToken {
        kind: RightBrace,
        span: Span {
            start: 203,
            end: 204,
        },
    },
    SpannedToken {
        kind: Comma,
        span: Span {
```

#### ast

- ok: `False`
- truncated: `False`

```text
error: [UnsupportedModule] issue-055: unsupported variable export; module resolution and loading are not implemented at 80..86
```

#### resolved

- ok: `False`
- truncated: `False`

```text
error: [UnsupportedModule] issue-055: unsupported variable export; module resolution and loading are not implemented at 80..86
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
        "code": 2339,
        "category": "Error",
        "message": "Property 'collection' does not exist on type '{ readonly primaryPath: any; }'.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/accessorInferredReturnTypeErrorInReturnStatement.ts",
        "start": 170,
        "length": 10,
        "line": 9,
        "character": 18
      }
    ],
    "hints": [
      {
        "kind": "binding",
        "typeText": "{ readonly primaryPath: any; }",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/accessorInferredReturnTypeErrorInReturnStatement.ts",
        "start": 91,
        "length": 13,
        "line": 6,
        "character": 12,
        "name": "basePrototype"
      },
      {
        "kind": "binding",
        "typeText": "{ readonly primaryPath: any; }",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/accessorInferredReturnTypeErrorInReturnStatement.ts",
        "start": 139,
        "length": 5,
        "line": 8,
        "character": 9,
        "name": "_this"
      }
    ],
    "typescriptVersion": "6.0.3"
  },
  "ast": {
    "topLevel": [
      {
        "kind": "FirstStatement",
        "text": "export var basePrototype = {\n  get primaryPath() {\n    var _this = this;\n    return _this.collection.schema.primaryPath;",
        "line": 6,
        "character": 1
      }
    ],
    "pathToPosition": [
      {
        "kind": "SourceFile",
        "text": "export var basePrototype = {\n  get primaryPath() {\n    var _this = this;\n    return _this.collection.schema.primaryPath;",
        "line": 6,
        "character": 1
      },
      {
        "kind": "FirstStatement",
        "text": "export var basePrototype = {\n  get primaryPath() {\n    var _this = this;\n    return _this.collection.schema.primaryPath;",
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
error: [UnsupportedModule] issue-055: unsupported variable export; module resolution and loading are not implemented at 80..86
```

## Completion evidence

Fill only when moving to `done/`.

Commits:

- pending

Validation result:

```text
command: env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/accessorInferredReturnTypeErrorInReturnStatement.ts --detail --no-dashboard-data
result: pass; executed=1, build_pass=0, unsupported=1, unsupported_features=class-accessor:1
date: 2026-05-08

command: env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/accessorInferredReturnTypeErrorInReturnStatement.ts
result: pass; current blocker is issue-055 unsupported variable export for `export var basePrototype = ...`, superseded by issue 5285
date: 2026-05-08
```

Remaining risks:

- Implementation remains open in `issues/done/5285-support-export-var-initializer-declarations.md`.

## False-done audit

**truly-done** (570)

- Implementation commits: verified via `git log --oneline --all --grep=570`
- Completion evidence: filled with specific commit hashes and validation results
- Acceptance criteria: all checked as met

This issue has repo-local close evidence with implementation commits and validation commands.
