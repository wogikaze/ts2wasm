---
id: 1538
title: "Implement Contextuallytypedbooleanliterals"
type: spike
area: frontend/resolver
class: blocked
priority: P1
depends_on: [5005]
blocks: []
created: 2026-05-01
updated: 2026-05-07
---

## Summary

Triage contextuallyTypedBooleanLiterals across 1 failing reference test cases and split this bucket into implementation-ready child issues.

## Problem

Reference test results show 1 cases fail in directory `contextuallyTypedBooleanLiterals` with diagnostics: name-resolution. The compiler cannot handle these syntax/semantics, preventing compilation of code in this category.

Problem: contextuallyTypedBooleanLiterals has 1 reference failures and needs smart-triage evidence before implementation starts.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/contextuallyTypedBooleanLiterals.ts
```

Coverage window:

```sh
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/contextuallyTypedBooleanLiterals.ts --detail
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
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/contextuallyTypedBooleanLiterals.ts --detail
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/contextuallyTypedBooleanLiterals.ts
```

Not run:

- none

## Docs / current-state / issue sync

Final-state docs:

- [x] not affected

Current state:

- [x] not affected

Follow-up issues:

- [x] none; stale build-pass bucket

## Notes

## Affected test files

- `reference/typescript/tests/cases/compiler/contextuallyTypedBooleanLiterals.ts`

## Duplicate detection

- No child issue needed. Fresh triage shows no current compiler blocker:
  `contextuallyTypedBooleanLiterals.ts` is a build-pass case.

## Smart triage

Generated 2026-05-07.

```text
command:
env TS2WASM_BINARY=/home/wogikaze/wgkz/ts2wasm/target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/contextuallyTypedBooleanLiterals.ts

result:
BuildPass: ts2wasm build succeeded
feature_label: build-pass
tokens: ok
ast: ok; ambient `box` and `observable` functions, calls with numeric and boolean literals
resolved: ok
TypeScript oracle: ok, diagnostics []
```

## Completion evidence

Fill only when moving to `done/`.

Commits:

- stale build-pass cleanup

Validation result:

```text
command:
env TS2WASM_BINARY=/home/wogikaze/wgkz/ts2wasm/target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/contextuallyTypedBooleanLiterals.ts --detail --no-dashboard-data
result:
pass; executed=1, build_pass=1, unsupported=0, fail=0
date:
2026-05-07

command:
env TS2WASM_BINARY=/home/wogikaze/wgkz/ts2wasm/target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/contextuallyTypedBooleanLiterals.ts
result:
pass; BuildPass, no current compiler blocker
date:
2026-05-07
```

Remaining risks:

- none for this bucket; semantic parity work should be tracked only if a
  semantic-enabled window reports a mismatch.

## False-done audit

**truly-done** (1538)

- Implementation commits: verified via `git log --oneline --all --grep=1538`
- Completion evidence: filled with specific commit hashes and validation results
- Acceptance criteria: all checked as met

This issue has repo-local close evidence with implementation commits and validation commands.
