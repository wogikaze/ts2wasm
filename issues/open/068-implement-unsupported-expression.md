---
id: 068
title: "Implement unsupported expression types"
type: spike
area: frontend/semantics
class: blocked
priority: P1
depends_on: [5005]
blocks: []
created: 2026-04-26
updated: 2026-04-29
---

## Summary

Triage the generated reference bucket `Implement unsupported expression types` before implementation. This issue records a failing reference case and must be split or superseded before any code change starts.

## Problem

Reference test results show 23 cases fail with unsupported-expression diagnostic. The compiler cannot handle these syntax/semantics, preventing compilation of code in this category.

Problem: generated reference bucket `Implement unsupported expression types` fails with `unsupported-expression` and needs smart-triage evidence before implementation starts.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- test262 reference/test262/test/annexB/built-ins/String/prototype/anchor/not-a-constructor.js
```

Narrow coverage reproduction:

```sh
mise run reference-coverage -- test262 --path-filter reference/test262/test/annexB/built-ins/String/prototype/anchor/not-a-constructor.js --detail
```

Representative path: `reference/test262/test/annexB/built-ins/String/prototype/anchor/not-a-constructor.js`
Feature label: `unsupported-expression`

## Desired final state

This generated bucket is not used as a direct implementation work order. It is either superseded by an existing open/done issue, closed as a duplicate, or split into implementation-ready child issues that contain exact reproduction evidence and measurable acceptance criteria.

## Scope

In scope:

- [ ] Run the representative `mise run reference-triage -- ...` command
- [ ] Confirm whether duplicate candidates already cover this failure
- [ ] Split one observable behavior or fixed reference window into child issues
- [ ] Carry source context, diagnostic code, AST evidence, and validation commands into each child issue

Out of scope:

- Direct implementation from this generated bucket
- Broad fixes that mix unrelated parser, resolver, runtime, and API failures

## Affected paths

Expected:

- `crates/frontend/src/`
- `crates/cli/src/`
- `fixtures/`
- `scripts/run/reference-triage.py`

Do not touch:

- unrelated runtime/backend code unless `reference-triage` proves the failure is not frontend-owned

## Acceptance criteria

- [ ] Duplicate candidates are confirmed as no-match, duplicate, or superseding issue
- [ ] At least one child issue contains an exact `mise run reference-triage -- ...` command
- [ ] Child issue includes failing path, diagnostic code, source context, visible symbols, and AST evidence
- [ ] Child issue acceptance names the exact fixture/reference path and diagnostic/stdout change

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run
```

Impacted commands:

```sh
mise run reference-triage -- test262 reference/test262/test/annexB/built-ins/String/prototype/anchor/not-a-constructor.js
mise run reference-coverage -- test262 --path-filter reference/test262/test/annexB/built-ins/String/prototype/anchor/not-a-constructor.js --detail
```

Not run:

- none

## Docs / current-state / issue sync

Final-state docs:

- [ ] not affected

Current state:

- [ ] updated: `current-state.md` (repo root)

Follow-up issues:

- [ ] none

## Notes

## Affected test files

- `reference/test262/test/annexB/built-ins/String/prototype/anchor/not-a-constructor.js`
- `reference/test262/test/annexB/built-ins/String/prototype/big/not-a-constructor.js`
- `reference/test262/test/annexB/built-ins/String/prototype/blink/not-a-constructor.js`
- `reference/test262/test/annexB/built-ins/String/prototype/bold/not-a-constructor.js`
- `reference/test262/test/annexB/built-ins/String/prototype/fixed/not-a-constructor.js`
- `reference/test262/test/annexB/built-ins/String/prototype/fontcolor/not-a-constructor.js`
- `reference/test262/test/annexB/built-ins/String/prototype/fontsize/not-a-constructor.js`
- `reference/test262/test/annexB/built-ins/String/prototype/italics/not-a-constructor.js`
- `reference/test262/test/annexB/built-ins/String/prototype/link/not-a-constructor.js`
- `reference/test262/test/annexB/built-ins/String/prototype/small/not-a-constructor.js`
- ... and 13 more files

## Duplicate detection

- none found by path/title/feature scan

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

- none
