---
id: 594
title: "Implement Aliaswithinterfaceexportassignmentusedinvarinitializer"
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

Triage aliasWithInterfaceExportAssignmentUsedInVarInitializer across 1 failing reference test cases and split this bucket into implementation-ready child issues.

## Problem

Reference test results show 1 cases fail in directory `aliasWithInterfaceExportAssignmentUsedInVarInitializer` with diagnostics: import-export. The compiler cannot handle these syntax/semantics, preventing compilation of code in this category.

Problem: aliasWithInterfaceExportAssignmentUsedInVarInitializer has 1 reference failures and needs smart-triage evidence before implementation starts.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/aliasWithInterfaceExportAssignmentUsedInVarInitializer.ts
```

Coverage window:

```sh
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/aliasWithInterfaceExportAssignmentUsedInVarInitializer.ts --detail
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
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/aliasWithInterfaceExportAssignmentUsedInVarInitializer.ts --detail
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/aliasWithInterfaceExportAssignmentUsedInVarInitializer.ts
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

- `reference/typescript/tests/cases/compiler/aliasWithInterfaceExportAssignmentUsedInVarInitializer.ts`

## Duplicate detection

- `issues/done/129-implement-aliasWithInterfaceExportAssignmentUsedInVarInitializer.md` - Implement Aliaswithinterfaceexportassignmentusedinvarinitializer (same reference path, same group key, title overlap)
- `issues/done/508-implement-aliasWithInterfaceExportAssignmentUsedInVarInitializer.md` - Implement Aliaswithinterfaceexportassignmentusedinvarinitializer (same reference path, same feature label, same group key, title overlap)

## Smart triage

### Smart triage: Triage import export: aliasWithInterfaceExportAssignmentUsedInVarInitializer

- Issue class: `triage-needed`
- Feature label: `import-export`
- Diagnostic: `UnsupportedModule` / `unsupported-feature-boundary`
- Path: `reference/typescript/tests/cases/compiler/aliasWithInterfaceExportAssignmentUsedInVarInitializer.ts`

Reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/aliasWithInterfaceExportAssignmentUsedInVarInitializer.ts
```

Source overview:

```json
{
  "suite": "tsc",
  "bytes": 334,
  "lines": 11,
  "extension": ".ts",
  "first_code_line": "interface c {"
}
```

Failure location:

```json
{
  "code": "UnsupportedModule",
  "message": "issue-055: unsupported static export; module resolution and loading are not implemented at 152..158",
  "span_start": 152,
  "span_end": 158,
  "line": 7,
  "column": 7,
  "feature_label": "import-export",
  "error_type": "unsupported-feature-boundary"
}
```

Source context:

```text
 4 | interface c {
 5 |     q3: number;
 6 | }
 7 | export = c;
 8 | 
 9 | // @Filename: aliasWithInterfaceExportAssignmentUsedInVarInitializer_1.ts
10 | import moduleA = require("./aliasWithInterfaceExportAssignmentUsedInVarInitializer_0");
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
    "path": "issues/done/129-implement-aliasWithInterfaceExportAssignmentUsedInVarInitializer.md",
    "title": "Implement Aliaswithinterfaceexportassignmentusedinvarinitializer",
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
    "path": "issues/done/508-implement-aliasWithInterfaceExportAssignmentUsedInVarInitializer.md",
    "title": "Implement Aliaswithinterfaceexportassignmentusedinvarinitializer",
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
- truncated: `True`

