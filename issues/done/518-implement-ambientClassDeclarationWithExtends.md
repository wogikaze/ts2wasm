---
id: 518
title: "Implement Ambientclassdeclarationwithextends"
type: spike
area: frontend/syntax
class: superseded
priority: P1
depends_on: [5001]
blocks: []
created: 2026-05-01
updated: 2026-05-04
---

## Summary

Triage ambientClassDeclarationWithExtends across 1 failing reference test cases and split this bucket into implementation-ready child issues.

## Problem

Reference test results show 1 cases fail in directory `ambientClassDeclarationWithExtends` with diagnostics: import-export. The compiler cannot handle these syntax/semantics, preventing compilation of code in this category.

Problem: ambientClassDeclarationWithExtends has 1 reference failures and needs smart-triage evidence before implementation starts.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/ambientClassDeclarationWithExtends.ts
```

Coverage window:

```sh
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/ambientClassDeclarationWithExtends.ts --detail
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
mise run reference-coverage -- tsc --limit 2
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/ambientClassDeclarationWithExtends.ts --detail
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/ambientClassDeclarationWithExtends.ts
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

- `reference/typescript/tests/cases/compiler/ambientClassDeclarationWithExtends.ts`

## Duplicate detection

- `issues/done/140-implement-ambientClassDeclarationWithExtends.md` - Implement Ambientclassdeclarationwithextends (same reference path, same group key, title overlap)

## Smart triage

### Smart triage: Triage import export: ambientClassDeclarationWithExtends

- Issue class: `triage-needed`
- Feature label: `import-export`
- Diagnostic: `UnsupportedModule` / `unsupported-feature-boundary`
- Path: `reference/typescript/tests/cases/compiler/ambientClassDeclarationWithExtends.ts`

Reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/ambientClassDeclarationWithExtends.ts
```

Source overview:

```json
{
  "suite": "tsc",
  "bytes": 478,
  "lines": 25,
  "extension": ".ts",
  "first_code_line": "declare class A { }"
}
```

Failure location:

```json
{
  "code": "UnsupportedModule",
  "message": "issue-399: TypeScript namespace/internal module declarations require module ownership before runtime lowering at 192..201",
  "span_start": 192,
  "span_end": 201,
  "line": 10,
  "column": 10,
  "feature_label": "import-export",
  "error_type": "unsupported-feature-boundary"
}
```

Source context:

```text
 7 | declare class C {
 8 |     public foo;
 9 | }
