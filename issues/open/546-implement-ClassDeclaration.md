---
id: 546
title: "Implement Classdeclaration"
type: spike
area: frontend/syntax
class: done
priority: P1
depends_on: []
blocks: []
created: 2026-05-01
updated: 2026-05-07
completed: 2026-05-07
---

## Summary

Close the generated `ClassDeclaration` bucket as stale: fresh coverage shows the representative window now builds successfully.

## Problem

Reference test results used to show 11 cases failing in directory `ClassDeclaration` with diagnostics: parser-syntax. Fresh coverage on 2026-05-07 shows the current path-filter window now builds successfully.

Problem: this generated bucket is stale. The current compiler advances through the old class declaration parser blockers.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/ClassDeclaration10.ts
```

Coverage window:

```sh
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/ClassDeclaration10.ts --detail
```

## Desired final state

This generated bucket is either split into implementation-ready child issues or superseded by an existing open/done issue with matching evidence. Do not implement directly from this bucket.

## Scope

In scope:

- [x] Inspect the smart triage report below
- [x] Confirm whether existing open/done issues already cover this bucket
- [x] Close as stale build-pass bucket
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

- [x] Duplicate candidates below are confirmed as no-match for required implementation work
- [x] No child issue needed because the current path-filter window is build-pass
- [x] This issue includes affected paths, diagnostic classification, and parser/TypeScript AST evidence
- [x] Coverage names the exact reference window and build-pass result

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run
```

Impacted commands:

```sh
mise run reference-coverage -- tsc --limit 22
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/ClassDeclaration10.ts --detail
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/ClassDeclaration10.ts
```

Not run:

- `cargo fmt --all --check` and `cargo nextest run`; this close is an
  issue-lifecycle-only stale build-pass update, so focused reference and issue
  checks were used instead.

## Docs / current-state / issue sync

Final-state docs:

- [x] not affected

Current state:

- [x] not affected

Follow-up issues:

- [x] none

## Notes

## Affected test files

- `reference/typescript/tests/cases/compiler/ClassDeclaration10.ts`
- `reference/typescript/tests/cases/compiler/ClassDeclaration21.ts`
- `reference/typescript/tests/cases/compiler/ClassDeclaration11.ts`
- `reference/typescript/tests/cases/compiler/ClassDeclaration14.ts`
- `reference/typescript/tests/cases/compiler/ClassDeclaration13.ts`
- `reference/typescript/tests/cases/compiler/ClassDeclaration15.ts`
- `reference/typescript/tests/cases/compiler/ClassDeclaration9.ts`
- `reference/typescript/tests/cases/compiler/ClassDeclaration22.ts`
- `reference/typescript/tests/cases/compiler/ClassDeclaration26.ts`
- `reference/typescript/tests/cases/compiler/ClassDeclaration8.ts`
- ... and 1 more files

## Duplicate detection

- No implementation child issue is needed for the affected files.
- `#072`,
  `issues/open/460-implement-ClassDeclaration.md`, and
  `issues/open/781-implement-ClassDeclaration.md` are historical duplicates
  for the same bucket; the current focused window is build-pass.

- `#072` - Implement Classdeclaration (same reference path, same feature label, same group key, title overlap)
- `#073` - Implement Classdeclarationwithinvalidconstonpropertydeclaration (same feature label, same group key, title overlap)
- `issues/open/084-implement-abstractClassUnionInstantiation.md` - Implement Abstractclassunioninstantiation (same feature label, same group key, title overlap)
- `issues/open/086-implement-abstractPropertyBasics.md` - Implement Abstractpropertybasics (same feature label, same group key, title overlap)
- `issues/open/088-implement-abstractPropertyNegative.md` - Implement Abstractpropertynegative (same feature label, same group key, title overlap)
- `issues/open/091-implement-accessInstanceMemberFromStaticMethod.md` - Implement Accessinstancememberfromstaticmethod (same feature label, same group key, title overlap)
- `#092` - Implement Accessoverriddenbaseclassmember (same feature label, same group key, title overlap)
- `issues/open/093-implement-accessStaticMemberFromInstanceMethod.md` - Implement Accessstaticmemberfrominstancemethod (same feature label, same group key, title overlap)
- `issues/open/166-implement-ambiguousCallsWhereReturnTypesAgree.md` - Implement Ambiguouscallswherereturntypesagree (same feature label, same group key, title overlap)
- `issues/open/185-implement-anyIdenticalToItself.md` - Implement Anyidenticaltoitself (same feature label, same group key, title overlap)

