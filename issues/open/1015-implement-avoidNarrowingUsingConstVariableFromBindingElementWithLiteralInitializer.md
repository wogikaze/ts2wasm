---
id: 1015
title: "Implement Avoidnarrowingusingconstvariablefrombindingelementwithliteralinitializer"
type: spike
area: frontend/syntax
class: superseded
priority: P1
depends_on: []
blocks: []
created: 2026-05-01
updated: 2026-05-06
completed: 2026-05-06
status: done
---

## Summary

Triage avoidNarrowingUsingConstVariableFromBindingElementWithLiteralInitializer across 1 failing reference test case and split this generated bucket into a smaller implementation-ready child issue.

## Problem

Reference test results show 1 case fails in directory `avoidNarrowingUsingConstVariableFromBindingElementWithLiteralInitializer` with diagnostics: import-export. Fresh triage shows the specific current blocker is the entry-module static export boundary for `export function test(...)`.

Problem: avoidNarrowingUsingConstVariableFromBindingElementWithLiteralInitializer has 1 reference failure that is now tracked by child issue 5144.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/avoidNarrowingUsingConstVariableFromBindingElementWithLiteralInitializer.ts
```

Coverage window:

```sh
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/avoidNarrowingUsingConstVariableFromBindingElementWithLiteralInitializer.ts --detail
```

## Desired final state

This generated bucket is superseded by `issues/done/5144-support-entry-export-function-declarations.md`.

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
- [x] Child issue 5144 contains an exact `mise run reference-triage -- ...` command
- [x] Child issue includes failing path, diagnostic code, source context, visible symbols, and parser/TypeScript AST evidence
- [x] Child issue acceptance names the exact fixture/reference path and diagnostic/stdout change

## Validation

Required commands:

```sh
python scripts/manager.py update-issue-index --check
python scripts/manager.py check-issue-health
python scripts/manager.py check-issue-readiness -- --fail-ready-below 80
git diff --check
```

Impacted commands:

```sh
mise run reference-coverage -- tsc --limit 2
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/avoidNarrowingUsingConstVariableFromBindingElementWithLiteralInitializer.ts --detail
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/avoidNarrowingUsingConstVariableFromBindingElementWithLiteralInitializer.ts
```

Not run:

- `cargo fmt --all --check`; issue split only, no Rust code changed
- `cargo nextest run`; issue split only, no implementation changed

## Docs / current-state / issue sync

Final-state docs:

- [x] not affected

Current state:

- [x] not affected

Follow-up issues:

- [x] created: `issues/done/5144-support-entry-export-function-declarations.md`

## Notes

## Affected test files

- `reference/typescript/tests/cases/compiler/avoidNarrowingUsingConstVariableFromBindingElementWithLiteralInitializer.ts`

## Duplicate detection

- none found by path/title/feature scan

## Smart triage

### Smart triage: Triage import export: avoidNarrowingUsingConstVariableFromBindingElementWithLiteralInitializer

- Issue class: `triage-needed`
- Feature label: `import-export`
- Diagnostic: `UnsupportedModule` / `unsupported-feature-boundary`
- Path: `reference/typescript/tests/cases/compiler/avoidNarrowingUsingConstVariableFromBindingElementWithLiteralInitializer.ts`

Reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/avoidNarrowingUsingConstVariableFromBindingElementWithLiteralInitializer.ts
```

Failure location:

```json
{
  "code": "UnsupportedModule",
  "message": "issue-5005: entry module `export test` uses a declaration form outside the current static export slice; only export const and export default are supported at 129..236",
  "span_start": 129,
  "span_end": 236,
  "line": 7,
  "column": 8,
  "feature_label": "import-export",
  "error_type": "unsupported-feature-boundary"
}
```

Source context:

```text
 4 |
 5 | declare const foo: ["a", string, number] | ["b", string, boolean];
 6 |
 7 | export function test(arg: { index?: number }) {
 8 |   const { index = 0 } = arg;
 9 |
10 |   if (foo[index] === "a") {
```

Compiler evidence:

```text
AST: ExportDecl(Function { name: "test", ... })
resolved dump after the module boundary reports unresolved ambient `foo`
TypeScript oracle: ok, no diagnostics; function `test` has type `void`
```

## Completion evidence

Commits:

- pending local commit

Validation result:

```text
command:
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/avoidNarrowingUsingConstVariableFromBindingElementWithLiteralInitializer.ts
result:
pass; emitted UnsupportedModule / import-export report for entry-module `export function`; split to issue 5144
date:
2026-05-06
```

Remaining risks:

- none

## False-done audit

Date: 2026-05-06

Classification: truly-done.

Audit result: retained in `issues/done/`. This issue has repo-local close evidence
(completion evidence with validation commands) or proper superseded classification
with child issues in `issues/open/`. The acceptance criteria documented in the issue
are satisfied by the repo-local evidence cited in the completion evidence section.

Future-work tracking: no untracked future-work item was identified in this issue
during this metadata/evidence audit.

