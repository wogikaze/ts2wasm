---
id: 1267
title: "Implement Collisionargumentsclassconstructor"
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
> Evidence: Empty completion evidence. No feat/fix commit for #1267.

## Summary

Triage collisionArgumentsClassConstructor across 1 reference case and close it
after splitting the current constructor-overload blocker into an
implementation-ready child issue.

## Problem

Reference test results previously showed 1 case failing in directory
`collisionArgumentsClassConstructor` with diagnostics: arguments-object. Fresh
triage shows the current first blocker is `DuplicateFunction` for bodyless
constructor overload signatures, before the intended strict-mode `arguments`
diagnostics.

Problem: class constructor overload signatures are treated as duplicate
constructor definitions.

## Current failure

Representative reproduction:

```sh
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/collisionArgumentsClassConstructor.ts
```

Coverage window:

```sh
python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/collisionArgumentsClassConstructor.ts --detail --no-dashboard-data
```

## Desired final state

This generated bucket is closed after splitting the current observable blocker
into `issues/done/5334-parse-class-constructor-overload-signatures.md`.

## Scope

In scope:

- [x] Inspect the smart triage report below
- [x] Confirm whether existing open/done issues already cover this bucket
- [x] Split the constructor overload signature blocker into a child issue
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
- [x] Child issue 5334 contains an exact reference-triage command
- [x] Child issue 5334 includes failing path, diagnostic code, source context, visible symbols, and parser/TypeScript AST evidence
- [x] Child issue 5334 acceptance names the exact reference path and diagnostic change

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run
```

Impacted commands:

```sh
python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/collisionArgumentsClassConstructor.ts --detail --no-dashboard-data
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/collisionArgumentsClassConstructor.ts
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

- [x] created: `issues/done/5334-parse-class-constructor-overload-signatures.md`

## Notes

## Affected test files

- `reference/typescript/tests/cases/compiler/collisionArgumentsClassConstructor.ts`

## Duplicate detection

- `issues/done/5199-report-function-overload-list-class-merge-diagnostics.md` - related top-level function/class overload merge issue, not constructor overload ownership
- `issues/done/5323-report-missing-constructor-parameter-list.md` - related malformed bare-constructor diagnostic, not valid overload signatures
- no exact existing owner found for valid class constructor overload signatures

## Smart triage

Fresh triage shows this generated arguments-object bucket is currently blocked
by constructor overload handling.

### Smart triage: collisionArgumentsClassConstructor

- Issue class: `triage-needed`
- Feature label: `duplicate-function`
- Diagnostic: `DuplicateFunction` / `compiler-diagnostic`
- Path: `reference/typescript/tests/cases/compiler/collisionArgumentsClassConstructor.ts`

Reproduction:

```sh
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/collisionArgumentsClassConstructor.ts
```

Coverage:

```sh
python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/collisionArgumentsClassConstructor.ts --detail --no-dashboard-data
```

Coverage result:

```text
unsupported=1
unsupported_diagcodes=DuplicateFunction:1
unsupported_features=duplicate-function:1
```

Source context:

```ts
class c5 {
    constructor(i: number, ...arguments);
    constructor(i: string, ...arguments);
    constructor(i: any, ...arguments) {
        var arguments: any[];
    }
}
```

Compiler evidence:

```text
tokens: ok
ast: ok; multiple class Function members named `constructor`
resolved: fails with DuplicateFunction duplicate constructor definition
```

TypeScript oracle evidence:

```text
TS1210: Code contained in a class is evaluated in JavaScript's strict mode which does not allow this use of 'arguments'.
```

Split result:

- `issues/done/5334-parse-class-constructor-overload-signatures.md`

## Completion evidence

Commits:

- Split to `issues/done/5334-parse-class-constructor-overload-signatures.md`.

Validation result:

```text
command: python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/collisionArgumentsClassConstructor.ts
result: pass on the main checkout with TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm; DuplicateFunction constructor overload blocker split to issue 5334
date: 2026-05-07

command: python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/collisionArgumentsClassConstructor.ts --detail --no-dashboard-data
result: pass on the main checkout with TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm; DuplicateFunction/duplicate-function
date: 2026-05-07
```

Remaining risks:

- After issue 5334 lands, this reference will likely expose TS1210 strict-mode
  class `arguments` binding diagnostics.
