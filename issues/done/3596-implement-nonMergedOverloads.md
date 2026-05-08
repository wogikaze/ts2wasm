---
id: 3596
title: "Implement Nonmergedoverloads"
type: spike
area: frontend/syntax
class: blocked
priority: P1
depends_on: [5489]
blocks: []
created: 2026-05-01
updated: 2026-05-08
---

## Summary

Triage nonMergedOverloads across 1 failing reference test cases and split this bucket into implementation-ready child issues.

## Problem

Fresh triage shows the current blocker is not generic import/export syntax.
The parser accepts the local `var f` and exported overload pair, then reports a
generic duplicate-local/duplicate-function diagnostic. TypeScript reports
TS2395 because merged declaration `f` mixes local and exported declarations.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/nonMergedOverloads.ts
```

Coverage window:

```sh
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/nonMergedOverloads.ts --detail
```

## Desired final state

This generated bucket is closed after splitting the TS2395 mixed
exported/local var/function merge diagnostic to
`issues/open/5489-report-mixed-exported-local-var-function-merges.md`.

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
- [x] Child issue 5489 contains an exact `reference-triage` command
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
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/nonMergedOverloads.ts --detail --no-dashboard-data
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/nonMergedOverloads.ts
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

- [x] created: `issues/open/5489-report-mixed-exported-local-var-function-merges.md`

## Notes

## Affected test files

- `reference/typescript/tests/cases/compiler/nonMergedOverloads.ts`

## Duplicate detection

- no exact TS2395 owner found for entry-module local `var` plus exported
  function overload declarations
- related but not exact: issue 5436 (namespace vars), issue 5442 (default
  function/namespace), issue 5200 (valid function overload implementation
  grouping)

## Smart triage

Generated on 2026-05-08.

Focused coverage:

```text
command: env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/nonMergedOverloads.ts --detail --no-dashboard-data
result: unsupported=1; unsupported_diagcodes=DuplicateLocal:1; unsupported_features=duplicate-local:1
```

Smart triage:

```text
command: env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/nonMergedOverloads.ts
diagnostic: DuplicateLocal
message: duplicate local variable: `f` at 82..90
source: var f = 10; export function f(); export function f() {}
tokens: ok through local var and exported function overload declarations
ast: ok; Let f plus two ExportDecl Function f nodes
resolved: DuplicateFunction duplicate function definition `f`
typescript oracle: TS2395 Individual declarations in merged declaration 'f' must be all exported or all local.
```

## Completion evidence

Fill only when moving to `done/`.

Commits:

- `pending`

Validation result:

```text
command: env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/nonMergedOverloads.ts --detail --no-dashboard-data
result: unsupported duplicate-local; split to issue 5489
date: 2026-05-08

command: env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/nonMergedOverloads.ts
result: DuplicateLocal/DuplicateFunction before TypeScript TS2395 parity; split to issue 5489
date: 2026-05-08
```

Remaining risks:

- After TS2395 parity is implemented, this fixture may still expose normal
  top-level overload grouping behavior from issue 5200.
