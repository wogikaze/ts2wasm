---
id: 1266
title: "Implement Collisionargumentsarrowfunctions"
type: spike
area: frontend/syntax
class: done
priority: P1
depends_on: []
blocks: []
created: 2026-05-01
updated: 2026-05-07
completed: 2026-05-07
status: done
---
> **Reopened by false-done audit** (2026-05-07)
> Classification: must-reopen
> Reason: Batch-migrated to issues/done/ without implementation commits.
> Evidence: Empty completion evidence. No feat/fix commit for #1266.

## Summary

Triage collisionArgumentsArrowFunctions across 1 reference case and close it
after splitting the current false build-pass diagnostic into an
implementation-ready child issue.

## Problem

Reference test results previously showed 1 case failing in directory
`collisionArgumentsArrowFunctions` with diagnostics: arguments-object. Fresh
triage shows the compiler now parses and resolves the file, then returns
`BuildPass` while TypeScript reports TS1100 strict-mode `arguments` diagnostics.

Problem: strict-mode `arguments` bindings in arrow parameters and arrow bodies
currently build-pass silently instead of reporting TS1100-style diagnostics.

## Current failure

Representative reproduction:

```sh
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/collisionArgumentsArrowFunctions.ts
```

Coverage window:

```sh
python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/collisionArgumentsArrowFunctions.ts --detail --no-dashboard-data
```

## Desired final state

This generated bucket is closed after splitting the current observable behavior
into `issues/done/5333-report-strict-mode-arguments-binding-diagnostics.md`.

## Scope

In scope:

- [x] Inspect the smart triage report below
- [x] Confirm whether existing open/done issues already cover this bucket
- [x] Split the strict-mode `arguments` binding diagnostic into a child issue
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
- [x] Child issue 5333 contains an exact reference-triage command
- [x] Child issue 5333 includes failing path, diagnostic code, source context, visible symbols, and parser/TypeScript AST evidence
- [x] Child issue 5333 acceptance names the exact reference path and diagnostic change

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run
```

Impacted commands:

```sh
python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/collisionArgumentsArrowFunctions.ts --detail --no-dashboard-data
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/collisionArgumentsArrowFunctions.ts
```

Not run:

- `cargo fmt --all --check`; issue split/close only, no Rust code changed
- `cargo nextest run`; issue split/close only, no implementation changed

## Docs / current-state / issue sync

Final-state docs:

- [x] not affected

Current state:

- [x] not affected

Follow-up issues:

- [x] created: `issues/done/5333-report-strict-mode-arguments-binding-diagnostics.md`

## Notes

## Affected test files

- `reference/typescript/tests/cases/compiler/collisionArgumentsArrowFunctions.ts`

## Duplicate detection

- `issues/done/649-implement-argumentsBindsToFunctionScopeArgumentList.md` - related generated name-resolution bucket for implicit function-scope `arguments`, not the strict-mode binding diagnostic
- `issues/done/658-implement-argumentsReferenceInObjectLiteral.md` - related generated arguments-object bucket for object literal reference parsing, not this diagnostic
- no exact existing owner found for TS1100 strict-mode `arguments` binding diagnostics

## Smart triage

Fresh triage shows this generated arguments-object bucket now build-passes in
ts2wasm and needs semantic diagnostic coverage.

### Smart triage: collisionArgumentsArrowFunctions

- Issue class: `none`
- Feature label: `build-pass`
- Diagnostic: `BuildPass` / `pass`
- Path: `reference/typescript/tests/cases/compiler/collisionArgumentsArrowFunctions.ts`

Reproduction:

```sh
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/collisionArgumentsArrowFunctions.ts
```

Coverage:

```sh
python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/collisionArgumentsArrowFunctions.ts --detail --no-dashboard-data
```

Coverage result:

```text
executed=1
build_pass=1
unsupported=0
```

Source context:

```ts
var f1 = (i: number, ...arguments) => {
    var arguments: any[];
}
var f12 = (arguments: number, ...rest) => {
    var arguments = 10;
}
```

Compiler evidence:

```text
tokens: ok
ast: ok; ArrowFn params include `...arguments` and `arguments`
resolved: ok; local body bindings named `arguments` are retained
```

TypeScript oracle evidence:

```text
TS1100: Invalid use of 'arguments' in strict mode.
```

Split result:

- `issues/done/5333-report-strict-mode-arguments-binding-diagnostics.md`

## Completion evidence

Commits:

- Split to `issues/done/5333-report-strict-mode-arguments-binding-diagnostics.md`.

Validation result:

```text
command: python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/collisionArgumentsArrowFunctions.ts
result: pass on the main checkout with TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm; BuildPass with TypeScript TS1100 oracle diagnostics
date: 2026-05-07

command: python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/collisionArgumentsArrowFunctions.ts --detail --no-dashboard-data
result: pass on the main checkout with TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm; build_pass=1 unsupported=0
date: 2026-05-07
```

Remaining risks:

- Sibling collisionArguments buckets may map to the same strict-mode diagnostic
  child after their own fresh triage.
