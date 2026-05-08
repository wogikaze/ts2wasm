---
id: 3592
title: "Implement Nongenericclassextendinggenericclasswithany"
type: spike
area: frontend/semantics
class: blocked
priority: P1
depends_on: [5356]
blocks: []
created: 2026-05-01
updated: 2026-05-08
---

## Summary

Triage nonGenericClassExtendingGenericClassWithAny across 1 failing reference test cases and split this bucket into implementation-ready child issues.

## Problem

Fresh coverage shows this generated bucket is no longer a build blocker:
`nonGenericClassExtendingGenericClassWithAny.ts` builds successfully. The
remaining TypeScript oracle diagnostic is TS2564 for uninitialized generic
class field `Foo<T>.t`, which is owned by issue 5356.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/nonGenericClassExtendingGenericClassWithAny.ts
```

Coverage window:

```sh
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/nonGenericClassExtendingGenericClassWithAny.ts --detail
```

## Desired final state

This generated bucket is superseded by
`issues/open/5356-report-uninitialized-generic-class-fields.md`.

## Scope

In scope:

- [x] Inspect the smart triage report below
- [x] Confirm whether existing open/done issues already cover this bucket
- [x] Supersede this bucket with issue 5356
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
- [x] Superseding issue 5356 contains exact reference-triage commands and TS2564 acceptance
- [x] This issue records failing path, diagnostic code, source context, visible symbols, and parser/TypeScript AST evidence

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
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/nonGenericClassExtendingGenericClassWithAny.ts --detail --no-dashboard-data
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/nonGenericClassExtendingGenericClassWithAny.ts
```

Not run:

- cargo fmt --all --check: metadata-only issue close
- cargo nextest run: metadata-only issue close

## Docs / current-state / issue sync

Final-state docs:

- [x] not affected

Current state:

- [x] not affected

Follow-up issues:

- [x] none

## Notes

## Affected test files

- `reference/typescript/tests/cases/compiler/nonGenericClassExtendingGenericClassWithAny.ts`

## Duplicate detection

- Superseded by `issues/open/5356-report-uninitialized-generic-class-fields.md`.

## Smart triage

Generated on 2026-05-08.

Focused coverage:

```text
command: env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/nonGenericClassExtendingGenericClassWithAny.ts --detail --no-dashboard-data
result: build_pass=1; unsupported=0
```

Smart triage:

```text
command: env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/nonGenericClassExtendingGenericClassWithAny.ts
headline: BuildPass
visible symbols: class Foo, class Bar
tokens: ok through `class Foo<T>`, typed field `t: T`, and `class Bar extends Foo<any>`
ast: ok; ClassDecl Foo and ClassDecl Bar with extends Foo
resolved: ok; ClassDecl Bar extends "Foo"
typescript oracle: TS2564 Property 't' has no initializer and is not definitely assigned in the constructor.
```

## Completion evidence

Fill only when moving to `done/`.

Commits:

- `45cf4543c`

Validation result:

```text
command: env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/nonGenericClassExtendingGenericClassWithAny.ts --detail --no-dashboard-data
result: build_pass; remaining TS2564 gap is owned by issue 5356
date: 2026-05-08

command: env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/nonGenericClassExtendingGenericClassWithAny.ts
result: BuildPass with oracle TS2564 for `Foo<T>.t`; superseded by issue 5356
date: 2026-05-08
```

Remaining risks:

- After issue 5356, this reference may expose generic heritage or type-erasure
  parity gaps around `Foo<any>`.
