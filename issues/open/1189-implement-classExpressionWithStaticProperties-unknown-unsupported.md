---
id: 1189
title: "Implement Classexpressionwithstaticproperties Unknown Unsupported"
type: spike
area: frontend/syntax
class: triage-needed
priority: P2
depends_on: [5248]
blocks: []
created: 2026-05-01
updated: 2026-05-06
---
> **Reopened by false-done audit** (2026-05-07)
> Classification: must-reopen
> Reason: Batch-migrated to issues/done/ without implementation commits.
> Evidence: Empty completion evidence. No feat/fix commit for #1189.

## Summary

Closed as superseded by `issues/open/5248-lower-class-expressions.md`.
Fresh triage shows this generated bucket is not a separate frontend syntax
family: `classExpressionWithStaticProperties3.ts` parses to `ClassExpr`, then
lowering stops at the existing `issue-313` class-expression boundary.

## Problem

Reference test results showed 1 case in
`classExpressionWithStaticProperties-unknown-unsupported` with diagnostic
`unknown-unsupported`.

Problem: fresh smart triage resolves the unknown bucket to
`UnsupportedSyntax: issue-313: class expression lowering not yet implemented`.

## Current failure

Representative reproduction:

```sh
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/classExpressionWithStaticProperties3.ts
```

Coverage window:

```sh
python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/classExpressionWithStaticProperties3.ts --detail --no-dashboard-data
```

## Desired final state

This generated bucket is either split into implementation-ready child issues or superseded by an existing open/done issue with matching evidence. Do not implement directly from this bucket.

## Scope

In scope:

- [x] Inspect the smart triage report below
- [x] Confirm whether existing open/done issues already cover this bucket
- [x] Supersede with the existing class-expression lowering owner
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
- [x] Existing owner contains the exact class-expression lowering command
- [x] This issue includes failing path, diagnostic code, source context, visible symbols, and parser/TypeScript AST evidence
- [x] Existing owner names the diagnostic/stdout change for the lowering boundary

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run
```

Impacted commands:

```sh
mise run reference-coverage -- tsc --limit 2
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/classExpressionWithStaticProperties3.ts --detail
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/classExpressionWithStaticProperties3.ts
```

Not run:

- none

## Docs / current-state / issue sync

Final-state docs:

- [x] not affected

Current state:

- [x] not affected

Follow-up issues:

- [x] superseded by `issues/open/5248-lower-class-expressions.md`

## Notes

## Affected test files

- `reference/typescript/tests/cases/compiler/classExpressionWithStaticProperties3.ts`

Source context:

```ts
declare var console: any;
const arr: {y(): number}[] = [];
for (let i = 0; i < 3; i++) {
    arr.push(class C {
        static x = i;
        static y = () => C.x * 2;
    });
}
arr.forEach(C => console.log(C.y()));
```

## Duplicate detection

- `issues/open/694-implement-arrayToLocaleStringES-unknown-unsupported.md` - Implement Arraytolocalestringes Unknown Unsupported (same feature label, title overlap)

## Smart triage

Fresh command:

```sh
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/classExpressionWithStaticProperties3.ts
```

Observed result on 2026-05-06:

```text
Smart triage: Triage class
Feature label: class
Diagnostic: UnsupportedSyntax
Message: issue-313: class expression lowering not yet implemented
Tokenization: ok
Parser AST: ok
Resolved/lowered pipeline: fails in lower_program
```

The parser-visible representative is a `ClassExpr` in call-argument position:
`arr.push(class C { ... })`. TypeScript's parser accepts the source and reports
only the fixture-level `console` redeclaration diagnostic (`TS2403`) from the
selected lib setup.

Superseding owner:

- `issues/open/5248-lower-class-expressions.md`

Reason:

- 5248 already tracks the implementation-ready `issue-313` class-expression
  lowering boundary. This bucket adds a call-argument representative; if the
  first assignment/initializer slice in 5248 does not cover that expression
  position, split a follow-up from 5248 after the first lowering implementation.

## Completion evidence

Fill only when moving to `done/`.

Commits:

- Superseded by `issues/open/5248-lower-class-expressions.md`; see local commit for this issue cleanup.

Validation result:

```text
command: python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/classExpressionWithStaticProperties3.ts
result: pass; diagnostic resolved to issue-313 class expression lowering
date: 2026-05-06
```

Remaining risks:

- `classExpressionWithStaticProperties3.ts` uses a class expression as a call
  argument. 5248's first acceptance slice is assignment/initializer-focused, so
  a narrower expression-position child may still be needed after 5248 advances.
