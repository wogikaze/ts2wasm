---
id: 3608
title: "Implement Nounusedtypeparameterconstraint"
type: spike
area: frontend/syntax
class: superseded
priority: P1
depends_on: []
blocks: [5229]
created: 2026-05-01
updated: 2026-05-08
status: done
---

## Summary

Triage nounusedTypeParameterConstraint across 1 failing reference test cases and split this bucket into implementation-ready child issues.

## Problem

Reference test results show 1 cases fail in directory `nounusedTypeParameterConstraint` with diagnostics: import-export. The compiler cannot handle these syntax/semantics, preventing compilation of code in this category.

Problem: nounusedTypeParameterConstraint has 1 reference failures and needs smart-triage evidence before implementation starts.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/nounusedTypeParameterConstraint.ts
```

Coverage window:

```sh
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/nounusedTypeParameterConstraint.ts --detail
```

## Desired final state

This generated bucket is either split into implementation-ready child issues or superseded by an existing open/done issue with matching evidence. Do not implement directly from this bucket.

Current close decision: superseded by
`issues/open/5229-resolve-imports-between-filename-sections.md`.

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
- [x] At least one child issue contains an exact `mise run reference-triage -- ...` command
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
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/nounusedTypeParameterConstraint.ts --detail
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/nounusedTypeParameterConstraint.ts
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

Close note, 2026-05-08:

- Fresh triage shows the current blocker is
  `UnsupportedModule: issue-232: missing local module \`./bar\`` for
  `import { IEventSourcedEntity } from "./bar";`.
- Tokens and AST are ok for the named import; module graph resolution then
  looks for on-disk `bar.ts` / `bar.js` siblings instead of resolving the
  earlier virtual `//@filename: bar.ts` section.
- TypeScript oracle also reports TS2307 in the current raw-source runner view,
  so no-unused type-parameter diagnostics are not reachable until virtual
  `@filename` section import resolution advances.
- Existing issue 5229 owns local import resolution between virtual
  `@Filename` / `@filename` sections, so this generated bucket is folded into
  5229 rather than split into a new child.

## Affected test files

- `reference/typescript/tests/cases/compiler/nounusedTypeParameterConstraint.ts`

## Duplicate detection

- none found by path/title/feature scan

## Smart triage

Fresh smart triage on 2026-05-08:

```text
Diagnostic: UnsupportedModule / unsupported-feature-boundary
Feature label: import-export
Path: reference/typescript/tests/cases/compiler/nounusedTypeParameterConstraint.ts
Failure location: missing local module `./bar` at 221..228
Source context: import { IEventSourcedEntity } from "./bar";
AST: ImportNamed source "./bar"
TypeScript oracle: TS2307 cannot find module "./bar"
```

Focused coverage on 2026-05-08:

```text
executed=1
build_pass=0
unsupported=1
blocked=0
unsupported_diagcodes=UnsupportedModule:1
unsupported_features=import-export:1
```

## Completion evidence

Fill only when moving to `done/`.

Commits:

- `pending local commit`

Validation result:

```text
command: python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/nounusedTypeParameterConstraint.ts
result: pass; reproduced current issue-232 missing local module blocker for virtual section import `./bar`
date: 2026-05-08

command: python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/nounusedTypeParameterConstraint.ts --detail --no-dashboard-data
result: pass; executed=1 build_pass=0 unsupported=1 unsupported_diagcodes=UnsupportedModule:1
date: 2026-05-08
```

Remaining risks:

- Issue 5229 still needs implementation. After virtual `./bar` resolution,
  this file may expose later no-unused type-parameter or type-only
  import/export diagnostics.
