---
id: 1440
title: "Implement Constdeclarations Import Export"
type: spike
area: frontend/syntax
class: done
priority: P1
depends_on: []
blocks: []
created: 2026-05-01
updated: 2026-05-07
completed: 2026-05-07
---

## Summary

Triage constDeclarations-import-export across 5 failing reference test cases and split this bucket into implementation-ready child issues.

## Problem

Reference test results show 5 cases fail in directory `constDeclarations-import-export` with diagnostics: import-export. The compiler cannot handle these syntax/semantics, preventing compilation of code in this category.

Problem: constDeclarations-import-export has 5 reference failures and needs smart-triage evidence before implementation starts.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/constDeclarations-access3.ts
```

Coverage window:

```sh
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/constDeclarations-access3.ts --detail
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
- [x] At least one child issue contains an exact `reference-triage` command
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
mise run reference-coverage -- tsc --limit 10
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/constDeclarations-access3.ts --detail
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/constDeclarations-access3.ts
```

Not run:

- none

## Docs / current-state / issue sync

Final-state docs:

- [x] not affected

Current state:

- [x] not affected

Follow-up issues:

- [x] created: `issues/open/5311-parse-property-access-arithmetic-compound-assignments.md`

## Notes

## Affected test files

- `reference/typescript/tests/cases/compiler/constDeclarations-access3.ts`
- `reference/typescript/tests/cases/compiler/constDeclarations-access5.ts`
- `reference/typescript/tests/cases/compiler/constDeclarations-access4.ts`
- `reference/typescript/tests/cases/compiler/constDeclarations-ambient.ts`
- `reference/typescript/tests/cases/compiler/constDeclarations2.ts`

## Duplicate detection

- `issues/open/432-implement-import-export.md` - Implement import/export module syntax (same feature label, title overlap)
- `issues/done/457-implement-APISample-import-export.md` - Implement Apisample Import Export (same feature label, title overlap)
- `issues/open/463-implement-FunctionDeclaration-import-export.md` - Implement Functiondeclaration Import Export (same feature label, title overlap)
- `issues/open/543-implement-APISample-import-export.md` - Implement Apisample Import Export (same feature label, title overlap)
- `issues/done/549-implement-FunctionDeclaration-import-export.md` - Implement Functiondeclaration Import Export (same feature label, title overlap)
- `issues/open/662-implement-arrayAssignmentTest-import-export.md` - Implement Arrayassignmenttest Import Export (same feature label, title overlap)
- `issues/open/732-implement-assignmentCompatability-import-export.md` - Implement Assignmentcompatability Import Export (same feature label, title overlap)
- `issues/done/766-implement-augmentedTypesEnum-import-export.md` - Implement Augmentedtypesenum Import Export (same feature label, title overlap)
- `issues/done/055-implement-import-export.md` - Umbrella: implement import and export (same feature label, title overlap)

## Smart triage

Fresh triage on 2026-05-07 showed the generated import/export label is stale
for the representative blockers. The first observable failures are parser
failures for property access arithmetic compound assignments:

```sh
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/constDeclarations-access3.ts
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/constDeclarations-access5.ts
python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/constDeclarations --detail --no-dashboard-data
```

Observed evidence:

```text
constDeclarations-access3.ts:
UnsupportedSyntax expected Semicolon, got Some(PlusEqual) at 110..112
source context:
10 | M.x = 1;
11 | M.x += 2;
12 | M.x -= 3;
visible symbol: namespace export const x
TypeScript oracle: TS2540 Cannot assign to 'x' because it is a read-only property.

constDeclarations-access5.ts:
UnsupportedSyntax expected Semicolon, got Some(PlusEqual)
source includes:
export const x = 0;
import m = require('./constDeclarations_access_1');
m.x = 1;
m.x += 2;
TypeScript AST includes expression statements for the property compound assignments.
```

Focused coverage for `constDeclarations` reported:

```text
executed=17
build_pass=5
unsupported=12
unsupported_diagcodes=UnsupportedSyntax:8,UnresolvedName:2,DuplicateLocal:1,UnsupportedTypeScriptSyntax:1
constDeclarations-ambient.ts: build_pass
constDeclarations2.ts: build_pass
constDeclarations-access3.ts: UnsupportedSyntax unknown-unsupported
constDeclarations-access5.ts: UnsupportedSyntax unknown-unsupported
```

Split child: `issues/open/5311-parse-property-access-arithmetic-compound-assignments.md`.

Existing related issues are no-match for this exact first blocker:

- Issue 661 covers identifier-target arithmetic assignment typing and currently stops on `f *= 1`.
- Issue 5178 covers bitwise compound assignment operators only.
- Issue 5164 covers exponentiation compound assignment only.
- Issue 5300 covers class-binding assignment diagnostics only.

## Completion evidence

Fill only when moving to `done/`.

Commits:

- pending

Validation result:

```text
command: python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/constDeclarations-access3.ts
result: pass; current blocker identified as property access arithmetic compound assignment parser syntax, split to issue 5311
date: 2026-05-07

command: python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/constDeclarations-access5.ts
result: pass; same property access compound assignment parser blocker confirmed for imported const access
date: 2026-05-07

command: python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/constDeclarations --detail --no-dashboard-data
result: pass; mixed stale bucket confirmed with 5 build-pass cases and property-compound/parser blockers among remaining unsupported cases
date: 2026-05-07
```

Remaining risks:

- Further constDeclarations cases still expose separate parser, name-resolution, and duplicate-local blockers after issue 5311 advances.
