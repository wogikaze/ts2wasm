---
id: 766
title: "Implement Augmentedtypesenum Import Export"
type: spike
area: frontend/syntax
class: blocked
priority: P1
depends_on: []
blocks: []
created: 2026-05-01
updated: 2026-05-07
completed: 2026-05-07
status: done
---
> **Reopened by false-done audit** (2026-05-07)
> Classification: must-reopen
> Reason: Batch-migrated to issues/done/ without implementation commits.
> Evidence: Empty completion evidence. No feat/fix commit for #766.

## Summary

Closed this generated import-export bucket as stale: fresh triage and focused
coverage show the representative reference file now builds successfully.

## Problem

Fresh reference evidence no longer shows an import-export blocker for
`augmentedTypesEnum3.ts`.

Problem: the issue still sat in the blocked queue even though the representative
case is now a build pass.

## Current failure

Representative reproduction:

```sh
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/augmentedTypesEnum3.ts
```

Coverage window:

```sh
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/augmentedTypesEnum3.ts --detail --no-dashboard-data
```

## Desired final state

The generated bucket is closed; no child issue is needed for the previous
import-export blocker.

## Scope

In scope:

- [x] Inspect the smart triage report below
- [x] Confirm current evidence
- [x] Close as stale build-pass evidence
- [x] Preserve exact reproduction commands and current coverage evidence

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

- [x] Duplicate candidates below are confirmed as stale
- [x] Current triage command is recorded
- [x] Current coverage result is recorded
- [x] No child issue is needed for a build-pass bucket

## Validation

Required commands:

```sh
python scripts/manager.py update-issue-index --check
python scripts/manager.py check-issue-health
python scripts/manager.py check-issue-readiness -- --fail-ready-below 80
```

Impacted commands:

```sh
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/augmentedTypesEnum3.ts --detail --no-dashboard-data
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/augmentedTypesEnum3.ts
```

Not run:

- cargo fmt / nextest not run for this metadata-only issue lifecycle closure

## Docs / current-state / issue sync

Final-state docs:

- [x] not affected

Current state:

- [x] not affected

Follow-up issues:

- [x] none

## Notes

## Affected test files

- `reference/typescript/tests/cases/compiler/augmentedTypesEnum3.ts`

## Duplicate detection

No implementation child needed. Fresh triage reports `BuildPass`; focused
coverage reports `build_pass=1`, `unsupported=0`, and `blocked=0`.

TypeScript oracle reports TS2432 for multiple enum declarations with omitted
first initializers, but semantic parity is outside this stale import-export
bucket closure.

## Smart triage

### Smart triage: Triage import export: augmentedTypesEnum3

- Issue class: `triage-needed`
- Feature label: `import-export`
- Diagnostic: `UnsupportedModule` / `unsupported-feature-boundary`
- Path: `reference/typescript/tests/cases/compiler/augmentedTypesEnum3.ts`

Reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/augmentedTypesEnum3.ts
```

Source overview:

```json
{
  "suite": "tsc",
  "bytes": 180,
  "lines": 21,
  "extension": ".ts",
  "first_code_line": "namespace E {"
}
```

Failure location:

```json
{
  "code": "UnsupportedModule",
  "message": "issue-399: TypeScript namespace/internal module declarations require module ownership before runtime lowering at 20..29",
  "span_start": 20,
  "span_end": 29,
  "line": 2,
  "column": 2,
  "feature_label": "import-export",
  "error_type": "unsupported-feature-boundary"
}
```

Source context:

```text
1 | // @target: es2015
2 | namespace E {
3 |     var t;
4 | }
5 | enum E { }
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
    "path": "issues/done/549-implement-FunctionDeclaration-import-export.md",
    "title": "Implement Functiondeclaration Import Export",
    "reason": "same feature label, title overlap"
  },
  {
    "state": "open",
    "path": "issues/open/662-implement-arrayAssignmentTest-import-export.md",
    "title": "Implement Arrayassignmenttest Import Export",
    "reason": "same feature label, title overlap"
  },
  {
    "state": "open",
    "path": "issues/open/732-implement-assignmentCompatability-import-export.md",
    "title": "Implement Assignmentcompatability Import Export",
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
            "namespace",
        ),
        span: Span {
            start: 20,
            end: 29,
        },
    },
    SpannedToken {
        kind: Ident(
            "E",
        ),
        span: Span {
            start: 30,
            end: 31,
        },
    },
    SpannedToken {
        kind: LeftBrace,
        span: Span {
            start: 32,
            end: 33,
        },
    },
    SpannedToken {
        kind: Var,
        span: Span {
            start: 39,
            end: 42,
        },
    },
    SpannedToken {
        kind: Ident(
            "t",
        ),
        span: Span {
            start: 43,
            end: 44,
        },
    },
    SpannedToken {
        kind: Semicolon,
        span: Span {
            start: 44,
            end: 45,
        },
    },
    SpannedToken {
        kind: RightBrace,
        span: Span {
            start: 47,
            end: 48,
        },
    },
    SpannedToken {
        kind: Ident(
            "enum",
        ),
        span: Span {
            start: 50,
            end: 54,
        },
    },
    SpannedToken {
        kind: Ident(
            "E",
        ),
        span: Span {
            start: 55,
            end: 56,
        },
    },
    SpannedToken {
        kind: LeftBrace,
        span: Span {
            start: 57,
            end: 58,
        },
    },
    SpannedToken {
        kind: RightBrace,
        span: Span {
            start: 59,
            end: 60,
        },
    },
    SpannedToken {
        kind: Ident(
            "enum",
        ),
        span: Span {
            start: 64,
            end: 68,
        },
    },
    SpannedToken {
        kind: Ident(
            "F",
        ),
        span: Span {
            start: 69,
            end: 70,
        },
    },
    SpannedToken {
        kind: LeftBrace,
        span: Span {
            start: 71,
            end: 72,
        },
    },
    SpannedToken {
        kind: RightBrace,
        span: Span {
            start: 73,
            end: 74,
        },
    },
    SpannedToken {
        kind: Ident(
            "namespace",
        ),
        span: Span {
            start: 76,
            end: 85,
        },
    },
    SpannedToken {
        kind: Ident(
            "F",
        ),
        span: Span {
            start: 86,
            end: 87,
        },
    },
    SpannedToken {
        kind: LeftBrace,
        span: Span {
            start: 88,
            end: 89,
        },
    },
    SpannedToken {
        kind: Var,
        span: Span {
            start: 90,
            end: 93,
        },
    },
    SpannedToken {
        kind: Ident(
            "t",
        ),
        span: Span {
            start: 94,
            end: 95,
        },
    },
    SpannedToken {
        kind: Semicolon,
        span: Span {
            start: 95,
            end: 96,
        },
    },
    SpannedToken {
        kind: RightBrace,
        span: Span {
            start: 97,
            end: 98,
        },
    },
    SpannedToken {
        kind: Ident(
            "namespace",
        ),
        span: Span {
            start: 102,
            end: 111,
        },
    },
    SpannedToken {
        kind: Ident(
            "A",
        ),
        span: Span {
            start: 112,
            end: 113,
        },
    },
    SpannedToken {
        kind: LeftBrace,
        span: Span {
            start: 114,
            end: 115,
        },
```

#### ast

- ok: `False`
- truncated: `False`

```text
error: [UnsupportedModule] issue-399: TypeScript namespace/internal module declarations require module ownership before runtime lowering at 20..29
```

#### resolved

- ok: `False`
- truncated: `False`

```text
error: [UnsupportedModule] issue-399: TypeScript namespace/internal module declarations require module ownership before runtime lowering at 20..29
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
        "code": 2432,
        "category": "Error",
        "message": "In an enum with multiple declarations, only one declaration can omit an initializer for its first enum element.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/augmentedTypesEnum3.ts",
        "start": 166,
        "length": 1,
        "line": 17,
        "character": 5
      }
    ],
    "hints": [
      {
        "kind": "binding",
        "typeText": "any",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/augmentedTypesEnum3.ts",
        "start": 43,
        "length": 1,
        "line": 3,
        "character": 9,
        "name": "t"
      },
      {
        "kind": "binding",
        "typeText": "any",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/augmentedTypesEnum3.ts",
        "start": 94,
        "length": 1,
        "line": 8,
        "character": 19,
        "name": "t"
      },
      {
        "kind": "binding",
        "typeText": "any",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/augmentedTypesEnum3.ts",
        "start": 125,
        "length": 1,
        "line": 11,
        "character": 9,
        "name": "o"
      },
      {
        "kind": "binding",
        "typeText": "any",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/augmentedTypesEnum3.ts",
        "start": 195,
        "length": 1,
        "line": 20,
        "character": 9,
        "name": "p"
      }
    ],
    "typescriptVersion": "6.0.3"
  },
  "ast": {
    "topLevel": [
      {
        "kind": "ModuleDeclaration",
        "text": "namespace E {\r\n    var t;\r\n}",
        "line": 2,
        "character": 1
      },
      {
        "kind": "EnumDeclaration",
        "text": "enum E { }",
        "line": 5,
        "character": 1
      },
      {
        "kind": "EnumDeclaration",
        "text": "enum F { }",
        "line": 7,
        "character": 1
      },
      {
        "kind": "ModuleDeclaration",
        "text": "namespace F { var t; }",
        "line": 8,
        "character": 1
      },
      {
        "kind": "ModuleDeclaration",
        "text": "namespace A {\r\n    var o;\r\n}",
        "line": 10,
        "character": 1
      },
      {
        "kind": "EnumDeclaration",
        "text": "enum A {\r\n    b\r\n}",
        "line": 13,
        "character": 1
      },
      {
        "kind": "EnumDeclaration",
        "text": "enum A {\r\n    c\r\n}",
        "line": 16,
        "character": 1
      },
      {
        "kind": "ModuleDeclaration",
        "text": "namespace A {\r\n    var p;\r\n}",
        "line": 19,
        "character": 1
      }
    ],
    "pathToPosition": [
      {
        "kind": "SourceFile",
        "text": "namespace E {\r\n    var t;\r\n}\r\nenum E { }\r\n\r\nenum F { }\r\nnamespace F { var t; }\r\n\r\nnamespace A {\r\n    var o;\r\n}\r\nenum A {",
        "line": 2,
        "character": 1
      },
      {
        "kind": "ModuleDeclaration",
        "text": "namespace E {\r\n    var t;\r\n}",
        "line": 2,
        "character": 1
      }
    ]
  }
}
```

Stack trace:

```text
error: [UnsupportedModule] issue-399: TypeScript namespace/internal module declarations require module ownership before runtime lowering at 20..29
```

## Completion evidence

Commits:

- this commit

Validation result:

```text
command: env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/augmentedTypesEnum3.ts
result: pass; BuildPass, no compiler blocker
date: 2026-05-07

command: env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/augmentedTypesEnum3.ts --detail --no-dashboard-data
result: pass; executed=1, build_pass=1, unsupported=0, blocked=0
date: 2026-05-07
```

Remaining risks:

- TypeScript oracle reports TS2432 enum declaration diagnostics; semantic parity is outside this stale import-export blocker closure.
