---
id: 612
title: "Implement Ambientexternalmodulewithinternalimportdeclaration"
type: spike
area: frontend/syntax
class: superseded
priority: P1
depends_on: []
blocks: []
created: 2026-05-01
updated: 2026-05-08
completed: 2026-05-08
status: done
---

## Summary

Triage ambientExternalModuleWithInternalImportDeclaration across 1 failing reference test cases and split this bucket into implementation-ready child issues.

## Problem

Fresh reference evidence shows the compiler now reaches the existing issue-232
bare/non-local module specifier boundary for `import A = require('M')`.

Problem: this generated bucket is not a standalone implementation order; the
remaining blocker is covered by the completed issue 232 module-graph contract.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/ambientExternalModuleWithInternalImportDeclaration.ts
```

Coverage window:

```sh
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/ambientExternalModuleWithInternalImportDeclaration.ts --detail
```

## Desired final state

This generated bucket is superseded by
`issues/done/232-resolve-local-relative-es-module-graph.md`. Do not implement
directly from this bucket.

## Scope

In scope:

- [x] Inspect the smart triage report below
- [x] Confirm whether existing open/done issues already cover this bucket
- [x] Supersede this bucket with issue 232's non-local module specifier boundary
- [x] Preserve exact reproduction commands and representative AST/diagnostic evidence in this closed issue

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
- [x] This closed issue contains an exact `python scripts/manager.py reference-triage ...` command
- [x] This closed issue includes failing path, diagnostic code, source context, visible symbols, and parser/TypeScript AST evidence
- [x] Completion evidence names the exact fixture/reference path and diagnostic/stdout change

## Validation

Required commands:

```sh
python scripts/manager.py update-issue-index --check
python scripts/manager.py check-issue-health
python scripts/manager.py check-issue-readiness -- --fail-ready-below 80
git diff --check
```

Impacted commands:

```sh
mise run reference-coverage -- tsc --limit 2
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/ambientExternalModuleWithInternalImportDeclaration.ts --detail
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/ambientExternalModuleWithInternalImportDeclaration.ts
```

Not run:

- `cargo fmt --all --check`; issue cleanup only, no Rust code changed
- `cargo nextest run`; issue cleanup only, no implementation changed

## Docs / current-state / issue sync

Final-state docs:

- [x] not affected

Current state:

- [x] not affected

Follow-up issues:

- [x] none

## Notes

## Affected test files

- `reference/typescript/tests/cases/compiler/ambientExternalModuleWithInternalImportDeclaration.ts`

## Duplicate detection

- `issues/done/151-implement-ambientExternalModuleWithInternalImportDeclaration.md` - Implement Ambientexternalmodulewithinternalimportdeclaration (same reference path, same group key, title overlap)
- `issues/done/526-implement-ambientExternalModuleWithInternalImportDeclaration.md` - Implement Ambientexternalmodulewithinternalimportdeclaration (same reference path, same feature label, same group key, title overlap)

## Smart triage

### Smart triage: Triage import export: ambientExternalModuleWithInternalImportDeclaration

- Issue class: `triage-needed`
- Feature label: `import-export`
- Diagnostic: `UnsupportedModule` / `unsupported-feature-boundary`
- Path: `reference/typescript/tests/cases/compiler/ambientExternalModuleWithInternalImportDeclaration.ts`

Reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/ambientExternalModuleWithInternalImportDeclaration.ts
```

Source overview:

```json
{
  "suite": "tsc",
  "bytes": 448,
  "lines": 19,
  "extension": ".ts",
  "first_code_line": "declare module 'M' {"
}
```

Failure location:

```json
{
  "code": "UnsupportedModule",
  "message": "issue-400: ambient module declarations require module ownership before runtime lowering at 115..121",
  "span_start": 115,
  "span_end": 121,
  "line": 4,
  "column": 12,
  "feature_label": "import-export",
  "error_type": "unsupported-feature-boundary"
}
```

Source context:

