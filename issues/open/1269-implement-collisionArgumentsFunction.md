---
id: 1269
title: "Implement Collisionargumentsfunction"
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
> Evidence: Empty completion evidence. No feat/fix commit for #1269.

## Summary

Triage collisionArgumentsFunction across 1 reference case and close it as
superseded by the existing top-level function overload implementation issue.

## Problem

Reference test results previously showed 1 case failing in directory
`collisionArgumentsFunction` with diagnostics: arguments-object. Fresh triage
shows the current first blocker is `DuplicateFunction` for bodyless top-level
function overload signatures, before the intended strict-mode `arguments`
diagnostics.

Problem: valid TypeScript top-level function overload signatures are treated as
duplicate function definitions.

## Current failure

Representative reproduction:

```sh
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/collisionArgumentsFunction.ts
```

Coverage window:

```sh
python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/collisionArgumentsFunction.ts --detail --no-dashboard-data
```

## Desired final state

This generated bucket is closed as superseded by
`issues/open/5200-validate-top-level-function-overload-implementations.md`.

## Scope

In scope:

- [x] Inspect the smart triage report below
- [x] Confirm existing issue 5200 covers the current first blocker
- [x] Supersede this generated bucket without creating a duplicate child
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
- [x] Superseding issue 5200 owns top-level function overload signatures being treated as duplicate functions
- [x] This issue includes failing path, diagnostic code, source context, visible symbols, and parser/TypeScript AST evidence
- [x] Completion evidence names the exact reference path and diagnostic/stdout change

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run
```

Impacted commands:

```sh
python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/collisionArgumentsFunction.ts --detail --no-dashboard-data
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/collisionArgumentsFunction.ts
```

Not run:

- `cargo fmt --all --check`; issue close only, no Rust code changed
- `cargo nextest run`; issue close only, no implementation changed

## Docs / current-state / issue sync

Final-state docs:

- [x] not affected

Current state:

- [x] not affected

Follow-up issues:

- [x] superseded by `issues/open/5200-validate-top-level-function-overload-implementations.md`

## Notes

## Affected test files

- `reference/typescript/tests/cases/compiler/collisionArgumentsFunction.ts`

## Duplicate detection

- `issues/open/5200-validate-top-level-function-overload-implementations.md` - exact owner for valid top-level function overload signatures currently reported as duplicate functions
- `issues/open/5199-report-function-overload-list-class-merge-diagnostics.md` - related function overload/class merge diagnostic, not this implementation group
- broad duplicate-function generated buckets are not exact owners for this current first blocker

## Smart triage

Fresh triage shows this generated arguments-object bucket is currently blocked
by top-level function overload implementation grouping.

### Smart triage: collisionArgumentsFunction

- Issue class: `triage-needed`
- Feature label: `duplicate-function`
- Diagnostic: `DuplicateFunction` / `compiler-diagnostic`
- Current compiler message: `duplicate function definition: f4 at 848..856`
- Path: `reference/typescript/tests/cases/compiler/collisionArgumentsFunction.ts`

Reproduction:

```sh
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/collisionArgumentsFunction.ts
```

Coverage:

```sh
python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/collisionArgumentsFunction.ts --detail --no-dashboard-data
```

Coverage result:

```text
unsupported=1
unsupported_diagcodes=DuplicateFunction:1
unsupported_features=duplicate-function:1
```

Source context:

```ts
function f4(arguments: number, ...rest);
function f4(arguments: string, ...rest);
function f4(arguments: any, ...rest) {
    var arguments: any;
}
```

Compiler evidence:

```text
tokens: ok
ast: ok; multiple top-level Function declarations named `f4`
validate_ast: fails with DuplicateFunction duplicate function definition `f4`
```

TypeScript oracle evidence:

```text
TS1100: Invalid use of 'arguments' in strict mode.
```

Superseding owner:

- `issues/open/5200-validate-top-level-function-overload-implementations.md`

## Completion evidence

Commits:

- Superseded by `issues/open/5200-validate-top-level-function-overload-implementations.md`; see local commit for this issue cleanup.

Validation result:

```text
command: python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/collisionArgumentsFunction.ts
result: pass on the main checkout with TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm; DuplicateFunction top-level overload blocker superseded by issue 5200
date: 2026-05-07

command: python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/collisionArgumentsFunction.ts --detail --no-dashboard-data
result: pass on the main checkout with TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm; DuplicateFunction/duplicate-function
date: 2026-05-07
```

Remaining risks:

- After issue 5200 lands, this reference will likely expose TS1100 strict-mode
  `arguments` binding diagnostics.
