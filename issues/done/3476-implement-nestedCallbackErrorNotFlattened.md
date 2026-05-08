---
id: 3476
title: "Implement Nestedcallbackerrornotflattened"
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

Closed after folding the current ambient assignment-target name-resolution
blocker into `issues/open/5344-resolve-ambient-var-assignment-targets.md`.

## Problem

Reference test results previously showed 1 case failing in directory
`nestedCallbackErrorNotFlattened` with diagnostics: name-resolution.

Fresh triage on 2026-05-08 shows the first current blocker is
`declare let y` being erased before name resolution, so assignment target
`y = x` reports `UnresolvedName`. Issue 5344 already owns ambient assignment
targets and now includes this `declare let` representative.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/nestedCallbackErrorNotFlattened.ts
```

Coverage window:

```sh
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/nestedCallbackErrorNotFlattened.ts --detail
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
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/nestedCallbackErrorNotFlattened.ts --detail
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/nestedCallbackErrorNotFlattened.ts
```

Not run:

- `cargo fmt --all --check` (issue-only duplicate fold; no Rust changes)
- `cargo nextest run` (issue-only duplicate fold; no Rust changes)

## Docs / current-state / issue sync

Final-state docs:

- [x] not affected

Current state:

- [x] not affected

Follow-up issues:

- [x] updated: `issues/open/5344-resolve-ambient-var-assignment-targets.md`

## Notes

## Affected test files

- `reference/typescript/tests/cases/compiler/nestedCallbackErrorNotFlattened.ts`

## Duplicate detection

- folded into `issues/open/5344-resolve-ambient-var-assignment-targets.md`
  because the current failure is an ambient `declare let` assignment target.
- related expression-position ambient value work remains in
  `issues/open/5161-model-ambient-value-declarations-for-name-resolution.md`,
  but this representative fails on the left side of `y = x`.

## Smart triage

Generated on 2026-05-08 with:

```sh
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/nestedCallbackErrorNotFlattened.ts
```

Result:

```text
Feature label: name-resolution
Diagnostic: UnresolvedName / resolver-symbol
Message: unresolved name: `y` at 398..404
Source: declare let y: Cb<Cb<Cb<Cb<string>>>>; y = x;
tokens: ok; includes declare const x, declare let y, and assignment
ast: ok; type-only declarations are erased, assignment `y = x` remains
resolved: fails in resolve_names
TypeScript oracle: TS2322 nested callback return-type incompatibility at `y = x`
```

Coverage evidence:

```sh
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/nestedCallbackErrorNotFlattened.ts --detail --no-dashboard-data
```

```text
executed=1
build_pass=0
unsupported=1
unsupported_diagcodes=UnresolvedName:1
unsupported_features=name-resolution:1
semantic_enabled=0
```

## Completion evidence

Fill only when moving to `done/`.

Commits:

- `...`

Validation result:

```text
command: env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/nestedCallbackErrorNotFlattened.ts --detail --no-dashboard-data
result: pass; executed=1, unsupported=1, current blocker folded into issue 5344
date: 2026-05-08

command: env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/nestedCallbackErrorNotFlattened.ts
result: pass; UnresolvedName for ambient `declare let y` assignment target
date: 2026-05-08

command: cargo fmt --all --check
result: not run; no Rust changes
date: 2026-05-08

command: cargo nextest run
result: not run; no Rust changes
date: 2026-05-08
```

Remaining risks:

- Nested callback assignability semantics are not proven; this bucket currently
  stops before TypeScript's TS2322 diagnostic because ambient assignment
  targets are not resolver-visible.
