---
id: 1303
title: "Implement Collisionrestparameterfunction"
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
> Evidence: Empty completion evidence. No feat/fix commit for #1303.

## Summary

Triage collisionRestParameterFunction across 1 reference case and close it as
superseded by the existing top-level function overload implementation issue.

## Problem

Reference test results previously showed 1 case failing in directory
`collisionRestParameterFunction` with diagnostics: parser-syntax. Fresh triage
shows tokens and AST now succeed; the current first blocker is
`DuplicateFunction` for top-level function overload signatures with rest
parameters.

Problem: valid TypeScript top-level function overload signatures are treated
as duplicate function definitions.

## Current failure

Representative reproduction:

```sh
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/collisionRestParameterFunction.ts
```

Coverage window:

```sh
python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/collisionRestParameterFunction.ts --detail --no-dashboard-data
```

## Desired final state

This generated bucket is closed because the current observable blocker is
owned by
`issues/done/5200-validate-top-level-function-overload-implementations.md`.

## Scope

In scope:

- [x] Inspect the smart triage report below
- [x] Confirm whether existing open/done issues already cover this bucket
- [x] Supersede this bucket with the existing top-level function overload issue
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

- [x] Duplicate candidates below are confirmed and this issue is superseded
- [x] Superseding issue 5200 owns top-level function overload signatures being treated as duplicate functions
- [x] This issue preserves failing path, diagnostic code, source context, visible symbols, and parser/TypeScript AST evidence
- [x] Completion evidence names the exact reference path and diagnostic change

## Validation

Required commands:

```sh
python scripts/manager.py update-issue-index --check
python scripts/manager.py check-issue-health
python scripts/manager.py check-issue-readiness -- --fail-ready-below 80
git diff --check
```

Impacted commands:

```sh
python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/collisionRestParameterFunction.ts --detail --no-dashboard-data
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/collisionRestParameterFunction.ts
```

Not run:

- `cargo fmt --all --check`; issue metadata only, no Rust code changed
- `cargo nextest run`; issue metadata only, no implementation changed

## Docs / current-state / issue sync

Final-state docs:

- [x] not affected

Current state:

- [x] not affected

Follow-up issues:

- [x] none; superseded by `issues/done/5200-validate-top-level-function-overload-implementations.md`

## Notes

## Affected test files

- `reference/typescript/tests/cases/compiler/collisionRestParameterFunction.ts`

## Duplicate detection

- `issues/done/5200-validate-top-level-function-overload-implementations.md` - exact owner for valid top-level function overload signatures currently reported as duplicate functions
- `issues/open/5335-validate-nested-function-overload-implementations.md` - related nested function overload issue, not top-level
- `issues/done/5199-report-function-overload-list-class-merge-diagnostics.md` - related top-level function/class merge diagnostic, not this valid overload group

## Smart triage

Fresh triage shows this generated parser-syntax bucket is currently blocked by
top-level function overload implementation grouping already tracked by issue
5200.

### Smart triage: Triage duplicate function: collisionRestParameterFunction

- Issue class: `triage-needed`
- Feature label: `duplicate-function`
- Diagnostic: `DuplicateFunction` / `compiler-diagnostic`
- Path: `reference/typescript/tests/cases/compiler/collisionRestParameterFunction.ts`

Reproduction:

```sh
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/collisionRestParameterFunction.ts
```

Coverage:

```sh
python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/collisionRestParameterFunction.ts --detail --no-dashboard-data
```

Coverage result:

```text
executed=1
build_pass=0
semantic_pass=0
unsupported=1
blocked=0
unsupported_diagcodes=DuplicateFunction:1
unsupported_features=duplicate-function:1
semantic_enabled=0
```

Source context:

```ts
function f4(_i: number, ...rest);
function f4(_i: string, ...rest);
function f4(_i: any, ...rest) {
}
```

Compiler evidence:

```text
tokens: ok; includes top-level function declarations, declare functions, DotDotDot rest parameters, and overload signatures
ast: ok; multiple top-level Function declarations named f4, including bodyless overload signatures and one implementation
resolved: fails in validate_ast with DuplicateFunction duplicate function definition `f4` at 545..553
visible symbols: functions f1, f1NoError, f2, f2NoError, f3, f3NoError, and first f4 signature before failure
```

TypeScript oracle evidence:

```text
ok: true
diagnostics: []
```

Superseded by:

- `issues/done/5200-validate-top-level-function-overload-implementations.md`

## Completion evidence

Commits:

- Superseded by `issues/done/5200-validate-top-level-function-overload-implementations.md`.

Validation result:

```text
command: python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/collisionRestParameterFunction.ts
result: pass on the main checkout with TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm; DuplicateFunction top-level function overload blocker superseded by issue 5200
date: 2026-05-07

command: python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/collisionRestParameterFunction.ts --detail --no-dashboard-data
result: pass on the main checkout with TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm; DuplicateFunction/duplicate-function
date: 2026-05-07
```

Remaining risks:

- After issue 5200 lands, this reference may expose later rest-parameter
  collision semantics that need a separate child issue.
