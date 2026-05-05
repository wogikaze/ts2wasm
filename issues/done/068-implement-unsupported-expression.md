---
id: 068
title: "Implement unsupported expression types"
type: spike
area: frontend/semantics
class: superseded
priority: P1
depends_on: []
blocks: []
created: 2026-04-26
updated: 2026-04-29
completed: 2026-05-06
status: done
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

- [x] Run the representative `mise run reference-triage -- ...` command
- [x] Confirm whether duplicate candidates already cover this failure
- [x] Split one observable behavior or fixed reference window into child issues
- [x] Carry source context, diagnostic code, AST evidence, and validation commands into each child issue

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
mise run reference-triage -- test262 reference/test262/test/annexB/built-ins/String/prototype/anchor/not-a-constructor.js
mise run reference-coverage -- test262 --path-filter reference/test262/test/annexB/built-ins/String/prototype/anchor/not-a-constructor.js --detail
```

Not run:

- none

## Docs / current-state / issue sync

Final-state docs:

- [x] not affected

Current state:

- [x] updated: `current-state.md` (repo root)

Follow-up issues:

- [x] created: `issues/open/5138-split-reflect-construct-isconstructor-window.md`

## Notes

## Triage findings

2026-05-06:

- The representative path no longer reproduces as generic `unsupported-expression`.
- `mise run reference-triage -- test262 reference/test262/test/annexB/built-ins/String/prototype/anchor/not-a-constructor.js` emitted a smart triage report with:
  - Issue class: `triage-needed`
  - Feature label: `name-resolution`
  - Diagnostic: `UnresolvedName` / `resolver-symbol`
  - Failure: `unresolved name: Reflect at 1548..1555`
  - Visible symbols before failure include `print`, `NaN`, `Infinity`, `$262`, `$ERROR`, `$DONOTEVALUATE`, `assert`, and `isConstructor`.
- The fixed reference window is the Annex B String HTML-method `not-a-constructor` group, which uses test262 `isConstructor.js` and `Reflect.construct`.
- Child issue 5138 owns the remaining decision/implementation split for `Reflect.construct`/`isConstructor` support or an explicit issue-linked diagnostic. This generated bucket should not be selected directly.

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

- issue-state commit closing this generated bucket after child issue 5138 split

Validation result:

```text
command: mise run reference-triage -- test262 reference/test262/test/annexB/built-ins/String/prototype/anchor/not-a-constructor.js
result: emitted smart triage report classifying the failure as UnresolvedName/resolver-symbol for Reflect at 1548..1555; tool session did not close cleanly after emitting the report
date: 2026-05-06

command: python scripts/manager.py update-issue-index --check
result: pass
date: 2026-05-06

command: python scripts/manager.py check-issue-health
result: pass
date: 2026-05-06

command: python scripts/manager.py check-issue-readiness -- --fail-ready-below 80
result: pass
date: 2026-05-06
```

Remaining risks:

- The remaining reference behavior is intentionally open in issue 5138.
