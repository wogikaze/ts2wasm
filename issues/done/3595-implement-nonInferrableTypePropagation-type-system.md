---
id: 3595
title: "Implement Noninferrabletypepropagation Type System"
type: spike
area: frontend/semantics
class: blocked
priority: P1
depends_on: [5161,5488]
blocks: []
created: 2026-05-01
updated: 2026-05-08
---

## Summary

Triage nonInferrableTypePropagation-type-system across 2 failing reference test cases and split this bucket into implementation-ready child issues.

## Problem

Fresh triage shows neither representative reaches non-inferrable type
propagation semantics yet. `nonInferrableTypePropagation1.ts` stops at ambient
`declare const thing` name resolution, owned by issue 5161.
`nonInferrableTypePropagation3.ts` stops at type-only `any[]` syntax being
misparsed as issue-5150 empty element access, split to issue 5488.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/nonInferrableTypePropagation1.ts
```

Coverage window:

```sh
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/nonInferrableTypePropagation1.ts --detail
```

## Desired final state

This generated bucket is closed after mapping the two current blockers to
`issues/open/5161-model-ambient-value-declarations-for-name-resolution.md` and
`issues/open/5488-parse-array-type-suffixes-in-erased-type-positions.md`.

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
- [x] Existing issue 5161 and new issue 5488 contain exact `reference-triage` evidence
- [x] Child/owner issues include failing path, diagnostic code, source context, visible symbols, and parser/TypeScript AST evidence
- [x] Child/owner issue acceptance names the exact fixture/reference path and diagnostic/stdout change

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
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/nonInferrableTypePropagation1.ts --detail --no-dashboard-data
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/nonInferrableTypePropagation1.ts
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/nonInferrableTypePropagation3.ts --detail --no-dashboard-data
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/nonInferrableTypePropagation3.ts
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

- [x] created: `issues/open/5488-parse-array-type-suffixes-in-erased-type-positions.md`
- [x] updated: `issues/open/5161-model-ambient-value-declarations-for-name-resolution.md`

## Notes

## Affected test files

- `reference/typescript/tests/cases/compiler/nonInferrableTypePropagation1.ts`
- `reference/typescript/tests/cases/compiler/nonInferrableTypePropagation3.ts`

## Duplicate detection

- `issues/open/345-implement-tsc-type-alias-coverage.md` - Implement TypeScript type alias coverage for tsc suite (23 cases) (same feature label, title overlap)
- `issues/open/5161-model-ambient-value-declarations-for-name-resolution.md`
  owns `nonInferrableTypePropagation1.ts`
- `issues/open/5488-parse-array-type-suffixes-in-erased-type-positions.md`
  owns `nonInferrableTypePropagation3.ts`

## Smart triage

Generated on 2026-05-08.

Focused coverage:

```text
command: env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/nonInferrableTypePropagation1.ts --detail --no-dashboard-data
result: unsupported=1; unsupported_features=name-resolution:1; build_pass=0

command: env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/nonInferrableTypePropagation3.ts --detail --no-dashboard-data
result: unsupported=1; unsupported_features=type-system:1; build_pass=0
```

Smart triage:

```text
command: env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/nonInferrableTypePropagation1.ts
diagnostic: UnresolvedName
message: unresolved name: `thing` at 609..614
source: createAndUnbox(() => thing.pipe(...)
visible symbols: thing, log, result1
ast: ok; ambient functions erased and result1 call retained
resolved: fail at resolve_names
typescript oracle: ok; diagnostics=[]

command: env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/nonInferrableTypePropagation3.ts
diagnostic: UnsupportedSyntax
message: issue-5150: empty element access `expr[]` requires an index expression
source: declare type Callback<Args extends any[], Out, R> = (...args: Args) => (data: Out) => R;
tokens: ok through `Args extends any[]`
ast/resolved: fail before AST
typescript oracle: ok; diagnostics=[]
```

## Completion evidence

Fill only when moving to `done/`.

Commits:

- `8119c191a`

Validation result:

```text
command: env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/nonInferrableTypePropagation1.ts --detail --no-dashboard-data
result: unsupported name-resolution; superseded by issue 5161
date: 2026-05-08

command: env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/nonInferrableTypePropagation1.ts
result: UnresolvedName for ambient `thing`, owned by issue 5161
date: 2026-05-08

command: env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/nonInferrableTypePropagation3.ts --detail --no-dashboard-data
result: unsupported type-system; parser issue-5150 boundary split to issue 5488
date: 2026-05-08

command: env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/nonInferrableTypePropagation3.ts
result: issue-5150 for type-only `any[]`, split to issue 5488
date: 2026-05-08
```

Remaining risks:

- After issues 5161 and 5488, these fixtures may expose the intended
  non-inferrable type propagation semantic behavior.
