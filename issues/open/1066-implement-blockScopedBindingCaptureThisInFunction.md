---
id: 1066
title: "Implement Blockscopedbindingcapturethisinfunction"
type: spike
area: reference/triage
class: superseded
priority: P2
depends_on: [5179]
blocks: []
created: 2026-05-01
updated: 2026-05-06
status: done
completed: 2026-05-06
---

## Summary

Triage blockScopedBindingCaptureThisInFunction across 1 failing reference test cases and split this bucket into implementation-ready child issues.

## Problem

Reference test results show 1 cases fail in directory `blockScopedBindingCaptureThisInFunction` with diagnostics: runtime-subset. The compiler cannot handle these syntax/semantics, preventing compilation of code in this category.

Problem: blockScopedBindingCaptureThisInFunction has 1 reference failures and needs smart-triage evidence before implementation starts.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/blockScopedBindingCaptureThisInFunction.ts
```

Coverage window:

```sh
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/blockScopedBindingCaptureThisInFunction.ts --detail
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

- `issues/open/`
- `scripts/run/reference-triage.py`
- `fixtures/`

Do not touch:

- implementation code until the triage report assigns a concrete frontend/runtime/backend owner

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
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/blockScopedBindingCaptureThisInFunction.ts --detail
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/blockScopedBindingCaptureThisInFunction.ts
```

Not run:

- none

## Docs / current-state / issue sync

Final-state docs:

- [x] not affected

Current state:

- [x] not affected

Follow-up issues:

- [x] added: `issues/open/5179-report-implicit-this-before-closure-runtime-guard.md`

## Notes

## Affected test files

- `reference/typescript/tests/cases/compiler/blockScopedBindingCaptureThisInFunction.ts`

## Duplicate detection

- Generic `runtime-subset` buckets are not matches; they share only the generated feature label.
- `issues/open/062e-function-closures.md` is related but already completed a narrower closure slice and explicitly left broader `this`/`arguments` closure support out of scope.
- `issues/open/597-implement-allowJsClassThisTypeCrash.md` has the same `issue-062e` diagnostic in a different reference file, but remains a generated triage bucket rather than an implementation-ready child.

## Smart triage

Generated on 2026-05-06.

- Path: `reference/typescript/tests/cases/compiler/blockScopedBindingCaptureThisInFunction.ts`
- Diagnostic: `UnsupportedRuntimeSubset` / `unsupported-feature-boundary`
- Current compiler message: `issue-062e: nested function `` closures with `this` or `arguments` are not supported in this slice`
- Source context: `() => function () { for (let someKey in {}) { this.helloWorld(); () => someKey; } };`
- Visible symbols before failure: binding `someKey`
- Compiler evidence: token dump includes `This`, member `helloWorld`, `for-in` binding `someKey`, and nested arrow `() => someKey`; AST construction succeeds; resolved/lowered pipeline fails at `issue-062e`.
- TypeScript oracle: `TS2683: 'this' implicitly has type 'any' because it does not have a type annotation.` at the `this` token.
- Superseding child: `issues/open/5179-report-implicit-this-before-closure-runtime-guard.md`

## Completion evidence

Fill only when moving to `done/`.

Commits:

- `pending`

Validation result:

```text
command: python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/blockScopedBindingCaptureThisInFunction.ts
result: pass; current blocker identified as implicit-this diagnostic hidden by issue-062e runtime guard, split to issue 5179
date: 2026-05-06
```

Remaining risks:

- Later triage may expose loop-capture or closure runtime work after issue 5179 handles the earlier implicit-`this` diagnostic.

## False-done audit

Date: 2026-05-06

Classification: truly-done.

Audit result: retained in `issues/open/`. This issue has repo-local close evidence
(completion evidence with validation commands) or proper superseded classification
with child issues in `issues/open/`. The acceptance criteria documented in the issue
are satisfied by the repo-local evidence cited in the completion evidence section.

Future-work tracking: no untracked future-work item was identified in this issue
during this metadata/evidence audit.

