---
id: 1278
title: "Implement Collisioncodegenmodulewithmemberinterfaceconflict"
type: spike
area: frontend/syntax
class: blocked
priority: P1
depends_on: [432]
blocks: []
created: 2026-05-01
updated: 2026-05-07
---
> **Reopened by false-done audit** (2026-05-07)
> Classification: must-reopen
> Reason: Batch-migrated to issues/open/ without implementation commits.
> Evidence: Empty completion evidence. No feat/fix commit for #1278.

## Summary

Triage collisionCodeGenModuleWithMemberInterfaceConflict across 1 failing reference test cases and split this bucket into implementation-ready child issues.

## Problem

Reference test results previously showed 1 case failing in directory `collisionCodeGenModuleWithMemberInterfaceConflict` with diagnostics: import-export. Fresh triage now build-passes, so this generated bucket is stale.

Problem: collisionCodeGenModuleWithMemberInterfaceConflict was listed as an import-export blocker, but current smart triage shows no compiler blocker.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/collisionCodeGenModuleWithMemberInterfaceConflict.ts
```

Coverage window:

```sh
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/collisionCodeGenModuleWithMemberInterfaceConflict.ts --detail
```

## Desired final state

This generated bucket is closed because the representative reference now build-passes and coverage no longer reports import-export unsupported syntax.

## Scope

In scope:

- [x] Inspect the smart triage report below
- [x] Confirm whether existing open/done issues already cover this bucket
- [x] Confirm no child issue is needed because the representative now build-passes
- [x] Preserve exact reproduction commands and representative AST/diagnostic evidence in this closed bucket

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
- [x] No child issue needed because current triage is `BuildPass`
- [x] Smart triage evidence below includes failing path, current diagnostic, source context, visible symbols, and parser/TypeScript evidence
- [x] Coverage evidence names the exact reference path and current `build_pass` result

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run
```

Impacted commands:

```sh
mise run reference-coverage -- tsc --limit 2
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/collisionCodeGenModuleWithMemberInterfaceConflict.ts --detail
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/collisionCodeGenModuleWithMemberInterfaceConflict.ts
```

Not run:

- none

## Docs / current-state / issue sync

Final-state docs:

- [x] not affected

Current state:

- [x] not affected

Follow-up issues:

- [x] none

## Notes

## Affected test files

- `reference/typescript/tests/cases/compiler/collisionCodeGenModuleWithMemberInterfaceConflict.ts`

## Duplicate detection

- no matching open/done implementation issue is needed because the current reference build-passes.

## Smart triage

Reproduction:
`python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/collisionCodeGenModuleWithMemberInterfaceConflict.ts`.

Focused coverage:
`python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/collisionCodeGenModuleWithMemberInterfaceConflict.ts --detail --no-dashboard-data`.

Current diagnostic:

```text
BuildPass: ts2wasm build succeeded
```

Focused coverage result:

```text
executed=1
build_pass=1
semantic_pass=0
unsupported=0
unsupported_diagcodes=
unsupported_features=
```

Representative source:

```ts
namespace m1 {
    export interface m1 {
    }
    export class m2 implements m1 {
    }
}
var foo = new m1.m2();
```

Compiler evidence:

```text
tokens: ok
ast: ok; retained AST contains `new m1.m2()`
resolved: ok; constructor name currently resolves to `m2`
visible symbols: m2, foo
```

TypeScript oracle evidence:

```text
typescript ok; diagnostics: []
```

## Completion evidence

Fill only when moving to `done/`.

Commits:

- closed as stale; current triage build-passes

Validation result:

```text
command: python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/collisionCodeGenModuleWithMemberInterfaceConflict.ts
result: pass on the main checkout with TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm; BuildPass
date: 2026-05-07
```

Remaining risks:

- Semantic runtime parity is not enabled for this reference window, but there is
  no current import-export blocker to split from this generated bucket.
