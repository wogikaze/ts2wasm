---
id: 1121
title: "Implement Casttest"
type: spike
area: frontend/syntax
class: superseded
priority: P2
depends_on: [5218]
blocks: []
created: 2026-05-01
updated: 2026-05-06
completed: 2026-05-06
status: done
---

## Summary

Triage castTest across 1 failing reference test case and split this bucket into an implementation-ready child issue.

## Problem

Reference test results showed 1 case failing in directory `castTest` with diagnostics: unknown-unsupported. Fresh triage shows the casts and object literal parse successfully, then lowering stops at the `issue-062e` nested-function `this` closure runtime boundary.

Problem: castTest has 1 reference failure whose current actionable blocker is now tracked by child issue 5218.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/castTest.ts
```

Coverage window:

```sh
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/castTest.ts --detail
```

## Desired final state

This generated bucket is superseded by `issues/done/5218-parse-typescript-this-parameters-in-function-expressions.md`.

## Scope

In scope:

- [x] Inspect the smart triage report below
- [x] Confirm whether existing open/done issues already cover this bucket
- [x] Split the current observable blocker into child issue 5218
- [x] Preserve exact reproduction commands and representative AST/diagnostic evidence in the child issue

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
- [x] Child issue 5218 contains an exact `mise run reference-triage -- ...` command
- [x] Child issue includes failing path, diagnostic code, source context, visible symbols, and parser/TypeScript AST evidence
- [x] Child issue acceptance names the exact reference path and diagnostic/stdout change

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
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/castTest.ts --detail
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/castTest.ts
```

Not run:

- `cargo fmt --all --check`; issue split only, no Rust code changed
- `cargo nextest run`; issue split only, no implementation changed

## Docs / current-state / issue sync

Final-state docs:

- [x] not affected

Current state:

- [x] not affected

Follow-up issues:

- [x] created: `issues/done/5218-parse-typescript-this-parameters-in-function-expressions.md`

## Notes

## Affected test files

- `reference/typescript/tests/cases/compiler/castTest.ts`

## Duplicate detection

- none found by path/title/feature scan

## Smart triage

Generated on 2026-05-06:

- command: `python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/castTest.ts`
- issue class: `triage-needed`
- feature label: `runtime-subset`
- diagnostic: `UnsupportedRuntimeSubset` / `unsupported-feature-boundary`
- message: `issue-062e: nested function closures with this or arguments are not supported in this slice`
- child issue: `issues/done/5218-parse-typescript-this-parameters-in-function-expressions.md`

Source context:

```text
24 | var p_cast = <Point> ({
25 |     x: 0,
26 |     y: 0,
27 |     add: function(dx, dy) {
28 |         return new Point(this.x + dx, this.y + dy);
29 |     },
30 |     mult: function(p) { return p; }
31 | })
```

Visible symbols before failure include bindings `x`, `z`, `y`, `a`, `b`, `s`,
`ar`, `f`, declared class `Point`, and binding `p_cast`.

Compiler evidence:

```text
tokens: ok
AST: object literal property `add` is a FunctionExpr returning `new Point(this.x + dx, this.y + dy)`
resolved/lowered: issue-062e nested function closure with `this` or `arguments`
TypeScript oracle: reports TS2352 for earlier null casts and provides binding/type evidence for `p_cast: Point`
```

## Completion evidence

Closed as a generated triage bucket. The actionable nested-function `this`
closure blocker is tracked by child issue 5218.

Commits:

- this split commit

Validation result:

```text
command: python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/castTest.ts
result: fail with issue-062e nested-function `this` closure runtime boundary; split to issue 5218
date: 2026-05-06
```

Remaining risks:

- After issue 5218 is implemented, earlier TypeScript cast-overlap diagnostics or later class constructor value behavior may need separate tracking.
