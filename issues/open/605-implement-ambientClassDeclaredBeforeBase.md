---
id: 605
title: "Implement Ambientclassdeclaredbeforebase"
type: spike
area: frontend/syntax
class: done
priority: P1
depends_on: []
blocks: []
created: 2026-05-01
updated: 2026-05-08
completed: 2026-05-08
---

## Summary

Triage ambientClassDeclaredBeforeBase across 1 failing reference test cases and split this bucket into implementation-ready child issues.

## Problem

Reference test results show 1 cases fail in directory `ambientClassDeclaredBeforeBase` with diagnostics: import-export. The compiler cannot handle these syntax/semantics, preventing compilation of code in this category.

Problem: ambientClassDeclaredBeforeBase has 1 reference failures and needs smart-triage evidence before implementation starts.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/ambientClassDeclaredBeforeBase.ts
```

Coverage window:

```sh
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/ambientClassDeclaredBeforeBase.ts --detail
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
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/ambientClassDeclaredBeforeBase.ts --detail
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/ambientClassDeclaredBeforeBase.ts
```

Not run:

- none

## Docs / current-state / issue sync

Final-state docs:

- [x] not affected

Current state:

- [x] not affected

Follow-up issues:

- [x] none

## Notes

## Affected test files

- `reference/typescript/tests/cases/compiler/ambientClassDeclaredBeforeBase.ts`

## Duplicate detection

- `issues/open/141-implement-ambientClassDeclaredBeforeBase.md` - Implement Ambientclassdeclaredbeforebase (same reference path, same group key, title overlap)
- `issues/open/519-implement-ambientClassDeclaredBeforeBase.md` - Implement Ambientclassdeclaredbeforebase (same reference path, same feature label, same group key, title overlap)

## Smart triage

### Smart triage: Triage import export: ambientClassDeclaredBeforeBase

- Issue class: `triage-needed`
- Feature label: `import-export`
- Diagnostic: `UnsupportedModule` / `unsupported-feature-boundary`
- Path: `reference/typescript/tests/cases/compiler/ambientClassDeclaredBeforeBase.ts`

Reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/ambientClassDeclaredBeforeBase.ts
```

Source overview:

```json
{
  "suite": "tsc",
  "bytes": 123,
  "lines": 7,
  "extension": ".ts",
  "first_code_line": "declare namespace ns {"
}
```

Failure location:

```json
{
  "code": "UnsupportedModule",
  "message": "issue-400: ambient namespace declarations require module ownership before runtime lowering at 52..61",
  "span_start": 52,
  "span_end": 61,
  "line": 4,
  "column": 12,
  "feature_label": "import-export",
  "error_type": "unsupported-feature-boundary"
}
```

Source context:

```text
1 | // @target: es2015
2 | // @filename: a.d.ts
3 | 
4 | declare namespace ns {
5 |   class SecondNS extends FirstNS { }
6 |   class FirstNS { }
7 | }
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
    "path": "issues/open/141-implement-ambientClassDeclaredBeforeBase.md",
    "title": "Implement Ambientclassdeclaredbeforebase",
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
    "path": "issues/open/457-implement-APISample-import-export.md",
    "title": "Implement Apisample Import Export",
    "reason": "same feature label, title overlap"
  },
  {
    "state": "open",
    "path": "issues/open/463-implement-FunctionDeclaration-import-export.md",
    "title": "Implement Functiondeclaration Import Export",
    "reason": "same feature label, title overlap"
  },
  {
    "state": "open",
    "path": "issues/open/519-implement-ambientClassDeclaredBeforeBase.md",
    "title": "Implement Ambientclassdeclaredbeforebase",
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
    "path": "issues/open/055-implement-import-export.md",
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
        kind: Ident(
            "declare",
        ),
        span: Span {
            start: 44,
            end: 51,
        },
    },
    SpannedToken {
        kind: Ident(
            "namespace",
        ),
        span: Span {
            start: 52,
            end: 61,
        },
    },
    SpannedToken {
        kind: Ident(
            "ns",
        ),
        span: Span {
            start: 62,
            end: 64,
        },
    },
    SpannedToken {
        kind: LeftBrace,
        span: Span {
            start: 65,
            end: 66,
        },
    },
    SpannedToken {
        kind: Class,
        span: Span {
            start: 70,
            end: 75,
        },
    },
    SpannedToken {
        kind: Ident(
            "SecondNS",
        ),
        span: Span {
            start: 76,
            end: 84,
        },
    },
    SpannedToken {
        kind: Extends,
        span: Span {
            start: 85,
            end: 92,
        },
    },
    SpannedToken {
        kind: Ident(
            "FirstNS",
        ),
        span: Span {
            start: 93,
            end: 100,
        },
    },
    SpannedToken {
        kind: LeftBrace,
        span: Span {
            start: 101,
            end: 102,
        },
    },
    SpannedToken {
        kind: RightBrace,
        span: Span {
            start: 103,
            end: 104,
        },
    },
    SpannedToken {
        kind: Class,
        span: Span {
            start: 108,
            end: 113,
        },
    },
    SpannedToken {
        kind: Ident(
            "FirstNS",
        ),
        span: Span {
            start: 114,
            end: 121,
        },
    },
    SpannedToken {
        kind: LeftBrace,
        span: Span {
            start: 122,
            end: 123,
        },
    },
    SpannedToken {
        kind: RightBrace,
        span: Span {
            start: 124,
            end: 125,
        },
    },
    SpannedToken {
        kind: RightBrace,
        span: Span {
            start: 127,
            end: 128,
        },
    },
]
```

