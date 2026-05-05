---
id: 085
title: "Implement Abstractinterfaceidentifiername"
type: spike
area: frontend/syntax
class: docs-ready
priority: P1
depends_on: []
blocks: []
created: 2026-04-26
updated: 2026-04-29
---

## Summary

Triage the generated reference bucket `Implement Abstractinterfaceidentifiername` before implementation. This issue records a failing reference case and must be split or superseded before any code change starts.

## Problem

Reference test results show 1 cases fail in directory `abstractInterfaceIdentifierName` with diagnostics: parser-syntax. The compiler cannot handle these syntax/semantics, preventing compilation of code in this category.

Problem: generated reference bucket `Implement Abstractinterfaceidentifiername` fails with `parser-syntax` and needs smart-triage evidence before implementation starts.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/abstractInterfaceIdentifierName.ts
```

Narrow coverage reproduction:

```sh
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/abstractInterfaceIdentifierName.ts --detail
```

Representative path: `reference/typescript/tests/cases/compiler/abstractInterfaceIdentifierName.ts`
Feature label: `parser-syntax`

## Desired final state

This generated bucket is not used as a direct implementation work order. It is either superseded by an existing open/done issue, closed as a duplicate, or split into implementation-ready child issues that contain exact reproduction evidence and measurable acceptance criteria.

## Scope

In scope:

- [x] Run the representative `mise run reference-triage -- ...` command
- [x] Confirm whether duplicate candidates already cover this failure
- [x] Split one observable behavior or fixed reference window into child issues
- [x] Carry source context, diagnostic code, AST evidence, and validation commands into each child issue

Out of scope:

- Direct implementation from this generated bucket
- Broad fixes that mix unrelated parser, resolver, runtime, and API failures

## Affected paths

Expected:

- `issues/open/`
- `scripts/run/reference-triage.py`
- `crates/frontend/src/`
- `crates/cli/src/`
- `fixtures/`

Do not touch:

- unrelated runtime/backend files unless `reference-triage` proves the failure is not parser/frontend

## Acceptance criteria

- [x] Duplicate candidates are confirmed as no-match, duplicate, or superseding issue
- [x] At least one child issue contains an exact `mise run reference-triage -- ...` command
- [x] Child issue includes failing path, diagnostic code, source context, visible symbols, and AST evidence
- [x] Child issue acceptance names the exact fixture/reference path and diagnostic/stdout change

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run
```

Impacted commands:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/abstractInterfaceIdentifierName.ts
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/abstractInterfaceIdentifierName.ts --detail
```

Not run:

- none

## Docs / current-state / issue sync

Final-state docs:

- [x] not affected

Current state:

- [x] updated: `current-state.md` (repo root)

Follow-up issues:

- [x] none

## Notes

## Affected test files

- `reference/typescript/tests/cases/compiler/abstractInterfaceIdentifierName.ts`

## Duplicate detection

- none found by path/title/feature scan

## Completion evidence

Closed by generated-issue regeneration on 2026-04-29: the current `tsc --limit 200 --detail` window no longer emits this reference bucket.

Commits:

- pending

Validation result:

```text
command: mise run reference-coverage -- tsc --limit 200 --detail
result: pass; bucket absent from regenerated detail output
date: 2026-04-29
```

Remaining risks:

- The broader tsc corpus may still contain related failures outside the current regenerated window.

## False-done audit

Date: 2026-05-05

Classification: truly-done.

Audit result: retained in `issues/done/`. The issue has repo-local completion evidence, is not marked `blocked` or `triage-needed`, and `python scripts/manager.py check issues` validates the resulting issue graph.

Evidence files:
- `issues/done/085-implement-abstractInterfaceIdentifierName.md`
- `issues/index.md` after regeneration

Future-work tracking: no untracked future-work item was identified in this issue during this metadata/evidence audit.
