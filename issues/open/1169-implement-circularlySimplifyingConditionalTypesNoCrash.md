---
id: 1169
title: "Implement Circularlysimplifyingconditionaltypesnocrash"
type: spike
area: frontend/semantics
class: done
priority: P1
depends_on: []
blocks: []
created: 2026-05-01
updated: 2026-05-06
completed: 2026-05-06
status: done
---
> **Reopened by false-done audit** (2026-05-07)
> Classification: must-reopen
> Reason: Batch-migrated to issues/open/ without implementation commits.
> Evidence: Empty completion evidence. No feat/fix commit for #1169.

## Summary

Triage circularlySimplifyingConditionalTypesNoCrash across 1 failing reference test cases and split this bucket into implementation-ready child issues.

## Problem

Reference test results previously showed 1 case failing in directory `circularlySimplifyingConditionalTypesNoCrash` with diagnostics: type-system. Fresh coverage now builds the representative successfully.

Problem: the generated blocker is stale in the current runner view. There is no current ts2wasm build blocker for `circularlySimplifyingConditionalTypesNoCrash.ts`.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/circularlySimplifyingConditionalTypesNoCrash.ts
```

Coverage window:

```sh
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/circularlySimplifyingConditionalTypesNoCrash.ts --detail
```

## Desired final state

This generated bucket is closed as stale build-pass evidence. Do not implement directly from this bucket.

## Scope

In scope:

- [x] Inspect the smart triage report below
- [x] Confirm whether existing open/done issues already cover this bucket
- [x] Confirm the current runner has no build blocker for this bucket
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

- [x] Duplicate candidates below are confirmed as no-match or this issue is stale
- [x] This closed issue contains an exact `python scripts/manager.py reference-triage ...` command
- [x] This closed issue includes the reference path, build-pass result, source context, and TypeScript oracle evidence
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
python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/circularlySimplifyingConditionalTypesNoCrash.ts --detail --no-dashboard-data
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/circularlySimplifyingConditionalTypesNoCrash.ts
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

- `reference/typescript/tests/cases/compiler/circularlySimplifyingConditionalTypesNoCrash.ts`

## Duplicate detection

- No matching implementation-ready blocker issue is needed because the current build passes.

## Smart triage

Fresh triage shows this generated type-system bucket is stale: the current
compiler build passes.

### Smart triage: circularlySimplifyingConditionalTypesNoCrash

- Issue class: `none`
- Feature label: `build-pass`
- Diagnostic: `BuildPass` / `pass`
- Path: `reference/typescript/tests/cases/compiler/circularlySimplifyingConditionalTypesNoCrash.ts`

Reproduction:

```sh
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/circularlySimplifyingConditionalTypesNoCrash.ts
```

Coverage:

```sh
python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/circularlySimplifyingConditionalTypesNoCrash.ts --detail --no-dashboard-data
```

Coverage result:

```text
executed=1
build_pass=1
unsupported=0
```

Source context:

```ts
type Shared<
    InjectedProps,
    DecorationTargetProps extends Shared<InjectedProps, DecorationTargetProps>
    > = {
        [P in Extract<keyof InjectedProps, keyof DecorationTargetProps>]: InjectedProps[P] extends DecorationTargetProps[P] ? DecorationTargetProps[P] : never;
    };

const myStoreConnect: Connect = function(
    mapStateToProps?: any,
    mapDispatchToProps?: any,
    mergeProps?: any,
    options: unknown = {},
) {
    return connect(mapStateToProps, mapDispatchToProps, mergeProps, options);
};
```

Compiler evidence:

```text
tokens: ok
ast: ok; all declarations erase to an empty runtime AST
resolved: ok; empty resolved program
build: pass
```

TypeScript oracle evidence:

```text
typescript ok: true
diagnostics: []
```

Resolution:

```text
There is no current ts2wasm build blocker to split. This bucket is stale.
```

## Completion evidence

Fill only when moving to `done/`.

The `circularlySimplifyingConditionalTypesNoCrash` bucket is complete as stale build-pass evidence.

Commits:

- no child issue required

Validation result:

```text
command: python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/circularlySimplifyingConditionalTypesNoCrash.ts --detail --no-dashboard-data
result: pass on the main checkout; 1 executed, build_pass=1, unsupported=0
date: 2026-05-06

command: python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/circularlySimplifyingConditionalTypesNoCrash.ts
result: pass; ts2wasm build succeeded
date: 2026-05-06
```

Remaining risks:

- none