## Smart triage

Generated 2026-05-07.

```text
command:
env TS2WASM_BINARY=/home/wogikaze/wgkz/ts2wasm/target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/ClassDeclaration --detail --no-dashboard-data

result:
pass; executed=14, build_pass=14, unsupported=0

representative triage:
env TS2WASM_BINARY=/home/wogikaze/wgkz/ts2wasm/target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/ClassDeclaration10.ts

representative result:
BuildPass / pass; ts2wasm build succeeded

compiler evidence:
tokens: ok; class C with constructor and foo declarations tokenizes
ast: ok; ClassDecl C contains constructor and foo function members
resolved: ok; constructor is recognized and foo is a ClassMethod
TypeScript oracle: TS2390/TS2391 source diagnostics only
```

## Historical smart triage

### Smart triage: Triage parser syntax: ClassDeclaration10

- Issue class: `triage-needed`
- Feature label: `parser-syntax`
- Diagnostic: `UnsupportedSyntax` / `parser-or-frontend-unsupported`
- Path: `reference/typescript/tests/cases/compiler/ClassDeclaration10.ts`

Reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/ClassDeclaration10.ts
```

Source overview:

```json
{
  "suite": "tsc",
  "bytes": 76,
  "lines": 6,
  "extension": ".ts",
  "first_code_line": "class C {"
}
```

Failure location:

```json
{
  "code": "UnsupportedSyntax",
  "message": "expected LeftBrace, got Some(Semicolon) at 66..67",
  "span_start": 66,
  "span_end": 67,
  "line": 5,
  "column": 2,
  "feature_label": "parser-syntax",
  "error_type": "parser-or-frontend-unsupported"
}
```

Source context:

```text
2 | // @strict: false
3 | class C {
4 |    constructor();
5 |    foo();
6 | }
```

Visible symbols before failure:

```json
[
  {
    "kind": "class",
    "name": "C",
    "line": 3,
    "column": 1
  }
]
```

Duplicate candidates:

```json
[
  {
    "state": "open",
    "path": "#072",
    "title": "Implement Classdeclaration",
    "reason": "same reference path, same feature label"
  },
  {
    "state": "open",
    "path": "issues/open/442-implement-parser-syntax.md",
    "title": "Implement parser syntax extensions",
    "reason": "same feature label, title overlap"
  },
  {
    "state": "open",
    "path": "issues/done/460-implement-ClassDeclaration.md",
    "title": "Implement Classdeclaration",
    "reason": "same reference path, same feature label"
  },
  {
    "state": "open",
    "path": "issues/done/464-implement-FunctionDeclaration-parser-syntax.md",
    "title": "Implement Functiondeclaration Parser Syntax",
    "reason": "same feature label, title overlap"
  },
  {
    "state": "done",
    "path": "issues/open/059-implement-parser-syntax-extensions.md",
    "title": "Implement parser syntax extensions for TypeScript and advanced JS",
    "reason": "same feature label, title overlap"
  },
  {
    "state": "done",
    "path": "issues/done/065-implement-parser-syntax.md",
    "title": "Implement parser syntax extensions",
    "reason": "same feature label, title overlap"
  },
  {
    "state": "done",
    "path": "issues/done/065a-merge-duplicate-parser-syntax-issue-into-059.md",
    "title": "Merge duplicate parser syntax issue into 059",
    "reason": "same feature label, title overlap"
  },
  {
    "state": "done",
    "path": "issues/done/200-implement-parser-syntax.md",
    "title": "Implement parser syntax extensions",
    "reason": "same feature label, title overlap"
  },
  {
    "state": "done",
    "path": "issues/done/243-implement-numeric-literal-separator-parser.md",
    "title": "Implement numeric literal separator parser support",
    "reason": "same feature label, title overlap"
  },
  {
    "state": "done",
    "path": "issues/done/244-implement-bigint-literal-parser-classification.md",
    "title": "Implement BigInt literal parser classification",
    "reason": "same feature label, title overlap"
  }
]
```

Error-specific suggestions:

- Start at lexer/parser support and add a minimal fixture for the exact source construct at the failing span.
- Use `dump --tokens` and the TypeScript AST path to decide whether this is tokenization, precedence, or statement dispatch.

Compiler dumps:

#### tokens

- ok: `True`
- truncated: `False`

```text
== tokens ==
[
    SpannedToken {
        kind: Class,
        span: Span {
            start: 39,
            end: 44,
        },
    },
    SpannedToken {
        kind: Ident(
            "C",
        ),
        span: Span {
            start: 45,
            end: 46,
        },
    },
    SpannedToken {
        kind: LeftBrace,
        span: Span {
            start: 47,
            end: 48,
        },
    },
    SpannedToken {
        kind: Ident(
            "constructor",
        ),
        span: Span {
            start: 53,
            end: 64,
        },
    },
    SpannedToken {
        kind: LeftParen,
        span: Span {
            start: 64,
            end: 65,
        },
    },
    SpannedToken {
        kind: RightParen,
        span: Span {
            start: 65,
            end: 66,
        },
    },
    SpannedToken {
        kind: Semicolon,
        span: Span {
            start: 66,
            end: 67,
        },
    },
    SpannedToken {
        kind: Ident(
            "foo",
        ),
        span: Span {
            start: 72,
            end: 75,
        },
    },
    SpannedToken {
        kind: LeftParen,
        span: Span {
            start: 75,
            end: 76,
        },
    },
    SpannedToken {
        kind: RightParen,
        span: Span {
            start: 76,
            end: 77,
        },
    },
    SpannedToken {
        kind: Semicolon,
        span: Span {
            start: 77,
            end: 78,
        },
    },
    SpannedToken {
        kind: RightBrace,
        span: Span {
            start: 80,
            end: 81,
        },
    },
]
```

#### ast

- ok: `False`
- truncated: `False`

```text
error: [UnsupportedSyntax] expected LeftBrace, got Some(Semicolon) at 66..67
```

#### resolved

- ok: `False`
- truncated: `False`

```text
error: [UnsupportedSyntax] expected LeftBrace, got Some(Semicolon) at 66..67
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
        "code": 2390,
        "category": "Error",
        "message": "Constructor implementation is missing.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/ClassDeclaration10.ts",
        "start": 53,
        "length": 11,
        "line": 4,
        "character": 4
      },
      {
        "code": 2391,
        "category": "Error",
        "message": "Function implementation is missing or not immediately following the declaration.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/ClassDeclaration10.ts",
        "start": 72,
        "length": 3,
        "line": 5,
        "character": 4
      }
    ],
    "hints": [],
    "typescriptVersion": "6.0.3"
  },
  "ast": {
    "topLevel": [
      {
        "kind": "ClassDeclaration",
        "text": "class C {\r\n   constructor();\r\n   foo();\r\n}",
        "line": 3,
        "character": 1
      }
    ],
    "pathToPosition": [
      {
        "kind": "SourceFile",
        "text": "class C {\r\n   constructor();\r\n   foo();\r\n}",
        "line": 3,
        "character": 1
      },
      {
        "kind": "ClassDeclaration",
        "text": "class C {\r\n   constructor();\r\n   foo();\r\n}",
        "line": 3,
        "character": 1
      },
      {
        "kind": "Constructor",
        "text": "constructor();",
        "line": 4,
        "character": 4
      }
    ]
  }
}
```

Stack trace:

```text
error: [UnsupportedSyntax] expected LeftBrace, got Some(Semicolon) at 66..67
```

## Completion evidence

Fill only when moving to `done/`.

Commits:

- Closed as stale build-pass bucket; no child issue created.

Validation result:

```text
command:
env TS2WASM_BINARY=/home/wogikaze/wgkz/ts2wasm/target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/ClassDeclaration --detail --no-dashboard-data
result:
pass; executed=14, build_pass=14, unsupported=0
date:
2026-05-07

command:
env TS2WASM_BINARY=/home/wogikaze/wgkz/ts2wasm/target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/ClassDeclaration10.ts
result:
pass; BuildPass / pass, ast/resolved dumps succeed; TypeScript oracle reports expected source diagnostics TS2390/TS2391
date:
2026-05-07
```

Remaining risks:

- none
---

## ⚠️ False-done audit (re-opened from issues/done/)

**Why this was false-done**: This generated triage spike issue was copy-closed to `issues/done/` as part of a batch close cycle without actual triage completion. The old done/ copy only changed checkbox state without adding a status note, child issues, implementation commits, or completion evidence. This pass fills the missing evidence and closes it as stale build-pass.

**True-done checklist** (all must pass):

1. [x] Perform actual triage review of the reference failure case
2. [x] Either create child implementation issue(s) or confirm this issue is superseded by an existing issue (with "Status" note)
3. [x] Fill in completion evidence section with triage results
4. [x] Remove stale open/ copy if it exists

**Commands that must pass**:

```sh
cargo fmt --all --check
cargo nextest run
```
