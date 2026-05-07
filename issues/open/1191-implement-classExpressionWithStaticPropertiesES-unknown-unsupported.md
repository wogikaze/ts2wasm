---
id: 1191
title: "Implement Classexpressionwithstaticpropertieses Unknown Unsupported"
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
> Evidence: Empty completion evidence. No feat/fix commit for #1191.

## Summary

Closed as superseded by `issues/open/5248-lower-class-expressions.md`.
Fresh triage shows both ES unknown-unsupported representatives parse to
`ClassExpr` and stop at the existing `issue-313` class-expression lowering
boundary.

## Problem

Reference test results showed 2 cases in
`classExpressionWithStaticPropertiesES-unknown-unsupported` with diagnostic
`unknown-unsupported`.

Problem: fresh smart triage resolves both representatives to
`UnsupportedSyntax: issue-313: class expression lowering not yet implemented`.

## Current failure

Representative reproduction:

```sh
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/classExpressionWithStaticPropertiesES64.ts
```

Coverage window:

```sh
python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/classExpressionWithStaticPropertiesES --detail --no-dashboard-data
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
- [x] Existing owner contains the class-expression lowering command and added representative notes
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
mise run reference-coverage -- tsc --limit 4
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/classExpressionWithStaticPropertiesES64.ts --detail
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/classExpressionWithStaticPropertiesES64.ts
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

- `reference/typescript/tests/cases/compiler/classExpressionWithStaticPropertiesES64.ts`
- `reference/typescript/tests/cases/compiler/classExpressionWithStaticPropertiesES63.ts`

Source contexts:

```ts
// ES64
(class { static x = 0; });
```

```ts
// ES63
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

Fresh commands:

```sh
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/classExpressionWithStaticPropertiesES64.ts
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/classExpressionWithStaticPropertiesES63.ts
```

Observed result on 2026-05-06:

```text
classExpressionWithStaticPropertiesES64.ts:
  Diagnostic: UnsupportedSyntax
  Message: issue-313: class expression lowering not yet implemented
  Tokens: ok
  AST: Expr -> ClassExpr { name: "", ... }
  Resolved/lowered: lower_program fails

classExpressionWithStaticPropertiesES63.ts:
  Diagnostic: UnsupportedSyntax
  Message: issue-313: class expression lowering not yet implemented
  Tokens: ok
  AST: arr.push(ClassExpr { name: "C", ... })
  Resolved/lowered: lower_program fails
```

TypeScript accepts ES64 without diagnostics. TypeScript accepts ES63 syntax and
reports only the fixture-level `console` redeclaration diagnostic (`TS2403`) for
the selected lib setup.

Superseding owner:

- `issues/open/5248-lower-class-expressions.md`

Reason:

- 5248 already owns the implementation-ready `issue-313` class-expression
  lowering boundary. ES63 adds the same call-argument representative as 1189 in
  an ES target. ES64 adds an anonymous parenthesized expression-statement
  representative; if that expression-position behavior is not covered by the
  first 5248 lowering slice, split it from 5248 after the initial implementation
  advances.

## Completion evidence

Fill only when moving to `done/`.

Commits:

- Superseded by `issues/open/5248-lower-class-expressions.md`; see local commit for this issue cleanup.

Validation result:

```text
command: python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/classExpressionWithStaticPropertiesES64.ts
result: pass; diagnostic resolved to issue-313 class expression lowering
date: 2026-05-06
```

Remaining risks:

- ES64 is anonymous and parenthesized. 5248's first acceptance slice is
  named assignment/initializer-focused, so a narrower anonymous/expression
  position child may still be needed after 5248 advances.
