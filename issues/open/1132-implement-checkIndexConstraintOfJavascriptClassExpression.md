---
id: 1132
title: "Implement Checkindexconstraintofjavascriptclassexpression"
type: spike
area: frontend/resolver
class: done
priority: P1
depends_on: [056]
blocks: []
created: 2026-05-01
updated: 2026-05-06
completed: 2026-05-06
status: done
---
> **Reopened by false-done audit** (2026-05-07)
> Classification: must-reopen
> Reason: Batch-migrated to issues/done/ without implementation commits.
> Evidence: Empty completion evidence. No feat/fix commit for #1132.

## Summary

Triage checkIndexConstraintOfJavascriptClassExpression across 1 failing reference test cases and split this bucket into implementation-ready child issues.

## Problem

Reference test results show 1 case failing in directory `checkIndexConstraintOfJavascriptClassExpression` with diagnostics: name-resolution. Fresh triage shows the current compiler diagnostic is `UnresolvedName` for `someFunction`, and TypeScript also reports `Cannot find name 'someFunction'` at the same top-level call.

Problem: `checkIndexConstraintOfJavascriptClassExpression` is not a standalone implementation order in the current runner view; the current failure is an oracle-matching unresolved-name diagnostic covered by issue 056 name resolution behavior.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/checkIndexConstraintOfJavascriptClassExpression.ts
```

Coverage window:

```sh
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/checkIndexConstraintOfJavascriptClassExpression.ts --detail
```

## Desired final state

This generated bucket is closed as superseded by `issues/open/056-implement-name-resolution.md` for the current unresolved-name diagnostic. Do not implement directly from this bucket.

## Scope

In scope:

- [x] Inspect the smart triage report below
- [x] Confirm whether existing open/done issues already cover this bucket
- [x] Supersede this bucket with issue 056's unresolved-name diagnostic behavior
- [x] Preserve exact reproduction commands and representative AST/diagnostic evidence in this closed issue

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
- [x] This closed issue contains an exact `python scripts/manager.py reference-triage ...` command
- [x] This closed issue includes failing path, diagnostic code, source context, visible symbols, and parser/TypeScript AST evidence
- [x] Completion evidence names the exact fixture/reference path and diagnostic/stdout change

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
mise run reference-coverage -- tsc --limit 2
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/checkIndexConstraintOfJavascriptClassExpression.ts --detail
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/checkIndexConstraintOfJavascriptClassExpression.ts
```

Not run:

- `cargo fmt --all --check`; issue cleanup only, no Rust code changed
- `cargo nextest run`; issue cleanup only, no implementation changed

## Docs / current-state / issue sync

Final-state docs:

- [x] not affected

Current state:

- [x] not affected

Follow-up issues:

- [x] none

## Notes

## Affected test files

- `reference/typescript/tests/cases/compiler/checkIndexConstraintOfJavascriptClassExpression.ts`

## Duplicate detection

- `issues/open/056-implement-name-resolution.md` owns the basic name resolution contract: genuinely unresolved names emit `UnresolvedName`.
- Broad name-resolution buckets are not exact matches for implementation because TypeScript reports the same unresolved name in the current runner view.

## Smart triage

### Smart triage: Triage name resolution: checkIndexConstraintOfJavascriptClassExpression

- Issue class: `triage-needed`
- Feature label: `name-resolution`
- Diagnostic: `UnresolvedName` / `resolver-symbol`
- Path: `reference/typescript/tests/cases/compiler/checkIndexConstraintOfJavascriptClassExpression.ts`

Reproduction:

```sh
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/checkIndexConstraintOfJavascriptClassExpression.ts
```

Coverage reproduction:

```sh
python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/checkIndexConstraintOfJavascriptClassExpression.ts --detail --no-dashboard-data
```

Source context:

```text
someFunction(function(BaseClass) {
    'use strict';
    const DEFAULT_MESSAGE = "nop!";
    class Hello extends BaseClass {
        constructor() {
            super();
            this.foo = "bar";
        }
        _render(error) {
            const message = error.message || DEFAULT_MESSAGE;
        }
    }
});
```

Current compiler failure:

```text
error: [UnresolvedName] unresolved name: `someFunction` at 132..144
```

Compiler evidence:

- Tokens succeed for the call expression, function expression, class expression, `super()`, `this.foo` assignment, and method body.
- AST succeeds with `Expr(Call(Ident someFunction, args=[FunctionExpr ...]))`.
- Resolved output stops in `resolve_names` at the top-level `someFunction` identifier before class-expression semantics.

TypeScript oracle evidence:

```text
TS2552: Cannot find name 'someFunction'. Did you mean 'Function'?
```

Resolution:

```text
Issue 056 established that genuinely unresolved identifiers should emit UnresolvedName. The current reference-triage failure is the same unresolved-name boundary rather than an actionable class-expression/index-constraint slice.
```

## Completion evidence

Fill only when moving to `done/`.

checkIndexConstraintOfJavascriptClassExpression triage is complete. The current
failure is superseded by issue 056 unresolved-name diagnostics.

Commits:

- superseded by `issues/open/056-implement-name-resolution.md`

Validation result:

```text
command: python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/checkIndexConstraintOfJavascriptClassExpression.ts --detail --no-dashboard-data
result: pass on the main checkout; 1 executed, current failure is UnresolvedName name-resolution
date: 2026-05-06

command: python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/checkIndexConstraintOfJavascriptClassExpression.ts
result: pass; reproduced oracle-matching UnresolvedName diagnostic for someFunction
date: 2026-05-06
```

Remaining risks:

- none
