---
id: 1206
title: "Implement Classextensionnameoutput"
type: spike
area: frontend/syntax
class: triage-needed
priority: P1
depends_on: []
blocks: []
created: 2026-05-01
updated: 2026-05-06
---
> **Reopened by false-done audit** (2026-05-07)
> Classification: must-reopen
> Reason: Batch-migrated to issues/open/ without implementation commits.
> Evidence: Empty completion evidence. No feat/fix commit for #1206.

## Summary

Closed as stale. Fresh coverage and smart triage show
`classExtensionNameOutput.ts` is now build-pass, so there is no remaining
compiler blocker to split from this generated bucket.

## Problem

Reference test results previously showed 1 `parser-syntax` failure in
`classExtensionNameOutput`.

Problem: the representative now builds successfully; the stale bucket should
not remain open as executable work.

## Current failure

Representative reproduction:

```sh
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/classExtensionNameOutput.ts
```

Coverage window:

```sh
python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/classExtensionNameOutput.ts --detail --no-dashboard-data
```

## Desired final state

This generated bucket is either split into implementation-ready child issues or superseded by an existing open/done issue with matching evidence. Do not implement directly from this bucket.

## Scope

In scope:

- [x] Inspect the smart triage report below
- [x] Confirm there is no remaining compiler blocker to split
- [x] Close the stale generated bucket
- [x] Preserve exact reproduction commands and representative evidence

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

- [x] Duplicate candidates below are confirmed as no-match or this issue is stale
- [x] No child issue needed because fresh triage is BuildPass
- [x] This issue includes the representative path, source context, visible symbols, and parser/TypeScript AST evidence
- [x] Completion evidence names the exact fixture/reference path and build-pass result

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run
```

Impacted commands:

```sh
mise run reference-coverage -- tsc --limit 2
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/classExtensionNameOutput.ts --detail
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/classExtensionNameOutput.ts
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

- `reference/typescript/tests/cases/compiler/classExtensionNameOutput.ts`

Source context:

```ts
class A {}
if (true) {
  class B extends A {}

  const foo = function () {
    new B();
  }
}
```

## Duplicate detection

- none found by path/title/feature scan

## Smart triage

Fresh commands:

```sh
python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/classExtensionNameOutput.ts --detail --no-dashboard-data
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/classExtensionNameOutput.ts
```

Observed result on 2026-05-06:

```text
coverage: build_pass=1 unsupported=0
triage: BuildPass / ts2wasm build succeeded
tokens: ok
AST: ok; nested ClassDecl B extends A and FunctionExpr with New B()
resolved: ok; If body contains ClassDecl B extends A and New { class_name: "B" }
TypeScript oracle: ok, diagnostics=[]
```

## Completion evidence

Fill only when moving to `done/`.

Commits:

- Closed as stale build-pass; see local commit for this issue cleanup.

Validation result:

```text
command: python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/classExtensionNameOutput.ts
result: pass; BuildPass
date: 2026-05-06
```

Remaining risks:

- none
