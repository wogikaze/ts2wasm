---
id: 3591
title: "Implement Nonexportedelementsofmergedmodules"
type: spike
area: frontend/syntax
class: blocked
priority: P1
depends_on: [5486]
blocks: []
created: 2026-05-01
updated: 2026-05-08
---

## Summary

Triage nonExportedElementsOfMergedModules across 1 failing reference test cases and split this bucket into implementation-ready child issues.

## Problem

Fresh coverage shows this generated bucket is no longer a build blocker:
`nonExportedElementsOfMergedModules.ts` builds successfully. The remaining gap
is semantic parity: TypeScript reports TS2339 for non-exported merged
namespace member value access `B.x`.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/nonExportedElementsOfMergedModules.ts
```

Coverage window:

```sh
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/nonExportedElementsOfMergedModules.ts --detail
```

## Desired final state

This generated bucket is superseded by the implementation-ready child issue
`issues/open/5486-report-non-exported-merged-namespace-value-member-access.md`.

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
- [x] Child issue contains an exact `reference-triage` command
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
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/nonExportedElementsOfMergedModules.ts --detail --no-dashboard-data
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/nonExportedElementsOfMergedModules.ts
```

Not run:

- cargo fmt --all --check: metadata-only issue split/close
- cargo nextest run: metadata-only issue split/close

## Docs / current-state / issue sync

Final-state docs:

- [x] not affected

Current state:

- [x] not affected

Follow-up issues:

- [x] created: `issues/open/5486-report-non-exported-merged-namespace-value-member-access.md`

## Notes

## Affected test files

- `reference/typescript/tests/cases/compiler/nonExportedElementsOfMergedModules.ts`

## Duplicate detection

- `issues/open/5287-bind-namespace-declarations-for-qualified-value-access.md`
  is related but not a duplicate because this path already build-passes.
- `issues/open/5409-report-non-exported-namespace-member-type-annotations.md`
  covers type annotations, not value access.
- `issues/open/5313-report-non-exported-namespace-member-qualified-heritage.md`
  covers class heritage, not value access.
- `issues/open/5436-report-mixed-exported-local-namespace-vars.md` covers
  TS2395 same-name export/local var conflicts, not missing member access.

## Smart triage

Generated on 2026-05-08.

Focused coverage:

```text
command: env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/nonExportedElementsOfMergedModules.ts --detail --no-dashboard-data
result: build_pass=1; unsupported=0
```

Smart triage:

```text
command: env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/nonExportedElementsOfMergedModules.ts
headline: BuildPass
visible symbols: x, y
tokens: ok through merged namespace blocks, enum A, nested namespace B, export var x/y, and B.x/B.y
ast: ok; []
resolved: ok; []
typescript oracle: TS2339 Property 'x' does not exist on type 'typeof B'.
```

## Completion evidence

Fill only when moving to `done/`.

Commits:

- `163e93aa`

Validation result:

```text
command: env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/nonExportedElementsOfMergedModules.ts --detail --no-dashboard-data
result: build_pass; remaining gap split to issue 5486
date: 2026-05-08

command: env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/nonExportedElementsOfMergedModules.ts
result: BuildPass with oracle TS2339 for `B.x`; split to issue 5486
date: 2026-05-08
```

Remaining risks:

- After issue 5486, this reference may expose additional namespace merge or
  enum visibility parity gaps.
