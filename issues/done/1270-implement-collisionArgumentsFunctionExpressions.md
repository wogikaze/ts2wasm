---
id: 1270
title: "Implement Collisionargumentsfunctionexpressions"
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
> Evidence: Empty completion evidence. No feat/fix commit for #1270.

## Summary

Triage collisionArgumentsFunctionExpressions across 1 reference case and close
it after splitting the current nested function overload blocker into an
implementation-ready child issue.

## Problem

Reference test results previously showed 1 case failing in directory
`collisionArgumentsFunctionExpressions` with diagnostics: arguments-object.
Fresh triage shows the current first blocker is `DuplicateLocal` for bodyless
nested function overload signatures, before the intended strict-mode
`arguments` diagnostics.

Problem: nested function overload signatures are treated as duplicate local
bindings.

## Current failure

Representative reproduction:

```sh
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/collisionArgumentsFunctionExpressions.ts
```

Coverage window:

```sh
python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/collisionArgumentsFunctionExpressions.ts --detail --no-dashboard-data
```

## Desired final state

This generated bucket is closed after splitting the current observable blocker
into `issues/done/5335-validate-nested-function-overload-implementations.md`.

## Scope

In scope:

- [x] Inspect the smart triage report below
- [x] Confirm whether existing open/done issues already cover this bucket
- [x] Split the nested function overload signature blocker into a child issue
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
- [x] Child issue 5335 contains an exact reference-triage command
- [x] Child issue 5335 includes failing path, diagnostic code, source context, visible symbols, and parser/TypeScript AST evidence
- [x] Child issue 5335 acceptance names the exact reference path and diagnostic change

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run
```

Impacted commands:

```sh
python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/collisionArgumentsFunctionExpressions.ts --detail --no-dashboard-data
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/collisionArgumentsFunctionExpressions.ts
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

- [x] created: `issues/done/5335-validate-nested-function-overload-implementations.md`

## Notes

## Affected test files

- `reference/typescript/tests/cases/compiler/collisionArgumentsFunctionExpressions.ts`

## Duplicate detection

- `issues/open/5200-validate-top-level-function-overload-implementations.md` - related top-level overload grouping, not nested function declarations
- broad duplicate-local generated buckets are not exact owners for this current first blocker
- no exact existing owner found for nested function overload implementation grouping

## Smart triage

Fresh triage shows this generated arguments-object bucket is currently blocked
by nested function overload implementation grouping.

### Smart triage: collisionArgumentsFunctionExpressions

- Issue class: `triage-needed`
- Feature label: `duplicate-local`
- Diagnostic: `DuplicateLocal` / `compiler-diagnostic`
- Current compiler message: `duplicate local variable: f4 at 708..716`
- Path: `reference/typescript/tests/cases/compiler/collisionArgumentsFunctionExpressions.ts`

Reproduction:

```sh
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/collisionArgumentsFunctionExpressions.ts
```

Coverage:

```sh
python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/collisionArgumentsFunctionExpressions.ts --detail --no-dashboard-data
```

Coverage result:

```text
unsupported=1
unsupported_diagcodes=DuplicateLocal:1
unsupported_features=duplicate-local:1
```

Source context:

```ts
function foo() {
    function f4(arguments: number, ...rest);
    function f4(arguments: string, ...rest);
    function f4(arguments: any, ...rest) {
        var arguments: any;
    }
}
```

Compiler evidence:

```text
tokens: ok
ast: ok; Function foo contains multiple nested Function declarations named `f4`
resolved: fails with DuplicateLocal duplicate local variable `f4`
```

TypeScript oracle evidence:

```text
TS1100: Invalid use of 'arguments' in strict mode.
```

Split result:

- `issues/done/5335-validate-nested-function-overload-implementations.md`

## Completion evidence

Commits:

- Split to `issues/done/5335-validate-nested-function-overload-implementations.md`.

Validation result:

```text
command: python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/collisionArgumentsFunctionExpressions.ts
result: pass on the main checkout with TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm; DuplicateLocal nested function overload blocker split to issue 5335
date: 2026-05-07

command: python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/collisionArgumentsFunctionExpressions.ts --detail --no-dashboard-data
result: pass on the main checkout with TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm; DuplicateLocal/duplicate-local
date: 2026-05-07
```

Remaining risks:

- After issue 5335 lands, this reference will likely expose TS1100 strict-mode
  `arguments` binding diagnostics.
