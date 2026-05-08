---
id: 3597
title: "Implement Nonnullfullinference"
type: spike
area: frontend/semantics
class: blocked
priority: P1
depends_on: [5490]
blocks: []
created: 2026-05-01
updated: 2026-05-08
---

## Summary

Triage nonNullFullInference across 1 failing reference test cases and split this bucket into implementation-ready child issues.

## Problem

Fresh triage shows this generated bucket is no longer a build blocker:
`nonNullFullInference.ts` build-passes. The remaining TypeScript oracle
diagnostic is TS2345 for pushing `number` into an empty-array local inferred as
`never[]`, split to issue 5490.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/nonNullFullInference.ts
```

Coverage window:

```sh
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/nonNullFullInference.ts --detail
```

## Desired final state

This generated bucket is closed after splitting the semantic false-pass to
`issues/open/5490-report-array-push-into-never-array.md`.

## Scope

In scope:

- [x] Inspect the smart triage report below
- [x] Confirm whether existing open/done issues already cover this bucket
- [x] Split one feature family, one observable behavior, or one fixed reference window into child issues
- [x] Preserve exact reproduction commands and representative AST/diagnostic evidence

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
- [x] Child issue 5490 contains an exact `reference-triage` command
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
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/nonNullFullInference.ts --detail --no-dashboard-data
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/nonNullFullInference.ts
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

- [x] created: `issues/open/5490-report-array-push-into-never-array.md`

## Notes

## Affected test files

- `reference/typescript/tests/cases/compiler/nonNullFullInference.ts`

## Duplicate detection

- no exact owner found for TS2345 on `arr.push(n)` where `arr` is inferred as
  `never[]`

## Smart triage

Generated on 2026-05-08.

Focused coverage:

```text
command: env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/nonNullFullInference.ts --detail --no-dashboard-data
result: build_pass=1; unsupported=0
```

Smart triage:

```text
command: env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/nonNullFullInference.ts
headline: BuildPass
visible symbols: testNonNullInference, last, n, testNonNullInferenceWithArrays, result, arr
tokens/ast/resolved: ok; non-null assertions are erased to ordinary identifiers
typescript oracle: TS2345 Argument of type 'number' is not assignable to parameter of type 'never'.
oracle hint: arr has type never[]
```

## Completion evidence

Fill only when moving to `done/`.

Commits:

- `8ed994937`

Validation result:

```text
command: env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/nonNullFullInference.ts --detail --no-dashboard-data
result: build_pass; semantic false-pass split to issue 5490
date: 2026-05-08

command: env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/nonNullFullInference.ts
result: BuildPass with oracle TS2345 on `arr.push(n)` into `never[]`
date: 2026-05-08
```

Remaining risks:

- Full evolving-array inference may require broader follow-up after issue 5490.
