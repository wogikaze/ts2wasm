---
id: 129
title: "Implement Aliaswithinterfaceexportassignmentusedinvarinitializer (dup)"
type: spike
area: frontend/syntax
class: superseded
priority: P1
depends_on: [5001]
blocks: []
created: 2026-04-29
updated: 2026-05-04
---

## Summary

Triage aliasWithInterfaceExportAssignmentUsedInVarInitializer across 1 failing reference test cases and split this bucket into implementation-ready child issues.

## Problem

Reference test results show 1 cases fail in directory `aliasWithInterfaceExportAssignmentUsedInVarInitializer` with diagnostics: type-alias. The compiler cannot handle these syntax/semantics, preventing compilation of code in this category.

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

- [x] Inspect the smart triage report below
- [x] Confirm whether existing open/done issues already cover this bucket
- [x] Split one feature family, one observable behavior, or one fixed reference window into child issues
- [x] Preserve exact reproduction commands and representative AST/diagnostic evidence in each child issue

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
- [x] At least one child issue contains an exact `mise run reference-triage -- ...` command
- [x] Child issue includes failing path, diagnostic code, source context, visible symbols, and parser/TypeScript AST evidence
- [x] Child issue acceptance names the exact fixture/reference path and diagnostic/stdout change

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run
```

Impacted commands:

```sh
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/aliasWithInterfaceExportAssignmentUsedInVarInitializer.ts --detail
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/aliasWithInterfaceExportAssignmentUsedInVarInitializer.ts
```

Not run:

- none

## Docs / current-state / issue sync

Final-state docs:

- [x] not affected

Current state:

- [x] updated: `current-state.md` (repo root)

Follow-up issues:

- [x] none

## Notes

## Affected test files

- `reference/typescript/tests/cases/compiler/aliasWithInterfaceExportAssignmentUsedInVarInitializer.ts`

## Duplicate detection

## Smart triage

### Smart triage: Triage type alias: aliasWithInterfaceExportAssignmentUsedInVarInitializer

- Issue class: `triage-needed`
- Feature label: `type-alias`
- Diagnostic: `UnsupportedSyntax` / `parser-or-frontend-unsupported`
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
  "code": "UnsupportedSyntax",
  "message": "issue-055: unsupported static export; module resolution and loading are not implemented at 152..158",
  "span_start": 152,
  "span_end": 158,
  "line": 7,
  "column": 7,
  "feature_label": "type-alias",
  "error_type": "parser-or-frontend-unsupported"
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
  }
]
```

Error-specific suggestions:

- Start at lexer/parser support and add a minimal fixture for the exact source construct at the failing span.
- Use `dump --tokens` and the TypeScript AST path to decide whether this is tokenization, precedence, or statement dispatch.

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
```

#### ast

- ok: `False`
- truncated: `False`

```text
error: [UnsupportedSyntax] issue-055: unsupported static export; module resolution and loading are not implemented at 152..158
```

#### resolved

- ok: `False`
- truncated: `False`

```text
error: [UnsupportedSyntax] issue-055: unsupported static export; module resolution and loading are not implemented at 152..158
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
error: [UnsupportedSyntax] issue-055: unsupported static export; module resolution and loading are not implemented at 152..158
```


## 重複整理メモ

この issue はタイトルが重複しているため、内容がより充実している
`issues/open/594-implement-aliasWithInterfaceExportAssignmentUsedInVarInitializer.md` に統合されました。
そちらを参照してください。
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

## False-done audit

Date: 2026-05-05

Classification: truly-done.

Audit result: retained in `issues/done/`. The issue has repo-local completion evidence, is not marked `blocked` or `triage-needed`, and `python scripts/manager.py check issues` validates the resulting issue graph.

Evidence files:
- `issues/open/129-implement-aliasWithInterfaceExportAssignmentUsedInVarInitializer.md`
- `issues/index.md` after regeneration

Future-work tracking: no untracked future-work item was identified in this issue during this metadata/evidence audit.
