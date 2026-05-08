---
id: 1194
title: "Implement Classextendingany"
type: spike
area: frontend/syntax
class: triage-needed
priority: P1
depends_on: [5255]
blocks: []
created: 2026-05-01
updated: 2026-05-06
---
> **Reopened by false-done audit** (2026-05-07)
> Classification: must-reopen
> Reason: Batch-migrated to issues/done/ without implementation commits.
> Evidence: Empty completion evidence. No feat/fix commit for #1194.

## Summary

Closed by splitting the current name-resolution blocker to
`issues/done/5255-resolve-super-property-accesses.md`.

## Problem

Reference test results previously showed 1 `parser-syntax` failure in
`classExtendingAny`.

Problem: fresh triage now resolves the bucket to `UnresolvedName` for the
parsed `super['unknown']` element-access expression.

## Current failure

Representative reproduction:

```sh
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/classExtendingAny.ts
```

Coverage window:

```sh
python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/classExtendingAny.ts --detail --no-dashboard-data
```

## Desired final state

This generated bucket is either split into implementation-ready child issues or superseded by an existing open/done issue with matching evidence. Do not implement directly from this bucket.

## Scope

In scope:

- [x] Inspect the smart triage report below
- [x] Confirm no existing open/done issue owns the exact current boundary
- [x] Split one observable behavior into an implementation-ready child issue
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

- [x] Duplicate candidates below are confirmed as no-match for the exact current boundary
- [x] Child issue contains an exact `python scripts/manager.py reference-triage ...` command
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
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/classExtendingAny.ts --detail
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/classExtendingAny.ts
```

Not run:

- none

## Docs / current-state / issue sync

Final-state docs:

- [x] not affected

Current state:

- [x] not affected

Follow-up issues:

- [x] `issues/done/5255-resolve-super-property-accesses.md`

## Notes

## Affected test files

- `reference/typescript/tests/cases/compiler/classExtendingAny.ts`

Source context:

```ts
declare var Err: any
class A extends Err {
    payload: string
    constructor() {
        super(1,2,3,3,4,56)
        super.unknown
        super['unknown']
    }
    process() {
        return this.payload + "!";
    }
}
```

## Duplicate detection

- none found by path/title/feature scan

## Smart triage

Fresh commands:

```sh
python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/classExtendingAny.ts --detail --no-dashboard-data
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/classExtendingAny.ts
```

Observed result on 2026-05-06:

```text
coverage: unsupported=1
unsupported_diagcodes: UnresolvedName:1
unsupported_features: name-resolution:1

Diagnostic: UnresolvedName
Message: unresolved name: `super` at 252..257
Feature label: name-resolution
Tokens: ok
AST: ok; ClassDecl A extends Err and super property expressions are parsed
Resolved pipeline: resolve_names fails
```

TypeScript's oracle reports only `TS2564` for `payload` not being definitely
assigned. The current ts2wasm blocker is the resolver treating `super` in
`super['unknown']` as a normal unresolved identifier instead of a special
receiver or explicit super-property semantic boundary.

Split issue:

- `issues/done/5255-resolve-super-property-accesses.md`

## Completion evidence

Fill only when moving to `done/`.

Commits:

- Split to `issues/done/5255-resolve-super-property-accesses.md`; see local commit for this issue cleanup.

Validation result:

```text
command: python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/classExtendingAny.ts
result: pass; current blocker split to issue 5255
date: 2026-05-06
```

Remaining risks:

- `classExtendingAny.ts` may expose a later runtime/class semantics blocker
  after 5255 advances past the current resolver-symbol failure.
