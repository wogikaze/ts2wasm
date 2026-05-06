---
id: 1036
title: "Implement Baseexpressiontypeparameters"
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

Triage baseExpressionTypeParameters across 1 failing reference test cases and split this bucket into implementation-ready child issues.

## Problem

Reference test results show 1 case failing in directory `baseExpressionTypeParameters` with diagnostics: parser-syntax. Fresh smart triage shows the current blocker is a top-level `<string>Spec.prop;` angle-bracket type assertion expression statement.

Problem: `baseExpressionTypeParameters` is not a standalone implementation order; the executable parser slice is split to issue 5154.

## Current failure

Representative reproduction:

```sh
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/baseExpressionTypeParameters.ts
```

Coverage window:

```sh
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/baseExpressionTypeParameters.ts --detail
```

## Desired final state

This generated bucket is closed as superseded by an implementation-ready child issue. Do not implement directly from this bucket.

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
- [x] At least one child issue contains an exact `python scripts/manager.py reference-triage ...` command
- [x] Child issue includes failing path, diagnostic code, source context, visible symbols, and parser/TypeScript AST evidence
- [x] Child issue acceptance names the exact fixture/reference path and diagnostic/stdout change

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
mise run reference-coverage -- tsc --limit 2
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/baseExpressionTypeParameters.ts --detail
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/baseExpressionTypeParameters.ts
```

Not run:

- `cargo fmt --all --check`; issue split only, no Rust code changed
- `cargo nextest run`; issue split only, no implementation changed

## Docs / current-state / issue sync

Final-state docs:

- [x] not affected

Current state:

- [x] not affected

Follow-up issues:

- [x] created: `issues/done/5154-parse-angle-bracket-type-assertion-statements.md`

## Notes

## Affected test files

- `reference/typescript/tests/cases/compiler/baseExpressionTypeParameters.ts`

## Duplicate detection

- none found by path/title/feature scan

## Smart triage

### Smart triage: parser syntax: baseExpressionTypeParameters

- Issue class: `triage-needed`
- Feature label: `parser-syntax`
- Diagnostic: `UnsupportedSyntax` / `parser-or-frontend-unsupported`
- Path: `reference/typescript/tests/cases/compiler/baseExpressionTypeParameters.ts`

Reproduction:

```sh
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/baseExpressionTypeParameters.ts
```

Source context:

```text
11 | class Gen<T> extends base<T>() {}  // Error, T not in scope
12 | class Spec extends Gen<string> {}
13 | 
14 | <string>Spec.prop;
```

Current compiler failure:

```text
error: [UnsupportedSyntax] expected LeftBrace, got Some(Semicolon) at 254..255
```

TypeScript oracle evidence:

```text
TypeScript AST path: ExpressionStatement -> TypeAssertionExpression `<string>Spec.prop` -> PropertyAccessExpression `Spec.prop`.
TypeScript diagnostic: TS2562 for the earlier `base<T>()` class heritage expression, not for this assertion syntax.
```

## Completion evidence

Fill only when moving to `done/`.

Commits:

- split to `issues/done/5154-parse-angle-bracket-type-assertion-statements.md`

Validation result:

```text
command: python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/baseExpressionTypeParameters.ts
result: pass; reproduced parser semicolon failure on top-level angle-bracket type assertion
date: 2026-05-06
```

Remaining risks:

- none

## False-done audit

Date: 2026-05-06

Classification: truly-done.

Audit result: retained in `issues/done/`. This issue has repo-local close evidence
(completion evidence with validation commands) or proper superseded classification
with child issues in `issues/open/`. The acceptance criteria documented in the issue
are satisfied by the repo-local evidence cited in the completion evidence section.

Future-work tracking: no untracked future-work item was identified in this issue
during this metadata/evidence audit.

