---
id: 1225
title: "Implement Classmemberwithmissingidentifier"
type: spike
area: frontend/syntax
class: triage-needed
priority: P1
depends_on: [5265]
blocks: []
created: 2026-05-01
updated: 2026-05-06
---
> **Reopened by false-done audit** (2026-05-07)
> Classification: must-reopen
> Reason: Batch-migrated to issues/done/ without implementation commits.
> Evidence: Empty completion evidence. No feat/fix commit for #1225.

## Summary

Closed by splitting the current missing class member identifier parser
diagnostic to
`issues/done/5265-report-missing-class-member-identifier-after-modifier.md`.

## Problem

Reference test results show 2 cases failing in directory
`classMemberWithMissingIdentifier` with diagnostics: parser-syntax. Fresh triage
shows both fail at the same `public {` class member boundary.

Problem: `public {};` and `public {[name:string]:VariableDeclaration};` report
`expected property name, got LeftBrace` instead of a missing class member
declaration diagnostic after the access modifier.

## Current failure

Representative reproduction:

```sh
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/classMemberWithMissingIdentifier.ts
```

Coverage window:

```sh
python scripts/manager.py reference-coverage tsc --path-filter classMemberWithMissingIdentifier --detail --no-dashboard-data
```

## Desired final state

This generated bucket is either split into implementation-ready child issues or superseded by an existing open/done issue with matching evidence. Do not implement directly from this bucket.

## Scope

In scope:

- [x] Inspect the smart triage report below
- [x] Confirm existing open/done issues do not cover the exact current boundary
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
- [x] Child issue contains exact `python scripts/manager.py reference-triage ...` commands
- [x] Child issue includes failing paths, diagnostic code, source context, visible symbols, and parser/TypeScript AST evidence
- [x] Child issue acceptance names the exact fixture/reference paths and diagnostic/stdout change

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run
```

Impacted commands:

```sh
mise run reference-coverage -- tsc --limit 4
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/classMemberWithMissingIdentifier.ts --detail
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/classMemberWithMissingIdentifier.ts
```

Not run:

- none

## Docs / current-state / issue sync

Final-state docs:

- [x] not affected

Current state:

- [x] not affected

Follow-up issues:

- [x] `issues/done/5265-report-missing-class-member-identifier-after-modifier.md`

## Notes

## Affected test files

- `reference/typescript/tests/cases/compiler/classMemberWithMissingIdentifier.ts`
- `reference/typescript/tests/cases/compiler/classMemberWithMissingIdentifier2.ts`

Source contexts:

```ts
class C {
    public {};
}
```

```ts
class C {
    public {[name:string]:VariableDeclaration};
}
```

## Duplicate detection

- Broad parser-syntax buckets are not exact matches because this blocker is the
  specific class member modifier followed by `{` missing-identifier boundary.
- No existing open/done issue owns the `classMemberWithMissingIdentifier*`
  references.

## Smart triage

Fresh commands:

```sh
python scripts/manager.py reference-coverage tsc --path-filter classMemberWithMissingIdentifier --detail --no-dashboard-data
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/classMemberWithMissingIdentifier.ts
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/classMemberWithMissingIdentifier2.ts
```

Observed result on 2026-05-06:

```text
coverage: unsupported=2
unsupported_diagcodes: UnsupportedSyntax:2
unsupported_features: unknown-unsupported:2

classMemberWithMissingIdentifier.ts:
  Diagnostic: UnsupportedSyntax
  Message: expected property name, got LeftBrace at 63..64
  Source: public {};
  TypeScript oracle: TS1146 Declaration expected, TS1005 ';' expected

classMemberWithMissingIdentifier2.ts:
  Diagnostic: UnsupportedSyntax
  Message: expected property name, got LeftBrace at 63..64
  Source: public {[name:string]:VariableDeclaration};
  TypeScript oracle: TS1146 Declaration expected, plus recovery diagnostics
```

Split issue:

- `issues/done/5265-report-missing-class-member-identifier-after-modifier.md`

## Completion evidence

Fill only when moving to `done/`.

Commits:

- Split to `issues/done/5265-report-missing-class-member-identifier-after-modifier.md`; see local commit for this issue cleanup.

Validation result:

```text
command: python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/classMemberWithMissingIdentifier.ts
result: pass; current blocker split to issue 5265
date: 2026-05-06
```

Remaining risks:

- none
