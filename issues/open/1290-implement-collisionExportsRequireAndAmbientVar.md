---
id: 1290
title: "Implement Collisionexportsrequireandambientvar"
type: spike
area: frontend/syntax
class: done
priority: P1
depends_on: []
blocks: []
created: 2026-05-01
updated: 2026-05-07
completed: 2026-05-07
status: done
---
> **Reopened by false-done audit** (2026-05-07)
> Classification: must-reopen
> Reason: Batch-migrated to issues/done/ without implementation commits.
> Evidence: Empty completion evidence. No feat/fix commit for #1290.

## Summary

Closed as stale. Fresh triage and focused coverage show
`reference/typescript/tests/cases/compiler/collisionExportsRequireAndAmbientVar.ts`
now build-passes, so there is no current import/export compiler blocker to
split into a child issue.

## Problem

Reference test results previously showed 1 case failing in directory
`collisionExportsRequireAndAmbientVar` with diagnostics: import-export. Fresh
triage shows the current compiler builds the representative path.

Problem: the generated import/export bucket is stale. TypeScript still reports
TS2395 merged declaration export/local diagnostics for ambient vars named
`exports` and `require`, but semantic parity is not enabled for this coverage
window and those diagnostics are not the generated compiler build blocker.

## Current failure

Representative reproduction:

```sh
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/collisionExportsRequireAndAmbientVar.ts
```

Coverage window:

```sh
python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/collisionExportsRequireAndAmbientVar.ts --detail --no-dashboard-data
```

## Desired final state

This generated bucket is closed as stale because the representative path now
reports `build_pass`. Do not implement directly from this bucket.

## Scope

In scope:

- [x] Inspect the smart triage report below
- [x] Confirm the representative case is now build-pass with no active import/export blocker
- [x] Close as stale build-pass instead of creating a child issue
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

- [x] Duplicate candidates below are confirmed; no separate issue is needed
- [x] No child issue needed because the representative case now build-passes
- [x] This issue includes path, diagnostic status, source context, visible symbols, and parser/TypeScript AST evidence
- [x] Completion evidence names the exact reference path and build-pass result

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
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/collisionExportsRequireAndAmbientVar.ts --detail
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/collisionExportsRequireAndAmbientVar.ts
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

- [x] none; current compiler build has no import/export blocker on this path

## Notes

## Affected test files

- `reference/typescript/tests/cases/compiler/collisionExportsRequireAndAmbientVar.ts`

## Duplicate detection

No exact implementation child is created because the original import/export
blocker no longer reproduces. Fresh triage only found this generated bucket as
a same-path duplicate candidate.

Resolution:

```text
The original import/export blocker is stale. The reference window now reports build_pass with semantic checking disabled, so no implementation-ready blocker is split from this generated bucket.
```

## Smart triage

### Smart triage: Build pass: collisionExportsRequireAndAmbientVar

- Issue class: `none`
- Feature label: `build-pass`
- Diagnostic: `BuildPass` / `pass`
- Path: `reference/typescript/tests/cases/compiler/collisionExportsRequireAndAmbientVar.ts`

Reproduction:

```sh
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/collisionExportsRequireAndAmbientVar.ts
```

Coverage reproduction:

```sh
python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/collisionExportsRequireAndAmbientVar.ts --detail --no-dashboard-data
```

Coverage result:

```text
executed=1
build_pass=1
semantic_pass=0
unsupported=0
blocked=0
unsupported_features=
semantic_enabled=0
```

Source context:

```ts
export declare var exports: number;
export declare var require: string;
declare namespace m1 {
    var exports: string;
    var require: number;
}
namespace m2 {
    export declare var exports: number;
    export declare var require: string;
    var a = 10;
}
```

Compiler evidence:

```text
tokens: ok; includes export declare var exports/require, declare namespace m1, namespace m2/m4, and binding a
ast: ok; []
resolved: ok; []
visible symbols: bindings exports/require in module/global and namespace scopes plus binding a initialized to 10
```

TypeScript oracle evidence:

```text
TS2395: Individual declarations in merged declaration 'exports' must be all exported or all local.
TS2395: Individual declarations in merged declaration 'require' must be all exported or all local.
```

## Completion evidence

Fill only when moving to `done/`.

Commits:

- Closed as stale; see local commit for this issue cleanup.

Validation result:

```text
command: python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/collisionExportsRequireAndAmbientVar.ts
result: pass on the main checkout with TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm; build succeeded and original import/export blocker is stale
date: 2026-05-07

command: python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/collisionExportsRequireAndAmbientVar.ts --detail --no-dashboard-data
result: pass on the main checkout with TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm; executed=1, build_pass=1, unsupported=0
date: 2026-05-07
```

Remaining risks:

- TypeScript still reports TS2395 diagnostics; those are future semantic
  parity/type-checking gaps, not the generated import/export compiler blocker
  closed here.
