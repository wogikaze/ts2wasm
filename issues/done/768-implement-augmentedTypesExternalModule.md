---
id: 768
title: "Implement Augmentedtypesexternalmodule"
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

## Summary

Closed this generated bucket as superseded by
`issues/open/5285-support-export-var-initializer-declarations.md`.

## Problem

Fresh triage shows the current first blocker is the initialized
`export var a = 1;` declaration in `augmentedTypesExternalModule1.ts`.
Existing issue 5285 already owns this exact initialized export-var parser
boundary.

Problem: the generated bucket remained blocked even though its executable work
is already tracked by issue 5285.

## Current failure

Representative reproduction:

```sh
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/augmentedTypesExternalModule1.ts
```

Coverage window:

```sh
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/augmentedTypesExternalModule1.ts --detail --no-dashboard-data
```

## Desired final state

This generated bucket is closed. Implement from
`issues/open/5285-support-export-var-initializer-declarations.md`.

## Scope

In scope:

- [x] Inspect the smart triage report below
- [x] Confirm whether existing open/done issues already cover this bucket
- [x] Supersede this generated bucket with the existing implementation-ready issue
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

- [x] Duplicate candidates below are confirmed and this issue is superseded
- [x] Superseding issue 5285 contains the implementation scope
- [x] Current triage evidence is recorded
- [x] Superseding issue acceptance names the export-var diagnostic change

## Validation

Required commands:

```sh
python scripts/manager.py update-issue-index --check
python scripts/manager.py check-issue-health
python scripts/manager.py check-issue-readiness -- --fail-ready-below 80
```

Impacted commands:

```sh
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/augmentedTypesExternalModule1.ts --detail --no-dashboard-data
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/augmentedTypesExternalModule1.ts
```

Not run:

- cargo fmt / nextest not run for this metadata-only issue lifecycle closure

## Docs / current-state / issue sync

Final-state docs:

- [x] not affected

Current state:

- [x] not affected

Follow-up issues:

- [x] `issues/open/5285-support-export-var-initializer-declarations.md`

## Notes

## Affected test files

- `reference/typescript/tests/cases/compiler/augmentedTypesExternalModule1.ts`

## Duplicate detection

Superseded by `issues/open/5285-support-export-var-initializer-declarations.md`.

Evidence:

- Current source: `export var a = 1;`
- Current diagnostic: `issue-055: unsupported variable export`
- Existing issue 5285 scope: initialized `export var name = expr;`

Related no-match issues:

- `issues/open/5283-support-entry-export-var-declarations.md` covers typed
  declaration-only `export var name: type;`.
- `issues/done/5175-support-export-let-destructuring-declarations.md` covers
  exported `let` destructuring.

## Smart triage

### Smart triage: Triage import export: augmentedTypesExternalModule1

- Issue class: `triage-needed`
- Feature label: `import-export`
- Diagnostic: `UnsupportedModule` / `unsupported-feature-boundary`
- Path: `reference/typescript/tests/cases/compiler/augmentedTypesExternalModule1.ts`

Reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/augmentedTypesExternalModule1.ts
```

Source overview:

```json
{
  "suite": "tsc",
  "bytes": 125,
  "lines": 5,
  "extension": ".ts",
  "first_code_line": "export var a = 1;"
}
```

Failure location:

```json
{
  "code": "UnsupportedModule",
  "message": "issue-055: unsupported variable export; module resolution and loading are not implemented at 36..42",
  "span_start": 36,
  "span_end": 42,
  "line": 3,
  "column": 3,
  "feature_label": "import-export",
  "error_type": "unsupported-feature-boundary"
}
```

Source context:

```text
1 | // @target: es2015
2 | //@module: amd
3 | export var a = 1;
4 | class c5 { public foo() { } }
5 | namespace c5 { } // should be ok everywhere
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
- truncated: `False`

