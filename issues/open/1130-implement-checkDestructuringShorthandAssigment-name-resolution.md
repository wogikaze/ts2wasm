---
id: 1130
title: "Implement Checkdestructuringshorthandassigment Name Resolution"
type: spike
area: frontend/resolver
class: done
priority: P1
depends_on: [5224]
blocks: []
created: 2026-05-01
updated: 2026-05-06
completed: 2026-05-06
status: done
---
> **Reopened by false-done audit** (2026-05-07)
> Classification: must-reopen
> Reason: Batch-migrated to issues/done/ without implementation commits.
> Evidence: Empty completion evidence. No feat/fix commit for #1130.

## Summary

Triage checkDestructuringShorthandAssigment-name-resolution across 1 failing reference test cases and split this bucket into implementation-ready child issues.

## Problem

Reference test results show 1 case failing in directory `checkDestructuringShorthandAssigment-name-resolution` with diagnostics: name-resolution. Fresh triage shows the parser turns the parenthesized destructuring assignment `({ b = "5" } = {})` into an assignment whose target name is the whole pattern string, then name resolution reports `UnresolvedName`.

Problem: `checkDestructuringShorthandAssigment.ts` is too broad for direct implementation. Its current observable blocker is now tracked by `issues/done/5224-handle-package-json-virtual-sections-in-multifile-references.md`.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/checkDestructuringShorthandAssigment.ts
```

Coverage window:

```sh
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/checkDestructuringShorthandAssigment.ts --detail
```

## Desired final state

This generated bucket is superseded by `issues/done/5224-handle-package-json-virtual-sections-in-multifile-references.md`. Do not implement directly from this bucket.

## Scope

In scope:

- [x] Inspect the smart triage report below
- [x] Confirm whether existing open/done issues already cover this bucket
- [x] Split the current observable blocker into issue 5224
- [x] Preserve exact reproduction commands and representative AST/diagnostic evidence in this issue and issue 5224

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
- [x] Issue 5224 contains an exact `python scripts/manager.py reference-triage ...` command
- [x] Issue 5224 includes failing path, diagnostic code, source context, visible symbols, and parser/TypeScript AST evidence
- [x] Issue 5224 acceptance names the exact fixture/reference path and diagnostic/stdout change

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
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/checkDestructuringShorthandAssigment.ts --detail
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/checkDestructuringShorthandAssigment.ts
```

Not run:

- `cargo fmt --all --check`; issue cleanup only, no Rust code changed
- `cargo nextest run`; issue cleanup only, no implementation changed

## Docs / current-state / issue sync

Final-state docs:

- [x] not affected

Current state:

- [x] not affected

Follow-up issues:

- [x] created: `issues/done/5224-handle-package-json-virtual-sections-in-multifile-references.md`

## Notes

## Affected test files

- `reference/typescript/tests/cases/compiler/checkDestructuringShorthandAssigment.ts`

## Duplicate detection

- Broad name-resolution buckets are not exact matches because the first blocker is a parser/AST representation problem: the destructuring pattern text is turned into a synthetic identifier before resolver lookup.
- Existing destructuring parser/runtime issues do not cover parenthesized object destructuring assignments in call arguments with shorthand defaults.

## Smart triage

### Smart triage: Triage name resolution: checkDestructuringShorthandAssigment

- Issue class: `triage-needed`
- Feature label: `name-resolution`
- Diagnostic: `UnresolvedName` / `resolver-symbol`
- Path: `reference/typescript/tests/cases/compiler/checkDestructuringShorthandAssigment.ts`

Reproduction:

```sh
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/checkDestructuringShorthandAssigment.ts
```

Coverage reproduction:

```sh
python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/checkDestructuringShorthandAssigment.ts --detail --no-dashboard-data
```

Source context:

```text
function Test({ b = "" } = {}) {}
Test(({ b = "5" } = {}));
```

Current compiler failure:

```text
error: [UnresolvedName] unresolved name: `{b = "5"}` at 174..191
```

Compiler evidence:

- Tokens succeed for the function parameter destructuring default and for the call argument `({ b = "5" } = {})`.
- AST succeeds but represents the call argument as `Assign { name: "{b = \"5\"}", expr: Object {} }`.
- Resolver then treats the pattern string as a name and reports `UnresolvedName`.

TypeScript oracle evidence:

```text
TS18004: No value exists in scope for the shorthand property 'b'. Either declare one or provide an initializer.
```

Resolution:

```text
The current blocker is now tracked by child issue 5224. It is narrower than the generated bucket: parse parenthesized object destructuring assignments in call arguments without synthesizing an identifier from the pattern text.
```

## Completion evidence

Fill only when moving to `done/`.

checkDestructuringShorthandAssigment triage is complete. The actionable
blocker is tracked by child issue 5224.

Commits:

- child issue: `issues/done/5224-handle-package-json-virtual-sections-in-multifile-references.md`

Validation result:

```text
command: python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/checkDestructuringShorthandAssigment.ts --detail --no-dashboard-data
result: pass on the main checkout; 1 executed, current failure is UnresolvedName name-resolution
date: 2026-05-06

command: python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/checkDestructuringShorthandAssigment.ts
result: pass; reproduced UnresolvedName for synthetic destructuring pattern name and split to issue 5224
date: 2026-05-06
```

Remaining risks:

- none