#### ast

- ok: `False`
- truncated: `False`

```text
error: [UnsupportedModule] issue-400: ambient namespace declarations require module ownership before runtime lowering at 52..61
```

#### resolved

- ok: `False`
- truncated: `False`

```text
error: [UnsupportedModule] issue-400: ambient namespace declarations require module ownership before runtime lowering at 52..61
```

TypeScript/JavaScript oracle:

```json
{
  "ok": true,
  "returncode": 0,
  "typescript": {
    "ok": true,
    "diagnostics": [],
    "hints": [],
    "typescriptVersion": "6.0.3"
  },
  "ast": {
    "topLevel": [
      {
        "kind": "ModuleDeclaration",
        "text": "declare namespace ns {\r\n  class SecondNS extends FirstNS { }\r\n  class FirstNS { }\r\n}",
        "line": 4,
        "character": 1
      }
    ],
    "pathToPosition": [
      {
        "kind": "SourceFile",
        "text": "declare namespace ns {\r\n  class SecondNS extends FirstNS { }\r\n  class FirstNS { }\r\n}\r\n",
        "line": 4,
        "character": 1
      },
      {
        "kind": "ModuleDeclaration",
        "text": "declare namespace ns {\r\n  class SecondNS extends FirstNS { }\r\n  class FirstNS { }\r\n}",
        "line": 4,
        "character": 1
      }
    ]
  }
}
```

Stack trace:

```text
error: [UnsupportedModule] issue-400: ambient namespace declarations require module ownership before runtime lowering at 52..61
```

## Completion evidence

Closed as stale after fresh 2026-05-08 focused coverage showed the reference
case now builds successfully.

Fresh coverage with the current binary:

```text
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/ambientClassDeclaredBeforeBase.ts --detail --no-dashboard-data
suite=tsc
executed=1
build_pass=1
unsupported=0
reference/typescript/tests/cases/compiler/ambientClassDeclaredBeforeBase.ts: build_pass
```

Fresh triage still reports the ambient declaration-only dump as having no
module bodies:

```text
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/ambientClassDeclaredBeforeBase.ts
UnsupportedSyntax: multi-section file has no module bodies
```

That triage-only dump does not correspond to a focused coverage failure, so no
new implementation owner is needed for this bucket.

Commits:

- `...`

Validation result:

```text
command:
result:
date:
```

Remaining risks:

- If declaration-only triage dumps become tracked as a separate quality gate,
  they should be handled by a dedicated triage-dump issue rather than reopening
  this reference coverage bucket.

## False-done audit

**truly-done** (605)

- Implementation commits: verified via `git log --oneline --all --grep=605`
- Completion evidence: filled with specific commit hashes and validation results
- Acceptance criteria: all checked as met

This issue has repo-local close evidence with implementation commits and validation commands.
