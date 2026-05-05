---
id: 595
title: "Implement Aliasesinsystemmodule"
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

Triage aliasesInSystemModule across 2 failing reference test cases and split this bucket into implementation-ready child issues.

## Problem

Reference test results show 2 cases fail in directory `aliasesInSystemModule` with diagnostics: import-export. The compiler cannot handle these syntax/semantics, preventing compilation of code in this category.

Problem: aliasesInSystemModule has 2 reference failures and needs smart-triage evidence before implementation starts.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/aliasesInSystemModule2.ts
```

Coverage window:

```sh
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/aliasesInSystemModule2.ts --detail
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
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/aliasesInSystemModule2.ts --detail
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/aliasesInSystemModule2.ts
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

- `reference/typescript/tests/cases/compiler/aliasesInSystemModule2.ts`
- `reference/typescript/tests/cases/compiler/aliasesInSystemModule1.ts`

## Duplicate detection

- `issues/done/130-implement-aliasesInSystemModule.md` - Implement Aliasesinsystemmodule (same reference path, same group key, title overlap)
- `issues/done/509-implement-aliasesInSystemModule.md` - Implement Aliasesinsystemmodule (same reference path, same feature label, same group key, title overlap)

## Smart triage

### Smart triage: Triage import export: aliasesInSystemModule2

- Issue class: `triage-needed`
- Feature label: `import-export`
- Diagnostic: `UnsupportedModule` / `unsupported-feature-boundary`
- Path: `reference/typescript/tests/cases/compiler/aliasesInSystemModule2.ts`

Reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/aliasesInSystemModule2.ts
```

Source overview:

```json
{
  "suite": "tsc",
  "bytes": 343,
  "lines": 18,
  "extension": ".ts",
  "first_code_line": "import {alias} from \"foo\";"
}
```

Failure location:

```json
{
  "code": "UnsupportedModule",
  "message": "issue-055: unsupported default import; module resolution and loading are not implemented at 92..98",
  "span_start": 92,
  "span_end": 98,
  "line": 6,
  "column": 1,
  "feature_label": "import-export",
  "error_type": "unsupported-feature-boundary"
}
```

Source context:

```text
3 | // @isolatedModules: true
4 | 
5 | import {alias} from "foo";
6 | import cls = alias.Class;
7 | export import cls2 = alias.Class;
8 | 
9 | let x = new alias.Class();
```

Visible symbols before failure:

```json
[
  {
    "kind": "import",
    "name": "foo",
    "line": 5,
    "column": 1
  }
]
```

Duplicate candidates:

```json
[
  {
    "state": "open",
    "path": "issues/done/130-implement-aliasesInSystemModule.md",
    "title": "Implement Aliasesinsystemmodule",
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
    "path": "issues/done/509-implement-aliasesInSystemModule.md",
    "title": "Implement Aliasesinsystemmodule",
    "reason": "same reference path, same feature label"
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
        kind: Import,
        span: Span {
            start: 65,
            end: 71,
        },
    },
    SpannedToken {
        kind: LeftBrace,
        span: Span {
            start: 72,
            end: 73,
        },
    },
    SpannedToken {
        kind: Ident(
            "alias",
        ),
        span: Span {
            start: 73,
            end: 78,
        },
    },
    SpannedToken {
        kind: RightBrace,
        span: Span {
            start: 78,
            end: 79,
        },
    },
    SpannedToken {
        kind: Ident(
            "from",
        ),
        span: Span {
            start: 80,
            end: 84,
        },
    },
    SpannedToken {
        kind: String(
            "foo",
        ),
        span: Span {
            start: 85,
            end: 90,
        },
    },
    SpannedToken {
        kind: Semicolon,
        span: Span {
            start: 90,
            end: 91,
        },
    },
    SpannedToken {
        kind: Import,
        span: Span {
            start: 92,
            end: 98,
        },
    },
    SpannedToken {
        kind: Ident(
            "cls",
        ),
        span: Span {
            start: 99,
            end: 102,
        },
    },
    SpannedToken {
        kind: Equal,
        span: Span {
            start: 103,
            end: 104,
        },
    },
    SpannedToken {
        kind: Ident(
            "alias",
        ),
        span: Span {
            start: 105,
            end: 110,
        },
    },
    SpannedToken {
        kind: Dot,
        span: Span {
            start: 110,
            end: 111,
        },
    },
    SpannedToken {
        kind: Ident(
            "Class",
        ),
        span: Span {
            start: 111,
            end: 116,
        },
    },
    SpannedToken {
        kind: Semicolon,
        span: Span {
            start: 116,
            end: 117,
        },
    },
    SpannedToken {
        kind: Export,
        span: Span {
            start: 118,
            end: 124,
        },
    },
    SpannedToken {
        kind: Import,
        span: Span {
            start: 125,
            end: 131,
        },
    },
    SpannedToken {
        kind: Ident(
            "cls2",
        ),
        span: Span {
            start: 132,
            end: 136,
        },
    },
    SpannedToken {
        kind: Equal,
        span: Span {
            start: 137,
            end: 138,
        },
    },
    SpannedToken {
        kind: Ident(
            "alias",
        ),
        span: Span {
            start: 139,
            end: 144,
        },
    },
    SpannedToken {
        kind: Dot,
        span: Span {
            start: 144,
            end: 145,
        },
    },
    SpannedToken {
        kind: Ident(
            "Class",
        ),
        span: Span {
            start: 145,
            end: 150,
        },
    },
    SpannedToken {
        kind: Semicolon,
        span: Span {
            start: 150,
            end: 151,
        },
    },
    SpannedToken {
        kind: Let,
        span: Span {
            start: 153,
            end: 156,
        },
    },
    SpannedToken {
        kind: Ident(
            "x",
        ),
        span: Span {
            start: 157,
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
        kind: New,
        s
```

#### ast

- ok: `False`
- truncated: `False`

```text
error: [UnsupportedModule] issue-055: unsupported default import; module resolution and loading are not implemented at 92..98
```

#### resolved

- ok: `False`
- truncated: `False`

```text
error: [UnsupportedModule] issue-055: unsupported default import; module resolution and loading are not implemented at 92..98
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
        "message": "Cannot find module 'foo' or its corresponding type declarations.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/aliasesInSystemModule2.ts",
        "start": 85,
        "length": 5,
        "line": 5,
        "character": 21
      }
    ],
    "hints": [
      {
        "kind": "binding",
        "typeText": "any",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/aliasesInSystemModule2.ts",
        "start": 157,
        "length": 1,
        "line": 9,
        "character": 5,
        "name": "x"
      },
      {
        "kind": "binding",
        "typeText": "any",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/aliasesInSystemModule2.ts",
        "start": 184,
        "length": 1,
        "line": 10,
        "character": 5,
        "name": "y"
      },
      {
        "kind": "binding",
        "typeText": "any",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/aliasesInSystemModule2.ts",
        "start": 203,
        "length": 1,
        "line": 11,
        "character": 5,
        "name": "z"
      },
      {
        "kind": "binding",
        "typeText": "any",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/aliasesInSystemModule2.ts",
        "start": 275,
        "length": 1,
        "line": 15,
        "character": 7,
        "name": "x"
      },
      {
        "kind": "binding",
        "typeText": "any",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/aliasesInSystemModule2.ts",
        "start": 304,
        "length": 1,
        "line": 16,
        "character": 7,
        "name": "y"
      },
      {
        "kind": "binding",
        "typeText": "any",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/aliasesInSystemModule2.ts",
        "start": 326,
        "length": 1,
        "line": 17,
        "character": 7,
        "name": "z"
      }
    ],
    "typescriptVersion": "6.0.3"
  },
  "ast": {
    "topLevel": [
      {
        "kind": "ImportDeclaration",
        "text": "import {alias} from \"foo\";",
        "line": 5,
        "character": 1
      },
      {
        "kind": "ImportEqualsDeclaration",
        "text": "import cls = alias.Class;",
        "line": 6,
        "character": 1
      },
      {
        "kind": "ImportEqualsDeclaration",
        "text": "export import cls2 = alias.Class;",
        "line": 7,
        "character": 1
      },
      {
        "kind": "FirstStatement",
        "text": "let x = new alias.Class();",
        "line": 9,
        "character": 1
      },
      {
        "kind": "FirstStatement",
        "text": "let y = new cls();",
        "line": 10,
        "character": 1
      },
      {
        "kind": "FirstStatement",
        "text": "let z = new cls2();",
        "line": 11,
        "character": 1
      },
      {
        "kind": "ModuleDeclaration",
        "text": "namespace M {\n  export import cls = alias.Class;\n  let x = new alias.Class();\n  let y = new cls(); \n  let z = new cls2()",
        "line": 13,
        "character": 1
      }
    ],
    "pathToPosition": [
      {
        "kind": "SourceFile",
        "text": "import {alias} from \"foo\";\nimport cls = alias.Class;\nexport import cls2 = alias.Class;\n\nlet x = new alias.Class();\nlet y",
        "line": 5,
        "character": 1
      },
      {
        "kind": "ImportEqualsDeclaration",
        "text": "import cls = alias.Class;",
        "line": 6,
        "character": 1
      }
    ]
  }
}
```

Stack trace:

```text
error: [UnsupportedModule] issue-055: unsupported default import; module resolution and loading are not implemented at 92..98
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
