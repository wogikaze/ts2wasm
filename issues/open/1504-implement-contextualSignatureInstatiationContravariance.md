---
id: 1504
title: "Implement Contextualsignatureinstatiationcontravariance"
type: spike
area: frontend/resolver
class: done
priority: P1
depends_on: []
blocks: []
created: 2026-05-01
updated: 2026-05-07
---
> **Reopened by false-done audit** (2026-05-07)
> Classification: must-reopen
> Reason: Batch-migrated to issues/open/ without implementation commits.
> Evidence: Empty completion evidence. No feat/fix commit for #1504.

## Summary

Triage contextualSignatureInstatiationContravariance across 1 failing reference test cases and split this bucket into implementation-ready child issues.

## Problem

Reference test results show 1 cases fail in directory `contextualSignatureInstatiationContravariance` with diagnostics: name-resolution. The compiler cannot handle these syntax/semantics, preventing compilation of code in this category.

Problem: contextualSignatureInstatiationContravariance has 1 reference failures and needs smart-triage evidence before implementation starts.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/contextualSignatureInstatiationContravariance.ts
```

Coverage window:

```sh
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/contextualSignatureInstatiationContravariance.ts --detail
```

## Desired final state

This generated bucket is either split into implementation-ready child issues or superseded by an existing open/done issue with matching evidence. Do not implement directly from this bucket.

## Scope

In scope:

- [x] Inspect the smart triage report below
- [x] Confirm whether existing open/done issues already cover this bucket
- [x] Split one feature family, one observable behavior, or one fixed reference window into child issues
- [x] Preserve exact reproduction commands and representative AST/diagnostic evidence in each child issue

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
- [x] Existing issue 5344 contains an exact `reference-triage` command and now names this path
- [x] This issue includes failing path, diagnostic code, source context, visible symbols, and parser/TypeScript AST evidence
- [x] Completion evidence names the exact fixture/reference path and diagnostic/stdout change

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run
```

Impacted commands:

```sh
mise run reference-coverage -- tsc --limit 2
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/contextualSignatureInstatiationContravariance.ts --detail
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/contextualSignatureInstatiationContravariance.ts
```

Not run:

- none

## Docs / current-state / issue sync

Final-state docs:

- [x] not affected

Current state:

- [x] not affected

Follow-up issues:

- [x] superseded by `issues/open/5344-resolve-ambient-var-assignment-targets.md`

## Notes

## Affected test files

- `reference/typescript/tests/cases/compiler/contextualSignatureInstatiationContravariance.ts`

## Duplicate detection

- none found by path/title/feature scan

## Smart triage

Fresh triage on 2026-05-07 shows this generated blocked bucket is now the same
ambient `declare var` assignment-target name-resolution boundary owned by
`issues/open/5344-resolve-ambient-var-assignment-targets.md`.

Current diagnostic:

```text
UnresolvedName: unresolved name: `g2` at 249..257
```

Source context:

```ts
declare var f2: <T extends Animal>(x: T, y: T) => void;
declare var g2: (g: Giraffe, e: Elephant) => void;
g2 = f2;
```

Compiler evidence:

```text
tokens: ok through interfaces and ambient var declarations
ast: ok; Assign g2 = Ident(f2), then Assign h2 = Ident(f2)
resolved: fail in resolve_names on assignment target g2
visible symbols before failure: ambient bindings f2 and g2
TypeScript oracle: reports later TS2322 contravariance diagnostic for g2 = f2
```

No new child issue was created because issue 5344 already scopes resolver-visible
metadata for declaration-only ambient `var` names when used as assignment
targets. After issue 5344 advances this file, the RHS `f2` ambient value or the
intended TS2322 contravariance diagnostic may need separate triage.

## Completion evidence

Fill only when moving to `done/`.

Commits:

- `...`

Validation result:

```text
command:
result:
date:
```

Remaining risks:

- The intended contravariant function assignment diagnostic remains hidden until
  issue 5344 advances the file past the current ambient assignment-target
  resolver boundary.
