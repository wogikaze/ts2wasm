---
id: 1159
title: "Implement Circularmoduleimports"
type: spike
area: frontend/syntax
class: done
priority: P1
depends_on: []
blocks: []
created: 2026-05-01
updated: 2026-05-06
completed: 2026-05-06
---

## Summary

Triage circularModuleImports across 1 failing reference test cases and split this bucket into implementation-ready child issues.

## Problem

Reference test results previously showed 1 case failing in directory `circularModuleImports` with diagnostics: import-export. Fresh coverage and triage now show `ts2wasm build succeeded`; the original import/export blocker is stale.

Problem: `circularModuleImports.ts` no longer has a current compiler build blocker in this runner view. TypeScript still reports TS2303 for a circular import alias, but semantic parity is not enabled for this coverage window and is not the generated import/export blocker.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/circularModuleImports.ts
```

Coverage window:

```sh
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/circularModuleImports.ts --detail
```

## Desired final state

This generated bucket is closed as stale because the representative path now reports `build_pass`. Do not implement directly from this bucket.

## Scope

In scope:

- [x] Inspect the smart triage report below
- [x] Confirm whether existing open/done issues already cover this bucket
- [x] Close as stale build-pass instead of creating a child issue
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

- [x] Duplicate candidates below are confirmed as no-match
- [x] This closed issue contains an exact `python scripts/manager.py reference-triage ...` command
- [x] This closed issue includes path, build-pass status, source context, visible symbols, and parser/TypeScript AST evidence
- [x] Completion evidence names the exact fixture/reference path and stdout change

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
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/circularModuleImports.ts --detail
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/circularModuleImports.ts
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

- [x] none; current compiler build has no import/export blocker on this path

## Notes

## Affected test files

- `reference/typescript/tests/cases/compiler/circularModuleImports.ts`

## Duplicate detection

No exact implementation child is created because the original import/export
blocker no longer reproduces. Fresh triage only found this generated bucket as
a same-path candidate.

## Smart triage

### Smart triage: Build pass

- Issue class: `none`
- Feature label: `build-pass`
- Diagnostic: `BuildPass` / `pass`
- Path: `reference/typescript/tests/cases/compiler/circularModuleImports.ts`

Reproduction:

```sh
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/circularModuleImports.ts
```

Coverage reproduction:

```sh
python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/circularModuleImports.ts --detail --no-dashboard-data
```

Source context:

```ts
namespace M
{
    import A = B;
    import B = A;
}
```

Coverage result:

```text
executed=1
build_pass=1
unsupported=0
blocked=0
unsupported_features=
```

Compiler evidence:

```text
tokens: ok; includes namespace M, import A = B, import B = A
ast: ok; namespace/import-alias declarations are erased in this runner slice, producing []
resolved: ok; []
visible symbols before failure: []
```

TypeScript oracle evidence:

```text
TS2303: Circular definition of import alias 'A'.
```

## Completion evidence

Fill only when moving to `done/`.

Commits:

- filled by closing commit

Validation result:

```text
command: python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/circularModuleImports.ts --detail --no-dashboard-data
result: pass on the main checkout; 1 executed, build_pass=1, unsupported=0
date: 2026-05-06

command: python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/circularModuleImports.ts
result: pass; build succeeded and original import/export blocker is stale
date: 2026-05-06
```

Remaining risks:

- TypeScript still reports TS2303 for the circular import alias; future semantic-parity coverage may need a separate diagnostic issue.
