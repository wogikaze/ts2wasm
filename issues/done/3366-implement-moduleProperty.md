---
id: 3366
title: "Implement Moduleproperty"
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

Closed as stale. Fresh coverage and smart triage for `moduleProperty1.ts` and
`moduleProperty2.ts` now report BuildPass, so this generated bucket no longer
has an actionable compiler blocker.

## Problem

Reference test results previously showed 2 cases failing in directory
`moduleProperty` with diagnostics: import-export. Fresh coverage on
2026-05-08 reports:

```text
executed=2
build_pass=2
unsupported=0
```

Problem: this generated bucket is stale because both affected cases now build
successfully under the reference-coverage build contract.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/moduleProperty1.ts
```

Coverage window:

```sh
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/moduleProperty1.ts --detail
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
- [x] Fresh triage reports BuildPass for both affected files
- [x] This closure includes affected paths, source context, visible symbols, parser token evidence, and TypeScript oracle evidence
- [x] No child issue is needed because there is no current compiler blocker

## Validation

Required commands for this closure:

```sh
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/moduleProperty --detail --no-dashboard-data
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/moduleProperty1.ts
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/moduleProperty2.ts
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

- `reference/typescript/tests/cases/compiler/moduleProperty1.ts`
- `reference/typescript/tests/cases/compiler/moduleProperty2.ts`

## Duplicate detection

- none found by path/title/feature scan

## Smart triage

Fresh run on 2026-05-08:

```sh
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/moduleProperty --detail --no-dashboard-data
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/moduleProperty1.ts
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/moduleProperty2.ts
```

Coverage result:

```text
executed=2
build_pass=2
unsupported=0
reference/typescript/tests/cases/compiler/moduleProperty1.ts: build_pass
reference/typescript/tests/cases/compiler/moduleProperty2.ts: build_pass
```

`moduleProperty1.ts` evidence:

```text
diagnostic: BuildPass / ts2wasm build succeeded
source context: namespace M with local `var x`, local `var y`, and `export var z`; namespace M2 with `private y = x` and `export var z = y`
visible symbols: x, y, z, x, z
tokens: ok; namespace, var, private, export var, and initializer tokens are present
ast: ok; no executable AST emitted for namespace-only source
resolved: ok; no executable declarations emitted
TypeScript diagnostics: 1128 on `private`, 2304 for `y`
```

`moduleProperty2.ts` evidence:

```text
diagnostic: BuildPass / ts2wasm build succeeded
source context: namespace M with function-local `x`, local `y`, `export var z`, and namespace N reading `M.y` / `M.z`
visible symbols: f, x, y, z, test1, test2, test3, test4
tokens: ok; namespace, function, var, export var, and qualified access tokens are present
ast: ok; no executable AST emitted for namespace-only source
resolved: ok; no executable declarations emitted
TypeScript diagnostics: 2304 for function-local `x`, 2339 for non-exported `M.y`
```

## Completion evidence

Fill only when moving to `done/`.

Commits:

- local closure commit; see git log for this issue file

Validation result:

```text
command: env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/moduleProperty --detail --no-dashboard-data
result: pass; executed=2, build_pass=2
date: 2026-05-08

command: env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/moduleProperty1.ts
result: pass; BuildPass, no current compiler blocker
date: 2026-05-08

command: env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/moduleProperty2.ts
result: pass; BuildPass, no current compiler blocker
date: 2026-05-08
```

Remaining risks:

- This closure only covers ts2wasm build status. It does not claim TypeScript
  diagnostic parity for namespace visibility errors beyond the current
  reference-coverage contract.
