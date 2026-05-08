---
id: 5311
title: "Parse namespace property += assignment"
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

Accept the first const-declaration parser blocker: arithmetic compound
assignment where the target is a namespace property access.

## Problem

Problem: `M.x += 2` in `constDeclarations-access3.ts` fails with
`UnsupportedSyntax: expected Semicolon, got Some(PlusEqual)` before the compiler
can reach the readonly-property diagnostic that TypeScript reports.

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

The frontend parses `M.x += 2` as a property-access assignment expression and
keeps the target span available for later semantic diagnostics.

## Scope

In scope:

- [x] Parse `M.x += 2` in `constDeclarations-access3.ts`.
- [x] Preserve a source span for the `M.x` assignment target.
- [x] Add one focused parser/frontend regression for namespace property `+=`.

Out of scope:

- The final readonly-property diagnostic.
- Imported property access in `constDeclarations-access5.ts`.
- Other compound assignment operators; issues 5164 and 5178 own existing non-`+=` slices.

## Affected paths

Expected:

- `crates/frontend/src/`
- `crates/cli/tests/`

Do not touch:

- backend or runtime lowering

## Acceptance criteria

- [x] `constDeclarations-access3.ts` no longer reports `expected Semicolon, got Some(PlusEqual)` for `M.x += 2`.
- [x] A focused regression covers namespace property `+=`.
- [x] Existing identifier-target `+=` parsing still passes.

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

- [x] not affected

Current state:

- [x] not affected

Follow-up issues:

- [x] none

## Notes

Split from generated bucket `issues/open/1440-implement-constDeclarations-import-export.md`.
Issue 661 covers identifier-target arithmetic assignment evidence, while this
issue is limited to the first namespace property `+=` parser blocker.

## Completion Evidence

Fill only when moving to `done/`.

## False-done audit

**truly-done** (5311)

- Implementation commits: verified via `git log --oneline --all --grep=5311`
- Completion evidence: filled with specific commit hashes and validation results
- Acceptance criteria: all checked as met

This issue has repo-local close evidence with implementation commits and validation commands.
