---
id: 1039
title: "Implement Basetypeorderchecking"
type: spike
area: frontend/syntax
class: superseded
priority: P1
depends_on: []
blocks: []
created: 2026-05-01
updated: 2026-05-06
completed: 2026-05-06
status: done
---

## Summary

Triage baseTypeOrderChecking across 1 failing reference test case and split this generated bucket into an implementation-ready child issue.

## Problem

Fresh smart triage shows the current blocker is a parser failure for generic type arguments in a class heritage clause: `class Class4<T> extends Class3<T> {}`.

Problem: `baseTypeOrderChecking` is not a standalone implementation order; the executable parser slice is split to issue 5156.

## Current failure

Representative reproduction:

```sh
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/baseTypeOrderChecking.ts
```

Current diagnostic:

```text
error: [UnsupportedSyntax] expected LeftBrace, got None
```

## Desired final state

This generated bucket is closed as superseded by `issues/open/5156-parse-generic-type-arguments-in-class-heritage.md`. Do not implement directly from this bucket.

## Scope

In scope:

- [x] Inspect the smart triage report below
- [x] Confirm whether existing open/done issues already cover this bucket
- [x] Split one observable parser behavior into a child issue
- [x] Preserve exact reproduction commands and representative AST/diagnostic evidence in the child issue

Out of scope:

- Direct implementation from this generated bucket
- Broad multi-feature fixes without child issue split

## Affected paths

Expected:

- `crates/frontend/src/parser/statements_class.rs`
- `crates/frontend/src/parser/statements_ts.rs`
- `crates/frontend/src/parser/tests.rs`
- `fixtures/`

Do not touch:

- unrelated runtime/backend code unless `reference-triage` proves the failure is not frontend-owned

## Acceptance criteria

- [x] Duplicate candidates below are confirmed as no-match or this issue is superseded
- [x] Child issue 5156 contains an exact `python scripts/manager.py reference-triage ...` command
- [x] Child issue includes failing path, diagnostic code, source context, visible symbols, and parser/TypeScript AST evidence
- [x] Child issue acceptance names the exact reference path and diagnostic change

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
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/baseTypeOrderChecking.ts
```

Not run:

- `cargo fmt --all --check`; issue split only, no owned Rust code changed
- `cargo nextest run`; issue split only, no implementation changed

## Docs / current-state / issue sync

Final-state docs:

- [x] not affected

Current state:

- [x] not affected

Follow-up issues:

- [x] created: `issues/open/5156-parse-generic-type-arguments-in-class-heritage.md`

## Notes

## Affected test files

- `reference/typescript/tests/cases/compiler/baseTypeOrderChecking.ts`

## Duplicate detection

- `issues/open/059-implement-parser-syntax-extensions.md` is a broad parser umbrella, not an executable child for this exact class heritage type-argument failure.
- No existing implementation-ready issue matched the exact `extends Class3<T>` parser boundary.

## Smart triage

### Smart triage: Triage parser syntax: baseTypeOrderChecking

- Issue class: `triage-needed`
- Feature label: `parser-syntax`
- Diagnostic: `UnsupportedSyntax` / `parser-or-frontend-unsupported`
- Path: `reference/typescript/tests/cases/compiler/baseTypeOrderChecking.ts`

Reproduction:

```sh
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/baseTypeOrderChecking.ts
```

Source context:

```text
2 | var someVariable: Class4<Class2>;
22 | class Class3<T>
24 | {
26 |                public memberVariable: Class2;
28 | }
32 | class Class4<T> extends Class3<T>
34 | {
36 | }
```

Visible symbols before failure:

```json
[
  {
    "kind": "binding",
    "name": "someVariable",
    "line": 2,
    "column": 1
  },
  {
    "kind": "class",
    "name": "Class1",
    "line": 6,
    "column": 1
  },
  {
    "kind": "class",
    "name": "Class2",
    "line": 14,
    "column": 1
  },
  {
    "kind": "class",
    "name": "Class3",
    "line": 22,
    "column": 1
  },
  {
    "kind": "class",
    "name": "Class4",
    "line": 32,
    "column": 1
  }
]
```

Parser/oracle evidence:

```text
tokens: ok; `<T>` tokens are visible both after class names and in `extends Class3<T>`
ast: fail; UnsupportedSyntax expected LeftBrace, got None
resolved: fail at same parser diagnostic
TypeScript oracle: no parse error; reports TS2564 for `memberVariable` definite assignment
```

Resolution:

```text
Issue 5156 now owns the concrete parser contract: TypeScript type arguments in class heritage clauses are erased/skipped before class body parsing.
```

## Completion evidence

Commits:

- superseded by `issues/open/5156-parse-generic-type-arguments-in-class-heritage.md`

Validation result:

```text
command: python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/baseTypeOrderChecking.ts
result: pass; reproduced current parser-syntax failure and split issue 5156
date: 2026-05-06
```

Remaining risks:

- Issue 5156 still needs implementation.

## False-done audit

Date: 2026-05-06

Classification: truly-done.

Audit result: retained in `issues/done/`. This issue has repo-local close evidence
(completion evidence with validation commands) or proper superseded classification
with child issues in `issues/open/`. The acceptance criteria documented in the issue
are satisfied by the repo-local evidence cited in the completion evidence section.

Future-work tracking: no untracked future-work item was identified in this issue
during this metadata/evidence audit.

