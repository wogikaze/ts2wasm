---
id: 1012
title: "Implement Autonumberinginenums"
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

Triage autonumberingInEnums across 1 generated reference bucket entry and close it if current evidence shows no implementation blocker.

## Problem

Older reference test results showed 1 case failing in directory `autonumberingInEnums` with diagnostics: parser-syntax. Fresh smart triage on 2026-05-06 shows the case now builds successfully, so this generated bucket is stale.

Problem: autonumberingInEnums no longer has a current compiler blocker; no child implementation issue is needed for this generated bucket.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/autonumberingInEnums.ts
```

Coverage window:

```sh
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/autonumberingInEnums.ts --detail
```

## Desired final state

This generated bucket is closed as stale because the only affected reference case currently reports `BuildPass` / `pass`.

## Scope

In scope:

- [x] Inspect the smart triage report below
- [x] Confirm whether existing open/done issues already cover this bucket
- [x] No child issue created because fresh triage found no current compiler blocker
- [x] Preserve exact reproduction commands and representative diagnostic evidence in this closed issue

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
- [x] This closed issue contains an exact `mise run reference-triage -- ...` command
- [x] This closed issue includes the reference path, diagnostic code, and source context
- [x] Completion evidence records the exact fixture/reference path and diagnostic result

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run
```

Impacted commands:

```sh
mise run reference-coverage -- tsc --limit 2
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/autonumberingInEnums.ts --detail
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/autonumberingInEnums.ts
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

## Affected test files

- `reference/typescript/tests/cases/compiler/autonumberingInEnums.ts`

## Duplicate detection

- none found by path/title/feature scan

## Smart triage

### Smart triage: Build pass: autonumberingInEnums

- Issue class: `none`
- Feature label: `build-pass`
- Diagnostic: `BuildPass` / `pass`
- Path: `reference/typescript/tests/cases/compiler/autonumberingInEnums.ts`

Reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/autonumberingInEnums.ts
```

Source overview:

```json
{
  "suite": "tsc",
  "bytes": 81,
  "lines": 8,
  "extension": ".ts",
  "first_code_line": "enum Foo {"
}
```

Failure location:

```json
{
  "code": "BuildPass",
  "message": "ts2wasm build succeeded",
  "span_start": null,
  "span_end": null,
  "line": null,
  "column": null,
  "feature_label": "build-pass",
  "error_type": "pass"
}
```

Source context:

```text
// @target: es2015
enum Foo {
    a = 1
}

enum Foo {
    b // should work fine
}
```

Duplicate candidates:

```json
[
  {
    "state": "open",
    "path": "issues/open/1012-implement-autonumberingInEnums.md",
    "title": "Implement Autonumberinginenums",
    "reason": "same reference path, title overlap"
  }
]
```

Error-specific suggestions:

- No compiler blocker was found by the build step; use reference-coverage for semantic parity evidence.

## Completion evidence

Commits:

- pending local commit

Validation result:

```text
command:
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/autonumberingInEnums.ts
result:
emitted BuildPass / pass smart-triage report for the only affected reference path; no compiler blocker found
date:
2026-05-06
```

Remaining risks:

- none
