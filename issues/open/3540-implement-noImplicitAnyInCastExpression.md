---
id: 3540
title: "Implement Noimplicitanyincastexpression"
type: spike
area: frontend/syntax
class: superseded
priority: P2
depends_on: []
blocks: []
created: 2026-05-01
updated: 2026-05-08
---

## Summary

Closed as a stale generated bucket. Fresh focused coverage and triage show
`noImplicitAnyInCastExpression.ts` now build-passes.

## Problem

Fresh triage shows the parser erases parenthesized angle-bracket cast
expressions and resolves the resulting object-literal expression statements:

```ts
(<IFoo>{ a: null });
(<IFoo>{ a: 2, b: undefined });
(<IFoo>{ c: null });
```

Problem: the generated unknown-unsupported bucket is stale and no longer has a
compiler blocker to split.

## Current failure

Representative reproduction:

```sh
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/noImplicitAnyInCastExpression.ts
```

Coverage window:

```sh
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/noImplicitAnyInCastExpression.ts --detail --no-dashboard-data
```

Observed 2026-05-08:

```text
coverage: executed=1 build_pass=1 unsupported=0 blocked=0
triage: BuildPass / pass
```

## Desired final state

This generated bucket is closed as superseded by current build-pass behavior.
No child issue is needed.

## Scope

In scope:

- [x] Inspect the smart triage report below
- [x] Confirm whether existing open/done issues already cover this bucket
- [x] Confirm the representative now build-passes
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
- [x] The done issue contains an exact `python scripts/manager.py reference-triage ...` command
- [x] The done issue includes path, BuildPass diagnostic, source context, and parser/TypeScript AST evidence
- [x] No child issue is created because the representative has no current compiler blocker

## Validation

Required commands:

```sh
git diff --check
python scripts/manager.py update-issue-index --check
python scripts/manager.py check-issue-health
python scripts/manager.py check-issue-readiness -- --fail-ready-below 80
```

Impacted commands:

```sh
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/noImplicitAnyInCastExpression.ts --detail --no-dashboard-data
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/noImplicitAnyInCastExpression.ts
```

Not run:

- `cargo fmt --all --check`; metadata-only stale bucket closure.
- `cargo nextest run`; metadata-only stale bucket closure.

## Docs / current-state / issue sync

Final-state docs:

- [x] not affected

Current state:

- [x] not affected

Follow-up issues:

- [x] none

## Notes

## Affected test files

- `reference/typescript/tests/cases/compiler/noImplicitAnyInCastExpression.ts`

## Duplicate detection

- `issues/open/5154-parse-angle-bracket-type-assertion-statements.md` is
  related but owns top-level `<T>expr;` assertion statements. This
  representative's parenthesized cast expressions now parse and build-pass.
- `issues/done/5125-implement-as-type-assertion-expression.md` covers `as`
  assertions, not this angle-bracket form.
- No child issue was created because this representative now build-passes.

## Smart triage

### Smart triage: Build pass: noImplicitAnyInCastExpression

- Issue class: `none`
- Feature label: `build-pass`
- Diagnostic: `BuildPass` / `pass`
- Path: `reference/typescript/tests/cases/compiler/noImplicitAnyInCastExpression.ts`

Compiler evidence:

```text
tokens: ok through interface IFoo and all three parenthesized angle-bracket casts
ast: ok; casts erase to object-literal expression statements
resolved: ok; object expressions remain after type erasure
```

TypeScript oracle:

```text
TS2352 diagnostics are reported for the intentionally invalid cast targets.
No unsupported compiler blocker remains in ts2wasm for this generated bucket.
```

## Completion evidence

Fill only when moving to `done/`.

Commits:

- `...`

Validation result:

```text
command:
result:
date:
```

Remaining risks:

- none
