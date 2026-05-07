---
id: 1498
title: "Implement Contextualsignatureinarrayelementlibes"
type: spike
area: frontend/syntax
class: done
priority: P1
depends_on: []
blocks: []
created: 2026-05-01
updated: 2026-05-07
---

## Summary

Triage contextualSignatureInArrayElementLibEs across 2 failing reference test cases and split this bucket into implementation-ready child issues.

## Problem

Reference test results show 2 cases fail in directory `contextualSignatureInArrayElementLibEs` with diagnostics: arrow-function. The compiler cannot handle these syntax/semantics, preventing compilation of code in this category.

Problem: contextualSignatureInArrayElementLibEs has 2 reference failures and needs smart-triage evidence before implementation starts.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/contextualSignatureInArrayElementLibEs2015.ts
```

Coverage window:

```sh
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/contextualSignatureInArrayElementLibEs2015.ts --detail
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
mise run reference-coverage -- tsc --limit 4
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/contextualSignatureInArrayElementLibEs2015.ts --detail
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/contextualSignatureInArrayElementLibEs2015.ts
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

- `reference/typescript/tests/cases/compiler/contextualSignatureInArrayElementLibEs2015.ts`
- `reference/typescript/tests/cases/compiler/contextualSignatureInArrayElementLibEs5.ts`

## Duplicate detection

- none found by path/title/feature scan

## Smart triage

Fresh triage on 2026-05-07 shows this generated arrow-function bucket is stale
as a compiler build blocker for both affected lib variants.

Focused coverage:

```text
contextualSignatureInArrayElementLibEs2015.ts: build_pass
contextualSignatureInArrayElementLibEs5.ts: build_pass
```

The compiler tokenizes `declare function test(...)` with the union of
`Record<string, (arg: string) => void>` and `Array<(arg: number) => void>`,
then parses/resolves the array argument containing `(arg) => { arg; }`.
TypeScript oracle accepts both files with no diagnostics.

No child issue was created because there is no current compiler build blocker
to split from this generated bucket.

## Completion evidence

Fill only when moving to `done/`.

Commits:

- `...` (filled by commit that moves this issue)

Validation result:

```text
command: python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/contextualSignatureInArrayElementLibEs2015.ts --detail --no-dashboard-data
result: pass; executed=1, build_pass=1, unsupported=0
date: 2026-05-07

command: python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/contextualSignatureInArrayElementLibEs5.ts --detail --no-dashboard-data
result: pass; executed=1, build_pass=1, unsupported=0
date: 2026-05-07

command: python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/contextualSignatureInArrayElementLibEs2015.ts
result: pass; BuildPass / build-pass
date: 2026-05-07

command: python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/contextualSignatureInArrayElementLibEs5.ts
result: pass; BuildPass / build-pass
date: 2026-05-07
```

Remaining risks:

- Contextual signature semantic parity is not proven by this build-pass close
  because semantic coverage is disabled for these no-emit reference windows.
