---
id: 1492
title: "Implement Contextualparameterandselfreferentialconstraint"
type: spike
area: frontend/syntax
class: done
priority: P2
depends_on: []
blocks: []
created: 2026-05-01
updated: 2026-05-07
---
> **Reopened by false-done audit** (2026-05-07)
> Classification: must-reopen
> Reason: Batch-migrated to issues/done/ without implementation commits.
> Evidence: Empty completion evidence. No feat/fix commit for #1492.

## Summary

Triage contextualParameterAndSelfReferentialConstraint across 1 failing reference test cases and split this bucket into implementation-ready child issues.

## Problem

Reference test results show 1 cases fail in directory `contextualParameterAndSelfReferentialConstraint` with diagnostics: unknown-unsupported. The compiler cannot handle these syntax/semantics, preventing compilation of code in this category.

Problem: contextualParameterAndSelfReferentialConstraint has 1 reference failures and needs smart-triage evidence before implementation starts.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/contextualParameterAndSelfReferentialConstraint1.ts
```

Coverage window:

```sh
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/contextualParameterAndSelfReferentialConstraint1.ts --detail
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
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/contextualParameterAndSelfReferentialConstraint1.ts --detail
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/contextualParameterAndSelfReferentialConstraint1.ts
```

Not run:

- none

## Docs / current-state / issue sync

Final-state docs:

- [x] not affected

Current state:

- [x] not affected

Follow-up issues:

- [x] superseded by `issues/done/5161-model-ambient-value-declarations-for-name-resolution.md`

## Notes

## Affected test files

- `reference/typescript/tests/cases/compiler/contextualParameterAndSelfReferentialConstraint1.ts`

## Duplicate detection

- none found by path/title/feature scan

## Smart triage

Fresh triage on 2026-05-07 shows this generated unknown-unsupported bucket is
blocked by the ambient value declaration name-resolution gap already tracked by
issue 5161.

Current diagnostic:

```text
UnresolvedName: unresolved name: `repeat` at 532..538
```

Source context:

```ts
declare const repeat: {
  <O extends NoExcessProperties<RepeatOptions<A>, O>, A>(
    options: O,
  ): (self: Effect<A>) => Effect<A>;
};

pipe(
  {} as Effect<boolean>,
  repeat({
```

Smart triage visible symbols list `repeat` at line 21 before the resolver
failure. TypeScript accepts the file with no diagnostics and infers the nested
callback parameter `x` as `boolean`.

This bucket is superseded by
`issues/done/5161-model-ambient-value-declarations-for-name-resolution.md`,
which owns declaration-only `declare var` / `declare let` / `declare const`
values being resolver-visible without runtime emission.

## Completion evidence

Fill only when moving to `done/`.

Commits:

- `...` (filled by commit that moves this issue)

Validation result:

```text
command: python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/contextualParameterAndSelfReferentialConstraint1.ts --detail --no-dashboard-data
result: pass; executed=1, build_pass=0, unsupported=1, diagnostic UnresolvedName
date: 2026-05-07

command: python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/contextualParameterAndSelfReferentialConstraint1.ts
result: pass; ambient `declare const repeat` name-resolution blocker mapped to issue 5161
date: 2026-05-07
```

Remaining risks:

- The self-referential constraint and contextual parameter inference behavior
  remains hidden until issue 5161 advances this file past ambient value
  name-resolution.
