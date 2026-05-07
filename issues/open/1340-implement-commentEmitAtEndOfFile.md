---
id: 1340
title: "Implement Commentemitatendoffile"
type: spike
area: frontend/syntax
class: triage-needed
priority: P1
depends_on: []
blocks: []
created: 2026-05-01
updated: 2026-05-06
---
> **Reopened by false-done audit** (2026-05-07)
> Classification: must-reopen
> Reason: Batch-migrated to issues/done/ without implementation commits.
> Evidence: Empty completion evidence. No feat/fix commit for #1340.

## Summary

Closed as stale. Fresh focused coverage and triage show
`commentEmitAtEndOfFile1.ts` now builds successfully, so there is no current
compiler blocker to split.

## Problem

Reference test results originally showed 1 case failing in directory
`commentEmitAtEndOfFile` with diagnostics: parser-syntax. Fresh coverage now
reports `build_pass=1`.

Problem: none remaining for this generated bucket; the representative file
builds successfully.

## Current failure

Representative reproduction:

```sh
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/commentEmitAtEndOfFile1.ts
```

Coverage window:

```sh
python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/commentEmitAtEndOfFile1.ts --detail --no-dashboard-data
```

## Desired final state

This generated bucket is closed as stale because the representative reference
now builds successfully. Do not implement directly from this bucket.

## Scope

In scope:

- [x] Inspect the smart triage report below
- [x] Confirm no current blocker remains
- [x] Close this stale generated bucket
- [x] Preserve exact reproduction commands and representative build-pass evidence

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

- [x] Duplicate candidates below are confirmed; no child issue needed because the path is build-pass
- [x] This issue contains exact `python scripts/manager.py reference-triage ...` command
- [x] This issue includes failing path, build-pass code, source context, visible symbols, and AST evidence
- [x] Completion evidence names the exact reference path and build-pass result

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run
```

Impacted commands:

```sh
python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/commentEmitAtEndOfFile1.ts --detail --no-dashboard-data
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/commentEmitAtEndOfFile1.ts
```

Not run:

- `cargo fmt --all --check` (not run; issue metadata only)
- `cargo nextest run` (not run; issue metadata only)

## Docs / current-state / issue sync

Final-state docs:

- [x] not affected

Current state:

- [x] not affected

Follow-up issues:

- [x] none

## Notes

## Affected test files

- `reference/typescript/tests/cases/compiler/commentEmitAtEndOfFile1.ts`

Source context:

```ts
// test
var f = ''
// test #2
namespace foo {
        function bar() { }
}
// test #3
namespace empty {
}
// test #4
```

## Duplicate detection

- Fresh triage shows the only duplicate candidate is this issue's own path.
- No child issue is needed because the representative is build-pass.

## Smart triage

Fresh commands:

```sh
python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/commentEmitAtEndOfFile1.ts --detail --no-dashboard-data
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/commentEmitAtEndOfFile1.ts
```

Observed result on 2026-05-06:

```text
coverage: build_pass=1
unsupported=0
blocked=0

Diagnostic: BuildPass
Message: ts2wasm build succeeded
tokens: ok
AST: ok
resolved: ok
TypeScript oracle: ok, no diagnostics
```

## Completion evidence

Fill only when moving to `done/`.

Commits:

- Closed as stale build-pass bucket; see local commit for this issue cleanup.

Validation result:

```text
command: python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/commentEmitAtEndOfFile1.ts
result: pass; BuildPass, no current blocker to split
date: 2026-05-06
```

Remaining risks:

- none
