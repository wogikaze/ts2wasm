---
id: 3593
title: "Implement Nonidenticaltypeconstraints"
type: spike
area: frontend/syntax
class: blocked
priority: P1
depends_on: [5356,5487]
blocks: []
created: 2026-05-01
updated: 2026-05-08
---

## Summary

Triage nonIdenticalTypeConstraints across 1 failing reference test cases and split this bucket into implementation-ready child issues.

## Problem

Fresh coverage shows this generated bucket is no longer a build blocker:
`nonIdenticalTypeConstraints.ts` builds successfully. The remaining TypeScript
oracle diagnostics are TS2428 for non-identical merged declaration type
parameters, now split to issue 5487, plus TS2564 strict property initialization
diagnostics owned by issue 5356.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/nonIdenticalTypeConstraints.ts
```

Coverage window:

```sh
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/nonIdenticalTypeConstraints.ts --detail
```

## Desired final state

This generated bucket is closed after splitting the TS2428 semantic parity
work into `issues/open/5487-report-nonidentical-merged-type-parameters.md`
and recording the TS2564 overlap with issue 5356.

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
- [x] Child issue 5487 contains an exact `reference-triage` command
- [x] Child issue includes failing path, diagnostic code, source context, visible symbols, and parser/TypeScript AST evidence
- [x] Child issue acceptance names the exact fixture/reference path and diagnostic/stdout change

## Validation

Required commands:

```sh
git diff --check
python scripts/manager.py update-issue-index --check
python scripts/manager.py check-issue-health
python scripts/manager.py check-issue-readiness -- --fail-ready-below 80
```

Impacted commands:

```sh
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/nonIdenticalTypeConstraints.ts --detail --no-dashboard-data
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/nonIdenticalTypeConstraints.ts
```

Not run:

- cargo fmt --all --check: metadata-only issue split
- cargo nextest run: metadata-only issue split

## Docs / current-state / issue sync

Final-state docs:

- [x] not affected

Current state:

- [x] not affected

Follow-up issues:

- [x] created: `issues/open/5487-report-nonidentical-merged-type-parameters.md`

## Notes

## Affected test files

- `reference/typescript/tests/cases/compiler/nonIdenticalTypeConstraints.ts`

## Duplicate detection

- no exact TS2428 / identical type parameter owner found by issue search
- TS2564 overlap is owned by
  `issues/open/5356-report-uninitialized-generic-class-fields.md`

## Smart triage

Generated on 2026-05-08.

Focused coverage:

```text
command: env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/nonIdenticalTypeConstraints.ts --detail --no-dashboard-data
result: build_pass=1; unsupported=0
```

Smart triage:

```text
command: env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/nonIdenticalTypeConstraints.ts
headline: BuildPass
visible symbols: class Different, class Foo, class Qux, class Bar, class Baz, class Quux
tokens: ok through class/interface merged declarations and generic constraints
ast: ok; class declarations Different, Foo, Qux, Bar, Baz, Quux are parsed
resolved: ok; class declarations are resolved
typescript oracle: TS2428 for Foo, Qux, Quux; TS2564 for uninitialized fields
```

## Completion evidence

Fill only when moving to `done/`.

Commits:

- `pending`

Validation result:

```text
command: env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/nonIdenticalTypeConstraints.ts --detail --no-dashboard-data
result: build_pass; TS2428 split to issue 5487 and TS2564 owned by issue 5356
date: 2026-05-08

command: env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/nonIdenticalTypeConstraints.ts
result: BuildPass with oracle TS2428 for non-identical merged declaration type parameters
date: 2026-05-08
```

Remaining risks:

- Issue 5487 must distinguish invalid `Foo`/`Qux`/`Quux` merged declarations
  from valid matching `Bar`/`Baz` declarations.