```text
== tokens ==
[
    SpannedToken {
        kind: Export,
        span: Span {
            start: 36,
            end: 42,
        },
    },
    SpannedToken {
        kind: Var,
        span: Span {
            start: 43,
            end: 46,
        },
    },
    SpannedToken {
        kind: Ident(
            "a",
        ),
        span: Span {
            start: 47,
            end: 48,
        },
    },
    SpannedToken {
        kind: Equal,
        span: Span {
            start: 49,
            end: 50,
        },
    },
    SpannedToken {
        kind: Number(
            1,
        ),
        span: Span {
            start: 51,
            end: 52,
        },
    },
    SpannedToken {
        kind: Semicolon,
        span: Span {
            start: 52,
            end: 53,
        },
    },
    SpannedToken {
        kind: Class,
        span: Span {
            start: 55,
            end: 60,
        },
    },
    SpannedToken {
        kind: Ident(
            "c5",
        ),
        span: Span {
            start: 61,
            end: 63,
        },
    },
    SpannedToken {
        kind: LeftBrace,
        span: Span {
            start: 64,
            end: 65,
        },
    },
    SpannedToken {
        kind: Ident(
            "public",
        ),
        span: Span {
            start: 66,
            end: 72,
        },
    },
    SpannedToken {
        kind: Ident(
            "foo",
        ),
        span: Span {
            start: 73,
            end: 76,
        },
    },
    SpannedToken {
        kind: LeftParen,
        span: Span {
            start: 76,
            end: 77,
        },
    },
    SpannedToken {
        kind: RightParen,
        span: Span {
            start: 77,
            end: 78,
        },
    },
    SpannedToken {
        kind: LeftBrace,
        span: Span {
            start: 79,
            end: 80,
        },
    },
    SpannedToken {
        kind: RightBrace,
        span: Span {
            start: 81,
            end: 82,
        },
    },
    SpannedToken {
        kind: RightBrace,
        span: Span {
            start: 83,
            end: 84,
        },
    },
    SpannedToken {
        kind: Ident(
            "namespace",
        ),
        span: Span {
            start: 86,
            end: 95,
        },
    },
    SpannedToken {
        kind: Ident(
            "c5",
        ),
        span: Span {
            start: 96,
            end: 98,
        },
    },
    SpannedToken {
        kind: LeftBrace,
        span: Span {
            start: 99,
            end: 100,
        },
    },
    SpannedToken {
        kind: RightBrace,
        span: Span {
            start: 101,
            end: 102,
        },
    },
]
```

#### ast

- ok: `False`
- truncated: `False`

```text
error: [UnsupportedModule] issue-055: unsupported variable export; module resolution and loading are not implemented at 36..42
```

#### resolved

- ok: `False`
- truncated: `False`

```text
error: [UnsupportedModule] issue-055: unsupported variable export; module resolution and loading are not implemented at 36..42
```

TypeScript/JavaScript oracle:

```json
{
  "ok": true,
  "returncode": 0,
  "typescript": {
    "ok": true,
    "diagnostics": [],
    "hints": [
      {
        "kind": "binding",
        "typeText": "number",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/augmentedTypesExternalModule1.ts",
        "start": 47,
        "length": 1,
        "line": 3,
        "character": 12,
        "name": "a"
      }
    ],
    "typescriptVersion": "6.0.3"
  },
  "ast": {
    "topLevel": [
      {
        "kind": "FirstStatement",
        "text": "export var a = 1;",
        "line": 3,
        "character": 1
      },
      {
        "kind": "ClassDeclaration",
        "text": "class c5 { public foo() { } }",
        "line": 4,
        "character": 1
      },
      {
        "kind": "ModuleDeclaration",
        "text": "namespace c5 { }",
        "line": 5,
        "character": 1
      }
    ],
    "pathToPosition": [
      {
        "kind": "SourceFile",
        "text": "export var a = 1;\r\nclass c5 { public foo() { } }\r\nnamespace c5 { } // should be ok everywhere",
        "line": 3,
        "character": 1
      },
      {
        "kind": "FirstStatement",
        "text": "export var a = 1;",
        "line": 3,
        "character": 1
      },
      {
        "kind": "ExportKeyword",
        "text": "export",
        "line": 3,
        "character": 1
      }
    ]
  }
}
```

Stack trace:

```text
error: [UnsupportedModule] issue-055: unsupported variable export; module resolution and loading are not implemented at 36..42
```

## Completion evidence

Commits:

- this commit

Validation result:

```text
command: env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/augmentedTypesExternalModule1.ts
result: pass; issue-055 unsupported variable export, superseded by issue 5285
date: 2026-05-07

command: env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/augmentedTypesExternalModule1.ts --detail --no-dashboard-data
result: pass; executed=1, unsupported=1, UnsupportedSyntax=1
date: 2026-05-07
```

Remaining risks:

- Issue 5285 still needs implementation; this closure only removes the duplicate generated bucket from the blocked queue.
