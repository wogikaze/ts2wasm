---
id: 771
title: "Implement Augmentedtypesmodules"
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
> Evidence: Empty completion evidence. No feat/fix commit for #771.

## Summary

Closed this generated import-export bucket as stale: fresh triage and focused
coverage show all representative reference files now build successfully.

## Problem

Fresh reference evidence no longer shows an import-export blocker for the
`augmentedTypesModules*` reference window.

Problem: the issue still sat in the blocked queue even though all five
representative cases are now build passes.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/augmentedTypesModules.ts
```

Coverage window:

```sh
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/augmentedTypesModules.ts --detail
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
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/augmentedTypesModules --detail --no-dashboard-data
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/augmentedTypesModules.ts
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

- `reference/typescript/tests/cases/compiler/augmentedTypesModules.ts`
- `reference/typescript/tests/cases/compiler/augmentedTypesModules2.ts`
- `reference/typescript/tests/cases/compiler/augmentedTypesModules3.ts`
- `reference/typescript/tests/cases/compiler/augmentedTypesModules3b.ts`
- `reference/typescript/tests/cases/compiler/augmentedTypesModules4.ts`

## Duplicate detection

No implementation child needed for the old import-export blocker. Fresh triage
reports `BuildPass`; focused coverage reports `build_pass=5`, `unsupported=0`,
and `blocked=0`.

TypeScript oracle still reports semantic duplicate-identifier TS2300 and
namespace-before-class/function merge TS2434 diagnostics. TS2434 is already
tracked by `issues/open/5330-report-namespace-before-class-merge-diagnostic.md`;
semantic parity is outside this stale import-export bucket closure.

Current coverage:

```text
executed=5
build_pass=5
semantic_pass=0
unsupported=0
blocked=0
unsupported_diagcodes=
unsupported_features=
semantic_enabled=0
reference/typescript/tests/cases/compiler/augmentedTypesModules.ts: build_pass
reference/typescript/tests/cases/compiler/augmentedTypesModules2.ts: build_pass
reference/typescript/tests/cases/compiler/augmentedTypesModules3.ts: build_pass
reference/typescript/tests/cases/compiler/augmentedTypesModules4.ts: build_pass
reference/typescript/tests/cases/compiler/augmentedTypesModules3b.ts: build_pass
```

## Current smart triage

### Smart triage: Build pass: augmentedTypesModules

- Issue class: `none`
- Feature label: `build-pass`
- Diagnostic: `BuildPass` / `pass`
- Path: `reference/typescript/tests/cases/compiler/augmentedTypesModules.ts`

Reproduction:

```sh
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/augmentedTypesModules.ts
```

Failure location:

```json
{
  "code": "BuildPass",
  "message": "ts2wasm build succeeded",
  "span_start": null,
  "span_end": null,
  "line": null,
  "column": null,
  "feature_label": "build-pass",
  "error_type": "pass"
}
```

## Stale generated smart triage

### Smart triage: Triage import export: augmentedTypesModules

- Issue class: `triage-needed`
- Feature label: `import-export`
- Diagnostic: `UnsupportedModule` / `unsupported-feature-boundary`
- Path: `reference/typescript/tests/cases/compiler/augmentedTypesModules.ts`

Reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/augmentedTypesModules.ts
```

Source overview:

```json
{
  "suite": "tsc",
  "bytes": 2211,
  "lines": 97,
  "extension": ".ts",
  "first_code_line": "namespace m1 { }"
}
```

Failure location:

```json
{
  "code": "UnsupportedModule",
  "message": "issue-399: TypeScript namespace/internal module declarations require module ownership before runtime lowering at 40..49",
  "span_start": 40,
  "span_end": 49,
  "line": 3,
  "column": 3,
  "feature_label": "import-export",
  "error_type": "unsupported-feature-boundary"
}
```

Source context:

```text
1 | // @target: es2015
2 | // module then var
3 | namespace m1 { }
4 | var m1 = 1; // Should be allowed
5 | 
6 | namespace m1a { var y = 2; } // error
```

Visible symbols before failure:

```json
[
  {
    "kind": "binding",
    "name": "na",
    "line": 2,
    "column": 16
  }
]
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
    "state": "open",
    "path": "issues/done/766-implement-augmentedTypesEnum-import-export.md",
    "title": "Implement Augmentedtypesenum Import Export",
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
            start: 40,
            end: 49,
        },
    },
    SpannedToken {
        kind: Ident(
            "m1",
        ),
        span: Span {
            start: 50,
            end: 52,
        },
    },
    SpannedToken {
        kind: LeftBrace,
        span: Span {
            start: 53,
            end: 54,
        },
    },
    SpannedToken {
        kind: RightBrace,
        span: Span {
            start: 55,
            end: 56,
        },
    },
    SpannedToken {
        kind: Var,
        span: Span {
            start: 58,
            end: 61,
        },
    },
    SpannedToken {
        kind: Ident(
            "m1",
        ),
        span: Span {
            start: 62,
            end: 64,
        },
    },
    SpannedToken {
        kind: Equal,
        span: Span {
            start: 65,
            end: 66,
        },
    },
    SpannedToken {
        kind: Number(
            1,
        ),
        span: Span {
            start: 67,
            end: 68,
        },
    },
    SpannedToken {
        kind: Semicolon,
        span: Span {
            start: 68,
            end: 69,
        },
    },
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
            "m1a",
        ),
        span: Span {
            start: 104,
            end: 107,
        },
    },
    SpannedToken {
        kind: LeftBrace,
        span: Span {
            start: 108,
            end: 109,
        },
    },
    SpannedToken {
        kind: Var,
        span: Span {
            start: 110,
            end: 113,
        },
    },
    SpannedToken {
        kind: Ident(
            "y",
        ),
        span: Span {
            start: 114,
            end: 115,
        },
    },
    SpannedToken {
        kind: Equal,
        span: Span {
            start: 116,
            end: 117,
        },
    },
    SpannedToken {
        kind: Number(
            2,
        ),
        span: Span {
            start: 118,
            end: 119,
        },
    },
    SpannedToken {
        kind: Semicolon,
        span: Span {
            start: 119,
            end: 120,
        },
    },
    SpannedToken {
        kind: RightBrace,
        span: Span {
            start: 121,
            end: 122,
        },
    },
    SpannedToken {
        kind: Var,
        span: Span {
            start: 133,
            end: 136,
        },
    },
    SpannedToken {
        kind: Ident(
            "m1a",
        ),
        span: Span {
            start: 137,
            end: 140,
        },
    },
    SpannedToken {
        kind: Equal,
        span: Span {
            start: 141,
            end: 142,
        },
    },
    SpannedToken {
        kind: Number(
            1,
        ),
        span: Span {
            start: 143,
            end: 144,
        },
    },
    SpannedToken {
        kind: Semicolon,
        span: Span {
            start: 144,
            end: 145,
        },
    },
    SpannedToken {
        kind: Ident(
            "namespace",
        ),
        span: Span {
            start: 158,
            end: 167,
        },
    },
    SpannedToken {
        kind: Ident(
            "m1b",
        ),
        span: Span {
            start: 168,
            end: 171,
```

#### ast

- ok: `False`
- truncated: `False`

```text
error: [UnsupportedModule] issue-399: TypeScript namespace/internal module declarations require module ownership before runtime lowering at 40..49
```

#### resolved

- ok: `False`
- truncated: `False`

```text
error: [UnsupportedModule] issue-399: TypeScript namespace/internal module declarations require module ownership before runtime lowering at 40..49
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
        "code": 2300,
        "category": "Error",
        "message": "Duplicate identifier 'm1a'.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/augmentedTypesModules.ts",
        "start": 104,
        "length": 3,
        "line": 6,
        "character": 11
      },
      {
        "code": 2300,
        "category": "Error",
        "message": "Duplicate identifier 'm1a'.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/augmentedTypesModules.ts",
        "start": 137,
        "length": 3,
        "line": 7,
        "character": 5
      },
      {
        "code": 2300,
        "category": "Error",
        "message": "Duplicate identifier 'm1b'.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/augmentedTypesModules.ts",
        "start": 168,
        "length": 3,
        "line": 9,
        "character": 11
      },
      {
        "code": 2300,
        "category": "Error",
        "message": "Duplicate identifier 'm1b'.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/augmentedTypesModules.ts",
        "start": 208,
        "length": 3,
        "line": 10,
        "character": 5
      },
      {
        "code": 2300,
        "category": "Error",
        "message": "Duplicate identifier 'm1d'.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/augmentedTypesModules.ts",
        "start": 337,
        "length": 3,
        "line": 17,
        "character": 11
      },
      {
        "code": 2300,
        "category": "Error",
        "message": "Duplicate identifier 'm1d'.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/augmentedTypesModules.ts",
        "start": 394,
        "length": 3,
        "line": 20,
        "character": 5
      },
      {
        "code": 2434,
        "category": "Error",
        "message": "A namespace declaration cannot be located prior to a class or function with which it is merged.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/augmentedTypesModules.ts",
        "start": 533,
        "length": 3,
        "line": 26,
        "character": 11
      },
      {
        "code": 2434,
        "category": "Error",
        "message": "A namespace declaration cannot be located prior to a class or function with which it is merged.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/augmentedTypesModules.ts",
        "start": 628,
        "length": 3,
        "line": 29,
        "character": 11
      },
      {
        "code": 2434,
        "category": "Error",
        "message": "A namespace declaration cannot be located prior to a class or function with which it is merged.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/augmentedTypesModules.ts",
        "start": 1192,
        "length": 3,
        "line": 52,
        "character": 11
      }
    ],
    "hints": [
      {
        "kind": "binding",
        "typeText": "number",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/augmentedTypesModules.ts",
        "start": 62,
        "length": 2,
        "line": 4,
        "character": 5,
        "name": "m1"
      },
      {
        "kind": "binding",
        "typeText": "number",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/augmentedTypesModules.ts",
        "start": 114,
        "length": 1,
        "line": 6,
        "character": 21,
        "name": "y"
      },
      {
        "kind": "binding",
        "typeText": "number",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/augmentedTypesModules.ts",
        "start": 137,
        "length": 3,
        "line": 7,
        "character": 5,
        "name": "m1a"
      },
      {
        "kind": "binding",
        "typeText": "number",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/augmentedTypesModules.ts",
        "start": 185,
        "length": 1,
        "line": 9,
        "character": 28,
        "name": "y"
      },
      {
        "kind": "binding",
        "typeText": "number",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/augmentedTypesModules.ts",
        "start": 208,
        "length": 3,
        "line": 10,
        "character": 5,
        "name": "m1b"
      },
      {
        "kind": "binding",
        "typeText": "number",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/augmentedTypesModules.ts",
        "start": 294,
        "length": 3,
        "line": 15,
        "character": 5,
        "name": "m1c"
      },
      {
        "kind": "binding",
        "typeText": "number",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/augmentedTypesModules.ts",
        "start": 394,
        "length": 3,
        "line": 20,
        "character": 5,
        "name": "m1d"
      },
      {
        "kind": "function",
        "typeText": "void",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/augmentedTypesModules.ts",
        "start": 467,
        "length": 2,
        "line": 24,
        "character": 10,
        "name": "m2"
      },
      {
        "kind": "binding",
        "typeText": "number",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/augmentedTypesModules.ts",
        "start": 543,
        "length": 1,
        "line": 26,
        "character": 21,
        "name": "y"
      },
      {
        "kind": "function",
        "typeText": "void",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/augmentedTypesModules.ts",
        "start": 562,
        "length": 3,
        "line": 27,
        "character": 10,
        "name": "m2a"
      },
      {
        "kind": "binding",
        "typeText": "number",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/augmentedTypesModules.ts",
        "start": 645,
        "length": 1,
        "line": 29,
        "character": 28,
        "name": "y"
      },
      {
        "kind": "function",
        "typeText": "void",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/augmentedTypesModules.ts",
        "start": 664,
        "length": 3,
        "line": 30,
        "character": 10,
        "name": "m2b"
      },
      {
        "kind": "function",
        "typeText": "void",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/augmentedTypesModules.ts",
        "start": 774,
        "length": 3,
        "line": 33,
        "character": 10,
        "name": "m2c"
      },
      {
        "kind": "binding",
        "typeText": "number",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/augmentedTypesModules.ts",
        "start": 814,
        "length": 1,
        "line": 34,
        "character": 28,
        "name": "y"
      },
      {
        "kind": "function",
        "typeText": "void",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/augmentedTypesModules.ts",
        "start": 863,
        "length": 3,
        "line": 37,
        "character": 18,
        "name": "m2d"
      },
      {
        "kind": "function",
        "typeText": "void",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/augmentedTypesModules.ts",
        "start": 897,
        "length": 3,
        "line": 39,
        "character": 18,
        "name": "m2e"
      },
      {
        "kind": "function",
        "typeText": "void",
        "file"
```

Stack trace:

```text
error: [UnsupportedModule] issue-399: TypeScript namespace/internal module declarations require module ownership before runtime lowering at 40..49
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
