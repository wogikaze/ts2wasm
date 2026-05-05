---
id: 630
title: "Implement Amddependencycommentname"
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

Triage amdDependencyCommentName across 4 failing reference test cases and split this bucket into implementation-ready child issues.

## Problem

Reference test results show 4 cases fail in directory `amdDependencyCommentName` with diagnostics: import-export. The compiler cannot handle these syntax/semantics, preventing compilation of code in this category.

Problem: amdDependencyCommentName has 4 reference failures and needs smart-triage evidence before implementation starts.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/amdDependencyCommentName4.ts
```

Coverage window:

```sh
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/amdDependencyCommentName4.ts --detail
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
mise run reference-coverage -- tsc --limit 8
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/amdDependencyCommentName4.ts --detail
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/amdDependencyCommentName4.ts
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

- `reference/typescript/tests/cases/compiler/amdDependencyCommentName4.ts`
- `reference/typescript/tests/cases/compiler/amdDependencyCommentName2.ts`
- `reference/typescript/tests/cases/compiler/amdDependencyCommentName1.ts`
- `reference/typescript/tests/cases/compiler/amdDependencyCommentName3.ts`

## Duplicate detection

- `issues/done/172-implement-amdDependencyCommentName.md` - Implement Amddependencycommentname (same reference path, same group key, title overlap)

## Smart triage

### Smart triage: Triage import export: amdDependencyCommentName4

- Issue class: `triage-needed`
- Feature label: `import-export`
- Diagnostic: `UnsupportedModule` / `unsupported-feature-boundary`
- Path: `reference/typescript/tests/cases/compiler/amdDependencyCommentName4.ts`

Reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/amdDependencyCommentName4.ts
```

Source overview:

```json
{
  "suite": "tsc",
  "bytes": 456,
  "lines": 22,
  "extension": ".ts",
  "first_code_line": "import \"unaliasedModule1\";"
}
```

Failure location:

```json
{
  "code": "UnsupportedModule",
  "message": "issue-055: unsupported default import; module resolution and loading are not implemented at 259..265",
  "span_start": 259,
  "span_end": 265,
  "line": 10,
  "column": 1,
  "feature_label": "import-export",
  "error_type": "unsupported-feature-boundary"
}
```

Source context:

```text
 7 | 
 8 | import "unaliasedModule1";
 9 | 
