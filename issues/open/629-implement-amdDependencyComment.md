---
id: 629
title: "Implement Amddependencycomment"
type: spike
area: frontend/syntax
class: blocked
priority: P1
depends_on: [432]
blocks: []
created: 2026-05-01
updated: 2026-05-01
---

## Summary

Triage amdDependencyComment across 2 failing reference test cases and split this bucket into implementation-ready child issues.

## Problem

Reference test results show 2 cases fail in directory `amdDependencyComment` with diagnostics: import-export. The compiler cannot handle these syntax/semantics, preventing compilation of code in this category.

Problem: amdDependencyComment has 2 reference failures and needs smart-triage evidence before implementation starts.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/amdDependencyComment2.ts
```

Coverage window:

```sh
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/amdDependencyComment2.ts --detail
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
mise run reference-coverage -- tsc --limit 4
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/amdDependencyComment2.ts --detail
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/amdDependencyComment2.ts
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

- `reference/typescript/tests/cases/compiler/amdDependencyComment2.ts`
- `reference/typescript/tests/cases/compiler/amdDependencyComment1.ts`

## Duplicate detection

- `issues/done/171-implement-amdDependencyComment.md` - Implement Amddependencycomment (same reference path, same group key, title overlap)

## Smart triage

### Smart triage: Triage import export: amdDependencyComment2

- Issue class: `triage-needed`
- Feature label: `import-export`
- Diagnostic: `UnsupportedModule` / `unsupported-feature-boundary`
- Path: `reference/typescript/tests/cases/compiler/amdDependencyComment2.ts`

Reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/amdDependencyComment2.ts
```

Source overview:

```json
{
  "suite": "tsc",
  "bytes": 100,
  "lines": 6,
  "extension": ".ts",
  "first_code_line": "import m1 = require(\"m2\")"
}
```

Failure location:

```json
{
  "code": "UnsupportedModule",
  "message": "issue-055: unsupported default import; module resolution and loading are not implemented at 71..77",
  "span_start": 71,
  "span_end": 77,
  "line": 5,
  "column": 5,
  "feature_label": "import-export",
  "error_type": "unsupported-feature-boundary"
}
```

Source context:

```text
2 | //@module: amd
3 | ///<amd-dependency path='bar'/>
4 | 
5 | import m1 = require("m2")
6 | m1.f();
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
    "path": "issues/done/171-implement-amdDependencyComment.md",
    "title": "Implement Amddependencycomment",
    "reason": "same reference path"
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
- truncated: `False`

```text
== tokens ==
[
    SpannedToken {
        kind: Import,
        span: Span {
            start: 71,
            end: 77,
        },
    },
    SpannedToken {
        kind: Ident(
            "m1",
        ),
        span: Span {
            start: 78,
            end: 80,
        },
    },
    SpannedToken {
        kind: Equal,
        span: Span {
            start: 81,
            end: 82,
        },
    },
    SpannedToken {
        kind: Ident(
            "require",
        ),
        span: Span {
            start: 83,
            end: 90,
        },
    },
    SpannedToken {
        kind: LeftParen,
        span: Span {
            start: 90,
            end: 91,
        },
    },
    SpannedToken {
        kind: String(
            "m2",
        ),
        span: Span {
            start: 91,
            end: 95,
        },
    },
    SpannedToken {
        kind: RightParen,
        span: Span {
            start: 95,
            end: 96,
        },
    },
    SpannedToken {
        kind: Ident(
            "m1",
        ),
        span: Span {
            start: 98,
            end: 100,
        },
    },
    SpannedToken {
        kind: Dot,
        span: Span {
            start: 100,
            end: 101,
        },
    },
    SpannedToken {
        kind: Ident(
            "f",
        ),
        span: Span {
            start: 101,
            end: 102,
        },
    },
    SpannedToken {
        kind: LeftParen,
        span: Span {
            start: 102,
            end: 103,
        },
    },
    SpannedToken {
        kind: RightParen,
        span: Span {
            start: 103,
            end: 104,
        },
    },
    SpannedToken {
        kind: Semicolon,
        span: Span {
            start: 104,
            end: 105,
        },
    },
]
```

#### ast

- ok: `False`
- truncated: `False`

```text
error: [UnsupportedModule] issue-055: unsupported default import; module resolution and loading are not implemented at 71..77
```

#### resolved

- ok: `False`
- truncated: `False`

```text
error: [UnsupportedModule] issue-055: unsupported default import; module resolution and loading are not implemented at 71..77
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
        "message": "Cannot find module 'm2' or its corresponding type declarations.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/amdDependencyComment2.ts",
        "start": 91,
        "length": 4,
        "line": 5,
        "character": 21
      }
    ],
    "hints": [],
    "typescriptVersion": "6.0.3"
  },
  "ast": {
    "topLevel": [
      {
        "kind": "ImportEqualsDeclaration",
        "text": "import m1 = require(\"m2\")",
        "line": 5,
        "character": 1
      },
      {
        "kind": "ExpressionStatement",
        "text": "m1.f();",
        "line": 6,
        "character": 1
      }
    ],
    "pathToPosition": [
      {
        "kind": "SourceFile",
        "text": "import m1 = require(\"m2\")\r\nm1.f();",
        "line": 5,
        "character": 1
      },
      {
        "kind": "ImportEqualsDeclaration",
        "text": "import m1 = require(\"m2\")",
        "line": 5,
        "character": 1
      }
    ]
  }
}
```

Stack trace:

```text
error: [UnsupportedModule] issue-055: unsupported default import; module resolution and loading are not implemented at 71..77
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