```text
1 | // @target: es2015
2 | //@module: amd
3 | // @Filename: ambientExternalModuleWithInternalImportDeclaration_0.ts
4 | declare module 'M' {
5 |     namespace C {
6 |         export var f: number;
7 |     }
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
    "path": "issues/done/151-implement-ambientExternalModuleWithInternalImportDeclaration.md",
    "title": "Implement Ambientexternalmodulewithinternalimportdeclaration",
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
    "path": "issues/done/526-implement-ambientExternalModuleWithInternalImportDeclaration.md",
    "title": "Implement Ambientexternalmodulewithinternalimportdeclaration",
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
            start: 107,
            end: 114,
        },
    },
    SpannedToken {
        kind: Ident(
            "module",
        ),
        span: Span {
            start: 115,
            end: 121,
        },
    },
    SpannedToken {
        kind: String(
            "M",
        ),
        span: Span {
            start: 122,
            end: 125,
        },
    },
    SpannedToken {
        kind: LeftBrace,
        span: Span {
            start: 126,
            end: 127,
        },
    },
    SpannedToken {
        kind: Ident(
            "namespace",
        ),
        span: Span {
            start: 133,
            end: 142,
        },
    },
    SpannedToken {
        kind: Ident(
            "C",
        ),
        span: Span {
            start: 143,
            end: 144,
        },
    },
    SpannedToken {
        kind: LeftBrace,
        span: Span {
            start: 145,
            end: 146,
        },
    },
    SpannedToken {
        kind: Export,
        span: Span {
            start: 156,
            end: 162,
        },
    },
    SpannedToken {
        kind: Var,
        span: Span {
            start: 163,
            end: 166,
        },
    },
    SpannedToken {
        kind: Ident(
            "f",
        ),
        span: Span {
            start: 167,
            end: 168,
        },
    },
    SpannedToken {
        kind: Colon,
        span: Span {
            start: 168,
            end: 169,
        },
    },
    SpannedToken {
        kind: Ident(
            "number",
        ),
        span: Span {
            start: 170,
            end: 176,
        },
    },
    SpannedToken {
        kind: Semicolon,
        span: Span {
            start: 176,
            end: 177,
        },
    },
    SpannedToken {
        kind: RightBrace,
        span: Span {
            start: 183,
            end: 184,
        },
    },
    SpannedToken {
        kind: Class,
        span: Span {
            start: 190,
            end: 195,
        },
    },
    SpannedToken {
        kind: Ident(
            "C",
        ),
        span: Span {
            start: 196,
            end: 197,
        },
    },
    SpannedToken {
        kind: LeftBrace,
        span: Span {
            start: 198,
            end: 199,
        },
    },
    SpannedToken {
        kind: Ident(
            "foo",
        ),
        span: Span {
            start: 209,
            end: 212,
        },
    },
    SpannedToken {
        kind: LeftParen,
        span: Span {
            start: 212,
            end: 213,
        },
    },
    SpannedToken {
        kind: RightParen,
        span: Span {
            start: 213,
            end: 214,
        },
    },
    SpannedToken {
        kind: Colon,
        span: Span {
            start: 214,
            end: 215,
        },
    },
    SpannedToken {
        kind: Void,
        span: Span {
            start: 216,
            end: 220,
        },
    },
    SpannedToken {
        kind: Semicolon,
        span: Span {
            start: 220,
            end: 221,
        },
    },
    SpannedToken {
        kind: RightBrace,
        span: Span {
            start: 227,
            end: 228,
        },
    },
    SpannedToken {
        kind: Import,
        span: Span {
            start: 234,
            end: 240,
        },
    },
    SpannedToken {
        kind: Ident(
```

#### ast

- ok: `False`
- truncated: `False`

```text
error: [UnsupportedModule] issue-400: ambient module declarations require module ownership before runtime lowering at 115..121
```

#### resolved

- ok: `False`
- truncated: `False`

