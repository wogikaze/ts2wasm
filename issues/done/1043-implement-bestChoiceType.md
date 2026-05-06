---
id: 1043
title: "Implement Bestchoicetype"
type: spike
area: frontend/syntax
class: superseded
priority: P2
depends_on: [5160]
blocks: []
created: 2026-05-01
updated: 2026-05-06
status: done
completed: 2026-05-06
---

## Summary

Triage bestChoiceType across 1 failing reference test cases and split this bucket into implementation-ready child issues.

## Problem

Reference test results show 1 cases fail in directory `bestChoiceType` with diagnostics: unknown-unsupported. The compiler cannot handle these syntax/semantics, preventing compilation of code in this category.

Problem: bestChoiceType has 1 reference failures and needs smart-triage evidence before implementation starts.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/bestChoiceType.ts
```

Coverage window:

```sh
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/bestChoiceType.ts --detail
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
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/bestChoiceType.ts --detail
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/bestChoiceType.ts
```

Not run:

- none

## Docs / current-state / issue sync

Final-state docs:

- [x] not affected

Current state:

- [x] not affected

Follow-up issues:

- [x] created/updated: `issues/done/5160-lower-plain-ternary-conditional-expressions.md`

## Notes

## Affected test files

- `reference/typescript/tests/cases/compiler/bestChoiceType.ts`

## Duplicate detection

- none found by path/title/feature scan

## Smart triage

Generated on 2026-05-06.

- Path: `reference/typescript/tests/cases/compiler/bestChoiceType.ts`
- Diagnostic: `UnsupportedSyntax` / `parser-or-frontend-unsupported`
- Failure: `ternary operator not yet supported at 317..327`
- Source context: `let y = x ? x : [];` in `function f2()`
- TypeScript AST path: `FunctionDeclaration -> Block -> FirstStatement -> VariableDeclarationList -> VariableDeclaration -> ConditionalExpression -> Identifier(x)`
- Split child: `issues/done/5160-lower-plain-ternary-conditional-expressions.md`

## Completion evidence

Fill only when moving to `done/`.

Commits:

- `pending`

Validation result:

```text
command: python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/bestChoiceType.ts
result: pass; current blocker identified as builtin resolver rejection of `Expr::Ternary`, split to issue 5160
date: 2026-05-06
```

Remaining risks:

- Best-choice TypeScript inference expectations need follow-up triage after issue 5160 advances the pipeline beyond the current ternary unsupported diagnostic.
