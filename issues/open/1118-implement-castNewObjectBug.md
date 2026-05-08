---
id: 1118
title: "Implement Castnewobjectbug"
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
> **Reopened by false-done audit** (2026-05-07)
> Classification: must-reopen
> Reason: Batch-migrated to issues/open/ without implementation commits.
> Evidence: Empty completion evidence. No feat/fix commit for #1118.

## Summary

Triage castNewObjectBug across 1 failing reference test cases and split this bucket into implementation-ready child issues.

## Problem

Original reference test results showed 1 case failing in directory `castNewObjectBug` with diagnostics: object-literal. Fresh focused coverage now reports `build_pass`, so the original blocker is no longer reproducible.

Problem: castNewObjectBug had 1 generated reference failure and needed smart-triage evidence before implementation starts. Current evidence shows no compiler blocker remains for this path.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/castNewObjectBug.ts
```

Coverage window:

```sh
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/castNewObjectBug.ts --detail
```

## Desired final state

This generated bucket is closed because its sole reference path now builds successfully.

## Scope

In scope:

- [x] Inspect the smart triage report below
- [x] Confirm whether existing open/done issues already cover this bucket
- [x] Confirm the generated blocker is no longer reproducible
- [x] Preserve exact reproduction commands and representative AST/diagnostic evidence in this issue

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

- [x] Duplicate candidates below are confirmed as no-match or this issue is superseded by build-pass evidence
- [x] The issue contains an exact `mise run reference-triage -- ...` command
- [x] The issue includes reference path, diagnostic code, source context, visible symbols, and parser/TypeScript AST evidence
- [x] Acceptance names the exact fixture/reference path and diagnostic/stdout change

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
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/castNewObjectBug.ts --detail
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/castNewObjectBug.ts
```

Not run:

- `cargo fmt --all --check`; issue metadata only, no Rust code changed
- `cargo nextest run`; issue metadata only, no implementation changed

## Docs / current-state / issue sync

Final-state docs:

- [x] not affected

Current state:

- [x] not affected

Follow-up issues:

- [x] none

## Notes

## Affected test files

- `reference/typescript/tests/cases/compiler/castNewObjectBug.ts`

## Duplicate detection

- none found by path/title/feature scan

## Smart triage

Generated on 2026-05-06:

- command: `python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/castNewObjectBug.ts`
- issue class: `none`
- feature label: `build-pass`
- diagnostic: `BuildPass` / `pass`
- result: `ts2wasm build succeeded`
- source context:

```text
// @target: es2015
interface Foo { }
var xx = <Foo> new Object();
```

Compiler evidence:

```text
tokens: ok; includes `<Foo> new Object()`
AST: Let xx = New(Ident("Object"), args=[])
resolved: Let("xx", New { class_name: "Object", args: [] })
TypeScript oracle: ok, no diagnostics; binding `xx` has type `Foo`
```

Focused coverage:

```text
command: python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/castNewObjectBug.ts --detail --no-dashboard-data
result: build_pass=1, semantic_pass=0, fail=0, unsupported=0, blocked=0
date: 2026-05-06
```

## Completion evidence

Closed as a generated triage bucket whose only affected reference path now
builds successfully.

Commits:

- this closure commit

Validation result:

```text
command: python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/castNewObjectBug.ts
result: pass; BuildPass / build-pass with AST and resolved evidence for `var xx = <Foo> new Object();`
date: 2026-05-06
```

Remaining risks:

- none