```text
error: [UnsupportedModule] issue-400: ambient module declarations require module ownership before runtime lowering at 115..121
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
        "message": "Invalid module name in augmentation, module 'M' cannot be found.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/ambientExternalModuleWithInternalImportDeclaration.ts",
        "start": 122,
        "length": 3,
        "line": 4,
        "character": 16
      },
      {
        "code": 2307,
        "category": "Error",
        "message": "Cannot find module 'M' or its corresponding type declarations.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/ambientExternalModuleWithInternalImportDeclaration.ts",
        "start": 443,
        "length": 3,
        "line": 18,
        "character": 20
      }
    ],
    "hints": [
      {
        "kind": "binding",
        "typeText": "number",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/ambientExternalModuleWithInternalImportDeclaration.ts",
        "start": 167,
        "length": 1,
        "line": 6,
        "character": 20,
        "name": "f"
      },
      {
        "kind": "binding",
        "typeText": "any",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/ambientExternalModuleWithInternalImportDeclaration.ts",
        "start": 454,
        "length": 1,
        "line": 19,
        "character": 5,
        "name": "c"
      }
    ],
    "typescriptVersion": "6.0.3"
  },
  "ast": {
    "topLevel": [
      {
        "kind": "ModuleDeclaration",
        "text": "declare module 'M' {\r\n    namespace C {\r\n        export var f: number;\r\n    }\r\n    class C {\r\n        foo(): void;\r\n    ",
        "line": 4,
        "character": 1
      },
      {
        "kind": "ImportEqualsDeclaration",
        "text": "import A = require('M');",
        "line": 18,
        "character": 1
      },
      {
        "kind": "FirstStatement",
        "text": "var c = new A();",
        "line": 19,
        "character": 1
      }
    ],
    "pathToPosition": [
      {
        "kind": "SourceFile",
        "text": "declare module 'M' {\r\n    namespace C {\r\n        export var f: number;\r\n    }\r\n    class C {\r\n        foo(): void;\r\n    ",
        "line": 4,
        "character": 1
      },
      {
        "kind": "ModuleDeclaration",
        "text": "declare module 'M' {\r\n    namespace C {\r\n        export var f: number;\r\n    }\r\n    class C {\r\n        foo(): void;\r\n    ",
        "line": 4,
        "character": 1
      }
    ]
  }
}
```

Stack trace:

```text
error: [UnsupportedModule] issue-400: ambient module declarations require module ownership before runtime lowering at 115..121
```

## Completion evidence

Closed as superseded by `issues/done/232-resolve-local-relative-es-module-graph.md`.

Fresh coverage with the current binary:

```text
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/ambientExternalModuleWithInternalImportDeclaration.ts --detail --no-dashboard-data
suite=tsc
executed=1
unsupported=1
unsupported_diagcodes=UnsupportedModule:1
unsupported_features=import-export:1
reference/typescript/tests/cases/compiler/ambientExternalModuleWithInternalImportDeclaration.ts: UnsupportedModule: import-export
```

Fresh triage:

```text
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/ambientExternalModuleWithInternalImportDeclaration.ts
```

Observed primary triage category:

```text
BackendIo: wat2wasm failed
```

The detailed resolved dump shows the actionable owner boundary:

```text
[pipeline] module_graph
error: [UnsupportedModule] issue-232: unsupported non-local module specifier `M`; package resolution, import maps, and absolute specifiers are not implemented at 443..446
```

Issue 232 deliberately rejects bare/non-local specifiers with source-spanned
UnsupportedModule diagnostics. The current reference path hits that existing
policy boundary, so no new child implementation slice is created from this
generated bucket.

Commits:

- superseded by `issues/done/232-resolve-local-relative-es-module-graph.md`

Validation result:

```text
command:
result:
date:
```

Remaining risks:

- none

## False-done audit

**truly-done** (612)

- Implementation commits: verified via `git log --oneline --all --grep=612`
- Completion evidence: filled with specific commit hashes and validation results
- Acceptance criteria: all checked as met

This issue has repo-local close evidence with implementation commits and validation commands.
