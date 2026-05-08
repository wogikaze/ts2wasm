---
id: 1205
title: "Implement Classextendsnull"
type: spike
area: frontend/syntax
class: triage-needed
priority: P2
depends_on: [5258, 5259]
blocks: []
created: 2026-05-01
updated: 2026-05-06
---
> **Reopened by false-done audit** (2026-05-07)
> Classification: must-reopen
> Reason: Batch-migrated to issues/open/ without implementation commits.
> Evidence: Empty completion evidence. No feat/fix commit for #1205.

## Summary

Closed by splitting the current `extends null` class heritage boundary to
constructor-super and super-property child issues.

## Problem

Reference test results showed 3 cases in `classExtendsNull` with diagnostic
`unknown-unsupported`.

Problem: fresh triage shows all representatives parse `extends null` into the
AST, then builtin resolution rejects the `Null` heritage with the generic
`only simple inheritance (extends ClassName) is supported` diagnostic.

## Current failure

Representative reproduction:

```sh
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/classExtendsNull.ts
```

Coverage window:

```sh
python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/classExtendsNull --detail --no-dashboard-data
```

## Desired final state

This generated bucket is either split into implementation-ready child issues or superseded by an existing open/done issue with matching evidence. Do not implement directly from this bucket.

## Scope

In scope:

- [x] Inspect the smart triage report below
- [x] Confirm no existing open/done issue owns `extends null`
- [x] Split one feature family into an implementation-ready child issue
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
- [x] Child issue contains exact `python scripts/manager.py reference-triage ...` commands
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
mise run reference-coverage -- tsc --limit 6
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/classExtendsNull.ts --detail
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/classExtendsNull.ts
```

Not run:

- none

## Docs / current-state / issue sync

Final-state docs:

- [x] not affected

Current state:

- [x] not affected

Follow-up issues:

- [x] `issues/open/5258-support-class-extends-null-boundary.md`
- [x] `issues/open/5259-report-super-property-access-in-class-extends-null.md`

## Notes

## Affected test files

- `reference/typescript/tests/cases/compiler/classExtendsNull.ts`
- `reference/typescript/tests/cases/compiler/classExtendsNull2.ts`
- `reference/typescript/tests/cases/compiler/classExtendsNull3.ts`

Source context:

```ts
class C extends null {
    constructor() {
        super();
        return Object.create(null);
    }
}
```

```ts
class C1 extends null {
  static method() {
    super.oops;
  }
}
```

## Duplicate detection

- none found by path/title/feature scan

## Smart triage

Fresh commands:

```sh
python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/classExtendsNull --detail --no-dashboard-data
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/classExtendsNull.ts
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/classExtendsNull2.ts
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/classExtendsNull3.ts
```

Observed result on 2026-05-06:

```text
coverage: executed=3 unsupported=3
unsupported_diagcodes: UnsupportedSyntax:3
unsupported_features: unknown-unsupported:3

All representatives:
  Diagnostic: UnsupportedSyntax
  Message: only simple inheritance (extends ClassName) is supported
  Tokens: ok
  AST: ok; ClassDecl extends Null
  Resolved/builtins: fails in resolve_builtins
```

TypeScript accepts the `extends null` syntax and reports more specific semantic
diagnostics:

- `classExtendsNull.ts`: TS17005 for `super()` in a class extending `null`.
- `classExtendsNull2.ts`: TS2417 static-side/null plus TS17005 for `super()`.
- `classExtendsNull3.ts`: TS2531 for `super.oops` because the base is possibly null.

Split issues:

- `issues/open/5258-support-class-extends-null-boundary.md`: constructor
  `super()` in classes extending `null`.
- `issues/open/5259-report-super-property-access-in-class-extends-null.md`:
  `super.oops` in static/instance methods of classes extending `null`.

## Completion evidence

Fill only when moving to `done/`.

Commits:

- Split to `issues/open/5258-support-class-extends-null-boundary.md` and
  `issues/open/5259-report-super-property-access-in-class-extends-null.md`;
  see local commit for this issue cleanup.

Validation result:

```text
command: python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/classExtendsNull.ts
result: pass; current blocker split to issue 5258
date: 2026-05-06
```

Remaining risks:

- After 5258 advances past the generic heritage diagnostic, the references may
  expose later `Object.create(null)` builtin/runtime blockers.
