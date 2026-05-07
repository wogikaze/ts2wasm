---
id: 1222
title: "Implement Classmemberinitializerscoping"
type: spike
area: frontend/syntax
class: triage-needed
priority: P1
depends_on: []
blocks: []
created: 2026-05-01
updated: 2026-05-06
---

## Summary

Closed as stale. Fresh coverage and smart triage show
`classMemberInitializerScoping2.ts` is now build-pass, so there is no remaining
compiler blocker to split from this generated bucket.

## Problem

Reference test results previously showed 1 case failing in directory
`classMemberInitializerScoping` with diagnostics: parser-syntax.

Problem: the representative now builds successfully; the stale bucket should
not remain open as executable work.

## Current failure

Representative reproduction:

```sh
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/classMemberInitializerScoping2.ts
```

Coverage window:

```sh
python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/classMemberInitializerScoping2.ts --detail --no-dashboard-data
```

## Desired final state

This generated bucket is either split into implementation-ready child issues or superseded by an existing open/done issue with matching evidence. Do not implement directly from this bucket.

## Scope

In scope:

- [x] Inspect the smart triage report below
- [x] Confirm there is no remaining compiler blocker to split
- [x] Close the stale generated bucket
- [x] Preserve exact reproduction commands and representative evidence

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

- [x] Duplicate candidates below are confirmed as no-match or this issue is stale
- [x] No child issue needed because fresh triage is BuildPass
- [x] This issue includes representative path, source context, visible symbols, and parser/TypeScript AST evidence
- [x] Completion evidence names the exact fixture/reference path and build-pass result

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run
```

Impacted commands:

```sh
mise run reference-coverage -- tsc --limit 2
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/classMemberInitializerScoping2.ts --detail
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/classMemberInitializerScoping2.ts
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

- `reference/typescript/tests/cases/compiler/classMemberInitializerScoping2.ts`

Representative source:

```ts
const x = 1
class C {
    p = x
    constructor(x: string) { }
}
```

## Duplicate detection

- none needed; fresh triage is BuildPass

## Smart triage

Fresh commands:

```sh
python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/classMemberInitializerScoping2.ts --detail --no-dashboard-data
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/classMemberInitializerScoping2.ts
```

Observed result on 2026-05-06:

```text
coverage: build_pass=1 unsupported=0
triage: BuildPass / ts2wasm build succeeded
tokens: ok; const x, class C, field initializer p = x, typed constructor parameter
AST: ok; Let x, ClassDecl C with constructor
resolved: ok; Let x, ClassDecl C constructor parameter x
TypeScript oracle: TS2301 semantic diagnostic for class field initializer
  referencing constructor parameter `x`
```

## Completion evidence

Fill only when moving to `done/`.

Commits:

- Closed as stale build-pass; see local commit for this issue cleanup.

Validation result:

```text
command: python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/classMemberInitializerScoping2.ts
result: pass; BuildPass
date: 2026-05-06
```

Remaining risks:

- TypeScript still reports semantic diagnostic TS2301, but semantic parity is
  not enabled in this coverage window and no compiler build blocker remains.
