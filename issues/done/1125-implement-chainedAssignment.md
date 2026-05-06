---
id: 1125
title: "Implement Chainedassignment"
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

Triage chainedAssignment across 1 generated reference bucket entry and close it if current evidence shows no implementation blocker.

## Problem

Older reference test results showed 1 case failing in directory `chainedAssignment` with diagnostics: parser-syntax. Fresh focused coverage and smart triage on 2026-05-06 show the case now builds successfully, so this generated bucket is stale.

Problem: chainedAssignment no longer has a current compiler blocker; no child implementation issue is needed for this generated bucket.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/chainedAssignment3.ts
```

Coverage window:

```sh
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/chainedAssignment3.ts --detail
```

## Desired final state

This generated bucket is closed as stale because the only affected reference case currently reports `BuildPass` / `pass`.

## Scope

In scope:

- [x] Inspect the smart triage report below
- [x] Confirm whether existing open/done issues already cover this bucket
- [x] No child issue created because fresh triage found no current compiler blocker
- [x] Preserve exact reproduction commands and representative AST/diagnostic evidence in this closed issue

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
- [x] This closed issue contains an exact `mise run reference-triage -- ...` command
- [x] This closed issue includes the reference path, diagnostic code, source context, visible symbols, and parser/TypeScript AST evidence
- [x] Completion evidence records the exact fixture/reference path and diagnostic result

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
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/chainedAssignment3.ts --detail
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/chainedAssignment3.ts
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

- [x] none

## Notes

## Affected test files

- `reference/typescript/tests/cases/compiler/chainedAssignment3.ts`

## Duplicate detection

- none found by path/title/feature scan

## Smart triage

### Smart triage: Build pass: chainedAssignment3

- Issue class: `none`
- Feature label: `build-pass`
- Diagnostic: `BuildPass` / `pass`
- Path: `reference/typescript/tests/cases/compiler/chainedAssignment3.ts`

Reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/chainedAssignment3.ts
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
class A {
    id: number;
}

class B extends A {
    value: string;
}
```

Compiler evidence:

```text
tokens: ok
AST: chained `a = b = ...`, `b = a = ...`, and `a.id = b.value = null` are represented
resolved: ok; chained assignments and property assignment are preserved in resolved IR
TypeScript oracle: reports type diagnostics including TS2322 and TS2741, but no parser blocker
```

Focused coverage:

```text
command: python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/chainedAssignment3.ts --detail --no-dashboard-data
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
command: python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/chainedAssignment3.ts
result: pass; emitted BuildPass / pass smart-triage report for the only affected reference path
date: 2026-05-06
```

Remaining risks:

- Semantic coverage is not enabled for this path; TypeScript type diagnostics remain outside this generated compiler-blocker closure.