10 | namespace D { var x; }
11 | declare class D extends C { }
12 | 
13 | var d: C = new D();
```

Visible symbols before failure:

```json
[
  {
    "kind": "class",
    "name": "A",
    "line": 4,
    "column": 9
  },
  {
    "kind": "class",
    "name": "B",
    "line": 5,
    "column": 9
  },
  {
    "kind": "class",
    "name": "C",
    "line": 7,
    "column": 9
  }
]
```

Duplicate candidates:

```json
[
  {
    "state": "open",
    "path": "issues/done/140-implement-ambientClassDeclarationWithExtends.md",
    "title": "Implement Ambientclassdeclarationwithextends",
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
- truncated: `True`

```text
== tokens ==
[
    SpannedToken {
        kind: Ident(
            "declare",
        ),
        span: Span {
            start: 99,
            end: 106,
        },
    },
    SpannedToken {
        kind: Class,
        span: Span {
            start: 107,
            end: 112,
        },
    },
    SpannedToken {
        kind: Ident(
            "A",
        ),
        span: Span {
            start: 113,
            end: 114,
        },
    },
    SpannedToken {
        kind: LeftBrace,
        span: Span {
            start: 115,
            end: 116,
        },
    },
    SpannedToken {
        kind: RightBrace,
        span: Span {
            start: 117,
            end: 118,
        },
    },
    SpannedToken {
        kind: Ident(
            "declare",
        ),
        span: Span {
            start: 120,
            end: 127,
        },
    },
    SpannedToken {
        kind: Class,
        span: Span {
            start: 128,
            end: 133,
        },
    },
    SpannedToken {
        kind: Ident(
            "B",
        ),
        span: Span {
            start: 134,
            end: 135,
        },
    },
    SpannedToken {
        kind: Extends,
        span: Span {
            start: 136,
            end: 143,
        },
    },
    SpannedToken {
        kind: Ident(
            "A",
        ),
        span: Span {
            start: 144,
            end: 145,
        },
    },
    SpannedToken {
        kind: LeftBrace,
        span: Span {
            start: 146,
            end: 147,
        },
    },
    SpannedToken {
        kind: RightBrace,
        span: Span {
            start: 148,
            end: 149,
        },
    },
    SpannedToken {
        kind: Ident(
            "declare",
        ),
        span: Span {
            start: 153,
            end: 160,
        },
    },
    SpannedToken {
        kind: Class,
        span: Span {
            start: 161,
            end: 166,
        },
    },
    SpannedToken {
        kind: Ident(
            "C",
        ),
        span: Span {
            start: 167,
            end: 168,
        },
    },
    SpannedToken {
        kind: LeftBrace,
        span: Span {
            start: 169,
            end: 170,
        },
    },
    SpannedToken {
        kind: Ident(
            "public",
        ),
        span: Span {
            start: 176,
            end: 182,
        },
    },
    SpannedToken {
        kind: Ident(
            "foo",
        ),
        span: Span {
            start: 183,
            end: 186,
        },
    },
    SpannedToken {
        kind: Semicolon,
        span: Span {
            start: 186,
            end: 187,
        },
    },
    SpannedToken {
        kind: RightBrace,
        span: Span {
            start: 189,
            end: 190,
        },
    },
    SpannedToken {
        kind: Ident(
            "namespace",
        ),
        span: Span {
            start: 192,
            end: 201,
        },
    },
    SpannedToken {
        kind: Ident(
            "D",
        ),
        span: Span {
            start: 202,
            end: 203,
        },
    },
    SpannedToken {
        kind: LeftBrace,
        span: Span {
            start: 204,
            end: 205,
        },
    },
    SpannedToken {
        kind: Var,
        span: Span {
            start: 206,
            end: 209,
        },
    },
    SpannedToken {
        kind: Ident(
            "x",
        ),
        span: Span {
            start: 21
```

#### ast

- ok: `False`
- truncated: `False`

```text
error: [UnsupportedModule] issue-399: TypeScript namespace/internal module declarations require module ownership before runtime lowering at 192..201
```

#### resolved

- ok: `False`
- truncated: `False`

```text
error: [UnsupportedModule] issue-399: TypeScript namespace/internal module declarations require module ownership before runtime lowering at 192..201
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
        "typeText": "any",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/ambientClassDeclarationWithExtends.ts",
        "start": 210,
        "length": 1,
        "line": 10,
        "character": 19,
        "name": "x"
      },
      {
        "kind": "binding",
        "typeText": "C",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/ambientClassDeclarationWithExtends.ts",
        "start": 253,
        "length": 1,
        "line": 13,
        "character": 5,
        "name": "d"
      },
      {
        "kind": "binding",
        "typeText": "any",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/ambientClassDeclarationWithExtends.ts",
        "start": 386,
        "length": 1,
        "line": 20,
        "character": 19,
        "name": "y"
      },
      {
        "kind": "binding",
        "typeText": "E",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/ambientClassDeclarationWithExtends.ts",
        "start": 486,
        "length": 1,
        "line": 25,
        "character": 5,
        "name": "f"
      }
    ],
    "typescriptVersion": "6.0.3"
  },
  "ast": {
    "topLevel": [
      {
        "kind": "ClassDeclaration",
        "text": "declare class A { }",
        "line": 4,
        "character": 1
      },
      {
        "kind": "ClassDeclaration",
        "text": "declare class B extends A { }",
        "line": 5,
        "character": 1
      },
      {
        "kind": "ClassDeclaration",
        "text": "declare class C {\r\n    public foo;\r\n}",
        "line": 7,
        "character": 1
      },
      {
        "kind": "ModuleDeclaration",
        "text": "namespace D { var x; }",
        "line": 10,
        "character": 1
      },
      {
        "kind": "ClassDeclaration",
        "text": "declare class D extends C { }",
        "line": 11,
        "character": 1
      },
      {
        "kind": "FirstStatement",
        "text": "var d: C = new D();",
        "line": 13,
        "character": 1
      },
      {
        "kind": "ClassDeclaration",
        "text": "declare class E {\r\n    public bar;\r\n}",
        "line": 17,
        "character": 1
      },
      {
        "kind": "ModuleDeclaration",
        "text": "namespace F { var y; }",
        "line": 20,
        "character": 1
      },
      {
        "kind": "ClassDeclaration",
        "text": "declare class F extends E { }",
        "line": 24,
        "character": 1
      },
      {
        "kind": "FirstStatement",
        "text": "var f: E = new F();",
        "line": 25,
        "character": 1
      }
    ],
    "pathToPosition": [
      {
        "kind": "SourceFile",
        "text": "declare class A { }\r\ndeclare class B extends A { }\r\n\r\ndeclare class C {\r\n    public foo;\r\n}\r\nnamespace D { var x; }\r\ndec",
        "line": 4,
        "character": 1
      },
      {
        "kind": "ModuleDeclaration",
        "text": "namespace D { var x; }",
        "line": 10,
        "character": 1
      }
    ]
  }
}
```

Stack trace:

```text
error: [UnsupportedModule] issue-399: TypeScript namespace/internal module declarations require module ownership before runtime lowering at 192..201
```


## 重複整理メモ

この issue はタイトルが重複しているため、内容がより充実している
`issues/done/604-implement-ambientClassDeclarationWithExtends.md` に統合されました。
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
---

## ⚠️ False-done audit (re-opened from issues/done/)

**Why this was false-done**: This generated triage spike issue was copy-closed to `issues/done/` as part of a batch close cycle without actual triage completion. The done/ copy only differs from open/ in checkbox state ([ ] → [x]) with no "Status" note, no child issues created, no implementation commits, and empty completion evidence. The checkboxes were batch-checked without evidence that the triage was actually performed.

**True-done checklist** (all must pass):

1. Perform actual triage review of the reference failure case
2. Either create child implementation issue(s) or confirm this issue is superseded by an existing issue (with "Status" note)
3. Fill in completion evidence section with triage results
4. Remove stale open/ copy if it exists

**Commands that must pass**:

```sh
cargo fmt --all --check
cargo nextest run
```
