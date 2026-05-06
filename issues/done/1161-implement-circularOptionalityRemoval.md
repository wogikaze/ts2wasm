---
id: 1161
title: "Implement Circularoptionalityremoval"
type: spike
area: frontend/resolver
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

Triage circularOptionalityRemoval across 1 failing reference test cases and split this bucket into implementation-ready child issues.

## Problem

Reference test results show 1 case failing in directory `circularOptionalityRemoval` with diagnostics: name-resolution. Fresh triage shows the current compiler diagnostic is `UnresolvedName` for `someCondition`, and TypeScript also reports TS2304 for the same identifier.

Problem: `circularOptionalityRemoval` is not a standalone implementation order; the current build failure is an oracle-matching unresolved-name diagnostic covered by issue 056 name resolution behavior.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/circularOptionalityRemoval.ts
```

Coverage window:

```sh
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/circularOptionalityRemoval.ts --detail
```

## Desired final state

This generated bucket is closed as superseded by `issues/done/056-implement-name-resolution.md` for the current unresolved-name diagnostic. Do not implement directly from this bucket.

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
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/circularOptionalityRemoval.ts --detail
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/circularOptionalityRemoval.ts
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

- [x] none for the current build blocker

## Notes

## Affected test files

- `reference/typescript/tests/cases/compiler/circularOptionalityRemoval.ts`

## Duplicate detection

Fresh duplicate scan found broad name-resolution candidates, but the current
failure is already covered by completed issue 056's genuine unresolved-name
diagnostic behavior. No implementation-ready child is created.

## Smart triage

### Smart triage: name resolution

- Issue class: `triage-needed`
- Feature label: `name-resolution`
- Diagnostic: `UnresolvedName` / `resolver-symbol`
- Path: `reference/typescript/tests/cases/compiler/circularOptionalityRemoval.ts`

Reproduction:

```sh
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/circularOptionalityRemoval.ts
```

Coverage reproduction:

```sh
python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/circularOptionalityRemoval.ts --detail --no-dashboard-data
```

Source context:

```ts
function fn1(x: number | undefined = x > 0 ? x : 0) { }

function fn2(x?: string = someCondition ? 'value1' : x) { }
```

Coverage result:

```text
unsupported=1
unsupported_diagcodes=UnresolvedName:1
unsupported_features=name-resolution:1
```

Compiler evidence:

```text
tokens: ok
ast: ok; Function fn1 and Function fn2 with parameter default ternaries
resolved: UnresolvedName for someCondition at 178..191
visible symbols before failure: function fn1
```

TypeScript oracle evidence:

```text
TS2304: Cannot find name 'someCondition'.
TS2502/TS2372/TS18048 diagnostics also report parameter self-reference and optionality/default issues for x.
```

Resolution:

```text
Issue 056 established `UnresolvedName` as the expected diagnostic for genuinely unresolved identifiers. The current build failure is invalid source with a TypeScript oracle diagnostic for the same unresolved `someCondition`; no new implementation child is created from this generated bucket.
```

## Completion evidence

Fill only when moving to `done/`.

Commits:

- superseded by `issues/done/056-implement-name-resolution.md`

Validation result:

```text
command: python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/circularOptionalityRemoval.ts --detail --no-dashboard-data
result: pass; reproduced oracle-matching UnresolvedName/name-resolution diagnostic for someCondition
date: 2026-05-06

command: python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/circularOptionalityRemoval.ts
result: pass; reproduced oracle-matching `UnresolvedName` diagnostic for someCondition
date: 2026-05-06
```

Remaining risks:

- Future semantic-parity coverage may need a separate issue for TS2502/TS2372/TS18048 parameter self-reference and optionality/default diagnostics.
