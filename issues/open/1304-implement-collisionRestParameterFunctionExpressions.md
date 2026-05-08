---
id: 1304
title: "Implement Collisionrestparameterfunctionexpressions"
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
> Reason: Batch-migrated to issues/open/ without implementation commits.
> Evidence: Empty completion evidence. No feat/fix commit for #1304.

## Summary

Triage collisionRestParameterFunctionExpressions across 1 reference case and
close it as superseded by the existing nested function overload implementation
issue.

## Problem

Reference test results previously showed 1 case failing in directory
`collisionRestParameterFunctionExpressions` with diagnostics: parser-syntax.
Fresh triage shows tokens and AST now succeed; the current first blocker is
`DuplicateLocal` for nested function overload signatures with rest parameters.

Problem: valid TypeScript nested function overload signatures are treated as
duplicate local bindings.

## Current failure

Representative reproduction:

```sh
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/collisionRestParameterFunctionExpressions.ts
```

Coverage window:

```sh
python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/collisionRestParameterFunctionExpressions.ts --detail --no-dashboard-data
```

## Desired final state

This generated bucket is closed because the current observable blocker is
owned by
`issues/open/5335-validate-nested-function-overload-implementations.md`.

## Scope

In scope:

- [x] Inspect the smart triage report below
- [x] Confirm whether existing open/done issues already cover this bucket
- [x] Supersede this bucket with the existing nested function overload issue
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
- [x] Superseding issue 5335 owns nested function overload signatures being treated as duplicate locals
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
python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/collisionRestParameterFunctionExpressions.ts --detail --no-dashboard-data
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/collisionRestParameterFunctionExpressions.ts
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

- [x] none; superseded by `issues/open/5335-validate-nested-function-overload-implementations.md`

## Notes

## Affected test files

- `reference/typescript/tests/cases/compiler/collisionRestParameterFunctionExpressions.ts`

## Duplicate detection

- `issues/open/5335-validate-nested-function-overload-implementations.md` - exact owner for valid nested function overload signatures currently reported as duplicate locals
- `issues/open/5200-validate-top-level-function-overload-implementations.md` - related top-level function overload issue, not nested
- `issues/open/5198-support-class-method-overload-signatures-for-element-access-calls.md` - related class method overload issue, not nested functions

## Smart triage

Fresh triage shows this generated parser-syntax bucket is currently blocked by
nested function overload implementation grouping already tracked by issue 5335.

### Smart triage: Triage duplicate local: collisionRestParameterFunctionExpressions

- Issue class: `triage-needed`
- Feature label: `duplicate-local`
- Diagnostic: `DuplicateLocal` / `compiler-diagnostic`
- Path: `reference/typescript/tests/cases/compiler/collisionRestParameterFunctionExpressions.ts`

Reproduction:

```sh
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/collisionRestParameterFunctionExpressions.ts
```

Coverage:

```sh
python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/collisionRestParameterFunctionExpressions.ts --detail --no-dashboard-data
```

Coverage result:

```text
executed=1
build_pass=0
semantic_pass=0
unsupported=1
blocked=0
unsupported_diagcodes=DuplicateLocal:1
unsupported_features=duplicate-local:1
semantic_enabled=0
```

Source context:

```ts
function foo() {
    function f4(_i: number, ...rest);
    function f4(_i: string, ...rest);
    function f4(_i: any, ...rest) {
    }
}
```

Compiler evidence:

```text
tokens: ok; includes nested function declarations, DotDotDot rest parameters, and overload signatures
ast: ok; Function foo body contains multiple nested Function declarations named f4, including bodyless overload signatures and one implementation
resolved: fails in resolve_names with DuplicateLocal duplicate local variable `f4` at 469..477
visible symbols: function foo, nested functions f1/f1NoError/f3/f3NoError, and first f4 signature before failure
```

TypeScript oracle evidence:

```text
ok: true
diagnostics: []
```

Superseded by:

- `issues/open/5335-validate-nested-function-overload-implementations.md`

## Completion evidence

Commits:

- Superseded by `issues/open/5335-validate-nested-function-overload-implementations.md`.

Validation result:

```text
command: python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/collisionRestParameterFunctionExpressions.ts
result: pass on the main checkout with TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm; DuplicateLocal nested function overload blocker superseded by issue 5335
date: 2026-05-07

command: python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/collisionRestParameterFunctionExpressions.ts --detail --no-dashboard-data
result: pass on the main checkout with TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm; DuplicateLocal/duplicate-local
date: 2026-05-07
```

Remaining risks:

- After issue 5335 lands, this reference may expose later rest-parameter
  collision semantics that need a separate child issue.
