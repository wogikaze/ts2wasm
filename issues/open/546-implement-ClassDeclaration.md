---
id: 546
title: "Implement Classdeclaration"
type: spike
area: frontend/syntax
class: triage-needed
priority: P1
depends_on: []
blocks: []
created: 2026-05-01
updated: 2026-05-01
---

## Summary

Triage ClassDeclaration across 11 failing reference test cases and split this bucket into implementation-ready child issues.

## Problem

Reference test results show 11 cases fail in directory `ClassDeclaration` with diagnostics: parser-syntax. The compiler cannot handle these syntax/semantics, preventing compilation of code in this category.

Problem: ClassDeclaration has 11 reference failures and needs smart-triage evidence before implementation starts.

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
mise run reference-coverage -- tsc --limit 22
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/ClassDeclaration10.ts --detail
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/ClassDeclaration10.ts
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

- `issues/done/072-implement-ClassDeclaration.md` - Implement Classdeclaration (same reference path, same feature label, same group key, title overlap)
- `issues/done/073-implement-ClassDeclarationWithInvalidConstOnPropertyDeclaration.md` - Implement Classdeclarationwithinvalidconstonpropertydeclaration (same feature label, same group key, title overlap)
- `issues/done/084-implement-abstractClassUnionInstantiation.md` - Implement Abstractclassunioninstantiation (same feature label, same group key, title overlap)
- `issues/done/086-implement-abstractPropertyBasics.md` - Implement Abstractpropertybasics (same feature label, same group key, title overlap)
- `issues/done/088-implement-abstractPropertyNegative.md` - Implement Abstractpropertynegative (same feature label, same group key, title overlap)
- `issues/done/091-implement-accessInstanceMemberFromStaticMethod.md` - Implement Accessinstancememberfromstaticmethod (same feature label, same group key, title overlap)
- `issues/done/092-implement-accessOverriddenBaseClassMember.md` - Implement Accessoverriddenbaseclassmember (same feature label, same group key, title overlap)
- `issues/done/093-implement-accessStaticMemberFromInstanceMethod.md` - Implement Accessstaticmemberfrominstancemethod (same feature label, same group key, title overlap)
- `issues/done/166-implement-ambiguousCallsWhereReturnTypesAgree.md` - Implement Ambiguouscallswherereturntypesagree (same feature label, same group key, title overlap)
- `issues/done/185-implement-anyIdenticalToItself.md` - Implement Anyidenticaltoitself (same feature label, same group key, title overlap)

## Smart triage

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
    "path": "issues/done/072-implement-ClassDeclaration.md",
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
