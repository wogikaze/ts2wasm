---
id: 1083
title: "Implement Breakiniterationorswitchstatement"
type: spike
area: frontend/resolver
class: blocked
priority: P1
depends_on: [056]
blocks: []
created: 2026-05-01
updated: 2026-05-06
completed: 2026-05-06
status: done
---

## Summary

Triage breakInIterationOrSwitchStatement across 1 failing reference test cases and split this bucket into implementation-ready child issues.

## Problem

Reference test results show 1 case failing in directory `breakInIterationOrSwitchStatement` with diagnostics: name-resolution. Fresh smart triage shows the current compiler diagnostic is `UnresolvedName` for `something`, and TypeScript also reports `Cannot find name 'something'` for the same invalid `for-in` expression.

Problem: `breakInIterationOrSwitchStatement` is not a standalone implementation order; the current failure is an oracle-matching unresolved-name diagnostic covered by issue 056 name resolution behavior.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/breakInIterationOrSwitchStatement4.ts
```

Coverage window:

```sh
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/breakInIterationOrSwitchStatement4.ts --detail
```

## Desired final state

This generated bucket is closed as superseded by `issues/open/056-implement-name-resolution.md` for the current unresolved-name diagnostic. Do not implement directly from this bucket.

## Scope

In scope:

- [x] Inspect the smart triage report below
- [x] Confirm whether existing open/done issues already cover this bucket
- [x] Supersede this bucket with issue 056's genuine unresolved-name diagnostic behavior
- [x] Preserve exact reproduction commands and representative AST/diagnostic evidence in this closed issue

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
- [x] This closed issue contains an exact `python scripts/manager.py reference-triage ...` command
- [x] This closed issue includes failing path, diagnostic code, source context, visible symbols, and parser/TypeScript AST evidence
- [x] Completion evidence names the exact fixture/reference path and diagnostic/stdout change

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
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/breakInIterationOrSwitchStatement4.ts --detail
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/breakInIterationOrSwitchStatement4.ts
```

Not run:

- `cargo fmt --all --check`; issue cleanup only, no Rust code changed
- `cargo nextest run`; issue cleanup only, no implementation changed

## Docs / current-state / issue sync

Final-state docs:

- [x] not affected

Current state:

- [x] not affected

Follow-up issues:

- [x] none

## Notes

## Affected test files

- `reference/typescript/tests/cases/compiler/breakInIterationOrSwitchStatement4.ts`

## Duplicate detection

- `issues/open/056-implement-name-resolution.md` is an exact owner for a genuine unresolved identifier diagnostic.
- Break/continue parser issues are not matches: tokens and AST succeed for `for (var i in something) { break; }`.

## Smart triage

### Smart triage: name resolution: breakInIterationOrSwitchStatement4

- Issue class: `triage-needed`
- Feature label: `name-resolution`
- Diagnostic: `UnresolvedName` / `resolver-symbol`
- Path: `reference/typescript/tests/cases/compiler/breakInIterationOrSwitchStatement4.ts`

Reproduction:

```sh
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/breakInIterationOrSwitchStatement4.ts
```

Source context:

```text
1 | // @target: es2015
2 | for (var i in something) {
3 |   break;
4 | }
```

Current compiler failure:

```text
error: [UnresolvedName] unresolved name: `something` at 34..43
```

Compiler evidence:

- Tokens succeed and include `For`, `Var`, `Ident("i")`, `In`, `Ident("something")`, `Break`, and braces.
- AST succeeds as `ForIn { var: "i", iter: Ident("something"), body: [Break { label: None }] }`.
- Visible symbols before failure include the `for-in` binding `i`.
- Name resolution fails only at the unresolved `something` iterator expression.

TypeScript oracle evidence:

```text
TS2304: Cannot find name 'something'.
AST path: ForInStatement -> Identifier `something`.
```

Resolution:

```text
Issue 056 established `UnresolvedName` as the expected diagnostic for genuinely unresolved identifiers. The current reference failure is invalid source with a TypeScript oracle diagnostic for the same unresolved `something`; no new implementation child is created from this generated bucket.
```

## Completion evidence

Fill only when moving to `done/`.

Commits:

- superseded by `issues/open/056-implement-name-resolution.md`

Validation result:

```text
command: python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/breakInIterationOrSwitchStatement4.ts
result: pass; reproduced oracle-matching `UnresolvedName` diagnostic for `something`
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

