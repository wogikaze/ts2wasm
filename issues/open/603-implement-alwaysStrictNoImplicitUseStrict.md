---
id: 603
title: "Implement Alwaysstrictnoimplicitusestrict (audit reopened #603)"
type: spike
area: frontend/syntax
class: blocked
priority: P1
depends_on: [432]
blocks: []
created: 2026-05-01
updated: 2026-05-05status: open
---

## Summary

Triage alwaysStrictNoImplicitUseStrict across 1 failing reference test cases and split this bucket into implementation-ready child issues.

## Problem

Reference test results show 1 cases fail in directory `alwaysStrictNoImplicitUseStrict` with diagnostics: import-export. The compiler cannot handle these syntax/semantics, preventing compilation of code in this category.

Problem: alwaysStrictNoImplicitUseStrict has 1 reference failures and needs smart-triage evidence before implementation starts.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/alwaysStrictNoImplicitUseStrict.ts
```

Coverage window:

```sh
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/alwaysStrictNoImplicitUseStrict.ts --detail
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
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/alwaysStrictNoImplicitUseStrict.ts --detail
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/alwaysStrictNoImplicitUseStrict.ts
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

- `reference/typescript/tests/cases/compiler/alwaysStrictNoImplicitUseStrict.ts`

## Duplicate detection

- `issues/done/139-implement-alwaysStrictNoImplicitUseStrict.md` - Implement Alwaysstrictnoimplicitusestrict (same reference path, same group key, title overlap)
- `issues/open/517-implement-alwaysStrictNoImplicitUseStrict.md` - Implement Alwaysstrictnoimplicitusestrict (same reference path, same feature label, same group key, title overlap)

## Smart triage

### Smart triage: Triage import export: alwaysStrictNoImplicitUseStrict

- Issue class: `triage-needed`
- Feature label: `import-export`
- Diagnostic: `UnsupportedModule` / `unsupported-feature-boundary`
- Path: `reference/typescript/tests/cases/compiler/alwaysStrictNoImplicitUseStrict.ts`

Reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/alwaysStrictNoImplicitUseStrict.ts
```

Source overview:

```json
{
  "suite": "tsc",
  "bytes": 169,
  "lines": 10,
  "extension": ".ts",
  "first_code_line": "namespace M {"
}
```

Failure location:

```json
{
  "code": "UnsupportedModule",
  "message": "issue-399: TypeScript namespace/internal module declarations require module ownership before runtime lowering at 94..103",
  "span_start": 94,
  "span_end": 103,
  "line": 6,
  "column": 1,
  "feature_label": "import-export",
  "error_type": "unsupported-feature-boundary"
}
```

Source context:

```text
3 | // @alwaysStrict: true
4 | // @noImplicitUseStrict: true
5 | 
6 | namespace M {
7 |     export function f() {
8 |         var arguments = [];
9 |     }
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
    "path": "issues/done/139-implement-alwaysStrictNoImplicitUseStrict.md",
    "title": "Implement Alwaysstrictnoimplicitusestrict",
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
    "path": "issues/open/517-implement-alwaysStrictNoImplicitUseStrict.md",
    "title": "Implement Alwaysstrictnoimplicitusestrict",
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
        kind: Ident(
            "namespace",
        ),
        span: Span {
            start: 94,
            end: 103,
        },
    },
    SpannedToken {
        kind: Ident(
            "M",
        ),
        span: Span {
            start: 104,
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
        kind: Export,
        span: Span {
            start: 112,
            end: 118,
        },
    },
    SpannedToken {
        kind: Function,
        span: Span {
            start: 119,
            end: 127,
        },
    },
    SpannedToken {
        kind: Ident(
            "f",
        ),
        span: Span {
            start: 128,
            end: 129,
        },
    },
    SpannedToken {
        kind: LeftParen,
        span: Span {
            start: 129,
            end: 130,
        },
    },
    SpannedToken {
        kind: RightParen,
        span: Span {
            start: 130,
            end: 131,
        },
    },
    SpannedToken {
        kind: LeftBrace,
        span: Span {
            start: 132,
            end: 133,
        },
    },
    SpannedToken {
        kind: Var,
        span: Span {
            start: 142,
            end: 145,
        },
    },
    SpannedToken {
        kind: Ident(
            "arguments",
        ),
        span: Span {
            start: 146,
            end: 155,
        },
    },
    SpannedToken {
        kind: Equal,
        span: Span {
            start: 156,
            end: 157,
        },
    },
    SpannedToken {
        kind: LeftBracket,
        span: Span {
            start: 158,
            end: 159,
        },
    },
    SpannedToken {
        kind: RightBracket,
        span: Span {
            start: 159,
            end: 160,
        },
    },
    SpannedToken {
        kind: Semicolon,
        span: Span {
            start: 160,
            end: 161,
        },
    },
    SpannedToken {
        kind: RightBrace,
        span: Span {
            start: 166,
            end: 167,
        },
    },
    SpannedToken {
        kind: RightBrace,
        span: Span {
            start: 168,
            end: 169,
        },
    },
]
```

#### ast

- ok: `False`
- truncated: `False`

```text
error: [UnsupportedModule] issue-399: TypeScript namespace/internal module declarations require module ownership before runtime lowering at 94..103
```

#### resolved

- ok: `False`
- truncated: `False`

```text
error: [UnsupportedModule] issue-399: TypeScript namespace/internal module declarations require module ownership before runtime lowering at 94..103
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
        "code": 1100,
        "category": "Error",
        "message": "Invalid use of 'arguments' in strict mode.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/alwaysStrictNoImplicitUseStrict.ts",
        "start": 146,
        "length": 9,
        "line": 8,
        "character": 13
      }
    ],
    "hints": [
      {
        "kind": "function",
        "typeText": "void",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/alwaysStrictNoImplicitUseStrict.ts",
        "start": 128,
        "length": 1,
        "line": 7,
        "character": 21,
        "name": "f"
      },
      {
        "kind": "binding",
        "typeText": "never[]",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/alwaysStrictNoImplicitUseStrict.ts",
        "start": 146,
        "length": 9,
        "line": 8,
        "character": 13,
        "name": "arguments"
      }
    ],
    "typescriptVersion": "6.0.3"
  },
  "ast": {
    "topLevel": [
      {
        "kind": "ModuleDeclaration",
        "text": "namespace M {\n    export function f() {\n        var arguments = [];\n    }\n}",
        "line": 6,
        "character": 1
      }
    ],
    "pathToPosition": [
      {
        "kind": "SourceFile",
        "text": "namespace M {\n    export function f() {\n        var arguments = [];\n    }\n}",
        "line": 6,
        "character": 1
      },
      {
        "kind": "ModuleDeclaration",
        "text": "namespace M {\n    export function f() {\n        var arguments = [];\n    }\n}",
        "line": 6,
        "character": 1
      }
    ]
  }
}
```

Stack trace:

```text
error: [UnsupportedModule] issue-399: TypeScript namespace/internal module declarations require module ownership before runtime lowering at 94..103
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

## Status

Superseded by issue #139. Duplicate from separate coverage run.

## Reopened by audit

Date: 2026-05-05

Classification: must-reopen.

Reopen reason: frontmatter still says `class: blocked`, which is incompatible with a completed issue unless explicit supersedence/closure evidence is present.

Violated acceptance: the issue cannot provide repo-local close evidence for its checked acceptance criteria while it remains in this state. Acceptance checkboxes were reset for re-verification.

Evidence files:
- `issues/open/603-implement-alwaysStrictNoImplicitUseStrict.md` before this move
- `issues/open/603-implement-alwaysStrictNoImplicitUseStrict.md` after this move

Split follow-up: none created in this audit wave; this reopened issue remains the tracking item.
