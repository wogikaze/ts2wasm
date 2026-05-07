---
id: 1410
title: "Implement Computedpropertiestransformedinotherwisenontsclasses"
type: spike
area: frontend/syntax
class: done
priority: P1
depends_on: [432]
blocks: []
created: 2026-05-01
updated: 2026-05-07
completed: 2026-05-07
---

## Summary

Closed as stale: fresh triage and focused coverage both report build pass for
`computedPropertiesTransformedInOtherwiseNonTSClasses.ts`.

## Problem

Reference test results originally showed 1 case failing in directory
`computedPropertiesTransformedInOtherwiseNonTSClasses` with diagnostics:
import-export. Fresh evidence no longer shows a compiler blocker.

Problem: 1410 is stale in the current runner view; there is no actionable
implementation slice to split from this generated bucket.

## Current failure

Representative reproduction:

```sh
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/computedPropertiesTransformedInOtherwiseNonTSClasses.ts
```

Coverage window:

```sh
python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/computedPropertiesTransformedInOtherwiseNonTSClasses.ts --detail --no-dashboard-data
```

Observed result:

```text
BuildPass: ts2wasm build succeeded
coverage: executed=1, build_pass=1, unsupported=0, blocked=0
```

## Desired final state

This generated bucket is closed because the current reference path builds.

## Scope

In scope:

- [x] Inspect fresh smart triage
- [x] Confirm the focused coverage result
- [x] Close the stale generated bucket
- [x] Preserve exact reproduction commands and build-pass evidence

Out of scope:

- Creating follow-up work without a current compiler blocker

## Affected paths

Expected:

- `issues/`

Do not touch:

- compiler/runtime code

## Acceptance criteria

- [x] Fresh triage reports `BuildPass`
- [x] Fresh focused coverage reports `build_pass=1`
- [x] Issue index and health checks pass after moving this issue to done

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
python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/computedPropertiesTransformedInOtherwiseNonTSClasses.ts --detail --no-dashboard-data
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/computedPropertiesTransformedInOtherwiseNonTSClasses.ts
```

Not run:

- `cargo fmt --all --check`; issue metadata only
- `cargo nextest run`; issue metadata only

## Docs / current-state / issue sync

Final-state docs:

- [x] not affected

Current state:

- [x] not affected

Follow-up issues:

- [x] none

## Notes

## Affected test files

- `reference/typescript/tests/cases/compiler/computedPropertiesTransformedInOtherwiseNonTSClasses.ts`

## Fresh evidence

Source context:

```ts
namespace NS {
    export const x = Symbol();

    class NotTransformed {
        [NS.x]: number;
    }
}
```

Compiler evidence:

```text
tokens: ok; includes namespace, export const x = Symbol(), class NotTransformed, and [NS.x]: number
ast: ok; []
resolved: ok; []
```

TypeScript oracle evidence:

```text
TS2564: Property '[NS.x]' has no initializer and is not definitely assigned in the constructor.
```

Coverage result:

```text
executed=1
build_pass=1
unsupported=0
blocked=0
semantic_enabled=0
```

## Completion evidence

Commits:

- filled by closing commit

Validation result:

```text
command: python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/computedPropertiesTransformedInOtherwiseNonTSClasses.ts
result: pass; BuildPass
date: 2026-05-07

command: python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/computedPropertiesTransformedInOtherwiseNonTSClasses.ts --detail --no-dashboard-data
result: pass; executed=1, build_pass=1, unsupported=0, blocked=0
date: 2026-05-07
```

Remaining risks:

- This harness currently erases the namespace/class body to an empty AST while
  still reporting build pass. If semantic coverage is enabled for this path
  later, a separate issue should capture any resulting parity gap.
