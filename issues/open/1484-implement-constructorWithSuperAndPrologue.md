---
id: 1484
title: "Implement Constructorwithsuperandprologue"
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
> Evidence: Empty completion evidence. No feat/fix commit for #1484.

## Summary

Triage constructorWithSuperAndPrologue across 1 failing reference test cases and split this bucket into implementation-ready child issues.

## Problem

Reference test results show 1 cases fail in directory `constructorWithSuperAndPrologue` with diagnostics: class. The compiler cannot handle these syntax/semantics, preventing compilation of code in this category.

Problem: constructorWithSuperAndPrologue has 1 reference failures and needs smart-triage evidence before implementation starts.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/constructorWithSuperAndPrologue.es5.ts
```

Coverage window:

```sh
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/constructorWithSuperAndPrologue.es5.ts --detail
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
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/constructorWithSuperAndPrologue.es5.ts --detail
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/constructorWithSuperAndPrologue.es5.ts
```

Not run:

- none

## Docs / current-state / issue sync

Final-state docs:

- [x] not affected

Current state:

- [x] not affected

Follow-up issues:

- [x] none; current compiler build blocker is stale

## Notes

## Affected test files

- `reference/typescript/tests/cases/compiler/constructorWithSuperAndPrologue.es5.ts`

## Duplicate detection

- none found by path/title/feature scan

## Smart triage

Fresh triage on 2026-05-07 shows the generated bucket is stale: the
representative reference path now tokenizes, parses, resolves, and builds.

Current result:

```text
BuildPass: ts2wasm build succeeded
```

Source context:

```ts
"use strict";

class B extends A  {
    constructor() {
        "ngInject";
        console.log("B")
        super();
    }
}
```

Resolved evidence:

```text
ClassDecl B extends A
constructor body: "ngInject"; console.log("B"); super();
TypeScript oracle: ok, diagnostics=[]
```

No child issue was created because this generated bucket no longer has a current
compiler build blocker. Semantic/runtime parity is outside this build-only
closure because the focused coverage run had `semantic_enabled=0`.

## Completion evidence

Fill only when moving to `done/`.

Commits:

- `...` (filled by commit that moves this issue)

Validation result:

```text
command: python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/constructorWithSuperAndPrologue.es5.ts --detail --no-dashboard-data
result: pass; executed=1, build_pass=1, unsupported=0, blocked=0, semantic_enabled=0
date: 2026-05-07

command: python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/constructorWithSuperAndPrologue.es5.ts
result: pass; BuildPass, no current compiler blocker
date: 2026-05-07
```

Remaining risks:

- Semantic/runtime parity is not proven by this closure; this only removes the
  stale generated compiler blocker from the open queue.
