---
id: 528
title: "Implement Ambientexternalmodulewithrelativemodulename (audit reopened #528)"
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

Triage ambientExternalModuleWithRelativeModuleName across 1 failing reference test cases and split this bucket into implementation-ready child issues.

## Problem

Reference test results show 1 cases fail in directory `ambientExternalModuleWithRelativeModuleName` with diagnostics: import-export. The compiler cannot handle these syntax/semantics, preventing compilation of code in this category.

Problem: ambientExternalModuleWithRelativeModuleName has 1 reference failures and needs smart-triage evidence before implementation starts.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/ambientExternalModuleWithRelativeModuleName.ts
```

Coverage window:

```sh
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/ambientExternalModuleWithRelativeModuleName.ts --detail
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
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/ambientExternalModuleWithRelativeModuleName.ts --detail
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/ambientExternalModuleWithRelativeModuleName.ts
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

- `reference/typescript/tests/cases/compiler/ambientExternalModuleWithRelativeModuleName.ts`

## Duplicate detection

- `issues/done/153-implement-ambientExternalModuleWithRelativeModuleName.md` - Implement Ambientexternalmodulewithrelativemodulename (same reference path, same group key, title overlap)

## Smart triage

### Smart triage: Triage import export: ambientExternalModuleWithRelativeModuleName

- Issue class: `triage-needed`
- Feature label: `import-export`
- Diagnostic: `UnsupportedModule` / `unsupported-feature-boundary`
- Path: `reference/typescript/tests/cases/compiler/ambientExternalModuleWithRelativeModuleName.ts`

Reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/ambientExternalModuleWithRelativeModuleName.ts
```

Source overview:

```json
{
  "suite": "tsc",
  "bytes": 134,
  "lines": 8,
  "extension": ".ts",
  "first_code_line": "declare module \"./relativeModule\" {"
}
```

Failure location:

```json
{
  "code": "UnsupportedModule",
  "message": "issue-400: ambient module declarations require module ownership before runtime lowering at 28..34",
  "span_start": 28,
  "span_end": 34,
  "line": 2,
  "column": 10,
  "feature_label": "import-export",
  "error_type": "unsupported-feature-boundary"
}
```

Source context:

```text
1 | // @target: es2015
2 | declare module "./relativeModule" {
3 |     var x: string;
4 | }
5 |
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
    "path": "issues/done/153-implement-ambientExternalModuleWithRelativeModuleName.md",
    "title": "Implement Ambientexternalmodulewithrelativemodulename",
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
            "declare",
        ),
        span: Span {
            start: 20,
            end: 27,
        },
    },
    SpannedToken {
        kind: Ident(
            "module",
        ),
        span: Span {
            start: 28,
            end: 34,
        },
    },
    SpannedToken {
        kind: String(
            "./relativeModule",
        ),
        span: Span {
            start: 35,
            end: 53,
        },
    },
    SpannedToken {
        kind: LeftBrace,
        span: Span {
            start: 54,
            end: 55,
        },
    },
    SpannedToken {
        kind: Var,
        span: Span {
            start: 61,
            end: 64,
        },
    },
    SpannedToken {
        kind: Ident(
            "x",
        ),
        span: Span {
            start: 65,
            end: 66,
        },
    },
    SpannedToken {
        kind: Colon,
        span: Span {
            start: 66,
            end: 67,
        },
    },
    SpannedToken {
        kind: Ident(
            "string",
        ),
        span: Span {
            start: 68,
            end: 74,
        },
    },
    SpannedToken {
        kind: Semicolon,
        span: Span {
            start: 74,
            end: 75,
        },
    },
    SpannedToken {
        kind: RightBrace,
        span: Span {
            start: 77,
            end: 78,
        },
    },
    SpannedToken {
        kind: Ident(
            "declare",
        ),
        span: Span {
            start: 82,
            end: 89,
        },
    },
    SpannedToken {
        kind: Ident(
            "module",
        ),
        span: Span {
            start: 90,
            end: 96,
        },
    },
    SpannedToken {
        kind: String(
            ".\\relativeModule",
        ),
        span: Span {
            start: 97,
            end: 116,
        },
    },
    SpannedToken {
        kind: LeftBrace,
        span: Span {
            start: 117,
            end: 118,
        },
    },
    SpannedToken {
        kind: Var,
        span: Span {
            start: 124,
            end: 127,
        },
    },
    SpannedToken {
        kind: Ident(
            "x",
        ),
        span: Span {
            start: 128,
            end: 129,
        },
    },
    SpannedToken {
        kind: Colon,
        span: Span {
            start: 129,
            end: 130,
        },
    },
    SpannedToken {
        kind: Ident(
            "string",
        ),
        span: Span {
            start: 131,
            end: 137,
        },
    },
    SpannedToken {
        kind: Semicolon,
        span: Span {
            start: 137,
            end: 138,
        },
    },
    SpannedToken {
        kind: RightBrace,
        span: Span {
            start: 140,
            end: 141,
        },
    },
]
```

#### ast

- ok: `False`
- truncated: `False`

```text
error: [UnsupportedModule] issue-400: ambient module declarations require module ownership before runtime lowering at 28..34
```

#### resolved

- ok: `False`
- truncated: `False`

```text
error: [UnsupportedModule] issue-400: ambient module declarations require module ownership before runtime lowering at 28..34
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
        "code": 2436,
        "category": "Error",
        "message": "Ambient module declaration cannot specify relative module name.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/ambientExternalModuleWithRelativeModuleName.ts",
        "start": 35,
        "length": 18,
        "line": 2,
        "character": 16
      },
      {
        "code": 2436,
        "category": "Error",
        "message": "Ambient module declaration cannot specify relative module name.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/ambientExternalModuleWithRelativeModuleName.ts",
        "start": 97,
        "length": 19,
        "line": 6,
        "character": 16
      }
    ],
    "hints": [
      {
        "kind": "binding",
        "typeText": "string",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/ambientExternalModuleWithRelativeModuleName.ts",
        "start": 65,
        "length": 1,
        "line": 3,
        "character": 9,
        "name": "x"
      },
      {
        "kind": "binding",
        "typeText": "string",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/ambientExternalModuleWithRelativeModuleName.ts",
        "start": 128,
        "length": 1,
        "line": 7,
        "character": 9,
        "name": "x"
      }
    ],
    "typescriptVersion": "6.0.3"
  },
  "ast": {
    "topLevel": [
      {
        "kind": "ModuleDeclaration",
        "text": "declare module \"./relativeModule\" {\r\n    var x: string;\r\n}",
        "line": 2,
        "character": 1
      },
      {
        "kind": "ModuleDeclaration",
        "text": "declare module \".\\\\relativeModule\" {\r\n    var x: string;\r\n}",
        "line": 6,
        "character": 1
      }
    ],
    "pathToPosition": [
      {
        "kind": "SourceFile",
        "text": "declare module \"./relativeModule\" {\r\n    var x: string;\r\n}\r\n\r\ndeclare module \".\\\\relativeModule\" {\r\n    var x: string;\r\n",
        "line": 2,
        "character": 1
      },
      {
        "kind": "ModuleDeclaration",
        "text": "declare module \"./relativeModule\" {\r\n    var x: string;\r\n}",
        "line": 2,
        "character": 1
      }
    ]
  }
}
```

Stack trace:

```text
error: [UnsupportedModule] issue-400: ambient module declarations require module ownership before runtime lowering at 28..34
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

Superseded by issue #153. Duplicate from separate coverage run.

## Reopened by audit

Date: 2026-05-05

Classification: must-reopen.

Reopen reason: frontmatter still says `class: blocked`, which is incompatible with a completed issue unless explicit supersedence/closure evidence is present.

Violated acceptance: the issue cannot provide repo-local close evidence for its checked acceptance criteria while it remains in this state. Acceptance checkboxes were reset for re-verification.

Evidence files:
- `issues/open/528-implement-ambientExternalModuleWithRelativeModuleName.md` before this move
- `issues/open/528-implement-ambientExternalModuleWithRelativeModuleName.md` after this move

Split follow-up: none created in this audit wave; this reopened issue remains the tracking item.
