---
id: 1330
title: "Implement Collisionthisexpressionandlocalvarwithsuperexperssion"
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
> Evidence: Empty completion evidence. No feat/fix commit for #1330.

## Summary

Closed after splitting the current blocker to
`issues/done/5341-resolve-lexical-super-property-captures-in-method-arrows.md`.

## Problem

Reference test results show 1 case failing in directory
`collisionThisExpressionAndLocalVarWithSuperExperssion`. Fresh triage shows
tokens and AST construction now succeed, but the resolver/lowering pipeline
reports `UnresolvedName: unresolved name: this` for a `super.foo()` property
call captured inside an arrow function in a derived class method.

Problem: `collisionThisExpressionAndLocalVarWithSuperExperssion.ts` is blocked by lexical `super.foo()` in class method arrows, now split to issue 5341.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/collisionThisExpressionAndLocalVarWithSuperExperssion.ts
```

Coverage window:

```sh
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/collisionThisExpressionAndLocalVarWithSuperExperssion.ts --detail
```

## Desired final state

This generated bucket is closed. Implementation should proceed through issue
5341 for the method-arrow lexical `super.foo()` resolver/lowering blocker.

## Scope

In scope:

- [x] Inspect the smart triage report below
- [x] Confirm related `super` issues do not exactly own this method-arrow case
- [x] Split one observable behavior into child issue 5341
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

- [x] Duplicate candidates below are confirmed and the exact behavior is split to 5341
- [x] Child issue 5341 contains an exact `reference-triage` command
- [x] This issue includes failing path, diagnostic code, source context, visible symbols, and parser/TypeScript AST evidence
- [x] Child issue acceptance names the exact reference path and diagnostic change

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
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/collisionThisExpressionAndLocalVarWithSuperExperssion.ts --detail
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/collisionThisExpressionAndLocalVarWithSuperExperssion.ts
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

- [x] created: `issues/done/5341-resolve-lexical-super-property-captures-in-method-arrows.md`

## Notes

## Affected test files

- `reference/typescript/tests/cases/compiler/collisionThisExpressionAndLocalVarWithSuperExperssion.ts`

## Duplicate detection

- `issues/done/5255-resolve-super-property-accesses.md` is related but covers bare `super` property receivers resolving as normal identifiers, not the unresolved synthetic `this` produced by lexical `super.foo()` in arrows.
- `issues/done/5204-resolve-lexical-super-property-captures-in-super-call-arguments.md` is related but scoped to arrow arguments passed to `super(...)`, not arrows inside ordinary derived class methods.
- No exact existing implementation-ready issue owned this method-arrow lexical `super.foo()` blocker, so this bucket was split to issue 5341.

## Smart triage

Generated 2026-05-07.

Fresh commands:

```sh
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/collisionThisExpressionAndLocalVarWithSuperExperssion.ts --detail --no-dashboard-data
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/collisionThisExpressionAndLocalVarWithSuperExperssion.ts
```

Observed result:

```text
coverage: executed=1 build_pass=0 unsupported=1
unsupported_diagcodes: UnresolvedName:1
unsupported_features: name-resolution:1

Diagnostic: UnresolvedName
Message: unresolved name: `this`
tokens: ok; includes class inheritance, method-local `_this`, arrows, and `super.foo()`
ast: ok; contains `Call(Member(Ident super, "foo"))` inside arrow bodies in derived methods
resolved/lowered: fails with unresolved synthetic `this`
TypeScript oracle: ok, no diagnostics; binding `f` has type `() => void`
Child issue: 5341
```

## Completion evidence


Commits:

- Split to `issues/done/5341-resolve-lexical-super-property-captures-in-method-arrows.md`; see local commit for this issue cleanup.

Validation result:

```text
command: env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/collisionThisExpressionAndLocalVarWithSuperExperssion.ts
result: pass; reproduced unresolved synthetic `this` for lexical `super.foo()` in method arrows and split issue 5341
date: 2026-05-07

command: env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/collisionThisExpressionAndLocalVarWithSuperExperssion.ts --detail --no-dashboard-data
result: executed=1 build_pass=0 unsupported=1 unsupported_diagcodes=UnresolvedName:1 unsupported_features=name-resolution:1
date: 2026-05-07
```

Remaining risks:

- none; remaining implementation work is tracked by issue 5341
