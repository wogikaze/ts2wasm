---
id: 1041
title: "Implement Basetypewrappinginstantiationchain"
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

Triage baseTypeWrappingInstantiationChain across 1 failing reference test case and merge it into the existing implementation-ready class heritage generic parser issue.

## Problem

Fresh smart triage shows the current blocker is the same parser feature family as issue 5156: TypeScript type arguments in class heritage clauses are parsed as runtime operators. This case adds nested type arguments closed by a `RightShift` token.

Problem: `baseTypeWrappingInstantiationChain` is not a standalone implementation order; it is superseded by issue 5156.

## Current failure

Representative reproduction:

```sh
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/baseTypeWrappingInstantiationChain.ts
```

Current diagnostic:

```text
error: [UnsupportedSyntax] expected LeftBrace, got Some(Class) at 144..149
```

## Desired final state

This generated bucket is closed as superseded by `issues/done/5156-parse-generic-type-arguments-in-class-heritage.md`. Do not implement directly from this bucket.

## Scope

In scope:

- [x] Inspect the smart triage report below
- [x] Confirm whether existing open/done issues already cover this bucket
- [x] Merge the nested generic class heritage parser shape into issue 5156
- [x] Preserve exact reproduction commands and representative AST/diagnostic evidence in the existing child issue

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
- [x] Issue 5156 contains an exact `python scripts/manager.py reference-triage ...` command for this reference path
- [x] Issue 5156 includes failing path, diagnostic code, source context, visible symbols, and parser/TypeScript AST evidence
- [x] Issue 5156 acceptance names the exact reference path and diagnostic change

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
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/baseTypeWrappingInstantiationChain.ts
```

Not run:

- `cargo fmt --all --check`; issue merge only, no Rust implementation changed
- `cargo nextest run`; issue merge only, no implementation changed

## Docs / current-state / issue sync

Final-state docs:

- [x] not affected

Current state:

- [x] not affected

Follow-up issues:

- [x] updated: `issues/done/5156-parse-generic-type-arguments-in-class-heritage.md`

## Notes

## Affected test files

- `reference/typescript/tests/cases/compiler/baseTypeWrappingInstantiationChain.ts`

## Duplicate detection

- `issues/done/5156-parse-generic-type-arguments-in-class-heritage.md` already owns generic class heritage parsing and now includes the nested `RightShift` variant from this bucket.

## Smart triage

### Smart triage: Triage parser syntax: baseTypeWrappingInstantiationChain

- Issue class: `triage-needed`
- Feature label: `parser-syntax`
- Diagnostic: `UnsupportedSyntax` / `parser-or-frontend-unsupported`
- Path: `reference/typescript/tests/cases/compiler/baseTypeWrappingInstantiationChain.ts`

Reproduction:

```sh
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/baseTypeWrappingInstantiationChain.ts
```

Source context:

```text
2 | class CBaseBase<T3> {
3 |     constructor(x: Parameter<T3>) { }
4 | }
6 | class CBase<T2> extends CBaseBase<Wrapper<T2>> {
8 | }
10 | class Parameter<T4> {
```

Visible symbols before failure:

```json
[
  {
    "kind": "class",
    "name": "CBaseBase",
    "line": 2,
    "column": 1
  },
  {
    "kind": "class",
    "name": "CBase",
    "line": 6,
    "column": 1
  },
  {
    "kind": "class",
    "name": "Par",
    "line": 10,
    "column": 1
  }
]
```

Parser/oracle evidence:

```text
tokens: ok; nested `Wrapper<T2>>` is tokenized with a `RightShift`
ast: fail; UnsupportedSyntax expected LeftBrace, got Some(Class)
resolved: fail at same parser diagnostic
TypeScript oracle: no parse error; unrelated TS2564 for `Wrapper.property`
```

Resolution:

```text
Issue 5156 now owns the nested generic heritage parser contract for this reference case.
```

## Completion evidence

Commits:

- superseded by `issues/done/5156-parse-generic-type-arguments-in-class-heritage.md`

Validation result:

```text
command: python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/baseTypeWrappingInstantiationChain.ts
result: pass; reproduced current parser-syntax failure and merged nested heritage shape into issue 5156
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

