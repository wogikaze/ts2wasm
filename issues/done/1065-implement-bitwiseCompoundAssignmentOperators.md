---
id: 1065
title: "Implement Bitwisecompoundassignmentoperators"
type: spike
area: frontend/syntax
class: superseded
priority: P2
depends_on: [5178]
blocks: []
created: 2026-05-01
updated: 2026-05-06
status: done
completed: 2026-05-06
---

## Summary

Triage bitwiseCompoundAssignmentOperators across 1 failing reference test cases and split this bucket into implementation-ready child issues.

## Problem

Reference test results show 1 cases fail in directory `bitwiseCompoundAssignmentOperators` with diagnostics: unknown-unsupported. The compiler cannot handle these syntax/semantics, preventing compilation of code in this category.

Problem: bitwiseCompoundAssignmentOperators has 1 reference failures and needs smart-triage evidence before implementation starts.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/bitwiseCompoundAssignmentOperators.ts
```

Coverage window:

```sh
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/bitwiseCompoundAssignmentOperators.ts --detail
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
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/bitwiseCompoundAssignmentOperators.ts --detail
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/bitwiseCompoundAssignmentOperators.ts
```

Not run:

- none

## Docs / current-state / issue sync

Final-state docs:

- [x] not affected

Current state:

- [x] not affected

Follow-up issues:

- [x] added: `issues/done/5178-parse-bitwise-compound-assignment-operators.md`

## Notes

## Affected test files

- `reference/typescript/tests/cases/compiler/bitwiseCompoundAssignmentOperators.ts`

## Duplicate detection

- Generic `unknown-unsupported` buckets are not matches; they share only the generated feature label.
- `issues/open/661-implement-arithAssignTyping.md` is related but not an exact match. Its first current parser blocker is arithmetic `*=` in a different reference file, while this bucket stops at bitwise `^=`.
- `issues/done/5164-parse-exponentiation-compound-assignment.md` is related but owns only `**=`.
- `issues/done/5170-support-bitwise-or-binary-lowering.md` is not a match because it explicitly excludes `|=` compound assignment.

## Smart triage

Generated on 2026-05-06.

- Path: `reference/typescript/tests/cases/compiler/bitwiseCompoundAssignmentOperators.ts`
- Diagnostic: `UnsupportedSyntax` / `parser-or-frontend-unsupported`
- Current compiler message: `unsupported expression: Some(SpannedToken { kind: Equal, span: Span { start: 47, end: 48 } }) at 49..50`
- First failing source line: `a ^= a;`
- Visible symbols before failure: binding `a` initialized to `true`, binding `b` initialized to `1`
- Compiler evidence: token dump emits `Ident("a")`, `Caret`, `Equal`, `Ident("a")`; AST/resolved construction fails at `Equal` before representing `^=`.
- TypeScript oracle: reports `TS2447` for boolean `^=`, `TS2362` for invalid left-hand operands, and `TS2363` for invalid right-hand operands; the same file exercises `&=` and `|=`.
- Superseding child: `issues/done/5178-parse-bitwise-compound-assignment-operators.md`

## Completion evidence

Fill only when moving to `done/`.

Commits:

- `pending`

Validation result:

```text
command: python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/bitwiseCompoundAssignmentOperators.ts
result: pass; current blocker identified as bitwise compound assignment parser syntax, split to issue 5178
date: 2026-05-06
```

Remaining risks:

- Later triage may expose operand typing diagnostics after issue 5178 advances past the parser blocker.

## False-done audit

Date: 2026-05-06

Classification: truly-done.

Audit result: retained in `issues/done/`. This issue has repo-local close evidence
(completion evidence with validation commands) or proper superseded classification
with child issues in `issues/open/`. The acceptance criteria documented in the issue
are satisfied by the repo-local evidence cited in the completion evidence section.

Future-work tracking: no untracked future-work item was identified in this issue
during this metadata/evidence audit.