10 | import r1 = require("aliasedModule1");
11 | r1;
12 | 
13 | import {p1, p2, p3} from "aliasedModule2";
```

Visible symbols before failure:

```json
[
  {
    "kind": "import",
    "name": "unaliasedModule1",
    "line": 8,
    "column": 1
  }
]
```

Duplicate candidates:

```json
[
  {
    "state": "open",
    "path": "issues/done/172-implement-amdDependencyCommentName.md",
    "title": "Implement Amddependencycommentname",
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
- truncated: `True`

```text
== tokens ==
[
    SpannedToken {
        kind: Import,
        span: Span {
            start: 231,
            end: 237,
        },
    },
    SpannedToken {
        kind: String(
            "unaliasedModule1",
        ),
        span: Span {
            start: 238,
            end: 256,
        },
    },
    SpannedToken {
        kind: Semicolon,
        span: Span {
            start: 256,
            end: 257,
        },
    },
    SpannedToken {
        kind: Import,
        span: Span {
            start: 259,
            end: 265,
        },
    },
    SpannedToken {
        kind: Ident(
            "r1",
        ),
        span: Span {
            start: 266,
            end: 268,
        },
    },
    SpannedToken {
        kind: Equal,
        span: Span {
            start: 269,
            end: 270,
        },
    },
    SpannedToken {
        kind: Ident(
            "require",
        ),
        span: Span {
            start: 271,
            end: 278,
        },
    },
    SpannedToken {
        kind: LeftParen,
        span: Span {
            start: 278,
            end: 279,
        },
    },
    SpannedToken {
        kind: String(
            "aliasedModule1",
        ),
        span: Span {
            start: 279,
            end: 295,
        },
    },
    SpannedToken {
        kind: RightParen,
        span: Span {
            start: 295,
            end: 296,
        },
    },
    SpannedToken {
        kind: Semicolon,
        span: Span {
            start: 296,
            end: 297,
        },
    },
    SpannedToken {
        kind: Ident(
            "r1",
        ),
        span: Span {
            start: 298,
            end: 300,
        },
    },
    SpannedToken {
        kind: Semicolon,
        span: Span {
            start: 300,
            end: 301,
        },
    },
    SpannedToken {
        kind: Import,
        span: Span {
            start: 303,
            end: 309,
        },
    },
    SpannedToken {
        kind: LeftBrace,
        span: Span {
            start: 310,
            end: 311,
        },
    },
    SpannedToken {
        kind: Ident(
            "p1",
        ),
        span: Span {
            start: 311,
            end: 313,
        },
    },
    SpannedToken {
        kind: Comma,
        span: Span {
            start: 313,
            end: 314,
        },
    },
    SpannedToken {
        kind: Ident(
            "p2",
        ),
        span: Span {
            start: 315,
            end: 317,
        },
    },
    SpannedToken {
        kind: Comma,
        span: Span {
            start: 317,
            end: 318,
        },
    },
    SpannedToken {
        kind: Ident(
            "p3",
        ),
        span: Span {
            start: 319,
            end: 321,
        },
    },
    SpannedToken {
        kind: RightBrace,
        span: Span {
            start: 321,
            end: 322,
        },
    },
    SpannedToken {
        kind: Ident(
            "from",
        ),
        span: Span {
            start: 323,
            end: 327,
        },
    },
    SpannedToken {
        kind: String(
            "aliasedModule2",
        ),
        span: Span {
            start: 328,
            end: 344,
        },
    },
    SpannedToken {
        kind: Semicolon,
        span: Span {
            start: 344,
            end: 345,
        },
    },
    SpannedToken {
        kind: Ident(
            "p1",
        ),
        span: Span {
            start: 3
```

#### ast

- ok: `False`
- truncated: `False`

```text
error: [UnsupportedModule] issue-055: unsupported default import; module resolution and loading are not implemented at 259..265
```

#### resolved

- ok: `False`
- truncated: `False`

```text
error: [UnsupportedModule] issue-055: unsupported default import; module resolution and loading are not implemented at 259..265
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
        "code": 2882,
        "category": "Error",
        "message": "Cannot find module or type declarations for side-effect import of 'unaliasedModule1'.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/amdDependencyCommentName4.ts",
        "start": 238,
        "length": 18,
        "line": 8,
        "character": 8
      },
      {
        "code": 2307,
        "category": "Error",
        "message": "Cannot find module 'aliasedModule1' or its corresponding type declarations.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/amdDependencyCommentName4.ts",
        "start": 279,
        "length": 16,
        "line": 10,
        "character": 21
      },
      {
        "code": 2307,
        "category": "Error",
        "message": "Cannot find module 'aliasedModule2' or its corresponding type declarations.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/amdDependencyCommentName4.ts",
        "start": 328,
        "length": 16,
        "line": 13,
        "character": 26
      },
      {
        "code": 2307,
        "category": "Error",
        "message": "Cannot find module 'aliasedModule3' or its corresponding type declarations.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/amdDependencyCommentName4.ts",
        "start": 365,
        "length": 16,
        "line": 16,
        "character": 15
      },
      {
        "code": 2307,
        "category": "Error",
        "message": "Cannot find module 'aliasedModule4' or its corresponding type declarations.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/amdDependencyCommentName4.ts",
        "start": 407,
        "length": 16,
        "line": 19,
        "character": 21
      },
      {
        "code": 2882,
        "category": "Error",
        "message": "Cannot find module or type declarations for side-effect import of 'unaliasedModule2'.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/amdDependencyCommentName4.ts",
        "start": 437,
        "length": 18,
        "line": 22,
        "character": 8
      }
    ],
    "hints": [],
    "typescriptVersion": "6.0.3"
  },
  "ast": {
    "topLevel": [
      {
        "kind": "ImportDeclaration",
        "text": "import \"unaliasedModule1\";",
        "line": 8,
        "character": 1
      },
      {
        "kind": "ImportEqualsDeclaration",
        "text": "import r1 = require(\"aliasedModule1\");",
        "line": 10,
        "character": 1
      },
      {
        "kind": "ExpressionStatement",
        "text": "r1;",
        "line": 11,
        "character": 1
      },
      {
        "kind": "ImportDeclaration",
        "text": "import {p1, p2, p3} from \"aliasedModule2\";",
        "line": 13,
        "character": 1
      },
      {
        "kind": "ExpressionStatement",
        "text": "p1;",
        "line": 14,
        "character": 1
      },
      {
        "kind": "ImportDeclaration",
        "text": "import d from \"aliasedModule3\";",
        "line": 16,
        "character": 1
      },
      {
        "kind": "ExpressionStatement",
        "text": "d;",
        "line": 17,
        "character": 1
      },
      {
        "kind": "ImportDeclaration",
        "text": "import * as ns from \"aliasedModule4\";",
        "line": 19,
        "character": 1
      },
      {
        "kind": "ExpressionStatement",
        "text": "ns;",
        "line": 20,
        "character": 1
      },
      {
        "kind": "ImportDeclaration",
        "text": "import \"unaliasedModule2\";",
        "line": 22,
        "character": 1
      }
    ],
    "pathToPosition": [
      {
        "kind": "SourceFile",
        "text": "import \"unaliasedModule1\";\n\nimport r1 = require(\"aliasedModule1\");\nr1;\n\nimport {p1, p2, p3} from \"aliasedModule2\";\np1;\n\n",
        "line": 8,
        "character": 1
      },
      {
        "kind": "ImportEqualsDeclaration",
        "text": "import r1 = require(\"aliasedModule1\");",
        "line": 10,
        "character": 1
      }
    ]
  }
}
```

Stack trace:

```text
error: [UnsupportedModule] issue-055: unsupported default import; module resolution and loading are not implemented at 259..265
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
