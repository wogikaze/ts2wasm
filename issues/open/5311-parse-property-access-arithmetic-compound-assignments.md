---
id: 5311
title: "Parse property-access += assignments"
type: feature
area: frontend/syntax
class: implementation-ready
priority: P1
depends_on: []
blocks: []
created: 2026-05-07
updated: 2026-05-07
---

## Summary

Accept the first property-access compound-assignment parser blocker:
arithmetic `+=` where the target is a property access.

## Problem

Problem: `M.x += 2` in `constDeclarations-access3.ts` and
`stringMap.foo += 1` in `noUncheckedIndexedAccessCompoundAssignments.ts` fail with
`UnsupportedSyntax: expected Semicolon, got Some(PlusEqual)` before the compiler
can reach the relevant semantic diagnostics that TypeScript reports or accepts.

## Current failure

```sh
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/constDeclarations-access3.ts
```

Observed 2026-05-07:

```text
failure: UnsupportedSyntax expected Semicolon, got Some(PlusEqual) at 110..112
source:
10 | M.x = 1;
11 | M.x += 2;
12 | M.x -= 3;
visible symbol: namespace export const x
TypeScript oracle: TS2540 Cannot assign to 'x' because it is a read-only property.
```

## Desired final state

The frontend parses property-access `+=` expressions such as `M.x += 2` and
`stringMap.foo += 1` as assignment expressions and keeps the target span
available for later semantic diagnostics.

## Scope

In scope:

- [ ] Parse `M.x += 2` in `constDeclarations-access3.ts`.
- [ ] Parse `stringMap.foo += 1` in `noUncheckedIndexedAccessCompoundAssignments.ts`.
- [ ] Preserve a source span for the property-access assignment target.
- [ ] Add one focused parser/frontend regression for property-access `+=`.

Out of scope:

- The final readonly-property diagnostic.
- Imported property access in `constDeclarations-access5.ts`.
- Other compound assignment operators such as property-access `*=`.
- Element-access compound assignments; see issue 5478 for element-access `+=`.
- Bitwise/exponentiation compound operators; issues 5164 and 5178 own existing non-arithmetic slices.

## Affected paths

Expected:

- `crates/frontend/src/`
- `crates/cli/tests/`

Do not touch:

- backend or runtime lowering

## Acceptance criteria

- [ ] `constDeclarations-access3.ts` no longer reports `expected Semicolon, got Some(PlusEqual)` for `M.x += 2`.
- [ ] `noUncheckedIndexedAccessCompoundAssignments.ts` no longer reports `expected Semicolon, got Some(PlusEqual)` for `stringMap.foo += 1`.
- [ ] A focused regression covers property-access `+=`.
- [ ] Existing identifier-target `+=` parsing still passes.

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run -p ts2wasm-frontend
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/constDeclarations-access3.ts
```

Impacted commands:

```sh
python scripts/manager.py check-issue-readiness -- --fail-ready-below 80
```

Not run:

- none

## Docs / current-state / issue sync

Final-state docs:

- [ ] not affected

Current state:

- [ ] not affected

Follow-up issues:

- [ ] none

## Notes

Split from generated bucket `issues/done/1440-implement-constDeclarations-import-export.md`.
Issue 661 covers identifier-target arithmetic assignment evidence, while this
issue is limited to property-access `+=` parser blockers.

2026-05-08 fold-in:

- `issues/done/3570-implement-noUncheckedIndexedAccessCompoundAssignments.md`
  reaches the same property-access `+=` parser boundary at
  `stringMap.foo += 1`.
- The later property-access `*=`, element-access updates, element-access `+=`,
  and noUncheckedIndexedAccess-specific semantic diagnostics should be
  re-triaged after this issue advances.

## Completion Evidence

Fill only when moving to `done/`.
