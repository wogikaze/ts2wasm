---
id: 1481
title: "Implement Constructorwithcapturedsuper"
type: spike
area: frontend/syntax
class: done
priority: P1
depends_on: [5001]
blocks: []
created: 2026-05-01
updated: 2026-05-07
---
> **Reopened by false-done audit** (2026-05-07)
> Classification: must-reopen
> Reason: Batch-migrated to issues/done/ without implementation commits.
> Evidence: Empty completion evidence. No feat/fix commit for #1481.

## Summary

Triage constructorWithCapturedSuper across 1 failing reference test cases and split this bucket into implementation-ready child issues.

## Problem

Reference test results show 1 cases fail in directory `constructorWithCapturedSuper` with diagnostics: class. The compiler cannot handle these syntax/semantics, preventing compilation of code in this category.

Problem: constructorWithCapturedSuper has 1 reference failures and needs smart-triage evidence before implementation starts.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/constructorWithCapturedSuper.ts
```

Coverage window:

```sh
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/constructorWithCapturedSuper.ts --detail
```

## Desired final state

This generated bucket is either split into implementation-ready child issues or superseded by an existing open/done issue with matching evidence. Do not implement directly from this bucket.

## Scope

In scope:

- [x] Inspect the smart triage report below
- [x] Confirm whether existing open/done issues already cover this bucket
- [x] Split one feature family, one observable behavior, or one fixed reference window into child issues
- [x] Preserve exact reproduction commands and representative AST/diagnostic evidence in each child issue

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
- [x] At least one child issue contains an exact `mise run reference-triage -- ...` command
- [x] Child issue includes failing path, diagnostic code, source context, visible symbols, and parser/TypeScript AST evidence
- [x] Child issue acceptance names the exact fixture/reference path and diagnostic/stdout change

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run
```

Impacted commands:

```sh
mise run reference-coverage -- tsc --limit 2
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/constructorWithCapturedSuper.ts --detail
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/constructorWithCapturedSuper.ts
```

Not run:

- none

## Docs / current-state / issue sync

Final-state docs:

- [x] not affected

Current state:

- [x] not affected

Follow-up issues:

- [x] split to `issues/done/5363-support-class-constructor-outer-local-return-captures.md`

## Notes

## Affected test files

- `reference/typescript/tests/cases/compiler/constructorWithCapturedSuper.ts`

## Duplicate detection

- none found by path/title/feature scan

## Smart triage

Fresh triage on 2026-05-07 shows the generated bucket is no longer an
implementation-ready unit. Tokens and AST succeed, but name resolution stops at
the first class constructor lexical-capture blocker before the later derived
constructor/super behavior can be evaluated.

Current diagnostic:

```text
error: [UnsupportedSyntax] issue-289: class constructor `constructor` references outer local `oneA`; class constructor lexical captures require environment support at 83..87
```

Source context:

```ts
let oneA: A;

class A {
    constructor() {
        return oneA;
    }
}
```

TypeScript oracle: accepted with no diagnostics.

This bucket was split to `issues/done/5363-support-class-constructor-outer-local-return-captures.md`.

## Completion evidence

Fill only when moving to `done/`.

Commits:

- `...` (filled by commit that moves this issue)

Validation result:

```text
command: python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/constructorWithCapturedSuper.ts --detail --no-dashboard-data
result: pass; executed=1, build_pass=0, unsupported=1, diagnostic UnsupportedSyntax issue-289
date: 2026-05-07

command: python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/constructorWithCapturedSuper.ts
result: pass; reproduced issue-289 constructor outer-local return capture and split child issue 5363
date: 2026-05-07
```

Remaining risks:

- Later behavior in `constructorWithCapturedSuper.ts`, including derived
  constructors, `super()`, early returns, loops, try/finally, and nested
  callbacks, remains hidden until issue 5363 advances past the base constructor
  capture.
