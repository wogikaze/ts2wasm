---
id: 3475
title: "Implement Nestedblockscopedbindings"
type: spike
area: frontend/resolver
class: blocked
priority: P2
depends_on: [5006]
blocks: []
status: done
created: 2026-05-01
updated: 2026-05-08
---

## Summary

Closed after splitting the current representative blocker into child issue
`issues/open/5458-allow-block-scoped-shadowing-in-nested-blocks-and-switch-cases.md`.

## Problem

Reference test results previously showed 9 cases failing in directory
`nestedBlockScopedBindings` with diagnostics: scope-analysis.

Fresh triage on 2026-05-08 shows the representative currently stops at a false
`DuplicateLocal` diagnostic for `var x; { let x; }`. The narrow block/switch
shadowing work is tracked by issue 5458.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/nestedBlockScopedBindings11.ts
```

Coverage window:

```sh
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/nestedBlockScopedBindings11.ts --detail
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
mise run reference-coverage -- tsc --limit 18
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/nestedBlockScopedBindings11.ts --detail
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/nestedBlockScopedBindings11.ts
```

Not run:

- `cargo fmt --all --check` (issue-only split; no Rust changes)
- `cargo nextest run` (issue-only split; no Rust changes)

## Docs / current-state / issue sync

Final-state docs:

- [x] not affected

Current state:

- [x] not affected

Follow-up issues:

- [x] created: `issues/open/5458-allow-block-scoped-shadowing-in-nested-blocks-and-switch-cases.md`

## Notes

## Affected test files

- `reference/typescript/tests/cases/compiler/nestedBlockScopedBindings11.ts`
- `reference/typescript/tests/cases/compiler/nestedBlockScopedBindings1.ts`
- `reference/typescript/tests/cases/compiler/nestedBlockScopedBindings10.ts`
- `reference/typescript/tests/cases/compiler/nestedBlockScopedBindings12.ts`
- `reference/typescript/tests/cases/compiler/nestedBlockScopedBindings16.ts`
- `reference/typescript/tests/cases/compiler/nestedBlockScopedBindings3.ts`
- `reference/typescript/tests/cases/compiler/nestedBlockScopedBindings15.ts`
- `reference/typescript/tests/cases/compiler/nestedBlockScopedBindings2.ts`
- `reference/typescript/tests/cases/compiler/nestedBlockScopedBindings9.ts`

## Duplicate detection

- `issues/open/3164-implement-letDeclarations-duplicate-local.md` is a broad
  generated `letDeclarations-*` duplicate-local bucket, not this
  `nestedBlockScopedBindings11.ts` representative.
- `issues/open/343-implement-duplicate-local-detection.md` owns broad
  duplicate-local diagnostics for actual duplicates, while this representative
  is a false duplicate across nested scopes.

## Smart triage

Generated on 2026-05-08 with:

```sh
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/nestedBlockScopedBindings11.ts
```

Result:

```text
Feature label: duplicate-local
Diagnostic: DuplicateLocal / compiler-diagnostic
Message: duplicate local binding: `x` at 35..41
Source: var x; { let x; () => x; }
tokens: ok
ast: ok; top-level var x, nested let x, arrow capture, switch case let y
resolved: fails during validate_ast
TypeScript oracle: ok, diagnostics=[]
```

Coverage evidence:

```sh
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/nestedBlockScopedBindings11.ts --detail --no-dashboard-data
```

```text
executed=1
build_pass=0
unsupported=1
unsupported_diagcodes=DuplicateLocal:1
unsupported_features=duplicate-local:1
semantic_enabled=0
```

## Completion evidence

Fill only when moving to `done/`.

Commits:

- `...`

Validation result:

```text
command: env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/nestedBlockScopedBindings11.ts --detail --no-dashboard-data
result: pass; executed=1, unsupported=1, current blocker captured in child issue 5458
date: 2026-05-08

command: env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/nestedBlockScopedBindings11.ts
result: pass; DuplicateLocal for nested block `let x` shadowing outer `var x`
date: 2026-05-08

command: cargo fmt --all --check
result: not run; no Rust changes
date: 2026-05-08

command: cargo nextest run
result: not run; no Rust changes
date: 2026-05-08
```

Remaining risks:

- The other `nestedBlockScopedBindings*.ts` files in this generated bucket may
  expose additional block-scope or closure blockers after issue 5458 advances.
