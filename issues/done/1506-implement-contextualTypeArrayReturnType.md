---
id: 1506
title: "Implement Contextualtypearrayreturntype"
type: spike
area: frontend/syntax
class: done
priority: P1
depends_on: []
blocks: []
created: 2026-05-01
updated: 2026-05-07
---
> **Reopened by false-done audit** (2026-05-07)
> Classification: must-reopen
> Reason: Batch-migrated to issues/done/ without implementation commits.
> Evidence: Empty completion evidence. No feat/fix commit for #1506.

## Summary

Triage contextualTypeArrayReturnType across 1 failing reference test cases and split this bucket into implementation-ready child issues.

## Problem

Reference test results show 1 cases fail in directory `contextualTypeArrayReturnType` with diagnostics: parser-syntax. The compiler cannot handle these syntax/semantics, preventing compilation of code in this category.

Problem: contextualTypeArrayReturnType has 1 reference failures and needs smart-triage evidence before implementation starts.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/contextualTypeArrayReturnType.ts
```

Coverage window:

```sh
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/contextualTypeArrayReturnType.ts --detail
```

## Desired final state

This generated bucket is either split into implementation-ready child issues or superseded by an existing open/done issue with matching evidence. Do not implement directly from this bucket.

## Scope

In scope:

- [x] Inspect the smart triage report below
- [x] Confirm whether existing open/done issues already cover this bucket
- [x] Close as stale because the representative path now build-passes
- [x] Preserve exact reproduction commands and representative AST/diagnostic evidence

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

- [x] Duplicate candidates below are confirmed; no child issue is needed
- [x] No child issue created because the compiler now reports build_pass
- [x] This issue includes path, diagnostic status, source context, visible symbols, and parser/TypeScript AST evidence
- [x] Completion evidence names the exact reference path and build-pass result

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run
```

Impacted commands:

```sh
mise run reference-coverage -- tsc --limit 2
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/contextualTypeArrayReturnType.ts --detail
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/contextualTypeArrayReturnType.ts
```

Not run:

- none

## Docs / current-state / issue sync

Final-state docs:

- [x] not affected

Current state:

- [x] not affected

Follow-up issues:

- [x] none; current compiler build has no parser blocker on this path

## Notes

## Affected test files

- `reference/typescript/tests/cases/compiler/contextualTypeArrayReturnType.ts`

## Duplicate detection

- none found by path/title/feature scan

## Smart triage

Fresh triage on 2026-05-07 shows the original parser-syntax blocker is stale.
The focused reference path now build-passes.

Current status:

```text
BuildPass: ts2wasm build succeeded
```

Coverage result:

```text
executed=1
build_pass=1
unsupported=0
blocked=0
```

Compiler evidence:

```text
tokens: ok through interfaces, index signature, arrow property, array return, and object literal
ast: ok; Let style with Object.initialLeftPageTransforms ArrowFn returning Array[{ ry: null }]
resolved: ok; style binding resolves with arrow body and array return
TypeScript oracle: reports later TS2322 because null is not assignable to Transform3D
```

No child issue was created because there is no current compiler blocker in the
build path. The TypeScript oracle diagnostic is semantic parity work and should
be triaged separately if semantic checking for this window is selected.

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

- Semantic parity is not enabled for this focused coverage window. TypeScript's
  TS2322 contextual array return diagnostic remains outside this issue cleanup.
