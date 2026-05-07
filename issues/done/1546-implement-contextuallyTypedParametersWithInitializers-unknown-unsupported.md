---
id: 1546
title: "Implement Contextuallytypedparameterswithinitializers Unknown Unsupported"
type: spike
area: frontend/syntax
class: done
priority: P2
depends_on: []
blocks: []
created: 2026-05-01
updated: 2026-05-07
completed: 2026-05-07
---

## Summary

Triage contextuallyTypedParametersWithInitializers-unknown-unsupported across 2 failing reference test cases and split this bucket into implementation-ready child issues.

## Problem

Reference test results show 2 cases fail in directory `contextuallyTypedParametersWithInitializers-unknown-unsupported` with diagnostics: unknown-unsupported. Fresh triage on 2026-05-07 shows both affected files now build successfully.

Problem: contextuallyTypedParametersWithInitializers-unknown-unsupported is stale. The affected files no longer have a compiler blocker in the current runner view.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/contextuallyTypedParametersWithInitializers2.ts
```

Coverage window:

```sh
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/contextuallyTypedParametersWithInitializers2.ts --detail
```

## Desired final state

This generated bucket is either split into implementation-ready child issues or superseded by an existing open/done issue with matching evidence. Do not implement directly from this bucket.

## Scope

In scope:

- [x] Inspect the smart triage report below
- [x] Confirm whether existing open/done issues already cover this bucket
- [x] Close as stale build-pass bucket
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

- [x] Duplicate candidates below are confirmed as no-match for required implementation work
- [x] No child issue needed because both affected files are build-pass
- [x] This issue includes affected paths, diagnostic classification, and parser/TypeScript AST evidence
- [x] Coverage names the exact reference window and remaining sibling blockers

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run
```

Impacted commands:

```sh
mise run reference-coverage -- tsc --limit 4
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/contextuallyTypedParametersWithInitializers2.ts --detail
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/contextuallyTypedParametersWithInitializers2.ts
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

- `reference/typescript/tests/cases/compiler/contextuallyTypedParametersWithInitializers2.ts`
- `reference/typescript/tests/cases/compiler/contextuallyTypedParametersWithInitializers4.ts`

## Duplicate detection

- No implementation child issue is needed for the two affected files.
- `issues/open/1544-implement-contextuallyTypedParametersWithInitializers-arrow-function.md`
  owns the sibling `contextuallyTypedParametersWithInitializers3.ts` current
  name-resolution blocker shown by the broader path-filter coverage run.
- `issues/open/1545-implement-contextuallyTypedParametersWithInitializers-import-export.md`
  owns the sibling `contextuallyTypedParametersWithInitializers1.ts` current
  name-resolution blocker shown by the broader path-filter coverage run.
- `issues/open/694-implement-arrayToLocaleStringES-unknown-unsupported.md` is a
  same-label generated candidate but not a path or syntax match.

## Smart triage

Generated 2026-05-07.

```text
command:
env TS2WASM_BINARY=/home/wogikaze/wgkz/ts2wasm/target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/contextuallyTypedParametersWithInitializers2.ts

result:
BuildPass / pass; ts2wasm build succeeded

compiler evidence:
tokens: ok
ast: ok; ambient generic function `test1`, object literal method arrows,
     and arrow parameters with defaults such as `max = 500` and `arg = num`
     parse successfully
resolved: ok
TypeScript oracle: ok, diagnostics []

command:
env TS2WASM_BINARY=/home/wogikaze/wgkz/ts2wasm/target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/contextuallyTypedParametersWithInitializers4.ts

result:
BuildPass / pass; ts2wasm build succeeded

compiler evidence:
tokens: ok
ast: ok; ambient generic function `test`, rest tuple-ish type syntax in the
     erased signature, and arrow parameter default `max = 3` parse successfully
resolved: ok
TypeScript oracle: ok, diagnostics []
```

## Completion evidence

Fill only when moving to `done/`.

Commits:

- Closed as stale build-pass bucket; no child issue created.

Validation result:

```text
command:
env TS2WASM_BINARY=/home/wogikaze/wgkz/ts2wasm/target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/contextuallyTypedParametersWithInitializers --detail --no-dashboard-data
result:
pass; executed=4, build_pass=2, unsupported=2. Affected files 2 and 4 are build_pass; sibling files 1 and 3 remain UnresolvedName under issues 1545 and 1544.
date:
2026-05-07

command:
env TS2WASM_BINARY=/home/wogikaze/wgkz/ts2wasm/target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/contextuallyTypedParametersWithInitializers2.ts
result:
pass; BuildPass
date:
2026-05-07

command:
env TS2WASM_BINARY=/home/wogikaze/wgkz/ts2wasm/target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/contextuallyTypedParametersWithInitializers4.ts
result:
pass; BuildPass
date:
2026-05-07
```

Remaining risks:

- Sibling files `contextuallyTypedParametersWithInitializers1.ts` and
  `contextuallyTypedParametersWithInitializers3.ts` remain unsupported in the
  broader path-filter coverage window and are tracked by issues 1545 and 1544.
