---
id: 3467
title: "Implement Narrowingtypeofundefined Name Resolution"
type: spike
area: frontend/resolver
class: blocked
priority: P1
depends_on: [5005]
blocks: []
status: done
created: 2026-05-01
updated: 2026-05-08
---

## Summary

Closed after splitting the current narrowed array-parameter spread blocker to
`issues/open/5456-support-array-spread-over-narrowed-array-typed-parameters.md`.

## Problem

Reference test results show 1 case fails in directory
`narrowingTypeofUndefined-name-resolution` with diagnostics: name-resolution.

Fresh triage shows the current blocker has advanced past name resolution. The
source parses, resolves, and reaches lower_program, where `[...arg]` reports
the issue-274 array-spread boundary even though `arg` is array-like after
`typeof arg !== "undefined"`.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/narrowingTypeofUndefined2.ts
```

Coverage window:

```sh
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/narrowingTypeofUndefined2.ts --detail
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
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/narrowingTypeofUndefined2.ts --detail
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/narrowingTypeofUndefined2.ts
```

Not run:

- `cargo fmt --all --check` (no Rust changes)
- `cargo nextest run` (no Rust changes)

## Docs / current-state / issue sync

Final-state docs:

- [x] not affected

Current state:

- [x] not affected

Follow-up issues:

- [x] `issues/open/5456-support-array-spread-over-narrowed-array-typed-parameters.md`

## Notes

## Affected test files

- `reference/typescript/tests/cases/compiler/narrowingTypeofUndefined2.ts`

## Duplicate detection

- Name-resolution buckets are no-match because fresh triage resolves names and
  reaches lower_program.
- `issues/open/274-implement-spread-operator.md` is a broad spread meta issue,
  not a narrow implementation owner.
- `issues/open/353-spread-iterator-protocol.md` is related but no-match: it
  owns general iterator protocol for custom iterables, generators, Map, and
  non-array operands.
- No existing open/done issue was found for array literal spread over a
  narrowed array-typed parameter.

## Smart triage

Generated on 2026-05-08 with:

```sh
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/narrowingTypeofUndefined2.ts
```

Result:

```text
Feature label: spread
Diagnostic: UnsupportedSyntax / parser-or-frontend-unsupported
Message: issue-274: array literal spread is only supported for literal arrays and ASCII literal-derived strings in this milestone
Failing construct: const m = [...arg];
tokens: ok
ast: ok; typeof guard, typed const, for-of, and array spread parse
resolved: ok through builtins
lower_program: issue-274 array literal spread boundary
TypeScript oracle: ok, diagnostics=[]
```

Coverage evidence:

```sh
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/narrowingTypeofUndefined2.ts --detail --no-dashboard-data
```

```text
executed=1
build_pass=0
unsupported=1
unsupported_diagcodes=UnsupportedSyntax:1
unsupported_features=unknown-unsupported:1
```

## Completion evidence

Fill only when moving to `done/`.

Commits:

- `...`

Validation result:

```text
command: env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/narrowingTypeofUndefined2.ts --detail --no-dashboard-data
result: pass; reproduced unsupported=1, UnsupportedSyntax, unknown-unsupported
date: 2026-05-08

command: env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/narrowingTypeofUndefined2.ts
result: pass; current blocker is issue-274 spread over narrowed array-typed parameter, split to issue 5456
date: 2026-05-08

command: cargo fmt --all --check
result: not run; no Rust changes
date: 2026-05-08

command: cargo nextest run
result: not run; no Rust changes
date: 2026-05-08
```

Remaining risks:

- After 5456 is implemented, this reference may expose a later generic
  narrowing, for-of, or runtime spread blocker. Split those separately if they
  appear.
