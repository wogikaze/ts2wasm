---
id: 1200
title: "Implement Classextendsinterface Unknown Unsupported"
type: spike
area: frontend/syntax
class: triage-needed
priority: P2
depends_on: [5256]
blocks: []
created: 2026-05-01
updated: 2026-05-06
---
> **Reopened by false-done audit** (2026-05-07)
> Classification: must-reopen
> Reason: Batch-migrated to issues/done/ without implementation commits.
> Evidence: Empty completion evidence. No feat/fix commit for #1200.

## Summary

Closed by splitting the current class heritage diagnostic boundary to
`issues/done/5256-report-non-constructor-class-heritage-expressions.md`.

## Problem

Reference test results showed 1 case in
`classExtendsInterface-unknown-unsupported` with diagnostic
`unknown-unsupported`.

Problem: fresh triage shows the source parses to a class declaration whose
heritage is a member expression, then builtin resolution rejects it with the
generic `only simple inheritance (extends ClassName) is supported` diagnostic.

## Current failure

Representative reproduction:

```sh
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/classExtendsInterface_not.ts
```

Coverage window:

```sh
python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/classExtendsInterface_not.ts --detail --no-dashboard-data
```

## Desired final state

This generated bucket is either split into implementation-ready child issues or superseded by an existing open/done issue with matching evidence. Do not implement directly from this bucket.

## Scope

In scope:

- [x] Inspect the smart triage report below
- [x] Confirm no existing open/done issue owns this exact boundary
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
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/classExtendsInterface_not.ts --detail
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/classExtendsInterface_not.ts
```

Not run:

- none

## Docs / current-state / issue sync

Final-state docs:

- [x] not affected

Current state:

- [x] not affected

Follow-up issues:

- [x] `issues/done/5256-report-non-constructor-class-heritage-expressions.md`

## Notes

## Affected test files

- `reference/typescript/tests/cases/compiler/classExtendsInterface_not.ts`

Source context:

```ts
class C extends "".bogus {}
```

## Duplicate detection

- `issues/open/694-implement-arrayToLocaleStringES-unknown-unsupported.md` - Implement Arraytolocalestringes Unknown Unsupported (same feature label, title overlap)

## Smart triage

Fresh commands:

```sh
python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/classExtendsInterface_not.ts --detail --no-dashboard-data
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/classExtendsInterface_not.ts
```

Observed result on 2026-05-06:

```text
coverage: unsupported=1
unsupported_diagcodes: UnsupportedSyntax:1
unsupported_features: unknown-unsupported:1

Diagnostic: UnsupportedSyntax
Message: only simple inheritance (extends ClassName) is supported
Tokens: ok
AST: ok; ClassDecl C extends Member(String(""), "bogus")
Resolved/builtins: fails in resolve_builtins
```

TypeScript's oracle reports `TS2339` at property `bogus`, proving this is not a
tokenization/parser blocker. The current ts2wasm blocker is the broad
simple-inheritance diagnostic for a parsed non-constructor class heritage
expression.

Split issue:

- `issues/done/5256-report-non-constructor-class-heritage-expressions.md`

## Completion evidence

Fill only when moving to `done/`.

Commits:

- Split to `issues/done/5256-report-non-constructor-class-heritage-expressions.md`; see local commit for this issue cleanup.

Validation result:

```text
command: python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/classExtendsInterface_not.ts
result: pass; current blocker split to issue 5256
date: 2026-05-06
```

Remaining risks:

- `classExtendsInterface_not.ts` is intentionally invalid TypeScript. 5256
  should improve classification/diagnostic quality rather than implement broad
  arbitrary heritage runtime semantics.
