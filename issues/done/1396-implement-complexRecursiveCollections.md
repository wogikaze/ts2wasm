---
id: 1396
title: "Implement Complexrecursivecollections"
type: spike
area: frontend/syntax
class: blocked
priority: P1
depends_on: [432]
blocks: [5187]
created: 2026-05-01
updated: 2026-05-07
---

## Summary

Closed as superseded by
`issues/done/5187-lower-namespace-only-multi-section-files.md`.

Fresh focused triage shows `complexRecursiveCollections.ts` currently exposes
the same namespace-only multi-section boundary already owned by issue 5187: the
virtual section `immutable.ts` contains declaration/namespace-only content and
the dump path reports that namespace lowering is not implemented.

## Problem

Reference test results originally showed 1 case failing in directory
`complexRecursiveCollections` with diagnostics: import-export. Fresh focused
triage on 2026-05-07 reports `UnsupportedRuntimeSubset` for namespace-only
multi-section handling.

Problem: the TypeScript reference file has multiple virtual sections:

```text
// @Filename: complex.ts
... type/interface-only declarations ...

// @Filename: immutable.ts
declare namespace Immutable { ... }
declare module "immutable" { export = Immutable }
```

The smart triage dump path reports that `immutable.ts` contains namespace-only
declarations and cannot be lowered yet.

## Current failure

Representative reproduction:

```sh
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/complexRecursiveCollections.ts
```

Coverage window:

```sh
python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/complexRecursiveCollections.ts --detail --no-dashboard-data
```

Observed 2026-05-07:

```text
smart triage: UnsupportedRuntimeSubset
message: multi-section section `immutable.ts` contains namespace-only declarations; namespace lowering is not implemented
```

Focused batch coverage on the same date:

```text
coverage: executed=1, build_pass=1, unsupported=0, blocked=0
semantic_enabled=0
```

TypeScript oracle evidence:

```text
TypeScript parses the virtual sections and reports later type-system diagnostics
inside the recursive Immutable declarations, not a source parsing failure.
```

## Desired final state

This generated bucket is closed. Implementation proceeds through issue 5187,
which owns preserving or lowering namespace-only multi-section bodies enough for
the next namespace/type diagnostic to surface.

After issue 5187 lands, this reference path should be re-triaged because it may
surface recursive type-system diagnostics, `declare module` handling, or
coverage accounting differences between batch build and smart triage dumps.

## Scope

In scope:

- [x] Inspect the smart triage report below
- [x] Confirm whether existing open/done issues already cover this bucket
- [x] Supersede this bucket with issue 5187's namespace-only multi-section work
- [x] Preserve exact reproduction commands and representative AST/diagnostic evidence

Out of scope:

- Direct implementation from this generated bucket
- Recursive collection type checking
- `declare module "immutable"` runtime/module lowering
- Coverage accounting differences after the batch path reports build pass

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
- [x] Existing issue 5187 covers namespace-only multi-section handling
- [x] This issue includes failing path, diagnostic code, source context, visible symbols, and parser/TypeScript AST evidence
- [x] Completion evidence names the exact reference path and diagnostic/stdout change

## Validation

Required commands:

```sh
python scripts/manager.py update-issue-index
python scripts/manager.py update-issue-index --check
python scripts/manager.py check-issue-health
python scripts/manager.py check-issue-readiness -- --fail-ready-below 80
git diff --check
```

Impacted commands:

```sh
python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/complexRecursiveCollections.ts --detail --no-dashboard-data
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/complexRecursiveCollections.ts
```

Not run:

- `cargo fmt --all --check` (issue metadata only)
- `cargo nextest run` (issue metadata only)

## Docs / current-state / issue sync

Final-state docs:

- [x] not affected

Current state:

- [x] not affected

Follow-up issues:

- [x] superseded by: `issues/done/5187-lower-namespace-only-multi-section-files.md`

## Notes

## Affected test files

- `reference/typescript/tests/cases/compiler/complexRecursiveCollections.ts`

## Duplicate detection

- `issues/done/5187-lower-namespace-only-multi-section-files.md` owns keeping
  namespace-only/declaration-only multi-section bodies observable enough for
  the next namespace/scope/type diagnostic.
- `issues/open/5292-skip-tsconfig-filename-sections-in-reference-harness.md`
  is unrelated; this file has source/declaration virtual sections, not a
  `tsconfig.json` section.
- Broader type-system issues are later blockers because the current smart triage
  first stops at namespace-only multi-section handling.

## Smart triage

Generated 2026-05-07.

```text
### Smart triage: Triage runtime subset: complexRecursiveCollections

- Issue class: triage-needed
- Feature label: runtime-subset
- Diagnostic: UnsupportedRuntimeSubset / unsupported-feature-boundary
- Path: reference/typescript/tests/cases/compiler/complexRecursiveCollections.ts
```

Diagnostic evidence:

```text
multi-section section `immutable.ts` contains namespace-only declarations;
namespace lowering is not implemented at 222..231
```

AST/resolved evidence:

```text
tokens: ok through type/interface declarations and declare namespace sections
ast: []
resolved: []
```

TypeScript oracle evidence:

```text
typescriptVersion: 6.0.3
diagnostics include recursive type-system/interface compatibility diagnostics
after parsing succeeds.
```

## Completion evidence

Commits:

- superseded by `issues/done/5187-lower-namespace-only-multi-section-files.md`

Validation result:

```text
command: python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/complexRecursiveCollections.ts
result: pass; reproduced namespace-only multi-section runtime-subset boundary
date: 2026-05-07

command: python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/complexRecursiveCollections.ts --detail --no-dashboard-data
result: pass; executed=1, build_pass=1, unsupported=0, blocked=0
date: 2026-05-07
```

Remaining risks:

- The focused batch coverage path currently reports build pass while smart
  triage exposes a namespace-only dump/runtime-subset boundary. Re-triage after
  issue 5187 advances this area.
