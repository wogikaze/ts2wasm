---
id: 1326
title: "Implement Collisionthisexpressionandlocalvarinfunction"
type: spike
area: frontend/syntax
class: blocked
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
> Evidence: Empty completion evidence. No feat/fix commit for #1326.

## Summary

Closed as superseded by `issues/done/5340-preserve-function-after-object-type-declaration.md`.

## Problem

Reference test results show 1 case failing in directory `collisionThisExpressionAndLocalVarInFunction` with parser-syntax diagnostics. Fresh triage confirms tokens succeed, but AST construction stops while consuming the TypeScript-only `var console: { log(val: any); }` object type declaration before the following runtime `function x()`.

Problem: `collisionThisExpressionAndLocalVarInFunction.ts` reports `UnsupportedTypeScriptSyntax: unterminated TypeScript type annotation at 164..165`, which is now tracked by issue 5340.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/collisionThisExpressionAndLocalVarInFunction.ts
```

Coverage window:

```sh
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/collisionThisExpressionAndLocalVarInFunction.ts --detail
```

## Desired final state

This generated bucket is superseded by implementation-ready issue 5340, which owns preserving a following function declaration after `var name: { ... }` object type declarations. Do not implement directly from this bucket.

## Scope

In scope:

- [x] Inspect the smart triage report below
- [x] Confirm issue 5340 covers the current blocker
- [x] Close this generated bucket as superseded
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

- [x] Duplicate candidates below are confirmed and this issue is superseded by 5340
- [x] Issue 5340 contains an exact `reference-triage` command for the object type declaration parser boundary
- [x] This issue records failing path, diagnostic code, source context, visible symbols, and parser/TypeScript AST evidence
- [x] Issue 5340 acceptance names the exact parser preservation behavior

## Validation

Required commands:

```sh
python scripts/manager.py update-issue-index
python scripts/manager.py update-issue-index --check
python scripts/manager.py check-issue-health
python scripts/manager.py check-issue-readiness -- --fail-ready-below 80
git diff --check
```

Impacted commands:

```sh
mise run reference-coverage -- tsc --limit 2
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/collisionThisExpressionAndLocalVarInFunction.ts --detail
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/collisionThisExpressionAndLocalVarInFunction.ts
```

Not run:

- `cargo fmt --all --check` (not run; issue metadata only)
- `cargo nextest run` (not run; issue metadata only)

## Docs / current-state / issue sync

Final-state docs:

- [x] not affected

Current state:

- [x] not affected

Follow-up issues:

- [x] created: `issues/done/5340-preserve-function-after-object-type-declaration.md`

## Notes

## Affected test files

- `reference/typescript/tests/cases/compiler/collisionThisExpressionAndLocalVarInFunction.ts`

## Duplicate detection

- `issues/done/5340-preserve-function-after-object-type-declaration.md` is the exact owner for `var console: { log(...); }` followed by `function x()`.
- `issues/done/5339-preserve-var-after-object-type-declaration.md` is related but covers the same object type declaration followed by another `var` declaration.
- `issues/done/5201-parse-object-type-literal-call-signatures.md` is related but narrower: it covers call-signature members like `(name: string): string`, not method signatures like `log(val: any);`.
- Arrow and `this.x` behavior after parsing remains unproven until issue 5340 advances past the object type declaration boundary.

## Smart triage

Generated 2026-05-07.

Fresh commands:

```sh
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/collisionThisExpressionAndLocalVarInFunction.ts --detail --no-dashboard-data
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/collisionThisExpressionAndLocalVarInFunction.ts
```

Observed result:

```text
coverage: executed=1 build_pass=0 unsupported=1
unsupported_diagcodes: UnsupportedSyntax:1
unsupported_features: unknown-unsupported:1

Diagnostic: UnsupportedTypeScriptSyntax
Message: unterminated TypeScript type annotation at 164..165
Source context: `var console: { log(val: any); }` followed by `function x() { ... }`
Visible symbols before failure: binding console; function x; local binding _this
tokens: ok; includes typed `var console`, function x, local `_this`, single-parameter arrow, `this.x`, and closing braces
ast: fails before representing the function body
TypeScript oracle: reports duplicate global `console` and TS2683 for `this`, proving it parses the object type declaration and following function separately
Superseded by: 5340
```

## Completion evidence


Commits:

- Superseded by `issues/done/5340-preserve-function-after-object-type-declaration.md`; see local commit for this issue cleanup.

Validation result:

```text
command: env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/collisionThisExpressionAndLocalVarInFunction.ts
result: pass; reproduced object type declaration parser failure and split issue 5340
date: 2026-05-07

command: env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/collisionThisExpressionAndLocalVarInFunction.ts --detail --no-dashboard-data
result: executed=1 build_pass=0 unsupported=1 unsupported_diagcodes=UnsupportedSyntax:1 unsupported_features=unknown-unsupported:1
date: 2026-05-07
```

Remaining risks:

- The intended arrow `this.x` behavior remains unproven until issue 5340 advances past the object type declaration parser failure.
