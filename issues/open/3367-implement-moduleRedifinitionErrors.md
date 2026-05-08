---
id: 3367
title: "Implement Moduleredifinitionerrors"
type: maintenance
area: frontend/syntax
class: superseded
priority: P1
depends_on: [432]
blocks: []
created: 2026-05-01
updated: 2026-05-08
completed: 2026-05-08
status: done
---

## Summary

Closed as stale. Fresh coverage and smart triage for
`moduleRedifinitionErrors.ts` now report BuildPass, so this generated bucket no
longer has an actionable compiler blocker.

## Problem

Reference test results previously showed 1 case failing in directory
`moduleRedifinitionErrors` with diagnostics: import-export. Fresh coverage on
2026-05-08 reports:

```text
executed=1
build_pass=1
unsupported=0
```

Problem: this generated bucket is stale because the affected case now builds
successfully under the reference-coverage build contract.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/moduleRedifinitionErrors.ts
```

Coverage window:

```sh
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/moduleRedifinitionErrors.ts --detail
```

## Desired final state

This generated bucket is either split into implementation-ready child issues or superseded by an existing open/done issue with matching evidence. Do not implement directly from this bucket.

## Scope

In scope:

- [x] Inspect the smart triage report below
- [x] Confirm whether existing open/done issues already cover this bucket
- [x] Close this bucket as stale BuildPass
- [x] Preserve exact reproduction commands and representative evidence in this closure

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
- [x] Fresh triage reports BuildPass
- [x] This closure includes affected path, source context, visible symbols, parser/AST/resolved evidence, and TypeScript oracle evidence
- [x] No child issue is needed because there is no current compiler blocker

## Validation

Required commands for this closure:

```sh
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/moduleRedifinitionErrors.ts --detail --no-dashboard-data
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/moduleRedifinitionErrors.ts
python scripts/manager.py update-issue-index
python scripts/manager.py update-issue-index --check
python scripts/manager.py check-issue-health
python scripts/manager.py check-issue-readiness -- --fail-ready-below 80
git diff --check
```

Impacted commands:

```sh
cargo fmt --all --check
cargo nextest run
```

Not run:

- Cargo gates; no Rust source changed.

## Docs / current-state / issue sync

Final-state docs:

- [x] not affected

Current state:

- [x] not affected

Follow-up issues:

- [x] none

## Notes

## Affected test files

- `reference/typescript/tests/cases/compiler/moduleRedifinitionErrors.ts`

## Duplicate detection

- none found by path/title/feature scan

## Smart triage

Fresh run on 2026-05-08:

```sh
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/moduleRedifinitionErrors.ts --detail --no-dashboard-data
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/moduleRedifinitionErrors.ts
```

Coverage result:

```text
executed=1
build_pass=1
unsupported=0
reference/typescript/tests/cases/compiler/moduleRedifinitionErrors.ts: build_pass
```

Current evidence:

```text
diagnostic: BuildPass / ts2wasm build succeeded
source context: class A {} followed by namespace A {}
visible symbols: class A
tokens: ok; class A and namespace A tokens are present
ast: ok; ClassDecl A is retained
resolved: ok; ClassDecl A is retained
TypeScript oracle: ok=true, diagnostics=[]
```

## Completion evidence

Fill only when moving to `done/`.

Commits:

- local closure commit; see git log for this issue file

Validation result:

```text
command: env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/moduleRedifinitionErrors.ts --detail --no-dashboard-data
result: pass; executed=1, build_pass=1
date: 2026-05-08

command: env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/moduleRedifinitionErrors.ts
result: pass; BuildPass, no current compiler blocker
date: 2026-05-08
```

Remaining risks:

- This closure only covers ts2wasm build status. It does not claim diagnostic
  parity for future namespace/class merge checks beyond the current
  reference-coverage contract.