```text
== tokens ==
[
    SpannedToken {
        kind: Ident(
            "interface",
        ),
        span: Span {
            start: 117,
            end: 126,
        },
    },
    SpannedToken {
        kind: Ident(
            "c",
        ),
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
        kind: Ident(
            "q3",
        ),
        span: Span {
            start: 136,
            end: 138,
        },
    },
    SpannedToken {
        kind: Colon,
        span: Span {
            start: 138,
            end: 139,
        },
    },
    SpannedToken {
        kind: Ident(
            "number",
        ),
        span: Span {
            start: 140,
            end: 146,
        },
    },
    SpannedToken {
        kind: Semicolon,
        span: Span {
            start: 146,
            end: 147,
        },
    },
    SpannedToken {
        kind: RightBrace,
        span: Span {
            start: 149,
            end: 150,
        },
    },
    SpannedToken {
        kind: Export,
        span: Span {
            start: 152,
            end: 158,
        },
    },
    SpannedToken {
        kind: Equal,
        span: Span {
            start: 159,
            end: 160,
        },
    },
    SpannedToken {
        kind: Ident(
            "c",
        ),
        span: Span {
            start: 161,
            end: 162,
        },
    },
    SpannedToken {
        kind: Semicolon,
        span: Span {
            start: 162,
            end: 163,
        },
    },
    SpannedToken {
        kind: Import,
        span: Span {
            start: 242,
            end: 248,
        },
    },
    SpannedToken {
        kind: Ident(
            "moduleA",
        ),
        span: Span {
            start: 249,
            end: 256,
        },
    },
    SpannedToken {
        kind: Equal,
        span: Span {
            start: 257,
            end: 258,
        },
    },
    SpannedToken {
        kind: Ident(
            "require",
        ),
        span: Span {
            start: 259,
            end: 266,
        },
    },
    SpannedToken {
        kind: LeftParen,
        span: Span {
            start: 266,
            end: 267,
        },
    },
    SpannedToken {
        kind: String(
            "./aliasWithInterfaceExportAssignmentUsedInVarInitializer_0",
        ),
        span: Span {
            start: 267,
            end: 327,
        },
    },
    SpannedToken {
        kind: RightParen,
        span: Span {
            start: 327,
            end: 328,
        },
    },
    SpannedToken {
        kind: Semicolon,
        span: Span {
            start: 328,
            end: 329,
        },
    },
    SpannedToken {
        kind: Var,
        span: Span {
            start: 331,
            end: 334,
        },
    },
    SpannedToken {
        kind: Ident(
            "d",
        ),
        span: Span {
            start: 335,
            end: 336,
        },
    },
    SpannedToken {
        kind: Equal,
        span: Span {
            start: 337,
            end: 338,
        },
    },
    SpannedToken {
        kind: Ident(
            "b",
        ),
        span: Span {
            start: 339,
            end: 340,
        },
    },
    SpannedToken {
        kind: Dot,
        span: Span {
            start: 340,
```

#### ast

- ok: `False`
- truncated: `False`

```text
error: [UnsupportedModule] issue-055: unsupported static export; module resolution and loading are not implemented at 152..158
```

#### resolved

- ok: `False`
- truncated: `False`

```text
error: [UnsupportedModule] issue-055: unsupported static export; module resolution and loading are not implemented at 152..158
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
        "message": "Cannot find module './aliasWithInterfaceExportAssignmentUsedInVarInitializer_0' or its corresponding type declarations.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/aliasWithInterfaceExportAssignmentUsedInVarInitializer.ts",
        "start": 267,
        "length": 60,
        "line": 10,
        "character": 26
      },
      {
        "code": 2304,
        "category": "Error",
        "message": "Cannot find name 'b'.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/aliasWithInterfaceExportAssignmentUsedInVarInitializer.ts",
        "start": 339,
        "length": 1,
        "line": 11,
        "character": 9
      }
    ],
    "hints": [
      {
        "kind": "binding",
        "typeText": "any",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/aliasWithInterfaceExportAssignmentUsedInVarInitializer.ts",
        "start": 335,
        "length": 1,
        "line": 11,
        "character": 5,
        "name": "d"
      }
    ],
    "typescriptVersion": "6.0.3"
  },
  "ast": {
    "topLevel": [
      {
        "kind": "InterfaceDeclaration",
        "text": "interface c {\r\n    q3: number;\r\n}",
        "line": 4,
        "character": 1
      },
      {
        "kind": "ExportAssignment",
        "text": "export = c;",
        "line": 7,
        "character": 1
      },
      {
        "kind": "ImportEqualsDeclaration",
        "text": "import moduleA = require(\"./aliasWithInterfaceExportAssignmentUsedInVarInitializer_0\");",
        "line": 10,
        "character": 1
      },
      {
        "kind": "FirstStatement",
        "text": "var d = b.q3;",
        "line": 11,
        "character": 1
      }
    ],
    "pathToPosition": [
      {
        "kind": "SourceFile",
        "text": "interface c {\r\n    q3: number;\r\n}\r\nexport = c;\r\n\r\n// @Filename: aliasWithInterfaceExportAssignmentUsedInVarInitializer_1",
        "line": 4,
        "character": 1
      },
      {
        "kind": "ExportAssignment",
        "text": "export = c;",
        "line": 7,
        "character": 1
      }
    ]
  }
}
```

Stack trace:

```text
error: [UnsupportedModule] issue-055: unsupported static export; module resolution and loading are not implemented at 152..158
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
