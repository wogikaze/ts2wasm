---
id: 3352
title: "Implement Modulenoemit"
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

Closed as a stale generated bucket. Fresh coverage and triage for
`moduleNoEmit.ts` report `build_pass`, and the TypeScript oracle reports no
diagnostics.

## Problem

Reference test results previously showed 1 case failing in directory
`moduleNoEmit` with diagnostics: import-export. Fresh coverage no longer
reproduces a compiler build blocker:

```text
reference/typescript/tests/cases/compiler/moduleNoEmit.ts: build_pass
```

Problem: the generated bucket is stale and should not stay in the blocked queue.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/moduleNoEmit.ts
```

Coverage window:

```sh
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/moduleNoEmit.ts --detail
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
- [x] Fresh coverage proves the generated failure is no longer a current build blocker
- [x] This closed bucket preserves the exact reference path, build-pass triage output, source context, token evidence, and TypeScript oracle result
- [x] No child issue was needed because there is no current build blocker

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run
```

Impacted commands:

```sh
mise run reference-coverage -- tsc --limit 2
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/moduleNoEmit.ts --detail
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/moduleNoEmit.ts
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

- `reference/typescript/tests/cases/compiler/moduleNoEmit.ts`

## Duplicate detection

- none found by path/title/feature scan

## Smart triage

Fresh coverage on 2026-05-08:

```sh
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/moduleNoEmit.ts --detail --no-dashboard-data
```

Result:

```text
executed=1
build_pass=1
unsupported=0
reference/typescript/tests/cases/compiler/moduleNoEmit.ts: build_pass
```

Fresh triage on 2026-05-08:

```sh
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/moduleNoEmit.ts
```

Result:

```text
BuildPass: ts2wasm build succeeded
tokens include namespace Foo and expression statement `1 + 1;`
TypeScript oracle ok: true, diagnostics: []
```

## Completion evidence

Fill only when moving to `done/`.

Commits:

- `...`

Validation result:

```text
command: env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/moduleNoEmit.ts --detail --no-dashboard-data
result: pass; executed=1, build_pass=1
date: 2026-05-08

command: env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/moduleNoEmit.ts
result: pass; BuildPass and TypeScript oracle ok
date: 2026-05-08
```

Remaining risks:

- none for this generated bucket.
