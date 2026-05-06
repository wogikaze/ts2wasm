---
id: 1242
title: "Implement Classupdatetests"
type: spike
area: runtime/builtins
class: triage-needed
priority: P1
depends_on: []
blocks: []
created: 2026-05-01
updated: 2026-05-06
---

## Summary

Closed after splitting the current blocker to
`issues/open/5268-support-derived-constructor-parameter-properties-after-super.md`.
Fresh triage shows the current failure is the issue-226 derived constructor
parameter-property boundary.

## Problem

Reference test results show 1 case fails in directory `classUpdateTests`. Fresh
triage confirms the current blocker is not generic parser syntax; it is the
unsupported derived constructor parameter-property ordering boundary.

Problem: `classUpdateTests.ts` reports `issue-226` for
`constructor(private p1:number)` in a derived class whose body begins with
`super()`.

## Current failure

Representative reproduction:

```sh
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/classUpdateTests.ts
```

Coverage window:

```sh
python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/classUpdateTests.ts --detail --no-dashboard-data
```

## Desired final state

This generated bucket is either split into implementation-ready child issues or superseded by an existing open/done issue with matching evidence. Do not implement directly from this bucket.

## Scope

In scope:

- [x] Inspect the smart triage report below
- [x] Confirm done issue 226 covers basic parameter properties but not this remaining derived-constructor form
- [x] Split one observable behavior into child issue 5268
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

- [x] Duplicate candidates below are confirmed as no-match for the exact remaining behavior
- [x] Child issue 5268 contains an exact `python scripts/manager.py reference-triage ...` command
- [x] Child issue includes failing path, diagnostic code, source context, visible symbols, and parser/TypeScript AST evidence
- [x] Child issue acceptance names the exact reference path and diagnostic change

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run
```

Impacted commands:

```sh
mise run reference-coverage -- tsc --limit 2
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/classUpdateTests.ts --detail
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/classUpdateTests.ts
```

Not run:

- none

## Docs / current-state / issue sync

Final-state docs:

- [x] not affected

Current state:

- [x] not affected

Follow-up issues:

- [x] created: `issues/open/5268-support-derived-constructor-parameter-properties-after-super.md`

## Notes

## Affected test files

- `reference/typescript/tests/cases/compiler/classUpdateTests.ts`

Source context:

```ts
class L extends G {
    constructor(private p1:number) {
        super(); // NO ERROR
    }
}
```

## Duplicate detection

- `issues/done/226-implement-parameter-properties.md` covers the basic
  parameter-property slice and records remaining unsupported forms as precise
  diagnostics.
- No existing open issue owned the derived constructor + leading `super()`
  parameter-property behavior, so this bucket was split to issue 5268.

## Smart triage

Fresh commands:

```sh
python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/classUpdateTests.ts --detail --no-dashboard-data
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/classUpdateTests.ts
```

Observed result on 2026-05-06:

```text
coverage: unsupported=1
unsupported_diagcodes: UnsupportedSyntax:1
unsupported_features: unknown-unsupported:1

Diagnostic: UnsupportedTypeScriptSyntax
Message: issue-226: parameter properties in derived constructors require a leading super(...) call at 906..916
Source: constructor(private p1:number) { super(); }
tokens: ok
AST: fails on derived constructor parameter property
resolved: same unsupported-feature boundary
TypeScript oracle: parses; later expected diagnostics include TS2415
Child issue: 5268
```

## Completion evidence

Fill only when moving to `done/`.

Commits:

- Split to `issues/open/5268-support-derived-constructor-parameter-properties-after-super.md`; see local commit for this issue cleanup.

Validation result:

```text
command: python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/classUpdateTests.ts
result: pass; reproduced issue-226 derived constructor parameter-property boundary and split child issue 5268
date: 2026-05-06
```

Remaining risks:

- none; remaining implementation work is tracked by issue 5268
